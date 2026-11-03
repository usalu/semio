# r2 vf2: energy clip Python tests after the cast-id merge

Runner: `T/m9_tests.sh "<module>"` per `T/🧰️ht/modules.txt` line, script `T/🧰️vf2/run.sh`.

| line | module | before | after |
|---|---|---|---|
| 1 | cooling internal-gains | 6 errors (KeyError `cooling-internal-gains-lamp/desk`) | 3 OK |
| 2 | cooling clip tests | 4 OK | 4 OK |
| 3 | ventilation volume | 2 OK | 2 OK |
| 4 | ventilation architecture | 5 OK | 5 OK |
| 16 | audience | 1 error ("Authored sources changed while rendering": concurrent edits, transient) | 7 OK |
| 17 | output | 1 error (import of demand practice) | 9 OK |
| 18 | clip VideoContractTest | 3 errors (practice import) | 31 OK (1 skipped) |
| 19-26 | presentation/browser/geometry/interaction/hierarchy/chapters | not failing | OK (19: 9, 20: 4, 21: 1, 22: 8, 23: 21, 24: 8, 25: 12, 26: 1) |
| 28-32 | presentation, stream, fonts, heading, regulations sun | not failing | OK (15, 3, 13, 3, 2) |

Line 3 (e03 "no result") ran OK now; the earlier crash was the mid-merge state or the uv resolution failure below.

## Fixes
- `❄️cooling/🎬️clip/👥️internal-gains/🧪️tests/🐍️.py` (cast check, ~l.37-48): cast ids now resolve through the `_SHARED` map of the clip source (`desk` -> `shared-desk`, `lamp` -> `shared-lamp-summer`), otherwise `cast_prefix + name`. Assertions unchanged.
- `📊️demand/🎬️clip/🧪️practice/🐍️.py:84`: `_Path` (undefined) -> `Path`; broke import of the practice scenes (lines 17/18).
- `🏗️authoring/📦️packages/🐍️python/pyproject.toml:27`: `pandas==3.1.0` -> `pandas==2.3.3`; `pyet==1.5.0` requires pandas<3, so every `uv run --group test` was unsatisfiable (another agent's pin addition; agent `or` may touch the same).

## Open
- `🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only.py:207` `test_thermal_span_every_scene_draws_its_thermal_casts_on_one_span` fails (19 pass) for `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`, spans [25, 78]. The module-level group holds `_vacuum`/`_shower_head` (`physics-fundamentals-vacuum/shower`, 78 K) next to `_lamp` (`shared-lamp-winter`) and the blind (25 K). In reality Beat2_EnergieImAlltag draws `physics-fundamentals-clerk` (25 K, l.987) together with `_vacuum()` (l.1044) and `_shower_head` (l.1090) at 78 K. Real fix needs a design decision: a 78 K clerk (and lamp) in that scene, or a 25 K vacuum/shower; not done, to avoid hiding the mismatch by moving helpers into a class. Owner: physics clip / visuals.
- `--lang` removal (agent lg) did not affect any of my modules.
