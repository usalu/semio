# Final Checks (2026-10-29, evening)

Scripts: `final-run.sh` (tests), `final-cap.sh <en|de>` (layout captures, 8 non-trees scenes sequentially per language). Outputs: `🗑️generated/final/` (`*.log`, `*.exit`, `*.dur`), frames in `🗑️generated/frames/<series>/<Scene>/<lang>/`.

## Tests

| Suite | Tests | Result | Owner of failures |
|---|---|---|---|
| nakagin python | 59 | 2 FAIL | foreign (codex remodel of 🏢️nakagin) |
| nakagin node (`🟨️.cjs`) | n/a | exit 1: Ajv strict mode, missing type "object" at `#/properties/capsule/properties/interior/properties/corner_radius/allOf/1` in the nakagin schema | foreign (nakagin schema is being edited) |
| grasshopper python | 64 | OK | |
| grasshopper node | n/a | OK | |
| grasshopper registry | see log | OK (exit 0) | |
| excel python | 85 | OK | |
| excel node | n/a | OK | |
| excel registry | see log | OK (exit 0) | |
| morph toolkit (`🔀️morph/🧪️tests`) | 37 | OK (needs `PYTHONPATH=/c/git/semio`) | |
| grasshopper presentation | pending | running in background (`g-presentation.log`/`.exit`) | |
| excel presentation | pending | running in background (`x-presentation.log`/`.exit`) | |

Nakagin python failures (both foreign): `AssemblyTests.test_all_six_families_have_closed_parts_and_correct_window_wall` and `AssemblyTests.test_capsule_parts_match_portable_contract` (`shell-back` used 4 times instead of 2). The named known failure `test_capsule_row_positions_follow_primary_phase_datums` did not fail in this run. No scene or toolkit failure; nothing fixed.

## Layout captures

Machine is heavily loaded (parallel presentation suites, other renders, the slow `tower_axon` occlusion loop in `🏢️nakagin/🎨️visuals/🐍️.py`): about 20 s per animation step. At write time only GH `Beat1_Anatomy` (en and de) was mid-run with 21 of its frames saved; no `layout.md` produced yet, so no layout findings and no scene edits. Not finished: all 8 scenes x 2 languages (the captures keep running in the background via `final-cap.sh`; each scene writes `layout.md` and `cap-<Scene>-<lang>.exit` when done; check `cap-en.done` / `cap-de.done`). Older `layout.md` files under `🗑️generated/frames` (Tower 17:00, Program en 19:43) predate the current sources and are not valid.

## Fixes

None made.

## Update: first finished capture (GH Beat1_Anatomy, en)

`🗑️generated/frames/🦗️grasshopper/Beat1_Anatomy/en/layout.md`: 59 strict findings in 18 of 24 frames. Frame `hook`: the viewport shows the whole tower (two roofs, podium) although it is labelled "1 of 140 capsules"; the roof glyphs run under the viewport title and the podium touches the caption box. Cause is in the tower/capsule drawing produced by `🏢️nakagin/🎨️visuals/🐍️.py` (foreign, mid-edit, not touched), not in the scene file. Not fixed; re-check after the nakagin remodel settles. Other frames of this scene and all other scenes remain unreviewed; captures still running.
