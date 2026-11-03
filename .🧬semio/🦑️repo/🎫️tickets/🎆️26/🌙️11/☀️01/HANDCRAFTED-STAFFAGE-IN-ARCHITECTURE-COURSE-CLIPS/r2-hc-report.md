# R2 hygiene report (agent hc): OI-71 and OI-69

Scope: files touched by this ticket under `🎓️teaching/🏛️architecture`. Working copy only. No git modifying commands were run. Git queries were read-only (`status`, `show HEAD:`, `diff --stat`).

## OI-71: CRLF line endings versus HEAD

- `.gitattributes` sets `* text=auto eol=lf` (plus `**/🧫️fixtures/** -text` and binary rules). Global git config has `core.autocrlf=true` from `C:/Program Files/Git/etc/gitconfig`. No repo-level `core.autocrlf` or `core.eol`.
- `git status --porcelain` under `🎓️teaching/🏛️architecture` lists 147 entries. 99 are modified or added files that exist on disk. The rest are deletions.
- Compared each of the 99 with `git show HEAD:<path>`. HEAD style per file is either pure LF or pure CRLF. No file was mixed in HEAD.
- Six files had a working copy whose line endings differed from HEAD. They were rewritten with the script below. The other 93 already matched.

| File (under `🎓️teaching/🏛️architecture/`) | HEAD | Working copy before |
|---|---|---|
| `⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` | LF | 516 CRLF |
| `⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py` | LF | 187 CRLF |
| `⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json` | LF | 470 CRLF |
| `⚡️energy/🏗️authoring/📜️script.ts` | CRLF (pure) | 45 CRLF, 2 bare LF |
| `⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | LF | 1459 CRLF |
| `⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | LF | 1713 CRLF |

- `script.ts` is the exception. Its HEAD blob is CRLF throughout (47 of 47 lines). The brief said to match HEAD, so it was normalised to all CRLF.
- Script: `🧰️hc/eol_fix.py` (ticket-local). It writes through a temp file and `os.replace`. It converts only line endings and leaves the rest of the bytes alone. A `PermissionError` (file locked by a running process) would be reported as `LOCKED`. No lock occurred in this run.
- Check after the fix: `git diff --stat HEAD` for `solar-radiation` shows only content lines. `script.ts` shows 45 insertions and 45 deletions.

## OI-69: unused imports (F401) and unused locals (F841)

- Scope: all 50 `🐍️.py` paths in the ticket's status list under `🎓️teaching/🏛️architecture`. Clip files, plus the BIM, optimization and grasshopper clip files. Two of them are missing on disk (see below).
- Tool: ruff 0.x at `/c/Users/Ueli/.local/bin/ruff`, run with `--isolated --select F401,F841`. `repo tools exec` has no registered ruff entry, so ruff was called directly.
- Result after the fix: `All checks passed!` for the 48 files on disk. Zero F401 and zero F841 in the touched set.

### F401 (unused imports)
- `ruff check --fix --select F401` on each file, one at a time. The batch run hit `os error 1224` (user-mapped section open) on about 23 files. This is a file lock held by another process, and it was transient. Per-file retries succeeded.
- Five clip files had a dead header pair `from pathlib import Path as _Path` and `import sys as _sys`. Removed with Edit: `recap`, `practice`, `solar-gains`, `internal-gains`, `calculation`. `calculation` also lost `import math`.
- The `_Path` and `_sys` aliases are not used in any of these files and not imported by other modules (grep of `import .* _sys|_Path` under architecture: no importers).

### F841 (unused locals)
Script: `🧰️hc/f841_fix.py`. It uses exact-text substitutions. Each one asserts exactly one match, then writes through a temp file and `os.replace`.

Rules applied:
- Pure right-hand sides (dict or attribute reads, comprehensions, lambdas, `max(...)`, `next(...)`, `FACTORIAL[...]`) were deleted outright.
- Calls that build and arrange or move Mobjects were kept as expression statements, so their side effects still run. Examples: `chips`, `legend`, `loss_box`, `shelf`, `equation`, `none` (grasshopper `VGroup()` deleted, no side effect).
- Guards were kept. `language()` raises when no language is authored (`🎨️visuals/🐍️.py:63`). `self._german()` calls it. In `optimization/📉️extrema`, `self._german()` is kept as a bare call. In `experiments` and `index` the `self.language()` call is kept as a bare call.
- Tuple unpacking: the unused name was dropped from the tuple, for example `br, t = ...` in `regulations` and `conduction`, `(xb, _)` in `information`, `bl, br, tr, roof_peak = ...` in `conduction`.

Removed locals per file (ruff's F841 line in parentheses):
- `⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` (363) `components`. The multi-line `VGroup(...)` statement was deleted.
- `⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` (1086) `chips`. Kept as an expression.
- `⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py` (362, 383) `tr`, `tl`.
- `⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` (936) `legend`. Kept as an expression.
- `⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py` (721) `rad_x`.
- `⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py` (405) `loss_box`. Kept as an expression. (1318) `floor_y`.
- `⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py` (319) `tr`. (1022) `tl`.
- `⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (2744) `br`. (3160) `shelf`, kept as an expression.
- `🎯️optimization/🎬️clip/⚖️index/🐍️.py` (526) `german`, with `self.scene.language()` kept as a bare call. (969) `cost` lambda. (1185) `equation`, kept as an expression. (1284) `free`. (1578) `flipped`.
- `🎯️optimization/🎬️clip/📉️extrema/🐍️.py` (784) `german`, with `self._german()` kept as a bare call.
- `🎯️optimization/🎬️clip/🧪️experiments/🐍️.py` (688) `effects`. (972) `german`, with `self.language()` kept as a bare call.
- `🎯️optimization/🎬️clip/🧪️tests/🐍️.py` (191) `head`.
- `🏗️bim/🎬️clip/📊️information/🐍️.py` (578) `yb`. (799) `slab_t`.
- `🏗️bim/🎬️clip/🔌️exchange/🐍️.py` (612) `analysis`. (620) `slab`. (743) `mine`.
- `🏗️bim/🎬️clip/🔗️consistency/🐍️.py` (498, 790) `span`. (762) `num`. (810) `entry`.
- `🏗️bim/🎬️clip/🧱️model/🐍️.py` (3138) `gx`. Only `gy` kept.
- `🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py` (639) `none`.

Caution noted: `🏗️bim/🎬️clip/🧱️model/🐍️.py` is also the BIM model file that OI-01 names (reserve fix, model owner). Only the one `gx` line was touched there.

## Verification

- `compile_touched.py` (ticket-local, uses `compile()` on source bytes, no `__pycache__`): 48 OK, 0 failing, 2 skipped (missing on disk).
- Staffage guard: `m9_tests.sh 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only`. Ran 20 tests, OK, 236 s.
- Visuals tests: `m9_tests.sh 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️`. Ran 7 tests, OK, 0.3 s.

## Still open

- Two touched paths are missing on disk, so they were skipped. `❄️cooling/🎬️clip/👥️internal-gains/🏃️activity/🧪️tests/🐍️.py` (OI-106, deleted per git state) and `🏗️authoring/🎨️visuals/🧪️tests/🐍️silhouettes.py`.
- The files in the ticket still change under other agents. Concurrent edits were visible in `solar-radiation` and in the `internal-gains` and `calculation` headers during this pass. The fixes here are scoped to the lines named above.
- The F401 and F841 result is for the 50-file ticket list only. Other clip files under architecture that were not touched by this ticket were not scanned.
- The `repo` binary had no ruff tool registered, so ruff ran directly. If the team wants ruff under `repo tools exec`, that registration is needed first.
- Not run: the full unittest suites for the clip modules (for example BIM, optimization). The brief limited verification to the compile check, the guard and the visuals tests.
- Scripts kept in the ticket: `🧰️hc/eol_fix.py`, `🧰️hc/f841_fix.py`, `🧰️hc/compile_touched.py`, `🧰️hc/touched-py.txt` (input list). Tool logs were deleted, and `🗑️generated/hc` was removed.
