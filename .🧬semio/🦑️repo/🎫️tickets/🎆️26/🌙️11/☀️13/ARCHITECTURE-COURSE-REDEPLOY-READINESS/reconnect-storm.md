# Reconnect Storm: Does the Course Client Always Recover After a Proctor Restart?

Question (chaos run 24): two real browsers stayed on "Course server not reachable" without a presence footer after the
proctor was killed and restarted, although HTTP reached the proctor. Goal: prove the client always recovers, or fix it.

Verdict: **not always, and the harness overlap was not the (only) cause.** The storm spec reproduced the indicator half
of the symptom against the static release build with exactly one proctor process owning the port, and two defects were
fixed (a connection indicator that never probed on its own, and a presence join that could wait forever). After the fixes
every page recovered in every cycle of every run (see "Results").

## Every path where a retry could stop or stall

Presence socket (`PresenceRoom` in `👥️presence/🔌️socket`, rooms in `👥️presence/🧭️rooms`):

| # | Path | Verdict |
|---|------|---------|
| P1 | A join attempt whose socket neither opens, errors nor closes (a proctor or middlebox that accepts the connection and never answers a handshake, or black-holes packets): the attempt promise never settles, so `retryWithJitteredBackoff` never reaches its next iteration. Only the OS connect timeout (minutes) or never frees it. | **Real stop. Fixed**: a socket must be welcomed within `PRESENCE_JOIN_MS` (10 s) or it is closed and the room joins again. |
| P2 | An open, welcomed socket that dies silently (no FIN/RST: Wi-Fi hand-over, NAT drop, sleeping laptop, a frozen or black-holed proctor). The client had no liveness signal (the server's protocol-level pings are invisible to a browser), so a dead socket was noticed by the browser's TCP keepalive/send timeout only. | **Real stop. Fixed** with an app-level `heartbeat` frame: see "Liveness frame" below. |
| P3 | `run()` loop ending: `retryWithJitteredBackoff` has no `maxAttempts` here, so it only ends on `stop()`; a synchronously throwing `connect` (bad URL, mixed content) rejects the attempt and is retried at the cap; `rejoinDelay` is capped at `timing.maxMs` (15 s); nothing throws out of `run()` (`.catch(() => undefined)` only swallows the abort). | Cannot stop retrying. |
| P4 | Timers cleared on `visibilitychange`, `online`, `offline`: `PresenceRoom` and `CoursePresence` listen to none of them, so no event can clear or starve a retry timer. A hidden tab only suffers the browser's own timer throttling (1 s, and about once a minute after five hidden minutes in Chromium), which stretches a 15 s backoff but never stops it. | No stop. Recovery while hidden is bounded by browser throttling, not by the client. |
| P5 | Race between the `online` event and a socket close: `online` is not used by presence; a new socket created while offline fails and closes, and is retried. Chromium's `setOffline` does not drop established sockets. | No stop (covered by the offline-around, offline-after and fuzz cycles). |
| P6 | Flapping server (admits and drops sockets): `rejoinDelay` grows to the cap (15 s) and resets after 10 s of a stable session. | No stop; worst case one cap. |
| P7 | `CoursePresence.update` replaces a room only when its scope changes; `stop()` only on unmount. A restart changes no scope. | No stop. |

HTTP path and connection indicator (`🛂️proctor`, `🧭️session`, `🏆️leaderboard`, `📮️outbox`):

| # | Path | Verdict |
|---|------|---------|
| H1 | The indicator is driven by `ProctorClient.reachability`, which changes **only when a call ends**. After a call failed (a leaderboard poll, a one-off crowd read, a command given up by `patient()` which calls `silent()`) nothing asked the proctor again. Recovery depended on the next incidental call. | **Real stall. Fixed**: `CourseSession` probes the contract (`GET /instance`, jittered backoff capped at 15 s) while the proctor counts as unreachable, until it answers. |
| H2 | The only periodic caller on the overview and leaderboard is `usePolling`, whose backoff after unanswered polls grows to `POLL_BACKOFF_MAX_MS` = 300 s, which also decides when a stale "away" indicator may flip back; `ask` also does nothing while the tab is hidden, and a look at the tab never cuts a backoff short. Run screens do not poll at all. | **Real stall** (this is what the baseline showed). **Fixed**: the indicator no longer depends on the poll (H1), and the poll backoff ends at once when the proctor answers again (`session.revived`). |
| H3 | `background()` loads (course, learner, run) stop retrying on a non-transient answer (a 404 after `PROJECTION_GRACE_MS` = 10 s counted from the first attempt, any other 4xx): they raise a "refused" notice with a retry button. | Not reachable by a restart of this proctor (it catches up its projections before it listens, see `proctor.log`). Not changed. |
| H4 | `Outbox.pump` ends on an exception of the store or of an observer (`catch { return }`) with entries left; the next trigger (enqueue, `online`, a reachability change, `reconnect()`) starts it again. | A restart triggers none of these exceptions. Not changed; residual risk on a failing `localStorage`. |
| H5 | Contract re-agreement: `agreed` is cached for ever when the proctor agreed, so a restart of the same contract costs nothing; a refusal, a 400 or an unreadable answer asks again (`agree(anew)`); a contract found incompatible is left alone for `AGREEMENT_RECHECK_MS` (30 s) and asked again afterwards. A transient failure of `agree()` leaves the agreement unknown and rejects every concurrent caller, so each retries at the cap. The shared `agreeing` promise cannot stick: it clears in `finally`, and every `fetch` has a 10 s timeout. The probe respects the 30 s recheck window. | No stop. |
| H6 | `online` event: `Lifecycle.watchBrowser` only wakes the outbox and re-renders; it never touched reachability. | Covered by H1's probe. |
| H7 | `patient()` gives a call up after the learner's patience (3 s) and marks the proctor unreachable without any later call. | Covered by H1's probe. |

## What changed

- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts`: `ProctorClient.probe` (asks the contract anew, notes what it found, leaves an incompatible proctor to the 30 s recheck).
- `…/🧭️session/📡️proctor-link/🟦️.ts`: `ProctorLink.revive` (probe through shortages with the configured backoff).
- `…/🧭️session/🟦️.ts`: `CourseSession.revive` (one probe per lifetime while unreachable or incompatible; started on every reachability change, on `start()` and on `reconnect()`; ends on success or when the session stops) and `CourseSession.revived` (subscription: the proctor answers again).
- `…/🏆️leaderboard/🟦️.tsx` and `…/🏠️home/🟦️.tsx`: `usePolling` accepts that subscription and ends a backoff at once when the proctor is back.
- `…/👥️presence/🔌️socket/🟦️.ts` and `…/⚛️react/🟦️.tsx`: `PRESENCE_JOIN_MS` and the `joinMs` option: a socket that does not join in time is closed and replaced.
- Tests (all in `🧰️framework/🛍️products/📚️course/🧪️tests`): `🚪️presence-room` (stalled handshakes, default join time, left room), `🧩️deputy-and-contract` ("a proctor that is back": probe finds the proctor again with no other traffic, no probe while it answers, none after stop), `🚦️rate-limits` (a backoff ends when the proctor is revived; two older tests keep their meaning with `joinMs` set very high).
- Storm spec: `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🔁️reconnect-storm/🟦️.ts` and `🥒️.feature`; project `reconnect` (after `chaos`, exclusive shard) in `🎭️e2e/🎚️config/🟦️.ts`, the shard table `🎭️e2e/🔒️sessions/🔣️.json` and its fixture vectors `🎭️e2e/🔒️sessions/🧫️fixtures/🔣️.json`.
- Ticket scripts: `reconnect-build.mjs` (release site build into the ticket folder, proctor origin 8797), `reconnect-stack.mjs` (static site on 6066 with SPA fallback, one production-mode proctor on 8797, killed by PID, one control queue, refuses to start when the port is taken, waits for the port to free after a kill), `reconnect-run.sh`, `reconnect-repeat.sh`.

## The storm spec

Ten browser learners (alternating en/de; 3 in a run, 3 on the leaderboard page, 4 on the overview). Every cycle injects one of
seven kinds in turn, with a down-time from {0.1, 0.3, 1, 3, 8, 20, 45, 90} s: plain kill, kill during a presence handshake
(on the first new `websocket` event after the restart), kill while a command is in flight (on the first `POST /commands`),
network drop around the kill, network drop after the kill, hidden tabs around the kill (`visibilityState` override plus
`visibilitychange`), and a six-step random fuzz of all of these. After the cycle the proctor, the networks and the tabs are all
back; every page must show the presence footer with all learners online and the indicator calm within
`RECOVERY_BOUND_MS = 2 x 15 s (RETRY_TIMING.maxMs) + 10 s = 40 s` (plus 5 s of slack). The last cycle submits a whole run
and kills the proctor while `submit-run` is in flight; the learner must reach the results. Seeds are printed and reproducible
(`RECONNECT_SEED`).

## Results

Environment: static release build of the site (`reconnect-build.mjs`, proctor origin 127.0.0.1:8797 baked in), served
with SPA fallback on 6066; one production-mode proctor on 8797 accepting that origin; Chromium via Playwright, 10 learners.
The machine was heavily loaded by other jobs during all runs (load average 50 to 70), so these timings are pessimistic.
Exactly one proctor owned the port at any time: `reconnect-stack.mjs` serializes start and stop in one queue, kills by PID,
waits for the exit and for the port to free, and refuses to start when the port is taken; `proctor.log` shows no bind
failure in any run.

Baseline (before the fixes, seed 1, 8 cycles, same spec): **4 of 8 cycles left pages stuck**. The stuck pages (learners on the
overview or the leaderboard, i.e. the ones that poll) had their presence footer back with all ten online after 8 to 12 s but
their indicator still on `alert` ("Course server not reachable") 45 s later, in the cycles with a 45 s kill during an in-flight
command, a 90 s kill with hidden tabs, a 1 s fuzz and the submit kill. Worst calm time of the cycles that did recover: 36.1 s.
That is the run-24 symptom, reproduced with a single proctor process, so overlapping harness restarts were not required.
(The missing presence footer of run 24 was not reproduced: presence recovered in every cycle, worst 14.1 s.)

After the fixes (8 green runs, 69 cycles in all, every cycle including the submit-kill one):

| Seed | Cycles | Stuck pages | Worst presence | Worst calm |
|------|--------|-------------|----------------|------------|
| 1  | 8 | 0 | 11.8 s | 12.5 s |
| 11 | 9 | 0 | 15.1 s | 13.4 s |
| 12 | 9 | 0 | 13.5 s | 13.7 s |
| 13 | 9 | 0 | 15.5 s | 11.3 s |
| 14 | 9 | 0 | 17.1 s | 9.5 s |
| 15 | 9 | 0 | 13.7 s | 10.6 s |
| 21 | 9 | 0 | 13.8 s | 10.2 s |
| 22 | 9 | 0 | 11.4 s | 8.5 s |

**Measured worst recovery: 17.1 s for the presence footer with everyone online, 13.7 s for the indicator back to calm; the
bound is 40 s** (2 x the 15 s backoff cap + 10 s request timeout; the spec allows 5 s more before it calls a page stuck).
Every run injected 9 to 12 kills, one in a handshake and two in flight, plus the offline, hidden and fuzz cycles. Seeds 21 and
22 ran the final spec, whose generator walks all eight down-times (0.1 s to 90 s) in every run; the earlier seeds drew them at
random (all eight appear across the runs). The learner whose `submit-run` was cut off by the kill reached its results in every
run. The kill of a command in flight shows how the retry works: the command is sent again with the same id after the proctor
is back and decided once.

Unit tests (course-react vitest, jsdom): 15 files, 395 tests pass, among them `🚪️presence-room`, `🚦️rate-limits`,
`🧩️deputy-and-contract`, `🏆️leaderboard`, `👥️shared-presence`, `📬️outbox-delivery`, `🫡️deputy-decisions`,
`🎬️session-playthrough` and `🏔️challenge-session`. (A first run under a machine load of 70 to 140 failed some UI suites
on 5 s test timeouts; with the machine quiet, or the timeouts lifted, they pass.) `🎭️sessions` (the shard table vectors) passes with the
`reconnect` project (13 tests). The type checker reports one error, in `🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed`,
unrelated to this change; the changed files type-check.

Note: the first call of `reconnect-build.mjs` wrote its build over `🗑️generated/chaos/site` (the script it was copied from
had changed meanwhile); that site was rebuilt with `chaos-build.mjs` straight away.


## Liveness frame (silently dead presence sockets)

Schema first: the wire fixture `🖥️server/🧪️tests/🧫️fixtures/🔌️wire/🔣️.json` gained the vector `presence-heartbeat`
(`{"type":"heartbeat"}`) and the section `presence: { heartbeatMs: 10000, silenceFactor: 2.5 }`. Both languages test against it:
Rust (`🧪️tests/🔬️wire/🦀️.rs`: every frame has a vector, `PresenceSettings::default().heartbeat` is the fixture's value and is shorter
than the protocol keepalive) and TypeScript (`🧪️tests/🔬️wire/🟦️.ts`: round trip, `PRESENCE_HEARTBEAT_MS`, `PRESENCE_SILENCE_FACTOR`).

- Server (`🖥️server/🔨️modules/📡️gateway`): `PresenceFrame::Heartbeat` (`🧬️contract/🦀️.rs`), `PresenceSettings::heartbeat` (10 s,
  `👥️presence/🦀️.rs`), a heartbeat tick in `run_presence` (`🛰️presence-stream/🦀️.rs`); a client may not send one (refused as invalid).
  Unit test `an_idle_socket_is_sent_a_heartbeat_every_interval_and_a_client_may_not_send_one`.
- Client (`📚️course/🎯️targets/⚛️react/🔨️modules/👥️presence/🔌️socket/🟦️.ts`): every frame of any kind re-arms a watchdog of
  `PRESENCE_SILENCE_MS` = 2.5 x 10 s = 25 s once the socket is welcomed; when it fires the socket is closed and the room joins
  again through the existing retry path (`lost()` then `retryWithJitteredBackoff`). Unit tests in `🚪️presence-room`: heartbeats keep a
  socket for minutes, 25 s of silence closes it and rejoins (four times in a row), any other frame also counts, `stop()` clears the timer.
- Wire types and codec (`🖥️server/🧬️wire-types`, `🔤️codec`, package `wire-types` export): `heartbeat` variant, constants.
- Storm: cycles `black-hole` (proctor frozen with `SIGSTOP`, thawed with `SIGCONT`), `long-black-hole` (frozen for at least 40 s) and
  `freeze-then-kill` (frozen for at least 30 s, then killed while stopped). While the proctor is frozen for longer than the silence
  limit, the spec asserts that **every page dropped its presence footer** (the only way a frozen proctor's sockets close is the
  watchdog; counted as `silent-socket-closed`), then that every page recovers within the bound after the thaw or the restart.
  The gate's control endpoint gained `POST /proctor/pause` and `/proctor/resume` (`🎭️e2e/🟦️.ts`), as did `reconnect-stack.mjs`; the
  freeze cycles are skipped on Windows (no `SIGSTOP`).

## The tsc error in `🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed`

Not a defect of that file and not mine: the Rust enum `ContainerRole` and its schema row gained `cardStack` and `card`, the typed decoder
(modified today) already accepts them, but the generated TypeScript `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` (dated 14 November)
still lists only seven roles, so `choice(..., [... "card"])` no longer fits `ContainerRole`. Someone is mid-way through adding the role;
the generated file has to be regenerated by its owner. Left untouched.

### Results with the heartbeat (proctor and site rebuilt, black-hole cycles added)

| Seed | Cycles | Frozen cycles | Stuck pages | Worst presence | Worst calm | Silent sockets closed by the watchdog |
|------|--------|---------------|-------------|----------------|------------|-----|
| 31 | 10 | 2 (frozen 8 s, 20 s: below the silence limit) | 0 | 12.6 s | 11.4 s | not asserted yet |
| 41 | 11 | 3 | 0 | 13.7 s | 12.1 s | not asserted yet |
| 42 | 11 | 3 | 0 | 12.2 s | 13.5 s | 20 of 20 |
| 43 | 11 | 3 | 0 | 12.7 s | 13.5 s | 20 of 20 |
| 44 | 11 | 3 | 0 | 12.8 s | 12.3 s | 20 of 20 |

Worst recovery of the whole round: 13.7 s for the presence footer, 13.5 s for the indicator (bound 40 s). In seeds 42 to 44 every one
of the ten learners had dropped its socket after 40 s of a frozen proctor (two such freezes per run) before the thaw or the kill, so the
heartbeat watchdog, not luck, ended every black hole. One earlier attempt of the round failed at its start because a run I had
interrupted by hand left the proctor killed; the stack was restarted and the seeds rerun. Rust: `semio-framework-server` (wire, 6) and
`semio-framework-server-tests` + `semio-framework-server-presence` (138, with the heartbeat test) pass; the proctor workspace
(`teaching-proctor`, `-domain`, `-base`: unit, conformance, end-to-end, 133 tests) passes (the harness then reported its script deadline
as exhausted after the last binary, not a test failure). TypeScript: `🖥️server` wire test 19 passed, course-react touched files 12 files
248 tests, `🎭️sessions` 13 tests.

## How to rerun

1. `repo cargo --store=shared --configuration=release -- build --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --bin proctor --release`
2. `node reconnect-build.mjs` (the site) and `PROCTOR_EXECUTABLE=<proctor> node reconnect-stack.mjs` (the stack; stop it with SIGTERM).
3. `./reconnect-run.sh` (one run; `RECONNECT_CYCLES`, `RECONNECT_SEED`, `RECONNECT_LEARNERS`, `RECONNECT_REPORT`) or `./reconnect-repeat.sh` (`RECONNECT_SEEDS="11 12 13 14 15"`).
   Through the gate, the project is `reconnect` (`--project reconnect`), after `chaos` in the exclusive shard.

## Residual risks

- H3 and H4 (a non-transient answer ends a background load; a throwing store ends the outbox pass until the next trigger) are not reachable by a proctor restart; not changed.
- Hidden tabs in a real browser throttle timers (about once a minute after five minutes hidden); the storm emulates hidden tabs by `visibilityState`, which does not throttle, so a tab hidden for many minutes may take up to a minute longer than the bound after the proctor is back.
