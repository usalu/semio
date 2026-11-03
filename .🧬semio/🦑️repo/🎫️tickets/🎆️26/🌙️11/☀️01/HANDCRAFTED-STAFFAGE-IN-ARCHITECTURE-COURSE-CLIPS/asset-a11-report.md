# Asset A11: front elevation (radiator, split AC, tilt window)

S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, T = this ticket folder. Scripts: `T/🧰️a11/`.

## Design decision: a third viewpoint `front`

`elevation` stays the rig plane (side view of a figure, vertical section of a component), because bones, joints, poses and clips live in that plane. A second elevation seen from the front cannot share the rig kinematics, so it is a further viewpoint, exactly like `plan`:

- `Viewpoint = elevation | front | plan`. `geometry.front` (optional) holds shapes in front coordinates: x across the width from the centre, y up from the floor, metres.
- A front shape follows its bone only by the bone's y travel from the rest pose (plan: x travel). New optional `followTip` on front `rect` and `poly` stretches the shape vertically between the bone origin travel (at the rest height of the origin) and the tip travel (at the rest height of the tip), linear in between. It lets a tilting sash shrink and open a gap against the frame head. (Plan keeps its `followTip` on capsules.)
- No compatibility layer: the 125 other assets are untouched; `front` is simply absent for them.

Changed, schema first:

| Area | File | Change |
|---|---|---|
| JSON schema | `S/🧬️schema/🔣️.json` | `Viewpoint` enum, `geometry.front` (representations and symbols), `followTip` on rect and poly |
| TS twin | `S/🧬️schema/🟦️.ts` | `VIEWPOINTS`, `geometry.front`, `RectShape/PolyShape.followTip`, docs |
| Evaluation | `S/🔨️modules/🧮️evaluation/🟦️.ts` | `shapes.front`, `frontStretch`, header |
| Depiction | `S/🔨️modules/🎨️depiction/🟦️.ts` | `groundSpan` (ground line of a front view spans the drawn shapes) for SVG and CAD scene |
| Print suitability | `S/🔨️modules/🖨️representation/🟦️.ts` | stroke, gap and feature measured over the front (and plan) drawings too |
| Copy detection | `S/🔨️modules/🖨️representation/🪞️copies/🟦️.ts` | compares front geometry too |
| Catalog check | `S/🔨️modules/🗂️catalog/🟦️.ts` | checks front shapes, rule `viewpoint-unlisted` (front or plan geometry needs the viewpoint in `presentation.viewpoints`), rule `follow-tip-viewpoint` (replaces `follow-tip-plan-only`) |
| Cast exporter | `S/🔨️modules/📤️export/🎭️cast/🟦️.ts`, cast `🧬️schema/🔣️.schema.json` | `viewpoint: front`; a cast whose viewpoint has no geometry now fails ("no front geometry"); `heightM` of front and plan casts is the drawn height without the margin |
| CLI | `S/🔨️modules/📤️export/🎬️cli/🟦️.ts` | usage text |
| React | `🎯️targets/⚛️react/.../🌐️i18n` | `detail.viewpoint.front` (en, de) |

## Assets (all handcrafted, `T/🧰️a11/radiator_front.py`, `ac_front.py`, `window_front.py`)

- `panel-radiator` front at 1:200, 1:100, 1:50: 1.0 m x 0.6 m type 22, 0.11 m above the floor. Front plate in three bands, a slotted grille strip at the top edge with the convector fin tips (20 / 10 slits), outline, end seams (1:50), thermostatic head with index mark on the left and lockshield cap on the right, each with pipe and valve body. Thermal model: the front panel region is split into `panel-front-upper/-mid/-lower` (heat 38/33/29 % of 444 W, area 0.2 m2 each), giving 86 / 79 / 73 degC at rated, 59 / 55 / 51 degC at night (13 K and 8 K spread); `valve` became an operating region (3 W, about 51 degC) so the valve reads warm. Poses, `radiator-body` merged regions and the elevation drawing (1:50 splits the front strip into the three bands) were updated, the `front-plume` emitter binds `panel-front-mid`, an assumption text documents it.
- `split-ac-indoor-unit` front (0.85 x 0.29 m): casing, slatted intake strip, panel split, discharge vane, display window; manifest viewpoints `elevation, front, plan`.
- `tilt-window` front (0.9 x 1.25 m, coordinator extra scope): sill, two jambs and head fixed; two stiles, two rails, pane and spacer line carry `followTip`, the handle is rigid. The tilt reads through the gap above the sash head (0.04 m at 15 degrees, 0.16 m at 30 degrees).
- Not changed, checked: `server-rack` and `chalkboard` are already authored as front elevations (their `elevation` is the front), and `kitchen-extractor-hood` stands against the right wall in a section (side view is correct).
- Manifest rows (`S/🗂️catalog/🔣️.json`): `viewpoints` of `panel-radiator`, `split-ac-indoor-unit`, `tilt-window`.

## Casts and clip sites (`V/🧍️staffage/🔣️.json`)

Front (radiator on a free or rear position): `heating-fundamentals-panel-radiator` (room section, radiator mid room at x = -4.6), `optimization-extrema-radiator` (icon), `demand-primary-energy-radiator` (mid house), `demand-certificate-radiator` (mid house), new `heating-calculation-radiator-front` (house section, `🧮️calculation` site `UltimateEnergyBalance`). New `optimization-extrema-cooler` (front icon, replaces `_room._cooler` in `📉️extrema` `_icons`). `optimization-experiments-window` now `viewpoint: front` (anchor unchanged).

Kept as side view (radiator or AC against a cut wall in section): `optimization-room-radiator` and `optimization-room-cooler` (right wall, mirrored), `ventilation-natural-radiator-*` (under the window in the 0.35 m exterior wall), `demand-regulations`, `demand-power`, `demand-practice`, `demand-recap`, `demand-system-losses` (under windows at the exterior wall), `heating-conduction-radiator`, the mirrored pair `heating-calculation-radiator(-mirrored)` at both walls, `physics-fundamentals-radiator` (pipe feeds from the side) and the `shared`/`radiator` fitted casts.

Anchor change: a front cast is centred on its x anchor (side casts were anchored at the wall face); the sites were left at their old x.

## Tests

New `S/🧪️tests/🧍️front-projection/{🟦️.ts,🥒️.feature}` and `S/🧫️fixtures/🧍️front-projection/🔣️.json` (9 tests, all pass): y travel of a bone against gl-matrix (relative to the thigh, hand value 0.032767 m), radiator front extent 1.0 x 0.6 m from 0.11 m, thermal band order and at least 8 K spread, front and side cast sizes, front geometry for every authored scale of the radiator and the AC, refusal of a front cast without front geometry.

## Placement and size fixes from the render audits (R1, R2)

- `fundamentals`: radiator already front (mid room, true scale 0.6 m of the 2.5 m room, 0.11 m above the floor), no wall stub any more.
- `conduction` Beat5: `heating-conduction-radiator` now front, at true house scale on the slab (the house section is small, so it stays small but reads as a radiator).
- `calculation` `_plant_house` (AnlagenVerluste): radiators at true scale (were 0.65 of it), standing on the floor line against both exterior walls, `RAD_Y` follows. UltimateEnergyBalance uses the front cast `heating-calculation-radiator-front`.
- `natural` Beat2 (final, narration and picture agree): `ventilation-natural-radiator-on/off` are side-view casts under the window on the cut exterior wall, true scale (`unit = k`), body 0.15 m above the floor (anchor 0.04 m plus the 0.11 m clearance), cast `scale` 20 so the pens are thin against the 0.1 m panel; the 1:50 section outlines of the water plates use the 0.1 mm pen (`panel-radiator`, decision text updated) so their heat colour shows. The valve Circle is removed: the dial mark turns on an invisible point at the valve head of the asset. In the frame the radiator sits under the window with warm plates, convector zigzag and valve head, but at 480p it is only about 5 px wide (the true-scale section is 0.1 m deep), so it reads as a small warm bar; a front view would contradict the cut-wall position. `ventilation ... closeup radiator` test now asserts the true cast height (3 %), floor clearance, position under the sill and against the exterior wall face.
- Tests adapted to the new physical sizes: `V/🧪️tests/🐍️integration.py` (per-chapter cast), `🔣️scenes.json` (`radiator_height_m` 1.12 = cast frame height at true scale) and `🐍️scenes.py` (bottom on the floor line), ventilation `🧪️architecture/🔣️.cases.json` (0.71 m, clearance 0.04 m).

## Verification

- Catalog check, full manifest and all categories: `✓` for 132 entries, exit 0 (22 min). `panel-radiator`, `split-ac-indoor-unit`, `tilt-window`: office-laser suitable at 1:50, 1:100, 1:200.
- Staffage vitest: 50 of 51 files pass, 951 tests passed, 10 skipped; the only failing file is `📏️print-size` (missing `d3-format`, unrelated, see `q1-tests-report.md`). New suite `front-projection`: 9 of 9.
- Visuals Python tests (`V/🧪️tests`, 8 modules, 60 tests): the three failures of the first run (`integration.test_demand_emitters_and_occupants`, certificate) and two later ones (`scenes` heating plant, ventilation `closeup radiator`) came from sizes that changed on purpose and were fixed in the tests (see above); the affected tests pass now. The remaining modules passed in the first run.
- Casts exported and looked at: radiator front (thermal rated, night, off, line, solid, 1:100, 1:200) with the three-band gradient, AC front (thermal, line, 1:100, 1:200), window front (closed, tilted-open, fully-tilted, 1:100, 1:200, thermal).
- Scenes rendered at quality l, 15 fps and looked at: heating `fundamentals` Beat3_Konvektion (warm banded radiator on the rear wall), `calculation` UltimateEnergyBalance (front radiator in the house), `calculation` AnlagenVerluste, `conduction` Beat5_Gebaeudehuelle, ventilation `natural` Beat2_StossOderKipp (radiators clear and warm), and a still of the new icons (window, radiator, cooler) from `📉️extrema` and `🧪️experiments`.
- Final run after the last edits: full visuals set (8 modules in `V/🧪️tests` plus the ventilation `🧪️architecture` module) 65 tests OK; `integration` module alone 9 tests OK; scenes plus ventilation architecture 16 OK after removing the valve circle. Catalog check of `panel-radiator` after the pen change: `✓` at 1:200, 1:100, 1:50 (full manifest check was run before this pen change).
- Not run: React suite and `tsc` (same unrelated dependency problem as in `q1-tests-report.md`); the cast-export fixture was not extended with a front cast (the `front-projection` suite covers it).
- Generated output (`🗑️generated/a11`) deleted.

## Open points

- Gradient colours saturate in the shared thermal ramp at rated output (86 / 79 / 73 degC all read red); the night and comfort poses show the bands clearly.
- A front cast is centred on its x anchor; side casts anchor at the wall face. Other side-view sites were left unchanged.
