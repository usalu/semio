# result mirror-g (10-26 19:55): one shared, safe overlay tool

FOR ALL LANES: use `python3 -I C/mirror-tools/mirror.py` (usage: `C/mirror-tools/README.md`) for every staged-batch verification instead of your own symlink farm / workbench / overlay. Names `<lane>-<what>`, max 8 characters.
INCIDENT (mine): at ~19:45 my cleanup `drop --all` also removed a mirror named `v2g` (not mine, someone already used the tool) and the shared node_modules copy; recreate it, the copy is warm again. `drop --all` no longer exists (names or `--prefix` only; `--nm` refuses while mirrors exist).

## What was built (ticket tooling, `C/mirror-tools/`)
- `mirror.py` (stdlib, macOS/Linux/Windows): `create | run | diff | list | drop`. All files are REAL copies (macOS `clonefile(2)` via ctypes, Linux FICLONE, Windows plain copy); no symlink into the live tree.
- Selection from the working tree (tracked + untracked-not-ignored in the requested subtrees, so uncommitted live edits ride along), root files, ancestor sibling files, import closure over `📋️project.json` projects (default on), node_modules.
- node_modules: the managed one is on a DMG (cross-volume, no clone) -> one shared real copy `$TMPDIR/sm/.nm` (1.6 GB, built once), mirrors clone it. 134 absolute links into the live tree + relative links escaping to the root node_modules are rewritten into the mirror (without that, `ajv` resolved to the wrong place and tests failed).
- `--batch`: stage dir with `manifest.json` (obs-g/xplat format, base-hash check, atomic replace in the mirror) or a command with `{root}` (run sealed + guarded).
- Safety: macOS `sandbox-exec` seal (writes below the live repo denied, except `.cache .nx .tmp .repo .venv .pytest_cache`) + post-run write-through guard (mtime/size of every mirrored live file + directory listings; exit 97 with the paths, tagged WRITE-THROUGH SUSPECT vs hard-coded-live-path/peer).
- `selftest.py`: 20 laws on a throw-away git repo, `diff -rq` as independent oracle: 20/20 green (incl. unsealed write-through exit 97, seal EPERM, touched live file detected, `--peers-ok`, stale batch base exit 3, link rewrite, no escaping symlink).

## Tests on two real staged batches (live tree untouched, guard clean)
1. obs-g `staged/fleet` (manifest batch): create + apply 26 files; `diff` = exactly 26 entries (14 M, 12 A); fleet-status + guard tests 14 pass / 0 fail in the mirror (7 s).
2. quietfc-g (command batch, `edit_schema.py/edit_code.py/install_new.py {root}`): 28 differences incl. the 2 new files; host-capacity followup law 9 pass / 0 fail.

## Measurements (host load 25-43 during all of them, 10 cores busy: numbers are pessimistic)
| scenario | files | logical | create wall | disk cost |
|---|---|---|---|---|
| library + process + launcher, `--no-closure --no-nm` | 6,719 | 76 MB | 6-15 s (select 2.4 s, clone 4-8 s, 4 threads) | 5-8 MB (CoW metadata) |
| default (+ import closure 80 projects + 80 small node_modules) | 16,056 | 173 MB | 48-53 s (closure scan 20 s, clone 43 s) | ~40 MB metadata |
| `--only mcp` default closure | 11,764 | - | 43 s clone | - |
| first-ever shared node_modules copy | 57k files | 1.6 GB real | 155-175 s ONCE (read from the DMG) | 1.6 GB once, shared |
| clone of that copy into a mirror | 57k | - | 2.7 s | metadata only |
| guard around `run true` (16k files, 12.9k dirs, 2 snapshots) | | | 5-12 s | |
| `diff` of the 16k mirror | | | 12.9 s | |
| `drop` | | | rename instant; deletion 5-25 s (detached by default, `--sync` to wait) | |
Disk numbers are statvfs deltas (noisy by +-40 MB because peers write): clones share blocks until modified. Longest path in the default set: 252 chars with a 5-character name (so names <= 8 characters); a shorter `SEMIO_MIRROR_HOME` (e.g. `/tmp/sm`) gives headroom.

## Known limits
- Linux/Windows branches not executed (FICLONE, plain copy, no seal); the guard is pure Python.
- Closure follows imports and workspace package names only; non-import reads of another project need `--only`.
- No `.git` in the mirror. Untracked root-level files are not copied (listing them walks the whole repo, 26 s). Files > 64 MB skipped.
- Seal exceptions (`.cache` ...) are writable on purpose; with `--no-seal` only the guard applies and ignored locations outside the snapshot are not watched.
- A concurrent peer edit of a mirrored live file during `run` is reported like a write-through (tag tells them apart; `--peers-ok` downgrades).
