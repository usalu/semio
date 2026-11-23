# FEM publication (interim: installer fix done, republish run in progress)

## Step 1: EPERM root cause and fix (done)

Failure: `error: failed applying patch file: EPERM: Operation not permitted (open())` / `failed to apply patchfile (patches/1.patch)` (log `🗑️generated/pub-describe3.log`).

Evidence:
- `patches/1.patch` in the install generation (`.cache/b/d/m/j/964b950e623068b80043`) is the `nx@23.2.0` patch (154098 bytes; `0.patch` is `next@15.5.22`). The installer is Bun, which applies it by copying the pristine package from the fixed Bun cache (`.cache/b/d/m/bun-cache/nx@23.2.0@@@1`) to a new `..._patch_hash=<hash>` entry and then opening files for write.
- Nine files of the pristine cache entry are read-only (`-r--r--r--`) hardlinks into the shared CAS (`%LOCALAPPDATA%\semio\cas\v1\o\...`, also linked from `.cache/b/t/m/downloads/nx@23.2.0@@@1`), among them `dist/src/tasks-runner/task-orchestrator.js`, which the patch rewrites. Copying keeps the read-only attribute, so Bun's `open()` for write fails with EPERM. Older patch entries (`41dab621...`, `ccc4c9bc...`) were created before those files became read-only CAS links; any new patch hash now hits the problem.
- Not a transient lock: `repo leases status`/`repo doctor` showed no holder of the nx files; the failure is deterministic for a new patch hash.

Real defect (per coordinator, fixed at the source): the CAS adoption place `bun-cache` (`🏗️build/🗄️cas/🧬️schema/🔣️.json`, mode hardlink, "the bun content cache never changes a stored file in place") turned the pristine files (>= 64 KiB, so 9 of 1347 for nx) of the entry that Bun copies and patches into read-only hardlinks of the store. That assumption is false for packages in `patchedDependencies`. The earlier pattern workaround was reverted (schema and corpus back to the original).

Fix:
- New `🏗️build/🗄️cas/🧲️adopt/🔒️private/🟦️.ts`: `patchedDependencyKeysV1`, `patchedDependencyKeysOfWorkspaceV1` (root manifest plus every generation manifest), `patchedBunCacheEntriesV1` (pristine entries, `_patch_hash=` entries excluded) and `privatizeCasEntriesV1` (replaces read-only or shared files by writable private copies, copy then unlink then rename, never writing through the shared inode).
- `⌨️command/🟦️.ts` (`cache cas adopt`): privatizes those entries first and passes them in `leave`, so adoption never links them again.
- `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`: new `privatizeJavascriptDependencyPatchedCacheV1` runs at the start of every Bun install of a scope (reads that scope's `package.json` `patchedDependencies`), so an already-shared entry is repaired before Bun patches it.
- Live store repaired now with `privatize-live.ts` (ticket folder): 18 files in the `nx@23.2.0` and `next@15.5.22` pristine entries are writable unshared copies (verified: `task-orchestrator.js` is `-rw-r--r--`, link count 1).
- Test `patched package cache entries are writable private copies, never store hardlinks` in `🗄️cas/🧪️tests/🟦️.ts` passes (adoption links, privatize repairs, patched entry stays unshared on re-adopt, patch-hash entry untouched, Python oracle agrees on link count and writability). The cas suite otherwise shows 4 failures I did not touch: the Windows exec-mode adoption test (`one.skipped.mode`, 0 vs 1: no exec bit on NTFS) and collector write-timeout tests under host load.

## Step 2: describe fem rerun (in progress)

- `pub-describe4.log` (started ~17:50): the graph service did not answer within 900 s ("lease-sweep" source admission work in the service host, PID 29020 / host 40956); failed before any install. Not related to the fix.
- `pub-describe5.log` (current): started right after; graph slice read ongoing (`Reading the project graph slice`, inventory check 87 of 600 at 822 s). Previous cold reads took 30-100 min. After the graph read, the dependency sync runs; the new retry path should show `[DATA] Store dependency cache link refused; purging the Bun package cache and retrying` then `Bun package cache purged`, and the sync should complete (a full re-download of the Bun cache, about 2600 entries, needs network).
- Nothing else of steps 2-3 has run: no catalog regenerate/check, no republish verification, no servers on 6086/6087 started by me (netstat showed none listening at start).

## Remaining commands (after `pub-describe5.log` ends with success)

1. Verify `C:\semio\.cache\b\p\m\browser\prototype\modules\🏗️fem\🧾️receipt.json` differs from the Nov 20 values (provenance `dbb136d0`, wasm `2062ccd3`).
2. `repo run @semio-tech/s-dev-composition:generate`, then `:catalog-check` (fem with 2d+3d apps, empty diagnostics).
3. `repo run @semio-tech/framework-os-dev:dev-fem3d-react-prototype` (6087) and `dev-fem2d-react-prototype` (6086) in the background; curl for HTML and receipt hashes.
4. If the log shows the purge happened but the patch EPERM persists, the next candidate is to make the pristine entry writable copies before install (clear read-only on an owned copy, never on CAS links).

## Files changed
- Added `🏗️build/🗄️cas/🧲️adopt/🔒️private/🟦️.ts`; edited `🗄️cas/⌨️command/🟦️.ts`, `🗄️cas/🧪️tests/🟦️.ts`, `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`.
- Ticket: `privatize-live.ts`.

## cas suite failures (investigated, none from the private-entries change)
All were test assumptions that do not hold on Windows; fixed in `🗄️cas/🧪️tests/🟦️.ts` (full file now 45/45 except one run still to confirm):
- `casRootV1` POSIX path literal: now compared with `join("/h",".cache","semio","cas")`.
- `the command reports ...`: the test read `old.plan` back to check it survived a dry run; on NTFS that read updates the access time, which makes the file "freshly accessed" and protected, so the next German dry run planned nothing. It now checks with `statSync` (no atime touch).
- `adoption makes ...` (exec-bit): NTFS has no exec bit, so `tool.sh` (0o755) has the same mode as the others and legitimately shares the object. The test now asserts the platform semantics (win32: shared, mode skip 0, census groups [1,1,1], `tool.sh` removed with the others; elsewhere unchanged).
- The 4th failure (collector write-timeout) was host-load timing and passed on re-run.
