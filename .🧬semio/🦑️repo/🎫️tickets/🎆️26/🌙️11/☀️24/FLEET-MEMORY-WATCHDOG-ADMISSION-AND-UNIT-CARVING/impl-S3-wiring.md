# Implementation S3: wiring the host warden into every launch path

Date 2026-11-24. Scope: design §2 (switch, ensure, mirror, requeue). `W` = `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/`, `RE` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/`, `BLD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Nothing committed, no git command that modifies, no process signalled or restarted by hand, no enforce run, `mode` (`observe`) and `protect.json` (`/Users/ueli/.sinc/play`) untouched.

## Task 1: rollout switch

- `W/🧬️schema/🔣️.json` `Policy` (lines 7-8 of the const, description + `descriptionDe`): `defaultMode: "observe"` (const), `ensureMilliseconds: 15000`. Description (EN, DE) says: mode of a machine without a valid `mode` file; observe decides and alarms without signalling, enforce acts; the fleet is switched by changing this one value, each warden restarts itself on the changed source digest (the digest covers the schema).
- `W/🔁️loop/🟦️.ts` `defaultHostWardenPortsV1().mode` (line ~97): the `mode` file wins only when it holds exactly `observe` or `enforce`, otherwise `POLICY.defaultMode`. `W/⌨️main/🟦️.ts` docstring updated (no more "default enforce").
- Test: `W/🧪️tests/🟦️.ts` new "the rollout switch…" (missing file, `banana`, `enforce\n`, `observe`, `observe:true` port) and the policy `toMatchObject` now pins `defaultMode`/`ensureMilliseconds`; the default-ports test expects `defaultMode` instead of `"enforce"`.

## Task 2: cheap `ensureHostWardenV1`

- `W/🔁️loop/🟦️.ts` lines ~250-290: `digestMemory` (digest read once per process), `ensured` map keyed by `directory + code`; within `ensureMilliseconds` the earlier answer is returned with no file read (a `spawned` answer is remembered as `running`, so a burst of callers never double-spawns); `failed` is never remembered; option `memoise:false` for deterministic tests. The effectful part moved to `ensureOnce`.
- Test guard in the same function: a call that names neither `directory` nor `spawn` and runs under `NODE_ENV=test` (every `bun test`) returns `failed` and never spawns the machine's real warden. This is what keeps every existing worker/ready/test-file/scheduler test free of real spawns without touching them.
- Tests: the old ensure test passes `memoise:false`; new "ensure memoises" (second call within 14 999 ms removes the directory and still answers `running` without recreating it; at 15 000 ms and for another code it spawns; default digest path memoised).

## Task 3: call sites (all fire-and-forget, `void import(...).then(m=>m.ensureHostWardenV1()).catch(()=>undefined)`)

| path | file | anchor |
|---|---|---|
| a. gate | `W/../🎫️gate/🟦️.ts` `withHostCapacityV1` | line 282, after the bypass block, so only when the policy is enabled and the gate is not bypassed; skipped when `options.directory` is given (private directories of tests never spawn) |
| b. worker | `RE/👷️worker/🟦️.ts` | helper `ensureWardenV1` line ~227; called in `runRemoteExecutionWorkerV1` after the `sandbox` event (line ~311) and at the start of every `finishTask` (line ~447) |
| c. Ready supervisor | `RE/🛰️sidecar/🚦️ready/🟦️.ts` | `runReady` (line ~1310): a 1 s unref'd interval calls the mirror tick, which first runs the (memoised) ensure; one tick at once; cleared in `finally` |
| d. test-file | `BLD🔬️test-file/🏃️run/🟦️.ts` | lines 15-17 + 65: new port `ensureWarden` (default lazy ensure), called before each route that runs on the host |
| open | native-build broker | NOT wired (other agent owns `🏗️native-build/🎟️admission/**`) |

Tests: gate test `host-capacity/🧪️tests/🟦️.ts` 28 pass (no spawn: `options.directory` set and `NODE_ENV=test`).

## Task 4: per-worker `memory.json`

- Schema (`W/🧬️schema/🔣️.json`): `Absent` (`{version:1, atMs, absent:true}`) and `Mirror` = `oneOf [State, Absent]` (line ~241). Ready Policy (`RE/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json`): `memoryMirrorName` `memory.json`, `memoryMirrorMilliseconds` 5000, `memoryMirrorPollMilliseconds` 1000.
- Loop module: `currentHostWardenMirrorV1(directory?, nowMs?)` (line 80: the validated fresh state as written, else the absent record) and the validated reader for the fleet-status agent `readHostWardenMirrorV1(path): HostWardenMirrorV1|null` (line 85; null for a missing file, an invalid record or any extra key). Type `HostWardenMirrorV1` exported from `W/🟦️.ts`.
- Ready: `createReadyMemoryMirrorV1({store, origin, now?, ensure?, current?, write?})` (`RE/🛰️sidecar/🚦️ready/🟦️.ts` line ~708) writes `<store>/v1/origins/<origin>/memory.json` through `readyWriteStoreFileV1` (tmp+rename), at most every 5 s and at once when `level` or one of `suspended/resumed/cancelled` changed (absent to state counts as a change). Content is the State record exactly as the warden wrote it, or the absent record. The store is `SEMIO_FLEET_STORE_ROOT`.
- Group server (`g1-server`): `runReady` returns at once for the group server (`ports.server`: "the group server is never set up by this service"), before my interval starts. So the coordinator writes no `memory.json` and its Ready service does not ensure a warden. The coordinator's warden is started by the gate / CLI / test-file paths only. The fleet-status agent must treat a missing `memory.json` of the server origin as "server, not applicable", not as "no warden". A role that never runs stages but is not the server still runs the loop (and the mirror).
- Tests: Ready test "the memory mirror…" (absent first, no write within 5 s, write on level change, write on counter change, write after 5 s, no store, throwing ports); loop test "cancel records carry the command…" covers `currentHostWardenMirrorV1`/`readHostWardenMirrorV1` (state, absent, `absent:false`, extra key, missing file).

## Task 5: requeue instead of fail

### 5a remote unit
- `Cancel` record: `command` (first 512 chars of the process' command line, required) in schema + type + loop (`rows.find(pid).command.slice(0,512)` at the cancel write). New `readHostWardenCancelsSinceV1(sinceMs, directory?, nowMs?)` (line 75, newest first, within the record age bound); `readHostWardenCancelV1` shares the same lister.
- `RemoteExecutionRefusalV1` and `🏃️execution/🧬️schema/🌐️remote-execution.json` `Refusal` enum: `warden-cancelled`. Text EN/DE: `refusal.warden-cancelled` in `RE/🚀️offload/🧰️command/🧬️schema/🔣️.json` (the catalogue the worker already uses for `refusal.shipped`; the first attempt in `📖️texts` failed the command's "no unused message" test and was removed).
- Worker: `remoteExecutionWardenCancelledV1({exitCode, cancels, work, pid})` (pure, exported): exit 137 AND a record whose `pid` equals the task process pid OR whose `command` contains `<task work dir>/`. `executeRemoteExecutionActionV1` captures the task pid from the spawn callback, and after the process ended (not cancelled, not timed out) reads `wardenCancels(sinceMs=executionStart)` (injectable port `run.wardenCancels`, default `readHostWardenCancelsSinceV1`) and returns `{outcome:"refused", status:{code:8, message: EN / DE}, refusal:"warden-cancelled"}`; `finishTask` reports it with `refusal` and the `refused` event.
- Scheduler trace (`RE/🗓️scheduler/🟦️.ts` `report`, lines 315-329): `report` accepts a result when `operation.stage==="EXECUTING"`, attempt and worker match, which is the case for a running task; `refused` sets the worker cool-down, then for an offload operation adds this worker to `tried` and requeues when `placementIdleElsewhereV1`, else finishes it `DECLINED` (the client compiles locally); a non-offload operation is always requeued with `tried`. So NO scheduler edit was needed. Caveat: with no idle machine elsewhere an offloaded unit is declined to a local compile (same as an admission refusal today); I did not force a requeue.
- Doubled peak: the offload client only learns `killedFactor` x placed from a `failed`/137 result; a `refused` result never reaches that path, so nothing is taught.
- Limitation: the command is cut at 512 chars, so if the work directory path is beyond that point of the compiler's command line the pid match is the only fallback (the task root pid, which the warden does not cancel). Not measured on a real long `rustc` line. The process-group members are not enumerated.
- Tests (`RE/👷️worker/🧪️tests/🟦️.ts`): pure matrix (pid match, work-prefix match, similar prefix `ab123` vs `ab12` no match, no records, exit != 137) and a real child that SIGKILLs itself: record naming its work directory gives `refused`/code 8/`warden-cancelled`; an unrelated record or a throwing reader leaves a plain `completed` exit 137.

### 5b heavy question
- `BLD🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🟨️.mjs`: on `close` with `code===null` and signal `SIGKILL` (not silent-killed), `cancelOf(pid)` (port, default lazy `readHostWardenCancelV1`) is read; a record younger than the spawn gives `refuseBuildV1("failed","graph.heavy-question-cancelled",{pid,kind,megabytes,reason,question})`, anything else (no record, old record, failing reader) the old generic failure. `🟨️.d.mts` port type extended. Texts EN/DE `graph.heavy-question-cancelled` in `BLD📖️texts/🔣️.json` beside `-failed`.
- `BLD🕸️graph/📥️load/🛟️owner-isolation/🟦️.ts`: `ownerFailureDecisionV1` takes `cancelled?`: after `foreign`, `cancelled && round < retryRounds` answers `retry` (at the last round the old table decides: `stable` or `unsettled`). `heavyQuestionCancelledV1(error)` reads the key `build.graph.heavy-question-cancelled` on the error or its `cause` chain (depth 4); `ownerRetryPlanV1` has `error?` and both callers pass it (`observeOwnerWithRetryV1` line ~140, `project-nodes/🟨️.mjs` `ownerRetryPlanOfV1` passes `raised`).
- Tests: owner-isolation oracle `🐍️.py` extended (table keys with `c` suffix for cancelled, whole domain compared), new test for rounds, key detection, `observeOwnerWithRetryV1` (4 attempts, 3 pauses, then stable; success on the third; a plain failure is final at round 1). Heavy-question test: real child killed by SIGKILL with a record, no record, old record, throwing reader.

## Test results (`repo test-file`, exact)

| file | result |
|---|---|
| `W/🧪️tests/🟦️.ts` | 19 pass, 0 fail (was 16; 3 new) |
| `host-capacity/🧪️tests/🟦️.ts` | 28 pass, 0 fail, 4608 expect() |
| `RE/👷️worker/🧪️tests/🟦️.ts` | 10 pass, 0 fail (was 8) |
| `RE/🛰️sidecar/🚦️ready/🧪️tests/🟦️.ts` | 58 pass, 0 fail, 556 expect(), 148 s |
| `owner-isolation/🧪️tests/🟦️.ts` | 23 pass, 0 fail |
| `heavy-questions/🧪️tests/🟦️.ts` | 16 pass, 0 fail, 91 expect(), 217 s |
| `RE/⌨️command/🧪️tests/🟦️.ts` | 7 pass, 0 fail (first run 1 fail: unused text key, fixed) |
| `RE/🚀️offload/🧰️command/🧪️tests/🟦️.ts` | 11 pass, 0 fail |
| `RE/🧪️tests/🔎️refusal-reason/🟦️.ts` | 3 pass, 0 fail |
| `RE/🗓️scheduler/🧪️tests/🟦️.ts` | 22 pass, 0 fail |
| `BLD🔬️test-file/🧪️tests/🟦️.ts` | 16 pass, 0 fail |
| `BLD📖️texts/🧪️tests/🟦️.ts` | 3 pass, 0 fail |
| type check | `tsc --noEmit` through a scratch tsconfig over the 9 edited TypeScript files: no error in any edited line; the remaining messages are the scratch config lacking Node/Bun types (`once`, `Bun`, `bun:test`, `import.meta.dir`) |

## Live check on this machine (observe mode)

- A warden is running: pid 5597 (parent 1), started 03:23:19 by the `repo test-file` runs of this session (the new test-file call site fired), `mode: observe`, `code 3ac41650ee958588` which equals the digest of the current sources (computed with the pinned Bun after my last edit), lock `{"pid":5597,...}`, `state.json` 0.6 s old with 226 ticks, level `calm`, host 32768 MB total, 10184 MB available, swap 2171 MB used, 0 MB/s swap-out, compressed 8080 MB, pressure `normal`; counters suspended/resumed/cancelled 0/0/0, `alarms.jsonl` absent, `cancelled/` empty. Overhead as reported by the state: tick p50 92 ms, p95 212 ms, 0.57 % of a core, 46 MB resident.
- Play clone protected: `protect.json` = `["/Users/ueli/.sinc/play"]`; two processes with `/.sinc/play` in their command line exist, none is a subject (the only subject at the last read was one `pool-child`, 254 MB, command without `/.sinc/play`; earlier reads showed `test` and `heavy-question` subjects of my own test runs).
- `repo capacity status` (read-only) was run; it is not itself gated, so it does not prove the gate call site. The gate call site is proved by the unit tests only (no live gated command was run). The warden was already up when I looked, so I did not watch an "appearance" from a cold start; the spawn path is covered by the ensure test with an injected spawn and by the fact that the process exists with the current digest. It ends itself after 10 min without a subject or on a source change.

## Not done / not verified

- Native-build broker (`🏗️native-build/🎟️admission/**`) does not call ensure: OPEN for its owner. Machines that only build natively without the gate, a CLI call, a worker or the Ready loop get a warden from the other paths.
- The Ready supervisor on this machine and on the workers still runs the old code until each restarts itself: `memory.json` was not observed live, only unit-tested. The fleet-status agent can read it with `readHostWardenMirrorV1`.
- The group server writes no `memory.json` (see Task 4).
- Requeue of a warden-cancelled unit was tested at decision level and with a self-killing child, not with a real warden killing a real `rustc` under load; the 512-char command bound is the weak point (above). An offloaded unit with no idle machine elsewhere is declined to a local compile instead of requeued.
- A warden cancel of a heavy-question child relies on the child pid being the cancelled pid (the `heavy-question` kind scope is `tree`, the root of the tree is the child); pid reuse within the 1 h record window is guarded only by the record being younger than the spawn.
- `NODE_ENV=test` guard means a production process that sets `NODE_ENV=test` never starts a warden; none of the repo's launchers does.
- Linux/Windows: nothing run there; the call sites are platform-neutral, the warden stays observe-only on Windows.
- Flipping the fleet to enforce is the single `defaultMode` value (and a machine's `mode` file wins); it was not flipped.
