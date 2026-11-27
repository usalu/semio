# F115: one validated, locked, atomic writer for the build owner registry

Lane F115, 2026-11-26. Foreground, no sub-agents, no graph/run-many/cargo. Locks `graph` and `testing` taken only for the atomic swaps, released right after each.

## 1. Writers of `📋️build-owners.json` (audit)
| Writer | Atomic | Validated before publish | Concurrent RMW protection | Before |
|---|---|---|---|---|
| `reconcileBuildOwnerRegistry` (registry-consistency; callers: `verify build-owners sync`, crate-tree carve and scaffold, mutation-tree, taxonomy apply workflow) | yes (temp + rename) | NO (only the input was read validated, the synced output was not) | machine-local lock plus a read-compare before rename (small window); no cross-machine protection | had private `replaceOwnerRegistryFileV1` |
| `adoptBuildOwnerRegistryForV1` (private repository test workspaces) | no (`copyFileSync` COPYFILE_EXCL) | no | n/a (create only) | copied blindly, even an invalid registry |
| ticket scripts / hand edits / generic taxonomy path edits | no guarantee | no | none | outside the library; cannot be forced through the function |
All other mentions (observation, owner-discovery, registry-reads, invocation, discovery, prefetch, verification, playground, laws) only read.

## 2. Fix: one shared writer
New `🏗️build/🕸️graph/🧾️registry-consistency/🔏️writer/🟦️.ts` (sha e526272b) holds the lock (moved verbatim from registry-consistency) and:
- `updateBuildOwnerRegistryV1(file, compute, hooks?)`: under the lock reads the file, `compute(source)` returns `{value, text?}`, the text is validated with `readBuildOwnerRegistryV1` BEFORE anything is staged, written to a unique `.tmp` sibling, compared with the current file content (compare-and-swap) and renamed; a changed file raises `OwnerRegistryRaceV1` and `compute` runs again on the new content (up to 4 attempts); the temp is always removed.
- `publishOwnerRegistryTextV1` (the CAS publish), `createOwnerRegistryTextV1` (validated, staged, hard-linked so create-if-absent is atomic), `withOwnerRegistryLockV1`, `OwnerRegistryRaceV1`, a `rename` hook as test seam.
- `reconcileBuildOwnerRegistry` (sha 1cde9cb7) now only computes the plan and calls `updateBuildOwnerRegistryV1`; `replaceOwnerRegistryFileV1`, the retry loop and the lock were deleted there (no duplicate write code remains).
- `adoptBuildOwnerRegistryForV1` (sha b41dd6bf) uses `createOwnerRegistryTextV1`: an invalid checkout registry is refused instead of copied.
Limit: the lock is per machine (temp dir, not replicated). Two machines editing at the same moment are only guarded by replication itself; a duplicate that arrives by merge is caught by `readBuildOwnerRegistryV1` on the next write (refuses, never publishes).

## 3. Who produced the duplicate
Not provable from git (auto-commit dates are fake and the bad state was never committed). Evidence: the crate `🏪️store/🗂️schema-exports` was created 17:21 by the xlsx ticket (☀️20); the NORM ticket (☀️10, en1995-unit-ceiling note) lists `build-owners.json` among its updated files in the same window; the registry file was last written 17:39:29 and now holds the row once. `readBuildOwnerRegistryV1` already drops byte-identical rows, so the collision had two non-identical rows for the same name: two independent writers (a hand/script edit next to a sync, both unlocked and non-validating) is the likely cause. Both now have one safe path, but a script that writes the file directly still bypasses the writer: tickets must call `repo ./📜️script.ts verify build-owners sync` (goes through the writer), never edit the file.

## 4. Tests (`repo test-file`)
- `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🟦️.ts` (sha fb8e1de0): 43 pass / 0 fail. New: invalid registry (duplicate name, empty, non-JSON) refused, old file byte-identical, no temp left; 6 concurrent child processes (distinct rows plus one shared row) land exactly once each with 6 valid rows; a hand edit between read and rename is detected, compute reruns, the peer row survives; a failing rename and a child that exits in the rename hook leave the old file (stale temp tolerated, next writer lands); create-once and invalid create; source law that no other library source publishes the registry. The old lock tests now import from the writer module.
- `.../🧪️testing/🧫️private-repository/🧰️adoption/🧪️tests/🟦️.ts` (sha 34702689): 9 pass / 0 fail (new adoption test: copy once, keep existing, refuse invalid).
- `repo typecheck` registry-consistency + adoption folders: passed, 0 errors. `dangling-imports.py` on both: exit 0.

## Kept scripts
`T/🧰️scripts/F115/`: stage.py, land-consistency.py, land-adoption.py, land-tests.py, land-adoption-test.py, tests-append.ts.txt.
