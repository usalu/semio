# 📝️ sonnet-buildings: components (15) and appliances A (12)

Scripts: `T/🛠️scripts/buildings/` (`audit.ts` guard sweep, `deep.ts` all poses and 11 clip samples per scale, `val.ts` live `validateAsset`, `suit.ts`/`suit2.ts` print suitability, `ext.ts` per-shape extents, `fix-*.ts` the catalog edits, `render.sh`, `stack*.py`/`grid.py` contact sheets).
Proof renders (dark line, side + front + top at the same pose and scale): `T/🗑️generated/buildings/<asset>/` at 1:200, 1:100, 1:50 for all 27 assets (default pose), `symbols/` (7 symbols at 1:50), `b500/` (the two new 1:500 bands), `tw/` (tilt-window fully-tilted 1:50), `evb/` (blind cut-off 45 at 1:100).

## Audit result at the start

| Check | Result |
|---|---|
| Every authored band has side, front and plan shapes | yes, all 27 assets, all bands |
| Guard `sweep` and `consistency` (default pose extents vs `dimensions`) | clean |
| `deep.ts`: side/front/plan extent agreement on every pose and 11 clip samples, every scale | clean except tilt-window (side view lost the 1.25 m frame height when the sash tilted) |
| `unavailable` bands | 16, none with `cause` |
| Symbols (7) | each authored in one view only, in a frame that disagreed with the asset (see below) |
| Print suitability | external-venetian-blind 1:200 and 1:100 unsuitable (front/plan lamella strips under 0.2 mm), circulation-pump 1:50 unsuitable (gland 0.13 mm) |
| steel-column | front/plan depth 0.26 m vs `depthM` 0.2 |

## Changes (all catalog edits are JSON round-trips through `lib.ts`; `1.0` literals became `1`)

1. `cause: "below-printable-feature"` on all 16 unavailable bands.
2. Reasons corrected for gas-boiler, panel-radiator, circulation-pump (features of 0.25 mm, 0.2 mm and 0.12 mm were wrongly called "below every print profile"), electricity-meter, gas-meter, decentralised-hrv-fan, duct-fan, ceiling-diffuser (referenced symbols that do not exist). New text states the real overall paper size (well under 3 mm), the truly smallest drawn part and the 0.2 mm office-laser minimum; `limitingFeature` and `limitingSizeM` follow.
3. Symbols (all `true-extent`, model metres, real size, now with side, front and plan; static shapes bound to the rig root; every symbol note says what each view shows):

| Asset | Symbol | Defect found | Fix |
|---|---|---|---|
| glass-balustrade | balustrade-plan | plan only, panel length along x although the asset runs it laterally (z) | all three views from the true 1:200 geometry, panel along z |
| tilt-window | window-plan | plan only, rotated 90 degrees and 1.2 m wide (wall stubs) vs 0.9 m | wall cut at the reveals, three frame lines across the opening, side section, front outline; room side is -x |
| tilt-window | tilt-opening-sign | tilt sign was authored in `elevation`, but a facade view is the `front` | tilt lines now in `front`; side shows an arrow to the room (-x); plan shows reveals and glass line |
| sliding-door | sliding-door-plan | plan only, 0.20 m deep vs `depthM` 0.1 | plan 0.1 deep, slide arrow in side and plan, front edge-on with leaf behind the jamb |
| steel-column | section-plan | plan only, section 0.2 x 0.2 vs plate 0.3 x 0.26 | plan section plus base-plate outline, side and front with plates |
| spiral-stair | spiral-stair-plan | plan only | side and front: column, base plate, 15 treads, helical handrail (bone-local shapes converted to world metres) |
| stair-flight | flight-plan | plan only, 0..1.2 lateral (not centred), 3.6 m vs 3.885 m | centred plan with stringer outline to 3.855 m, side and front from the 1:200 geometry |

4. steel-column: `depthM` 0.2 to 0.26. Physical evidence: the 300 x 260 mm base plate is drawn in every band and is the real lateral extent. The 1:50 front splice mark (0.28 m) was shortened to 0.26 m.
5. tilt-window: elevation gets a `jamb-profile` (the 0.3 m deep, 1.2 m high frame section) at 1:50, 1:100 and 1:200, so the side view keeps the 1.25 m frame while the sash tilts (it was 1.112 m).
6. external-venetian-blind: 1:200 and 1:100 front and plan lamellae are body lines instead of 0.013 to 0.02 m filled strips (suitable now).
7. circulation-pump: 1:50 gland widened to 16.5 mm (was 6.5 mm, 0.13 mm on paper).
8. sliding-door 1:200 front gains the hidden leaf outline `leaf-behind`.
9. New 1:500 bands (rule D17, at least 3 mm overall): external-venetian-blind (5.5 mm; every fourth lamella as a line, each bound to its own bone so lift and tilt still move it; 1:500 pen 0.18) and hot-water-cylinder (3.0 mm; foot bar, two jacket halves, dome and three enlarged stubs, widths kept by overlapping the jacket). Both pass print suitability and carry the eight decision texts in en and de.

## Per-asset status (validator, suitability, deep sweep; all after the edits)

| Asset | 1:500 | 1:200 | 1:100 | 1:50 | Symbols | validateAsset | Notes |
|---|---|---|---|---|---|---|---|
| cantilever-balcony | authored | ok | ok | ok | none | 0 | |
| curtain-wall-bay | authored | ok | ok | ok | none | 0 | front is the section A-A at x 0.7, plan the section B-B at y 2.075 (spandrel and lintel-type parts below or above the cut are correctly absent) |
| external-venetian-blind | authored (new) | ok | ok | ok | none | 0 | |
| glass-balustrade | unavailable | ok | ok | ok | 1 | 0 | |
| glulam-beam | authored | ok | ok | ok | none | 0 | |
| masonry-wall-with-lintel | authored | ok | ok | ok | none | 0 | plan section at 1.5 m cuts below the lintel |
| panel-radiator | unavailable | ok | ok | ok | none | 0 | `widthM` 0.135 m checked: 0.035 m wall clearance plus 0.10 m panel, the side-view thickness; correct |
| sectional-garage-door | authored | ok | ok | ok | none | 0 | |
| sliding-door | authored | ok | ok | ok | 1 | 0 | |
| spiral-stair | authored | ok | ok | ok | 1 | 0 | |
| stair-flight | authored | ok | ok | ok | 1 | 0 | |
| steel-column | authored | ok | ok | ok | 1 | 0 | `depthM` 0.26 |
| tied-canopy | authored | ok | ok | ok | none | 0 | |
| tilt-window | unavailable | ok | ok | ok | 2 | 0 | |
| wheelchair-ramp | authored | ok | ok | ok | none | 0 | |
| ceiling-diffuser, circulation-pump, decentralised-hrv-fan, desktop-monitor, duct-fan, electric-fan-heater, electric-kettle, electricity-meter, gas-boiler, gas-hob, gas-meter | unavailable (all under 3 mm) | ok | ok | ok | none | 0 | cause and reasons corrected |
| hot-water-cylinder | authored (new) | ok | ok | ok | none | 0 | |

Handedness: all front shapes use paper x = z with left (+z) on the right of the sheet, plan paper up = +z; asymmetric items checked (tilt-window `jamb-left` at +0.4, hob knob at -z in plan, front and side).

## Operating clips and supports

- `deep.ts` evaluates every pose and 11 samples of every clip (door slide, window tilt, blind lift and tilt, garage travel, stair climb, ramp ascent, rotors) at all four bands and requires the side, front and plan extents to agree on the shared axes; clean everywhere after the tilt-window jamb.
- Rotors, flames and needles that exist only in the side view are edge-on and hidden inside their casing in front and plan (no visible motion); documented, not drawn.
- Supports and loads: anchors have `z` undefined (0); every authored component is laterally centred. The in-plane loads (balustrade wind/line/impact along x, blind gust along x, curtain-wall seismic along x) act along the normal or plane of the element correctly. The curtain-wall bay has no out-of-plane wind case (normal is z); proposed as a new pose `wind-suction` with `fzN` for sonnet-structural, not yet authored.

## Open

- panel-radiator `convection-outlet` has `directionDeg` 90 on a bone at 90 degrees. The thermal source code takes the direction in the bone frame (`cos a u + sin a v`), so it points backward. Correct value 0 (along the bone, upward). The `front-plume` is kind plume and rises regardless. Edit queued until the D24 catalog pause is lifted.
- Other emitters checked: all bone-0 assets (diffuser jets, fan vents, boiler flue, kettle steam) are correct in the bone frame.
- `repo` three-views vitest (ticket-local config) shows 22 failures, none in these 27 assets (bamboo-clump, cafe-chair, concrete-mixer-truck, domestic-cat, english-oak, excavator, goat, labrador-dog, mallard-duck, miscanthus-grass, person-with-white-cane, red-squirrel, seated-office-worker, seated-reader, weeping-willow).
- The manifest status changes for registry: external-venetian-blind and hot-water-cylinder gain an authored 1:500 band; all 16 unavailable bands now carry a cause; steel-column `depthM` is 0.26.
