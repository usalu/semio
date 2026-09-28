# Unstage Ticket Folder Without Restart

GitKraken unstage of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23` restarted the Mac because that day tree was staged as one giant diff.

## Cause

Almost all of the bulk was `END-TO-END-OS-HUB-COLLABORATION-MCP`.

- Day tree was 6.7GB.
- 33292 paths were staged, 32913 of them under `wp-l1`.
- `wp-l1/w3-backup` was 4.0GB: 26 copies of a dirty worktree, including 23 copies of the 112MB `semio-os-mcp` binary. Those copies were not hardlinked (`nlink` 1).
- Other `wp-*` folders held `target/` and `target-*` debug builds (`os-hub` up to 336MB) plus `🗑️generated` fixtures.
- 15 `node_modules` symlinks under `wp-st2/generated` pointed at `/Users/ueli/Documents/semio/node_modules` and `.🧬semio/🌐hub/s13-cx1-overlay/node_modules`. They were not index entries. A folder unstage still walks the worktree, so GitKraken could follow them into the real `node_modules` tree while also loading the staged binaries.

`target*` and `node_modules` were already gitignored. The snapshots had been staged anyway, and `w3-backup` was not ignored, so the binaries outside `target/` stayed eligible.

## What changed

- `git restore --staged --` the day path. Staged paths under that path: 0. Worktree edits that were already modifications stayed unstaged (` M`), and never-committed ticket files stayed untracked (`??`).
- Removed `w3-backup`, Cargo `target` / `target-*`, `🗑️generated`, `generated`, and `build` dumps under that day, plus every file over 5MB that was not already in HEAD.
- Removed the `node_modules` symlinks and the absolute symlinks sitting next to them.
- Restored two committed files that the size sweep had removed: `wp-s19/payload/0011.new` and `wp-sh2/overlay-files.txt`.
- `.gitignore` now ignores `**/w3-backup/`.

## After

- Day tree: 83MB, about 3000 files.
- No `node_modules` symlinks under that day.
- Nothing under that day is staged.

Reports and the small `wp-*` scripts are still in the ticket. Unstaging this folder in GitKraken no longer has a multi-gigabyte binary diff or a symlink into `node_modules` to walk.
