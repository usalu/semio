# Census: remote execution (wave 1121, read-only)

Base: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution` (abbreviated RE below). No source edits, no git writes, no ticket changes. Facts are from code reading and test runs on 2026-11-21.

## 1. Why `repo remote-execution status` says `configured:false`

Root cause: the status command reads only the explicit environment variable, never the fleet facts that the scheduler publishes.

- `⌨️command/🟦️.ts:183`: `endpoint=optional(environment,names.endpoint)` is `SEMIO_REMOTE_EXECUTION_ENDPOINT` only.
- `⌨️command/🟦️.ts:206`: `configured:endpoint!==undefined`. With no env var it is `false`, and `contract`, `scheduler`, `grpc` and `server` all stay `null`.
- `⌨️command/🟦️.ts:183` also derives the scheduler from that same env endpoint plus `policy.server.workerPort` (8981). It ignores the topology ports that `remoteExecutionPortsV1` takes from the group declaration (`🛰️sidecar/🧰️shared/🟦️.ts:20-24`), so even with the endpoint fixed it could probe the wrong worker port.
- The fleet path is different and correct. `🛰️sidecar/🧰️shared/🟦️.ts:33-38` (`remoteExecutionSchedulerEndpointV1`) takes the env var, then `readFleetExecutionV1` from `🌐️remote-cache/🛰️fleet/🟦️.ts:148`. `🚢️status/🟦️.ts:14` and `:19` (used by `repo fleet status` via `🚢️fleet-status/📥️readers/🟦️.ts:129-130`) use it.
- The publisher is `🛰️sidecar/🗓️scheduler/🟦️.ts:38` (`setFleetExecutionV1`), which writes `execution` into `<store>/v1/fleet/cache.json` (`🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts:80-83`). It clears the entry on shutdown (`:50`).
- Test gap: `⌨️command/🧪️tests/🟦️.ts:82` only asserts `configured:false` for an empty env. No test has a fleet-published endpoint, so the bug is not caught.
- Verdict: `status` is stale and disconnected from the fleet scheduler. It is not a missing deployment. The fix is one shared resolver (use `remoteExecutionSchedulerEndpointV1` plus the declared ports and the fleet credential) for `status`, `fleet status`, offload and Bazel.

## 2. Windows "POSIX action runner" gap

What the action runner is: the worker runs only the declared rustc unit. `🧱️actions/🟦️.ts:51-56` (`validateRemoteExecutionRustcActionV1`) requires the command to be exactly `/bin/sh -c <policy.script> semio-rustc <args>`. `policy.script` is a POSIX sh script (`🚀️offload/🧬️schema/🔣️.json`, `script` field: `export`, `eval`, `exec`, `$root`, `printf`). Other checks are the allow-list of rustc arguments, the environment deny-list (`:17-18`) and the two virtual output paths (`:57`). The worker calls it at `👷️worker/🟦️.ts:197`.

Where Windows is refused (three layers, the first decides):
1. Static platform gate, not a capability probe: `🌡️load/🔎️why/🟦️.ts:30-31` (`remoteExecutionActionPlatformWhyV1`, returns `action-unsupported` for `actions==="rustc"&&platform==="win32"`). The text at `:15` says "requires a POSIX action runner on this version of the worker".
2. `🛰️agent/🟦️.ts:29`: `workerEnabled=options.worker!==false&&platformWhy===undefined`. So on Windows no worker starts and nothing is admitted. `:50` reports `headroom:false` with the why. This matches the fleet view: `worker: absent`, why `action-unsupported`.
3. Requester side, `🚀️offload/🟦️.ts:51`: `if(process.platform==="win32")return settle(local("not-portable","windows"))`. A Windows machine never offloads.

What Windows would need (none of it exists):
- Process runner without `/bin/sh`: a direct spawn of `rustc.exe` (the name is already in `policy.tool.windowsExecutable`), with response-file handling for long argument lists, and a validator for that form. `🧱️actions/🟦️.ts:22` (`relative`) rejects drive-letter paths, and `:54` hard-codes `/bin/sh`.
- Path mapping: the offload plan and `offloadRealTextV1` (`🚀️offload/🗺️paths/🟦️.ts`) use `/` virtual paths (`.semio-offload/...`) and `--remap-path-prefix`. Drive letters and `\` separators are not mapped.
- Sandbox: `🔒️sandbox/🟦️.ts:78-93` gives Windows only `isolated-root` (no confinement). `👷️worker/🟦️.ts:310` throws when `confinement=required`, and `🩺️readiness/🟦️.ts:13` reports `sandbox-unavailable`. Policy `worker.confinement` is `best-effort` (`🔣️policy.json`), so this does not block; a Job Object or AppContainer would be needed for real confinement.
- Toolchain identity: `x86_64-pc-windows-msvc` host triple and `rustc -vV` parsing (`🚀️offload/📜️action/🟦️.ts:28-31` already works on text, so it is probably portable).
- Placement: design decision 7 says the single Windows machine shares with nobody until a second Windows machine exists. So the refusal is a placement limit, but the message wrongly blames "this version of the worker". It should be a probed capability (is a POSIX shell present or not), not a hard-coded platform check.
- Existing Windows-aware code in the worker: PATHEXT lookup (`👷️worker/🟦️.ts:131`), `taskkill` tree kill (`:125`), `windowsHide` spawn (`:150`). A start exists.

## 3. REAPI v2 gRPC server and the Bazel path

Not dead, and not a duplicate. The REAPI v2 server (`🖥️server/🟦️.ts:84-123`: Execute, WaitExecution, ActionCache, CAS) has two real clients:
- The rustc offload: `🚀️offload/🔌️client/🟦️.ts:19` (`executeOffloadV1`, `Execution/Execute` with `semio-offload=if-idle`) and `🚀️offload/🟦️.ts:78` (ride-through). Endpoint from env or fleet facts (`🚀️offload/🟦️.ts:55`).
- Bazel: `🔧️tools/🏗️bazel/🏃️execution/🟦️.ts:68` adds `--remote_executor`, `--noremote_local_fallback`, `--remote_instance_name`. It reads the env var only (`bazelRemoteExecutionV1`), not the fleet endpoint.

So the map from 11-13 is stale: it says "nothing else submits work", but since then the rustc offload is a REAPI client, wired into `⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts:139-148`.

Duplication to resolve: scheduler and endpoint resolution. `🗓️scheduler/🟦️.ts` (in-memory REAPI scheduler) is the one real scheduler. `🛰️sidecar/🗓️scheduler/🟦️.ts` only wraps it and publishes the endpoint. Endpoint resolution exists in three forms: fleet (`🛰️sidecar/🧰️shared/🟦️.ts:33`), offload (`env || fleet`, `🚀️offload/🟦️.ts:55`) and status/Bazel (env only). Recommendation: unify to one resolver (section 1). Bazel with `--noremote_local_fallback` (`🔧️tools/🏗️bazel/🏃️execution/🟦️.ts:68`) fails hermetic actions when the executor is down. That conflicts with design rule 4 ("unreachable never fails a build"). It is only active when the env var is set.

## 4. Offload robustness

Good:
- Every phase has a timeout: connect 3 s, queue 8 s, execute 25 min, upload 90 s, download 120 s, ride-through 20 s (`🔣️policy` in `🚀️offload/🧬️schema/🔣️.json`, `timeouts`). Client phases are at `🚀️offload/🔌️client/🟦️.ts:23-25`.
- Any fault becomes a reason and the unit compiles locally: `offloadRustcUnitV1` never throws (`🚀️offload/🟦️.ts:40-111`). Transient errors are retried with jittered back-off (`🔌️client/🟦️.ts:96-105`).
- Cancellation: SIGINT and SIGTERM abort the controller (`⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts:139-146`), and the client closes the stream, which cancels the remote action (`🔌️client/🟦️.ts:21`, `:48`). Worker kill on abort: `👷️worker/🟦️.ts:152`.
- Output is checked for missing files (`📜️script.ts:173`) and path-dependent outputs are rejected (`🚀️offload/🟦️.ts:99`).

Gaps:
- Upload timeout does not stop the work: `🚀️offload/🟦️.ts:72` races `buildOffloadActionV1` against a 90 s timer and falls back to local, but the upload keeps running. It should share an abort signal.
- No progress reaches the caller: `executeOffloadV1` has an `onStage` hook (`🔌️client/🟦️.ts:19`, `:32`), but the call at `🚀️offload/🟦️.ts:78` does not pass it. Progress is only the append log `x/offload.progress` (`:30-38`).
- Timeout mismatch: client execute waits 25 min (`1500000` ms), action timeout is 20 min (`actionSeconds: 1200`, `🚀️offload/📜️action/🟦️.ts:61`), server default and maximum are 30 min (`🔣️policy.json` `remoteExecution.server`). Harmless but inconsistent constants.
- Error classification by regex: `🚀️offload/🟦️.ts:28` treats stderr such as "couldn't read `" as infrastructure, so a real compile error is recompiled locally (double work).
- Scheduler state is in memory (from the 11-13 map, not re-verified).
- Policy drift: `maximumAttempts` is 2 in `🔣️policy.json`; the map said 3.

## 5. Tests

Run with `/Users/ueli/.bun/bin/bun test ./🟦️.ts` from each test dir (read-only, temp dirs only):

| Suite | Result |
|---|---|
| `🧪️tests/🔎️why` | 10 pass, 0 fail |
| `🗓️scheduler/🧪️tests` | 8 pass, 0 fail |
| `⌨️command/🧪️tests` (full) | 6 pass, 1 fail: "serve starts the store, the service and a worker process..." timed out at 120 s. The status tests pass. |
| `⌨️command/🧪️tests` `--test-name-pattern status` | 1 pass |
| `📡️wire/🧪️tests` | 8 pass, 0 fail (the map's `protobufjs` error is gone) |
| `🧱️actions/🧪️tests` | 5 pass |
| `🔒️sandbox/🧪️tests` | 5 pass |
| `🗓️scheduler/⚖️placement/🧪️tests` | 6 pass |
| `🧪️tests/🔎️refusal-reason` | 3 pass |
| `🧪️tests/🔧️bazel/🧭️flags` | 2 pass |
| `🛰️agent/💽️disk/🧪️tests/🪟️volume` | 2 pass |
| `🚀️offload/🧪️tests/🧰️toolchain` | 4 pass |
| `🚀️offload/🧪️tests/🎯️determinism` | 5 pass |
| `🚀️offload/🧪️tests/🧰️workspace` | 0 tests |

Not run: `🚀️offload/🧪️tests/🟦️.ts` and its cluster, signed-fleet and fleet-store tests, the `🛰️sidecar` suites (about 50 files), the top-level `🧪️tests` cluster and scenario suites, and the Bazel integration test. They start clusters or need cargo or Bazel.

The map's "command status test is stale" claim is no longer true: `apiVersions ["2.1","2.3"]` passes.

## Next actions (for a landing, not done here)
1. Route `status` through `remoteExecutionSchedulerEndpointV1` and the declared ports, and add a test with a fleet-published endpoint.
2. Turn the Windows refusal into a probed capability, or build a Windows runner (section 2).
3. Stop the upload work on timeout and pass `onStage` into the offload progress.
4. Decide whether Bazel's `--noremote_local_fallback` is acceptable, given design rule 4.
