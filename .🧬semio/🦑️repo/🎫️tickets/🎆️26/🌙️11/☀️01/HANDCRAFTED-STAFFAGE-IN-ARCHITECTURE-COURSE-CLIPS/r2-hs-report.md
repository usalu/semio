# Round 2 style pass (agent hs)

Scope: every changed `🐍️.py` and `🟦️.ts` under `🎓️teaching/🏛️architecture` (54 files from `git status`, list in `🧰️hs/files.txt`). Helper scripts: `🧰️hs/audit_py.py` (AST audit), `🧰️hs/fix_style.py` (comment strip and verified function drop), `🧰️hs/compile_check.py`, `🧰️hs/import_clips.py`.

## Fixed

- Comments inside definitions: removed 361 comment lines from 14 `🐍️.py` files (solar-radiation, heating-comparison, summer-strategy, internal-gains (cooling), heat-flow, volume, natural, visuals, solar-gains, calculation, conduction, fundamentals (heating), fundamentals (physics), and the other listed clips). AST before and after is identical for every file, so only comments changed. TypeScript: no `//` or indented `/*` comments inside functions; only top-level `/** */` docstrings, all emoji-prefixed.
- Dead helpers dropped (defined, no caller in the architecture tree; verified by identifier count across all `🐍️.py`/`🟦️.ts` there):
  - `solar-radiation/🐍️.py` `_bell`
  - `solar-gains/🐍️.py` `_shade_factor`
  - `fundamentals (physics)/🐍️.py` `_droplets`
  - `visuals/🐍️.py` `radiation_wave`, `radiation_waves`, `solar_wave_ray`, `convection_stream` (chain, `convection_stream` was the last caller), `animate_flow`, `animate_haze`, `paced_flow`
- Duplicate lamella extractor consolidated into V: new `lamella_faces` (`visuals/🐍️.py:240`), used by `fundamentals (physics)/🐍️.py:447` (`_blind`) and `solar-gains/🐍️.py:833` (`_blind_slats`). The two inline copies were identical.
- Duplicate `_clock` consolidated into V: new `tracked_clock` (`visuals/🐍️.py:1001`), used by `natural/🐍️.py` (lines 491, 593) and `volume/🐍️.py` (line 745). The two copies were identical; the empty `#region Instrument glyphs` in volume was removed.
- `[DEBUG]` leftovers in `🏗️authoring/🎬️clip/🪟️presentation/🐍️.py`: three temporary `print("[DEBUG] ...")` lines removed, plus the now-unused `started` timer and the local `import time`.
- Docstrings: every docstring in the changed files starts with an emoji. The two AST hits were keycap emojis (`0️⃣`, `#️⃣`), which are valid emoji prefixes, so nothing changed there.

## Verification

- `compile_check.py`: 36 changed Python files, 0 failures.
- Clip import probe (`import_clips.py`, teaching uv env): solar-radiation, summer-strategy, natural, volume, solar-gains, fundamentals (physics), presentation all import OK.
- Visuals tests (`m9_tests.sh` with `🧪️tests.🐍️staffage_only` and `🧪️tests.🐍️`): Ran 25 tests, 5 failures, all in `test_rule_a_no_clip_builds_a_physical_object_from_primitives`. Failing functions (none touched by this pass, and not in `🔣️staffage_only.json`'s allowlist):
  - `system-sizing/🐍️.py` `_vent_unit`
  - `primary-energy/🐍️.py` `_meter_glyph`
  - `practice/🐍️.py` `_gas_meter`, `_thermo`
  - `fundamentals (physics)/🐍️.py` `_thermometer`
  These are migration gaps in the clip code, not style issues. Owner should migrate them or extend the allowlist with reasons.

## Incident

Python writes to `solar-gains/🐍️.py` failed (EINVAL on `w`, then PermissionError on truncate), which left the new text followed by a stale tail of about 9 lines. I cut the tail with Edit, then confirmed the file parses, has no `_shade_factor`, and imports. The later `_blind_slats` and `lamella_faces` edits were applied with Edit. Scripted writes now go through a temp file and `os.replace`.

## Open (not changed)

- Unused `sys` / `Path` / `json` / `math` imports in about 25 clip headers (ruff F401). This looks like a shared template header, so I left it.
- Unused local variables (ruff F841) in about 25 clips, e.g. `calculation` `loss_box`, `floor_y`, `presentation` `shelf`. Not in the rule list.
- `_blind_hit` (fundamentals) and `_first_slat_hit` (solar-gains) share ray/lamella maths but differ in semantics (positive-along requirement and fallback), so they were not merged.
- Several touched files have CRLF line endings in the working copy (git warns on `solar-radiation`, `fundamentals (heating)`, others); HEAD has LF. Left as found.
- `internal-gains/activity/🧪️tests/🐍️.py` is listed by git but no longer exists on disk (deleted).
