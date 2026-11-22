# repo show project hang

## Findings
1. Bug (fixed): `withBuildCapacityV1` (host-capacity/🟦️.ts) passes the tool-stripped arguments to `buildCapacityAskV1`. `parseBuildInvocationV1` removes `--graph-policy=...`, so `sealedCurrentReadV1` never saw the explicit flag (only the env var form worked). A sealed `show --graph-policy=current` was therefore admitted as a normal `show` (3072 MB grant) and, because C: was ~97-100 % full, `admitBuildDiskV1` ran the disk-recovery collectors (cargo-incremental gc etc.) for ~100 s before the command started. Fix: the parsed `graphPolicy` is handed back to the ask via `SEMIO_BUILD_GRAPH_POLICY` in a copy of the environment. Sealed current reads now skip capacity and disk admission.
2. Not caused by fleet admission: the `build:source-admission` lines are the live-mode source capture of the tool itself (`implementationModeV1` default `live`, 20-45 s on this contended, nearly full disk). It runs for every command, including queries.
3. Not fixable in the query path: a bare `repo show project X` / `repo graph` uses policy `fresh` (documented default; README says only explicit `current` is a sealed read). This machine holds no sealed generation ("graph: this machine holds no sealed generation..."), so fresh must infer the whole 4785-project graph once. `--graph-policy=current` refuses with a typed message instead.
4. Environment: C: had 0-5 GiB free; lease store 93-98 % full with exclusive lease sweeps blocking `resource-directory-admission:v1` for seconds; several concurrent builds (trinity-jack check/build, cache gc).

## Verification
- Before: `repo show project @semio-tech/trinity-jack-rs --graph-policy=current` 103 s (disk recovery + capacity admission), default policy >100 s (timeout).
- After: same command 84 s total, no capacity/disk admission lines; remaining time is live source capture (~45 s) plus lease sweep and resource-directory admission under load, ending with the typed no-sealed-generation refusal.
- Test: sealed-read test extended (second test, flag form passes the door with no handle, no output, disk port never touched). Run with `SEMIO_TEST_PYTHON_BINARY` set: 2 pass.

## Files changed
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🔓️sealed-read/🟦️.ts

## Rebuild
Not needed: the installed `repo.exe` only launches the live TS tree (default mode live), the fix took effect immediately.

## Open follow-ups
- Seal a generation once (`repo graph --graph-policy fresh`, ideally when disk is freed) so `current` answers instantly.
- Free disk on C: and consider `SEMIO_IMPLEMENTATION_MODE=sealed` to avoid per-command source capture.

## Follow-up: `Closed host capacity Policy required: /swap/activeBasisPointsPerSecond is required`
- Cause: a transient schema/policy mismatch in the live tree, not a Windows probe gap. The closed `Policy` definition (`🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json`) gained the required `swap.activeBasisPointsPerSecond` (with `outBasisPointsPerSecond` and `shortBasisPoints`) at 03:05, while `🔣️policy.json` of the repo build only got the field at 03:12; another session is reworking the swap admission rule (see the swap diff against HEAD). A `repo run` that loaded the pair in between refused every command. After 03:12 the shipped policy validates (verified by reading `BUILD_HOST_CAPACITY_POLICY_V1`, swap = 2500 / 0.6 / 40 / 2500).
- Not a code defect in the reader, so no source change; the gap is that nothing pinned the pair together. Added a language-agnostic contract test `🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧷️shipped-policy/🟦️.ts`: the shipped policy must validate against the closed Policy definition with ajv (third party) and with our reader, and its swap keys must equal the schema's required list. 1 pass.
- Re-run `repo run` after the active edit settles; the queue of >30 min was the graph service waiting on an unsealed graph (see above), not on this policy.
- Source capture also races concurrent edits: `repo graph --graph-policy fresh` failed once with `ENOENT lstat ...🕸️graph/📥️inference/📜️contract-reads` (a directory renamed mid-capture). It is retried up to three times in the background to seal the graph.
- Correction (03:28): the ENOENT is not a race. `🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs` (edited 03:12 by another session) imports `../📜️contract-reads/🟨️.mjs` (`GENERATOR_CONTRACTS_FACT_V1`, `generatorContractReadHoldsV1`, `generatorContractsInScopeV1`, `ownerRecordHeadV1`, `trackGeneratorContractsV1`, `withGeneratorContractsScopeV1`; `👁️observation/🟨️.mjs` references it too). That module does not exist anywhere in the tree, so the live source capture of every command fails with `Internal error: ENOENT lstat ...📜️contract-reads`. Three sealing attempts failed identically. I did not invent the module: it belongs to the session adding it. Until it lands (or the import is reverted) no graph can be sealed and `repo run` cannot start, so `repo run` of a small target was not verifiable. Re-run `repo graph --graph-policy fresh` once it exists.

## Seal retry (03:43 to 04:00)

- `analysis-store` module landed (another session); `Cannot find module` error gone.
- `repo graph --graph-policy fresh` was killed by the 500 s timeout (exit 143): no sealed generation.
- `repo show project @semio-tech/trinity-jack-rs --graph-policy=current` fails after 39 s: this machine holds no sealed generation, and current policy never starts inference.
- Seal queued behind another process (40484) for `repository-cache-charge` (exclusive) during the run.
- Not yet verified: `repo run @semio-tech/trinity-js:test`.

## Seal retries with long timeouts (04:00 to 05:16)

- 30 min run: timed out (exit 124); its log was deleted when the ticket `🗑️generated` folder was cleaned.
- 50 min run (log kept in scratchpad): timed out (exit 124) at 05:16. 174 "Another process is building the dependencies" waits (up to 749 s+), phase `build:dependencies`, source scan counters at about 196k operations. About 5 repo.exe processes were alive.
- Conclusion: the seal does not finish while another process holds the dependency build; no repo code defect was identified from the log. Still no sealed generation, so `repo show ... --graph-policy=current` and `trinity-js:test` remain unverified.
- Next: find which process owns the dependency build (leases_status), let it finish, then rerun the seal.

## Who holds the dependency build (05:20 to 05:26)

- Holder: `javascript-dependency-build:C:\semio` exclusive, pid 40484 (bun, child of 13436, child of repo.exe 19632 `dependencies sync --all`, started 03:45 by bun 39140 from another session). Held 3600 s, phase `dependencies:store-stage`, priority agent.
- Liveness: the lease's standing state is `progressing`; `silentSeconds` oscillates 8 to 69 over 90 s, so heartbeats arrive. The holder is not dead or orphaned (parent chain alive), so no reclaim rule applies and nothing was killed.
- But the work itself is nearly idle: 0 bytes read and 2.5 KB written in 15 s, 0.45 CPU-s per 20 s, all threads waiting, no child processes. 428 CPU-s in 5000 s of life. The stage advances in 64-unit batches (`progressWorkUnits`) with a publish plus `yieldContinuation` per batch, so the heartbeat can stay alive while throughput is very low.
- Existing stall rule (`🐕️stall`) covers only the bun install child (probe mtimes, 300 s), not the store-stage, so a heartbeat-alive but throughput-starved holder blocks all graph seals indefinitely.
- Open question: what the store-stage waits on while idle (no lease waits: 0 waiting, 0 deadlock cycles). Needs a profile of pid 40484 or the stage code in `📦️dependencies/💾️storage/🟦️.ts` (line 520).
- Our own seal (graph, pid 27292/35564) is queued behind it; not killed.

## Profile of the idle store stage (05:27 to 05:40, time-boxed, no fix landed)

- Store stage code (`📦️dependencies/💾️storage/🟦️.ts` 520): per file it does `charge()`, `advance()`, then fsyncs the file, its parent and every new directory; every 64 work units `advance()` awaits `control.publish(...)` plus `control.yieldContinuation()`. The heartbeat (`silentSeconds` 8 to 69 s) is therefore proof of work-unit advance, about one batch of 64 per minute.
- Holder pid 40484 shows ~0 read, ~2 KB written per 15 s, all 23 threads waiting, no child processes. Store generations under `.cache/b/d/m/j`: several empty or tiny (a42d, c38d, ec2b empty since 04:54; 9c3e 70 files created 05:26) beside completed ones of 54k to 59k files; the stage steps through them at roughly one per 30 min.
- Most likely idle cause: per-file/per-directory fsync plus `yieldContinuation` pacing on a machine running dozens of cargo/rustc jobs (disk flush latency, not CPU). Not confirmed: the production `yieldContinuation` implementation was not located (test and capacity stubs are `setImmediate`), and no stack of the bun process could be taken.
- Not done: the idle-wait fix and the throughput stall rule (needs measured work-units per window from the standing record, a schema constant, and a test beside `🐕️stall`). Nothing killed; seal and the two checks still pending.

## Durability redesign check (05:40 to 06:05): premise not supported, no code changed

- Measured with `fsync-bench.mjs` (2 KB files, same disk, machine under cargo load): 300 files with per-file fsync 837 ms (2.8 ms/file); 300 files with one commit fsync 111 ms (0.37 ms/file). Per-file fsync is 7.5x slower but costs milliseconds, not tens of ms.
- The stage at `💾️storage/🟦️.ts:520` writes only the generation input files (bun.lock, bunfig, package.json, patches: 70 to 314 files per generation as seen under `.cache/b/d/m/j`), not the 55k-file `node_modules`. Even 314 per-file fsyncs plus directory fsyncs cost about 1 s, and 55k fsyncs would be about 150 s per generation, so fsync cannot explain one generation per 30 min.
- `yieldContinuation` of the host scheduler (`⏳️async/🪃️continuation`) is a macrotask post, not a throttle; custody and control checks are in-memory map lookups. No pacing wait found in the stage.
- Holder pid 40484: CPU 428 s at 05:20, 510 s at 06:00 (82 s in 40 min), no child processes, no installer process alive. It is neither hot-looping nor waiting on a lease (0 waiting). The `dependencies:store-stage` phase may simply be the last published phase, stale while the process waits elsewhere (an awaited promise or an external IPC reply); not identified without a stack of the bun process.
- Therefore a per-generation commit-manifest redesign would not address the observed delay. Next step needs a stack/inspector sample of pid 40484 (e.g. start the next `dependencies sync` with `bun --inspect` or add [DEBUG] timing around each await in the stage) to find the awaiting call.

## Open

- The `repo dependencies sync --all` chain started 03:45 (pids 19632, 13436, 40484) still holds `javascript-dependency-build` exclusively while nearly idle; every graph seal queues behind it. Not killed (another session's work).
- Next diagnostic step: `[DEBUG]` timing around each await in the store stage of `📦️dependencies/💾️storage/🟦️.ts` (around line 520), or run the next sync under `bun --inspect`, to find the awaiting call; then fix that call with a language-agnostic test.
- Still unverified: sealed graph, timed `repo show project @semio-tech/trinity-jack-rs --graph-policy=current`, `repo run @semio-tech/trinity-js:test`.
- Cleanup 06:10: stopped my own seal roots (timeout 42680 chain and the 8-attempt loop sh 8616) with taskkill /T.
