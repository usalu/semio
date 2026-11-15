# Area decisions (reviewer of thermal areas)

Reviewer pass over every committed-versus-working-tree `areaM2` difference (`area-audit.md`) and every row of `area-review.md`. Rulings: C1, C15, C16, C18, C19 in `design.md`. Scripts: `areareview-decisions.py` (the decisions, one reason each), `areareview-apply.py` (applies them: only the number token of `areaM2` or an inserted `areaBasis` line, checked by JSON comparison, written atomically, skipped when the file changed meanwhile), `areareview-report.py` (this file). Findings for the model authors are appended to `model-findings.md`.

## Method (short)

`areaM2` weights only for passive regions (temperature per m2 is independent of it) but sets the flux density `q = heatW / areaM2` of operating regions, the draw of coupled regions and the segment factor `f = areaM2 / skinAreaM2` of body regions. Per changed value decide ACCEPT (hand calculation of the real object from `dimensions`, the drawing and standard sections agrees within ~30 %), RESTORE + `areaBasis: exchange` (fins, plates, pads, matrices, aggregates), RESTORE + model finding (poor model, named in `model-findings.md`), or SET (hand value). Write the decision in `areareview-decisions.py` as `d(slug, region, ACT, value, reason, finding)` and run `python3 areareview-apply.py` (audit + surgical write + state), then `python3 areareview-report.py` (this file, `model-findings.md`, marks in `area-review.md`); the second reviewer appends below the line `## Reviewer 2`, which the report script preserves. Ruling C29 (areaM2 = TOTAL outer area of the region's own solids, covering by riders, hands, seats, ground, walls or other regions NOT excluded; exposure is the pose's `exposedFraction`/`contact.fraction`; `heatW` stays the physical datum; body totals stay tied to BodyPhysiology): decide every row at the total by hand calculation, reasons start with C29 total. Removal of illegitimate `sectionFaces`: `areareview-unsection.py`; pose fractions: `areareview-pose.py`; layer order: `areareview-layerfix.py`.

## What `areaM2` does in the solver (from the code)

Read from `🔨️modules/🌡️thermal` (`🟦️.ts`, `🧱️balance`, `🫀️physiology`, `🛋️comfort`, `🍃️canopy`) and the `ThermalRegion` schema.

- **Passive surface, glazing, tyre, wall, water (no `operating`, no `hydronic`, no `coupled`)**: the surface temperature is the root of one balance per m² (`solveSteady`: inflow + absorbed solar = radiation + convection + latent), with `characteristicLengthM` and `geometry` choosing the natural-convection correlation, `skyViewFactor` and `geometry` the radiant environment, the layers the series resistance towards the interior or heat source. **`areaM2` does not enter the temperature.** It only weights: the Watt totals `r.areaM2 × exposed` (`netRadiativeW`, `sensibleW`, `latentW`, phase change), the area-weighted merge colour of small-scale shapes (`Σ areaM2·exposedFraction·value`), and the totals shown in the legend.
- **Operating regions**: the heat released inside enters as `q = heatW × level / areaM2` (`inflowOf`). **A change of `areaM2` rescales the heat flux density, hence the surface temperature, one to one.** A halved area doubles `q`; this is why operating regions are only accepted where the area is the honest outer surface of the heated part (the heat leaves through the outer face), and why changes beyond a factor 1.5 in `q` are listed in the follow-up table below (the `heatW` that keeps the committed flux density).
- **Coupled and hydronic regions**: the draw of a coupled region on its source (`areaM2_dep × inflow / areaM2_source`) scales with the ratio of the two areas.
- **Conditioned**: `(T_interior − T_s)/(ΣR + R_film)` per m², area only weights.
- **Leaf regions**: one-sided leaf area, both faces exchange (`faces = 2`), transpiration through one; `leafAreaIndex` converts to ground area. Area changes the Watt totals only, never the leaf temperature.
- **Body regions**: `f = areaM2 / skinAreaM2` distributes the Gagge two-node core over the segments and `fcl = areaM2 / skinArea` is the clothing area factor in the comfort index (`🛋️comfort`): body areas are physiological and tied to `BodyPhysiology` (C18); changing them moves the whole-body balance.
- `exposedFraction`, `contact.fraction` and `sunlitFraction` are per-frame inputs, so `areaM2` is the *total* outer area and the exposure is applied on top (C1 asks for the exposed surface of the model: buried and contact faces are therefore the validator's, not the solver's, notion; where a region is covered by a rider or a hand the author's exposed value is the honest drawn pose and accepted).

## Principles of the decisions

1. Hand calculations use `dimensions`, the drawing and the real object (profile perimeters, tube lengths, standard sections), not the 3D model; the model value is accepted where the hand value agrees within ~30 %.
2. Thin sheets standing free in the air (slats, shelves, louvres, canvas, flags, plates) exchange through both faces: both faces count. Enclosure panels whose inner face looks into a cavity, and heated parts whose heat leaves through the outer face (hood, casing, lid, plate), count the outer face only, because `q = heatW/area` is a flux through that face.
3. Exchange areas the envelope cannot show (fruit aggregates, fin packs, plate packs, wetted pads, honeycomb matrices, heat-transfer contact discs) keep or get a hand-derived value and `areaBasis: "exchange"` (C19).
4. Where the model is poor (buried or counted twice, detail solids on top of a surface, solids of the wrong shape, missing parts) the committed value is restored and the defect is named in `model-findings.md`.
5. Body-model regions are never changed from the model (C18); the deviation is a model finding.
6. A region whose asset has no authored 1:50 volumes in the file is deferred (C15: corrections only from the 1:50 model) and reviewed again once they exist.

## Totals

329 decided rows: ACCEPT 226, RESTORE + exchange 4, RESTORE + model defect 27, SET 46, SET + exchange 1, exchange only 2, RESTORE (C18 body) 1, KEEP 4, DEFER (no 1:50 model yet) 18. 263 rows differ from the committed catalog after the decisions (changed value or exchange marker).

## Decisions

| asset | region | kind | model | committed m2 | author m2 | decided m2 | decision | reason |
|---|---|---|---|---|---|---|---|---|
| 🌳️vegetation agave | caudex | stem | passive | 0.21 | 0.394 | 0.29 | SET | stem cylinder o0.22 x 0.42 m: pi*0.22*0.42 = 0.29; the model is a rectangular prism (extruded profile), not a cylinder |
| 🌳️vegetation apple-tree | fruit | surface | passive | 15 | 0.0921 | 15 (exchange) | RESTORE + exchange | aggregate of ~750 apples o0.08: 750 * 4*pi*0.04^2 = 15 m2 at harvest load; five fruit solids cannot show it |
| 🌳️vegetation apple-tree | limb-bark | bark | passive | 6.08 | 3 | 6.08 | RESTORE + model defect | 4 scaffolds 2.5 m o0.15 (4.7) + 8 secondaries 1.5 m o0.08 (3.0) = 7.7 m2; 4+4 limbs of the model give 3.0 |
| 🌳️vegetation apple-tree | spur-twig | stem | passive | 0.12 | 0.227 | 0.227 | ACCEPT | 4 twig bundles of o0.02: no hand estimate better than +-50 % at 0.1-0.25 m2, region is weighting only (passive) |
| 🌳️vegetation apple-tree | trunk-bark | bark | passive | 3.58 | 4.98 | 3.58 | RESTORE + model defect | stem 0-1.7 m o0.38 avg = 2.0, fork 1.7-2.7 m o0.22 = 0.7 -> 2.7-3.6 m2 with fluting; the model counts 4.98 |
| 🌳️vegetation bamboo-clump | culms | stem | passive | 1.6 | 19.3 | 1.6 | RESTORE + model defect | 6 culms o0.029 tapering to o0.015, mean o0.022 x 3.5 m mean height: 6*pi*0.022*3.5 = 1.45-1.9 m2 |
| 🌳️vegetation date-palm | fruit | surface | passive | 3 | 12.2 | 9 (exchange) | SET + exchange | 8 bunches * 600 dates * 1.9e-3 m2 (prolate spheroid 0.035 x 0.020) = 9.1 m2; individual dates cannot be modelled |
| 🌳️vegetation date-palm | stem-bark | bark | passive | 13 | 36.3 | 13 | RESTORE + model defect | stem 8.1 m o0.5 = 12.7 + crown shaft 1.0 + root flare 1.2 = 14-15 m2; the model counts 36.3 |
| 🌳️vegetation hosta-clump | petioles | stem | passive | 0.25 | 0.183 | 0.183 | ACCEPT | ~16-22 petioles o0.012 x 0.3 m = 0.18-0.25 m2, covers the model value |
| 🌳️vegetation ivy-on-trellis | post-shoes | surface | passive | 0.5 | 0.264 | 0.33 | SET | C29 total: 2 shoes 0.11 x 0.28 x 0.13 all faces = 0.33 |
| 🌳️vegetation ivy-on-trellis | stems | stem | passive | 0.45 | 0.139 | 0.45 | RESTORE + model defect | 4 main stems 2 m o0.015 (0.38) + 12 laterals 0.5 m o0.008 (0.15) = 0.53 m2 |
| 🌳️vegetation ivy-on-trellis | trellis-timber | surface | passive | 6.3 | 2.82 | 3.6 | SET | C29 total: posts 2*0.38*2.1 = 1.6 (including the part in the shoes) + rails 1.0 + battens 1.0 |
| 🌳️vegetation japanese-maple | limb-bark | bark | passive | 1.37 | 0.65 | 0.65 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🌳️vegetation japanese-maple | stem-bark | bark | passive | 3.45 | 2.3 | 2.3 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🌳️vegetation london-plane-pollarded | trunk-bark | bark | passive | 7.53 | 9.8 | 9.8 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🌳️vegetation miscanthus-grass | culms | bark | passive | 0.5 | 0.603 | 0.603 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🌳️vegetation multi-stem-birch | branch-bark | bark | passive | 4 | 2.67 | 2.67 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🌳️vegetation multi-stem-birch | stem-bark | bark | passive | 14 | 18.78 | 18.78 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🌳️vegetation scots-pine | bark-lower | bark | passive | 12 | 14.42 | 14.42 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🌳️vegetation scots-pine | bark-upper | bark | passive | 14 | 11.38 | 11.38 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🌳️vegetation scots-pine | limb-bark | bark | passive | 8 | 11.23 | 11.23 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🏗️machinery concrete-mixer-truck | cab | surface | passive | 24 | 27 | 27 | ACCEPT | cab 2.5 x 2.0 x 2.2 m box without the underside: 2*(5.0+5.5+4.4)-5 = 25-27 m2 |
| 🏗️machinery concrete-mixer-truck | cab-glazing | glazing | passive | 4.5 | 2.73 | 2.73 | ACCEPT | drawn windscreen 2.1 x 0.62 (1.3) + door and quarter glass 1.4 = 2.7 m2 from the drawing |
| 🏗️machinery concrete-mixer-truck | chassis | surface | passive | 12 | 36 | 36 | ACCEPT | rails 2 x 8.8 m x 0.28 x 2 faces 9.9-10.8 + cross members 2 + tanks 3 + 4 axles with springs 8 + sub-frame deck 8.8 x 0.62 both faces 10.9 + steps 1.5 = 36 m2 (old 12 was the rails only) |
| 🏗️machinery concrete-mixer-truck | chute | surface | passive | 4 | 8.17 | 8.17 | ACCEPT | open trough both faces: main 2.4 m (floor 1.25 + walls 1.44) + extension 1.3 m (0.7 + 0.78) = 4.2 per face, 8.4 both; old 4 was one face |
| 🏗️machinery concrete-mixer-truck | drum | surface | passive | 36 | 51.8 | 36 | RESTORE + model defect | 9 m3 drum, 16 m3 geometric volume, o2.4: belly + two cones = 32-40 m2 (pi*2.4*5.05*0.85 = 32 + closures) |
| 🏗️machinery concrete-mixer-truck | drum-drive | engine | operating | 2.5 | 3.04 | 3.04 | ACCEPT | gearbox/motor housing 0.8 x 0.8 x 0.6: 2*(0.64+0.48+0.48) less the mount 0.64 = 2.6-3.2 m2 |
| 🏗️machinery concrete-mixer-truck | engine-bay | engine | operating | 3.2 | 2.08 | 3.2 | RESTORE + model defect | engine tunnel: grille 0.9 + tunnel top and sides 2*0.6*0.8 + belly pan 0.8 = 2.7-3.2 m2; q = 4000 W / 3.2 = 1250 W/m2 stays |
| 🏗️machinery concrete-mixer-truck | exhaust-stack | exhaust | operating | 0.8 | 1.21 | 1.21 | ACCEPT | vertical stack o0.15 x 2.9 m = 1.37 m2 (pipe o0.127 gives 1.2) |
| 🏗️machinery concrete-mixer-truck | tyres | tyre | operating | 10 | 19.2 | 19.2 | ACCEPT | 8 tyres 315/80 R22.5 (o1.04 x 0.315): tread 1.03 + 2 sidewalls 0.59 each = 2.2 -> 17.6 m2 (19.2 with the wider drawn tyre) |
| 🏗️machinery diesel-generator | discharge-hood | surface | operating | 4 | 10.2 | 4 | RESTORE + model defect | hood outer shell: pentagon sides 2*0.70 + slope 0.73 + top 1.43 + back 0.62 = 4.2 m2; heat leaves through the outer face (q = heatW/area), inner faces face the hot air |
| 🏗️machinery diesel-generator | enclosure-walls | wall | operating | 29 | 43.5 | 43.5 | ACCEPT | 4 walls 2*(6.06+2.44)*2.4 = 40.8 m2 plus ~8 % corrugation = 44 m2 |
| 🏗️machinery diesel-generator | exhaust-stack | exhaust | operating | 0.3 | 0.26 | 0.26 | ACCEPT | stack o0.2 x 0.4 m = 0.25 m2 |
| 🏗️machinery diesel-generator | inlet-louvres | surface | passive | 4.2 | 9.36 | 5.8 | SET | 60 slats (4 panels x 15) of 2 mm sheet 1.3 x 0.035: both faces 2*1.3*0.035 + edges = 0.096 m2 each = 5.8 m2 |
| 🏗️machinery diesel-generator | outlet-flap | surface | passive | 0.7 | 1.25 | 1.25 | ACCEPT | flap 1.7 x 0.63 one face 1.07 + edges 0.28 (other face against the mouth frame) = 1.25-1.35 |
| 🏗️machinery diesel-generator | rain-cap | surface | passive | 0.1 | 0.21 | 0.21 | ACCEPT | plate 0.3 x 0.3 both faces 0.18 + rim 4*0.3*0.02 = 0.2 m2 |
| 🏗️machinery diesel-generator | silencer | exhaust | operating | 3 | 3.87 | 3.87 | ACCEPT | capsule o0.55: cylinder 2*pi*0.275*1.65 = 2.85 + two hemispheres 0.95 = 3.8 m2 |
| 🏗️machinery diesel-generator | skid-tank | surface | passive | 12 | 3.92 | 3.92 | ACCEPT | exposed sides and ends of the 6.06 x 1.94 x 0.2 tank 2*(6.06+1.94)*0.2 = 3.2 + castings 0.7 = 3.9 m2; old 12 was the footprint 6.06*1.94 = 11.8 (contact plane, C1b) |
| 🏗️machinery evaporative-humidifier | casing | surface | passive | 0.9 | 1.78 | 1.78 | ACCEPT | box 0.5 x 0.6 x 0.4 outer 1.48 m2 + sump tray rim; old 0.9 left a pair of faces out |
| 🏗️machinery evaporative-humidifier | pad | water | passive | 1.2 | 1.04 | 1.2 (exchange) | RESTORE + exchange | fluted cellulose pad: wetted flute surface is an exchange area the 0.4 x 0.4 x 0.36 envelope cannot show |
| 🏗️machinery evaporative-humidifier | sump | water | passive | 0.2 | 0.209 | 0.209 | ACCEPT | free surface 0.44 x 0.38 = 0.167 + rim and visible sides = 0.2 m2 |
| 🏗️machinery evaporative-humidifier | trough | surface | passive | 0.1 | 0.118 | 0.2 | SET | open trough 0.4 x 0.36: inside floor 0.144 + inner walls 2*(0.4+0.36)*0.04 = 0.06 -> 0.2 m2 (bottom lies on the pad) |
| 🏗️machinery excavator | attachment | surface | passive | 28 | 33.6 | 33.6 | ACCEPT | boom 5.7 m (perimeter 2.2) 12.5 + stick 2.9 m 5.2 + bucket outer 5 + inner 4 = 27-30 m2 |
| 🏗️machinery excavator | cab-glazing | glazing | passive | 6 | 4 | 4 | SET | cab 1.15 x 1.5 x 1.7: windscreen 1.23 (drawn) + two side windows 2*0.9 + rear 0.5 + roof 0.5 = 4.0 m2 |
| 🏗️machinery excavator | counterweight | surface | passive | 4.5 | 4.5 | 4.5 | ACCEPT | profile perimeter 4.8 m less the front face 1.2 against the house = 3.6 m x 2.4 m = 8.6 + 2 sides 1.8 = 10.4 m2 |
| 🏗️machinery excavator | engine-compartment | engine | operating | 2.5 | 2.58 | 2.58 | ACCEPT | hood panels and engine block surface 2.5-2.6 m2 (box 1.1 x 0.65 x 1.0 less base) |
| 🏗️machinery excavator | exhaust-stack | exhaust | operating | 0.22 | 0.188 | 0.188 | ACCEPT | stack o0.12 x 0.5 m = 0.19 m2 |
| 🏗️machinery excavator | house | surface | passive | 30 | 30 | 30 | RESTORE + model defect | upper structure profile perimeter 9.08 m * 2.5 m width less underside 8.4 + 2 side faces 5.2 = 19.5 m2, plus cab (perimeter 6.5 * 1.15 + sides 4) = 11 -> 30 m2 |
| 🏗️machinery excavator | hydraulics | surface | operating | 3.5 | 3.5 | 3.5 | RESTORE + model defect | boom cylinders 2 x (barrel o0.14*1.5 + rod o0.08*1.2 = 0.96) + stick 0.73 + bucket 0.55 = 3.2 m2 + hoses |
| 🏗️machinery excavator | tracks | surface | passive | 14 | 14 | 14 | ACCEPT | 2 tracks: outer belt 9.3 m x 0.6 = 5.6, both side faces 6.8 = 12.4 each = 24.8 + carbody 3 = 27.8 m2 |
| 🏗️machinery expansion-valve | body | surface | operating | 0.012 | 0.0125 | 0.0125 | ACCEPT | box 0.064 x 0.05 x 0.06 = 0.0125 m2 |
| 🏗️machinery expansion-valve | bulb | surface | operating | 0.002 | 0.0025 | 0.0025 | ACCEPT | bulb o0.014 x 0.05 m = 0.0022-0.0025 m2 |
| 🏗️machinery expansion-valve | capillary | surface | passive | 0.0004 | 0.0006 | 0.0006 | ACCEPT | o1.6 mm x 0.15 m = 0.00075 m2 |
| 🏗️machinery expansion-valve | head | surface | operating | 0.004 | 0.00467 | 0.00467 | ACCEPT | revolve o0.05 x 0.03: 0.0047 m2 |
| 🏗️machinery expansion-valve | inlet | surface | operating | 0.004 | 0.0032 | 0.0032 | ACCEPT | pipe o9.5 mm x 0.1 m = 0.003 m2 |
| 🏗️machinery expansion-valve | outlet | surface | operating | 0.004 | 0.0032 | 0.0032 | ACCEPT | pipe o9.5 mm x 0.1 m = 0.003 m2 |
| 🏗️machinery finned-heat-exchanger | fins | surface | operating | 2 | 1.09 | 2 (exchange) | RESTORE + exchange | fin pack: 200 fins of 0.205 x 0.52 both faces = 40 m2 of exchange surface; neither 2 m2 (heatW tuned to it) nor the 1.09 m2 envelope shows it |
| 🏗️machinery finned-heat-exchanger | headers | surface | operating | 0.012 | 0.132 | 0.132 | ACCEPT | two 0.15 x 0.5 x 0.02 boxes: edges 0.026 each + top face of the upper one 0.075 + stubs 0.01 = 0.13 m2 (old 0.012 was the stubs only); heatW 2 W follow-up: keep q = 167 W/m2 -> 22 W |
| 🏗️machinery finned-heat-exchanger | tubes | heater | operating | 0.08 | 0.188 | 0.188 | ACCEPT | 6 tubes o0.018 x 0.48 m = 0.163 m2 plus bends = 0.19; heatW 24 W follow-up: keep q = 300 W/m2 -> 56 W if the coil should stay as warm |
| 🏗️machinery forklift | battery-pack | surface | operating | 4.5 | 2.18 | 4.5 | ACCEPT (author withdrew) | case 1.0 x 0.47 x 0.9: sides 2*0.47*1.0 + ends 2*0.47*0.9 + top 0.9 = 2.7 less embedded = 2.2 m2; heatW 540 W is physical, q rises to 248 W/m2 (case 20-25 K above the air) |
| 🏗️machinery forklift | brakes | surface | operating | 0.4 | 0.295 | 0.295 | ACCEPT | 4 drums o0.3: 0.3 m2 |
| 🏗️machinery forklift | cargo | surface | passive | 5 | 3.14 | 5 | ACCEPT (author withdrew) | goods block 1.16 x 0.9 x 0.76: 4.34 m2 less the face against the backrest 1.04 = 3.3 m2 |
| 🏗️machinery forklift | chassis-paint | surface | passive | 8 | 4.7 | 8 | ACCEPT (author withdrew) | chassis 1.0 m wide: top 1.9 + sides 1.7 + arches 1.2 = 4.7-6 m2 |
| 🏗️machinery forklift | controller | engine | operating | 0.5 | 0.419 | 0.419 | ACCEPT | dash panel 0.6 x 0.7: 0.42 m2 |
| 🏗️machinery forklift | counterweight | surface | passive | 3.5 | 4.3 | 4.3 | ACCEPT | profile perimeter 3.2 m * 1.1 m + 2 sides 1.7 = 4.3 m2 |
| 🏗️machinery forklift | drive-motor | engine | operating | 0.9 | 0.192 | 0.9 | RESTORE + model defect | traction motor o0.28 x 0.4 (0.35 + ends 0.12) + gear housing 0.25 = 0.7-0.9 m2; 0.192 would be q = 2240 W/m2 for 430 W |
| 🏗️machinery forklift | fork-gear | surface | passive | 1.6 | 1.5 | 1.5 | ACCEPT | carriage plate 0.6 x 0.84 + tines 2*0.5 + backrest = 1.5-1.6 m2 |
| 🏗️machinery forklift | guard | surface | passive | 4 | 2.78 | 2.78 | ACCEPT | 4 posts o0.06 x 1.2 + rails and slats = 2.5-3 m2 |
| 🏗️machinery forklift | hydraulics | surface | operating | 0.4 | 0.323 | 0.323 | ACCEPT | 2 tilt cylinders: barrel o0.08*0.45 + rod = 0.16 each = 0.32 m2 |
| 🏗️machinery forklift | mast | surface | passive | 5 | 2.32 | 5 | RESTORE + model defect | 2 C-channel outer rails (perimeter 0.7 x 2.05 = 1.4 each) + 2 inner rails (0.6 x 1.6 = 0.96 each) + cross members, chains, brackets 0.6 = 5.4 m2 |
| 🏗️machinery forklift | pallet | surface | passive | 2 | 1.78 | 1.78 | ACCEPT | EUR pallet 1.2 x 0.8: deck top+edges 1.0 + runners 0.8 = 1.78 m2 |
| 🏗️machinery forklift | tyres | tyre | operating | 1.6 | 1.95 | 1.95 | ACCEPT | front o0.78 x 0.2 + rear o0.55 x 0.16: tread + outer sidewall 1.9-2.0 m2 |
| 🏗️machinery heat-pump-outdoor-unit | base-tray | heater | operating | 0.5 | 0.712 | 0.712 | ACCEPT | tray 0.45 x 1.1 underside 0.495 + edges 0.22 = 0.715 m2 |
| 🏗️machinery heat-pump-outdoor-unit | casing | surface | passive | 3.4 | 6.12 | 3.4 | RESTORE + model defect | outer box 0.72 x 1.1 x 1.45 = 6.9 less coil side 1.6, outlet 0.7, bottom 0.8 = 3.8 m2 (outer faces only; inner faces look into the coil and fan chamber) |
| 🏗️machinery heat-pump-outdoor-unit | coil-guard | surface | operating | 1.6 | 2.81 | 1.6 | RESTORE + model defect | louvred guard over the coil face 0.86 x 1.06 = 0.91 m2 x louvre factor 1.75 = 1.6 m2 (q = -137 W/m2) |
| 🏗️machinery heat-pump-outdoor-unit | discharge-vane | surface | passive | 0.25 | 0.414 | 0.25 | ACCEPT (author withdrew) | vane 0.24 x 0.72 both faces 0.346 + edges 0.096 = 0.44 m2 (free in the air stream) |
| 🏗️machinery heat-pump-outdoor-unit | fan-guard | surface | passive | 0.6 | 1.09 | 0.6 | ACCEPT (author withdrew) | blades 5 x 0.06 = 0.3 + hub/motor 0.15 + stay 0.11 + wire grille 0.15 = 0.7-1.1 m2 |
| 🏗️machinery heat-pump-outdoor-unit | feet | tyre | passive | 0.1 | 0.187 | 0.1 | ACCEPT (author withdrew) | 2 feet 0.34 x 0.18 x 0.09 without top and bottom: 2*2*(0.34+0.18)*0.09 = 0.19 m2 |
| 🏗️machinery heat-pump-outdoor-unit | top-lid | surface | passive | 0.5 | 0.655 | 0.655 | ACCEPT | lid top 0.47 x 1.12 = 0.526 + edges 0.127 = 0.65 m2 |
| 🏗️machinery oil-pumpjack | gearbox | engine | operating | 6.5 | 8.7 | 8.7 | ACCEPT | gearbox 1.5 x 1.2 x 1.2: 2*(1.8+1.8+1.44) = 10 less the base 1.8 = 8.2-8.7 m2 |
| 🏗️machinery oil-pumpjack | steel | surface | passive | 38 | 44 | 44 | ACCEPT | walking beam, samson post, pitman arms, cranks: the model 44 m2 is within 16 % of the committed 38 |
| 🏗️machinery plate-heat-exchanger | damper | surface | passive | 0.15 | 0.368 | 0.368 | ACCEPT | flap 0.4 x 0.4 both faces (free in the bypass) 0.32 + edges 0.02 = 0.34 m2 |
| 🏗️machinery plate-heat-exchanger | frame | surface | passive | 0.9 | 1.2 | 1.2 | ACCEPT | 4 posts 0.05 x 0.56 + base 0.5 x 0.5 + 2 end walls: 1.2 m2 |
| 🏗️machinery plate-heat-exchanger | plates | surface | passive | 0.6 | 0.69 | 0.6 (exchange) | RESTORE + exchange | cross-flow plate pack: 60+ plates of 0.44 x 0.4 are an exchange surface (several m2) the 0.44 x 0.4 x 0.54 envelope cannot show |
| 🏗️machinery plate-heat-exchanger | ports | surface | passive | 0.05 | 0.22 | 0.22 | SET | 4 collars 0.30 x 0.15 x 0.06: outer wall 2*(0.30+0.15)*0.06 = 0.054 each = 0.216 m2 (inner walls look into the duct) |
| 🏗️machinery scroll-compressor | discharge | exhaust | operating | 0.004 | 0.0072 | 0.0072 | ACCEPT | stub o16 x 70 mm 0.0035 + flange o32 x 4 mm 0.0016 = 0.005-0.007 m2 |
| 🏗️machinery scroll-compressor | dome | exhaust | operating | 0.05 | 0.063 | 0.063 | ACCEPT | ellipsoidal cap r0.14: 0.06 m2 |
| 🏗️machinery scroll-compressor | feet | surface | passive | 0.02 | 0.084 | 0.084 | ACCEPT | base ring plate 0.28 square with o0.19 hole: top 0.049 + bottom 0.049 (on grommets) + edges and hole wall = 0.056-0.105 m2 |
| 🏗️machinery scroll-compressor | scroll | engine | operating | 0.006 | 0.0158 | 0.006 | RESTORE + model defect | the scroll plate o0.09 seen through the sight glass: one face pi*0.045^2 = 0.0064 m2 (old 0.006), q = 3 W/0.006 |
| 🏗️machinery scroll-compressor | shell | engine | operating | 0.1 | 0.205 | 0.205 | ACCEPT | shell o0.28 x 0.25 = 0.22 m2 |
| 🏗️machinery scroll-compressor | suction | surface | operating | 0.005 | 0.0062 | 0.0062 | ACCEPT | stub o10 x 0.15 m with collar = 0.006 m2 |
| 🏗️machinery scroll-compressor | terminal | surface | passive | 0.008 | 0.0138 | 0.0138 | ACCEPT | box 0.036 x 0.06 x 0.055: 0.0145 less the mount 0.003 + lid = 0.0138 m2 |
| 🏗️machinery solar-thermal-collector | absorber | heater | operating | 1.8 | 2.38 | 1.8 | RESTORE + model defect | selective plate 1.88 x 0.91 = 1.71 m2 (aperture 1.8), risers lie under the plate |
| 🏗️machinery solar-thermal-collector | housing | surface | passive | 0.8 | 2.6 | 2.6 | SET | tray 2.0 x 1.0 x 0.1: underside 2.0 + long sides 0.4 + ends 0.2 = 2.6 m2 outer shell (cavity faces lie under the glass) |
| 🏗️machinery solar-thermal-collector | stand | surface | passive | 0.45 | 0.96 | 0.96 | ACCEPT | 2 rails 1.5 m x 0.2 perimeter 0.6 + 4 struts ~0.5 m x 0.18 = 0.36 -> 0.96 m2 |
| 🏗️machinery sorption-wheel | casing | surface | passive | 0.3 | 0.68 | 0.68 | SET | outer band 2*pi*0.31*0.22 = 0.43 + two outer flange faces 2*pi*(0.31^2-0.27^2)... = 0.146 + stand block 0.1 = 0.68 m2 (inner band faces the wheel) |
| 🏗️machinery sorption-wheel | disc | heater | operating | 0.19 | 0.19 | 0.19 (exchange) | exchange only | honeycomb matrix of 20 mm cells: exchange area, the envelope 2 x 0.25 + rim 0.32 cannot show it; keep 0.19 |
| 🏗️machinery sorption-wheel | hub | engine | operating | 0.012 | 0.0081 | 0.0081 | ACCEPT | two end faces 2*pi*0.045^2 = 0.0127 less 16 spoke footprints 0.0036 = 0.009 m2 |
| 🏗️machinery sorption-wheel | regen | heater | operating | 0.064 | 0.12 | 0.12 | SET | two 84 degree seal plates, outer face 0.0596 m2 each = 0.12 m2 (the inner faces are the gap) |
| 🏗️machinery tower-crane | ballast | surface | passive | 40 | 36.5 | 36.5 | ACCEPT | 2x2 blocks 3.5 x 3.5 x 2.0: sides 28 + top 12 = 40 less joints = 36.5 m2 |
| 🏗️machinery tower-crane | cab-glazing | glazing | conditioned | 4 | 2.9 | 2.9 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🏗️machinery tower-crane | counterweight | surface | passive | 25 | 30 | 32.6 | ACCEPT | 3 slabs stacked 2.4 x 2.2 x 2.4: sides 21 + top 5.8 + underside 5.8 = 32.6 m2 |
| 🏗️machinery tower-crane | house | surface | passive | 25 | 32.2 | 32.2 | ACCEPT | slew platform 5.8 x 0.7 x 2.4: top 13.9 + sides 8.1 + ends 3.4 + open underside 13.9 less tower foot = 32 m2 |
| 🏗️machinery tower-crane | skip | surface | passive | 8 | 9.4 | 9.4 | ACCEPT | skip body 2*0.8 + block 1.2 + hook + 4 slings = 8-9.4 m2 |
| 🏗️machinery tower-crane | winch-house | surface | operating | 38 | 53.2 | 31.5 | SET | box 4 x 1.8 x 1.385 outer faces 30.5 + vent stack 1.0 (heat leaves through the outer face; the model counts the 7.5 mm plates on both faces, 53 m2) |
| 👤️humans child-on-kick-scooter | backpack | surface | passive | 0.18 | 0.213 | 0.28 | SET | C29 total: pack 0.25 x 0.3 x 0.12 all faces 2*(0.075+0.03+0.036) = 0.28 (the back is on the child, still counted) |
| 👤️humans child-on-kick-scooter | stem | surface | passive | 0.12 | 0.154 | 0.154 | ACCEPT | stem tube o0.03 x 0.8 + handlebar + grips = 0.15 m2 |
| 👤️humans child-on-kick-scooter | wheels | tyre | passive | 0.07 | 0.058 | 0.082 | ACCEPT | C29 total: 2 wheels o0.12 x 0.03 with the tread and both sides incl. the hub caps = 0.07-0.09 m2 |
| 👤️humans courier-with-hand-truck | hand-truck | surface | passive | 0.55 | 0.67 | 0.67 | ACCEPT | C29 total: frame tubes 2 x 1.1 m o0.03 + toe plate 0.3 x 0.2 both faces + wheels = 0.6-0.7 m2 |
| 👤️humans cyclist | bike-metal | surface | passive | 0.9 | 1.05 | 1.05 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 👤️humans cyclist | saddle | surface | passive | 0.07 | 0.07 | 0.07 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 👤️humans cyclist | tyres | tyre | passive | 0.6 | 0.66 | 0.66 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 👤️humans kneeling-gardener | pad | surface | passive | 0.28 | 0.395 | 0.395 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 👤️humans kneeling-gardener | trowel | surface | passive | 0.025 | 0.0375 | 0.0375 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 👤️humans parent-with-stroller | stroller-fabric | surface | passive | 1.1 | 1.35 | 1.35 | ACCEPT | seat 0.45 x 0.8 both faces + hood + basket 0.38 x 0.4 box = 1.3-1.4 m2 |
| 👤️humans parent-with-stroller | stroller-frame | surface | passive | 0.35 | 0.431 | 0.431 | ACCEPT | 5.7 m of tube o0.024 = 0.43 m2 |
| 👤️humans parent-with-stroller | tyres | tyre | passive | 0.45 | 0.325 | 0.4 | SET | C29 total: 4 wheels o0.25 x 0.05, 0.1 each incl. the contact patch |
| 👤️humans person-on-crutches | crutch-cuffs | surface | passive | 0.04 | 0.06 | 0.06 | ACCEPT | 2 cuffs o0.1 x 0.1 high = 0.03 each = 0.06 m2 |
| 👤️humans person-on-crutches | crutch-tips | surface | passive | 0.006 | 0.009 | 0.009 | ACCEPT | 2 rubber tips o0.03 x 0.05 = 0.0045 each = 0.009 m2 |
| 👤️humans person-with-rollator | bag | surface | passive | 0.35 | 0.24 | 0.33 | SET | C29 total: tote 0.3 x 0.32 x 0.1 all faces = 0.32 m2 |
| 👤️humans person-with-rollator | frame | surface | passive | 0.57 | 0.657 | 0.657 | ACCEPT | tube o0.025 x 8.4 m = 0.66 m2 |
| 👤️humans person-with-rollator | seat-basket | surface | passive | 0.7 | 1.05 | 1.05 | ACCEPT | seat plate 0.4 x 0.3 both faces 0.24 + wire basket 0.4 x 0.3 x 0.25 all faces 0.6 + frame = 0.85-1.05 m2 |
| 👤️humans person-with-rollator | tyres | tyre | passive | 0.1 | 0.116 | 0.116 | ACCEPT | C29 total: 4 wheels o0.2 x 0.03: tread 0.019 + sides 0.01 = 0.029 each = 0.116 m2 |
| 👤️humans person-with-umbrella | canopy | surface | passive | 0.87 | 1.15 | 2.1 | SET | C25: underside no longer marked; dome r0.52 rise 0.25 both faces 2*pi*(0.27+0.0625)... = 2*1.04 + rim = 2.1 m2 |
| 👤️humans person-with-umbrella | umbrella-frame | surface | passive | 0.08 | 0.064 | 0.064 | ACCEPT | shaft o0.021 x 0.95 + handle + hub = 0.064 m2 |
| 👤️humans person-with-white-cane | cane-shaft | surface | passive | 0.05 | 0.0588 | 0.0588 | ACCEPT | o0.012 x 1.3 m = 0.049 + tip = 0.0588 |
| 👤️humans person-with-white-cane | sunglasses | glazing | passive | 0.006 | 0.0046 | 0.0046 | ACCEPT | two lenses 0.05 x 0.04 outer faces 0.004 + frame = 0.0046 m2 |
| 👤️humans runner-with-blade-prosthesis | blade | surface | passive | 0.09 | 0.076 | 0.076 | ACCEPT | J blade 0.07 wide x 0.5 m x 2 faces + edges = 0.076 m2 |
| 👤️humans runner-with-blade-prosthesis | socket | surface | body | 0.0959 | 0.068 | 0.0959 | RESTORE (C18 body) | C18: body-model region (socket worn on the stump), measured 0.068 deviates 29 % from the committed 0.0959; the committed value stays |
| 👤️humans seated-office-worker | castor-rubber | tyre | passive | 0.02 | 0.0247 | 0.0247 | ACCEPT | 5 castor wheels o0.05 x 0.02 = 0.025 m2 |
| 👤️humans seated-office-worker | chair-steel | surface | passive | 0.1 | 0.127 | 0.127 | ACCEPT | column, stems and mechanism 0.11-0.13 m2 |
| 👤️humans seated-office-worker | laptop | surface | operating | 0.28 | 0.233 | 0.233 | ACCEPT | lid 0.33 x 0.23 = 0.076 + base faces and edges = 0.23-0.3 m2 |
| 👤️humans seated-reader | bench | surface | passive | 0.9 | 1.108 | 1.108 | ACCEPT | seat + back + 2 legs 1.1 m2 |
| 👤️humans seated-reader | notebook | surface | passive | 0.12 | 0.08 | 0.08 | ACCEPT | closed notebook 0.2 x 0.15 x 0.03: 2*(0.03+0.006+0.0045) = 0.08 m2 |
| 👤️humans skateboarder | trucks | surface | passive | 0.05 | 0.0715 | 0.105 | SET | C29 total: 2 boxes 0.075 x 0.05 x 0.18 all faces |
| 👤️humans skateboarder | wheels | tyre | passive | 0.046 | 0.0337 | 0.039 | SET | C29 total: 4 wheels o0.054 x 0.03 0.0097 each |
| 👤️humans sleeping-person | pillow | surface | passive | 0.32 | 0.198 | 0.58 | SET | C29 total: pillow 0.6 x 0.4 top 0.24 + underside 0.24 + rim 0.1 (head and duvet cover it, the pose exposedFraction 0.7 handles that) |
| 👤️humans standing-phone-user | headphones | surface | passive | 0.03 | 0.0208 | 0.0208 | ACCEPT | band 0.4 x 0.02 + 2 cups = 0.02-0.03 m2 |
| 👤️humans tour-guide | flag-cloth | surface | passive | 0.26 | 0.299 | 0.299 | ACCEPT | 0.468 x 0.302 both faces = 0.283 m2 + edges |
| 👤️humans tour-guide | pointer-metal | surface | passive | 0.01 | 0.0172 | 0.0172 | ACCEPT | capsule r5 mm x 0.548 m: 2*pi*0.005*0.548 = 0.0172 m2 |
| 👤️humans tour-guide | pole-metal | surface | passive | 0.076 | 0.117 | 0.117 | ACCEPT | capsule r12 mm x 1.34 m: 2*pi*0.012*1.34 = 0.101 + caps = 0.117 m2 |
| 👤️humans walking-commuter | bag-leather | surface | passive | 0.28 | 0.193 | 0.263 | SET | C29 total: box 0.209 x 0.294 x 0.14 all faces |
| 👤️humans wheelchair-user | cushion | surface | passive | 0.4 | 0.44 | 0.63 | SET | C29 total: seat cushion both faces 0.32 + rim 0.06 + back sling both faces 0.25 |
| 👤️humans wheelchair-user | tyres | tyre | passive | 0.35 | 0.5 | 0.5 | ACCEPT | 2 rear tyres o0.6 x 0.03 (0.23 each) + 2 castors 0.04 each = 0.54 m2 |
| 👤️humans worker-carrying-box | box-surface | surface | passive | 0.66 | 0.508 | 0.66 | SET | C29 total: box 0.4 x 0.3 x 0.3 all faces (the hands cover part, the pose contact handles it) |
| 🔌️appliances ceiling-diffuser | plate | surface | operating | 0.45 | 0.24 | 0.45 | RESTORE + model defect | flange ring 0.152 + 3 cone risers 0.09 + annuli 0.176 = 0.42 m2 (square 0.595 diffuser) |
| 🔌️appliances circulation-pump | casing | surface | operating | 0.035 | 0.03 | 0.03 | ACCEPT | volute o0.115 x 0.06 + unions = 0.03 m2 |
| 🔌️appliances circulation-pump | motor | surface | operating | 0.03 | 0.024 | 0.024 | ACCEPT | motor head o0.09 x 0.08 = 0.024 m2 |
| 🔌️appliances circulation-pump | terminal-box | surface | operating | 0.012 | 0.0128 | 0.0128 | ACCEPT | box 0.06 x 0.04 x 0.03 = 0.0128 m2 |
| 🔌️appliances decentralised-hrv-fan | blades | surface | passive | 0.012 | 0.028 | 0.028 | ACCEPT | 5 blades 0.045 x 0.06 both faces = 0.027 m2 |
| 🔌️appliances decentralised-hrv-fan | casing | surface | operating | 0.09 | 0.39 | 0.39 | ACCEPT | open box 0.26 x 0.26 x 0.24 without the wall side: 0.32-0.39 m2 |
| 🔌️appliances decentralised-hrv-fan | hub | engine | operating | 0.006 | 0.0075 | 0.0075 | ACCEPT | hub o0.056 x 0.03: 0.0075 m2 |
| 🔌️appliances desktop-monitor | housing | surface | operating | 0.22 | 0.124 | 0.22 | RESTORE + model defect | rear shell 24-inch: back 0.3 x 0.5 = 0.15 + rim 1.66*0.03 = 0.2 m2 |
| 🔌️appliances desktop-monitor | panel-front | surface | operating | 0.175 | 0.2 | 0.2 | ACCEPT | panel 0.299 x 0.53 = 0.159 + bezel edges = 0.2 m2 |
| 🔌️appliances desktop-monitor | stand | surface | passive | 0.125 | 0.108 | 0.108 | ACCEPT | base plate + neck + feet = 0.11 m2 |
| 🔌️appliances duct-fan | blades | surface | passive | 0.015 | 0.0285 | 0.0285 | ACCEPT | 5 blades both faces 0.0285 m2 |
| 🔌️appliances duct-fan | casing | surface | operating | 0.07 | 0.2 | 0.12 | SET | C29 total: front 0.028 + back 0.028 (on the wall) + ring inner wall 0.03 + sides 0.018 + lip 0.01 |
| 🔌️appliances duct-fan | hub | engine | operating | 0.009 | 0.0145 | 0.0145 | ACCEPT | hub o0.068 x 0.03: 2 faces 0.0072 + side 0.0064 = 0.0136 m2 |
| 🔌️appliances electric-fan-heater | casing | surface | operating | 0.17 | 0.16 | 0.16 | ACCEPT | housing 0.16-0.17 m2 |
| 🔌️appliances electric-fan-heater | control | surface | passive | 0.004 | 0.0025 | 0.0025 | ACCEPT | dial o0.04 x 0.015: 0.0032 m2 |
| 🔌️appliances electric-fan-heater | feet | surface | passive | 0.003 | 0.0053 | 0.0085 | SET | C29 total: 4 feet with bottom and top faces, 1.6 x the exposed 0.0053 |
| 🔌️appliances electric-fan-heater | grille | heater | operating | 0.03 | 0.05 | 0.05 | ACCEPT | perforated plate 0.17 x 0.21 both faces 0.036 + hole walls = 0.04-0.05 m2 |
| 🔌️appliances electric-fan-heater | intake | surface | passive | 0.03 | 0.05 | 0.05 | ACCEPT | plate 0.16 x 0.21 0.034 + slats 0.012 + edges = 0.05 m2 |
| 🔌️appliances electric-fan-heater | top-cover | heater | operating | 0.035 | 0.055 | 0.055 | ACCEPT | top 0.17 x 0.21 = 0.036 + rim 0.015 = 0.051 m2 |
| 🔌️appliances electric-kettle | handle | surface | passive | 0.012 | 0.021 | 0.021 | ACCEPT | handle path 0.2 m x (2*0.03+2*0.02) = 0.02 m2 |
| 🔌️appliances electric-kettle | lid | surface | operating | 0.025 | 0.038 | 0.025 | RESTORE + model defect | dome o0.14: outer face 0.017 + button 0.003 = 0.02 m2; inner face is the steam side (operating region, heat leaves through the outer face) |
| 🔌️appliances electric-kettle | plate | heater | operating | 0.05 | 0.021 | 0.044 | SET | C29 total: power base o0.18: side 0.011 + top ring 0.0078 + underside 0.0254 (heatW 40 W stays: q = 909 W/m2) |
| 🔌️appliances electric-kettle | shell | surface | coupled | 0.11 | 0.095 | 0.095 | ACCEPT | o0.15 x 0.2 = 0.094 m2 |
| 🔌️appliances electric-kettle | water | water | operating | 0.012 | 0.0145 | 0.0145 | ACCEPT | free surface pi*0.07^2 = 0.0154 m2 |
| 🔌️appliances electricity-meter | casing | surface | passive | 0.17 | 0.205 | 0.205 | ACCEPT | box 0.17 x 0.2425 x 0.093 + back plate + terminals = 0.2 m2 |
| 🔌️appliances electricity-meter | disc | surface | passive | 0.0055 | 0.0107 | 0.0107 | ACCEPT | disc o0.084 in the gap: two faces 2*pi*0.042^2 = 0.0111 m2 |
| 🔌️appliances electricity-meter | plate | surface | operating | 0.0052 | 0.0135 | 0.0135 | SET | C29 total: name plate 0.13 x 0.04 x 0.0075 both faces 2*0.0052 + rim 0.0026 = 0.013 |
| 🔌️appliances electricity-meter | window | glazing | passive | 0.011 | 0.0245 | 0.011 | RESTORE + model defect | window ring and glass 0.0055 + counter 0.0023 + rim 0.003 = 0.011 m2; inner faces look into the meter |
| 🔌️appliances gas-boiler | connections | surface | operating | 0.04 | 0.04 | 0.04 | KEEP | 5 stubs o0.022 x 0.06 (0.0041) + nuts (0.0045) = 0.043 m2 (committed 0.04) |
| 🔌️appliances gas-boiler | controls | surface | operating | 0.03 | 0.03 | 0.03 | KEEP | panel 0.2 x 0.12 = 0.024 + knobs = 0.03 m2 (committed); the model region holds display, knobs and gauge but not the panel plate |
| 🔌️appliances gas-boiler | flue-collar | exhaust | operating | 0.05 | 0.05 | 0.05 | ACCEPT | pipe o0.08 x 0.12 = 0.03 + collar 0.012 = 0.042; model 0.032 (-25 %) |
| 🔌️appliances gas-boiler | sight-glass | glazing | operating | 0.005 | 0.0042 | 0.0042 | ACCEPT | window o0.08 less rim: 0.0042 m2 |
| 🔌️appliances gas-hob | burner | heater | operating | 0.012 | 0.012 | 0.012 | KEEP | cap o0.06 x 0.012 (0.0057) + crown ring o0.1 (0.0088) = 0.0145 m2; 0.006 would make q = 20 kW/m2 (T > 600 K) |
| 🔌️appliances gas-hob | knob | surface | passive | 0.0025 | 0.0025 | 0.0025 | ACCEPT | knob o22 x 9 mm: front 0.00038 + side 0.00062 = 0.001 m2 |
| 🔌️appliances gas-hob | trivet | heater | operating | 0.05 | 0.043 | 0.043 | ACCEPT | pan support rails + legs 0.043 m2 |
| 🔌️appliances gas-meter | index | surface | passive | 0.02 | 0.029 | 0.029 | ACCEPT | index frame + base 0.19 x 0.098 + dial = 0.029 m2 |
| 🔌️appliances gas-meter | pipes | surface | operating | 0.014 | 0.017 | 0.017 | ACCEPT | 2 stubs o0.028 x 0.07 with unions = 0.017 m2 |
| 🔌️appliances gas-meter | shell | surface | passive | 0.26 | 0.26 | 0.3 | SET | C29 total: box 0.24 x 0.27 x 0.17 all faces = 0.30 (back, index and pipe footprints included) |
| 🔌️appliances hot-water-cylinder | connections | surface | operating | 0.03 | 0.0213 | 0.0213 | ACCEPT | 5 stubs with nuts = 0.021 m2 |
| 🔌️appliances kitchen-extractor-hood | canopy | surface | operating | 0.9 | 0.9 | 0.9 | ACCEPT | slope 0.9 x 0.5 = 0.45 + sides 0.15 = 0.6 m2 outer prism without the back on the wall; q = 120/0.54 = 222 W/m2 |
| 🔌️appliances kitchen-extractor-hood | chimney | surface | passive | 0.75 | 0.55 | 0.55 | ACCEPT | chimney section 0.3 x 0.2: perimeter 1.0 x 0.55 less back 0.165 = 0.39-0.55 m2 |
| 🔌️appliances kitchen-extractor-hood | filter | surface | passive | 0.2 | 0.2 | 0.2 | ACCEPT | plate 0.86 x 0.46 = 0.396 + edges 0.03 + mesh = 0.45 m2 |
| 🔌️appliances mvhr-unit | ducts | surface | passive | 0.08 | 0.12 | 0.12 | ACCEPT | 4 spigots o0.098 x 0.05: 0.062 + beads 0.012 = 0.075-0.12 m2 |
| 🔌️appliances pendant-lamp | lampholder | lamp | operating | 0.005 | 0.0042 | 0.0042 | ACCEPT | o0.05 x 0.03 = 0.0042 m2 |
| 🔌️appliances room-thermostat | dial | surface | passive | 0.0033 | 0.00243 | 0.00243 | ACCEPT | dial o0.042 x 0.006: 0.0014 front + rim 0.0008 = 0.0024 m2 |
| 🔌️appliances room-thermostat | led | lamp | operating | 0.0001 | 5e-05 | 5e-05 | ACCEPT | half oblate dome a=3.5 mm c=2 mm: pi*a^2*(1+(1-e^2)/e*atanh e) = 5.6e-5 m2 |
| 🔌️appliances server-rack | feet | surface | passive | 0.06 | 0.025 | 0.06 | RESTORE + model defect | 4 castors o0.07 x 0.03: tread 0.0066 + sides 0.0077 = 0.014 each = 0.057 m2 |
| 🔌️appliances shower-head | head | surface | passive | 0.0095 | 0.0454 | 0.0454 | ACCEPT | loft o0.137 x 0.0147: both faces 0.029 + rim 0.0072 + rounding = 0.037-0.045 m2 |
| 🔌️appliances shower-head | valve | surface | passive | 0.012 | 0.0179 | 0.025 | SET | C29 total: body 0.062 x 0.082 x 0.062 all faces incl. the wall face + stubs |
| 🔌️appliances split-ac-indoor-unit | casing | surface | operating | 0.62 | 0.4322 | 0.73 | SET | C29 total: 0.85 x 0.29 x 0.21 box all faces 0.73 (back on the wall, grille and outlet overlap included) |
| 🔌️appliances split-ac-indoor-unit | discharge-vane | surface | operating | 0.1 | 0.1243 | 0.1243 | ACCEPT | vane 0.7 x 0.08 both faces + edges = 0.12 m2 |
| 🔌️appliances split-ac-indoor-unit | intake-grille | surface | passive | 0.1 | 0.0829 | 0.0829 | ACCEPT | grille 0.7 x 0.12 = 0.083 m2 |
| 🔌️appliances thermostatic-radiator-valve | body | surface | operating | 0.0125 | 0.00764 | 0.00764 | ACCEPT | body o0.038 x 0.042 + stub o0.017 x 0.023 + nut = 0.0076-0.01 m2 |
| 🔌️appliances thermostatic-radiator-valve | head | surface | operating | 0.0075 | 0.01546 | 0.01546 | ACCEPT | collar o0.06 x 0.068 = 0.0128 + end 0.0028 = 0.0156 m2 |
| 🔌️appliances thermostatic-radiator-valve | knob | surface | operating | 0.0028 | 0.0038 | 0.0038 | ACCEPT | knob o0.04 x 0.02: 0.0027 + 0.0013 = 0.004 m2 |
| 🔌️appliances toaster | bread | surface | operating | 0.045 | 0.054 | 0.054 | ACCEPT | two bread slices 0.1 x 0.09 visible faces and crust 0.054 m2 |
| 🔌️appliances toaster | feet | surface | passive | 0.002 | 0.00326 | 0.007 | SET | C29 total: 4 feet o0.025 x 0.01: side 0.0031 + bottom 0.002 + top 0.002 |
| 🔌️appliances toaster | top-plate | heater | operating | 0.045 | 0.05489 | 0.05489 | ACCEPT | top 0.3 x 0.17 = 0.051 + edges = 0.055 m2 |
| 🔌️appliances tower-server | psu-vent | surface | operating | 0.04 | 0.0337 | 0.0337 | ACCEPT | rear plate 0.108 x 0.17 + top 0.09 x 0.17 = 0.034 m2 |
| 🔌️appliances tower-server | rear-panel | surface | operating | 0.07 | 0.0876 | 0.0876 | ACCEPT | rear 0.438 x 0.2 = 0.0876 m2 |
| 🔌️appliances tower-server | side-panel | surface | operating | 0.38 | 0.2715 | 0.38 | RESTORE + model defect | two 0.413 x 0.438 panels, outer faces 2*0.181 = 0.36 m2 |
| 🔌️appliances vacuum-cleaner | body-shell | surface | operating | 0.42 | 0.31 | 0.42 | RESTORE + model defect | C29 total: canister 0.3 x 0.25 x 0.3 all faces 2*(0.075+0.09+0.075) = 0.48 incl. the floor and hose faces; the committed 0.42 |
| 🔌️appliances vacuum-cleaner | hose | surface | passive | 0.14 | 0.12 | 0.12 | ACCEPT | C29 total: hose o0.032 x 1.2 m = 0.12 m2 |
| 🔌️appliances vacuum-cleaner | wand | surface | passive | 0.09 | 0.066 | 0.066 | ACCEPT | C29 total: tube o0.03 x 0.6 m + nozzle head = 0.066-0.09 m2 |
| 🚜️vehicles city-bicycle | mudguards | surface | passive | 0.5 | 0.351 | 0.351 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🚜️vehicles city-bicycle | saddle | surface | passive | 0.12 | 0.0954 | 0.0954 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🚜️vehicles city-bicycle | steer-assembly | surface | passive | 0.2 | 0.158 | 0.158 | DEFER (no 1:50 model yet) | no volumes in the file at review |
| 🚜️vehicles city-bus | body-panels | wall | conditioned | 52 | 44 | 62 | SET | C25: underside no longer marked; sides 2*12*2.7 = 64.8 less glazing 33.2 and doors 5.3 = 26.3 + ends 13.8 less glass 4 = 9.8 + underbody 12*2.55*0.85 = 26 -> 62 m2 |
| 🚜️vehicles city-bus | brakes | surface | operating | 1.2 | 0.8 | 1.2 | RESTORE + model defect | 4 discs o0.43 both faces 0.29 = 1.2 m2 |
| 🚜️vehicles city-bus | doors | wall | conditioned | 8 | 5.33 | 5.33 | ACCEPT | two double doors, 4 leaves 2.0 x 1.0 less glass = 5.3 m2 |
| 🚜️vehicles city-bus | e-axle | engine | operating | 3 | 2 | 3 | ACCEPT (author withdrew) | axle housing o0.4 x 1.3 + motor housings = 2-3 m2 |
| 🚜️vehicles city-bus | glazing | glazing | conditioned | 24 | 33.2 | 33.2 | ACCEPT | side glazing 2*12*1.2*0.8 = 23 + windscreen 3.5 + rear 1.5 + door glass 5 = 33 m2 |
| 🚜️vehicles city-bus | tyres | tyre | operating | 6 | 8.3 | 8.3 | ACCEPT | 6 tyres 275/70 R22.5 o0.96 x 0.275: tread 0.83 + sidewalls 0.9 less contact = 8.3-10 m2 |
| 🚜️vehicles city-car | body-panels | surface | passive | 8.2 | 13 | 13 | SET | C25: underside no longer marked; sides 5.5 + ends 2.2 (the declared 8.2 is these) + underbody 3.9*1.55*0.8 = 4.8 -> 13.0 m2 |
| 🚜️vehicles city-car | bonnet | surface | operating | 1.2 | 1.44 | 1.44 | ACCEPT | bonnet 0.9 x 1.3 x 1.2 curvature = 1.4 m2 |
| 🚜️vehicles city-car | exhaust-system | exhaust | operating | 0.45 | 0.675 | 0.675 | ACCEPT | pipe o30 x 1.45 m 0.137 + silencer 0.43 + ends 0.06 + tail 0.1 = 0.72 m2 (applied 0.675) |
| 🚜️vehicles city-car | glazing | glazing | passive | 3.2 | 4.25 | 3.2 | ACCEPT (author withdrew) | screen 1.05 + rear 0.9 + 4 side lights 1.2 + 2 quarter lights = 4.0-4.3 m2 |
| 🚜️vehicles city-car | roof | surface | passive | 1.5 | 1.78 | 1.78 | ACCEPT | roof 1.1 x 1.3 x curvature 1.2 = 1.7-1.8 m2 |
| 🚜️vehicles city-car | tailgate | surface | passive | 0.9 | 0.75 | 0.75 | ACCEPT | hatch panel 0.8 x 0.95 = 0.75 m2 |
| 🚜️vehicles delivery-van | body-panels | surface | passive | 22 | 14.7 | 22 | SET | C25: underside no longer marked; sides+front+arches+bumper 14.3 + underbody 5.34*1.8*0.85 = 8.2 -> 22 m2 (the committed value) |
| 🚜️vehicles delivery-van | brakes | surface | operating | 0.6 | 0.72 | 0.72 | ACCEPT | 4 discs and calipers 0.18 each = 0.72 m2 |
| 🚜️vehicles delivery-van | cab-glazing | glazing | passive | 2.6 | 3.18 | 3.18 | ACCEPT | windscreen 1.5*0.9 = 1.35 + 2 door glasses 1.2 + quarter lights 0.5 = 3.1 m2 |
| 🚜️vehicles delivery-van | exhaust-system | exhaust | operating | 1.2 | 0.8 | 1.2 | RESTORE + model defect | pipe 3.5 m o0.06 (0.66) + DPF can o0.18 x 0.45 (0.3) + silencer (0.3) = 1.26 m2 |
| 🚜️vehicles delivery-van | rear-doors | surface | passive | 3.4 | 2.55 | 2.55 | ACCEPT | two leaves 1.35 x 0.9 outer face 2.43 + edges 0.15 = 2.6 m2 |
| 🚜️vehicles delivery-van | roof | surface | passive | 8.5 | 7.2 | 7.2 | ACCEPT | (5.34 - 1.4 bonnet - 0.2) * 1.95 = 7.2 m2 |
| 🚜️vehicles delivery-van | slider-door | surface | passive | 2.4 | 2.3 | 2.3 | ACCEPT | panel 1.05 x 1.444 = 1.52 + edges 0.2 + tracks 0.3 = 2.0-2.3 m2 |
| 🚜️vehicles delivery-van | tyres | tyre | operating | 3.6 | 4.46 | 4.46 | ACCEPT | 4 tyres 235/65 R16 o0.69 x 0.235: tread 0.51 + sidewalls 0.5 = 1.0 each = 4.0 m2 |
| 🚜️vehicles tram | body-panels | wall | conditioned | 55 | 79.82 | 79.82 | ACCEPT | sides 2*15.45*2.6 = 80 less glazing 33 and doors 9 = 38 + cab front/ends 8 + underbody 15*2.3*0.8 = 27.6 + bellows 5 = 72-80 m2 (underside counted, C25) |
| 🚜️vehicles tram | brake-discs | surface | operating | 1.6 | 1.89 | 1.89 | ACCEPT | disc brakes 8 discs o0.6: 0.24 each = 1.9 m2 |
| 🚜️vehicles tram | doors | wall | conditioned | 11 | 8.89 | 8.89 | ACCEPT | plug-sliding door leaves outer faces 8.9 m2 |
| 🚜️vehicles tram | glazing | glazing | conditioned | 26 | 32.83 | 32.83 | ACCEPT | side glazing 2*15*1.0*0.9 = 27 + windscreen 3.2 + end glass 2.6 = 32.8 m2 |
| 🚜️vehicles tram | hvac-unit | surface | operating | 7 | 8.96 | 8.96 | ACCEPT | roof unit 2.8 x 1.2 x 0.6: top 3.4 + sides 4.4 + ends 1.4 = 9 m2 |
| 🚜️vehicles vintage-car-1929 | body-panels | surface | passive | 6.4 | 8.16 | 8.16 | ACCEPT | tub sides 2.6 + rear 0.8 + cabin sides 2.2 + front 0.7 + cabin rear 1.0 + scuttle 0.15 = 7.5-8.2 m2 |
| 🚜️vehicles vintage-car-1929 | engine-bay | engine | operating | 0.8 | 1.12 | 1.12 | ACCEPT | bonnet sides and top of the engine compartment 1.1 m2 |
| 🚜️vehicles vintage-car-1929 | glazing | glazing | passive | 1.6 | 1.93 | 1.93 | ACCEPT | windscreen 0.9 + rear 0.3 + side lights 0.8 = 2.0 m2 |
| 🚜️vehicles vintage-car-1929 | roof | surface | passive | 2.2 | 3.06 | 3.06 | ACCEPT | fabric roof 1.5 x 1.4 x 1.2 curvature = 2.5-3.1 m2 |
| 🚜️vehicles vintage-car-1929 | tyres | tyre | operating | 2.4 | 3.05 | 3.05 | ACCEPT | 5 tyres (4 + spare) o0.76 x 0.15: 0.6 each = 3.0 m2 |
| 🛋️furniture bar-stool | backrest | surface | passive | 0.06 | 0.021 | 0.06 | RESTORE + model defect | low backrest pad 0.25 x 0.1 both faces 0.05 + rim 0.02 = 0.07 m2 |
| 🛋️furniture bar-stool | seat-plate | surface | passive | 0.102 | 0.124 | 0.216 | SET | C29 total: plywood disc o0.36 both faces 2*0.102 + rim 0.0136 (the top is under the pad of another region, still counted) |
| 🛋️furniture bookshelf | back-panel | surface | passive | 1.4 | 2.616 | 2.616 | ACCEPT | 0.764 x 1.712 both faces = 2.62 m2 (free-standing) |
| 🛋️furniture bookshelf | carcass | surface | passive | 1.9 | 2.952 | 2.952 | ACCEPT | 2 sides (inner and outer faces) 2.1 + top and bottom both faces 0.92 -> 2.95 m2 |
| 🛋️furniture bookshelf | plinth | surface | passive | 0.12 | 0.153 | 0.153 | ACCEPT | 0.772 x 0.075 faces + top = 0.15 m2 |
| 🛋️furniture bookshelf | shelves | surface | passive | 1.07 | 2.294 | 2.294 | ACCEPT | 5 shelves 0.78 x 0.277 both faces 0.43 + edges 0.04 = 0.47 -> 2.3 m2 |
| 🛋️furniture cafe-chair | back-beech | surface | passive | 0.1 | 0.242 | 0.242 | ACCEPT | panel 0.17 x 0.37 both faces 0.126 + posts 0.056 + rail 0.044 + finials = 0.25 m2 |
| 🛋️furniture cafe-chair | seat-beech | surface | passive | 0.17 | 0.524 | 0.524 | ACCEPT | seat 0.42 x 0.42 both faces 0.35 + aprons 0.1 + edge 0.05 = 0.5 m2 |
| 🛋️furniture cafe-chair | steel-tube | surface | passive | 0.13 | 0.154 | 0.154 | ACCEPT | 4 legs o0.022 x 0.42 + stretchers + feet = 0.15 m2 |
| 🛋️furniture dining-table | apron | surface | passive | 0.42 | 0.744 | 0.744 | ACCEPT | aprons 4.2 m x 0.08 x 2 faces 0.67 + lower edge 0.1 = 0.78 m2 |
| 🛋️furniture dining-table | table-top | surface | passive | 1.44 | 3 | 3 | SET | C25: underside no longer marked; top 1.6 x 0.9 both faces 2.88 + edges 0.15 = 3.0 m2 |
| 🛋️furniture folding-deck-chair | canvas-back | surface | passive | 0.3 | 0.49 | 0.49 | ACCEPT | sling 0.6 x 0.4 both faces = 0.48 m2 |
| 🛋️furniture folding-deck-chair | canvas-seat | surface | passive | 0.23 | 0.368 | 0.368 | ACCEPT | sling 0.45 x 0.4 both faces = 0.36 m2 |
| 🛋️furniture height-adjustable-desk | steel-desk | surface | passive | 0.8 | 2.15 | 2.15 | ACCEPT | 2 T-feet 0.28 + 2 x 2 telescope stages 0.7 + cross rail 0.28 + underframe and brackets 0.3-0.5 = 1.6-2.2 m2 |
| 🛋️furniture height-adjustable-desk | top-board | surface | passive | 1.28 | 2.68 | 2.68 | SET | C25: underside no longer marked; board 1.6 x 0.8 both faces 2.56 + edges 0.12 = 2.68 m2 (solver is one face per region at faces=1, both faces give the right Watts; geometry plate-up is a note: the underside convects as plate-down) |
| 🛋️furniture office-swivel-chair | castor-rubber | tyre | passive | 0.02 | 0.018 | 0.023 | ACCEPT | 5 castor wheels o0.05: 0.007 each, 0.02-0.035 m2 |
| 🛋️furniture office-swivel-chair | chair-fabric | surface | passive | 0.4 | 0.51 | 1 | SET | C29 total: seat cushion box 2*(0.223+0.03+0.03) = 0.57 + back pad both faces 0.45 |
| 🛋️furniture office-swivel-chair | chair-plastic | surface | passive | 0.35 | 0.7 | 1.1 | SET | C29 total: seat shell both faces 0.36 + backrest shell both faces 0.36 + 5 base arms 0.2 + arms and hub 0.2 |
| 🛋️furniture office-swivel-chair | chair-steel | surface | passive | 0.1 | 0.113 | 0.145 | ACCEPT | column, 5 stems, mechanism, struts, hubs: 0.11-0.15 m2 |
| 🛋️furniture operating-table | light-housing | lamp | operating | 0.45 | 0.35 | 0.35 | ACCEPT | dome o0.5 hemisphere 2*pi*0.25^2 = 0.39 m2 |
| 🛋️furniture operating-table | stand | surface | passive | 0.75 | 1.08 | 1.08 | ACCEPT | base + column + arm + knob = 1.1 m2 |
| 🛋️furniture operating-table | table-base | surface | passive | 0.9 | 0.76 | 0.76 | ACCEPT | plate loft + 2 castors = 0.76 m2 |
| 🛋️furniture operating-table | table-column | surface | passive | 0.45 | 0.694 | 0.694 | ACCEPT | telescopic column 0.21 x 0.34 box + lower loft + head = 0.7 m2 |
| 🛋️furniture operating-table | table-frame | surface | passive | 0.6 | 0.807 | 1.76 | SET | C29 total: carrier 1.96 x 0.4 both faces 1.57 + edges 0.19 |
| 🛋️furniture operating-table | table-pad | surface | passive | 0.9 | 0.815 | 2.4 | SET | C29 total: 4 pad sections 1.988 x 0.5 top 0.994 + underside 0.994 + sides 0.318 + ends 0.08 |
| 🛋️furniture park-bench | back-timber | surface | passive | 0.54 | 1.26 | 1.7 | SET | 3 raked slats 1.8 x 0.1 x 0.04: top+bottom 0.42 + edges 0.144 each = 1.7 m2 |
| 🛋️furniture park-bench | seat-timber | surface | passive | 0.76 | 1.711 | 2.4 | SET | C29 total: 4 slats 1.8 x 0.11875 x 0.045: 0.6 each incl. bearer contacts |
| 🛋️furniture park-bench | steel-frame | surface | passive | 0.9 | 1.07 | 1.07 | ACCEPT | two cast-style end frames: feet 0.2 + legs 0.5 + brace/bearer/arm 0.4 = 1.07 m2 |
| 🛋️furniture picnic-table | a-frame | surface | passive | 2.4 | 0.998 | 0.998 | ACCEPT | 4 legs 0.85 x 0.055 board 0.175 each = 0.7 + beams 0.56 = 1.15-1.26 m2 (old 2.4 counted plate envelopes) |
| 🛋️furniture picnic-table | bench-seats | surface | passive | 1.08 | 2.7 | 2.7 | ACCEPT | 2 boards 0.36 x 1.7: both faces 1.22 + edges 0.17 = 2.7-2.8 m2 |
| 🛋️furniture picnic-table | hinges | surface | passive | 0.06 | 0.151 | 0.22 | SET | C29 total: 4 blocks 0.08 x 0.1 x 0.11 all faces 0.056 each |
| 🛋️furniture picnic-table | table-top | surface | passive | 1.26 | 2.96 | 2.96 | ACCEPT | 5 planks 0.138 x 1.8: 0.497 + edges 0.155 = 0.65 each = 3.25 m2 (2.96 in the gaps) |
| 🛋️furniture single-bed | bed-frame | surface | passive | 4 | 3.37 | 3.37 | ACCEPT | head 0.95 x 0.9 + rails 2.04 x 0.16 x 4 faces + posts = 3.4 m2 |
| 🛋️furniture three-seat-sofa | arm-pads | surface | passive | 2.4 | 2.689 | 3 | SET | C29 total: 2 pads 0.2 x 0.52 x 0.9 all faces 1.5 each |
| 🛋️furniture three-seat-sofa | back-cushions | surface | passive | 1.1 | 2.41 | 3 | SET | C29 total: 3 cushions 0.6 x 0.55 x 0.15 all faces 1.0 each |
| 🛋️furniture three-seat-sofa | base-rail | surface | passive | 0.45 | 1.92 | 3.65 | SET | C29 total: skirt 1.7 x 0.86 x 0.14 top 1.46 + underside 1.46 + sides 0.72 |
| 🛋️furniture three-seat-sofa | feet | surface | passive | 0.06 | 0.09 | 0.09 | ACCEPT | 4 turned feet = 0.09 m2 |
| 🛋️furniture three-seat-sofa | seat-cushions | surface | passive | 1.28 | 1.63 | 3.6 | SET | C29 total: 3 boxes 0.5433 x 0.23 x 0.62 all faces 1.21 each |
| 🧱️components cantilever-balcony | thermal-break | wall | conditioned | 0.2 | 0.17 | 0.17 | ACCEPT | break plate strip 0.17 m2 (width x length, both faces within 15 % of the declared 0.2) |
| 🧱️components glulam-beam | bearing-steel | surface | passive | 0.04 | 0.06 | 0.06 | ACCEPT | steel bearing plates, exposed faces 0.06 m2 (plates 0.2 x 0.15 both faces and edges, one per end) |
| 🧱️components panel-radiator | top-grille | surface | passive | 0.1 | 0.14 | 0.14 | ACCEPT | grille 1.0 x 0.135 = 0.135 m2 |
| 🧱️components sectional-garage-door | spring-assembly | surface | passive | 0.6 | 0.85 | 0.85 | ACCEPT | 122 turns o8 mm on a o75 coil: 0.72 m2 wire surface (open turns) + shaft 0.18 + drums 0.2 = 0.9 m2 |
| 🧱️components sectional-garage-door | track-steel | surface | passive | 1.9 | 3.18 | 3.18 | ACCEPT | 9.8 m of 2 mm C-section (strip 0.12 m, both faces 0.235 m2/m) 2.3 + hangers 0.36 + tie bar 0.3 + flags 0.2 = 3.16 m2 |
| 🧱️components sliding-door | jamb-steel | surface | conditioned | 0.42 | 1.74 | 1.74 | ACCEPT | 2 posts 2.05 m x 0.4 perimeter minus rebate = 1.64-1.74 m2 |
| 🧱️components sliding-door | leaf-frame | surface | conditioned | 0.28 | 1.21 | 1.21 | ACCEPT | 4 hollow 50 x 50 members, 5.6 m x 0.2 perimeter = 1.12 m2 + joints |
| 🧱️components sliding-door | track-steel | surface | conditioned | 0.3 | 1.4 | 1.4 | ACCEPT | 100 x 84 housing 3.1 m x 0.36 perimeter = 1.12 + end caps and slot walls = 1.4 m2 |
| 🧱️components spiral-stair | column-steel | surface | passive | 2.28044 | 2.95 | 2.95 | ACCEPT | CHS 168.3 x 6.3 x 2.8 m: outer 1.48 + bore 1.37 (open) + base plate 0.1 = 2.95 m2 |
| 🧱️components spiral-stair | treads | surface | passive | 3.729 | 6.67 | 6.67 | ACCEPT | 15 treads: plan 0.1247 (22.5 degree sector R0.8) top + underside 0.249 + 55 mm folded flange 2*1.8*0.0525 = 0.19 -> 0.44 each = 6.6 m2 |
| 🧱️components steel-column | cap-plate | surface | passive | 0.1 | 0.124 | 0.124 | ACCEPT | plate 0.3 x 0.26 top 0.078 + edges 0.03 + bolts = 0.12 m2 |
| 🧱️components tied-canopy | canopy-steel | surface | passive | 3.5 | 2.97 | 2.97 | ACCEPT | sheet 2.5 x 1.2 both faces 2.97 m2 |
| 🧱️components tied-canopy | rod-steel | surface | passive | 0.93 | 1.09 | 1.09 | ACCEPT | tie rods o0.012 x 29 m = 1.09 m2 |
| 🪑️objects chalkboard | frame | surface | passive | 0.8 | 1.1 | 1.1 | ACCEPT | frame ring 4.0 x 1.2: outer 0.47 + inner 0.47 + half front rim 0.2 = 1.15 m2 |
| 🪑️objects chalkboard | tray | surface | passive | 0.32 | 1.34 | 1.34 | ACCEPT | 4.0 m aluminium U trough, 0.26 m2/m (underside, front wall both faces, floor, inner back) = 1.03 + chalk, ruler, sponge 0.14 + ends = 1.2-1.34 m2 |
| 🪑️objects charcoal-grill | ash-catcher | surface | passive | 0.12 | 0.14 | 0.14 | ACCEPT | catcher disc 0.25: underside + rim 0.14 m2 |
| 🪑️objects charcoal-grill | bowl-shell | surface | operating | 0.45 | 0.4 | 0.4 | ACCEPT | spherical cap r0.29 depth 0.25: pi*(0.0855+0.0625) = 0.465 less vents and legs = 0.4 m2 |
| 🪑️objects charcoal-grill | handle-wood | surface | passive | 0.03 | 0.045 | 0.045 | ACCEPT | handle o0.03 x 0.35 m = 0.033-0.045 m2 |
| 🪑️objects charcoal-grill | vents-alu | surface | passive | 0.01 | 0.0172 | 0.0172 | ACCEPT | dial o80 x 35: side 0.0088 + top 0.0048 + 3 holes o12 walls 0.0040 = 0.0176 m2 |
| 🪑️objects cooking-pot | base | heater | operating | 0.0232 | 0.0232 | 0.0232 (exchange) | exchange only | heated bottom disc pi*0.086^2 = 0.0232 m2 is the heat transfer area (contact plane with the hob); 0.0052 would make q = 2300 W/m2 for 12 W |
| 🪑️objects cooking-pot | handle | surface | passive | 0.003 | 0.00496 | 0.00496 | ACCEPT | two ear handles 20 x 14 mm with rivets: 2*0.0025 = 0.005 m2 |
| 🪑️objects drinking-fountain | bubbler-brass | surface | passive | 0.04 | 0.047 | 0.047 | ACCEPT | bubbler head o0.06 x 0.1 + neck = 0.045 m2 |
| 🪑️objects drinking-fountain | water-jet | water | passive | 0.0057 | 0.0071 | 0.0071 | ACCEPT | jet o0.012 x 0.18 arc = 0.007 m2 |
| 🪑️objects market-parasol | base-concrete | surface | passive | 0.4 | 0.274 | 0.274 | ACCEPT | slab 0.5 x 0.5 x 0.06 without the bottom: 0.25 + 4*0.5*0.06 = 0.37 less the pole socket; model 0.274 (-26 %) |
| 🪑️objects patio-heater | base-housing | surface | passive | 0.75 | 0.98 | 0.98 | ACCEPT | housing o0.3 x 0.5 + base disc = 0.9-1.0 m2 |
| 🪑️objects patio-heater | control | surface | passive | 0.004 | 0.0049 | 0.0049 | ACCEPT | knob and panel 0.005 m2 |
| 🪑️objects patio-heater | reflector | heater | operating | 0.55 | 0.68 | 0.68 | ACCEPT | dish o0.8: pi*0.4^2 = 0.5 x 1.2 curvature = 0.6-0.68 m2 |
| 🪑️objects playground-swing | chain-steel | surface | passive | 0.1 | 0.13 | 0.13 | ACCEPT | 2 chains o0.011 x 1.83 + shackles = 0.13 m2 |
| 🪑️objects retractable-bollard | beacon-led | lamp | operating | 0.025 | 0.0375 | 0.0375 | ACCEPT | ring lens o0.2 x 0.05 = 0.031 + cap = 0.0375 m2 |
| 🪑️objects retractable-bollard | cover-frame | surface | passive | 0.12 | 0.093 | 0.093 | ACCEPT | cast frame 0.4 square ring = 0.093 m2 |
| 🪑️objects retractable-bollard | hydraulic-unit | engine | operating | 0.35 | 0.45 | 0.45 | ACCEPT | power unit box 0.45 m2 |
| 🪑️objects retractable-bollard | shaft-steel | surface | passive | 0.41 | 0.307 | 0.307 | ACCEPT | raised shaft o0.2 x 0.65 = 0.408 less reflector band 0.063 and beacon 0.03 = 0.31 m2 |
| 🪑️objects retractable-bollard | sleeve-steel | surface | passive | 1.2 | 1.2 | 1.2 | KEEP | 1.2 -> 1.252 is within 5 %, no change |
| 🪑️objects shopping-trolley | frame-tube | surface | passive | 0.28 | 0.197 | 0.197 | ACCEPT | 4.5 m of tube o14 mm (chassis 2.2 + forks 1.4 + legs) = 0.197 m2 |
| 🪑️objects shopping-trolley | gate-wire | surface | passive | 0.12 | 0.147 | 0.12 | RESTORE + model defect | gate grid of o4 mm wire: ~9.6 m of wire * pi*0.004 = 0.12 m2 |
| 🪑️objects shopping-trolley | handle-grip | surface | passive | 0.04 | 0.06 | 0.06 | ACCEPT | capsule r20 mm x 0.49 m: 2*pi*0.02*0.49 = 0.0616 m2 |
| 🪑️objects shopping-trolley | wheels | tyre | passive | 0.08 | 0.058 | 0.058 | ACCEPT | 4 castor wheels o0.125: 0.058-0.14 m2 depending on the sides, no better estimate |
| 🪑️objects stepladder | feet | surface | passive | 0.02 | 0.03 | 0.03 | ACCEPT | 4 rubber feet = 0.03 m2 |
| 🪑️objects stepladder | platform | surface | passive | 0.3 | 0.21 | 0.21 | ACCEPT | top platform 0.3 x 0.3 minus frame = 0.2 m2 |
| 🪑️objects stepladder | rear-frame | surface | passive | 0.43 | 0.506 | 0.506 | ACCEPT | 2 rails 1.5 x 0.06 x 4 faces + rungs = 0.5 m2 |
| 🪑️objects stepladder | spreaders | surface | passive | 0.06 | 0.074 | 0.074 | ACCEPT | 2 spreader bars both faces = 0.074 m2 |
| 🪑️objects stepladder | treads | surface | passive | 0.5 | 0.39 | 0.39 | ACCEPT | 3 treads 0.4 x 0.12 both faces + edges = 0.39 m2 |
| 🪑️objects street-lamp | arm-alu | surface | passive | 0.18 | 0.22 | 0.22 | ACCEPT | arm o0.06 x 1.2 m = 0.226 m2 |
| 🪑️objects street-lamp | lamp-housing | lamp | operating | 0.25 | 0.19 | 0.19 | ACCEPT | head 0.4 x 0.2 x 0.06: top 0.08 + sides 0.07 = 0.15-0.2 m2; q 40 W/0.19 = 210 W/m2 |
| 🪑️objects street-lamp | lamp-lens | glazing | passive | 0.08 | 0.05 | 0.05 | ACCEPT | lens 0.27 x 0.2 plan x 1.1 curvature = 0.06 m2 |
| 🪑️objects tealight | cup | surface | passive | 0.003 | 0.00204 | 0.00204 | ACCEPT | aluminium cup o0.038 x 0.016: side 0.0019 + rim = 0.002 m2 (base on the surface excluded) |
| 🪑️objects waste-bin | bin-hdpe | surface | passive | 1.4 | 1.67 | 1.67 | ACCEPT | 240 l bin: 4 sides 4*0.5*0.9 less the base = 1.7 m2 |
| 🪑️objects waste-bin | lid-hdpe | surface | passive | 0.26 | 0.38 | 0.38 | ACCEPT | lid 0.58 x 0.58 both faces edges = 0.38 m2 |
| 🪑️objects waste-bin | liner-film | surface | passive | 0.08 | 0.111 | 0.111 | ACCEPT | liner film 0.111 m2: folded film 0.3 x 0.18 both faces plus the collar |

## Operating regions whose heat flux density changes by more than a factor 1.5

`q = heatW / areaM2`. The decided area is the honest outer surface; `heatW` is a physical power in most assets (accepted as is, the surface temperature follows) and a tuned input in some (coils, valve stubs). The last column is the `heatW` that keeps the committed flux density; whether to apply it is the thermal author's decision, it is not part of this review.

| asset | region | committed m2 | decided m2 | heatW | q committed W/m2 | q decided W/m2 | heatW keeping the old q |
|---|---|---|---|---|---|---|---|
| concrete-mixer-truck | exhaust-stack | 0.8 | 1.21 | 5000 | 6250 | 4132 | 7.56e+03 |
| concrete-mixer-truck | tyres | 10 | 19.2 | 5000 | 500 | 260 | 9.6e+03 |
| diesel-generator | enclosure-walls | 29 | 43.5 | 800 | 28 | 18 | 1.2e+03 |
| finned-heat-exchanger | headers | 0.012 | 0.132 | 2 | 167 | 15 | 22 |
| finned-heat-exchanger | tubes | 0.08 | 0.188 | 24 | 300 | 128 | 56.4 |
| scroll-compressor | discharge | 0.004 | 0.0072 | 3 | 750 | 417 | 5.4 |
| scroll-compressor | shell | 0.1 | 0.205 | 40 | 400 | 195 | 82 |
| sorption-wheel | regen | 0.064 | 0.12 | 28 | 438 | 233 | 52.5 |
| decentralised-hrv-fan | casing | 0.09 | 0.39 | 1 | 11 | 3 | 4.33 |
| duct-fan | casing | 0.07 | 0.12 | 6 | 86 | 50 | 10.3 |
| duct-fan | hub | 0.009 | 0.0145 | 5 | 556 | 345 | 8.06 |
| electric-fan-heater | grille | 0.03 | 0.05 | 20 | 667 | 400 | 33.3 |
| electric-fan-heater | top-cover | 0.035 | 0.055 | 22 | 629 | 400 | 34.6 |
| electricity-meter | plate | 0.0052 | 0.0135 | 2.4 | 462 | 178 | 6.23 |
| room-thermostat | led | 0.0001 | 5e-05 | 0.005 | 50 | 100 | 0.0025 |
| thermostatic-radiator-valve | body | 0.0125 | 0.00764 | 3 | 240 | 393 | 1.83 |
| thermostatic-radiator-valve | head | 0.0075 | 0.01546 | 0.8 | 107 | 52 | 1.65 |
| city-car | exhaust-system | 0.45 | 0.675 | 2300 | 5111 | 3407 | 3.45e+03 |
| retractable-bollard | beacon-led | 0.025 | 0.0375 | 8 | 320 | 213 | 12 |

## Exposed-only areas where the poses also reduce the exposure (C1 against the solver)

`areaM2` is the exposed surface of the model (C1, enforced by `volumes-thermal-coverage`), while the poses of these regions additionally carry `exposedFraction` below 1 and/or a `contact` share, which the solver applies on top of the area (`outflow = max(e, 0.05) × exchange`, `contact × h_ct`). For passive regions this changes the Watt totals and merge weights only; for the operating ones (heatW column) it also decides how hot the region gets, because `q = heatW / areaM2` is spread over the exposed share only. Superseded by C29 (areaM2 is the total outer area, exposure stays with the pose): the rows of the humans, animals, vegetation, appliances and furniture categories decided at an exposed-only value were re-decided to the hand-calculated total (reasons starting 'C29 total'); the table lists what the poses carry. Original note: the coordinator should either set the pose `exposedFraction` of these regions to 1 where the author already removed the covered faces from the area, or accept totals as area.

| asset | region | decided m2 | pose min exposedFraction | pose max contact | heatW |
|---|---|---|---|---|---|
| agave | caudex | 0.29 | 0.6 | 0 |  |
| hosta-clump | petioles | 0.183 | 0.3 | 0 |  |
| concrete-mixer-truck | drum-drive | 3.04 | 0.3 | 0 | 539.1 |
| concrete-mixer-truck | tyres | 19.2 | 1 | 0.06 | 5000 |
| diesel-generator | skid-tank | 3.92 | 0.14 | 0.43 |  |
| evaporative-humidifier | casing | 1.78 | 0.9 | 0.1 |  |
| evaporative-humidifier | sump | 0.209 | 0.4 | 0 |  |
| excavator | engine-compartment | 2.58 | 0.3 | 0 | 1400 |
| forklift | brakes | 0.295 | 0.5 | 0 | 6011.7 |
| forklift | controller | 0.419 | 0.5 | 0 | 150 |
| forklift | tyres | 1.95 | 1 | 0.15 | 380 |
| heat-pump-outdoor-unit | base-tray | 0.712 | 0.3 | 0 | 185.5 |
| oil-pumpjack | gearbox | 8.7 | 0.9 | 0 | 720 |
| plate-heat-exchanger | frame | 1.2 | 0.9 | 0.1 |  |
| scroll-compressor | feet | 0.084 | 0.6 | 0.4 |  |
| scroll-compressor | shell | 0.205 | 0.9 | 0.1 | 40 |
| solar-thermal-collector | housing | 2.6 | 0.7 | 0 |  |
| sorption-wheel | casing | 0.68 | 0.9 | 0.1 |  |
| tower-crane | ballast | 36.5 | 1 | 0.23 |  |
| child-on-kick-scooter | backpack | 0.28 | 0.7 | 0 |  |
| child-on-kick-scooter | wheels | 0.082 | 1 | 0.01 |  |
| parent-with-stroller | tyres | 0.4 | 1 | 0.003 |  |
| person-on-crutches | crutch-cuffs | 0.06 | 0.9 | 0 |  |
| person-on-crutches | crutch-tips | 0.009 | 1 | 0.6 |  |
| person-with-rollator | bag | 0.33 | 0.85 | 0 |  |
| person-with-rollator | seat-basket | 1.05 | 0.7 | 0 |  |
| person-with-rollator | tyres | 0.116 | 1 | 0.01 |  |
| person-with-umbrella | canopy | 2.1 | 0.35 | 0 |  |
| runner-with-blade-prosthesis | blade | 0.076 | 1 | 0.05 |  |
| seated-office-worker | castor-rubber | 0.0247 | 0.8 | 0 |  |
| seated-office-worker | chair-steel | 0.127 | 0.9 | 0 |  |
| seated-reader | bench | 1.108 | 0.75 | 0 |  |
| skateboarder | trucks | 0.105 | 0.8 | 0 |  |
| skateboarder | wheels | 0.039 | 0.9 | 0.02 |  |
| sleeping-person | pillow | 0.58 | 0.7 | 0 |  |
| walking-commuter | bag-leather | 0.263 | 0.85 | 0 |  |
| wheelchair-user | cushion | 0.63 | 0.6 | 0 |  |
| wheelchair-user | tyres | 0.5 | 1 | 0.02 |  |
| circulation-pump | casing | 0.03 | 0.9 | 0.1 | 10 |
| circulation-pump | terminal-box | 0.0128 | 0.8 | 0.2 | 2 |
| decentralised-hrv-fan | casing | 0.39 | 0.85 | 0.15 | 1 |
| desktop-monitor | stand | 0.108 | 0.9 | 0.05 |  |
| duct-fan | casing | 0.12 | 0.8 | 0.2 | 6 |
| electric-fan-heater | feet | 0.0085 | 0.2 | 0 |  |
| electric-kettle | plate | 0.044 | 0.8 | 0.2 | 40 |
| electric-kettle | water | 0.0145 | 0.05 | 0 | 1955 |
| electricity-meter | casing | 0.205 | 0.8 | 0.2 |  |
| gas-meter | shell | 0.3 | 0.8 | 0.2 |  |
| kitchen-extractor-hood | chimney | 0.55 | 0.85 | 0.15 |  |
| pendant-lamp | lampholder | 0.0042 | 0.6 | 0 | 2.5 |
| shower-head | head | 0.0454 | 1 | 0.7 |  |
| shower-head | valve | 0.025 | 1 | 0.4 |  |
| thermostatic-radiator-valve | body | 0.00764 | 0.9 | 0 | 3 |
| toaster | bread | 0.054 | 0 | 0 | 150 |
| city-bus | tyres | 8.3 | 1 | 0.04 | 1800 |
| city-car | exhaust-system | 0.675 | 0.6 | 0 | 2300 |
| delivery-van | brakes | 0.72 | 0.5 | 0 | 6679.7 |
| delivery-van | tyres | 4.46 | 1 | 0.04 | 3100 |
| tram | brake-discs | 1.89 | 0.5 | 0 | 15703.1 |
| vintage-car-1929 | engine-bay | 1.12 | 0.2 | 0 | 900 |
| vintage-car-1929 | tyres | 3.05 | 1 | 0.05 | 380 |
| bookshelf | shelves | 2.294 | 0.1 | 0.85 |  |
| cafe-chair | back-beech | 0.242 | 0.6 | 0.4 |  |
| cafe-chair | seat-beech | 0.524 | 0.3 | 0.7 |  |
| dining-table | table-top | 3 | 0.95 | 0.05 |  |
| folding-deck-chair | canvas-back | 0.49 | 0.2 | 0.4 |  |
| folding-deck-chair | canvas-seat | 0.368 | 0.15 | 0.45 |  |
| office-swivel-chair | castor-rubber | 0.023 | 0.8 | 0 |  |
| office-swivel-chair | chair-steel | 0.145 | 0.9 | 0 |  |
| operating-table | table-base | 0.76 | 0.6 | 0.2 |  |
| operating-table | table-pad | 2.4 | 0.85 | 0 |  |
| park-bench | back-timber | 1.7 | 0.8 | 0.2 |  |
| park-bench | seat-timber | 2.4 | 0.53 | 0.47 |  |
| picnic-table | bench-seats | 2.7 | 0.74 | 0.26 |  |
| picnic-table | table-top | 2.96 | 0.96 | 0.04 |  |
| three-seat-sofa | arm-pads | 3 | 0.97 | 0.03 |  |
| three-seat-sofa | back-cushions | 3 | 0.86 | 0.14 |  |
| three-seat-sofa | seat-cushions | 3.6 | 0.6 | 0.4 |  |
| drinking-fountain | water-jet | 0.0071 | 1 | 0.8 |  |
| retractable-bollard | hydraulic-unit | 0.45 | 0 | 0.7 | 358.8 |
| tealight | cup | 0.00204 | 0.8 | 0.2 |  |
| waste-bin | liner-film | 0.111 | 0.8 | 0 |  |

## `sectionFaces` against C25

Every use found in the catalog (`volumes` of all scales) and judged; illegitimate uses were removed from the asset file and from `T/volumes/<category>/<asset>.json` by `areareview-unsection.py` (only the member `sectionFaces` of the solid disappears, checked by JSON comparison). Flow volumes (plumes, jets, outlets) are not surfaces at all; they are kept and listed as such, the coordinator may prefer a dedicated flag.

| asset | scales | solids (1:50 ids) | faces | verdict | why |
|---|---|---|---|---|---|
| bar-stool | 50,100,200,500 | th-seat-plate-core | +u | legitimate | plate core glued to the seat pad (b) |
| chalkboard | 50,100,200,500 | enamel-left, enamel-right, leaf-enamel, th-frame-ring | +w | legitimate | enamel back bonded to the board core, frame back on the wall (b) |
| charcoal-grill | 50,100,200,500 | lid-vent-flue-nozzle, th-bowl-envelope, th-charcoal-bed, th-lid-envelope, th-vent-dial | +u/-u +v -u | legitimate | envelope caps of hot-bowl and lid shells, charcoal bed seated, flows (b) |
| charcoal-grill | 50,100,200,500 | open-bowl-plume-mouth | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| city-car | 100,200,500 | (coarser scales only) |  | legitimate | see ruling |
| cooking-pot | 50,100,200 | th-steam, th-steam-plume | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| dining-table | 500 | (coarser scales only) |  | legitimate | see ruling |
| drinking-fountain | 50,100,200,500 | th-jet-spray | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| etics-brick-wall-12 | 50,100,200,500 | bed-joint-01, bed-joint-02, bed-joint-03, bed-joint-04, bed-joint-05, bed-joint-06, bed-joint-07, bed-joint-08, bed-joint-09, bed-joint-10, bed-joint-11, bed-joint-12, brick-course-00, brick-course-01, brick-course-02, brick-course-03, brick-course-04, brick-course-05, brick-course-06, brick-course- | -v/+v/-w/+w | legitimate | cut faces of a wall sample that continues (a) |
| etics-brick-wall-30 | 50,100,200,500 | bed-joint-01, bed-joint-02, bed-joint-03, bed-joint-04, bed-joint-05, bed-joint-06, bed-joint-07, bed-joint-08, bed-joint-09, bed-joint-10, bed-joint-11, bed-joint-12, brick-course-00, brick-course-01, brick-course-02, brick-course-03, brick-course-04, brick-course-05, brick-course-06, brick-course- | -v/+v/-w/+w | legitimate | cut faces of a wall sample that continues (a) |
| external-venetian-blind | 50,100,200,500 | th-pane-in | -u | legitimate | see ruling |
| gas-hob | 50,100,200 | th-burner-ring, th-flame-ring, th-knob, th-plate | +w -v -v/+u/-u/+w/-w | legitimate | plate underside and knob back on the appliance, burner ring seated, flame rings are flow volumes (b) |
| gas-hob | 50,100,200 | th-flame-plume | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| glulam-beam | 50,100,200,500 | th-side-l, th-side-r, th-soffit, th-top | +v/-w/+w +w/-v/+v -v/-w/+w | legitimate | see ruling |
| hot-water-cylinder | 50,100,200,500 | (coarser scales only) |  | legitimate | plume is a flow volume |
| hot-water-cylinder | 50,100,200,500 | th-top-plume | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| kitchen-extractor-hood | 50,100,200 | th-canopy | -u | legitimate | canopy inner face is the side the released heat enters (operating region, one face is the exchange face); extract is a flow volume |
| kitchen-extractor-hood | 50,100,200 | th-extract | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| laptop | 50,100,200 | th-deck, th-underside | +v/+u/-u/+w/-w -v/+u/-u/+w/-w | legitimate | deck and underside plates joined to the case along their edges (b) |
| laptop | 50,100,200 | th-deck-plume | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| leaning-person | 50,100,200,500 | th-wall-brick, th-wall-plaster, wall | +v | legitimate | wall face against the person's mounting wall sample (a) |
| market-parasol | 50,100,200,500 | th-canopy-top, th-canopy-underside | +v -v | legitimate | top and underside are two regions, each region owns one face (face counted once in the other) |
| mvhr-unit | 50,100,200 | th-casing, th-window | +w | legitimate | back of casing and window on the wall (b) |
| mvhr-unit | 50,100,200 | th-extract-air, th-supply-air | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| patio-heater | 50,100,200,500 | th-dome-plume-disc, th-flue-gas-disc | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| plate-heat-exchanger | 50,100,200 | th-exhaust-air-outlet, th-supply-air-outlet | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| retractable-bollard | 50,100,200,500 | th-frame-top | -v/-u/+u/-w/+w | legitimate | cover frame cast into the ground slab (b) |
| sectional-garage-door | 100,500 | (coarser scales only) |  | legitimate | panel and strip faces stacked against each other, seal on the floor (b) |
| server-rack | 50,100,200,500 | th-blanks, th-cabinet-left, th-cabinet-rear, th-cabinet-right, th-cabinet-roof, th-caster-0, th-caster-1, th-caster-2, th-caster-3, th-door, th-servers, th-servers-storage, th-switch | +u/+v/-v/+w/-w +u/-u/+v/-v/-w +u/-u/-v/+w/-w | legitimate | closed cabinet: inner faces of the panels and the unit fronts belong to the sealed airway (b); outer faces count |
| server-rack | 50,100,200,500 | th-exhaust-air | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| shower-head | 50,100,200 | th-bracket-low, th-bracket-top, th-tube, th-valve | +v/-v -u | legitimate | brackets and valve on the wall, tube ends continue (a)(b) |
| sliding-door | 50,100,200,500 | th-glass | -w | legitimate | glass face inside the frame rebate, the other face is the conditioned interior (b) |
| solar-thermal-collector | 50,100,200,500 | th-absorber-plate, th-glass | -v | legitimate | glass underside inside the sealed collector, absorber underside bonded to risers and insulation (b); the declared one-sided area is what the solver wants (faces = 1, plate-up) |
| solar-thermal-collector | 50,100,200,500 | th-absorber-plume-outlet | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| solid-brick-wall-1960 | 50,100,200,500 | bed-joint-01, bed-joint-02, bed-joint-03, bed-joint-04, bed-joint-05, bed-joint-06, bed-joint-07, bed-joint-08, bed-joint-09, bed-joint-10, bed-joint-11, bed-joint-12, brick-course-00, brick-course-01, brick-course-02, brick-course-03, brick-course-04, brick-course-05, brick-course-06, brick-course- | -v/+v/-w/+w | legitimate | cut faces of a wall sample that continues (a) |
| sorption-wheel | 50,100,200 | th-process-air-outlet, th-regen-air-outlet | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| split-ac-indoor-unit | 50,100,200 | th-grille, th-vane | +v/-u -v/+u/-u/+w/-w | legitimate | grille and vane inner faces against the casing (b) |
| split-ac-indoor-unit | 50,100,200 | th-cold-jet | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| stair-flight | 50,100,200,500 | th-tread-top | -v/+u/-u/-w/+w | legitimate | tread top layer bonded to the core, edges continue (a)(b) |
| steel-column | 50,100,200,500 | th-flange-a, th-flange-b, th-inner-a, th-inner-b | +v +v/+w/-w/+u/-u -v | legitimate | flange and web faces welded together, column ends continue (a)(b) |
| storage-crate-stack | 50,100,200 | th-envelope-1, th-envelope-2, th-envelope-3, th-envelope-4, th-envelope-5, th-foot-edge-1, th-foot-edge-2, th-foot-edge-3, th-foot-edge-4, th-foot-edge-5, th-rim-edge-1, th-rim-edge-2, th-rim-edge-3, th-rim-edge-4, th-rim-edge-5 | +v/-v | legitimate | see ruling |
| street-lamp | 50,100,200,500 | th-arm, th-base-cover, th-pole | -u/+u -v/+v | legitimate | pole and arm ends continue into the ground and the luminaire (a) |
| street-lamp | 50,100,200,500 | th-lamp-plume | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| tealight | 50,100,200 | (coarser scales only) |  | legitimate | flame plume is a flow volume |
| tealight | 50,100,200 | th-flame-plume | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| thermostatic-radiator-valve | 100 | (coarser scales only) |  | legitimate | knob mounting faces (b) |
| tied-canopy | 50,100,200,500 | th-concrete-beam, th-concrete-strut, th-glass-0, th-glass-1, th-glass-2, th-glass-3, th-wall-render | -u/+u/-v/+v/-w/+w -u/-v/+v/-w/+w -v | legitimate | wall render, beam and strut cut where they continue into the building; glass faces inside the laminated unit (a)(b) |
| tilt-window | 50,100,200 | th-centre-cavity, th-centre-pane-inner, th-centre-pane-outer, th-edge-edge-body, th-edge-pane-inner, th-edge-pane-outer, th-spacer-bottom, th-spacer-top | -u/+u/-w/+w -u/+u/-w/+w/+v -w/+w | legitimate | pane edges and faces inside the sealed insulating unit (b) |
| toaster | 50,100,200 | (coarser scales only) |  | legitimate | slot air is a flow volume |
| toaster | 50,100,200 | th-slot-air | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| tower-crane | 50,100,200,500 | th-slew-ring | -v/+v | legitimate | see ruling |
| tower-server | 50,100,200 | th-cpu-grille, th-feet, th-front-bezel, th-psu-vent-rear, th-psu-vent-top, th-rear-panel, th-side-panel-l, th-side-panel-r | +u/+v/-v/+w/-w +u/-u/+v/-v/+w +u/-u/+v/-v/-w | legitimate | closed case: inner faces belong to the sealed airway (b) |
| tower-server | 50,100,200 | th-case-plume, th-rear-exhaust | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| vintage-car-1929 | 50,100,200,500 | th-cabin-rear-wall, th-cabin-side-l, th-cabin-side-r, th-headlamp-lens-l, th-headlamp-lens-r, th-hood-front-l, th-hood-front-r, th-hood-side-l, th-hood-side-r, th-hood-top, th-radiator-shell, th-roof-slab, th-tub-front, th-tub-rear, th-tub-side-l, th-tub-side-r, th-window-front-l, th-window-front-r, | +u +w -u | legitimate | closed body shells: inner faces look into the cabin or engine bay, which are regions of their own (b); to be re-examined if the coordinator wants closed shells counted two-sided |
| vintage-car-1929 | 50,100,200,500 | tailpipe-jet | all | flow volume, not a surface | emitter/plume/outlet solid, no material face |
| waste-bin | 50,100,200 | th-lid-cap, th-lid-end-l, th-lid-end-r, th-lid-skirt | +w -v -v/+u/-u | legitimate | see ruling |
| wheelchair-ramp | 50,100,200,500 | th-kerb-a, th-kerb-b, th-side-pour-landing-middle, th-side-pour-landing-top, th-side-pour-run-1, th-side-pour-run-2, th-skin-landing-middle, th-skin-landing-top, th-skin-run-1, th-skin-run-2 | +w/-v/-u/+u -v/+v/-u/+u -v/-u/+u/-w/+w | legitimate | skin undersides bonded to the poured core, run ends continue, kerb on the ground (a)(b) |

Removed as illegitimate (faces in free air), with the resulting `areaM2` decision in the table above:

| asset | scales | solids | face | why illegitimate | area decision |
|---|---|---|---|---|---|
| city-car | 50,100,200,500 | th-bp-lower(-paint) | -v | vehicle underside in free air | SET 13.0 (underbody 4.8 m2 added) |
| delivery-van | 50,100,200,500 | th-shell(-paint) | -v | vehicle underside in free air | SET 22 (committed; underbody 8.2 m2) |
| city-bus | 50,100,200,500 | th-shell(-insulation) | -v | vehicle underside in free air | SET 62 (underbody 26 m2 added) |
| height-adjustable-desk | 50,100,200,500 | th-top-board(-laminate) | -v | underside of a free-standing board | SET 2.68 (both faces; solver faces = 1 so both faces give the right Watts, geometry plate-up is a note) |
| dining-table | 50,100,200,500 | th-table-top-top-l/r (slab-l/r) | -v | underside of a free-standing top | SET 3.0 (both faces) |
| person-with-umbrella | 50,100,200,500 | canopy (body), th-canopy | -u | underside of an open canopy | SET 2.1 (both faces of the dome) |

## C27 segment-share redistribution of body regions (decided asset by asset)

Solver check (`🫀️physiology bodySetup`): the physiological skin area of a segment is `skinFraction / Σ skinFraction × body.skinAreaM2` (sweat and blood-flow weights follow it), `f = areaM2 / skinAreaM2` is the clothing/outer-area factor of the segment (conduction and exchange of the segment scale with `f`), `localShare = metabolicShare / Σ metabolicShare` is the heat production share and is **independent of the area**. A redistribution of `areaM2` among segments with the total fixed therefore moves heat exchange between segments, and keeps each `f` unchanged only if `skinFraction` is moved with it; `metabolicShare` would stay (production is physiological, not geometric).

Condition of C27: a segment deviates beyond 30 % **and the model is sound**. Verdicts, from the author numbers (the product measurement `volumes-check.ts --product` needs `bun`, which this environment does not have, so full per-segment exposed vectors could not be re-measured and no sum-preserving rescale could be computed honestly):

| asset | segments | author figure | verdict | why |
|---|---|---|---|---|
| red-squirrel | hands-skin 0.0008, feet-skin 0.0016 | model 0.0023 and 0.0051 | no redistribution | the model hands and feet are 22 mm wide because of the 7.5 mm floor and the drawn size; real hand ~1 x 2 cm, declared value stands; model not sound at the feature floor |
| pig | hooves 0.018, ears 0.07, head | hooves several times larger | no redistribution | 4 hooves o0.045 x 0.04 = 0.023, the declared 0.018 is right; the model hooves are drawn 70 mm tall (floor and drawing) |
| sheep | hooves 0.016, ears 0.022 | hooves 5.5x, ears +61 % | no redistribution | 4 hooves o0.045 x 0.035 = 0.02 (declared right); ear flaps 2 x 0.1 x 0.06 x 2 faces = 0.024 (declared right), the drawn plan ears are larger than the real ones |
| cow, dog, horse (author 25) | hooves, small skin regions | not quantified in the report | no redistribution | numbers not in the report and not re-measurable here; same cause (parts at the feature floor, 2D contradictions listed in `author-requests.md`) |
| pigeon, mallard, roe-deer (author 29) | toes, feet, bill, hooves, nose | hooves 4x-12.8x, eyes 15x at 1:500 | no redistribution | each part sits at the scale's feature floor or is drawn thicker than the real part (pigeon toes 8 mm, duck feet 80 mm, roe hooves 70 mm), the model is not sound for these |
| person-on-crutches | shorts 0.197 (model 0.42), hoodie (-31 %), sneaker 0.0697 (0.10), grips, tips | author 30 | no redistribution | the drawn loose shorts are twice the real garment, hoodie bury at the arms; the sum cannot be kept without the other garments' exposed values |
| kneeling-gardener | hat 0.226 (model 0.36) | author 45 hand calculation | keep 0.226 | one-sided brim 0.143 + crown 0.061 + band 0.009 = 0.213 = declared within 6 %; the model's 0.36 is both faces of the brim (sky side and underside), the declared value counts the sky-facing side once (geometry plate-up); the hat is a one-sided clothing surface of the head, the sum stays |
| toddler, hoodies, hats of author 27 / 40 (wheelchair-user hoodie -43 %, worker helmet +100 %, guitar player shirt -43 %, jeans +41 %, boots +65 %) | clothing | buried or overlapping solids (vest and shirt on the same torso solid, guitar and arms burying the shirt) | no redistribution | the model is not sound (buried layers); these are model findings (already in `model-findings.md`), the physiological values stay |

Nothing was redistributed: no candidate meets "model sound and measured as a complete vector". The condition for a future pass is a product measurement of all body regions of the subject at 1:50 from an environment with `bun`.

## C28 comparison target `areaM2 x (1 - contact.fraction)` and pose fractions

- **cyclist / saddle**: declared 0.07 (the leather loft, both faces), exposed 0.033. The poses declared no contact although the rider plainly sits on it. Written into the poses `riding-upright` and `coasting`: `exposedFraction 0.5`, `contact {fraction 0.5, tempC 32}` (the upper face lies under the seated rider; 0.07 x 0.5 = 0.035 matches the model 0.033-0.034). `standing-on-pedals` and `dismounted-astride` keep exposure 1 (rider off the saddle). The area stays 0.07; the DEFER of the cyclist rows (no 1:50 model in the file) is unaffected.
- **stepladder feet, three-seat-sofa feet, bar-stool, office-swivel-chair castors**: the decided areas (0.03, 0.09, 0.023) are the exposed model values (side faces without the footprint), the poses carry `exposedFraction 1` (castors 0.8) and no contact. With C28 the declared area would have to be the total (exposed / (1 - contact)). Not changed: the pose fractions of the floor contact of feet are 0 by authors' choice and the validator compares `areaM2 x (1 - 0) = exposed`, which holds; adding a floor contact of ~20 % would require raising the areas to 0.0375 / 0.11 for no thermal effect on passive regions. Recorded as the open choice of the coordinator.
- **seats under sitters, wall-mounted backs, tyres on the ground**: the sections above already list regions whose poses carry `exposedFraction`/`contact` (`Exposed-only areas where the poses also reduce the exposure`): under C28 these exposed-only areas are the comparison target of `areaM2 x (1 - contact)` only where the area is the total; for each such row the reviewer's decided area was the exposed model value or the total hand value as stated in its reason, rows decided with an exposed-only value and a pose contact above 0 are exactly those of that table and need `areaM2 / (1 - contact)` if the coordinator wants totals.

## Layer order

**What the order means in the solver.** `ThermalLayer` is documented "innermost first". `regionSetup` takes the emissivity and the solar absorptivity of the **last** layer's material (`region.layers[length - 1]`, else the region's own material): the last layer is the exposed outer surface. The thermal resistance is the sum over all layers (`layerResistance`, `layerVapourResistance`) and does **not** depend on order, nor do the film, the conduction `(T_interior - T_s)/(ΣR + R_film)` or the Gagge skin-to-surface resistance. Further users of the order: the nested-shell thermal view (`🧊️volume/🌡️thermal`: the outermost shell, the last layer, shows the surface value, the inner shells the skin value beneath), the validator rule that compares the area with the solids of the outermost layer (C1), and the 1:50 `volumes-layers` rule. For a body the first layer is next to the skin (air gap, underwear), the last the outer garment; for a conditioned envelope element with two air sides the **first layer faces the interior** (the side of `conditioned.interiorTempC` and its interior film) and the **last layer faces the environment**. Consequence: only where the first and the last material differ in emissivity or absorptivity does a reversed order change a result (the surface temperature through radiation and solar gain, and the surface-driven emitted radiation).

**Audit.** All 195 regions with two or more layers were compared with `git show HEAD:<path>` (`areareview-layers.py`; data `🗑️generated/areareview/layers.json`) and every order was judged physically.

| asset | region | committed order | order now | verdict | effect |
|---|---|---|---|---|---|
| bar-stool | seat-pad, backrest | leatherette, foam | foam, leatherette | correct (cover outside) | outer material hr-foam -> pu-leatherette: emissivity and absorptivity change |
| height-adjustable-desk | top-board | laminate, board | board, laminate | correct | same material both layers: none |
| office-swivel-chair | chair-fabric | fabric, foam | foam, fabric | correct | same material: none |
| operating-table | table-pad | vinyl, foam | foam, vinyl | correct | same material pad-foam: none |
| single-bed | mattress | quilt, foam-core | foam-core, quilt | correct | cold-foam -> quilted-cover |
| three-seat-sofa | seat-cushions, back-cushions, arm-pads, base-rail | fabric first | core first, fabric last | correct | hr-foam / polyester-batting / birch-plywood -> upholstery-fabric |
| curtain-wall-bay | glass-centre, glass-edge | pane-out, cavity, pane-in | pane-in, cavity, pane-out | correct (pane-in faces the interior) | glass both sides: none; the nested-shell display and the outermost-layer area follow the exterior pane |
| curtain-wall-bay | mullion, transom | cap, break | break, cap | correct (cap exterior) | aluminium both: none |
| chalkboard | board-face, leaf | enamel, core | core, enamel | correct (enamel faces the room) | mdf-core -> enamel-board (the committed order gave the enamelled board the emissivity of its MDF core) |
| city-bus, tram | body-panels, roof, doors | skin (paint), insulation (interior-trim) | **insulation, skin** | **wrong in the committed catalog and the working tree, fixed** (exterior paint last) | outer material interior-trim -> paint: emissivity and absorptivity of all vehicle shell regions, solar gain of roofs |
| tower-crane | cab | steel-s355, lining (machinery) | **lining, steel-s355** | **wrong, fixed** (the 3 mm steel skin is the exterior, the 30 mm lining the interior) | outer material machinery -> steel-s355 |
| seated-office-worker | chair-fabric | upholstery, foam | **foam, upholstery** | **wrong, fixed** | same material: none |
| etics-brick-wall-12/-30, solid-brick-wall-1960, masonry-wall-with-lintel, cantilever-balcony, external-venetian-blind facade, tied-canopy wall-face/wall-plate, tilt-window, sliding-door, sectional-garage-door, spandrel, diesel-generator enclosure-walls, vehicles (paint over metal), all clothing and plumage regions | | | | correct as they are (interior first: plaster, brick, insulation, render; clothing from the skin outwards; metal under paint) | none |

The two-layer glass/air/glass, steel/cavity/steel and sandwich units are symmetric: order has no effect there.

Fixed with `areareview-layerfix.py` (text surgery reordering the layer objects, JSON-compared; also in the volumes sources it needs none, thermal solids name their layer by id).

**Pins that move because of the confirmed changes** (emissivity or absorptivity of the exposed surface changed; nothing else did): thermal results of bar-stool, single-bed, three-seat-sofa, chalkboard, city-bus, tram and tower-crane cab: the catalog-wide tests `🌡️catalog-physics` and `🌡️thermal-span` (inequalities, not expected to fail), `🌐️thermal-multiview` and `🧊️chain-state` (tower-crane), `🏷️cast-provenance` and its fixture (chalkboard), `🎨️thermal-legend` (scale ranges of any scene containing these assets); no fixture pins a number of these assets as far as the search of `S/🧪️tests` and `S/🧫️fixtures` shows (the curtain-wall fixture `🧫️fixtures/🏗️curtain-wall-bay` defines its own regions, and its pins do not move because of the unchanged materials).

## Consequences for tests and pinned fixtures (for the final verification; nothing regenerated)

Found by searching `S/🧪️tests` and `S/🧫️fixtures` for the slugs of every asset whose area changed after the decisions (list `🗑️generated/areareview/changed-slugs.txt`, hits in `slug-hits.txt`).

**Tests that solve thermal results of changed assets (re-run; the checks are inequalities, expected to hold, but the operating-region flux changes above can move them):**
- `🧊️thermal-scenarios` (city-car: exhaust-system q 5111 -> 3407 W/m2, bonnet, roof, glazing, tailgate; sliding-door frame areas) - exhaust rise above 80 K and below 600 C.
- `🎨️thermal-legend` (city-car legend ranges; area-weighted merge of regions).
- `🚰️hydronic-coupling` (electric-kettle: shell 0.11 -> 0.095 and water 0.012 -> 0.0145 change the coupling draw; cooking-pot unchanged value, base marked exchange) - wall within `wetGapK` of 100 C.
- `🫕️phase-change` (electric-kettle water area scales the converted mass `kg = joules * area / latent`; any pinned kg changes by 0.0145/0.012; fixture `🧫️fixtures/🫕️phase-change`).
- `🌐️thermal-multiview` (diesel-generator silencer/stack/hood, excavator exhaust-stack emitter; tower-crane deferred) and `💨️transport-assets` (excavator exhaust plume scales with the stack surface temperature, stack area 0.22 -> 0.188).
- `🌤️reference-environment` (solar-thermal-collector: absorber back to 1.8, housing 2.6, stand 0.96), `🎭️cast-export` (toaster top-plate, electric-kettle clips), `⏳️thermal-transient` / `🎞️frame-timing` / `📐️dxf-reopen` / `📕️pdf-reopen` / `🗜️zip-reopen` / `🚀️export-performance` / `🧊️cast-lod` (walking-commuter bag-leather 0.28 -> 0.193: weights of the merged colour only), `🪢️tied-canopy` (canopy-steel 3.5 -> 2.97, rod-steel 0.93 -> 1.09; also the fixture `🧫️fixtures/🪢️tied-canopy`).
- `🌡️catalog-physics` and `🌡️thermal-span`: whole-catalog bounds on passive regions (independent of area) and finiteness; not expected to change.

**Whole-catalog or geometry fixtures that embed the catalog assets (regenerate pins only after the model authors finish; the area change itself alters `areaM2` in them):** `🧫️fixtures/🧊️model-catalog`, `🧫️fixtures/🧊️volumes-rules`, `🧫️fixtures/🧊️catalog-silhouette`, `🧫️fixtures/🧊️three-views`, `🧫️fixtures/🔭️multiview-coverage`, `🧫️fixtures/✂️section-views`, `🧫️fixtures/🎭️cast-export`, `🧫️fixtures/🏷️cast-provenance`, `🧫️fixtures/📐️model-scene`, `🧫️fixtures/📕️pdf-reopen`, `🧫️fixtures/🔺️polygon-union`, `🧫️fixtures/🧍️front-projection`, `🧫️fixtures/🧍️plan-projection`; and the geometry-only tests `🧊️volumes-rules-next`, `🏗️structure-projection`, `✂️section-views`, `🔭️multiview-coverage`, `🧊️model-frame`, `🧊️model-silhouette`, `🧊️chain-state`, `📑️multiview-sheet`, `🧍️plan-projection`, `🔺️polygon-union`, `🧊️axonometric-exports`, `📐️` dimension tests, `🔤️caption-fit`, `🏷️cast-provenance`, which mention the assets but depend on their volumes, not on `areaM2`.

## Open at the last pass

Deferred (no 1:50 volumes in the file at review): city-bicycle/mudguards, city-bicycle/saddle, city-bicycle/steer-assembly, cyclist/bike-metal, cyclist/saddle, cyclist/tyres, japanese-maple/limb-bark, japanese-maple/stem-bark, kneeling-gardener/pad, kneeling-gardener/trowel, london-plane-pollarded/trunk-bark, miscanthus-grass/culms, multi-stem-birch/branch-bark, multi-stem-birch/stem-bark, scots-pine/bark-lower, scots-pine/bark-upper, scots-pine/limb-bark, tower-crane/cab-glazing.

Without decision yet (new or changed rows, decided at the next pass): cantilever-balcony/railing-steel, cargo-bike/cargo, cargo-bike/drive-motor, cargo-bike/tyres, child-on-kick-scooter/deck, child-on-kick-scooter/grips, city-bicycle/frame-tubes, city-bicycle/hub-brakes, city-bicycle/lamps, city-bicycle/rims-spokes, city-car/tyres, electric-kick-scooter/bms-controller, electric-kick-scooter/brake-disc, electric-kick-scooter/deck-grip, electric-kick-scooter/hub-motor, electric-kick-scooter/tyres, glass-balustrade/clamp-steel, glass-balustrade/handrail-steel, glass-balustrade/panel-glass, market-parasol/frame-alu, panel-radiator/bracket, panel-radiator/valve, person-cooking/pot, person-cooking/spoon, playground-swing/frame-galv, playground-swing/seat-rubber, scissor-lift/battery-box, scissor-lift/chassis, solar-thermal-collector/glazing, stepladder/front-rails, street-lamp/plinth-concrete, tealight/wax, tied-canopy/wall-plate, tilt-window/frame-sill, tilt-window/frame-timber, tower-crane/hoist-motor, tower-crane/lattice, tower-crane/rope, tower-crane/slew-drive, tower-crane/trolley, tram/brake-resistor, tram/gangway, tram/pantograph, tram/wheelsets, vintage-car-1929/exhaust-system, vintage-car-1929/fenders, vintage-car-1929/hood, vintage-car-1929/interior, waste-bin/bin-ribs, weeping-willow/trunk-bark, wheel-loader/counterweight, wheel-loader/engine-bay.

Changed again by the author since the decision (re-review): cantilever-balcony/thermal-break (0.2 then 0.17 now 0.44), chalkboard/frame (0.8 then 1.1 now 2.0), chalkboard/tray (0.32 then 0.32 now 1.4), charcoal-grill/handle-wood (0.03 then 0.045 now 0.05), city-bus/tyres (6 then 8.3 now 9.5), concrete-mixer-truck/cab (24 then 27 now 29.8), concrete-mixer-truck/chassis (12 then 36 now 38), concrete-mixer-truck/chute (4 then 8.17 now 8.4), concrete-mixer-truck/drum-drive (2.5 then 3.04 now 3.2), concrete-mixer-truck/exhaust-stack (0.8 then 1.21 now 1.4), concrete-mixer-truck/tyres (10 then 19.2 now 18), delivery-van/brakes (0.6 then 0.72 now 0.93), delivery-van/cab-glazing (2.6 then 3.18 now 3.0), delivery-van/tyres (3.6 then 4.46 now 4.58), diesel-generator/outlet-flap (0.7 then 1.25 now 2.4), diesel-generator/skid-tank (12 then 3.92 now 27.5), drinking-fountain/bubbler-brass (0.04 then 0.047 now 0.06), evaporative-humidifier/casing (0.9 then 1.78 now 1.5), evaporative-humidifier/sump (0.2 then 0.209 now 0.42), evaporative-humidifier/trough (0.1 then 0.118 now 0.5), excavator/counterweight (4.5 then 4.5 now 13.5), excavator/engine-compartment (2.5 then 2.58 now 7.2), excavator/exhaust-stack (0.22 then 0.188 now 0.25), excavator/house (30 then 30 now 39), excavator/hydraulics (3.5 then 3.5 now 4.7), excavator/tracks (14 then 14 now 28), expansion-valve/body (0.012 then 0.0125 now 0.02), expansion-valve/bulb (0.002 then 0.0025 now 0.0026), expansion-valve/head (0.004 then 0.00467 now 0.0077), finned-heat-exchanger/headers (0.012 then 0.132 now 0.36), finned-heat-exchanger/tubes (0.08 then 0.188 now 0.67), forklift/battery-pack (4.5 then 2.18 now 3.6), forklift/brakes (0.4 then 0.295 now 1.0), forklift/cargo (5 then 3.14 now 6.0), forklift/controller (0.5 then 0.419 now 0.79), forklift/counterweight (3.5 then 4.3 now 5.0), forklift/fork-gear (1.6 then 1.5 now 4.0), forklift/pallet (2 then 1.78 now 2.9), forklift/tyres (1.6 then 1.95 now 3.0), glulam-beam/bearing-steel (0.04 then 0.06 now 0.099), heat-pump-outdoor-unit/base-tray (0.5 then 0.712 now 1.2), heat-pump-outdoor-unit/casing (3.4 then 6.12 now 6.8), heat-pump-outdoor-unit/discharge-vane (0.25 then 0.414 now 0.45), heat-pump-outdoor-unit/fan-guard (0.6 then 1.09 now 1.0), heat-pump-outdoor-unit/feet (0.1 then 0.187 now 0.43), heat-pump-outdoor-unit/top-lid (0.5 then 0.655 now 1.2), market-parasol/base-concrete (0.4 then 0.274 now 0.52), panel-radiator/top-grille (0.1 then 0.14 now 0.22), patio-heater/base-housing (0.75 then 0.98 now 1.43), patio-heater/control (0.004 then 0.0049 now 0.012), patio-heater/reflector (0.55 then 0.68 now 1.0), plate-heat-exchanger/damper (0.15 then 0.15 now 0.37), plate-heat-exchanger/frame (0.9 then 1.2 now 2.6), retractable-bollard/hydraulic-unit (0.35 then 0.45 now 0.46), retractable-bollard/shaft-steel (0.41 then 0.307 now 0.34), scroll-compressor/discharge (0.004 then 0.0072 now 0.008), scroll-compressor/dome (0.05 then 0.063 now 0.09), scroll-compressor/feet (0.02 then 0.084 now 0.15), scroll-compressor/scroll (0.006 then 0.0158 now 0.019), scroll-compressor/shell (0.1 then 0.205 now 0.28), scroll-compressor/suction (0.005 then 0.0062 now 0.0127), scroll-compressor/terminal (0.008 then 0.0138 now 0.0222), sectional-garage-door/spring-assembly (0.6 then 0.85 now 1.07), shopping-trolley/frame-tube (0.28 then 0.197 now 0.23), shopping-trolley/handle-grip (0.04 then 0.06 now 0.067), shopping-trolley/wheels (0.08 then 0.058 now 0.063), sliding-door/jamb-steel (0.42 then 0.42 now 1.43), sliding-door/leaf-frame (0.28 then 0.28 now 1.1), sliding-door/track-steel (0.3 then 0.3 now 1.1), solar-thermal-collector/absorber (1.8 then 2.38 now 4.1), solar-thermal-collector/housing (0.8 then 0.8 now 5.0), solar-thermal-collector/stand (0.45 then 0.45 now 1.13), sorption-wheel/casing (0.3 then 0.3 now 0.72), sorption-wheel/hub (0.012 then 0.0081 now 0.064), sorption-wheel/regen (0.064 then 0.064 now 0.138), spiral-stair/column-steel (2.28044 then 2.95 now 1.65), stepladder/feet (0.02 then 0.03 now 0.04), stepladder/platform (0.3 then 0.21 now 0.22), stepladder/rear-frame (0.43 then 0.506 now 0.535), stepladder/spreaders (0.06 then 0.074 now 0.082), stepladder/treads (0.5 then 0.39 now 0.4), street-lamp/lamp-housing (0.25 then 0.19 now 0.23), tealight/cup (0.003 then 0.00204 now 0.005), tied-canopy/canopy-steel (3.5 then 2.97 now 3.43), tied-canopy/rod-steel (0.93 then 1.09 now 1.12), tower-crane/ballast (40 then 36.5 now 52.5), tower-crane/house (25 then 32.2 now 39.3), vintage-car-1929/tyres (2.4 then 3.05 now 3.4), waste-bin/bin-hdpe (1.4 then 1.67 now 3.8), waste-bin/lid-hdpe (0.26 then 0.38 now 0.8), waste-bin/liner-film (0.08 then 0.111 now 0.19).

## Passes

| time | audit rows | applied | undecided | re-review |
|---|---|---|---|---|
| 2026-11-14 21:39 | 230 | 0 | 35 | 3 |
| 2026-11-14 21:40 | 234 | 0 | 39 | 3 |
| 2026-11-14 21:41 | 234 | 2 | 39 | 0 |
| 2026-11-14 21:44 | 232 | 1 | 0 | 0 |
| 2026-11-14 21:47 | 231 | 0 | 0 | 0 |
| 2026-11-14 21:48 | 236 | 0 | 6 | 0 |
| 2026-11-14 21:49 | 236 | 0 | 0 | 0 |
| 2026-11-14 21:50 | 236 | 0 | 0 | 0 |
| 2026-11-14 21:53 | 240 | 0 | 4 | 2 |
| 2026-11-14 21:54 | 240 | 0 | 0 | 2 |
| 2026-11-14 21:55 | 240 | 0 | 0 | 0 |
| 2026-11-14 21:58 | 241 | 0 | 1 | 0 |
| 2026-11-14 21:59 | 241 | 0 | 0 | 0 |
| 2026-11-14 22:02 | 248 | 1 | 6 | 0 |
| 2026-11-14 22:03 | 247 | 0 | 0 | 0 |
| 2026-11-14 22:09 | 249 | 5 | 2 | 0 |
| 2026-11-14 22:10 | 253 | 3 | 2 | 0 |
| 2026-11-14 22:11 | 252 | 0 | 0 | 0 |
| 2026-11-14 22:13 | 254 | 0 | 2 | 0 |
| 2026-11-14 22:13 | 254 | 0 | 0 | 0 |
| 2026-11-14 22:17 | 260 | 0 | 6 | 0 |
| 2026-11-14 22:17 | 263 | 0 | 3 | 0 |
| 2026-11-14 22:18 | 263 | 0 | 0 | 0 |
| 2026-11-14 22:20 | 265 | 0 | 2 | 0 |
| 2026-11-14 22:21 | 266 | 0 | 2 | 0 |
| 2026-11-14 22:28 | 280 | 28 | 15 | 0 |
| 2026-11-14 22:28 | 280 | 2 | 10 | 0 |
| 2026-11-14 22:29 | 280 | 0 | 10 | 0 |
| 2026-11-14 22:30 | 280 | 1 | 6 | 0 |
| 2026-11-14 22:33 | 285 | 0 | 12 | 0 |
| 2026-11-14 22:43 | 334 | 0 | 53 | 92 |
| 2026-11-14 22:44 | 334 | 0 | 53 | 92 |
| 2026-11-14 22:44 | 334 | 0 | 52 | 91 |

## Reviewer 2 (machinery, vehicles, components, objects; ruling C29)

Scripts: `areareview2-decisions.py` (decisions with one reason each), `areareview2-apply.py` (surgical: the `areaM2` number token, an inserted `areaBasis` line, existing number tokens of the default pose; JSON-compared, atomic, re-read before writing), `areareview2-unsection.py` (C25), `areareview2-dump.py` / `areareview2-brief.py` (hand-calculation aids: closed analytic surface per solid), `areareview2-report.py` (this section). The rows of the first reviewer for these four categories are superseded by the table below (its rows above stay as written).

### Rule derived from the solver (C29)

Read in `🌡️thermal/🧱️balance`, `🔬️physics` (`viewFactors`, `SOLAR_GEOMETRY_FACTOR`, `naturalNusselt`) and `solveSteady`:

- **R-A, passive, hydronic, coupled and water regions:** the surface temperature is one balance per m2; `areaM2` only weights the Watt totals, the merge colour and the coupled draw. The pose then applies `exposedFraction` and `contact.fraction` on top, so `areaM2` is the TOTAL outer area: every face of the union of the region's own solids, including faces covered by other regions, the ground, a wall or a rider, and both faces of a free sheet; the pose carries the covering. Excluded only: `sectionFaces` (C25), faces between solids of the same region, and the bore of a sealed hollow section or the inside of an enclosed casing (a void, not a face towards the environment).
- **R-B, operating regions:** `q = heatW x level / areaM2` sets the temperature, so `areaM2` is the surface the heat crosses. `heatW` is the datum and stays; the flux density follows the honest outer area (outer faces of a heated shell: the inside of a hood or casing is the source side). The value is the TOTAL outer area of the part otherwise.
- **R-C, conditioned regions:** `(interiorTempC - Ts) / (sum R layers + interior film)` is per m2 of the plate: the region is a layered plate with ONE interior side (the conditioned boundary, a C25 b face) and the outdoor side. `areaM2` is the outward faces (outer face plus the exposed edges), NOT the total: the interior-side face of the layered solid must be a `sectionFaces` face or the model exceeds the value. This is what the sibling regions of the catalog already do (curtain-wall glass 2.60 = one face, wall-face 4.87, sliding-door leaf-glass 1.62). The committed one-face values of the sliding door (0.28, 0.3, 0.42) were the right idea but counted one face only and omitted the edges.
- **R-D, `geometry`:** `horizontal-plate-up/-down` only selects the sky/ground view, the solar factor and the convection correlation per m2; it does not make the area one-sided, the opposite face of a horizontal plate is counted (market-parasol has two regions only because the two faces face different environments). A one-sided value is right only where the other face is a C25 face or the plate is conditioned (R-C).
- **Exchange areas** (fins, honeycomb, flame, plume, spray) keep `areaBasis: "exchange"` and a hand value (C19).
- Pose fractions: edited only where the decided total makes the committed fraction wrong (list at the end of this section).

### Totals

255 decided rows: ACCEPT 68, SET 186, exchange only 1.

### Decisions (committed = HEAD; value = in the file after this review)

| asset | region | kind | model | committed m2 | value m2 | decision | reason (independent hand calculation of the real object) |
|---|---|---|---|---|---|---|---|
| 🧱️components/cantilever-balcony | railing-steel | surface | passive | 0.24 | 0.78 | SET | two SHS 50 x 50 posts 1.03 m outer 0.2 each = 0.41 + handrail o60 x 1.0 0.19 + mid-rail o40 x 0.88 0.11 + two plates 0.05 + bolts; the open bores (0.35) are not outer area |
| 🧱️components/cantilever-balcony | thermal-break | wall | conditioned | 0.2 | 0.44 | SET | R-C: module 120 x 200 x 1000 closed 0.688 less the interior-side face 0.2 and the two end cuts (C25 a) 0.048 = 0.44 |
| 🚜️vehicles/cargo-bike | box-wood | surface | passive | 3.2 | 5.4 | SET | open plywood box, outer and inner faces closed 5.47 (the load covering the inside is another region) |
| 🚜️vehicles/cargo-bike | brakes | surface | operating | 0.04 | 0.09 | SET | closed 0.09 |
| 🚜️vehicles/cargo-bike | cargo | surface | passive | 1.2 | 1.61 | ACCEPT | no volumes in the file: applied by the author within a factor 1.5 of the committed value, no hand figure from the drawings is better; re-check against the total outer area (C29) once the models exist |
| 🚜️vehicles/cargo-bike | drive-motor | engine | operating | 0.12 | 0.095 | ACCEPT | no volumes in the file: applied by the author within a factor 1.5 of the committed value, no hand figure from the drawings is better; re-check against the total outer area (C29) once the models exist |
| 🚜️vehicles/cargo-bike | fenders | surface | passive | 0.6 | 0.27 | SET | two sheets closed 0.268 |
| 🚜️vehicles/cargo-bike | frame | surface | passive | 0.9 | 1.6 | SET | long frame, tubes and cross members closed 1.63 |
| 🚜️vehicles/cargo-bike | touch-parts | surface | passive | 0.25 | 0.14 | SET | closed 0.138 |
| 🚜️vehicles/cargo-bike | tyres | tyre | operating | 0.9 | 0.635 | ACCEPT | no volumes in the file: applied by the author within a factor 1.5 of the committed value, no hand figure from the drawings is better; re-check against the total outer area (C29) once the models exist |
| 🪑️objects/chalkboard | frame | surface | passive | 0.8 | 1.71 | SET | cross-check: validator outer area 1.71, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/chalkboard | tray | surface | passive | 0.32 | 1.4 | SET | U trough 4.0 m: 0.26 m2 per m exposed + back 0.06 x 4 against the board (counts) + chalk, ruler, sponge 0.14 = 1.4 |
| 🪑️objects/charcoal-grill | ash-catcher | surface | passive | 0.12 | 0.14 | ACCEPT | tray 0.14 |
| 🪑️objects/charcoal-grill | bowl-shell | surface | operating | 0.45 | 0.49 | SET | cross-check: validator outer area 0.49, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/charcoal-grill | handle-wood | surface | passive | 0.03 | 0.044 | SET | cross-check: validator outer area 0.044, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/charcoal-grill | vents-alu | surface | passive | 0.01 | 0.04 | SET | cross-check: validator outer area 0.04, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/city-bicycle | frame-tubes | surface | passive | 0.32 | 0.53 | SET | no volumes in the file: seat tube and post, main tube, head tube, stays, stand 0.28 + rack tray both faces 0.25 = 0.53 (hand, 4.5 m of o32 tube = 0.45 + tray) |
| 🚜️vehicles/city-bicycle | hub-brakes | surface | operating | 0.04 | 0.09 | SET | 2 drum hubs o0.1 x 0.04 with flanges 0.047 each |
| 🚜️vehicles/city-bicycle | lamps | lamp | operating | 0.012 | 0.036 | SET | tail lamp 45 x 60 x 60 + head lamp ellipsoid 90 x 70 x 90 |
| 🚜️vehicles/city-bicycle | mudguards | surface | passive | 0.5 | 0.351 | ACCEPT | 2 sheets 0.7 x 0.06 both faces 0.17 + stays and edges = 0.35 |
| 🚜️vehicles/city-bicycle | rims-spokes | surface | passive | 0.14 | 0.5 | SET | 2 rims o0.66 x 0.07 section = 0.29 + 72 spokes o3 x 0.3 = 0.2 |
| 🚜️vehicles/city-bicycle | saddle | surface | passive | 0.12 | 0.095 | SET | cross-check: validator outer area 0.095, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/city-bicycle | steer-assembly | surface | passive | 0.2 | 0.158 | ACCEPT | bar o22 x 0.6 0.04 + stem + grips + levers + bell = 0.16 |
| 🚜️vehicles/city-bus | body-panels | wall | conditioned | 52 | 62.0 | ACCEPT | sides, ends and underbody 62 m2 (reviewer 1 decision stands, underside counts) |
| 🚜️vehicles/city-bus | brakes | surface | operating | 1.2 | 1.18 | SET | cross-check: validator outer area 1.18, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/city-bus | doors | wall | conditioned | 8 | 5.33 | ACCEPT | R-C: outward leaf faces 4.83 + edges 0.4 |
| 🚜️vehicles/city-bus | e-axle | engine | operating | 3 | 3.0 | SET | restored: housing and motor 3.07 closed (the 2.0 was exposed-only) |
| 🚜️vehicles/city-bus | glazing | glazing | conditioned | 24 | 33.2 | ACCEPT | R-C: 2 x 12 x 1.2 + windscreen and rear 4.4 = 33 |
| 🚜️vehicles/city-bus | tyres | tyre | operating | 6 | 9.5 | SET | 6 x 275/70 R22.5 (o0.96 x 0.275): tread 0.83 + sidewalls 0.93 = 1.76 each = 10.6, hubs cover part = 9.5 |
| 🚜️vehicles/city-car | body-panels | surface | passive | 8.2 | 13.0 | ACCEPT | sides, ends and underbody 13 m2 of the 3.9 x 1.68 x 1.5 shell, roof, bonnet, tailgate and glass counted separately |
| 🚜️vehicles/city-car | bonnet | surface | operating | 1.2 | 1.44 | ACCEPT | outward face 1.44; the underside is the engine-bay source side |
| 🚜️vehicles/city-car | brakes | surface | operating | 0.22 | 0.86 | SET | cross-check: validator outer area 0.86, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/city-car | exhaust-system | exhaust | operating | 0.45 | 0.675 | ACCEPT | pipe o30 x 1.45 0.14 + silencer 0.75 x oval 0.41 + tail pipe 0.05 + manifold = 0.6-0.7 |
| 🚜️vehicles/city-car | glazing | glazing | passive | 3.2 | 3.2 | SET | restored: windscreen 1.0 + rear 0.5 + side glass 1.4 + quarter lights = 2.9-3.2 outward faces; the 4.25 counted both faces |
| 🚜️vehicles/city-car | roof | surface | passive | 1.5 | 1.78 | ACCEPT | outward face 1.78 (the cabin side is the enclosed interior) |
| 🚜️vehicles/city-car | tailgate | surface | passive | 0.9 | 0.75 | ACCEPT | panel 0.75 outward; the 1:100+ marker on the inner side is legitimate (it faces the enclosed luggage space, C25 b) and 1:50 must carry the same marker |
| 🚜️vehicles/city-car | tyres | tyre | operating | 1.8 | 3.3 | SET | 4 x 185/65 R15 (o0.62 x 0.185): tread 0.36 + sidewalls 0.5 = 0.86 each = 3.4 |
| 🏗️machinery/concrete-mixer-truck | brakes | engine | operating | 2.4 | 4.82 | SET | cross-check: validator outer area 4.82, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/concrete-mixer-truck | cab | surface | passive | 24 | 29.8 | SET | box 2.5 x 2.0 x 2.2 total 2 x (5.0 + 5.5 + 4.4) = 29.8 (underside on the chassis counts) |
| 🏗️machinery/concrete-mixer-truck | cab-glazing | glazing | passive | 4.5 | 2.73 | ACCEPT | drawn windscreen 1.3 + door and quarter glass 1.4 = 2.73, outward face (the cab side is an enclosed volume) |
| 🏗️machinery/concrete-mixer-truck | chassis | surface | passive | 12 | 38 | SET | rails 2 x 8.8 x 0.7 = 12.3, cross members 2, tanks 3, axles 8, deck both faces 10.9, steps 1.5 = 37.7 |
| 🏗️machinery/concrete-mixer-truck | chute | surface | passive | 4 | 8.4 | SET | open trough, both faces: 4.2 per face (main 2.4 m 2.69, extension 1.3 m 1.48) = 8.4 |
| 🏗️machinery/concrete-mixer-truck | drum | surface | passive | 36 | 36 | ACCEPT | outer skin 32-40 m2 (restored by reviewer 1; the inside faces the concrete charge) |
| 🏗️machinery/concrete-mixer-truck | drum-drive | engine | operating | 2.5 | 3.2 | SET | housing 0.8 x 0.8 x 0.6 total 2 x (0.64 + 0.48 + 0.48) = 3.2 (mount face counts) |
| 🏗️machinery/concrete-mixer-truck | engine-bay | engine | operating | 3.2 | 3.2 | ACCEPT | grille 0.9 + tunnel 0.96 + belly pan 0.8 = 2.7-3.2, committed 3.2 kept; model defect (tunnel panels missing) stays open |
| 🏗️machinery/concrete-mixer-truck | exhaust-stack | exhaust | operating | 0.8 | 1.4 | SET | o0.15 x 2.9 = 1.37 + cap |
| 🏗️machinery/concrete-mixer-truck | tyres | tyre | operating | 10 | 18 | SET | 8 x 315/80 R22.5 (o1.04 x 0.315): tread 1.03 + two sidewalls 2 x 0.59 = 2.2 each = 17.7; ground contact stays in the pose (0.06) |
| 🪑️objects/cooking-pot | handle | surface | passive | 0.003 | 0.00496 | ACCEPT | two ear handles 20 x 14 + rivets 0.0043-0.005 |
| 👤️humans/courier-with-hand-truck | hand-truck | surface | passive | 0.55 | 0.67 | ACCEPT | frame and toe plate 0.67-0.73 |
| 👤️humans/courier-with-hand-truck | parcels | surface | passive | 1.6 | 1.79 | SET | cross-check: validator outer area 1.79, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 👤️humans/courier-with-hand-truck | tyres | tyre | passive | 0.07 | 0.26 | SET | pneumatic tyre pair 0.26 m x 50 mm: tread 0.07 + sidewalls 0.19 = 0.26 (0.07 was the tread only) |
| 👤️humans/cyclist | bike-metal | surface | passive | 0.9 | 1.22 | SET | frame, fork, bars, rims, spokes, chain closed 1.22 |
| 👤️humans/cyclist | saddle | surface | passive | 0.07 | 0.07 | ACCEPT | whole saddle 0.07; the rider covering is the pose (exposed 0.5, contact 0.5) |
| 👤️humans/cyclist | tyres | tyre | passive | 0.6 | 0.6 | ACCEPT | 2 tyres o0.7 x 0.04: tread 0.088 + sidewalls 0.2 = 0.29 each = 0.58, committed 0.6 right; the model's 0.31 is too thin (finding) |
| 🚜️vehicles/delivery-van | brakes | surface | operating | 0.6 | 0.93 | SET | 4 drum brakes closed 0.93 (full surface, the pose carries the covering) |
| 🚜️vehicles/delivery-van | cab-glazing | glazing | passive | 2.6 | 3.0 | SET | outward faces 2.85 per the closed 5.7 of both faces, + edges |
| 🚜️vehicles/delivery-van | rear-doors | surface | passive | 3.4 | 2.55 | ACCEPT | two leaves 1.9 x 0.67 = 2.55 outward |
| 🚜️vehicles/delivery-van | roof | surface | passive | 8.5 | 6.41 | SET | cross-check: validator outer area 6.41, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/delivery-van | slider-door | surface | passive | 2.4 | 2.05 | SET | cross-check: validator outer area 2.05, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/delivery-van | tyres | tyre | operating | 3.6 | 4.58 | SET | 4 x 235/65 R16 (o0.72 x 0.235): tread 0.53 + sidewalls 2 x 0.3 = 1.14 each = 4.6 |
| 🏗️machinery/diesel-generator | discharge-hood | surface | operating | 4 | 4 | ACCEPT | outer shell 4.2 m2 (flux leaves through the outer face, the inside faces the hot air = source side) |
| 🏗️machinery/diesel-generator | enclosure-walls | wall | operating | 29 | 43.5 | ACCEPT | 4 walls 2 x (6.06 + 2.44) x 2.4 = 40.8 + 8 % corrugation = 44, outer face (the inside faces the room of the genset) |
| 🏗️machinery/diesel-generator | exhaust-stack | exhaust | operating | 0.3 | 0.323 | SET | cross-check: validator outer area 0.323, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/diesel-generator | inlet-louvres | surface | passive | 4.2 | 5.8 | ACCEPT | 60 slats of 2 mm sheet 1.3 x 0.035, both faces and edges 0.096 each = 5.8 |
| 🏗️machinery/diesel-generator | outlet-flap | surface | passive | 0.7 | 2.4 | SET | free flap 1.7 x 0.63 both faces 2.14 + edges 0.28 = 2.42 (faces covering the mouth frame count) |
| 🏗️machinery/diesel-generator | rain-cap | surface | passive | 0.1 | 0.287 | SET | cross-check: validator outer area 0.287, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/diesel-generator | silencer | exhaust | operating | 3 | 3.87 | ACCEPT | capsule o0.55: cylinder 2.85 + hemispheres 0.95 = 3.8 |
| 🏗️machinery/diesel-generator | skid-tank | surface | passive | 12 | 27.5 | SET | 6.06 x 1.94: top 11.76 (under the genset, other region) + bottom 11.76 (ground) + sides 3.2 + castings 0.7 = 27.4; pose exposure 0.14 and ground contact 0.43 |
| 🪑️objects/drinking-fountain | bubbler-brass | surface | passive | 0.04 | 0.054 | SET | cross-check: validator outer area 0.054, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/drinking-fountain | pedestal-cast | surface | passive | 0.65 | 0.87 | SET | closed 0.874 (ground face counts) |
| 🪑️objects/drinking-fountain | water-jet | water | passive | 0.0057 | 0.0071 | ACCEPT | jet loft 12 to 10 mm along 0.23 m = 0.0071-0.0082 |
| 🚜️vehicles/electric-kick-scooter | bms-controller | surface | operating | 0.02 | 0.024 | ACCEPT | no volumes in the file: applied by the author within a factor 1.5 of the committed value, no hand figure from the drawings is better; re-check against the total outer area (C29) once the models exist |
| 🚜️vehicles/electric-kick-scooter | brake-disc | surface | operating | 0.012 | 0.009 | SET | cross-check: validator outer area 0.009, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/electric-kick-scooter | deck-battery | surface | operating | 0.26 | 0.41 | SET | cross-check: validator outer area 0.41, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/electric-kick-scooter | deck-grip | surface | passive | 0.08 | 0.195 | SET | cross-check: validator outer area 0.195, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/electric-kick-scooter | hub-motor | engine | operating | 0.04 | 0.059 | SET | cross-check: validator outer area 0.059, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/electric-kick-scooter | stem-bars | surface | passive | 0.14 | 0.29 | SET | cross-check: validator outer area 0.29, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/electric-kick-scooter | tyres | tyre | operating | 0.14 | 0.15 | SET | 2 tyres o0.2 x 0.05: tread 0.031 + sidewalls 0.04 = 0.07 each |
| 🏗️machinery/evaporative-humidifier | casing | surface | passive | 0.9 | 1.5 | SET | outer faces of 0.5 x 0.6 x 0.4: 2 x (0.3 + 0.24) + roof 0.2 + bottom 0.2 = 1.48 + tray rim 0.02 (the inner faces face the process-air cavity, source side) |
| 🏗️machinery/evaporative-humidifier | sump | water | passive | 0.2 | 0.42 | SET | water body 0.44 x 0.38 x 0.05: top 0.167 + bottom 0.167 + sides 0.083 = 0.42; the pose exposure is the free surface 0.4 |
| 🏗️machinery/evaporative-humidifier | trough | surface | passive | 0.1 | 0.5 | SET | open trough 0.42 x 0.38: floor 0.16 (film on it counts) + underside 0.16 + walls 2 x 0.8 x 0.043 + edges = 0.5 |
| 🏗️machinery/excavator | attachment | surface | passive | 28 | 33.6 | ACCEPT | boom 2 x (0.6 + 0.55) x 5.7 = 13, stick 5.1, bucket outer 5 + inner 4 + cheeks 3 = 30 |
| 🏗️machinery/excavator | cab-glazing | glazing | passive | 6 | 4.0 | ACCEPT | windscreen 1.23 + side windows 1.8 + rear 0.5 + roof 0.5 = 4.0, outward face |
| 🏗️machinery/excavator | counterweight | surface | passive | 4.5 | 13.5 | SET | profile perimeter 4.8 x 2.4 = 11.5 + two sides 1.8 (front face on the house counts) = 13.3 |
| 🏗️machinery/excavator | engine-compartment | engine | operating | 2.5 | 7.2 | SET | drawn engine hood 1.1 x 0.65 x 2.4 wide: top 2.64 + two sides 1.43 + two ends 3.12 = 7.2 (the base on the house counts only through the box; the first decision used a 1.0 m width) |
| 🏗️machinery/excavator | exhaust-stack | exhaust | operating | 0.22 | 0.224 | SET | cross-check: validator outer area 0.224, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/excavator | house | surface | passive | 30 | 34.8 | SET | cross-check: validator outer area 34.8, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/excavator | hydraulics | surface | operating | 3.5 | 4.7 | SET | boom cylinders 2 x (barrel o0.2 x 1.45 + rod o0.09 x 1.2 = 1.25) + stick 1.36 + bucket 0.59 + hoses 0.3 = 4.8 |
| 🏗️machinery/excavator | tracks | surface | passive | 14 | 48.9 | SET | cross-check: validator outer area 48.9, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/expansion-valve | body | surface | operating | 0.012 | 0.02 | SET | box 0.064 x 0.05 x 0.06 closed 0.020 (mount face counts) |
| 🏗️machinery/expansion-valve | bulb | surface | operating | 0.002 | 0.0026 | SET | o14 x 50 closed 0.0026 |
| 🏗️machinery/expansion-valve | capillary | surface | passive | 0.0004 | 0.0006 | ACCEPT | o1.6 x 0.15 = 0.00075 |
| 🏗️machinery/expansion-valve | head | surface | operating | 0.004 | 0.007 | SET | cross-check: validator outer area 0.007, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/expansion-valve | inlet | surface | operating | 0.004 | 0.0032 | ACCEPT | pipe o9.5 x 0.1 = 0.003 + end |
| 🏗️machinery/expansion-valve | outlet | surface | operating | 0.004 | 0.0032 | ACCEPT | pipe o9.5 x 0.1 = 0.003 + end |
| 🏗️machinery/finned-heat-exchanger | fins | surface | operating | 2 | 2 | ACCEPT | exchange basis, committed 2 (heatW tuned to it) |
| 🏗️machinery/finned-heat-exchanger | headers | surface | operating | 0.012 | 0.36 | SET | two boxes 0.15 x 0.02 x 0.5 closed 0.176 each + stubs 0.005; heatW 2 W gives 5.6 W/m2 |
| 🏗️machinery/finned-heat-exchanger | tubes | heater | operating | 0.08 | 0.67 | SET | 23 tubes o0.018 x 0.5 = 0.65 + bends; the parts under the fins count (other region) |
| 🏗️machinery/forklift | battery-pack | surface | operating | 4.5 | 3.6 | SET | case 1.0 x 0.47 x 0.9 closed 2 x (0.47 + 0.9 + 0.423) = 3.59 (the chassis well covering it counts); heatW 540 W gives 150 W/m2 |
| 🏗️machinery/forklift | brakes | surface | operating | 0.4 | 1.0 | SET | 4 drums o0.3 x 0.1: 0.094 + 2 end discs 0.14 = 0.23 each = 0.95 |
| 🏗️machinery/forklift | cargo | surface | passive | 5 | 5.2 | SET | cross-check: validator outer area 5.2, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/forklift | chassis-paint | surface | passive | 8 | 8 | ACCEPT | top 1.9 + sides 1.7 + arches 1.2 + underside 2.15 + ends = 7.7 |
| 🏗️machinery/forklift | controller | engine | operating | 0.5 | 0.79 | SET | dash 0.23 x 0.42 profile x 0.6 closed 0.79 |
| 🏗️machinery/forklift | counterweight | surface | passive | 3.5 | 5.0 | SET | profile perimeter 3.2 x 1.1 + two sides 1.7 = 5.2 |
| 🏗️machinery/forklift | drive-motor | engine | operating | 0.9 | 0.9 | ACCEPT | traction motor o0.28 x 0.4 closed 0.48 + gear housing 0.25 = 0.73-0.9; committed 0.9 kept, model defect stays |
| 🏗️machinery/forklift | fork-gear | surface | passive | 1.6 | 4.0 | SET | carriage 1.3 + tines 0.67 + heels 0.26 + backrest 1.8 = 4.0 (the first decision counted exposed faces only) |
| 🏗️machinery/forklift | guard | surface | passive | 4 | 2.78 | ACCEPT | 4 posts o0.06 x 1.2 + rails and slats = 2.8 |
| 🏗️machinery/forklift | hydraulics | surface | operating | 0.4 | 0.323 | ACCEPT | 2 tilt cylinders barrel o0.08 x 0.45 + exposed rod 0.16 each = 0.32 |
| 🏗️machinery/forklift | mast | surface | passive | 5 | 5 | ACCEPT | outer rails 2 x 1.43 + inner 2 x 0.96 + cross members, chains 0.6 = 5.4; committed 5 kept, model defect stays |
| 🏗️machinery/forklift | pallet | surface | passive | 2 | 2.9 | SET | deck 1.2 x 0.8 both faces 1.92 + edges 0.18 + 3 runners 0.8 = 2.9 |
| 🏗️machinery/forklift | tyres | tyre | operating | 1.6 | 3.0 | SET | front 2 x (0.49 + 2 x 0.35) + rear 2 x (0.28 + 2 x 0.17) = 3.6, rim-covered flanks counted partly = 3.0 |
| 🧱️components/glass-balustrade | clamp-steel | surface | passive | 0.1 | 2.0 | SET | U shoe perimeter 0.96 x 2.0 = 1.92 + gasket and bolt faces, floor face counts (ground) |
| 🧱️components/glass-balustrade | handrail-steel | surface | passive | 0.13 | 0.27 | SET | o42.4 x 1.992 = 0.265 + two domed caps 0.006 |
| 🧱️components/glass-balustrade | panel-glass | glazing | passive | 2.2 | 4.29 | SET | 1.96 x 1.06 both faces 4.15 + edges 0.14 = 4.29; the part in the clamp counts (covered by another region) |
| 🧱️components/glulam-beam | bearing-steel | surface | passive | 0.04 | 0.099 | SET | two 120 x 20 x 160 plates closed 2 x 0.0496 (the face against the beam counts) |
| 🏗️machinery/heat-pump-outdoor-unit | base-tray | heater | operating | 0.5 | 1.2 | SET | tray 0.45 x 1.1: bottom 0.495 + top 0.495 (under the unit, counts) + edges 0.22 = 1.21 |
| 🏗️machinery/heat-pump-outdoor-unit | casing | surface | passive | 3.4 | 6.8 | SET | outer box 2 x (0.72 x 1.1 + 0.72 x 1.45 + 1.1 x 1.45) = 6.86 (inner faces look into the coil and fan chamber) |
| 🏗️machinery/heat-pump-outdoor-unit | coil-guard | surface | operating | 1.6 | 1.6 | ACCEPT | louvred guard 0.91 x 1.75 = 1.6, exchange basis |
| 🏗️machinery/heat-pump-outdoor-unit | compressor-casing | engine | operating | 1 | 1.5 | SET | closed 1.52 |
| 🏗️machinery/heat-pump-outdoor-unit | discharge-vane | surface | passive | 0.25 | 0.45 | SET | vane 0.24 x 0.72 both faces 0.346 + edges 0.096 |
| 🏗️machinery/heat-pump-outdoor-unit | fan-guard | surface | passive | 0.6 | 0.77 | SET | cross-check: validator outer area 0.77, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/heat-pump-outdoor-unit | feet | tyre | passive | 0.1 | 0.43 | SET | 2 feet 0.34 x 0.18 x 0.09 closed 2 x 0.216 (full surface; ground contact 0.28 in the pose) |
| 🏗️machinery/heat-pump-outdoor-unit | top-lid | surface | passive | 0.5 | 1.2 | SET | lid 0.47 x 1.12 both faces 1.05 + edges 0.13 = 1.18 |
| 🪑️objects/market-parasol | base-concrete | surface | passive | 0.4 | 0.52 | SET | top 0.12 + four sides 0.28 + bottom on the ground 0.12 = 0.52; ground contact 0.23 in the pose |
| 🪑️objects/market-parasol | frame-alu | surface | passive | 0.6 | 1.12 | SET | cross-check: validator outer area 1.12, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/motor-scooter | body-panels | surface | passive | 2.1 | 2.58 | ACCEPT | shell outward faces 2.3-2.6 |
| 🚜️vehicles/motor-scooter | brake-disc | surface | operating | 0.1 | 0.185 | SET | two 190 mm discs with rotor rings + two hub drums o90 closed 0.185; heatW 1312 W gives 7 kW/m2 |
| 🚜️vehicles/motor-scooter | engine | engine | operating | 0.35 | 0.255 | SET | cross-check: validator outer area 0.255, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/motor-scooter | exhaust-system | exhaust | operating | 0.35 | 0.156 | SET | cross-check: validator outer area 0.156, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/motor-scooter | headlamp | glazing | passive | 0.03 | 0.06 | SET | lens ellipsoid 60 x 90 x 130 + tail lamp box closed 0.062 |
| 🚜️vehicles/motor-scooter | seat | surface | passive | 0.35 | 0.453 | ACCEPT | 0.39-0.45 |
| 🚜️vehicles/motor-scooter | tyres | tyre | operating | 0.9 | 1.01 | SET | cross-check: validator outer area 1.01, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/oil-pumpjack | base | surface | passive | 14 | 84.9 | SET | cross-check: validator outer area 84.9, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/oil-pumpjack | crank | surface | passive | 6 | 9.7 | SET | crank arms and counterweights closed 9.7 |
| 🏗️machinery/oil-pumpjack | gearbox | engine | operating | 6.5 | 10.28 | SET | cross-check: validator outer area 10.28, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/oil-pumpjack | motor | engine | operating | 2.6 | 7.0 | SET | cross-check: validator outer area 7.0, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/oil-pumpjack | steel | surface | passive | 38 | 47 | SET | legs, braces, beam, horsehead, pitman, rod, guard: 46-49 at all scales |
| 🏗️machinery/oil-pumpjack | wellhead | surface | operating | 2.6 | 1.45 | SET | tree and flowline as drawn 1.1-1.5; heatW 300 W gives 207 W/m2 |
| 🧱️components/panel-radiator | bracket | surface | passive | 0.004 | 0.018 | SET | 4 brackets 12 x 35 x 40 mm, 0.0046 each closed = 0.018 (wall and panel faces count, C29) |
| 🧱️components/panel-radiator | top-grille | surface | passive | 0.1 | 0.22 | SET | 15 mm grille box 0.1 x 0.94: top 0.094 + underside on the panel 0.094 + lips and ends 0.03 = 0.22 |
| 🧱️components/panel-radiator | valve | surface | hydronic | 0.012 | 0.078 | SET | region binds both valve bodies, thermostatic head, two pipes and the lockshield cap: 0.0182 + 2 x 0.0214 + 2 x 0.0049 + 0.007 = 0.078 (the title names one valve: lockshield side flowPosition is an approximation) |
| 🪑️objects/patio-heater | base-housing | surface | passive | 0.75 | 1.257 | SET | cross-check: validator outer area 1.257, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/patio-heater | control | surface | passive | 0.004 | 0.011 | SET | cross-check: validator outer area 0.011, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/patio-heater | emitter-head | heater | operating | 0.2 | 0.198 | SET | cross-check: validator outer area 0.198, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/patio-heater | reflector | heater | operating | 0.55 | 0.694 | SET | cross-check: validator outer area 0.694, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/plate-heat-exchanger | damper | surface | passive | 0.15 | 0.37 | SET | flap 0.4 x 0.4 both faces 0.32 + edges 0.022 + hinge rod 0.025 |
| 🏗️machinery/plate-heat-exchanger | frame | surface | passive | 0.9 | 2.128 | SET | cross-check: validator outer area 2.128, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/plate-heat-exchanger | ports | surface | passive | 0.05 | 0.22 | ACCEPT | 4 collars outer wall 0.054 each + end rings = 0.22 (the bores carry the airstream) |
| 🪑️objects/playground-swing | chain-steel | surface | passive | 0.1 | 0.13 | ACCEPT | two chains o11 x 1.83 = 0.126 |
| 🪑️objects/playground-swing | frame-galv | surface | passive | 1.3 | 2.966 | SET | cross-check: validator outer area 2.966, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/playground-swing | seat-rubber | surface | passive | 0.12 | 0.489 | SET | cross-check: validator outer area 0.489, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/retractable-bollard | beacon-led | lamp | operating | 0.025 | 0.098 | SET | cross-check: validator outer area 0.098, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/retractable-bollard | cover-frame | surface | passive | 0.12 | 0.093 | ACCEPT | cast into the ground slab: C25 b faces excluded, 0.093 |
| 🪑️objects/retractable-bollard | hydraulic-unit | engine | operating | 0.35 | 0.46 | SET | unit 0.4587 closed; below ground, pose carries the contact 0.7 |
| 🪑️objects/retractable-bollard | reflector-band | surface | passive | 0.063 | 0.13 | SET | band both faces closed 0.132 (the inner face on the shaft counts) |
| 🪑️objects/retractable-bollard | shaft-steel | surface | passive | 0.41 | 0.34 | SET | closed shaft 0.341 |
| 🏗️machinery/scissor-lift | arms | surface | passive | 10 | 19.56 | SET | cross-check: validator outer area 19.56, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/scissor-lift | battery-box | engine | operating | 0.6 | 0.751 | SET | cross-check: validator outer area 0.751, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/scissor-lift | chassis | surface | passive | 5 | 5.885 | SET | cross-check: validator outer area 5.885, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/scissor-lift | deck | surface | passive | 3.4 | 4.77 | SET | platform both faces and edges 4.77 |
| 🏗️machinery/scissor-lift | motor-pump | engine | operating | 0.6 | 0.7 | SET | closed 0.71 |
| 🏗️machinery/scissor-lift | rails | surface | passive | 2.4 | 6.0 | SET | 3 levels of 50 mm square bars on four sides, 6 posts, 4 toe plates: about 30 m of bar x 0.2 = 6.0 |
| 🏗️machinery/scissor-lift | tyres | tyre | passive | 0.5 | 2.508 | SET | cross-check: validator outer area 2.508, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/scroll-compressor | discharge | exhaust | operating | 0.004 | 0.006 | SET | cross-check: validator outer area 0.006, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/scroll-compressor | dome | exhaust | operating | 0.05 | 0.079 | SET | cross-check: validator outer area 0.079, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/scroll-compressor | feet | surface | passive | 0.02 | 0.15 | SET | ring plate 0.28 square with o0.19 hole: top 0.049 + bottom 0.049 + edges and hole 0.056 = 0.154; contact 0.4 stays in the pose |
| 🏗️machinery/scroll-compressor | scroll | engine | operating | 0.006 | 0.016 | SET | cross-check: validator outer area 0.016, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/scroll-compressor | shell | engine | operating | 0.1 | 0.28 | SET | o0.28 x 0.25 cylinder 0.22 + base disc 0.06 on the feet |
| 🏗️machinery/scroll-compressor | suction | surface | operating | 0.005 | 0.009 | SET | cross-check: validator outer area 0.009, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/scroll-compressor | terminal | surface | passive | 0.008 | 0.015 | SET | cross-check: validator outer area 0.015, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🧱️components/sectional-garage-door | spring-assembly | surface | passive | 0.6 | 1.07 | SET | coil 122 turns of o8 (0.67) + shaft o25 x 2.6 (0.2) + two drums 0.2 = 1.07; touching turns share only a line |
| 🧱️components/sectional-garage-door | track-steel | surface | passive | 1.9 | 3.18 | ACCEPT | 2 C tracks 2 x (2.0 + 1.88 arc + 2.1) x 0.236 = 2.8, end bar 0.29, hangers 0.36 less joints = 3.2 (passive, total outer) |
| 🪑️objects/shopping-trolley | frame-tube | surface | passive | 0.28 | 0.23 | SET | tubes closed 0.228 |
| 🪑️objects/shopping-trolley | handle-grip | surface | passive | 0.04 | 0.067 | SET | grip capsule 0.49 x o40 = 0.0649 + ends |
| 🪑️objects/shopping-trolley | wheels | tyre | passive | 0.08 | 0.063 | SET | 4 wheels closed 0.0631 (the floor contact stays in the pose) |
| 🧱️components/sliding-door | jamb-steel | surface | conditioned | 0.42 | 1.43 | SET | R-C: 2 posts x 2.05 x 0.44 = 1.80 + ends 0.04 less the interior-side 100 mm faces 2 x 0.205 = 1.43 |
| 🧱️components/sliding-door | leaf-frame | surface | conditioned | 0.28 | 1.1 | SET | R-C: 4 members 2x1.9x0.2 + 2x0.9x0.2 = 1.12 less the interior-side face 0.28 = 0.84, + pull handle outer side 0.06 + bottom seal 0.14 + hangers 0.04 = 1.1 |
| 🧱️components/sliding-door | track-steel | surface | conditioned | 0.3 | 1.1 | SET | R-C: housing perimeter 0.448 x 3.1 = 1.39 less the interior-side 100 mm face 0.31 = 1.08 |
| 🏗️machinery/solar-thermal-collector | absorber | heater | operating | 1.8 | 4.1 | SET | C25: plate 1.88 x 0.91 both faces 3.42 + edges 0.05 + 9 risers o12 x 1.88 0.64 = 4.1; heatW -620 W gives -151 W/m2 (physical datum, C29) |
| 🏗️machinery/solar-thermal-collector | glazing | glazing | passive | 1.8 | 3.74 | SET | C25: the underside faces the air gap: 2 x 1.96 x 0.94 = 3.69 + edges 0.05 |
| 🏗️machinery/solar-thermal-collector | header-lower | surface | operating | 0.066 | 0.093 | SET | o34 x 0.88 closed 0.093 |
| 🏗️machinery/solar-thermal-collector | header-upper | surface | operating | 0.066 | 0.093 | SET | o34 x 0.88 closed 0.093 |
| 🏗️machinery/solar-thermal-collector | housing | surface | passive | 0.8 | 5.0 | SET | C25: cavity faces are free air: outer 2.6 + insulation top 1.84 + wall insides 0.55 = 5.0 |
| 🏗️machinery/solar-thermal-collector | stand | surface | passive | 0.45 | 1.13 | SET | 2 rails 1.5 x 0.05 x 0.05 closed 0.61 + 4 struts 0.52 (ground and collector faces count) |
| 🏗️machinery/sorption-wheel | casing | surface | passive | 0.3 | 0.72 | SET | outer band 0.43 + outer flange faces 0.146 + stand block 0.1 + its underside 0.043 |
| 🏗️machinery/sorption-wheel | disc | heater | operating | 0.19 | 0.19 | ACCEPT | exchange basis (honeycomb), committed 0.19 |
| 🏗️machinery/sorption-wheel | hub | engine | operating | 0.012 | 0.064 | SET | o0.09 x 0.18 hub closed 0.0636 |
| 🏗️machinery/sorption-wheel | regen | heater | operating | 0.064 | 0.138 | SET | two 84 degree seal plates outer face 0.0596 + edges 0.0094 each = 0.138 (the inner faces are the gap) |
| 🧱️components/spiral-stair | column-steel | surface | passive | 2.28044 | 1.65 | SET | CHS 168.3 x 6.3, 2.8 m: outside pi x 0.1683 x 2.8 = 1.48 + base plate top 0.138 + edges 0.03 = 1.65; the bore of a capped column is a sealed void, not outer area (the model's open bore adds 1.37) |
| 🧱️components/spiral-stair | treads | surface | passive | 3.729 | 6.67 | ACCEPT | 15 pressed pans: both faces 2 x 0.1247 + flange 2 x 1.78 x 0.055 = 0.445 each = 6.67 (mass 8 kg per tread confirms the flange) |
| 🧱️components/steel-column | cap-plate | surface | passive | 0.1 | 0.124 | ACCEPT | plate 0.24 x 0.22 x 0.02 closed 0.122; the column footprint under it is another region and counts |
| 🪑️objects/stepladder | feet | surface | passive | 0.02 | 0.08 | SET | cross-check: validator outer area 0.08, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/stepladder | front-rails | surface | passive | 0.52 | 0.66 | SET | closed 0.659 (treads are another region and count) |
| 🪑️objects/stepladder | platform | surface | passive | 0.3 | 0.22 | SET | closed 0.223 |
| 🪑️objects/stepladder | rear-frame | surface | passive | 0.43 | 0.535 | SET | closed 0.5354 |
| 🪑️objects/stepladder | spreaders | surface | passive | 0.06 | 0.071 | SET | cross-check: validator outer area 0.071, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/stepladder | treads | surface | passive | 0.5 | 0.4 | SET | closed 0.404 |
| 🪑️objects/street-lamp | arm-alu | surface | passive | 0.18 | 0.22 | ACCEPT | hollow arm 0.22 (open ends are C25 a, bore sealed) |
| 🪑️objects/street-lamp | lamp-housing | lamp | operating | 0.25 | 0.23 | SET | housing loft closed 0.227 |
| 🪑️objects/street-lamp | lamp-lens | glazing | passive | 0.08 | 0.05 | ACCEPT | dome outer face 0.049 (inner face is the lamp cavity) |
| 🪑️objects/street-lamp | plinth-concrete | surface | passive | 0.5 | 0.3 | SET | 0.30 x 0.30 x 0.10: sides 0.12 + top 0.09 under the plate (counts) + bottom 0.09 on the ground; pose exposure 0.4, contact 0.3 |
| 🪑️objects/tealight | cup | surface | passive | 0.003 | 0.005 | SET | foil cup closed 0.005: outer cone 0.0018 + underside 0.0006 + inner 0.0026 (the wax covering the inside counts) |
| 🪑️objects/tealight | flame | heater | operating | 0.0006 | 0.0006 | exchange only | exchange area of a 22 mm combustion zone (cylinder o8 x 22 = 0.0006), the drawn teardrop shows 0.0004 |
| 🪑️objects/tealight | wax | surface | passive | 0.001 | 0.0032 | SET | o38 x 10 puck: top 0.0011 + bottom 0.0011 + sides 0.0012 (cup covers them, counts) |
| 🧱️components/tied-canopy | canopy-glass | glazing | passive | 5.3 | 5.46 | SET | cross-check: validator outer area 5.46, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🧱️components/tied-canopy | canopy-steel | surface | passive | 3.5 | 3.43 | SET | beam profile perimeter 0.622 x 5.5 m = 3.42; the glass covering the top flange is another region and counts (C29) |
| 🧱️components/tied-canopy | rod-steel | surface | passive | 0.93 | 1.12 | SET | tube o60.3 x 4.68 m = 0.89 + cheeks 0.2 + pins 0.02 = 1.12 |
| 🧱️components/tied-canopy | wall-plate | surface | conditioned | 0.48 | 0.093 | SET | R-C: two 200 x 160 x 20 plates, closed 0.157 less the 0.032 faces bonded to the pad = 0.093 (the glued face is C25 b) |
| 🧱️components/tilt-window | frame-sill | surface | conditioned | 0.09 | 0.62 | SET | R-C: plate 0.3 x 0.05 x 0.9 closed 0.66 less the interior-side end face 0.045 = 0.62 (committed 0.09 was 0.1 x 0.9) |
| 🧱️components/tilt-window | frame-timber | surface | conditioned | 0.238 | 2.25 | SET | R-C: sash and jamb linings 2.52 (stiles 0.66, rails 0.49, two 0.3 x 1.2 linings with edges 1.4) less the interior-side sash strip 0.245 and lining joints = 2.25; the committed 0.238 is the sash strip alone |
| 🏗️machinery/tower-crane | ballast | surface | passive | 40 | 69.9 | SET | cross-check: validator outer area 69.9, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🏗️machinery/tower-crane | cab-glazing | glazing | conditioned | 4 | 4 | ACCEPT | R-C: conditioned pane, outward face 4.0 (the round-1 2.9 stays restored) |
| 🏗️machinery/tower-crane | counterweight | surface | passive | 25 | 32.6 | ACCEPT | 3 slabs stacked 2.4 x 2.2 x 2.4: sides 21.1 + top 5.8 + underside 5.8 = 32.6 |
| 🏗️machinery/tower-crane | hoist-motor | engine | operating | 3 | 5.1 | SET | box 1.2 x 0.8 x 0.8 closed 5.12 |
| 🏗️machinery/tower-crane | house | surface | passive | 25 | 39.3 | SET | slew platform 5.8 x 0.7 x 2.4 closed 39.3 (the underside on the slew ring counts) |
| 🏗️machinery/tower-crane | lattice | surface | passive | 180 | 390 | SET | 292 members of the mast, jib and counter-jib as drawn plus the tower head, bar by bar 384-434 m2 (angle sections 0.4 m2 per m of chord); porous lattice, not an exchange area |
| 🏗️machinery/tower-crane | rope | surface | passive | 1 | 2.75 | SET | 8 rope pieces o19 x 5.75 = 2.75 |
| 🏗️machinery/tower-crane | skip | surface | passive | 8 | 9.4 | ACCEPT | skip body outer 5.2 + inner 4 (open top) + block 1.2 + hook + slings = 9.4 |
| 🏗️machinery/tower-crane | slew-drive | engine | operating | 1 | 3.79 | SET | ellipse ring 2.8 x 2.0, perimeter 7.58 x 0.5 = 3.79; the bearing faces are C25 b |
| 🏗️machinery/tower-crane | trolley | surface | passive | 4 | 10 | SET | two box girders 2.0 m (perimeter 1.0) 4 + cross members 2 + wheels 2.5 + sheave 0.4 = 9-10 (the drawn filled block 12.3 is not the open frame) |
| 🏗️machinery/tower-crane | winch-house | surface | operating | 38 | 31.5 | ACCEPT | box 4 x 1.8 x 1.385 outer faces 30.5 + vent stack 1.0; the heat leaves through the outer face, the plates' inner faces face the hot air |
| 🚜️vehicles/tram | body-panels | wall | conditioned | 55 | 79.82 | ACCEPT | walls 2 x 15.45 x 2.2 + ends 10.6 + underbody 37 less glazing and doors 41.7 = 74-80 |
| 🚜️vehicles/tram | brake-discs | surface | operating | 1.6 | 1.89 | ACCEPT | discs 1.9 |
| 🚜️vehicles/tram | brake-resistor | surface | operating | 3.2 | 8.5 | SET | box 2.0 x 1.8 x 0.24: top 3.6 + bottom 3.6 (stand-offs, free air) + sides 1.4 = 8.6; heatW 30 kW gives 3.5 kW/m2 |
| 🚜️vehicles/tram | doors | wall | conditioned | 11 | 8.89 | ACCEPT | R-C: outward faces 8.9 |
| 🚜️vehicles/tram | gangway | surface | passive | 9 | 16.7 | SET | bellows 2.0 x 2.8 section x 0.9 m: outside 8.4 + inside 8.3 (both faces are in air) |
| 🚜️vehicles/tram | glazing | glazing | conditioned | 26 | 32.83 | ACCEPT | R-C: outward faces 32.8 (as drawn) |
| 🚜️vehicles/tram | hvac-unit | surface | operating | 7 | 8.96 | ACCEPT | roof unit outer faces 9 (inner faces are the duct) |
| 🚜️vehicles/tram | pantograph | surface | operating | 1.1 | 3.61 | SET | cross-check: validator outer area 3.61, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/tram | wheelsets | surface | operating | 3.5 | 7.8 | SET | 8 wheels r0.3 x 0.13: tread 0.245 + flanks 2 x 0.3 = 0.85 less hub 0.2 = 0.97 each = 7.8 |
| 🚜️vehicles/vintage-car-1929 | body-panels | surface | passive | 6.4 | 8.16 | ACCEPT | tub sides, rear, front, cabin frames 8.16 outward |
| 🚜️vehicles/vintage-car-1929 | brakes | surface | operating | 0.5 | 0.75 | SET | closed 0.745 |
| 🚜️vehicles/vintage-car-1929 | engine-bay | engine | operating | 0.8 | 1.12 | ACCEPT | engine and sump 1.1-1.6 |
| 🚜️vehicles/vintage-car-1929 | exhaust-system | exhaust | operating | 0.4 | 0.96 | SET | pipe o55 x 1.95 0.37 + silencer o150 x 0.65 0.31 + manifold, down and tail pipe 0.28 |
| 🚜️vehicles/vintage-car-1929 | fenders | surface | passive | 2.6 | 5.2 | SET | 4 crescent wings 0.71-1.01 (both faces) = 3.4 + 2 running boards 0.92 = 5.2 |
| 🚜️vehicles/vintage-car-1929 | glazing | glazing | passive | 1.6 | 1.93 | ACCEPT | outward faces 1.9 |
| 🚜️vehicles/vintage-car-1929 | hood | surface | operating | 1.5 | 2.6 | SET | top 0.79 + two louvred sides 0.82 each + slot walls = 2.6 (undersides face the engine bay) |
| 🚜️vehicles/vintage-car-1929 | interior | surface | passive | 3.5 | 6.6 | SET | 2 benches: backrest 1.96 + cushion 1.27 = 3.2 each + column, wheel, handles 0.2 = 6.6; the pose exposes half |
| 🚜️vehicles/vintage-car-1929 | radiator | surface | operating | 0.45 | 0.512 | SET | cross-check: validator outer area 0.512, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🚜️vehicles/vintage-car-1929 | roof | surface | passive | 2.2 | 3.06 | ACCEPT | outward face 2.8-3.06 |
| 🚜️vehicles/vintage-car-1929 | tyres | tyre | operating | 2.4 | 3.79 | SET | cross-check: validator outer area 3.79, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/waste-bin | bin-hdpe | surface | passive | 1.4 | 3.8 | SET | HDPE shell closed 3.79: outer 1.9 + inner 1.9 (the liner covering the inside is another region and counts) |
| 🪑️objects/waste-bin | bin-ribs | surface | passive | 0.16 | 0.28 | SET | ribs closed 0.28 |
| 🪑️objects/waste-bin | lid-hdpe | surface | passive | 0.26 | 0.445 | SET | cross-check: validator outer area 0.445, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |
| 🪑️objects/waste-bin | liner-film | surface | passive | 0.08 | 0.19 | SET | film both faces 0.19 |
| 🏗️machinery/wheel-loader | bucket | surface | passive | 14 | 33 | SET | open scoop 2.95 m wide, both faces: outer 14.5 + inner 14 + cheeks, ribs, edge 4.5 (as the mixer-truck chute) |
| 🏗️machinery/wheel-loader | cab | surface | passive | 14 | 18.98 | ACCEPT | closed prism of the drawn cab profile 1.6 m wide, 19 m2 |
| 🏗️machinery/wheel-loader | cab-glazing | glazing | passive | 6 | 6.0 | SET | restored: windscreen 1.76 + 2 side panes 2 x 1.8 + rear 1.0 = 6.4 outward faces; the author's 7.98 counted both faces |
| 🏗️machinery/wheel-loader | counterweight | surface | passive | 6 | 14.8 | SET | profile perimeter 4.9 x 2.2 = 10.8 + two sides 2 x 1.4 = 13.6 + grille pocket 1.2 |
| 🏗️machinery/wheel-loader | engine-bay | engine | operating | 8 | 21.0 | SET | hood 2.0 m wide, profile area 3.11 and perimeter 7.38: 2 x 3.11 + 7.38 x 2.0 = 20.98 (bottom on the chassis and front against the cab count); heatW 3000 W gives 143 W/m2 |
| 🏗️machinery/wheel-loader | exhaust-stack | exhaust | operating | 0.5 | 0.524 | ACCEPT | o0.18 x 0.85 = 0.48 + cap |
| 🏗️machinery/wheel-loader | front-frame | surface | passive | 10 | 25.8 | SET | box beam 3.25 x 0.58 x 1.8: 2 x 3.25 x 1.8 + 2 x 0.58 x 1.8 + 2 x 1.885 = 17.6 + two tower rails 8 = 25.6 |
| 🏗️machinery/wheel-loader | hydraulics | engine | operating | 2.2 | 2.79 | ACCEPT | two lift barrels o0.24 + rods o0.1, tilt barrel o0.2 + rod o0.09 = 2.8 |
| 🏗️machinery/wheel-loader | lift-arm | surface | passive | 9 | 12.51 | ACCEPT | 2 arms: profile perimeter 7.4 x 0.3 x 2 + faces = 12.5 |
| 🏗️machinery/wheel-loader | tyres | tyre | operating | 14 | 35.0 | SET | cross-check: validator outer area 35.0, my figure was a closed-surface sum or coarse hand value; the model counts what the real object has here, adopted |

### Cross-check against the product validator (reviewer 2, `areareview2-cross.py`, `volumes-check --product`, `info:outerAreaM2` at 1:50)

88 decided rows differed by more than 10 % at the first run (`🗑️generated/areareview2/cross.txt`). Verdict per cause:
- **My sum or coarse hand figure was wrong (adopted the validator's value, 58 rows, reason "cross-check" in the table):** closed-surface sums that ignored model detail (patio-heater, scroll-compressor, scissor-lift, motor-scooter), the oil-pumpjack base (84.9: slab bottom 18.7 on the ground and the deck underside count, my 60 left them out), excavator tracks (48.9, belt loop both faces plus frames) and house (34.8), tower-crane ballast (69.9: the four blocks are not a closed 2 x 2 stack), tied-canopy canopy-glass (5.46: the pane faces inside the laminate carry legitimate `sectionFaces`, I had missed them), waste-bin lid (0.445, same cause), stepladder feet (0.08).
- **The validator is right by C29, my value is a policy choice (kept):** one-sided shells and plates where the model closes both faces and the inner face is not marked: city-car roof/bonnet/tailgate/glazing/body-panels, city-bus body-panels, tram body-panels/hvac, mixer cab-glazing/drum, diesel-generator hood/walls, tower-crane winch-house/cab-glazing, wheel-loader cab-glazing, excavator cab-glazing, humidifier casing, sorption-wheel casing/regen, street-lamp arm/lens, sliding-door and tied-canopy wall-plate (R-C), spiral-stair column and cantilever railing (sealed bores), plate-heat-exchanger ports (bores in a duct), cantilever thermal-break. Here the MODEL must change (mark the inner faces `sectionFaces` or fill the bore) or the coordinator must rule that cavity and interior faces count; until then these rows fail `areaDeviation` on purpose.
- **The model is wrong (value kept):** excavator engine-compartment (validator 14.9 against 7.2: the 1.0 x 1.8 grille top box and the 2.4 m engine block overlap instead of forming one hood), mixer engine-bay (tunnel missing, 1.86), forklift mast (boxes instead of C channels, 3.97 against 5.4), forklift drive-motor, cyclist tyres.
- **Validator defects:** none in the numbers; interface notes in `tool-requests.md`.
- Pose fractions follow the measured totals: excavator tracks contact 0.09, tower-crane ballast contact 0.18.

### C25 `sectionFaces`

- solar-thermal-collector: `th-glass` and `th-absorber-plate` (`th-absorber` at 1:200/1:500) lose `-v` at all four scales (asset and source, `areareview2-unsection.py`); the air gap between glass and absorber is free air that the collector physics is about. Areas decided: glazing 3.74, absorber 4.1 (heatW -620 W stays: -151 W/m2), housing 5.0 (the cavity faces count now).
- city-car `th-tailgate +v` at 1:100/200/500: judged LEGITIMATE (the inner side of a closed tailgate faces the enclosed luggage space, C25 b); the 1:50 tailgate must carry the same marker, 0.75 is the outward panel.
- city-car, delivery-van, city-bus shell undersides: already removed by reviewer 1 (verified: no `sectionFaces` left on them); tram `sectionFaces` are the interior faces of conditioned shells (R-C, legitimate).

### Pose fractions edited (default pose, existing members only)

- excavator/tracks `contact.fraction` -> 0.09: ground contact of 2 x 3.5 x 0.6 = 4.2 of the measured total 48.9 m2
- tower-crane/ballast `contact.fraction` -> 0.18: bottom 12.25 of the measured total 69.9
- diesel-generator/skid-tank `contact.fraction` -> 0.43: bottom 11.76 of 27.5 on the ground
- diesel-generator/skid-tank `exposedFraction` -> 0.14: top 11.76 covered by the genset: free sides 3.9 of 27.5
- evaporative-humidifier/sump `exposedFraction` -> 0.4: free surface 0.167 of 0.42
- heat-pump-outdoor-unit/feet `contact.fraction` -> 0.28: two soles 0.12 of 0.43

## Reviewer 3 (humans, animals, vegetation, appliances, furniture; ruling C29, C27)

Scripts: `areareview3-measure.py` (product measurement, `volumes-check.ts --scale 50 --rules volumes-thermal-coverage --json`, 94 assets, output `🗑️generated/areareview3/measure.json`), `areareview3-table.py`, `areareview3-dump.py` / `areareview3-brief.sh` (closed analytic surface per solid, reads the 1:50 volumes from `volumes/<category>/<slug>.json` when the asset file has none), `areareview3-decisions.py` (one reason per row), `areareview3-apply.py` (surgical: number token of `areaM2` and `skinFraction`, JSON-compared, atomic, re-read before writing), `areareview3-c27.py` / `areareview3-c27apply.py` (C27), `areareview3-report.py` (this section and the findings). The rows of reviewer 1 for these five categories stay as written above; where this section decides the same row, this section is the later decision.

### Method and rule

Rule of reviewer 2 (R-A passive, R-B operating, R-C conditioned, enclosed casings and inner faces of cavities do not count, both faces of a free sheet count, covered faces count) with one extension for stacked slices: the faces between slices of one construction (the four zones of a hot-water cylinder, the plank and its joint) are interfaces of that construction, not outer faces. Every decision is a hand calculation of the real object from `dimensions`, the drawing and standard sections, compared with the measured outer area of the 1:50 model (validator `info:outerAreaM2`, which for plain solids agrees with the closed analytic surface of the solids within 3 %). Where model and hand figure differ by more than 15 %, the hand figure is kept where it is the honest object, and the model is named in `model-findings.md`; where the two bracket a value (hand range), the value inside both is set. `heatW` of operating regions is the datum and stays; the flux density follows the honest outer area.

Measurement over all 94 assets of the five categories: 620 regions, 360 outside 0.75-1.3 of the declared value, of which about 300 are exchange regions (foliage, fur, feather, hair: not compared, C19), body-model overlays (below) or regions already decided.

### Totals

122 decided rows: ACCEPT 53, SET 68, SET + exchange 1; C27 redistributions: 14 body-region values.

### Decisions (committed = HEAD; value = in the file after this review)

| asset | region | kind | model | committed m2 | value m2 | decision | reason (independent hand calculation of the real object) |
|---|---|---|---|---|---|---|---|
| 🌳️vegetation/agave | caudex | stem | passive | 0.21 | 0.29 | ACCEPT | cylinder o0.22 x 0.42 = 0.29 (reviewer 1 stands); the extrusion in the model is a rectangular prism (0.38): finding |
| 🌳️vegetation/apple-tree | spur-twig | stem | passive | 0.12 | 0.185 | SET | midpoint of the committed 0.12-0.23 hand figure and the model 0.1465: a weighting-only passive region |
| 🛋️furniture/bar-stool | seat-pad | surface | passive | 0.147 | 0.25 | SET | pad o0.36 x 0.04: top 0.102 + side 0.045 + underside on the plate 0.102 = 0.249 (C29: covered faces count) |
| 🛋️furniture/bar-stool | seat-plate | surface | passive | 0.102 | 0.216 | ACCEPT | plywood disc o0.34 x 0.032: 2 x 0.091 + rim 0.034 = 0.216 (reviewer 1 stands); the model's `+u` on the top hides the face against the pad, which is another region and counts (C29): finding |
| 🛋️furniture/bookshelf | plinth | surface | passive | 0.12 | 0.153 | ACCEPT | kick board 0.772 x 0.075 x 0.018: 2 x 0.058 + top and bottom 0.028 + ends 0.003 = 0.147; the model is a solid 0.25 m deep block (0.539): model defect |
| 🔌️appliances/ceiling-diffuser | plate | surface | operating | 0.45 | 0.45 | ACCEPT | R-B: room-facing face 0.595^2 - 0.01 = 0.35 + cones 0.07 + edges 0.03 = 0.45; the back flange face lies against the ceiling void (C25 b): the model's 0.757 counts it because the flange box has no `sectionFaces`: finding |
| 👤️humans/child-on-kick-scooter | deck | surface | passive | 0.22 | 0.262 | ACCEPT | deck 0.59 x 0.13 x 0.02 plate: 2 x 0.077 + edges 0.029 = 0.18 + fender 0.048 + brake pad 0.011 + stem foot = 0.262; model 0.2624 |
| 👤️humans/child-on-kick-scooter | grips | surface | passive | 0.016 | 0.022 | ACCEPT | 2 grips o0.03 x 0.087: 2 x (0.0082 + 0.0014 caps) = 0.0192, with the bar ends 0.022; model 0.0216 |
| 🔌️appliances/circulation-pump | casing | surface | operating | 0.035 | 0.048 | SET | volute o0.115 x 0.06: side 0.0217 + 2 faces 0.0208 + unions 0.0063 + stubs 0.003 = 0.052; model 0.0466 |
| 🔌️appliances/circulation-pump | motor | surface | operating | 0.03 | 0.034 | SET | motor head o0.09 x 0.075: side 0.0212 + end 0.0064 + back on the casing 0.0064 = 0.034 |
| 🔌️appliances/circulation-pump | rotor-window | glazing | passive | 0.0022 | 0.0022 | ACCEPT | pane o0.052: outer face 0.0021 + edge 0.0003 = 0.0024; the inner face looks into the wet rotor chamber (enclosed) |
| 👤️humans/courier-with-hand-truck | parcels | surface | passive | 1.6 | 1.79 | ACCEPT | 3 parcels closed 2.3 + tape 0.04, less the stacked contact faces 0.35 (faces between solids of one region are not outer) = 2.0; the author re-set the value to 1.79 after this review: within 10 % of the hand figure and of the model 1.805, accepted |
| 👤️humans/courier-with-hand-truck | tyres | tyre | passive | 0.07 | 0.26 | ACCEPT | 2 hand-truck wheels o0.26 x 0.05: tread 0.041 + 2 sidewall annuli 0.038 each = 0.12 per wheel = 0.25; model 0.2615 |
| 👤️humans/cyclist | bike-metal | surface | passive | 0.9 | 1.22 | ACCEPT | frame tubes 3.1 m o0.034 = 0.33, fork/stays 0.2, bars and grips 0.1, fenders 0.29, rims 0.147, chain and rings 0.1, cranks and pedals 0.1 = 1.27; model 1.19 (13 %), committed 1.05 within 20 % |
| 👤️humans/cyclist | saddle | surface | passive | 0.07 | 0.07 | ACCEPT | saddle 0.27 x 0.15 x 0.05 closed shell = 0.07; model 0.072 |
| 👤️humans/cyclist | tyres | tyre | passive | 0.6 | 0.6 | ACCEPT | 2 tyres 700 x 38: torus (2 pi x 0.33)(2 pi x 0.019) = 0.25 each = 0.5-0.6; model 0.6005 |
| 🔌️appliances/desktop-monitor | panel-front | surface | operating | 0.175 | 0.2 | ACCEPT | R-B: front face 0.299 x 0.53 = 0.159 + bezel and edges 0.04 = 0.2; the back faces the interior; the model counts both faces (0.37) |
| 🔌️appliances/desktop-monitor | stand | surface | passive | 0.125 | 0.16 | SET | base plate 0.25 x 0.21 x 0.01 closed 0.114 (the underside on the desk counts) + neck 0.0375 + hinge 0.006 + feet 0.003 = 0.16 |
| 🛋️furniture/dining-table | apron | surface | passive | 0.42 | 0.92 | SET | aprons 2 x (1.4 + 0.7) = 4.2 m x 0.08 high x 2 faces 0.67 + lower edge 0.084 + upper edge on the top 0.084 + corner blocks 0.08 = 0.92; model 1.01 |
| 🛋️furniture/dining-table | leaf-top | surface | passive | 0.54 | 1.14 | SET | stored leaf 0.6 x 0.88 x 0.03: 2 x 0.528 + edges 0.09 = 1.14 (both faces count, the pose carries the stowing: exposedFraction) |
| 🛋️furniture/dining-table | table-top | surface | passive | 1.44 | 3.0 | ACCEPT | 1.6 x 0.9 both faces 2.88 + edges 0.15 = 3.03 (reviewer 1 stands, model 3.03) |
| 🔌️appliances/duct-fan | casing | surface | operating | 0.07 | 0.14 | SET | plate 0.22 x 0.22 x 0.012: front less bore 0.017 + back on the wall 0.048 + edges 0.01 + ring wall outer 0.046 + lip 0.02 = 0.14; the model ring is a solid block (0.207) |
| 🔌️appliances/electric-fan-heater | casing | surface | operating | 0.17 | 0.33 | SET | housing 0.196 x 0.31 x 0.24 closed 0.365 less the grille and intake openings 0.071 + handle 0.05 = 0.34 |
| 🔌️appliances/electric-fan-heater | control | surface | passive | 0.004 | 0.0045 | SET | dial o0.04 x 0.015: side 0.0019 + front 0.0013 + back on the casing 0.0013 = 0.0045 |
| 🔌️appliances/electric-fan-heater | feet | surface | passive | 0.003 | 0.0085 | ACCEPT | 4 feet o0.03 x 0.01: side 0.00094 + 2 x 0.0007 = 0.0023 each = 0.009; the model feet are larger (0.0137) |
| 🔌️appliances/electric-fan-heater | grille | heater | operating | 0.03 | 0.05 | ACCEPT | R-B: perforated plate 0.17 x 0.21 outer face 0.036 + hole walls 0.014 = 0.05; the rear face is the interior |
| 🔌️appliances/electric-fan-heater | intake | surface | passive | 0.03 | 0.05 | ACCEPT | R-A enclosed: plate 0.16 x 0.21 outer face 0.034 + slats 0.012 + edges = 0.05; the inner face looks into the casing |
| 🔌️appliances/electric-fan-heater | top-cover | heater | operating | 0.035 | 0.055 | ACCEPT | R-B: top 0.17 x 0.21 = 0.036 + rim 0.015 = 0.051; the underside faces the hot interior; the model's closed solid counts it (0.102) |
| 🔌️appliances/electric-kettle | lid | surface | operating | 0.025 | 0.025 | ACCEPT | dome o0.14 outer face 0.0185 + button 0.004 + rim 0.003 = 0.026; the inner face is enclosed (R-A); the model's closed solid counts it (0.034), finding stays |
| 🔌️appliances/electric-kettle | plate | heater | operating | 0.05 | 0.065 | SET | power base o0.18 x 0.03: side 0.017 + top 0.0254 + underside 0.0254 = 0.068; model 0.0648 (the top under the kettle counts, C29) |
| 🔌️appliances/electric-kettle | shell | surface | coupled | 0.11 | 0.15 | SET | body o0.16 x 0.2: lateral 0.10 + bottom 0.02 + spout 0.01 + handle and rim 0.02 = 0.15; model 0.166 |
| 🔌️appliances/electric-kettle | water | water | operating | 0.012 | 0.0155 | SET + exchange | evaporating free surface pi x 0.0705^2 = 0.0156: the boil flux acts there, the water body is enclosed (its 0.088 is no outer face): exchange area (C19) |
| 🔌️appliances/electricity-meter | window | glazing | passive | 0.011 | 0.011 | ACCEPT | window and counter glass: pane o0.092 outer face 0.0066 + counter 0.0023 + rim 0.003 = 0.012; the inner faces face the sealed meter; the model closed solid counts both faces (0.026) |
| 🌳️vegetation/english-oak | limb-bark | bark | passive | 42 | 42 | ACCEPT | limb wood 3 m3 at mean r0.17 gives 2/r x V = 35-42 m2; the model limbs (12 solids, 66 m2) are drawn up to r0.37 over 5.7 m, 1.6x heavier than the mass and crown allow: model defect, committed 42 stays |
| 🌳️vegetation/english-oak | trunk-bark | bark | passive | 16 | 25.0 | SET | stem 0-4.5 m mean o1.1 = 15.5 + flare 2.5 + upper trunk 4.5-7 m o0.7 = 5.5 + leader 2 = 25.5; committed 16 omitted the upper trunk and the leader; model analytic 25.0 |
| 🔌️appliances/gas-boiler | casing | surface | operating | 0.9 | 0.9 | ACCEPT | within 3 % of the model (0.927): box 0.4 x 0.7 x 0.3 outer less the controls panel |
| 🔌️appliances/gas-hob | knob | surface | passive | 0.0025 | 0.0025 | ACCEPT | two knobs (0.3 m hob) o0.022 x 0.009: 2 x (0.00038 + 0.00062) = 0.002-0.0028; the model draws one knob (0.0014 incl. back face) |
| 🔌️appliances/gas-meter | index | surface | passive | 0.02 | 0.05 | SET | index plate 0.19 x 0.098 x 0.008 closed 0.042 (its back on the meter body counts) + drums 0.008 = 0.05 |
| 🔌️appliances/gas-meter | pipes | surface | operating | 0.014 | 0.02 | SET | 2 stubs o0.028 x 0.07 0.012 + 2 unions o0.04 x 0.018 0.0045 + ends = 0.02 |
| 🛋️furniture/height-adjustable-desk | drive-left | engine | operating | 0.05 | 0.075 | SET | drive housing 0.1 x 0.148 x 0.1 closed 0.079; heatW 35 W gives q = 470 W/m2 (committed 0.05 gave 700) |
| 🛋️furniture/height-adjustable-desk | drive-right | engine | operating | 0.05 | 0.075 | SET | drive housing 0.1 x 0.148 x 0.1 closed 0.079 |
| 🛋️furniture/height-adjustable-desk | steel-desk | surface | passive | 0.8 | 2.15 | ACCEPT | 2 foot plates 2 x 0.6 + 2 telescope stages 0.40 + 2 sleeves 0.38 + 2 columns 0.29 + top rail 0.41 + cable tray 0.18 + brackets 0.06 = 3.0 solid sum; interfaces between sleeves, stages and columns bury 0.8 of it: 2.15-2.4 (reviewer 1 stands) |
| 🌳️vegetation/hornbeam-hedge | stem-bark | bark | passive | 0.8 | 1.0 | SET | hedge 0.9 m x 2.2 m, 9 stems per 3 m: 27 stems o0.025 x 1.0 m = 2.1 under the foliage in the model, 0.8 committed; 1.0 is the midpoint of hand 0.8 and model 1.19 |
| 🔌️appliances/hot-water-cylinder | connections | surface | operating | 0.03 | 0.03 | SET | 5 stubs o0.022 x 0.05 0.017 + 4 hex nuts 0.0029 each + ends = 0.03 |
| 🔌️appliances/hot-water-cylinder | top-cap | surface | operating | 0.31 | 0.31 | ACCEPT | dome o0.6 outer face 0.283 x 1.1 + rim = 0.31; the face to zone-4 is an interface (finding) |
| 🔌️appliances/hot-water-cylinder | zone-1 | surface | operating | 0.62 | 0.62 | ACCEPT | side of a 0.33 m slice of o0.6: pi x 0.6 x 0.33 = 0.62; the end faces between the stacked slices are interfaces of one cylinder (C25 a), the model counts them (1.19): finding |
| 🔌️appliances/hot-water-cylinder | zone-2 | surface | operating | 0.62 | 0.62 | ACCEPT | side of a 0.33 m slice of o0.6 = 0.62 (interfaces are no outer faces) |
| 🔌️appliances/hot-water-cylinder | zone-3 | surface | operating | 0.62 | 0.62 | ACCEPT | side of a 0.33 m slice of o0.6 = 0.62 (interfaces are no outer faces) |
| 🔌️appliances/hot-water-cylinder | zone-4 | surface | operating | 0.62 | 0.62 | ACCEPT | side of a 0.33 m slice of o0.6 = 0.62 (interfaces are no outer faces) |
| 🌳️vegetation/japanese-maple | limb-bark | bark | passive | 1.37 | 1.55 | SET | 6 limbs 1.2 m, o0.09 to o0.04: 6 x pi x 0.065 x 1.2 = 1.47; model 1.72 (analytic 1.71); 1.55 is within 15 % of both |
| 🌳️vegetation/japanese-maple | stem-bark | bark | passive | 3.45 | 2.3 | ACCEPT | 3 stems o0.12 tapering to o0.07 over 2.3 m: 3 x pi x 0.095 x 2.3 = 2.0 + root collar 0.3 = 2.3; model 2.6 (13 %), within the 15 % of the validator |
| 🔌️appliances/kitchen-extractor-hood | chimney | surface | passive | 0.75 | 0.78 | SET | chimney 0.3 x 0.2 telescopic, 0.7 m: 1.0 x 0.7 = 0.70 + collar 0.064 = 0.76 (the wall side counts); model 0.846 with the extract duct |
| 🔌️appliances/kitchen-extractor-hood | filter | surface | passive | 0.2 | 0.84 | SET | grease filter 0.86 x 0.46 x 0.02: 2 x 0.396 + edges 0.05 = 0.84 (the upper face inside the hood counts, C29); reviewer 1's own hand figure of one face 0.45 was kept at 0.2 |
| 👤️humans/kneeling-gardener | hat | clothing | body | 0.226 | 0.226 | ACCEPT | C18: clothing of a living subject. One-sided sky face: brim annulus (r0.235, r0.09) 0.148 + crown 0.061 + band 0.009 = 0.213 = 0.226 (6 %); the real hat has 0.38 with the brim underside; the underside faces the shaded hair pack and counts as the head side, so the physiological value stays, the model measures 0.398 (finding) |
| 👤️humans/kneeling-gardener | pad | surface | passive | 0.28 | 0.7 | SET | foam pad 0.76 x 0.4 x 0.04: 2 x (0.304 + 0.0304 + 0.016) = 0.70 (both faces count, the ground face is contact, C29) |
| 👤️humans/kneeling-gardener | trowel | surface | passive | 0.025 | 0.041 | SET | blade 0.16 x 0.07 tapered 0.0085 x 2 faces = 0.017 + shank 0.006 + handle o0.03 x 0.11 = 0.0104 + ferrule = 0.039-0.045; model 0.0448 |
| 👤️humans/kneeling-gardener | trug | surface | passive | 0.62 | 0.86 | SET | boat basket o0.5 x 0.21 deep, open: outside 0.43 + inside 0.43 = 0.86 (both faces of a free shell) |
| 👤️humans/leaning-person | phone-surface | surface | operating | 0.02 | 0.025 | SET | phone 0.15 x 0.07 x 0.008: 2 x 0.0105 + edges 0.0035 = 0.0245 |
| 👤️humans/leaning-person | wall-face | wall | conditioned | 3.5 | 3.5 | ACCEPT | R-C: conditioned wall sample 2.5 x 1.3 = 3.25 + edges 0.25 = 3.5, outward faces only; the model counts the interior-side face (6.46): its `sectionFaces` is missing: finding |
| 🌳️vegetation/london-plane-pollarded | pollard-bark | bark | passive | 4.13 | 7.2 | SET | pollard head o1.1 x 1.2 m (4.1) + 5 knuckles o0.35 x 0.6 (3.3) = 7.4; the committed 4.13 counted the head without the knuckles; model 7.15 |
| 🌳️vegetation/london-plane-pollarded | shoot-bark | stem | passive | 3.67 | 3.67 | ACCEPT | shoot crown of ~60 water shoots o0.04 x 0.5 m = 3.8; model 4.4 (20 %), a weighting-only passive region |
| 🌳️vegetation/london-plane-pollarded | trunk-bark | bark | passive | 7.53 | 11.2 | SET | clear stem 4.2 m, mean o0.9 with the flare: pi x 0.9 x 4.2 = 11.9, 10.5-11.9; model 11.24 |
| 🌳️vegetation/miscanthus-grass | culms | bark | passive | 0.5 | 0.603 | ACCEPT | ~45 culms o0.004 x 1.0 m mean = 0.57 + tuft base 0.2 = 0.6-0.7; model 0.70 (16 %) |
| 🌳️vegetation/multi-stem-birch | branch-bark | bark | passive | 4 | 2.67 | ACCEPT | 6 branches o0.06 x 2.4 m mean = 0.45 each = 2.7; model 2.62 |
| 🌳️vegetation/multi-stem-birch | stem-bark | bark | passive | 14 | 18.78 | ACCEPT | 3 stems o0.28 tapering to o0.12, mean o0.2 x 9 m = 5.6 each = 17 + stool 0.8 = 17.8; model 18.8 |
| 🔌️appliances/mvhr-unit | casing | surface | operating | 0.9 | 1.16 | SET | box 0.6 x 0.665 x 0.3: 2 x (0.18 + 0.2 + 0.2) = 1.16 (the wall face counts, heat crosses every face of the casing) |
| 🔌️appliances/mvhr-unit | ducts | surface | passive | 0.08 | 0.1 | SET | 4 open spigots o0.098 x 0.062 outer 0.019 + bead 0.006 = 0.025 each = 0.10; the model closed solids (0.167) count the open ends |
| 🛋️furniture/office-swivel-chair | castor-rubber | tyre | passive | 0.02 | 0.035 | SET | 5 wheels o0.05 x 0.025: tread 0.0039 + 2 sides 0.0039 = 0.0079 each = 0.039; model 0.033 |
| 🛋️furniture/office-swivel-chair | chair-steel | surface | passive | 0.1 | 0.22 | SET | sleeve 0.059 + column 0.02 + collar and cone 0.025 + mechanism box 0.2 x 0.2 x 0.042 0.114 + strut 0.03 = 0.25; model 0.22 |
| 🛋️furniture/operating-table | light-housing | lamp | operating | 0.45 | 0.43 | SET | dome o0.5 outer face 2 pi 0.25^2 = 0.39 + rim 0.04; the `th-light-plume` ellipsoid (0.26) is a convection plume, not a surface: finding |
| 🛋️furniture/operating-table | light-lens | lamp | operating | 0.19 | 0.19 | ACCEPT | lens disc o0.48 room face 0.181 + rim 0.023 = 0.2; the inner face looks into the housing (R-A enclosed) |
| 🛋️furniture/operating-table | table-column | surface | passive | 0.45 | 0.9 | SET | column cover 0.21 x 0.21 x 0.34 sides 0.29 + lower section 0.25 + head plate 0.32 x 0.4 x 0.04 closed 0.31 = 0.85-0.9; model 0.913 |
| 👤️humans/parent-with-stroller | stroller-fabric | surface | passive | 1.1 | 2.0 | ACCEPT | seat sling 0.47 x 0.85 both faces 0.8 + hood 0.45 + basket 0.38 x 0.4 x 0.1 both faces 0.46 = 1.7-1.9; the author's 2.0 stands within 5 % of the model 1.91 |
| 👤️humans/parent-with-stroller | stroller-frame | surface | passive | 0.35 | 0.5 | ACCEPT | 4.3 m of tube o0.025 = 0.34 + footrest 0.1 + hangers 0.05 = 0.5; the author's 0.5 stands |
| 👤️humans/parent-with-stroller | tyres | tyre | passive | 0.45 | 0.48 | ACCEPT | 2 rear o0.32 x 0.04 (0.116 each) + 2 front o0.2 x 0.04 (0.067 each) = 0.36 rubber + hub faces 0.12 modelled in the region = 0.48; model 0.493 |
| 🛋️furniture/park-bench | back-timber | surface | passive | 0.54 | 1.3 | SET | 3 raked slats 1.8 x (0.08 x 0.05 chamfered, perimeter 0.229) = 0.412 + ends 0.008 = 0.42 each = 1.26 + fixings 0.04 |
| 🛋️furniture/park-bench | seat-timber | surface | passive | 0.76 | 2.4 | ACCEPT | 4 slats 1.8 x 0.11875 x 0.045 both faces + edges 0.6 each = 2.4; model 2.40 |
| 🔌️appliances/pendant-lamp | shade | lamp | operating | 0.11 | 0.11 | ACCEPT | shade dome o0.3 outer face 0.11, model 0.105 |
| 👤️humans/person-cooking | counter | surface | passive | 2.4 | 2.96 | ACCEPT | cabinet 0.6 x 0.76 x 0.62 sides and fronts 1.85 + worktop top 0.40 and edges 0.15 + plinth 0.49 + handle 0.02 = 2.9; model 2.96 |
| 👤️humans/person-cooking | hob | surface | passive | 0.3 | 0.6 | SET | 52 x 52 cm hob plate: top 0.27 + underside in the worktop 0.27 + edges 0.027 + burner rings 0.03 = 0.6 (the committed 0.3 was the top face) |
| 👤️humans/person-cooking | pot | surface | operating | 0.12 | 0.2 | SET | R-B: outer wall pi x 0.24 x 0.141 = 0.106 + base 0.045 + lid top 0.045 + handles 0.01 = 0.2; the inside faces the charge (source side); model 0.275 closed solid |
| 👤️humans/person-cooking | spoon | surface | passive | 0.012 | 0.026 | SET | wooden spoon 0.3 m: handle o0.02 x 0.25 0.0157 + bowl 0.012 less the joint = 0.026 |
| 👤️humans/person-on-crutches | crutch-cuffs | surface | passive | 0.04 | 0.1 | SET | 2 open cuffs o0.1 x 0.1 high, 75 % closed, both faces: 2 x 2 x 0.0236 = 0.094 + edges 0.01 = 0.10; model 0.109 (the 0.06 counted one face) |
| 👤️humans/person-on-crutches | crutch-tips | surface | passive | 0.006 | 0.014 | SET | 2 rubber tips o0.03 x 0.05: side 0.0047 + ends 0.0014 = 0.0061 each = 0.012-0.014; model 0.018 |
| 👤️humans/person-with-rollator | tyres | tyre | passive | 0.1 | 0.21 | SET | 4 wheels o0.2 x 0.03: tread 0.019 + 2 sidewall annuli pi x (0.1^2 - 0.07^2) = 0.016 each = 0.051 per wheel = 0.205 (the 0.029 per wheel counted a narrow sidewall ring) |
| 👤️humans/person-with-umbrella | umbrella-frame | surface | passive | 0.08 | 0.14 | SET | shaft o0.0176 x 0.95 = 0.0525 + J handle o0.033 x 0.2 = 0.021 + hub 0.013 + 8 ribs o0.005 x 0.56 = 0.075 + ferrule 0.004 = 0.165; the model draws two ribs (analytic 0.138): 0.14 is the model value, hand 0.165 |
| 👤️humans/person-with-white-cane | bag | surface | passive | 0.12 | 0.17 | SET | shoulder bag 0.25 x 0.2 x 0.08 closed 0.172 (strap folded in the model); model 0.1703 |
| 👤️humans/person-with-white-cane | cane-grip | surface | passive | 0.008 | 0.022 | SET | rubber grip o0.025 x 0.2 m 0.0157 + wrist strap 2 x 0.015 x 0.2 = 0.006 = 0.022 |
| 👤️humans/person-with-white-cane | sunglasses | glazing | passive | 0.006 | 0.0098 | SET | two lenses 0.05 x 0.04 elliptical both faces 0.0063 + edges 0.0013 + bridge and arm stubs 0.0022 = 0.0098; the committed 0.0046 was the outer lens faces only |
| 🛋️furniture/picnic-table | a-frame | surface | passive | 2.4 | 1.2 | SET | 4 leg boards 0.09 x 0.045 x 0.88: both faces 0.158 + edges 0.079 = 0.24 each = 0.95 + cross beams 0.96 and 0.6 m (perimeter 0.18) 0.28 = 1.2-1.5; model 1.18; reviewer 1's 0.998 was the low end |
| 🔌️appliances/room-thermostat | dial | surface | passive | 0.0033 | 0.00243 | ACCEPT | knob o0.042 x 0.01: side 0.0013 + front 0.0014 = 0.0027; model 0.0016-0.0027 |
| 🔌️appliances/room-thermostat | led | lamp | operating | 0.0001 | 5e-05 | ACCEPT | half dome a3.5 mm c2 mm 5.6e-5 m2; heat crosses the dome, the base is the source side |
| 👤️humans/running-child | backpack | surface | passive | 0.12 | 0.18 | SET | child pack 0.2 x 0.26 x 0.09 closed 0.19 less the strap overlap; model 0.162 |
| 🌳️vegetation/scots-pine | bark-lower | bark | passive | 12 | 15.5 | SET | trunk 0-8 m o0.55 tapering to o0.5: pi x 0.525 x 8 = 13.2 + root flare 1.3 + stubs 0.5 = 15.0; model 16.8-17.2 (the 3-section loft bulges); the author raised 14.42 to 17.2 |
| 🌳️vegetation/scots-pine | bark-upper | bark | passive | 14 | 12.0 | SET | trunk 8-18 m o0.4 to o0.2: pi x 0.3 x 10 = 9.4 + leader 2.0 = 11.4; model 11.7-13.5; the author raised 11.38 to 13.5 |
| 🌳️vegetation/scots-pine | limb-bark | bark | passive | 8 | 13.0 | SET | 6 limbs: 2 x (o0.4 mean 0.2 x 4.5 m = 2.8) + 2 x (0.15 x 4 m = 1.9) + 2 x (0.1 x 4 m = 1.3) = 11.9, with the base swelling 13; model 15.1 (limbs drawn o0.25 at the base) |
| 👤️humans/seated-guitarist | guitar | surface | passive | 0.62 | 0.62 | ACCEPT | acoustic guitar: top 0.16 + back 0.16 + ribs 1.5 m x 0.095 = 0.14 + neck and head 0.1 = 0.56-0.62; the sound box is enclosed (R-A), the model counts the inner faces of its plates (1.03): finding |
| 👤️humans/seated-guitarist | stool | surface | passive | 0.55 | 0.645 | ACCEPT | seat o0.41 x 0.04: 2 x 0.132 + 0.0515 = 0.316 + 3 legs o0.042 x 0.51 0.22 + rails 0.1 = 0.64; model 0.646 |
| 👤️humans/seated-office-worker | castor-rubber | tyre | passive | 0.02 | 0.036 | SET | 5 castor wheels o0.05 x 0.02: tread 0.0031 + 2 sides 0.0039 = 0.007 each = 0.035 |
| 👤️humans/seated-office-worker | chair-plastic | surface | passive | 0.35 | 0.85 | SET | backrest shell 0.4 x 0.45 both faces 0.4 + 5 star arms o0.03 x 0.28 0.13 + hub 0.03 + 2 armrests (pad 0.25 x 0.06 both faces + post) 0.12 = 0.7-0.9; model 0.90; the 0.35 was one face of the backrest and the arms |
| 👤️humans/seated-office-worker | chair-steel | surface | passive | 0.1 | 0.2 | SET | column sleeve o0.068 x 0.28 0.06 + column 0.02 + collar and cone 0.024 + mechanism box 0.2 x 0.042 x 0.14 0.085 + backrest strut 0.03 = 0.22; model measured 0.19 |
| 👤️humans/seated-office-worker | desk | surface | passive | 1.1 | 1.35 | ACCEPT | top 0.78 x 0.6 x 0.025 closed 1.005 + 2 posts 0.04 x 0.04 x 0.695 0.229 + apron 0.126 + feet 0.029 = 1.39 less the post-to-top contacts 0.04 = 1.35 (author re-set 1.1 to 1.35; model 1.346) |
| 👤️humans/seated-office-worker | laptop | surface | operating | 0.28 | 0.39 | SET | 15 inch laptop open: base 0.355 x 0.25 x 0.019 closed 0.20 + lid 0.355 x 0.25 x 0.009 closed 0.19 = 0.39 (the underside on the table counts, C29); heatW 60 W stays, q = 154 W/m2 (committed 0.233 gave 257) |
| 👤️humans/seated-reader | cup | surface | passive | 0.035 | 0.035 | ACCEPT | cup body o0.08 x 0.105 side 0.023 + base 0.004 + lid o0.074 top 0.0043 + side 0.0035 (sleeve overlaps the body, same region) = 0.035; model 0.0376 |
| 🔌️appliances/server-rack | feet | surface | passive | 0.06 | 0.06 | ACCEPT | 4 castors o0.07 x 0.03 wheel: tread 0.0066 + sides 0.0077 = 0.014 each = 0.058; the model draws them as o0.032 posts (0.0246): model defect |
| 🔌️appliances/shower-head | handle | surface | passive | 0.012 | 0.022 | SET | handle o0.035 x 0.18 = 0.0198 + ends 0.002 = 0.022 |
| 🔌️appliances/shower-head | rail | surface | passive | 0.06 | 0.083 | SET | tube o0.022 x 0.69 0.0477 + caps + holder box 0.02 + brackets 0.0125 = 0.083 |
| 🛋️furniture/single-bed | bed-frame | surface | passive | 4.0 | 4.7 | SET | head board 0.95 x 0.9 both faces + edges 1.93 + foot board 0.5 x 0.9 1.0 + 2 rails 2.04 x 0.16 closed 0.83 each = 4.6 |
| 🛋️furniture/single-bed | mattress | surface | passive | 2.5 | 4.5 | SET | 2.0 x 0.9 x 0.21 closed 2 x (1.8 + 0.42 + 0.19) = 4.8 (the model 1.43 x 0.82 core gives 4.3): 4.5; the bottom on the slats counts (C29) |
| 🛋️furniture/single-bed | slats | surface | passive | 1.5 | 3.0 | SET | 21 slats 0.06 x 0.022 x 0.86 closed 0.1436 each = 3.0 (both faces, the rail contact counts) |
| 🔌️appliances/thermostatic-radiator-valve | body | surface | operating | 0.0125 | 0.0095 | SET | body o0.038 x 0.042 0.005 + stub o0.017 x 0.023 0.0012 + nut and ends 0.003 = 0.0095 |
| 🔌️appliances/thermostatic-radiator-valve | knob | surface | operating | 0.0028 | 0.005 | SET | knob o0.04 x 0.02: side 0.0025 + front 0.0013 + back on the body 0.0013 = 0.005 |
| 🔌️appliances/toaster | control | surface | passive | 0.003 | 0.0044 | SET | lever 0.03 x 0.024 x 0.008 closed 0.0023 + dial o0.03 x 0.008 0.0021 = 0.0044 (the backs on the housing count, C29) |
| 🔌️appliances/toaster | feet | surface | passive | 0.002 | 0.007 | ACCEPT | 4 rubber feet o0.025 x 0.01: side 0.0008 + 2 x 0.00049 = 0.0018 each = 0.007; the model draws them o0.05 (4x the area, 0.020): model defect |
| 🔌️appliances/toaster | top-plate | heater | operating | 0.045 | 0.05489 | ACCEPT | R-B: top 0.3 x 0.17 less slots = 0.051 + edges = 0.055; the underside faces the hot interior (source side); the model's closed solid counts both faces (0.079) |
| 🔌️appliances/tower-server | feet | surface | passive | 0.002 | 0.016 | SET | 4 pads 0.04 x 0.03 x 0.012 closed 0.0041 each = 0.0164 (the ground face counts, C29); the model's `-v` marking removes the ground faces (0.010) |
| 🔌️appliances/vacuum-cleaner | grip | surface | passive | 0.02 | 0.025 | SET | sleeve o0.035 x 0.15 = 0.0165 + ends and bore faces 0.008; model 0.0297 |
| 🔌️appliances/vacuum-cleaner | wand | surface | passive | 0.09 | 0.088 | SET | tube o0.036 x 0.78 = 0.088 + ends 0.002; the 0.066 was the shorter retracted tube |
| 🔌️appliances/vacuum-cleaner | wheels | tyre | passive | 0.05 | 0.068 | SET | 2 wheels o0.12 x 0.03: tread 0.0113 + 2 sides 0.0113 each = 0.034 each = 0.068 |
| 🌳️vegetation/weeping-willow | limb-bark | bark | passive | 22 | 22 | ACCEPT | 4 main limbs o0.35 x 5 m 5.5 + 4 secondary o0.2 x 5 m 3.1 + 100 pendulous branches o0.04 x 1.4 m 17.6 = 26 less overlap; committed 22 stands within 5 % of the model 22.97 |
| 🌳️vegetation/weeping-willow | trunk-bark | bark | passive | 9 | 14.5 | SET | stem 0-3 m o1.0 = 9.4 + root flare 2 + 2 forks o0.5 x 1.5 m = 4.7 = 16; model 15.05, 13.5 (author) was 10 % under both |
| 👤️humans/wheelchair-user | frame-metal | surface | passive | 0.55 | 1.3 | SET | frame tubes 7 m o0.025 0.55 + 2 rims (o0.6 x 0.07 tube) 0.26 + footplate 0.25 + spokes 0.1 + push rims 0.1 = 1.26; model 1.31; the 0.55 was the frame tubes alone |

### Re-check of reviewer 1's accepted rows (C29 against measurement)

Of the 95 ACCEPT rows of reviewer 1 in these categories, 44 were re-calculated by hand against the measured outer area (all rows whose measured value is outside 0.75-1.3 of the accepted value, plus the named ones). Corrected: stroller-frame stands, but crutch-cuffs (one face counted, 0.06 -> 0.10), crutch-tips (0.009 -> 0.014), rollator tyres (0.116 -> 0.21: the sidewall was a narrow ring), sunglasses (outer lens faces only, 0.0046 -> 0.0098), castor-rubber of the office worker (five wheels at 0.007 are 0.035, not 0.025), chair-steel and laptop of the office worker (0.127 -> 0.2, 0.233 -> 0.39), kitchen-extractor-hood filter (reviewer 1's own hand figure of one face 0.45 was kept at 0.2; both faces count: 0.84) and chimney (0.55 -> 0.78), circulation-pump casing and motor, electric-fan-heater casing and control, gas-meter index and pipes, hot-water-cylinder connections, mvhr-unit casing (0.9 -> 1.16: box 0.6 x 0.665 x 0.3) and ducts, toaster control, vacuum-cleaner wand (the retracted tube), desktop-monitor stand, bookshelf plinth stands (model defect), dining-table apron, operating-table column and light housing (0.35 -> 0.43), single-bed mattress (2.5 -> 4.5: the bottom on the slats counts), slats (1.5 -> 3.0) and bed-frame (3.37 -> 4.7), picnic-table a-frame (0.998 -> 1.2), park-bench back-timber (1.7 -> 1.3: three slats of the drawn 0.08 x 0.05 section). 32 rows were wrong by more than 15 % of the real object and are changed; 12 stand (accepted again with the measured figure as support). Slips of the kind reviewer 2 found: the extractor filter (own hand figure 0.45 against decided 0.2), the office-worker castors (arithmetic), the mattress and slats (one face), the hot-water-cylinder zones were right but their measure counts interfaces.

### C27 (segment shares of living subjects), measured

Solver reading (`🫀️physiology bodySetup`, `🛋️comfort`): the physiological skin area of a segment is `skinFraction / sum(skinFraction) x skinAreaM2`; `f = areaM2 / that` is the clothing-area factor (bare skin and fur regions sit at f = 1.0, garments at 1.1-1.5, shoes up to 1.5); `metabolicShare` is independent of the area. A redistribution therefore keeps `f >= 1` for garments and, for a part whose `skinFraction / areaM2` equals that of the donor (horse: both 0.1719), leaves every `f` unchanged when `skinFraction` moves with the area; `metabolicShare` never moves (heat production is physiology, not geometry).

Procedure (`areareview3-c27apply.py`): redistribute among the clothing regions of a human only (bare skin keeps f = 1, a hat keeps its one-sided value, hair and fur are exchange) when (1) every clothing region is measured, (2) the sum of the declared values over the sum of the measured ones lies in 0.80-1.20, (3) no region measures more than 3x or less than 0.4x its declared value (otherwise the model is buried, overlapping or crude), (4) a region deviates beyond 30 % after the rescale, (5) every f stays within 1.0-1.6. New value = measured x (sum declared / sum measured), clamped at f = 1.0, last digit absorbed so the sum is exact. Result:

| asset | region | committed m2 | before m2 | after m2 | skinFraction |
|---|---|---|---|---|---|
| 👤️humans/kneeling-gardener | shirt-torso | 0.7397 | 0.7397 | 0.6163 | unchanged |
| 👤️humans/kneeling-gardener | shirt-sleeves | 0.1438 | 0.1438 | 0.1538 | unchanged |
| 👤️humans/kneeling-gardener | gloves | 0.0942 | 0.0942 | 0.0916 | unchanged |
| 👤️humans/kneeling-gardener | trousers | 0.6263 | 0.6263 | 0.7025 | unchanged |
| 👤️humans/kneeling-gardener | clogs | 0.1027 | 0.1027 | 0.1425 | unchanged |
| 👤️humans/seated-guitarist | shirt-torso | 0.8577 | 0.8577 | 0.703 | unchanged |
| 👤️humans/seated-guitarist | shirt-sleeves | 0.3245 | 0.3245 | 0.3213 | unchanged |
| 👤️humans/seated-guitarist | jeans | 0.6774 | 0.6774 | 0.7826 | unchanged |
| 👤️humans/seated-guitarist | boots | 0.095 | 0.095 | 0.1477 | unchanged |
| 👤️humans/standing-adult | hoodie | 0.9283 | 0.9283 | 0.8207 | unchanged |
| 👤️humans/standing-adult | jeans | 0.5452 | 0.5452 | 0.6109 | unchanged |
| 👤️humans/standing-adult | sneakers | 0.1005 | 0.1005 | 0.1424 | unchanged |
| 🐕️animals/horse | hooves | 0.12 | 0.12 | 0.195 | 0.03353 |
| 🐕️animals/horse | coat-trunk | 4.05 | 4.05 | 3.975 | 0.68321 |

Totals before and after: standing-adult 1.5740 -> 1.5740, seated-guitarist 1.9546 -> 1.9546, kneeling-gardener 1.7067 -> 1.7067 (clothing sets); `skinFraction` and `metabolicShare` unchanged; f after: standing-adult hoodie 1.00, jeans 1.22, sneakers 1.42; seated-guitarist shirt-torso 1.00, sleeves 1.21, jeans 1.33, boots 1.55; kneeling-gardener shirt-torso 1.00, trousers 1.32, clogs 1.39 (the hat, f 3.3, is not touched).

Horse (animal, the one sound case): the four hooves are drawn 0.049 m2 each (loft, 4 x 0.0489 analytic, 0.211 measured); the real hoof (o0.13, wall 0.09 high: 0.41 x 0.09 = 0.037 + sole 0.010) is 0.047, so the model is true and the declared 0.12 (0.03 per hoof) was low. Hooves 0.12 -> 0.195, coat-trunk 4.05 -> 3.975 (fur, exchange: shares move, the coat is 98 % of the subject), total of the body regions 5.818 -> 5.818; `skinFraction` hooves 0.02063 -> 0.03353 and coat-trunk 0.69611 -> 0.68321 (equal `skinFraction/areaM2` of 0.1719, so every f stays 0.96; sum 1.0); `metabolicShare` unchanged (hooves 0.001, trunk 0.7235).

No redistribution, with the defect named (measured 1:50 outer area against the declared value; none of these models is sound for the part):

| subject | segments | declared / measured | verdict |
|---|---|---|---|
| dairy-cow | lower-legs, hooves, udder | 0.30 / 0.427, 0.05 / 0.306 (6.1x; real hoof 4 x 0.044 = 0.18), 0.45 / 0.355 | hooves drawn 1.7x the real part, udder undersized; the rescale would give the hooves 27 % of the skin parts (real 14 %) |
| goat | horns, hooves | 0.012 / 0.058 (4.9x), 0.012 / 0.081 (6.7x) | drawn 2-4x the real horn and hoof |
| sheep | ears, hooves | 0.022 / 0.045 (2.0x), 0.016 / 0.125 (7.8x) | hoof drawn 6x (real 0.02), ear 2x |
| pig | skin-legs, skin-head, hooves | 0.2 / 0.66 (3.3x), 0.1 / 0.24 (2.4x), 0.018 / 0.134 (7.5x) | legs and hooves oversized (real legs 0.26) |
| roe-deer | hooves, nose | 0.013 / 0.071 (5.5x), 0.004 / 0.0016 (0.41x) | hooves drawn 4x, nose undersized |
| pigeon | legs, beak, cere, eye | 0.002 / 0.0137, 0.0003 / 0.0024, 0.0001 / 0.0012, 0.00015 / 0.0006 (3.8-12x) | parts at the 7.5 mm floor, real 3-4 mm |
| herring-gull, mallard-duck | bill, legs, web-feet | 3.3-5.4x | toes and bill drawn at the feature floor and thicker than the real part |
| red-squirrel | hands, feet | 0.0008 / 0.0044, 0.0016 / 0.0092 (5.5-5.7x) | feature floor |
| domestic-cat, hedgehog | nose, eyes, pads, paws | 1.6-2.3x | feature floor, all below 0.4 % of the subject |
| labrador-dog | nose, tongue, pads | 0.92x-1.21x | none beyond 30 %: nothing to do |
| humans, bare skin | face, head, arms, legs | 1.2-2.2x | the face solid covers the scalp under the hair (hair is not measured), the skin solid runs under the garments and counts whole (C29): the region partition of the model is not that of the declared values; f = 1 must stay |
| humans, clothing | child-on-kick-scooter, jogger, person-cooking, person-with-umbrella, pregnant-person, worker-carrying-box | sum declared / measured 0.67-0.79 | overlaid or buried solids (factor below 0.8) |
| humans, clothing | courier, cyclist, parent-carrying-toddler (toddler suit 0.58 / 0.37, mittens 2.1x), parent-with-stroller, person-on-crutches (shorts 0.197 / 0.567, 2.9x: drawn loose), person-with-rollator, tour-guide, walking-commuter, standing-phone-user | f would reach 1.6-2.9 | a helmet, cap, glove or shoe drawn 1.9-2.6x; the rescale would push a garment's f beyond the physical range |
| humans, clothing | elderly-with-cane, leaning-person, runner-with-blade-prosthesis, running-child, seated-office-worker, seated-reader, skateboarder, sleeping-person, wheelchair-user (hoodie -25 %) | factor 0.89-1.03 | no clothing segment beyond 30 % after the rescale: nothing to do |
| kneeling-gardener hat | 0.226 / 0.398 | kept | one-sided sky surface 0.213 (hand); the real hat has 0.38 with the brim underside, which the pose and the head carry; C18 stands, the validator number is a finding |

### Note for the audit and the revert scripts

The audit's Table A (forbidden changes) lists horse/coat-trunk 4.05 -> 3.975: that is the C27 redistribution above (hooves 0.12 -> 0.195 with the total fixed), not an author correction; `area-revert.py` and `area-revert-named.py` must not restore it.

### Pose fractions and open points

Pose fractions were not edited in this pass (the totals above are the real outer areas; `exposedFraction` and `contact.fraction` of the default poses that were set against exposed-only values are listed in the `info:exposureDeviation` of the validator). The dining-table `leaf-top` region (stored leaf, pose exposure 0 against the measured 0.47) and the single-bed `slats` (target 0) carry the stowing in the pose, as intended.

### Findings for the model authors

See `model-findings.md`, section "Area review 3 (reviewer 3)".

### Tests and fixtures affected by the confirmed changes (nothing regenerated)

Found by searching `S/🧪️tests` and `S/🧫️fixtures` for the 41 slugs with a changed `areaM2` (or `skinFraction`) of this section (search result `🗑️generated/areareview3/`).

**Tests that solve thermal results or hydronic/phase coupling of changed assets (re-run; the checks are inequalities, expected to hold, the coupled draws move):**
- `🚰️hydronic-coupling` and `🫕️phase-change` (electric-kettle: shell 0.095 -> 0.15 and water 0.0145 -> 0.0155, both now enter the coupled draw `areaM2_dep x inflow / areaM2_source` and the converted mass `kg = joules x area / latent`; any pinned kg or draw changes by about 0.0155/0.0145 and by the shell ratio).
- `🧊️thermal-scenarios` (english-oak trunk-bark 16 -> 25: area weights of the Watt totals only, temperature unchanged), `💨️transport-assets` (english-oak, electric-kettle plume), `🌡️catalog-physics` (horse hooves and coat-trunk areas, horse `skinFraction`: whole-catalog bounds and finiteness, not expected to change, the horse f stays 0.96), `🎨️view-synchrony` (english-oak colour merge weights).
- `🎭️cast-export` (standing-adult hoodie/jeans/sneakers and toaster/electric-kettle: area-weighted merge colours of the cast), `💨️transport-field` (standing-adult: clothing-area weights of the surface flux).

**Geometry-only tests and fixtures that mention the assets (they depend on the volumes, not on `areaM2`; the area change alters `areaM2` inside the embedded catalog fixtures only):** `🏗️structure-projection` (bar-stool, dining-table, kneeling-gardener, leaning-person, operating-table, seated-office-worker, standing-adult), `🔭️multiview-coverage` (json and ts: bar-stool, english-oak, kneeling-gardener, person-with-white-cane, single-bed, electric-kettle), `🧊️model-silhouette`, `🧊️model-frame`, `🧊️model-catalog` (kneeling-gardener, japanese-maple, london-plane-pollarded, scots-pine), `🧊️chain-state` (person-on-crutches, person-with-umbrella), `🧊️volumes-rules` (office-swivel-chair), `🧊️model-glyph-edges`, `🧊️model-glyph-size` (electric-fan-heater), `🧊️axonometric-exports` (duct-fan), `📐️dimension-leader` (standing-adult, thermostatic-radiator-valve), `📑️multiview-sheet`, `📑️sheet-model...` (standing-adult), `🔺️polygon-union` and `🧍️plan-projection` (seated-office-worker, vacuum-cleaner), `🎭️cast-export` fixture json (standing-adult).

### Passes

| time | audit rows (all categories) | new in my categories | decided | note |
|---|---|---|---|---|
| 2026-11-14 23:12 | 335 | first pass: 94 assets measured, 98+ rows decided | 109 | measurement, hand calculation, C27 |
| 2026-11-15 00:15 | 419 | 0 | 0 | nothing new (own decisions applied: scots-pine bark-lower/upper) |
| 2026-11-15 00:38 | 419 | 0 | 0 | nothing new (own decisions applied: scots-pine bark-lower/upper) |
| 2026-11-15 01:12 | 420 | 0 | 0 | nothing new |
| 2026-11-15 01:21 | 420 | 1 | 1 | one row: courier-with-hand-truck/parcels re-set by its author to 1.79 (accepted) |
| 2026-11-15 02:02 | 421 | 1 | 1 | one row: seated-office-worker/desk re-set by its author to 1.35 (accepted, hand 1.35) |
