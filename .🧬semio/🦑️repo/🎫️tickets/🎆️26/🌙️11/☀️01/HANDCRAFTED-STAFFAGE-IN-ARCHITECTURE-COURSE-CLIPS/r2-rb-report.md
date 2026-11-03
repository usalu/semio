# R2 rb: BIM clip tests, staffage renders, non-IFC drawings

Scripts in `T/🧰️rb/`: `bimtest.py` (unittest runner, verbose), `setup_probe.py` (stack sampler), `exchange_still.py`, `cons_still.py` (stills with stubbed model facts), `render.sh`.

## 1. BIM suites: blocked by a model error, not by staffage

Both suites ran in the background until they stopped by themselves (about 2 h 12 min):

| Suite | Result |
|---|---|
| `BimInformationContract` | 0 tests run, `setUpClass` ERROR after 7942 s |
| `BimConsistencyContract` | 0 tests run, `setUpClass` ERROR after 7978 s |

Cause (identical in both): `ModelCase.setUpClass` (`🧪️tests/🐍️.py:287`) calls `building_geometry`. The exact-Fraction build of walls, slabs and cutters (`🧱️model/🐍️.py` `_wall_cutters`, `_slab_geometry_entry`) takes more than two hours here, and then fails at `🧱️model/🐍️.py:2268`: `Finish reserve RES-FIN-ROOF-GRAVEL-SL-R-705 is not fully occupied by its declared physical owners: exact missing volume ... m3` (about 0.00297 m3). That is the 9-minute "slowness" of earlier runs: the whole class setup is geometry, nothing staffage related (stack samples: `fractions`, `_native_material_cells`, `_side`, `deepcopy`).

- The model package, its JSON and the tests are unchanged in the working tree versus HEAD (`git status`/`git diff --stat -- 🏗️bim` lists only the three clip files). The failure therefore pre-exists our staffage change (latest model commit 651c804).
- Every scene that calls `building_geometry` (information `present_information`, consistency `_prepare`, exchange `_facts`) hits the same error, so no full-clip render of these clips is possible until it is fixed. The renders I started were killed.
- Also run: `BimClipContract` (cheap tests do not need geometry) hit the 560 s limit in `test_all_bilingual_clauses_fit_native_caption_layout...` (renders every clause), not finished.
- Open: whoever owns the model must fix the gravel reserve of SL-R (model JSON or `_finish_reservations`) and make `setUpClass` cheaper (cache or smaller fixture). Not fixed here: a probe takes over 2 h per attempt.
- Side result, unrelated to BIM: `staffage_only` + `integration` visuals tests: 27 run, 6 failures from other agents' concurrent work (`staffage_only` rule A for `_vent_unit`, `_meter_glyph`, `_thermo`, `_gas_meter`, `_thermometer` of energy clips, and `integration` `test_physics_scene_constructor_bounds` radiator height 0.3408 vs 0.408).

## 2. Staffage scenes

Full clips could not be rendered (above), so I rendered the call sites at their authored coordinates with stubbed model values.

- consistency people (edit A and B cards, decider, review row), en and de: staffage figures are grounded and upright. Fixed: the figure in the edit cards poked over the card top line. `🔗️consistency/🐍️.py` `edit_card`: person now `(x - 2.1, y + 0.74, 0), color, 0.4`. Fixed: the German label "eine Person entscheidet" overran the card B border; `width=2.9` became `width=2.5` (`_decide`).
- information car at the bay scale (`BAY_K`, 4.6 x 1.75 m, heading 270): stretched car reads correctly beside piloti circles. The plan placement inside the full scene could not be viewed, so the R3 verdict stands.

## 3. Non-IFC drawings in other BIM clips

Grep over all BIM clip sources (`📋️plan`, `🧊️axon`, drafting, analysis, representations, models, objects, exchange, model). Only one hit: `🔌️exchange/🐍️.py` `_beat_buildup` drew the imposed load as a circle on a stem ("people"). Fixed:

- New cast `bim-exchange-imposed-person` in `V/🧍️staffage/🔣️.json` (standing-adult, relaxed, dark-line).
- `_beat_buildup` now casts three true-size persons (1.75 m, `PERSON_HEIGHT`) standing on the finish layer; the section view scale goes from 2.2 to 1.3 so they fit, centre moved to (-3.25, 0.1), section title moved to y = 3.0, stems removed from the animation and the `off` list. `_art` import added.
- Viewed en and de with stubbed slab and loads: figures grounded, labels clear of the post label, caption bar clear. With the real slab (width 1.9 m) three figures fit.

Everything else in the BIM clips is IFC geometry (furnishings, stairs, ramps from the model) or diagram glyphs (arrows, rings, dots). The only vehicle is the information car (already staffage).

## Files changed
`🏗️bim/🎬️clip/🔌️exchange/🐍️.py`, `🏗️bim/🎬️clip/🔗️consistency/🐍️.py`, `V/🧍️staffage/🔣️.json`. Generated output removed.
