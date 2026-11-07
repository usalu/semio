# 1105b audit aud-act22: activation runbook (act22)

UPDATE (actinfra, 11-05 evening): items 1-2 below are superseded by the section "FINAL freeze plan (actinfra)" at the end of this file; the single-component proof is GREEN (1105b-result-actinfra.md).

Read-only audit. No source or tool file edited, no cargo or repo build run. Probes: markers, df, lsof, ps, git grep on tracked files.

## Verdict

NOT READY TO START act22. Preflight items 1 (wit lane red), 2 (single-component proof not run), 6 (slots not paused: slots.k=6, slots.wextra=2) and 7 (fleet cargo and an e2e `repo run` in flight) are open. Markers, disk, port and tooling are fine.

## (1) Commands in order

Log directory: `C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`, `G=$C/🗑️generated/1105b`. `repo` is `/Users/ueli/.local/bin/repo`, `bun` is `/Users/ueli/.bun/bin/bun`.

Step 0. Gate the fleet (coordinator, before anything else)
- `printf 1 > $C/1105b-tools/slots.k; printf 0 > $C/1105b-tools/slots.wextra` (current values 6 and 2).
- Wait for queue `$G/slots/queue` to drain or stop the e2e lane. Pid 40773 (`repo run @semio-tech/mit-bestand-demonstrator:test`, 1 h 53 min old) and its slot wrapper pid 23060 are still running. Kill the wrapper and its children together, not the wrapper alone (orphaned cargo).
- Check `ps -eo pid,etime,command | grep "[r]epo "` shows no cargo lane before step 1.

Step 1. Wit gate (preflight 1)
- `cd /Users/ueli/Documents/semio && bun test ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🧬️wit-lowering/🟦️.ts ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🧪️tests/🟦️.ts > $G/act22-wit-bun.txt 2>&1`
- Rust side (must also pass): the wit lane reports `semio-os-store-replay` (1 error) and `semio-os-store-space-history` (51 errors) red. Store lane's `store-1` says rc 0. Re-check both with the lane's chk wrapper before step 2 (`$C/1105b-tools/slot.sh W-act22-wit env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --target wasm32-wasip2 -p semio-os-store-replay -p semio-os-store-space-history`). Do not start activation while these are red.

Step 2. Single-component proof (preflight 2, not run yet)
- Find the flow-list component project name with `repo show-projects` (not located statically here).
- `$C/1105b-tools/slot.sh W-act22-comp env SEMIO_LANGUAGE=en repo run <flow-list-component-project>:component > $G/act22-comp.txt 2>&1`
- Then the stage driver: `$C/1102-tools/actinfra/stages.sh` (exists, 356 B). Expect compile 14-25 min, stages about 2 min, descriptor `flow-extension-list`.

Step 3. Activate (act.sh; logs to `$G/act22-activate.log` and `.status`)
- `nohup $C/1105b-tools/act.sh act22-activate >/dev/null 2>&1 &` (detached; act.sh runs `repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype` with `SEMIO_PROGRESS_DEADLINE_TRACE=1 SEMIO_LEASE_PRIORITY=interactive SEMIO_LANGUAGE=en`).
- act.sh does not go through slot.sh, so the fleet gate in step 0 is manual.
- Poll with Monitor for `exit N` in `$G/act22-activate.status`. Do not poll the log in a loop.
- Success marker: `$G/act22-activate.status` ends with `exit 0`; `/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/dist/♻️activation/prototype` exists (today absent).

Step 4. Serve on 127.0.0.1:6029, detached with nohup
```
G=$C/🗑️generated/1105b
cd "/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator" && \
MIT_BESTAND_DEMONSTRATOR_PORT=6029 SEMIO_BUILD_MODE=prototype NODE_OPTIONS= \
SEMIO_BUILD_DYNAMIC_OUTPUT=false SEMIO_BUILD_TERMINAL=false \
nohup /Users/ueli/.local/bin/repo ./🔨️modules/🧩️runtime/📜️script.ts serve > "$G/act22-serve.log" 2>&1 < /dev/null &
echo $! > "$G/act22-serve.pid"
```
- Host is 127.0.0.1 unless `DEVCONTAINER=true` (`📜️script.ts:57`). Do not set DEVCONTAINER.
- Vite race fallback (act14-era scheduler/with-selector "no export default"): kill the pid, delete the vite cacheDir (computed by `viteCacheDirectory` in `🏗️builder/🌐️vite/🟦️.ts:55`), restart once.
- Alternative (supervised, auto-recycles a wedged listener): `$C/🔁️serve-supervisor.sh` (uses bun instead of repo; log `$C/🗑️generated/serve-6029-supervised.txt`).

Step 5. Probe
- `curl -s -o /dev/null -w '%{http_code}\n' --max-time 45 http://127.0.0.1:6029/` expect `200` (a cold landing boot can exceed 15 s).
- `lsof -nP -iTCP:6029 -sTCP:LISTEN` shows one listener, and `grep "Demonstrator ready" $G/act22-serve.log`.

Step 6. Browser verdict (harness)
- `$C/1105b-tools/e2eready/run.sh http://127.0.0.1:6029/` writes `$G/e2eready/run.log` and `$G/e2eready/run.rc` (run.sh creates the directory). Chromium is at `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` (present). Harness pre-check `run.sh --check` was 8/8 green in the e2e lane.

Step 7. test-e2e
- `(nohup $C/1105b-tools/slot.sh act22-test-e2e repo run @semio-tech/mit-bestand-demonstrator:test-e2e > $G/act22-test-e2e.txt 2>&1; echo $? > $G/act22-test-e2e.rc) < /dev/null > /dev/null 2>&1 &` (same pattern as the e2e lane; poll `$G/act22-test-e2e.rc`).
- Caveat: `test-e2e` dependsOn `serve-e2e` -> `prepare-e2e` (mutates host store, `prepare-test`). `serve-e2e` binds port 0 (`🔨️modules/🧩️runtime/📜️script.ts:82`), so it does not collide with 6029. Its results do not drive the 6029 server; step 6 is the 6029 check.

Teardown: `kill $(cat $G/act22-serve.pid)`, then confirm port 6029 is free. Restore `slots.k`=6 and `slots.wextra`=2 after the window.

## (2) Preflight items and current status

| # | Item | Status now | Evidence |
|---|------|------------|----------|
| 1 | wit lane green (wasm check + bun tests) | RED | `1105b-result-wit.md`: TS proven, Rust unverified; 51+1 foreign errors in store-space-history / store-replay. Bun tests not re-run here (paths: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🧬️wit-lowering/🟦️.ts`, `…/🛂️descriptor/🧪️tests/🟦️.ts`) |
| 2 | Single-component proof (flow-list, stages.sh) | NOT RUN | `1105b-result-actinfra.md` task 2 |
| 3 | Build-dir prefix <= 55, no Source cap fault | GREEN (prefix 53 statically; `admission-ok` marker present) | `.cargo/config.toml` build-dir `.cache/b/r/m/u/l/<lane>`; `pathlen.ts` in `1105b-tools/actinfra` |
| 4 | dependencies lock converged, CRATE_NAME, build-owners | GREEN for markers `lock-ok`, `sync-done`; `CRATE_NAME="semio-plugin-describe"` at `🖨️describe/🏗️component-build/🟦️.ts:13`; build-owners.json not checked | markers present |
| 5 | Disk >= 100 GiB free | GREEN, 165 GiB free (`df`, was 207 at actinfra report) | `df -h` |
| 6 | Pause cargo lanes (slots.k 1, wextra 0) | OPEN: slots.k=6, slots.wextra=2, slots s2-s6 held | `1105b-tools/slots.k`, `slots.wextra`, `$G/slots/` |
| 7 | No repo cargo on shared lanes during activation | OPEN: about 18 repo/cargo processes, load avg 16-18 | `ps` |
| 8 | Browser readiness (e2eready run.sh) | GREEN for harness (8/8 check), browser paths UNVERIFIED | `1105b-result-e2e.md` |

Markers in `$G` (all present, empty files): admission-ok, storage-ok, lock-ok, sync-done, declapp-ok, value-ok.
Port 6029: free (lsof returns no listener).
Expected absent before act22: `dist/♻️activation/prototype`, `.cache/b/p/m/browser/prototype/runtime/react`. Both absent.
Tooling present: repo, bun, nohup, lsof, curl.

## (3) Past activation failure modes (act14-act21) and fix status

| Act | Failure | Fix | In tree? |
|-----|---------|-----|----------|
| act14 | graph refused `build.test.aggregate-refused` (suite javascript-input-identity had no target) | target `test-javascript-input-identity` plus aggregate entry, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json` (lines 234, 2285) | YES |
| act15 | describe infra; node-graph wasm (replay-owned-state); puzzle-plugin wasm (pdf-schema-5, stdio2) | describe: emitter fixes (below). replay: replayfix rc 0. pdf-schema-5 / stdio2 | describe YES; replay YES (store-1 rc 0, but see wit); pdf-schema-5 / stdio2 UNVERIFIED |
| act16 | describe source churn; fonts + puzzle progress deadline under load | operational: freeze edits (hub12), `SEMIO_PROGRESS_DEADLINE_TRACE=1` in act.sh (diagnostic only, no tree change) | operational only |
| act17 | component driver regression from ccac6866cc1 (distinct cargo target flag views) | `requireCompatibleCargoFlagViewsV1` in `🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` | YES |
| act18 | emitter maximumBytes 2149580800 > reader cap 2147483648; Bazel one action at a time | fresh-component uses `authority.transport.maximumBytes` (`🖨️describe/🏭️fresh-component/🟦️.ts:346`); 2147483648 literal absent from framework source. worker-spare-instances patch (`1102-tools/actinfra/patches/worker-spare-instances.diff`) | cap YES; worker-spare UNVERIFIED (no identifier match in tree) |
| act19 | flow-list component describe "Finite unlinked artifact file required" (B0 retention rows); runtime worker_resume_pending refuses forever | norows retention (`retention-intermediates-norows.diff`, message in `browser-bundle/📦️distribution/📋️inventory/🟦️.ts`); worker-demand tests pass (actinfra t1, 31 pass). Continuation-fault patch `1102-tools/runtime/patches/apply-worker-continuation-fault.py` | retention YES; continuation-fault UNVERIFIED (no marker in runtime source) |
| act20 | emitter index.json 3 MB read with 1 MiB manifest bound | `readArtifactBytesV1(index.json, limits.maximumRetainedIndexBytes.const)` at `🖨️describe/🛠️emitter/🟦️.ts:40`, pinned by `🖨️describe/🛠️emitter/🧪️tests/📏️request/🟦️.ts:15-16` | YES |
| act21 | path 259 chars for replication-mutation-retained-field rlib | rename to `semio-framework-replication-retained-field` (Cargo.lock:6645; names.tsv) | YES in Cargo.lock, but `os/🎚️config/🏭️bridge/Cargo.lock` still has 3 lines naming replication-mutation-retained-field: check if that workspace is in the activation closure |

Path budget (act21 fix): worst path 252 chars with prefix 53, under 256.

## (4) Expected duration

- Graph and inference: about 3 min for the driver, about 25 min for graph (actinfra estimate).
- 77 Bazel actions, of which 32 are components plus describe-rs. Component compile 13-60 min each.
- At 2 jobs, roughly 6-9 h. At 1 job, 12 h or more (act21 reached 27/77 in about 1.5 h at 1 job).
- Cold cache (reset today) and current load (16-18) push toward the slow end.
- UNVERIFIED estimate. Serve start after activation is minutes, probe minutes, test-e2e 30-60 min plus dependency re-verification if the receipt is stale (`dependencies sync --all` already done at 11:25).

## Files and paths referenced

- `C/1105b-result-actinfra.md`, `C/1105b-result-graphinf.md`, `C/1105b-result-e2e.md`, `C/1105b-result-wit.md`
- `C/1105b-tools/act.sh`, `C/1105b-tools/slot.sh`, `C/1105b-tools/e2eready/run.sh`
- `C/🗑️generated/1105b/` (markers, logs)
- `C/1028-coordination.md` (act14-act21 entries)
- `♻️mit-bestand/🧺️demonstrator/📋️project.json` (activate-prototype, serve, test-e2e)
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts`

## FINAL freeze plan (actinfra, 11-05 evening)

### A. Preconditions (the coordinator calls the freeze when hub-11 + ext are green)
1. `C/🗑️generated/1105b/freeze` written, then every lane's production edit becomes a patch under `C/1105b-tools/<lane>/patches/`. The graph verification restarts on ANY source edit (gates lane finding; my own runs: a `repo run` that was verifying for 47 min restarted once because a peer edited the dependency generation module, e2e-test-2.txt:26796). The freeze must therefore hold from `act.sh` start until the first `[1 / 77]` line, and should hold until exit (an edit of a component's inputs invalidates that Bazel action).
2. Slot policy: `printf 0 > C/1105b-tools/slots.k; printf 0 > C/1105b-tools/slots.wextra`, then wait until `C/🗑️generated/1105b/slots/` has no `s*` directory and the queue is empty. Activation itself does not use slot.sh. No lane runs `repo run/show/graph`, `repo cargo` or `repo tools exec` during the window (each loads the same graph/process implementation and adds door load; the door grants the activation only the memory above the 4096 MB floor). Restore `slots.k` / `slots.wextra` after exit.
3. Disk >= 100 GiB free (165 GiB at the last audit), `fseventsd` below ~4 GB (it held 14-16 GB in acts 20-21 and starved the door).
4. Dependency receipt fresh (`sync-done`, `lock-ok` present). Any later `dependencies` change or a change of an installer input (nx patch) makes every `repo run` re-verify the generation for ~45 min.
5. Descriptor channel pin: `CHANNEL_VERSION` / `APP_CHANNEL_VERSION` is 25 (Rust `📡️spr/🧵️channel/🦀️.rs:24`, TS `🧾️app-channel/🔢️request-sequence/🟦️.ts:14`) but 70 tracked JSON files still say `"appChannelVersion": 24`, including the committed flow-list `🔣️.json` (the freshly built flow-list descriptor differs from the committed pack ONLY in `executionProtocol.appChannelVersion` 24 vs 25; sha 6bb239bb... vs d3a51210...). Rebuilt components emit 25; committed descriptors stay 24 until the channel-version owner (`🧑‍💻dev/🔖️channel-version`, consumers/pin) re-pins them. Run before the freeze, otherwise the catalog descriptor-pair validation (`📇️registry/🛂️descriptor-verification`, `📽️projection`) may refuse serve. UNVERIFIED whether activation itself refuses (it only writes new descriptors).

### B. Exact commands (coordinator; `act.sh` sets SEMIO_PROGRESS_DEADLINE_TRACE=1, SEMIO_LEASE_PRIORITY=interactive, SEMIO_LANGUAGE=en)
```
cd /Users/ueli/Documents/semio
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS
nohup $C/1105b-tools/act.sh act22 >/dev/null 2>&1 &     # = repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype
# monitor (Monitor tool or a terminal), prints one line per change and ALARM on restart / new ERROR, exits with the run:
bash $C/1105b-tools/actinfra/act-monitor.sh act22 60
```
`act-monitor.sh <label> [seconds]` reads `🗑️generated/1105b/<label>.log/.status`. Tested on act21's log: shows `verify=[2 of 2] bazel=[[30 / 77] ...] restarts=0 errors=2` and `ALARM new ERROR`, `DONE exit 130`. It does not flag a verification-counter reset (a second pass legitimately starts at 0 after the first closes; my killed run reset 2773 -> 71 without a "Restarting" line): if `verify=` drops while `bazel=[-]`, treat it as a restart loop and decide.

### C. Expected phases (from acts 18-21 and my runs; durations UNVERIFIED for an idle fleet)
| phase | log signal | expected |
|---|---|---|
| capacity door | `[capacity] waiting ...` | 0-30 min; admitted with 1 of 8 jobs when lanes hold memory (act19/21) |
| source/implementation + dependency receipt | `build:source-admission`, `Store dependency generation receipt` | 1-3 min when the receipt is fresh |
| graph input verification | `Verifying input closures: N of ~2840-2949` | 2.8 h observed at load 16-19 (about 200 closures per 10 min); restarts on any edit |
| graph plan / inference | `build:graph-plan`, inference of ~1650 projects | 5-25 min |
| Bazel | `[N / 77]`; 77 actions = 32 components + describe-rs + wasm/support actions (same count in act18-21) | describe-rs ~24 min; a component 14-42 min at 1 core (flow-list: 541 units, 41m44s, 427 MB wasm, then ~75 s of stages); with RepoBuild=2 the 32 components need 6-12 h |
| end | `exit 0`, `dist/♻️activation/prototype` and `.cache/b/p/m/browser/prototype/runtime/react/<lane>/activation` exist | |
Do not wait for a component action to print progress: the Bazel line only advances when an action finishes.

### D. Risks fixed since act21 that act22 now depends on
- Capture of a finished command (owned-execution/📥️capture/🟦️.ts: post-exit loan check removed, `requireCompletedCaptureAuthorityV1`): a component action no longer fails after its cargo build exited 0.
- jco strip-debug, emitter bounds, norows retention, worker demand, 53-char build-dir prefix (worst path 252): see 1105b-result-actinfra.md.
- Open: `debug: 1` wasm profile keeps each component at ~430 MB (patch `prototype-wasm-debug0.py` not applied; the strip guard copes, cost ~23 GB of IO per activation).

### E. publishComponentBrowserV1 (static check against today's descriptor change)
- Path: `driver-stages` `browser` stage -> `publishComponentBrowserV1` (`browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:167`) -> vendor scratch -> `assembleComponentModuleV1` -> `probeComponentModuleV1` (:140, cache stage `probe`) -> sha256 of the probed bytes must equal `receipt.semanticDescriptorSha256` (:157) -> `finalizePluginDescriptor` -> store publication.
- The describe(budget) change is consistent end to end: the probe source (`🛂️descriptor/🟦️.ts:51-63`) builds the same budget as the Rust emitter oracle (`descriptor-emission/🦀️.rs:405-428`: grant 1<<20 items, 64 MiB, fuel 1<<24, deadline 1<<52, operation/generation/epoch 1); the production emitter path uses the owned interpreter's `describe_observed(..., describe_budget(), ...)`; `ACTOR_COMPONENT_EXPORTS` is derived from the WIT files, so the five new exports are required automatically (wit lane updated the corpus).
- Proven on the real artifact (stage3): probe output 10,092 chars, `finalizePluginDescriptor` accepts it, pluginId `flow-extension-list`.
- Not exercised: the store publication itself and the receipt equality (needs the issued invocation), the owned-interpreter emitter on the new WIT (wit lane: host/guest-runtime crates not yet compiled when I checked).
- Weak spot: the `probe` cache key digests the module and vendor files plus node, but not `PLUGIN_DESCRIPTOR_PROBE_SOURCE` or its schemaVersion; a cached probe from before a probe-source change would be reused (stale descriptor bytes would then fail the receipt equality, not silently ship). Bump `schemaVersion` in `probeComponentModuleV1` when the probe source changes. Not changed (production edit, freeze).
