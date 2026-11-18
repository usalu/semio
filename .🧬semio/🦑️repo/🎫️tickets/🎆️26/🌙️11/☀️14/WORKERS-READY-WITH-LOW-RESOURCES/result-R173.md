# R173: scoped ownership walk wired, split of duties (living report, continues R162, R165, R168)

Status 2026-11-18 02:10 UTC: NOTHING landed, nothing live edited (freeze). Staged in scratch `scratchpad/R173/stage` (patch scripts `scratchpad/R173/patch-a.py`, `patch-a2.py`, law `stage/.../💾️storage/🧪️tests/✂️split/🟦️.ts`, 3 tests green in scratch). All scratch runs now use `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` (coordinator hazard); `~/Library/Caches/bun/@t@` has no entry naming `scratchpad/R173` (grep, 02:09).

## 1. Split of the three duties (staged)
* Finding: retirement (b) already has its own whole-repo walk (`retireJavascriptDependencyPreservationsInStoreV1`, called after a publish and by `clean`), and the preservation presence check (c) is only called by `clean`. The shared full walk of every selected command is the producer attempt (`withPublishedBunWorkspaceOwnershipV1(workspace,{kind:"all"}`, build/publish and lock) plus the two readers (`withJavascriptDependencyReadStorageV1`, build bootstrap `currentOwner`).
* Staged edits: `💾️storage/🟦️.ts` new pure `javascriptDependencyWalkSelectionV1` (all -> all; scope/scopes -> `{closure, roots}` without "."), `javascriptDependencyReadSelectionV1(scope)`, `javascriptDependencyRetiresPreservationsV1`; attempt and lock use the walk selection; retirement after publish only when the selection is `all`; reader site and `🏗️build/🚀️bootstrap/🟦️.ts` use the read selection. The two remaining `{kind:"all"}` walks in the storage file are exactly retirement and presence (law pins that by source shape).
* Law `✂️split` (3): table of the pure selections; model law over 200 seeded worlds: for every scope of the closure the owners, `relationSha256`, generation and group digest (`javascriptDependencySourceOwnerGenerationsV1`) and the requirements of every requested scope equal those of the complete walk; source-shape law of the split. Green in scratch (25 s).
* Consequence to decide: a selected sync no longer retires preservation capsules; they wait for the next complete sync or `clean`.
* NOT done: the generation receipts published by a selected attempt now record closure observations (argued safe: edits outside the closure cannot affect the scope); no end-to-end proof.

## Storage battery: what it needs (found)
56 of 67 baseline failures were "Original parent Script process invocation required". Needs: env `SEMIO_SCRIPT_PROCESS_INVOCATION` (a wire from `createScriptProcessEnvelope`), `SEMIO_TEST_ARTIFACT_DIR`, cwd = repo root, `SEMIO_TEST_PYTHON_BINARY=/usr/bin/python3` (incremental suite 6/12 -> 14/4 with it). Minting the wire in scratch (`scratchpad/R173/mint.ts`, capability `artifacts` only) works but changes the failure profile (13 `equal-copy`, 9 `stageNativePlatformProtectedArtifactInputGroupV1 is not a function`): more capabilities are needed, so a faithful baseline still needs the runner's full capability set. Staged-vs-live battery compare NOT completed.

## 4. Numbers (live, read-only, counting wrapper `scratchpad/R173/live.cjs`, load average 58)
| selection | paths | first: lstat / opendir / s | repeat (listings seeded): lstat / s | one recheck lstat / s |
|---|---|---|---|---|
| closure root scope only (repo-mcp and the framework probes live in it) | 29,885 | 95,275 / 29,725 / 6.5 | 65,548 / 2.2 | 35,802 / 0.4 to 1.4 |
| all | 98,542 | 309,216 / 98,206 / 29.8 | 211,016 / 10.8 | 112,801 / 3.2 to 3.7 |
| closure root + hub | 96,014 | 301,312 / 95,681 / 30.0 | 205,628 / 8.4 | 109,924 / 3.8 to 6.2 |
Root closure: 3.3x fewer paths, 3.2x fewer lstat, 4.9x fewer seconds first. A hub closure is almost the whole repo. The three-probe closure was not run separately (all probes are framework members, so it is the root closure). Targets (5 s first, 200 ms repeat) NOT met: repeat is 2.2 s.

## 2. D (lag): measured, not built
`lag.cjs` (fixed: the gap shows only after a timer turn) on a 100k-path tree: one recheck is a single event-loop gap of 3.5 to 13 s (the final synchronous pass); derivation lag with a non-yielding `advance` is the whole walk, with the real callers (yield every 64) it is bounded per batch. Slicing the final pass is sound in the interval sense (each path unchanged over [recorded, its stat], so a common instant lies before the pass), but it breaks the pinned frontier law and your instruction; restricting the final pass to a "changed set" is NOT sound without an fs-events barrier (a change after the async stat of an unchanged path is invisible). Options for you: sliced final pass (rewrite the frontier law), worker-thread pass, or accept. Not built: progress totals, lag law.

## 3. C (receipt single-flight): not built
Design only: receipt (generation digest, selection digest, store identity, per-path stamps) under `machinePathV1`, adoption by `replayCurrentMembershipV1` then one fence, fall back to derivation on false. Gain is at most the derivation's extra lstat per path (about 2x) plus plan CPU; it needs a schema and codec for a new persisted format. Blind spot: in-place content edits keeping size, mtime, ctime (the dependencies stage's full check covers them).

## Landing
Not queued as ready: the split is staged and law-green but the battery comparison is incomplete. Landing at 03:30 needs: copy the three staged files (`💾️storage/🟦️.ts`, `🏗️build/🚀️bootstrap/🟦️.ts`, the new law dir), run battery staged vs live with the cache disabled, tested restore.
