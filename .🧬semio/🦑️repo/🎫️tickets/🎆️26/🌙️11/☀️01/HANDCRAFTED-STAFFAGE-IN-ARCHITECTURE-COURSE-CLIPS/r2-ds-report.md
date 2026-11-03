# R2 AGENTS.md style pass (agent ds)

Scope: 84 `.py`/`.ts`/`.tsx` sources from `git status --porcelain -- "🎓️teaching/🏛️architecture"` (49 Python on disk, 35 TS/TSX). Scripts in `🧰️ds/` (`scan_py.py`, `scan_ts.py`, `scan_ts2.py`, `dead.py`, `dead2.py`, `del_comments.py`, `apply2.py`, `apply_ts.py`, `compile_all.py`). No git modifying commands.

## Fixed
- Comments inside bodies: 1 file, 10 lines (`⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py`: 4 prose comments, 3 `#region`, 3 `#endregion`). All other files had none. TS/TSX: zero body comments.
- Duplicate docstring emoji, Python (only members that this ticket's diff touched; one other holder keeps the emoji): 21 docstrings re-emojied in 20 files (internal-gains cooling, mechanical, volume, visuals `fitted_staffage` and `thermometer_glyph`, primary-energy, certificate, recap, solar-gains x2, heating internal-gains, calculation, conduction, heating fundamentals, physics fundamentals, optimization room `pendant` and `pendant_lit`, consistency, excel program, `staffage_only.py`).
- Duplicate docstring emoji, TS: 36 function/const docstrings re-emojied across 15 files (thermal, transient, balance 13, animation 4, thermal-view, transport-view, palette, depiction, sources 2, transport, svg, measure 2, representation, copies, evaluation, tests support).
- Debug output: no stray `print(`/`console.log`. Only `visuals/🐍️.py:2339` `[LAYOUT]` print (opt-in `LAYOUT_CHECK` diagnostic, intentional, left). The `[DEBUG] pf` hook was not touched.
- Dead helpers: 148 names unreferenced in their own file were grepped across the repo (rg, excluding venv, node_modules, md); every one is referenced elsewhere. Zero removals.

## Verification
- `python -m py_compile` on all 49 Python files: 49 compiled, 0 failed.
- `staffage_only` guard via `m9_tests.sh`: Ran 20 tests, OK.
- Edits only changed docstring emoji or deleted comment lines; EOL style preserved per file.

## Still open
- About 180 pre-existing duplicate docstring emoji groups in Python (heavy in `visuals/🐍️.py`, `solar-gains`, `fonts`, many clips) and interface-field JSDoc duplicates in staffage TS (`contracts`, `thermal`, `balance` fields) that predate this ticket's diff. Left to avoid mass churn in concurrently edited files.
- Three Python docstrings start with keycap/`±` glyphs (`physics fundamentals:210`, `consistency:306`, `:819`), treated as emoji-like and left.
