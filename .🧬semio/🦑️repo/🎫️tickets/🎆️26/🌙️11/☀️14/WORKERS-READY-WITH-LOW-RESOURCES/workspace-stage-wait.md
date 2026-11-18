# Workspace stage: what the workers do during the silent hours

Read-only investigation. Store snapshot clock: attest/ready/serve-state mirrors at 22:32-22:33 UTC (23:32 local). The store logs are rewritten live (`ready-log-workspace.txt` is the last 200 lines), so two reads taken seconds apart gave slightly different counters; ranges below give both where they differ.

Paths: store = `/Users/Shared/semio-store/v1/origins/<origin>/`, server = `/Users/ueli/Documents/semio/.cache/f/graph-server/`, code = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/` (abbreviated `build/` below).

## 1. Log shapes per origin

Collapsed line shapes (numbers replaced by N). "first-last" is the line range in the 200-line window.

### g1-m01
- No `ready-log-workspace.txt` in the store. Only `ready-log-dependencies/native/repo-mcp/tools.txt`, `ready.json`, `ready-timings.json`. Nothing to analyse for the workspace stage.

### g1-m02 (workspace, 201 lines, header 22:30:23Z)
| lines | count | shape |
|---|---|---|
| 1 | 1 | `<ts> workspace` |
| 2-201 | 187 | `Reading the project graph slice (N s): flush N of N.` |
| 3-195 | 13 | `[repo] still working after N s; last phase started: door.script` |

- Slice seconds: 1703-2078 (read 1) / 1823-2199 (read 2). Door counter: 4345-4675 (read 1) / 4465-4795 (read 2), one line every 30 s.
- Last non-heartbeat line: `Reading the project graph slice (2078 s): flush 1 of 1.`
- No `-failed` log.

### g1-m03 (workspace, 70 lines, header 22:28:54Z; failed log 21 lines, header 22:28:55Z)
Workspace log:
| lines | count | shape |
|---|---|---|
| 2 | 1 | `refused after N s if nothing of ours runs and not even N MB are free.` |
| 3-51 | 47 | `Reading the project graph slice (N s): reservation N of N; running (name, kind, seconds, rows): ; waiting for memory: N MB are asked for (N MB are enough to start), N MB are free above the N MB floor ...` |
| 7-54 | 4 | `[repo] still working after N s; last phase started: door.script` |
| 53-68 | 16 | `Reading the project graph slice (N s).` |
| 70 | 1 | `The plan cannot continue: the slice worker ended N times in a row with exit code N because the code on this machine kept changing under it; try again once the replica has settled.` |

Failed log: the same memory-wait line at 237/239 s, then `Reading ... (N s).` from 2 s to 32 s, one `still working after 711 s`, and the same exit-75 "code kept changing" text.
- Last line: `The plan cannot continue: the slice worker ended 4 times in a row with exit code 75 because the code on this machine kept changing under it; try again once the replica has settled.`
- ready.json now says `state: backoff`, `attempts: 7`, `exit: 1`, `refusal: stage-failed`, `tail` = `...(237 s): reservation 0 of 6553 ...`.

### g1-m07 (workspace, 201 lines, header 22:30:22Z)
| lines | count | shape |
|---|---|---|
| 2-201 | 187 | `Reading the project graph slice (N s): flush N of N.` |
| 6-198 | 13 | `[repo] still working after N s; last phase started: door.script` |

- Slice seconds 3295-3667 (read 1), door 6681-7041 (read 1). Last line `Reading the project graph slice (3667 s): flush 1 of 1.`
- Failed log (21 lines, header 20:31:44Z): `Reading the project graph slice (N s): verification 3575 of 3575.` at 38-72 s, one `still working after 50 s`, then last line `The resident service met an unexpected failure (reference 78e1ed55); try again, the details under that reference are in the service log .../semio-repo/501/host-f13cda8149a9.log.`

### g1-m08 (workspace, 200 lines, header 22:30:24Z)
| lines | count | shape |
|---|---|---|
| 2-201 | 187 | `Reading the project graph slice (N s): flush N of N.` |
| 9-200 | 13 | `[repo] still working after N s; last phase started: door.script` |

- Slice 2413-2786 s, door 3536-3896 s. Last line `Reading the project graph slice (2786 s): flush 1 of 1.`
- Failed log (21 lines, header 21:24:46Z): `flush 1 of 1` at 2273-2307 s, one `still working after 5337 s`, then last line `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 10267 MB, more than the 6553 MB this machine gives a plan, and only 3334 MB of memory were available ...`

### g1-m10 (workspace, 200 lines, header 22:30:17Z)
| lines | count | shape |
|---|---|---|
| 2-201 | 187 | `Reading the project graph slice (N s): flush N of N.` |
| 6-198 | 13 | `[repo] still working after N s; last phase started: door.script` |

- Slice 753-1126 s (read 1) / 880-1252 s (read 2); door 7941-8301 s (read 1) / 8061-8421 s (read 2). Last line `Reading the project graph slice (1252 s): flush 1 of 1.`
- No `-failed` log. The `ready.json` tail reads `(1236 s): flush 1 of 1` at 22:32:09Z, so the slice clock was still advancing at the snapshot.

### Interpretation of the window (all five)
- The slice clock advances exactly 2 s per two lines (the parent heartbeat, `build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs:60`), and the door prints every 30 s. Neither stops. What is silent is the worker's own progress: the only detail left is the last `flush 1 of 1`, re-sent every 2 s. No `verification`, `ask`, `waiting`, `cooperating`, `handoff` or `restarted` line appears anywhere in the 200-line window of m02, m07, m08 or m10.
- The 200-line window covers only about 6 minutes of slice time (374 s), so the actual silence can be longer than what the window shows. The log cannot say when the last real worker message was sent (inferred).

## 2. What the code does

### door.script
- `build/📜️script.ts:135`: `withScriptProcessEnvelope(envelope, original => profilePhaseV1("door.script", () => runScriptMain(new ScriptRouter(root,root).register("repo",RepoScript)...)))`. This is the whole admitted CLI run, after capacity admission (`door.capacity`, `door.admitted`, lines 120-131). `door.script` is therefore not an inner step. It is the last phase started in the door process.
- The "still working" line is `build/🚪️door/🔣️.json` (liveness text, `"[repo] still working after {seconds} s; last phase started: {phase}"`), printed by `startBuildDoorLivenessV1` (`build/🚪️door/🟦️.ts:121-128`). It fires after 5/10/20 s and then every 30 s, always, and the phase is `profileCurrentPhaseV1()`, i.e. the last phase *started in this process*. It says nothing about progress in the resident host or the slice worker (which run in other processes). Inferred from the code; the logs match it (the phase never changes from `door.script` for hours).

### "Reading the project graph slice (N s): ..." 
- Catalog: `build/🏡️resident/🧬️schema/🔣️.json:1962-1976` (`host-slice`, `host-slice-detail`, `host-slice-owners-*`).
- Emitted by `build/🏡️resident/🏛️service/🌐️host/🟨️.mjs:236` (`progress("slice", ms, 0, "host-slice-detail", {seconds, phase, done, total, running, decision})`), then `sendProgress` at `:205` forwards it to each client.
- The `(N s)` counter and the `phase done of total` text come from the slice worker's heartbeat in `build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs:60`: `beat=setInterval(()=>{note();onProgress?.(elapsed, detail)}, heartbeatMilliseconds)`. The heartbeat re-sends the **last worker `progress` message** (`detail`, set at `:66`) with a fresh elapsed time. **So `flush 1 of 1` with ticking seconds is a stale label (confirmed in the code, matches R62).** It is a liveness tick of the parent, not of the worker's work.
- Where the `flush` label comes from: `build/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs:194`: `await clock.time("flush", () => stage("flush", 1, async tick => { await graph.flush(); await tick(); }))`. This runs once at the end of every settled chunk (`settleChunk`, starting `:159`). The `verification` label comes from `:161` (one message per closure whose `ask` resolves).
- After the last chunk, `build/...closure-kinds/🟨️.mjs:223-224` runs `forgetParts(true); await clock.time("save", saveManifest)` and `noteTraceV1(...)`. Neither emits a progress message. So the last visible message is `flush 1 of 1`, and every step after it is silent until the next `ask` resolves.

### States after the slice verification, and which print nothing
| state | code | prints? |
|---|---|---|
| owners inference in the slice pool, per closure | `closure-kinds:161` (`verification N of N`) | yes, but only after each `ask` resolves; one slow `graph.get` is silent |
| flush of the chunk | `closure-kinds:194` | yes (`flush 0/1`, `1/1`) |
| save manifest / release after last chunk | `closure-kinds:223-224` (`clock.time("save")`) | no |
| footprint gate wait before a chunk (`pool` → `gate.admit`) | `build/🧠️memory/🦶️footprint/🟨️.mjs:141` (admit); `say("slowed")` only every `waitBoundMilliseconds` at `:156`; trace goes to a file | almost no (inferred: the wait is silent on the captured stderr) |
| reservation waits (memory headroom) | `slice-pool/🟨️.mjs:31` (`reserving`) and `:85-88` | yes, `waiting for memory ...` every heartbeat |
| owner too heavy → handoff to fleet | `slice-pool/🟨️.mjs:98` (`decision: handoff`), `wanted/read/🟦️.ts` (`fleetReadV1`) | yes once, then the stale detail returns |
| cut-and-continue restarts | `slice-pool/🟨️.mjs:97`, `:100` (`cut`), `:102-103` (`restarted`, cap `sliceRestartMaximum`) | yes once, then stale detail |
| fleet wait (served records from the server) | `build/🪪️graph-state/🙋️wanted/📖️read/🟦️.ts:45` (publish), `:52` (refresh), `:65` (`awaitWantedV1`), progress callback printing at most every 10 s on change | yes, `graph: waiting for the fleet: X of N projects arrived` (`wanted/🧬️schema/🔣️.json:101`); printed through `process.stderr` in `build/🕸️graph/🤝️prefetch/🟦️.ts:105` |

Important: none of the `waiting for the fleet` / `cooperating` lines appear in any of the five windows. Inferred: the fleet-wait loop is either not running in the process whose stderr is captured, or it is inside a step that prints nothing (the wait itself prints every 10 s if active).

## 3. Cross-check with store files and the server

### pool.json (per origin, all at 22:32-22:33)
- g1-m02: `state: pulling`, `wave` set, `running: 2`, `claimed 57, published 55, stored 89, held 18`, `rssMegabytes 2161`.
- g1-m03: `state: idle`, `reason: no-wave`.
- g1-m07: `state: idle`, `reason: nothing-fits` ("no owner of the wave fits the free memory and cores now"), `claimed 23, published 23`.
- g1-m08: `state: pulling`, `running: 1`, `rssMegabytes 1365`.
- g1-m10: `state: idle`, `reason: no-wave`, `claimed 3, published 3`.
- So none of them is in a pool wait for a wave that would carry the heavy owners. The pool is not pulling for the stuck workers (inferred: their heavy owners are not in any published wave).

### attest.json
- All five have `wallMs` at 22:33:08-22:33:10 UTC and the same `code` digest (`b4c6513b…`) and `manifest` (`7d5cc648…`). The replica is live and all five agree on code. So the "code kept changing" exits are older, not current.

### Server (g1-server) served state
- `.cache/f/graph-server/serve-state.json` (saved 22:32:13Z). `held` (requests the server has taken and not yet served):
  - `g1-m10`: 88 projects, **88 heavy**, `createdMs` 22:13:18Z, `expiresMs` 22:48:22Z, `told: true` (22:32:01Z).
  - `g1-m02`: 38 projects, 36 heavy, `createdMs` 21:58:53Z, `expiresMs` 22:47:43Z, `told: true`.
  - `g1-m08`: 46 projects, **46 heavy**, `createdMs` 21:45:19Z, `expiresMs` 22:51:04Z, `told: true`.
  - `g1-m03`, `g1-m07`, `g1-m01`: not held.
- `read-record.json`: the server's own read started ~19:57Z (`startedMs 1794774608750`), `beatMs` 22:33:15Z, `status: null` (still open).
- `read.log` tail (server's own read, not the workers'):
  - `[repo] still working after 7373 s; last phase started: source.served` (79 lines)
  - `[repository:command] still running after 3733 s` (81 lines)
  - `build:projects · Inferring projects: 28 of 48 projects · 13 running, longest 1785 s` (23 lines)
  - `graph: waiting for memory headroom (18816 of 12534 MB reserved, 8 projects running): @semio-tech/framework-rs needs 256 MB and has waited 957 s; after 60 s it runs alone if it fits ..., otherwise the run is refused with exit 75` (24 lines)
  - heap census 11.6-21.4 GB; `the process held 11892 MB of its grant of 14336 MB`.
- Reading: the server is itself stuck in a memory-headroom wait with more reserved than its budget (18816 of 12534 MB). It is serving nothing new. Its `read.log` is also silent for long periods (same pattern as the workers).
- `priority.json` (23:32 local) is the server's priority list; `pending.json` lists generation 239 with 2000+ projects. Not analysed further.

## 4. Conclusions per worker (inferred unless marked)

- **g1-m10**: waiting on the server (inferred). It has 88 heavy owners (all of them) posted as a wanted request to the server since 22:13Z; the server has taken them (`told` 22:32Z) but its own read is in a memory-headroom wait and has served none. The worker's door is alive; its slice heartbeat repeats `flush 1 of 1`. Its last real slice message is the flush of the last closure chunk; from then on there is no message, which fits a silent step after the last chunk or a long single closure derive. Not hung in the strict sense (door heartbeat alive, slice heartbeat alive), but no progress is visible.
- **g1-m02**: same pattern, 36 of 38 requested owners heavy (waiting on the server), 2 light ones either done or local. Waiting on server (inferred), with the same silent flush-then-nothing sign.
- **g1-m08**: 46 of 46 owners heavy, all waiting on the server since 21:45Z. Its earlier failure (`grew to 10267 MB`, owner-too-heavy) is consistent with a worker that must hand every owner to the fleet. Waiting on server (inferred).
- **g1-m07**: not a requester; computing locally or silent after a closure flush (inferred). Its pool is `nothing-fits`, so it is not pulling a wave either. Its earlier failure was the resident service (`78e1ed55`), not the wait.
- **g1-m03**: not running the stage now: `state: backoff`, 7 attempts, failing with `stage-failed` after the 4x exit-75 "code kept changing". Not waiting on the server.
- **g1-m01**: no workspace log in the store; no conclusion.

Why the silence lasts hours (inferred from the code):
1. The fleet-first wait never trips its stall bound while the server answers heartbeats: `awaitWantedV1` (`wanted/🟦️.ts:53-70`) counts "alive" from the server's `atMs` (refreshed every poll), and "stalled" only from `progressAtMs`. The server's `beatMs` is current (22:33Z), so the worker sees a live server and keeps waiting up to `waitMaximumMilliseconds` = 6 h (`wanted/🧬️schema/🔣️.json:16`). The 30 min stall bound (`waitMilliseconds`, `:14`) applies only if the server's progress stops moving; the server's read is in a wait, so that signal stalls, which should trip after 30 min. Whether it does depends on the server's `progressAtMs` field, which I did not verify (inferred).
2. The waiting loop prints at most every 10 s (`wanted/📖️read/🟦️.ts:~progress`), but prints go to the process's stderr; none appear, so the loop is probably not in the captured process right now (inferred, see section 2).

## 5. Changes needed

(a) Progress always printed with counts
- Replace the stale-detail replay in `slice-pool/🟨️.mjs:60` with an honest heartbeat: `{phase, done, total, lastMessageAgeSeconds}`, and print the age of the last real message, so a silent worker is visible as silent.
- Emit a progress line in `closure-kinds:223-224` (before `saveManifest` and `forgetParts`), and per-`ask` counts while a single `graph.get` is pending (`in-flight: N`, age of the oldest).
- Emit the footprint gate wait (`footprint/🟨️.mjs:156`) through the same progress channel (not only the trace file) with counts.
- Print the fleet-wait line at every `awaitWantedV1` progress change and at least every 30 s, with the server's last `progressAtMs` age; today it is throttled to 10 s on change and goes to stderr only (`wanted/📖️read/🟦️.ts`, `prefetch/🟦️.ts:105`).
- Door liveness (`build/🚪️door/🟦️.ts:121-128`) should report the host's last job progress age, not only `profileCurrentPhaseV1()`; `door.script` should be replaced by the host's current decision (`waiting for fleet: X of N, server age Y s`).

(b) Never wait on the server for owners the machine or the pool can infer
- `fleetReadV1` (`wanted/📖️read/🟦️.ts:16`, `classifyOwnersV1` at `:20-23`) makes every heavy owner wait for the fleet "however few they are", and only infers them solo after the wait. Change this: a heavy owner that fits the machine's solo grant (`soloGrant`) should be inferred locally, one at a time, before any wait; the fleet is only for owners that exceed the solo grant.
- Before posting a wanted request, consult the local owner pool / pool records (`owner-pool`, `pool.json` wave data): if the owner cost is known and the pool has no wave for it, infer.
- The server must not accept heavy requests it cannot reserve: `graph: waiting for memory headroom (18816 of 12534 MB reserved ...)` shows reservations above the budget. Cap reservations at the budget at admission.
- Silence detection must use the server's real progress (`progressAtMs`), not its heartbeat, so a server whose read is stuck for 30 min releases the worker to infer solo (currently bounded only by 6 h).

## Open points
- I did not read `ready.json` / `ready-timings.json` for g1-m01, and I did not verify the server's `progressAtMs` field.
- I did not check the footprint trace files (`<tmp>/semio-footprint/<pid>.log`), which may hold the silent gate waits.
- Store logs are live; re-run section 1 before acting on the exact counters.
