# Closure loader memory of the cargo kernel, 2026-11-26 (agent wrapper-memory)

## Findings
- Lazy per-module payload decode already exists in the loader (`splitCapturedSnapshotV1`, `admitCapturedImagesV1`); it was not the cost.
- The per-rustc stand-in (`rustc-wrapper-<digest>`, unit-cache launch) runs `bun script.ts` directly and does not use the closure loader. The 1.1 GB `bun --eval` helper is the `repo cargo` kernel (`⌨️command/🦀️cargo/🟦️.ts`) running under the stream loader.
- That kernel statically loads 1103 of 1974 captured modules (11.2 MB source, 27 MB snapshot). Cost per module was ~0.16 MB because the loader built a new `Bun.Transpiler` for every module (the five `import.meta` members were its `define`s).
- A `bun` that imports the same graph natively from disk needs 125 MB RSS / 78 MB footprint; an empty loader run 26 MB.

## Change
`🍞️bun/🟦️.ts` (`sourceClosureLoaderV1`): one shared transpiler per module kind (`ts`, `js`); `import.meta.url/path/filename/dir/dirname` now read `__semioMetaV1`, a frozen record the loader prefixes to each module's first line. Constant defines unchanged. Loader text +117 characters (the loader-identity command-line law stays green).

## Measured (cargo kernel snapshot, 1974 files, `--eval` loader, entry loaded, `/usr/bin/time -l`, 4 runs each)
| | RSS | peak footprint |
|---|---|---|
| before | 243-333 MB (median ~283) | 284-327 MB |
| after | 176-208 MB (median ~195) | 194-202 MB |
Synthetic 600-module crowd: 128 MB -> 81 MB RSS (0.158 -> 0.085 MB per module).
Not reached: the 150 MB target. The kernel needs ~1100 modules (launcher alone 706, invocation parser 130); at ~0.085 MB per module the remainder is the graph. Real `repo cargo` run was queued behind the capacity gate for 10+ minutes and was stopped, so no in-flight peak of the compile phase was measured (an earlier sample before the change: 422-479 MB at planning).

## Next lever
Static import cuts inside the launcher (`commands/setup`, commit hooks, windows compiler, native-build; each 15-30 modules) or a dedicated thin kernel entry. `wrapper-memory-graph.ts` and `wrapper-memory-cuts.ts` rank edges by modules saved.

## Tests
- New `🧪️tests/🧠️module-meta` (fixture `🔣️.json` + `🟦️.ts`): language-agnostic file trees; loader output equals fixture expectations and Bun's native loader; 600-module footprint bound (<65 MB growth over 50 modules; old design 87 MB).
- Green: loader-identity, stream (20), selection-independent suites trace, resolution, text, kernels, sites, static, warm, mismatch.
- Failing before and after (same counts, host/environment): `🚪️command/🧪️tests`, `🔏️selection`, `🟢️node/🧪️tests/📥️stdin`, runtime tests needing `hdiutil` / case-sensitive volume.

## Files
Scripts: `measure-descendant-peak.py`, `wrapper-memory-capture.ts`, `wrapper-memory-graph.ts`, `wrapper-memory-cuts.ts`.

## Follow-up 1: growth during a real cargo build (kernel RSS/footprint over time)
Method: `measure-descendant-peak.py` samples every captured loader process under `repo cargo ... check -p semio-framework-value` (RSS via ps, physical footprint via `proc_pid_rusage`), plus a temporary `[DEBUG]` heap hook in the cargo kernel (removed) with two V8 heap snapshots.
- The 1.1 GB is real: kernel footprint 808 MB (cached prototype check), 906 MB (release check), 1203 MB (release check with snapshots). Plus a nested `cache gc --reclaim` loader (590-740 MB) that the kernel starts beside the build and two small `run repo-cache-reset:*` loaders (160-180 MB each).
- It is not a retained-state leak. JS heap retained by the kernel is 155 MB (tick 10) to 184 MB (tick 30): 61 MB strings, 30 MB `Uint8Array` (snapshot and module images), 39 MB plain objects (the parsed plan/unit graph, +650k objects between the two snapshots), 9 MB module records. The JS heap saw-tooths up to 455 MB before each collection and the allocator keeps the freed pages (mimalloc), so the footprint sits at 2-4x the live data and ramps about 1.7 MB/s during a compile. A 7 MB JSON parse loop shows the same shape in isolation: 173 MB footprint, 126 MB with `MIMALLOC_PURGE_DELAY=0`.
- Root cause: the cargo and cache kernels were declared "never capped", so unlike every graph child they ran without the repo's own collector target (`BUN_JSC_forceRAMSize`, 512 MB) and allocator purge (`MIMALLOC_PURGE_DELAY=0`), which the memory policy documents as the remedy for exactly this (garbage sized by the 16-32 GB host).
- Fix: `repositoryCommandMemoryCappedV1` (`⌨️command/🧭️selection`): graph readers plus the kinds `cargo`, `cache`, `cache-ledger` (new `capped` table in the selection schema and fixture). `implementationProcessEnvironmentV1` and the sealed trampoline use it; the cap is inherited by the reclaim loaders the kernel starts.
- Measured (same command, release check, one run each): kernel footprint 906 -> 599 MB (RSS 760 -> 540 MB); nested gc loader 739 -> 401 MB; the two cache-reset loaders 177/162 MB unchanged. Whole helper tree about 1.8 GB -> 1.3 GB. Idle plateau after module load is about 250 MB; more reduction needs fewer loaded modules (below).
- Tests: `⌨️command/🧭️selection/🧪️tests` (new fixture table plus SQLite model), trampoline suite (spawned door carries the cap for cargo and cache, 9 pass), footprint suite (53 pass; its one failing farm test is unrelated), `⌨️command/🧪️tests` kernel-selection test now passes (the same 5 other tests fail with and without the changes).

## Follow-up 2: static import cuts (not applied)
`wrapper-memory-cuts.ts` ranks edges by modules saved. After the launcher itself (583-706 modules, needed), each independent cut saves only 12-34 modules (about 1-3% each): headroom chain under nx graph 34, compiler recovery 28, native-build index 26, setup commit hooks 18, windows compiler 17, test ceiling 12. Seven cuts together reach 828 of 970 modules (-15%, about 15-25 MB). Each needs an async wrapper at call sites inside suites that already fail in this environment, so I could not verify them and left them. A thin kernel entry needs `🚪️command/🟦️.ts` split (it pulls the 130-module invocation parser through selection); several tests read that file as text, so it is a separate ticket.

## Follow-up 3: the nested `cache gc --reclaim` loader
Who and why: `withBuildCapacityV1` (host-capacity door) calls `admitBuildDiskV1` for every Cargo command with an expected growth of 4 GiB. When the cache volume has less than `need + reserve` free (reserve = largest need x factor; true on this 16 GB Mac every time) it judged `collect-background` and started `recoverDiskV1` beside the build with `void`. Its `ledger` writer spawns a full `repo cache gc --reclaim=<bytes>` process (front door plus the 755-module cache-ledger kernel, 401-739 MB) and the `image-slack` writer spawns the `repo-cache-reset` reclaim/regrow loaders (140-200 MB each). So every compile ran 2-4 extra loaders concurrently, whether or not the ledger had anything to collect.
Fix (test-first):
- Cheap trigger: `🧹️cache-gc/📏️pressure` `ledgerOverBudgetAreasV1` reads only the size ledger (11 modules, 0.4 MB closure against 755 modules for the cache kernel) and names the areas above their budget with the existing sweep rule. In background mode the ledger writer spawns `repo cache gc --area=<over areas>` only for those; nothing over budget, no process.
- After, not during: a background recovery is no longer started at admission. `admitBuildDiskV1` returns a once-only `DeferredReliefV1`, `withBuildCapacityV1` runs it in the `finally` of the command body. A recovery below the cushion (collect-now) still runs first and blocks, so refusals and out-of-space handling are unchanged.
- Tests: `📏️pressure/🧪️tests` (JSON table plus SQLite model), two new tests in `💽️disk/🧪️tests` (no collector during admission and once after; ledger collector only for over-budget areas, blocking mode unchanged), and the old background expectation in `🧹️disk-recovery/🧪️tests` now asserts the deferral. Host-capacity disk (10), disk-recovery (23), host-capacity (20) and the cache-gc suites pass. Failing before and unrelated: `🪟️partial`, `🪜️phases`, `🛟️unsatisfiable` (warden/capacity status), two fixture files.
- Measured (same release check, `measure-descendant-peak.py`): before, kernel 599-906 MB footprint plus a concurrent `cache gc` loader of 401-739 MB plus two cache-reset loaders (171/162 MB) during the build. After: the kernel alone (620 MB footprint, 518 MB RSS) while the build runs; the only other loaders are four cache-reset reclaim/regrow loaders (137-200 MB) that start 20 s after the kernel exited, one at a time. No `cache gc` loader at all because the ledger is within budget.
- Not done: the image-slack reclaim/regrow still runs after every build (cheap trigger for it would need the image census; it is outside the ledger and not concurrent any more). A dedicated thin kernel for `cache gc` was skipped: the over-budget path still uses the full kernel, and it is now rare.

## Follow-up 4: the three remaining host-capacity failures are not caused by this work
`🪟️partial`, `🪜️phases` and `🛟️unsatisfiable` (under `🌡️host-capacity/🧪️tests`) contain no reference to disk admission, deferred relief or the collector (grep for `disk`, `relief`, `collector`, `admitBuildDisk` is empty). Their failing assertions are about the warden and the capacity decision:
- `🪟️partial` and `🪜️phases`: `repo capacity status` now prints an extra "Memory guard: no warden is running on this machine" line (partial expects 2 lines and gets 3, a complete sample expects 1 and gets 2; phases gets that line where it expects the grant line). The tests start no warden.
- `🛟️unsatisfiable`: a critical sample answers `pressure-critical` where the test expects `starved` after the starvation wait.
Cause: other agents' changes. `⌨️command`, `🔨️modules/🏃️process/🏗️build/🌡️host-capacity/{📏️decision,🛡️warden,🎫️gate}` were changed in commits 758 and 759 (2026-11-26), and `🛡️warden/🟦️.ts`, its loop and its test oracle have further uncommitted edits (`git status`, `git diff HEAD`, read-only); the three test files were last touched on 2026-11-25 and 2026-11-22. I left them unchanged; they need the owner of the warden change to update either the expectations or the status-line logic.
