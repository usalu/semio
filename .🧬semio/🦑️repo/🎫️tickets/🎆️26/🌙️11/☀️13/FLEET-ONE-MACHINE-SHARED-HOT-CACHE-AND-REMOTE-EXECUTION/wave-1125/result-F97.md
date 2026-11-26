# F97: graph-free inventory made cheap and exact

Lane F97, 2026-11-26. Birth test `repo test-file .../🌱️birth/🧪️tests`: 14/2 -> **16/0 in 23.5 s at the original 60 s timeouts** (no timeout bumps; an earlier 240 s bump was reverted).

## Why the walk cannot be pruned
The checkout holds 155,614 directories (48,411 `.rs` files, 1,615 manifests) after the existing skips. One `.rs` per directory is the taxonomy, so directory count follows file count. Skipping `🧫️fixtures` still leaves 107k dirs, `🧪️tests` as well 78k; 12,408 `.rs` files carry `#[path]` mounts, 1,212 of them under tests/fixtures, so pruning would change the answer. Real cost: one walk reads 155k directories and 48k files.

## Design (option c, then b)
- New `generated-owner/🗂️index/🟦️.ts`: per-process, per-root index of directory rows plus memoised per-file answers (`#[path]` mount targets, parsed manifests).
- **Exact change feed** (`WorkspaceChangeFeedV1`, satisfied structurally by the existing fenced journal client `connectChangeJournalV1`, fsevents/inotify, exact). A repeated call asks `since(cursor)`: only the parent directories of named paths are re-listed, structural directory rows (a/r/n) re-walk their subtree, file answers named by a row are dropped. No directory and no file is stamped. The journal's exclusions are verified against the walked tree at adoption; because `bazel-*` and `.r` are excluded by the journal, they are now also in `SKIPPED_DIRECTORIES_V1`.
- **Fallback without feed / when it refuses**: stamp revalidation (device, inode, mode, nlink, size, mtime and ctime in ns) per directory and per file, with the racy-entry rule (a stamp within 3 s of being read is not trusted, so same-second edits are seen).
- **Persistent snapshot** (`store` = `~/Library/Caches/semio/repo/<hash>/generated-owner`, next to the journal state, atomic temp+rename, 18 MB, tree stored as parent index + names): a new process restores it and lets the feed decide by cursor whether it is still exact; epoch change, gap or refusal falls back to a full stamped walk.
- Cancellation (`signal` checked per directory and file, nothing half applied that a retry cannot redo), progress (`report` once per listing, stage sequence unchanged), per-index serialisation.
- Warm CPU was dominated by `ownersOfV1` (13 M `dirname` calls); project directory is now precomputed and `within` is a prefix test. Mount targets that the listing names are known to exist and are not `access`-checked (40,937 candidates before, about 200 now).
- Callers (`🌱️birth`, `🎮️playground`) pass `generatedScanPortsV1(root)` from the new `🎛️commands/🛠️dev/📡️scan-feed/🟦️.ts` (library may depend on the journal; the process layer only sees the structural port).
- The birth test now builds its expected launches with the same ports (without them its own call re-stamped 155k directories on every case).

## Timings on the live checkout (host load 45-78, ms)
| Case | Before | After |
|---|---|---|
| One `generatedPrerequisiteArgumentsV1`, cold (walk 12 s + read 8 s + checks) | 48,158 / 57,327 | 17,000-26,400 (feed on, no snapshot) |
| Same, second call in a process | 11,379-37,103 (stamp memo first draft 8-15 s) | 150-420 |
| Same, new process with snapshot | n/a | 958 (3,753 with the first 52 MB format) |
| `listWorkspaceIndexedV1` alone, warm | 12,354 walk | 40-130 |
| Fresh full walk of the new code, load 44 | n/a | 5,293 |
| Live equality check, incremental (feed+snapshot) vs fresh full walk | n/a | equal, 48,410 sources both |

The very first cold walk of a machine stays 17-26 s at this load (155k directories plus 48k file reads); it happens once per journal epoch, every later process starts from the snapshot in under a second.

## Tests
- New `🗂️index/🧪️tests` (10/0): incremental answer equals a fresh full walk (separate cold index) and a Bun glob after adds, deletes, renames, vanished/replaced directories and nested creations, in three modes (stamps only, exact feed, feed that reports only file rows), same clock tick and after the racy window; a lying feed proves no stamping happens while the feed vouches; refusal falls back and recovers; derived answers follow same-size same-tick rewrites; snapshot resume after changes made while away, also followed by a refusal; cancellation and progress; missing root.
- `generated-owner/🧪️tests` 6/0 (new test: repeated scans of one root follow every edit). Birth 16/0. Also green: os mcp stdio staging (35/0), `static-sites-and-boundaries` (7/0).
- `repo typecheck` on generated-owner, scan-feed, birth, playground: passed (8 touched, 3 importers). `dangling-imports.py` on the same folders: exit 0.

## Caveats
- A mount target that is a symlink named `*.rs` whose target is broken counts as present when the listing names it (the previous `access` check said missing); no such mount exists in the checkout.
- The `📡️scan-feed` helper lives in the library because the process layer must not import the journal; it is not wired anywhere else.
- Lane did not touch the `build` area (held by F98); lock `process` released.
