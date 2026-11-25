# F42 result: pid-only liveness converted to the shared identity (partial)

NOT VERIFIED BY TESTS: host load stayed 47-125 for the whole lane (gate 45), so no `repo test-file` ran. Verified: `bun build --no-bundle` syntax of every touched file, dangling-imports exit 0 on every touched folder. Run the files listed in section 4 first once load < 45.

## 1. Shared module (`🧰️framework/🔨️modules/🏃️process/🔖️identity/`)
- `requireProcessIdentityV1(value, where)`: hard `ProcessIdentityRecordErrorV1` unless pid and startMs are integers >= 1. `processIdentityVerdictV1` is strict (no pid-only path). Schema `ProcessIdentity.startMs` minimum 1.
- `parseProcessIdentityV1` (undefined instead of throw) for transient claim files that age out when unreadable; durable records use `require` (hard error).
- `processHolderAliveV1(holder)`: raw table check, then creation-time compare, 2 s positive memo (one `ps` per holder per 2 s), unreadable table = alive. `ownProcessIdentityV1()` (OS-read, cached), `processIdentityForPidV1(pid)`, `processPidExistsV1` (the single raw `kill(pid,0)`, for processes this caller spawned).
- Law test `🔖️identity/🧪️tests/🔒️law/🟦️.ts` + baseline `🔣️.json`: counts `process.kill(<pid>, 0)` in production files (git grep, fs-walk fallback) outside the identity module; a file may not gain one, a converted file must leave the baseline; independent Python regex scan cross-checks the counts. Tests are exempt (they observe child death).

## 2. Sites converted (16 direct `kill(pid,0)` sites plus the records behind them)
1. unit-cache `🔒️lock` (claims, slot/light-slot files, tickets: `{pid,startMs,token,atMs}`; `processExistsV1` removed, `processAliveV1(holder)` = identity + not zombie; `claimHolderV1` returns startMs). Consumers: `🧵️lane` live markers (now atomic JSON `{pid,startMs,startedMs}`, judged from content, not from the file name), `🧹️gc/🏦️headroom` records, `🧹️gc/🗑️temporaries` holders (record without startMs = unknown holder, never deleted), remote-execution `🚀️offload/🚦️admission` ticket judgement.
2. cas `📌️pins` (schema `PinFile.startMs`; a pin without it makes `readCasPinsV1` throw, collection stops, nothing is deleted) and `🧹️collector` gc.lock.
3. graph `🚏️admission` nx daemon resident record (schema `Resident.startMs`; writer `🚦️service` uses own identity; `processAliveV1` export removed).
4. shared-worktree: replica `daemon.lock` (schema `Lock.startMs` replaces `startedMs`; own-pid special case gone), `🛟️supervisor` beat (schema `Beat.startMs`, parent watch by parent start time, unreadable parent never kills) and `🛡️resilience.readBeatV1`, `🧱️consolidate` lock (`owner.json`), `📍️relocate` (hard error on a lock without startMs), `📒️journal` staging names `<origin>-<pid>-<startMs>-<hex>` (legacy name forms dropped), `🧷️sidecars` record (schema `Record.startMs`; `processLivesV1` replaced by `sidecarRecordLivesV1`; writer in `🌐️remote-execution/🛰️sidecar`).
5. remote-cache `🧹️collector` lock files (3rd line `startMs`) and `💾️store` `.tmp/<pid>-<startMs>-<uuid>`.
All fixtures of these records in the tests were rewritten by hand.

## 3. NOT converted (63 files, listed with counts in `🔖️identity/🧪️tests/🔒️law/🔣️.json`; regenerate with `T/🧰️scripts/F42/baseline.py`)
Open: leases (`🔒️leases` 657 queue tickets, `🧭️order/🗂️registry` entries already carry `startedAtMs` = runtime time origin, not the OS creation time, so they need a `startMs` field, `📈️standing/🤝️yield`), host-capacity (gate status/asks/warden loop, disk-recovery), remote-cache/execution worker + sidecar ready/graph read, graph resident (`📜️journal`, `🧹️abandoned`, host/service/exit-log `.mjs`), graph generations `.mjs` (pin, claims, row-log, door-board, file-ports), project-farm/owner-pool/take/script-inputs, bazel sessions, cache-gc runner, doctor ports, agent-sessions, graph-state (replica/cursor/entry-lane), normalization transaction-lease chain, test run-lease, native-build, termination, sandbox, activation lease, local-hub, vite agent bridge, teaching cast, hub bootstrap, storage-budget closures/funding, bootstrap script + dependency collection, replica `🥾️entry` `.mjs` (dependency-free bootstrap, parent watch; needs a plain-JS twin of the identity module).
Many are children spawned by the caller (waiting for a process just signalled): for those `processPidExistsV1` is the intended pid-only check once the baseline is cleaned.

## 4. Test files to run first (one at a time, load < 45)
identity `🧪️tests/🟦️.ts` and `🔒️law/🟦️.ts`; unit-cache `🧪️tests/🧟️zombie`, `🪶️light-slot`, `🧹️gc/🧪️tests/🫀️live-holder`, `🧹️gc/🏦️headroom/🧪️tests`, `🧹️gc/🗑️temporaries/🧪️tests`, `🧹️gc/🧱️units/🧪️tests`, `🛰️fleet/📮️registry/🧪️tests`; offload `🧪️tests/🟦️.ts`; cas `🧪️tests/🟦️.ts`, `📌️pins/🧪️tests/🟦️.ts` (+ `🐍️.py`); graph `🚏️admission/🧪️tests`; shared-worktree `📒️journal/🧪️tests`, `🛡️resilience/🧪️tests`, `🔁️replica/🧱️consolidate/🧪️tests`, `🔁️replica/🧷️sidecars/🧪️tests`, `🔁️replica/🧪️tests/{🟦️,🛡️resilience,🛟️boot,💥️killpoints}`; remote-cache `🏷️retention`, `🖥️server`, `🧹️collector` tests.

## 5. Process notes for the coordinator
- Locks: only `cas` was locked; unit-cache, graph admission, replica and remote-cache edits landed without their area lock (edited before applying the rule). Edits were anchored and atomic; no restart-sensitive Cargo/package edit.
- Behaviour change to expect: old-format records (pid without startMs) are reaped as unreadable (claims, markers, resident records) or are a hard error (cas pins, relocate); holders rewrite on their next start, which the supervisor restart on code change triggers.
- Cost: first `ownProcessIdentityV1()` per process is one `ps` (about 1 s `powershell` on Windows), then cached; holder checks are memoized 2 s.
- `🪜️step-overdue` failure from F41 untouched.
