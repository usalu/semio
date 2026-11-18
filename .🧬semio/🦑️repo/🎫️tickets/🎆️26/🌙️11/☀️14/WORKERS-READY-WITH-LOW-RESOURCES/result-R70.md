# R70: g1-m05 `dependencies` stage, why every try walks again and why it is slow

Status: STAGED, NOT LANDED. Package: `T/🧰️scripts/R70/` (`land.py` with `--dry-run` ok: 1 clean modification, 5 new files; `stage/`). `T` = FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION. Scratch workspace copy `/private/tmp/r70/ws`.

## Live try: R67 code, no resume yet
- Try 9 started 02:59:31Z, after R67's landing (02:50Z). Its log has `Ownership walk: N steps ...` and `[DATA] Store ownership walk`, so it runs R67's code.
- It cannot have resumed: no earlier try wrote an R67 listing checkpoint. It should write one every 120 s. Whether it did is not visible, because the Windows store is not in the mirrored folder. The next try is the first that can seed from it.
- At 03:34Z the stage is at about 2070 s, still in `store-input-replay` (no `Ownership walk` or "stale" lines in the tail).

## Cause 1: nothing is persisted until a whole run succeeds
- The gate `verifyJavascriptDependencySyncCurrentV1` (`🧾️incremental/💾️storage`) answers `current` only when a sync receipt exists. `publishJavascriptDependencySyncFromReadsV1` writes that receipt only at the very end of a complete run.
- g1-m05 has never completed one run, so every try answers `stale: absent` and does the full producer. The Macs finished once, so their 20 s is the gate answering "current", not a walk.
- Path case, drive letter, mtime granularity and ACL reads are not shown as causes. The listing index is keyed by absolute path and re-based on the workspace, and the stamp is dev/ino/mode/mtimeNs/ctimeNs. No log line names a stale reason. The ctime of dependency symlinks was fixed by commit 744 and is not re-examined.

## Cause 2: the "walk" is mostly repeated fences, not discovery
- Measured here on the real repo (`bun` profile, load about 30): derivation visits 98 203 paths (97 864 `list`, 97 895 `state`, 323 manifests). One `custody.recheck()` costs 196 406 `frontier` operations plus 646 manifest reads. That is two replays, each doing two lstat passes over all 98 K paths, plus the manifest reads. It took 10 to 15 s under load.
- The counters in the Windows log match this. At 869 120 steps (975 s) the walk had derived once and rechecked about 3.4 times, so the step count is about 320 K plus k times 197 K, not a single walk.
- The producer calls `fence()` at about 33 sites: 8 fixed, 2 final, and one after every 64 published bindings (about 23 for roughly 1 500 packages). Every `fence()` does the full ownership recheck and `store-input-replay`. That replay re-opens, reads and hashes every staged input file through the Windows relative-handle layer. `store-input-replay` is that loop, one progress line per 64 work units.
- The Windows log shows about 2 lines per 10 s, which is roughly 4 files per second.
- Estimate for Windows: 200 to 330 s per recheck plus the input replay, times 33 fences, is over 2 h. Try 8's cap was hit at 2.08 M steps, which is about 10 rechecks done of about 33.
- Not established: why one Windows operation is about 100 to 800 times slower than on macOS. The derivation ran at 7 700 steps/s early on and the replays at 400 to 900 steps/s. Candidates are the relative-open FFI, Defender, and the replica daemon writing into the tree. Nothing in the evidence separates them.

## Fix (staged)
1. New `📦️dependencies/🧮️fence` (schema first, pure `fenceDueV1`, fixture, Python oracle, tests). An interim fence is paid in full only when no full fence has run yet, or the last one ended at least 8 times its duration ago (never more than 30 min). A `final` fence is always full. The skipped path still calls `checkpoint()` and `store.recheck()`.
2. In the producer (`💾️storage/🟦️.ts`): the old `fence` is renamed `fullFence`. `fence` is the budgeted interim fence and `finalFence` the always-full one. `verifyReady` and the fence before `if(phase==="build")return` use `finalFence`, so the full fences stay before the sources are rewritten and the receipts sealed. A `[DATA] Store dependency fences {full,skipped,fullSeconds}` line is written at each final fence.
3. `progress.frontierBatchPaths = 64`. Before, the frontier replay awaited `advance` for every path (the field was unset, so it defaulted to 1). The producer already filtered on `%64`, so behaviour is the same with about 64 times fewer awaits.
4. R67's listing checkpoint is unchanged. Once one run completes, the sync receipt makes every no-change try a gate answer.

## Proof
- Fence suite: 9 pass. It covers the schema, the fixture, a Python oracle that agrees on every step, a final-is-full law, a per-platform stage law, an instant-fence law, and a source wiring law.
- Fail before: swapping the original `storage/🟦️.ts` in made the wiring law fail. The module also did not exist before.
- Simulated stage of 33 sites, 1 500 bindings, 330 s per fence (Windows): 33 full fences, 11 377 s before; 3 full fences, 1 482 s after. macOS at 3 s per fence: 586 s before, 537 s after (15 full fences).
- Existing storage suites in the copy: `🏭️producer` 1 pass, `📤️diagnostics` 1 pass, `📏️work` 3 pass. In `🗂️workspaces/🟦️bun/🧪️tests`, `⏩️frontier` 2 pass, `♻️refresh` 29 pass, `🌊️churn` 7 pass.
- The `🟦️.ts` suite of the bun workspaces tests fails to load in the copy ("Cannot find package 'ajv'" at first; not re-run after ajv was copied in). The standing preflight tests were not run.

## Why it is not landed
- `💾️storage/🟦️.ts` is inside the tool closures. It appears in 40 importer/imported rows of the R68 production closure documents, and `🗂️workspaces/🟦️bun` appears in 6 and 7. Landing it stales every package record on the fleet again. The new `🧮️fence` module is imported from it, so it joins the closure.
- Per the brief, nothing was landed. Landing needs a coordinator decision. After landing, the next g1-m05 try should show the fence DATA line.

## Not done / not proven
- Anything about Windows speed: the per-op cost, whether the fence budget gets the first completion under the cap (estimated about 1 500 s of fences, plus the 330 s derive and the install work), or whether a listing checkpoint exists.
- A parallel or batched lstat for the ownership replay. I left it out: the frontier tests pin the callback sequence and I could not run the whole suite.
- Making `store-input-replay` itself cheaper (hashing only on the first and final replay).
- The `ledger:N` installed-paths verification (1 to 3 s per step in the log) may keep a no-change try on Windows far from seconds even once the gate says `current`. Unmeasured.
- Scratch note: I briefly created and removed `/tmp/dummy` (an empty file); the live worktree was only read.
