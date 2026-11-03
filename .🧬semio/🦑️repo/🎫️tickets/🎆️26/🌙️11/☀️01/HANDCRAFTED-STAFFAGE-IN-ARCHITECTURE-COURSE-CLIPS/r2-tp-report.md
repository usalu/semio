# r2-tp: thermal physics of the energy-clip casts

S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, T = this ticket. Scripts in `T/🧰️tp/` (`probe.py` exports all thermal casts and prints index timing, `temps.ts` prints region surface temperatures along a cast's timeline through the solver, `sheet.py` makes first/middle/last contact sheets with PyMuPDF, `thermal.py`/`poses.py`/`grep_json.py` dump asset thermal data, `patch_*.py` the edits).

## Method
All 62 distinct (asset, clip/pose, environment) combinations of the manifest were solved at 0/5/10/25/50/100 % of their clip and compared with physical limits; changed casts were re-exported (134 thermal casts) and rasterised (first, middle, last frame) and looked at.

## Findings and fixes

Environments (manifest `V/🧍️staffage/🔣️.json`, new preset in `S/🗂️catalog/🌤️environments/🔣️.json`):
- Split-AC indoor units were in `summer-heatwave-calm` (38 degC air, 900 W/m2 sun) although they hang in a room; the asset is rated for 27 degC indoor. New preset `indoor-warm-summer` (27 degC, MRT 28, 50 %, ISO 5151 reference added). Used by `optimization-room-cooler`, `optimization-extrema-cooler`, `cooling-heating-comparison-cooler`. Heatwave stays only for the fanning occupants (sweating 204 W latent, correct) and the blind has `summer-hot-sun`.
- Cooling `internal-gains` figures and devices (10 casts) were at 22 degC office neutral: now `indoor-warm-summer`. Heating `internal-gains`, `conduction`, `calculation` laptop and lamp (11 casts) are now `indoor-heated-winter`.
- The summer overheating beat of `heating-comparison` used winter devices next to the fanning occupant. New casts `cooling-heating-comparison-{toaster,laptop,lamp}-summer`, chosen by `overheated` in `V/../⚖️heating-comparison/🐍️.py` (`season`).
- Evaporator cast (`physics-fundamentals-evaporator`) sat in a 22 degC office although the scene says outside air 0 degC: now `winter-overcast`; condenser now `indoor-heated-winter`.

Asset bindings (steady surface temperatures from the solver, before / after):
- `finned-heat-exchanger`: evaporator fins -12.9 / -2.9 degC in cold outdoor air (about 5 degC in a 22 degC room), condenser fins 57 / 46 degC, tubes 52 / 51, headers 65 / 52. Before, fins were hotter than the tubes feeding them. Cold level -0.7, fins 340 W, headers 2 W, and skin heat capacities cut to 300/300/150 J/m2K (the fins took 6 min to follow the water; the summer-strategy heater now warms 22 to 41 degC in the 60 s clip). Pose texts en/de.
- `electric-fan-heater`: top cover 164 and grille 141 degC steady (melts ABS) now 70 / 73 degC; cover 22 W, grille 20 W, jet rise 49.1 / 50.7 K (energy balance 2000 W kept; texts en/de).
- `vacuum-cleaner`: exhaust grille 114 now 49 degC (7 W, exhaust air 543 W at +15 K).
- `laptop`: underside 64 now about 47 degC at 45 W (14/14/17 W). `server-rack`: switch 83 now 32 degC, servers 100 W, switch 10 W, exhaust +15.1 K.
- `split-ac-indoor-unit`: at the rated 27 degC the vane went to 4 degC, below its own 14 degC jet. Casing -20 W (3 K below air), vane -11 W (10 K below air, as the authors' text states).
- `cooking-pot`: the "heat to boil" clip ended with 46 degC water. Water 465 W, base 75 W, wall 48 W, skin capacities lowered: water 101 degC at 240 s (matches the scene: `POT_BOIL_RATE` 66 reaches 240 s at the 419 kJ mark), base 117 degC above it.
- `duct-fan`: hub steady 201 degC now 77 degC at 28 W electric (5 W hub, jet +0.3 K).
- `seated-office-worker` typing pose 1.2 met to 1.1 met (ISO typing; 135 W for this 1.94 m2 man now about 124 W; sleeper 81 W and standing 117 W unchanged).

Rates and timelines (scenes):
- `physics/fundamentals`: the vacuum ran at rate 1 on a 1500 s clip (always cold, first coarse frames). `VACUUM_RATE` 60 for `_vacuum()`, and the device icon. The vacuum, kettle and shower icons now start with rate 0 and start playing at the click on the start chip (they used to warm before the click).
- Verified, no change: boilers `BOILER_RATE` 60 (900 s in 15 s, shell steady at 7.5 s), toasters 8/12 (toasting phase 150 s ends at 19/12 s), laptop 24 (600 s in 25 s), cooler 45 (600 s in 13 s, vane 27 to 18 degC), rack 30, lamp warm-up 12, shower 40, kettle 60, heater 3, heat pump and pumpjack loops at rate 1. Tank and valve are driven by trackers.
- Radiator `thermostat-cycle` (40 s) is thermally static on purpose: panel time constant is about 30 min (surface changes under 1 K in the clip), so a switching radiator would need a minutes-long clip; it plays as a steady heater.

Breathing/plumes: occupant casts with `pathlines` are 5 s idle-breathing loops (12 breaths/min) in `office-neutral` / `indoor-heated-winter`; not changed.

## Verified
- Catalog check (`S/🔨️modules/🗂️catalog/🩺️check`) for the 9 touched assets: all pass.
- Staffage vitest (alias config): 51 files, 1030 passed, 11 skipped.
- Python: `🧍️staffage.🧪️tests.🐍️` plus `🧪️tests.🐍️staffage_only`: 54 tests, 1 failure outside my area (`test_rule_a_allowlist_has_a_reason_and_no_stale_entries`: stale allowlist entry `🏭️primary-energy::_plant_panel`, another agent's edit). The two edited clip modules compile.

## Open
- `electric-kettle` asset was edited by another agent while I worked; not touched.
- Cylinder jacket stratification is real but small (25.6 to 22.8 degC through insulation); not changed.
- The thermal palette is centred near 22 degC, so the 27 degC indoor start frame looks warm orange.
- Scene lengths were judged from the clips' narration structure, not measured from audio.
