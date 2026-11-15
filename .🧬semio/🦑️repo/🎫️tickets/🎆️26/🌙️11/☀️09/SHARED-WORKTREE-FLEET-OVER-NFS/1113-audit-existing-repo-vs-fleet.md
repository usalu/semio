# 1113 Audit: Existing Repo (Documents) vs Fleet Worktree (Shared)

Read-only audit. Snapshot taken about 08:58 to 09:03 local time on 2026-11-13. The fleet worktree is live, so counts drift after that.

- Documents: `/Users/ueli/Documents/semio` (the owner's original repo)
- Shared: `/Users/Shared/semio` (fleet-replicated worktree)
- Scratch: `/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🧰️scripts/1113-compare/` (`compare.py`, `compare.json`, `doc-status.txt`, `shared-status.txt`)

## 1. Repository state

| | Documents | Shared |
|---|---|---|
| `git rev-parse HEAD` | `5ececaa78b9265fd72b4f7e8f411afac3c9e1df1` | `5ececaa78b9265fd72b4f7e8f411afac3c9e1df1` |
| Branch | `🐙ueli/⛳wip` (not detached) | `🐙ueli/⛳wip` (not detached) |
| Ahead/behind | Identical HEAD, 0 ahead / 0 behind (`git rev-list --count` = 0). HEAD object present in both. | Same |

Status from `git --no-optional-locks status --porcelain=v1 -uall --no-renames`:

| XY | Documents | Shared |
|---|---|---|
| `A ` (staged add) | 7 | 1110 |
| `M ` (staged modify) | 120 | 18 |
| `D ` (staged delete) | 0 | 272 |
| `AM` (staged add, unstaged modify) | 0 | 13 |
| `MM` (staged and unstaged modify) | 0 | 28 |
| ` M` (unstaged modify) | 0 | 79 |
| `??` (untracked, not ignored) | 0 | 149 |
| Total lines | 127 | 1669 |

Totals: Documents has 127 staged, 0 unstaged, 0 untracked. Shared has 1441 staged (including 272 staged deletions), 120 with unstaged modifications, and 149 untracked.

## 2. Shared file sets

File sets from `git ls-files -z --cached --others --exclude-standard`.

- Documents: 271,507 paths. Shared: 272,503 paths. Union: 272,782.
- Only in Documents: 279
- Only in Shared: 1,275
- In both: 271,228
- Missing on disk on either side (for paths in both): 0

Of the 271,228 paths in both:

- Same content: 270,982
- Differ: 246
  - By size: 243
  - By content, same size: 3 (`sha256` differs)
  - By symlink target or type: 0

For the 246 differing paths, mtime comparison:

- Shared newer: 199
- Documents newer: 47
- Equal mtime: 0

Caution: many Documents mtimes cluster at 2026-11-12T22:22:19 to 22:22:20, which looks like a bulk restore or checkout, so mtime is only a weak signal.

The 3 same-size content differences are `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/🔣️.json`, `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧬️schema/🔣️.json`, and `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`.

### Top 30 directories (first 3 path components)

Differing paths (246 total; directories with fewer than 2 are omitted):

| Directory | Differing |
|---|---|
| `🧰️framework/🛍️products/🦑️repo` | 92 |
| `✏️s/🔌️plugins/🌊️flow` | 19 |
| `✏️s/🔌️plugins/🧩️puzzle` | 17 |
| `✏️s/🔌️plugins/📕️norm` | 16 |
| `🎓️teaching/🏛️architecture/📚️course` | 12 |
| `.🧬semio/🦑️repo/🎫️tickets` | 10 |
| `✏️s/🔌️plugins/🗄️stdio` | 9 |
| `🧰️framework/🛍️products/💻️os` | 7 |
| `✏️s/🔌️plugins/🪵️sourcing` | 6 |
| `🧰️framework/🔨️modules/🏃️process` | 6 |
| `(root)` | 6 |
| `✏️s/🔌️plugins/📐️cad` | 5 |
| `✏️s/🔌️plugins/🏭️process` | 5 |
| `🧰️framework/🔨️modules/📡️replication` | 5 |
| `✏️s/🔌️plugins/🏗️fem` | 4 |
| `🧰️framework/🛍️products/📓️print` | 3 |
| `🧰️framework/🔨️modules/🎠️kernel` | 2 |
| `🎓️teaching/🛂️proctor/🏗️bootstrap` | 2 |

Remaining directories with 1 differing path each (12): `🧰️framework/🔨️modules/🖱️ui`, `✏️s/🔌️plugins/🌀️procedural`, `✏️s`, `🧰️framework`, `✏️s/🔌️plugins/📖️playbook`, `🌎️hub/🧩️compositions/🎪️demonstrator`, `✏️s/🔌️plugins/🔋️energy`, `🌎️hub`, `✏️s/🔌️plugins/✒️writer`, `✏️s/🔌️plugins/🌍️gis`, `🌎️hub/🧩️compositions/🪵️sourcing`, `🎓️teaching/🛂️proctor/📦️packages`.

Only in Documents (279 total, 10 directories):

| Directory | Only in Documents |
|---|---|
| `.🧬semio/🦑️repo/🎫️tickets` | 211 |
| `🧰️framework/🛍️products/💻️os` | 36 |
| `✏️s/🔌️plugins/🔋️energy` | 10 |
| `teaching-morph-preview-launch-kit` | 6 |
| `staffage-launch-kit` | 4 |
| `puzzle-artifact-editors-windows-kit` | 4 |
| `holder` | 3 |
| `holder/claims` | 2 |
| `🌎️hub/📦️packages/🦀️rust` | 2 |
| `✏️s/🔌️plugins/🗄️stdio` | 1 |

Only in Shared (1,275 total, 6 directories):

| Directory | Only in Shared |
|---|---|
| `.🧬semio/🦑️repo/🎫️tickets` | 1187 |
| `🧰️framework/🛍️products/🦑️repo` | 64 |
| `🎓️teaching/🏛️architecture/⚡️energy` | 10 |
| `✏️s/🔌️plugins/🔋️energy` | 10 |
| `✏️s/🔌️plugins/📸️remodel` | 2 |
| `✏️s/🔌️plugins/🖨️raster` | 2 |

## 3. Gitignored local-only data (top level)

| Path | Documents | Shared |
|---|---|---|
| `.cache/` | 250G | 429M |
| `.nx/` | 38M | 38M |
| `.venv/` | 567M | 567M |
| `.tmp-diskproof/` | 144K | 144K |
| `.pytest_cache/` | 36K | 36K |
| `.tmp/` | 0B | 0B |
| `node_modules/` | present, 0B | present, 0B |
| `target/` | absent | absent |

The Documents `.cache` du ran past the 2-minute foreground window, so it was moved to a background run. It completed, and the 250G figure is exact. Shared shows 16 top-level ignored entries and Documents shows 15; the single extra was not identified.

## 4. Documents-only files newer than 2026-11-09

There are 14 such files, all listed below (below the 60 cap). Full list is in `compare.json`.

| mtime | bytes | path |
|---|---|---|
| 2026-11-12 22:35:17 | 1524 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️04/SOLVE-ALL-MERGE-CONFLICTS/resolution-2026-11-12-memory.md` |
| 2026-11-12 22:22:19 | 1763 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1112-result-puzzlefinal.md` |
| 2026-11-12 22:22:19 | 14064 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1112-result-graphslice.md` |
| 2026-11-12 22:22:19 | 4046 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1112-result-floweditor.md` |
| 2026-11-12 22:22:19 | 5262 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1112-rename-fieldprep.files` |
| 2026-11-12 22:22:19 | 7671 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1112-audit-unauthorized.md` |
| 2026-11-12 22:22:19 | 14046 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1112-audit-directcmds.md` |
| 2026-11-12 22:19:41 | 5535 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️11/ARCHITECTURE-COURSE-REDEPLOY-READINESS/readiness-report.md` |
| 2026-11-11 02:33:31 | 1224 | `teaching-morph-preview-launch-kit/run.sh` |
| 2026-11-11 02:33:31 | 13994 | `teaching-morph-preview-launch-kit/prompt.txt` |
| 2026-11-11 02:33:31 | 4906 | `teaching-morph-preview-launch-kit/fleet.md` |
| 2026-11-11 02:33:31 | 28344 | `teaching-morph-preview-launch-kit/agents.json` |
| 2026-11-11 02:33:31 | 5864 | `teaching-morph-preview-launch-kit/acceptance.md` |
| 2026-11-11 02:33:31 | 7108 | `teaching-morph-preview-launch-kit/README.md` |

Threshold used: mtime on or after 2026-11-10 00:00 local. Files dated 2026-11-09 exactly: 0 more.

## 5. Conclusion

Adopting `/Users/ueli/Documents/semio` into the fleet would involve these numbers. Each depends on which side wins for the differing files.

- Publish (new to the fleet): 279 paths that exist only in Documents, 14 of them newer than 2026-11-09. The 211 ticket-folder paths under `.🧬semio/🦑️repo/🎫️tickets` are the bulk.
- Overwrite: 246 paths differ. If Documents wins, 199 Shared-newer versions are overwritten (3 of them are same-size content edits, not only mtime). If Shared wins, 47 Documents-newer versions are lost.
- Lose: 1,275 paths exist only in Shared, including 1,187 under the tickets folder and 64 under `🧰️framework/🛍️products/🦑️repo`. Replacing the fleet tree with Documents removes these from the fleet. They are not in Documents at all.
- Index and uncommitted state: Documents has 127 staged changes and nothing unstaged or untracked. Shared has 1441 staged, 120 unstaged, and 149 untracked. Replacing the tree discards Shared's index and its 149 untracked files unless they are first copied.
- Local-only data: Documents `.cache` is 250G versus 429M in Shared. Moving the repo carries that size, and it will not replicate to the fleet. `.venv` (567M) and `.nx` (38M) are identical in size on both sides.

Estimates: none of the counts are estimates. The snapshot is point-in-time, and Shared changes continuously, so re-run `compare.py` immediately before any move.
