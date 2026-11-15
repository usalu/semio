# Graph service start timeout on the macOS host

Scope: `repo run @semio-tech/framework-os-dev:activate-shooting-react-prototype` ended with "The graph service could not answer (timeout)" after 66.6 s; `shooting-js:test` exited 75 (277 stale projects, no fleet delivery in 300 s); `repo service host status` hung in `build:source-admission`.

## Where the messages come from

- "was denied access": `build/🧭️invocation/🛰️plan/🟦️.ts` (retry path after a `consent` refusal).
- "Waiting for the repo graph service (N ms)" and the timeout: `build/🏡️resident/🚪️client/🛰️plan/🟨️.mjs` (`connectHostV1`, texts `host-wait` in `🏡️resident/🧬️schema/🔣️.json`).
- Service start: `build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs` (`spawnHostServiceV1`).

## Why the launcher-owned service was denied

The checkout is below `~/Documents`, a TCC-protected folder. On macOS the service is started as a launchd job whose program is the launcher `repo`; it only reads the checkout when the launcher holds the Files and Folders consent. The exit log (`$TMPDIR/semio-repo/<uid>/exit.log`) shows the launchd service (rss 2.9 GB) ending `access-denied` with `Relative filesystem fstatat failed (EPERM)` a few seconds after it started: the stored consent marker of the launcher was stale (the launcher binary changed or the grant lapsed). This is the designed path: the client invalidates the marker, retires that service and restarts it directly under the access of the calling app (`SEMIO_SERVICE_LAUNCHD=off`). Denial is not a defect of the restart; it is the cost of a launcher without consent. Several agents did this within 40 s (four `start` rows, `access-denied`, `adopted` and `superseded` exits between 05:46:46 and 05:47:13), each retiring the service of the other.

## Why the restarted service did not answer in 66 s

Evidence (read only: `repo fleet status`, the service log `host-ada6c7a0852c.log`, `exit.log`):

- The host is overcommitted: load 13 to 32 on 10 cores, memory admitted 16 GB of 16 GB, spare memory negative against the 2 GB floor.
- A fresh service first runs the whole launcher door (source admission, implementation source, `resource-directory-admission`) and only then listens. In the log `graph-plan` waited more than 180 s for 6.5 GB of free memory behind other agents' jobs. None of that is a defect of the planner; it is queueing on memory.
- The client decided "timeout" with `startTimeoutMilliseconds` (60 s) of **log silence** only. It could not tell a living, queued, cold-starting process from a dead one. A process that is blocked in a synchronous scan or in the capacity queue writes nothing for a minute on this host; the client then gave up and the retry spawned yet another start, adding to the thrash and the load.
- The launcher `status` hang is the same door cost: every `repo` invocation admits the sources under capacity before it answers; it is not a lease deadlock (`repo fleet status` showed 0 blocked, 0 alerts).

## Should the client attach to the in-progress planner

It already does as soon as the service listens: the planner is single-flight inside the service and a second client attaches to the same socket and waits for the answer with no timeout. What failed was the step before that, deciding that a service that has not yet bound its socket is gone. The right signal is liveness of the process that was started, not silence of its log.

## Fix

- `HostPolicy` (schema first): `startCeilingMilliseconds` 900000 and `startLivenessMilliseconds` 3000.
- `connectHostV1`: `spawnService` may resolve a handle `{alive()}`. While the started process lives the client keeps waiting up to the ceiling; when it died without a socket the start fails at once as `lost` with the reason from the exit log; without a handle the old silence rule applies. Policy overrides now merge over the authored defaults.
- `spawnHostServiceV1`: returns the handle (direct child: pid probe; launchd job: `launchctl list` pid).
- Tests: two new tests in `build/🏡️resident/🧪️tests/🛡️consent-service/🟦️.ts` (a live silent start connects after 700 ms despite a 150 ms silence timeout; a dead start fails as `lost` at once; a live start past the ceiling times out).

## Not fixed here (follow-up)

- Concurrent clients that each see a denial retire the service the other just started; a per-host single-flight of the denial recovery (lock around retire plus restart) would remove the thrash.
- The 6.5 GB `graph-plan` reservation cannot be admitted on this host while other agents run; reducing parallel agents or shrinking the reservation is a capacity decision.

## Test status

`repo run <project>:test` could not be used: `repo show targets` did not finish the graph (verifying 8170 closures, cancelled after 240 s). The whole file `build/🏡️resident/🧪️tests/🛡️consent-service/🟦️.ts` ran through `repo tools exec bun -- test`: 19 pass, 0 fail (including the two new tests).

## Restart thrash after a consent denial (fixed)

- `withRestartOwnerV1` (client): one host-wide owner per implementation tag, lock `restart-<tag>.lock`. The owner invalidates the marker, retires the denied service and starts the next generation (`SEMIO_SERVICE_LAUNCHD=off`); every other client waits for the lock and then attaches to that generation without retiring anything.
- A plan refusal now carries `servicePid` (from the welcome frame); `retireHostServiceV1({only})` leaves a service of another pid (a newer generation) alone.
- `requestInvocationPlanV1` (`build/🧭️invocation/🛰️plan/🟦️.ts`) uses both; the port `restartOwnerV1` is injectable.
- Tests (consent-service file): restart owner is unique and joiners wait; retire honours `only`. Outcome via `repo tools exec bun -- test`: consent-service 21 pass, 0 fail.
- `build/🧭️invocation/🧪️tests/🛰️plan/🟦️.ts`: 10 pass, 1 fail ("a refused or unreachable plan ends the command as a localized refusal"); the fixture expects the consent refusal text but the real consent probe on this macOS host answers ok for the fake `/Users/worker` root, a path the changed code does not touch (first probe unchanged); not caused by this change, environment dependent.

## Independent check: two failures in the invocation plan test (fixed)

1. Consent row: not a stale fixture but a host-dependent test. The test used the real consent probe, which answers ok for the fake `/Users/worker` root on this macOS host, so the new `graph.service-denied` text was produced instead of the frame text. The test now injects `probeConsentV1` (first ok, then denied), so the fixture (en+de) stays valid on every host.
2. Slice digest: not caused by the restart change. The long path of the test runtime directory falls back to the shared `/tmp/semio-repo/<uid>`; the previous run's deliberate corruption step (`core` to `c0re`) left a file of the right content-address name there, and the service's `spill` trusted any existing file of that name. Root fix: `spill` in `build/🏡️resident/🏛️service/🌐️host/🟨️.mjs` now verifies the existing file against its digest (`spillIntactV1`) and rewrites a damaged one.
- Result via `repo tools exec bun -- test`: invocation plan file 11 pass 0 fail; consent-service plus service host files 21 pass 0 fail.

## Mid-read loss in activate-opus-4 (fixed)

Exit log of the run: the launchd service (pid 98645) ended with `crash` at 06:53:36: `Error: Unknown message key host-slice-owners-stopped` thrown by `renderTextV1` inside `sendProgress`. The memory governor of the slice inference (fem-js over its limit) emitted the decision `stopped`, whose text was missing in the catalogue (`🏡️resident/🧬️schema/🔣️.json`) after the fleet-inference edits. Not retired by a peer, drift, launchd or the governor kill itself: a progress message crashed the service while it served a read. A `superseded` exit only happens idle (`connections 0, inFlight 0`), `drainAndExit` already waits for in-flight requests; the crash bypassed both.

Fixes:
- Catalogue: `host-slice-owners-stopped` (en, de).
- Service `text()` no longer throws for an unknown `host-slice*` key; it logs `message-key-unknown` and renders the plain owners or detail line.
- Client: `requestPlanV1` treats a connection lost after progress frames as a loss of the generation, not of the attempt: it attaches to the replacement without using an attempt, at most `reattachMaximum` (5, HostPolicy) times. A loss before any answer still counts. (The replacement recomputes the read; the fact index is warm only for finished owners.)
- Tests (consent-service file): reattach with a fake service that dies mid-answer; stopped text in both languages; unknown decision key does not break the service. Results: consent-service 27 pass 0 fail, invocation plan 12 pass 0 fail.
