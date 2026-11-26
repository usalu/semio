FINAL

# 🧭️ Multiview Lane MU: the 145 Assets No Batch Owns (2026-11-26 01:55)

Third fixer of the lane. Every figure below was read in a log that ran to its end; what did not run is named so. `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `W` = `🧰️matrix/MU` in this ticket.

List `🧰️matrix/batches/MU.tsv` (written by `W/list.py`: 375 catalog assets minus every asset of `batches/M*.tsv` minus `electric-fan-heater`): components 37, objects 31, vegetation 22, appliances 21, furniture 20, machinery 8, vehicles 3, humans 2, animals 1.

## 1. Result

- **143 of 145 assets are clean**: authors' gate `T/🔎️scan.ts` without a hard finding (123 non-plants at 20:05, 22 plants at 00:4x, every repaired asset again after its landing) and matrix `T/📊️matrix.ts --assets <id>` without a `fail` cell. Every one of the 145 records is newer than its catalog file (checked 01:50; `W/matrix1.sh` re-ran each asset whose file another lane had changed).
- **2 assets are open, 29 fail cells** (section 3): `used-bentwood-dining-chairs` 22, `used-timpani-pair` 7.
- Cells over the 145 records: 29,763 ok, 5,299 group-drift, 924 unavailable, 378 range-end, 29 fail.
- 12 assets were changed (section 2); 133 needed nothing.

## 2. Repaired assets

Chain per asset: `W/make/<writer>.py` writes `W/ops/<id>.json` where a writer exists, `W/apply.sh <id>` applies it with `fix.ts` twice (the second run reported "file unchanged" for every asset). Proofs per asset: scan clean, `validateAsset` 0 issues, check CLI `✓`, matrix (last column), and the suites of section 4.

| Asset | Before | Repair | Matrix after |
|---|---|---|---|
| `used-electric-bass-and-cabinet` | fixer's gate 120 hard (flat castor wheels) | `ops`: 16 wheel shapes solid (predecessor, 16:25) | 252 ok |
| `reclaimed-cobblestone-bag` | fixer's gate hard in the transition reinstalled > stored | `make/reclaimed-cobblestone-bag.py` (predecessor) | 252 ok |
| `reclaimed-paving-slab-pallet` | same kind | `make/reclaimed-paving-slab-pallet.py` (predecessor) | 252 ok |
| `excavator` | scan 3 hard: 1:500 did not print (`bucket-link` 0.18 mm) | `make/excavator.py` (predecessor, 17:37) | 175 ok, 56 group-drift, 21 range-end |
| `delivery-van` | fixer's gate 86 hard: flat tyres on the spinning wheel bones (`sync-height` 220 mm, `anchor-landmark` 259 mm; the front drew no tyres a quarter turn on) | `make/wheels.py`: tyres carried by the axle bones | 210 ok, 42 range-end |
| `electric-kick-scooter` | fixer's gate 8 hard (1:200 rear wheel, 108 mm) | `make/wheels.py`: the tyre carried by the deck | 161 ok, 70 group-drift, 21 unavailable |
| `used-timpani-pair` | fixer's gate 3 hard (1:500 mallets, 52 mm) | `make/wheels.py`: tyres on the castor bones, mallets solid at 1:500 | 175 ok, 70 group-drift, **7 fail** (section 3; not in the repaired views) |
| `lavender-bush` | matrix 7 fail `derived-view-in-band` (1:500 symbol: front equal to side, 0.0197) | `ops/lavender-bush.json`: the front of the symbol shows four spikes over the 0.76 m face instead of the five of the 0.80 m side | 91 ok, 140 group-drift, 21 unavailable |
| `running-child` | matrix 14 fail `sync-bone-height` (1:500 right forearm, 23.4 mm against 21.9 mm) | `ops/running-child.json`: the side elevation takes the body of revolution of front and plan (the other direction raised `volumes-feature` twice and was dropped) | 56 ok, 196 group-drift |
| `sunbather-on-towel` | matrix 70 fail (`sync-bone-height` 56, `sync-height` 56, `sync-lateral` 28): the roll twists the pelvis by 180 degrees | `make/sunbather-on-towel.py`: legs, feet, toes, head swept laterally in the side elevation; torso and pelvis as stepped section; plan hands as the bodies of the front; hand sweep 10 and 8 mm longer in the front (1:50, 1:100), to 0.148 m in the plan | 98 ok, 91 group-drift, 63 range-end |
| `used-bentwood-dining-chairs` | fixer's gate 120+ hard at every band (two chairs turn by 180 degrees) | `make/used-bentwood-dining-chairs.py`: 1:50, 1:100, 1:200 repaired; 1:500 left as it was | 230 ok, **22 fail** (section 3) |
| `tram` | `🕳️fill-rule` 8 overlaps (bogie legs, 1:50 and 1:100 side elevation) | `🧰️matrix/ML-4/fill-rule.py tram` (prepared by ML-4), run by `W/chain/tram.sh`, second run "unchanged" | 28 ok, 224 group-drift |

Notes on single proofs:

- Print: every authored scale prints for all twelve. Range ends that do not print, each the same before the repair (the trial said "print: identical"): `excavator` 1:20 (`mirror-arm-right-2` 0.193 mm), `delivery-van` 1:150 (`tail-pipe` 0.141 mm), `sunbather-on-towel` 1:350 and 1:1000 (`pelvis-1`, `neck-2`, `pelvis-2`, `torso-1`, 0.18 to 0.198 mm).
- Silhouette guard of the trials (`MR/measure.ts`, limit IoU 0.92): unchanged or within the limit for all; the closest is `sunbather-on-towel` front at 1:100 with 0.9207 (was 0.9221), then 1:50 with 0.9356 (was 0.9375).
- The write of `used-bentwood-dining-chairs` of 17:33 (predecessor: 4 validate issues, check `✗`, failing silhouette) was taken back at 19:02 before the present repair was landed.
- Sheets opened: `delivery-van`, `used-timpani-pair`, `sunbather-on-towel` (trial and landed). Rendered but not opened at a readable size: `electric-kick-scooter`, `running-child`, `used-bentwood-dining-chairs` (1:50 to 1:200), `tram` (the change is a patch of 10 x 20 mm on an 18 m vehicle; the overlap oracle of `🕳️fill-rule` is its proof). No sheet: `lavender-bush` (the sheet tool does not resolve the symbol), and none by this fixer for the four predecessor assets.
- `ops/tram.json` of the predecessor (wheel rebinding, never applied) was removed: the fixer's gate had no hard finding on the tram.

## 3. Open

| Asset | Fail cells | Finding | Why not cured here |
|---|---|---|---|
| `used-bentwood-dining-chairs` | 22 | 1:500 only: `sync-bone-height` (90 subjects, worst 231.7 mm), `sync-bone-forward` (42, 413.2 mm), `sync-forward` (8, 146.4 mm), `sync-height` (1, 28.2 mm), in 21 cells; plus 1 cell `language-geometry` at `500:band-max`, side elevation, mode `forces` | The cure exists (`make/used-bentwood-dining-chairs.py --with-500`: the tubes as the lofts of the 1:500 volume; gate 0 hard, prints) but drops the silhouette guard of the front to 0.905 (limit 0.92): the slender solids are not compared at the width of a loft drawing. Request 1. The `language-geometry` cell lies in the untouched band; request 2. |
| `used-timpani-pair` | 7 | `paper-height` at `500:band-max` (1:1000), side elevation, pose `stored`: drawn 0.91 mm against 0.93 mm (18.6 mm) | The 1:500 side elevation is byte-equal to the document before the repair and passes at 1:500; cause at the band end not established. Request 2. |

Requests filed in `requests.md`:

1. `## multiview MU — silhouette guard: a slender solid does not follow a loft (or member) drawing, so used-bentwood-dining-chairs cannot be cured at 1:500`
2. `## multiview MU — two residual matrix findings at the 1:500 band end (1:1000) on drawings no repair touched`

## 4. Suites (vitest with the name filter of the repaired assets, `W/steps.sh suites`)

| Suite | 12 names, 00:2x to 00:5x | `sunbather-on-towel` and `tram` after their landing, 01:3x | `sunbather-on-towel` after its last landing, 01:4x |
|---|---|---|---|
| `🧊️catalog-silhouette` | 12 passed | 2 passed | 1 passed |
| `🧊️three-views` | 24 passed | 4 passed | 2 passed |
| `🌡️catalog-physics` | 26 passed | 5 passed | 3 passed |
| `🫁️breathing-catalog` | 6 passed | 3 passed | 3 passed |
| `✅️asset-validation` | 10 passed | 10 passed | 10 passed |
| `⚡️asset-power` | no test matches the names (24 skipped) | same | same |
| `🕳️fill-rule` (whole) | 1 failed of 11 (before the tram repair) | 11 passed of 11 | not run again |

The first column ran on the landed files of all repaired assets except `sunbather-on-towel` and `tram`, which the later columns cover. No suite ran without a name filter except `🕳️fill-rule`.

## 5. Files

Updated (catalog, `S/🗂️catalog/…/🔣️.json`): `🔌️appliances/used-electric-bass-and-cabinet`, `🧱️components/reclaimed-cobblestone-bag`, `🧱️components/reclaimed-paving-slab-pallet`, `🏗️machinery/excavator`, `🚜️vehicles/delivery-van`, `🚜️vehicles/electric-kick-scooter`, `🚜️vehicles/tram`, `🪑️objects/used-timpani-pair`, `🌳️vegetation/lavender-bush`, `👤️humans/running-child`, `👤️humans/sunbather-on-towel`, `🛋️furniture/used-bentwood-dining-chairs`.

Created or updated in this ticket: `multiview-MU.md`; `requests.md` (two sections appended); `🧰️matrix/batches/MU.tsv`; `🧰️matrix/MU/` with the tools (`env.sh`, `q.sh`, `capped.sh`, `worker.sh`, `matrix1.sh`, `run-matrix.sh`, `gates.sh`, `steps.sh`, `apply.sh`, `apply.ts`, `fix.ts`, `trial.ts`, `try.sh`, `land.sh`, `measure.sh`, `look.sh`, `sheet.ts`, `print.ts`, `validate.ts`, `why.ts`, `where.ts`, `at.ts`, `files.ts`, `lib.py`, `mc.py`, `list.py`, `table.py`, `flat.py`, `turns.py`, `rig.py`, `await.sh`), `chain/` (`scan-rigid.sh`, `trees-rest.sh`, `tram.sh`, `last.sh`, `sun.sh`), `make/` (`excavator.py`, `reclaimed-cobblestone-bag.py`, `reclaimed-paving-slab-pallet.py`, `sunbather-on-towel.py`, `used-bentwood-dining-chairs.py`, `wheels.py`) and `ops/` (eleven files, one per repaired asset except `tram`).

Removed: `🧰️matrix/MU/ops/tram.json`; `🗑️generated/matrix/MU` (tool output, after this report).

## 6. Matrix record per asset

| Asset | Outcome | Cells | Failing checks |
|---|---|---|---|
| `agave` | done | 231 group-drift, 21 range-end |  |
| `atlas-cedar` | done | 252 ok |  |
| `banana-plant` | done | 189 ok, 63 range-end |  |
| `bar-stool` | done | 63 group-drift, 189 ok |  |
| `cantilever-balcony` | done | 56 group-drift, 196 ok |  |
| `chalkboard` | done | 252 ok |  |
| `cherry-laurel` | done | 252 ok |  |
| `circulation-pump` | done | 133 group-drift, 98 ok, 21 unavailable |  |
| `concrete-mixer-truck` | done | 28 group-drift, 182 ok, 42 range-end |  |
| `cooking-pot` | done | 231 ok, 21 unavailable |  |
| `date-palm` | done | 56 group-drift, 196 ok |  |
| `delivery-van` | done | 210 ok, 42 range-end |  |
| `desktop-monitor` | done | 231 ok, 21 unavailable |  |
| `drinking-fountain` | done | 252 ok |  |
| `electric-kettle` | done | 231 ok, 21 unavailable |  |
| `electric-kick-scooter` | done | 70 group-drift, 161 ok, 21 unavailable |  |
| `electricity-meter` | done | 231 ok, 21 unavailable |  |
| `english-yew` | done | 252 ok |  |
| `excavator` | done | 56 group-drift, 175 ok, 21 range-end |  |
| `expansion-valve` | done | 210 ok, 42 unavailable |  |
| `fiddle-leaf-fig` | done | 231 ok, 21 unavailable |  |
| `finned-heat-exchanger` | done | 231 ok, 21 unavailable |  |
| `folding-deck-chair` | done | 252 ok |  |
| `gas-boiler` | done | 231 ok, 21 unavailable |  |
| `gas-meter` | done | 231 ok, 21 unavailable |  |
| `ginkgo` | done | 252 ok |  |
| `glass-balustrade` | done | 70 group-drift, 161 ok, 21 unavailable |  |
| `grapevine-row` | done | 252 ok |  |
| `height-adjustable-desk` | done | 56 group-drift, 196 ok |  |
| `honey-locust` | done | 252 ok |  |
| `italian-cypress` | done | 252 ok |  |
| `japanese-cherry` | done | 252 ok |  |
| `laptop` | done | 231 ok, 21 unavailable |  |
| `lavender-bush` | done | 140 group-drift, 91 ok, 21 unavailable |  |
| `lombardy-poplar` | done | 252 ok |  |
| `london-plane-pollarded` | done | 252 group-drift |  |
| `market-parasol` | done | 42 group-drift, 210 ok |  |
| `masonry-wall-with-lintel` | done | 252 ok |  |
| `mvhr-unit` | done | 231 ok, 21 unavailable |  |
| `oil-pumpjack` | done | 252 ok |  |
| `operating-table` | done | 252 ok |  |
| `panel-radiator` | done | 231 ok, 21 unavailable |  |
| `picnic-table` | done | 168 group-drift, 84 ok |  |
| `playground-swing` | done | 84 group-drift, 168 ok |  |
| `prickly-pear` | done | 252 ok |  |
| `privet-hedge` | done | 252 ok |  |
| `reclaimed-brick-pallet` | done | 70 group-drift, 182 ok |  |
| `reclaimed-cobblestone-bag` | done | 252 ok |  |
| `reclaimed-concrete-wall-panel` | done | 252 ok |  |
| `reclaimed-floorboard-stack` | done | 70 group-drift, 182 ok |  |
| `reclaimed-glulam-girder` | done | 252 ok |  |
| `reclaimed-granite-kerbs` | done | 252 ok |  |
| `reclaimed-hollow-core-slab` | done | 252 ok |  |
| `reclaimed-lattice-girder` | done | 252 ok |  |
| `reclaimed-oak-beam` | done | 252 ok |  |
| `reclaimed-parquet-crate` | done | 252 ok |  |
| `reclaimed-paving-slab-pallet` | done | 252 ok |  |
| `reclaimed-rafter-bundle` | done | 252 ok |  |
| `reclaimed-roof-tile-pallet` | done | 252 ok |  |
| `reclaimed-sandstone-window-surround` | done | 252 ok |  |
| `reclaimed-steel-i-beam` | done | 252 ok |  |
| `reclaimed-steel-stair` | done | 252 ok |  |
| `reclaimed-stone-steps` | done | 252 ok |  |
| `reclaimed-timber-roof-truss` | done | 147 group-drift, 105 ok |  |
| `red-squirrel` | done | 168 group-drift, 42 unavailable |  |
| `retractable-bollard` | done | 168 group-drift, 84 ok |  |
| `room-thermostat` | done | 210 ok, 42 unavailable |  |
| `running-child` | done | 196 group-drift, 56 ok |  |
| `scissor-lift` | done | 42 group-drift, 189 ok, 21 unavailable |  |
| `server-rack` | done | 252 ok |  |
| `shopping-trolley` | done | 98 group-drift, 154 ok |  |
| `shower-head` | done | 84 group-drift, 147 ok, 21 unavailable |  |
| `shrub-rose` | done | 126 group-drift, 126 ok |  |
| `snake-plant` | done | 231 ok, 21 unavailable |  |
| `split-ac-indoor-unit` | done | 231 ok, 21 unavailable |  |
| `street-lamp` | done | 252 ok |  |
| `sunbather-on-towel` | done | 91 group-drift, 98 ok, 63 range-end |  |
| `sweetgum` | done | 252 ok |  |
| `thermostatic-radiator-valve` | done | 210 ok, 42 unavailable |  |
| `three-seat-sofa` | done | 168 group-drift, 84 ok |  |
| `tied-canopy` | done | 252 ok |  |
| `tilt-window` | done | 252 ok, 21 unavailable |  |
| `toaster` | done | 210 ok, 21 unavailable |  |
| `tower-crane` | done | 210 ok, 42 range-end |  |
| `tower-server` | done | 231 ok, 21 unavailable |  |
| `tram` | done | 224 group-drift, 28 ok |  |
| `tree-fern` | done | 189 ok, 63 range-end |  |
| `used-accordion` | done | 84 group-drift, 147 ok, 21 unavailable |  |
| `used-baby-cot` | done | 231 ok, 21 unavailable |  |
| `used-baby-grand-piano` | done | 252 ok |  |
| `used-backrest-park-bench` | done | 252 ok |  |
| `used-bathtub` | done | 252 ok |  |
| `used-beer-garden-set` | done | 14 group-drift, 238 ok |  |
| `used-bentwood-dining-chairs` | done | 22 fail, 230 ok | sync-forward 14, sync-bone-height 14, sync-bone-forward 14, sync-height 14, language-geometry 1 |
| `used-brine-water-heat-pump` | done | 231 ok, 21 unavailable |  |
| `used-bubbler-stone-fountain` | done | 252 group-drift |  |
| `used-buffer-storage-tank` | done | 252 ok |  |
| `used-bunk-bed` | done | 252 ok |  |
| `used-cast-iron-sectional-boiler` | done | 231 ok, 21 unavailable |  |
| `used-chess-table` | done | 252 ok |  |
| `used-chest-of-drawers` | done | 168 group-drift, 84 ok |  |
| `used-coat-stand` | done | 252 group-drift |  |
| `used-concrete-bench-block` | done | 252 ok |  |
| `used-concrete-table-tennis` | done | 252 ok |  |
| `used-corner-sofa` | done | 252 ok |  |
| `used-district-heating-substation` | done | 252 ok |  |
| `used-dog-waste-station` | done | 252 ok |  |
| `used-double-bed` | done | 252 ok |  |
| `used-dry-rotor-inline-pump` | done | 231 ok, 21 unavailable |  |
| `used-electric-bass-and-cabinet` | done | 252 ok |  |
| `used-expansion-vessels` | done | 252 ok |  |
| `used-floor-lamp` | done | 252 ok |  |
| `used-insulated-copper-coils` | done | 231 group-drift, 21 unavailable |  |
| `used-litter-bin-post` | done | 28 group-drift, 224 ok |  |
| `used-marching-snare-and-bass-drum` | done | 252 ok |  |
| `used-moving-boxes` | done | 210 ok, 21 unavailable |  |
| `used-oil-filled-radiator` | done | 210 ok, 21 unavailable |  |
| `used-outdoor-fitness-station` | done | 28 group-drift, 224 ok |  |
| `used-panel-radiator-pallet` | done | 231 ok, 21 unavailable |  |
| `used-park-noticeboard` | done | 252 ok |  |
| `used-pump-drinking-fountain` | done | 252 ok |  |
| `used-radiant-ceiling-strip` | done | 252 ok |  |
| `used-recycling-bin-cluster` | done | 252 ok |  |
| `used-seesaw` | done | 252 ok |  |
| `used-sewing-machine` | done | 210 ok, 21 unavailable |  |
| `used-slide-tower` | done | 252 group-drift |  |
| `used-stage-piano` | done | 252 ok |  |
| `used-steel-oil-storage-tanks` | done | 252 ok |  |
| `used-steel-pipe-rack` | done | 252 ok |  |
| `used-steel-string-acoustic-guitar` | done | 210 ok, 21 unavailable |  |
| `used-sun-lounger-bench` | done | 252 ok |  |
| `used-timpani-pair` | done | 7 fail, 70 group-drift, 175 ok | paper-height 7 |
| `used-tree-surround-bench` | done | 70 group-drift, 182 ok |  |
| `used-trumpet` | done | 231 ok, 21 unavailable |  |
| `used-underfloor-heating-manifold` | done | 70 group-drift, 182 ok |  |
| `used-upright-piano` | done | 168 group-drift, 84 ok |  |
| `used-violin` | done | 231 ok, 21 unavailable |  |
| `used-washbasin-vanity` | done | 210 ok, 21 unavailable |  |
| `used-wayfinding-signpost` | done | 252 ok |  |
| `used-writing-desk` | done | 168 group-drift, 84 ok |  |
| `used-xylophone` | done | 252 ok |  |
| `vacuum-cleaner` | done | 252 ok |  |
| `wheel-loader` | done | 147 group-drift, 84 ok, 21 range-end |  |
| `wheelchair-ramp` | done | 140 group-drift, 112 ok |  |
| `yucca` | done | 252 ok |  |
