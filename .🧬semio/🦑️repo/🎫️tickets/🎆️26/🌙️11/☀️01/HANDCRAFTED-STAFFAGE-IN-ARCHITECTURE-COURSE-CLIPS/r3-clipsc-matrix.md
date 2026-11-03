# r3-clipsc: staffage entity matrix, batch C (W16)

A = `🎓️teaching/🏛️architecture`, V = `A/⚡️energy/🏗️authoring/🎨️visuals`, T = this ticket folder. Paths are relative to A. Lines are from the tree on 2026-11-03.

Batch: `🏗️bim`, `📊️excel`, `🦗️grasshopper`, `🏢️nakagin`, `🏙️typology`, `🔀️morph`, `📐️parametric`, `🧲️fit`, `📚️course`, and the non-room-section part of `🎯️optimization`.

Mode legend: DL dark line, DS dark solid, LL light line, LS light solid, T thermal, F forces, M moments, model = domain model geometry (not staffage), diagram = teaching annotation or data drawing.

## 1. Series with no staffage at all

| series | what it contains | staffage entities | check |
|---|---|---|---|
| 🏙️typology | LaTeX assignment, three raster schema PNGs, tests | none (no clip) | `find`: only `📝️assignment`; no `🎬️clip` |
| 📐️parametric | LaTeX assignment, quiz JSON, `icons.json` of metabolism capsule icons | none (no clip). The icons are the Nakagin capsule icon assets of `🧰️framework`, a logo, not staffage | `📐️parametric/📝️assignment/icons.json`, grep for staffage: 0 |
| 🧲️fit | `🐍️.py` (`building_points`, `building_fit`): pure numeric fit of the Nakagin drawing, "without drawing it" | none | `🧲️fit/🐍️.py:1-25` |
| 📚️course | Web player, e2e, builder (TypeScript); the library browser link only | none (no clip) | grep: 0 |
| 🔀️morph | Flight and silhouette animation helpers (`silhouette`, `token`) used by Excel and Grasshopper | none. `silhouette` is the outline of any mobject, `token(kind="item")` a number chip | `🔀️morph/🐍️.py:96` |

## 2. BIM (`🏗️bim`)

| clip | entity | file:line | today | mode chosen | teaching objective | migration |
|---|---|---|---|---|---|---|
| information Beat1 | assumed 1929 car, plan, driving the model path | `🏗️bim/🎬️clip/📊️information/🐍️.py:210-214` (`_car`), calls 247, 342, 471, 705, 777 | cast `bim-information-car` (vintage-car-1929, parked, DL plan, scale 50) | DL plan | Clearance and reach: the car footprint against the pilotis; only the outline and proportions matter, the car is recoloured to the data colour or the warning colour | keep DL plan. Pick the authored scale per drawn size: site plan 0.22 u/m is 1:200, the bay vignettes 0.42 to 0.62 u/m are 1:100. Two ids. OI-118 see section 6 |
| information Beat1 | car in the closest-approach vignette and in the table overview | `📊️information/🐍️.py:342, 471, 705` | same | DL plan | same | `bim-information-car-bay` |
| consistency Beat6 | two editors and the "checked by a person" icon, dark line pictograms recoloured to the actor colour | `🔗️consistency/🐍️.py:273-276` (`person`), calls 1034, 1078 | cast `shared-person-relaxed-line` (standing-adult relaxed, DL elevation) | DL elevation | Who edits what: a role icon, no physics | rename to the BIM-owned id `bim-consistency-actor`, scale 100 (size 0.42 gives 0.35 u/m, 1:137) |
| consistency Beat6 | the decider | `🔗️consistency/🐍️.py:1105` | `bim-consistency-decider` (arms-crossed, DL elevation) | DL elevation | A person decides, a role icon | scale 100 (0.50 u/m, 1:96) |
| exchange Beat5 `_beat_buildup` | three standing people on the first-floor slab, "imposed load: people and furniture" | `🔌️exchange/🐍️.py:875-876` | `shared-person-relaxed-line`, DL, coloured DATA, scale 50 | DL now. The objective is the imposed load, so the canonical load representation is F (weight force at the feet) | Build the area load kN/m2 from self-weight, finishes and imposed load | W07 owns the force overlay. I only fix the scale (1.35 u/m, 1:35, scale 50) under the own id `bim-exchange-person`. Request to W07 in `r3-requests/clipsc.md` |
| exchange Beat5 | structural check of the Savoye frame: supports, loads, moments in `🧮️analysis` | `🎨️visuals/🧮️analysis/🐍️.py:178` (`_support_glyph`), `🔌️exchange` structural beats | diagram drawn by the BIM analysis, not staffage | F and M by W07 | Structural check: supports, N/V/M | owner W07 (the library solver and `structural-view` exist; the manifest holds 0 force or moment casts). No edit by W16 |
| objects, representations, models, exchange, consistency | the Villa Savoye teaching model: pilotis, slabs, walls, windows, curtain wall, stair, ramp, garden, gravel, boiler BL-1, radiator RAD-1, downpipes | `🏗️bim/🎬️clip/🧱️model/🐍️.py` (2294: `IfcBoiler`, `IfcSpaceHeater`), `🧩️models/🐍️.py:535-570` | IFC objects with GlobalIds, quantities and stated uncertainties, drawn from model geometry | model | The subject itself: the BIM model and its information. Boiler and radiator are IFC occurrences with envelopes ("equipment envelopes retain explicit uncertainties") | stays model geometry. A generic cast would drop the modelled envelope and its identity. Recorded for the coordinator, not staffage |
| all six | roof garden, gravel, hanging garden hatch | `🧩️models/🐍️.py:545-566`, `🧱️model` (`SP-TERRACE`) | model space and hatched area | model | Spaces and areas of the model | stays model |
| information, consistency | diagram glyphs (pointer, route, badge, chip), tables | many | diagram | diagram | annotations | no change |

Guard coverage of the series: `V/🧪️tests/🔣️staffage_only.json` allows `🏗️bim/…/📋️plan/🐍️.py::_plan_ramps` as "structure: IFC ramp footprints of the Savoye floor plan, drawn from model geometry".

## 3. Excel (`📊️excel`)

| clip | entity | file:line | today | mode chosen | teaching objective | migration |
|---|---|---|---|---|---|---|
| program Beat3 | one occupant per capsule plan, standing, seen from above | `📊️excel/🎬️clip/📋️program/🐍️.py:59-73` (`occupant_spots`), `:340-368` (`plan`, `occupant`), calls via `plan(people=True)` 426, 613 and `small_plans` 397 | cast `excel-program-occupant` (standing-adult relaxed, DL plan, scale 50), yawed by -90 degrees | DL plan | Free floor = net floor minus fittings; the occupant is a typed-in constant (one person) | scale per drawn size: big plan 0.82 u/m is 1:58 (scale 50), small plans 0.42 u/m is 1:114 (scale 100). Two ids `excel-program-occupant`, `excel-program-occupant-small`. OI-47 see section 6 |
| program Beat3 | capsule fittings bed, bath, storage, desk, sofa as hatched rectangles with dimension and area | `📋️program/🐍️.py:36` (`FITTING_COLORS`), `:328-390`, dimensions 429, 447 | rectangles `plan.fits[index]` | diagram (data rectangle: the cell values B/C of the type sheet are the width and length, the hatch is the area calculation) | Area = width x length x count, filled down; the rectangle is the cell | no change. A plan cast of the bed, desk and sofa exists (`🛋️furniture/single-bed`, `height-adjustable-desk`, `three-seat-sofa`, all with plan geometry) but the catalog has no plan bath unit and no built-in storage, and a cast cannot be fitted to a typed-in cell value without contradicting the rectangle. Gap forwarded, see section 6 |
| program Beat3 | capsule shell, net floor, window on the front wall in the plan | `📋️program/🐍️.py:340-365` | drawn from the type dimensions of the Nakagin model | diagram (area quantities) | Net floor area from the model numbers | no change |
| anatomy | live capsule with fittings, hatch, perimeter | `📊️excel/🎬️clip/🦴️anatomy/🐍️.py:149-191` | Nakagin model capsule plus area rectangles | model plus diagram | Functions SUM, ranges, errors | no change |
| references, energy | Nakagin capsule and tower axons, heat-flow arrows proportional to UA | `📊️excel/🎬️clip/📍️references/🐍️.py`, `⚡️energy/🐍️.py:269-330` | Nakagin model drawing plus arrows | model plus diagram | Cell references; heating load phi = UA x delta T | no change. The arrows are the lesson quantity; the thermal view of a capsule asset does not exist |

## 4. Grasshopper (`🦗️grasshopper`) and Nakagin (`🏢️nakagin`)

| clip | entity | file:line | today | mode | owner | note |
|---|---|---|---|---|---|---|
| all five | the Nakagin tower, capsules, cores, capitals, podium, bridges | `🏢️nakagin/🐍️.py`, `🎨️visuals/🐍️.py` (`NakaginDrawing3D`) | source-grounded reconstruction (CAD placement, measured scans, sourced skins) | model | W16 | The reconstructed building is the subject of the clips. Not staffage |
| capsule | capsule interior: floor bed, drawers, wetroom (tub, basin, toilet), media cabinet, TV, folding desk, shelving, lamp, sprinkler/detector | `🏢️nakagin/🐍️.py:2240-2300, 2323-2352, 2449-2567` | sourced 3D parts with named supports (39 discrete supports) inside the capsule assembly | model | W16 (report), W11 | The reference interior is a documented reconstruction ("interior=reference") with contact and clearance tests. Same ruling as the capsule shell. Recorded; W11 owns only the podium |
| tower | podium reception, cafe, office furniture | `🏢️nakagin/🐍️.py:1419-1441` (`_podium_room_parts`), `🎨️visuals/🐍️.py:1675` | boxes inside the building model, used for occlusion and tests | model | W11 (OI-46) | Not touched by W16 |
| casting, trees, anatomy | data trees, Grasshopper components, wires, panels, previews of capsule or tower | `🦗️grasshopper/🎬️clip/*/🐍️.py` | `present_trees` is a data diagram, not a tree | diagram | none | `🦗️grasshopper/🎬️clip/🌳️trees` draws abstract data trees; no vegetation |
| model | `window` of the CAD solid model | `🦗️grasshopper/🎬️clip/🧱️model/🐍️.py::window` | CAD window geometry | model | none | guard allows it as structure |
| Nakagin tests | podium, roof, occlusion, detail, survey | `🏢️nakagin/🧪️tests/*` | tests of model geometry | model | none | |

No clip of these two series draws a person, tree, car, animal or machine.

## 5. Optimization (`🎯️optimization`)

The six scenes (extrema, sampling, experiments, index, pareto, algorithms) are one running example, the seminar room section, drawn by `RoomVisuals.room_section` (`🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py:571`). That module, the occupant, radiator, lamp, cooler and window casts, belong to W15 (optimization comfort). The matrix lists them for completeness and does not edit them.

| scene | entity | file:line | cast / today | mode | owner |
|---|---|---|---|---|---|
| room_section (all) | seated or standing occupant | `🏠️room/🐍️.py:587`, `🧪️experiments/🐍️.py:276` | `optimization-room-occupant` (T) | T | W15 |
| room_section | radiator | `🏠️room/🐍️.py:588` | `shared-radiator-winter` (T) | T | W15 |
| room_section | pendant lamp, off and on | `🏠️room/🐍️.py:1336-1340` | `shared-lamp-off-neutral`, `shared-lamp-neutral` (T) | T | W15 |
| room_section | split cooler | `🏠️room/🐍️.py:1349-1350` | `optimization-room-cooler-mirrored` (T) | T | W15 |
| room_section | wall, slab, insulation, glass, overhang as section primitives | `🏠️room/🐍️.py` (`build`), allowed in `🔣️staffage_only.json` rule_c | structure | W11 / W15 | W15 |
| extrema | cooler, radiator, lamp pictures | `📉️extrema/🐍️.py:440` | three casts at 0.6 m | T | W15 |
| experiments | occupant and window icons | `🧪️experiments/🐍️.py:905-906` | `optimization-experiments-occupant`, `optimization-experiments-window` | T | W15 |
| **index** | lighting criterion pictogram (electricity) | `⚖️index/🐍️.py:127-136` (`glyph`), 15 call sites (423, 860, 1078, 1172, 1181, 1306, 1386, ...) | cast `shared-lamp-neutral` (thermal, scale 50) at 0.26 to 0.36 m widths; the other criteria use `icon_glyph` in the criterion colour | **DL** pictogram | **W16** |
| pareto, algorithms | none of their own (room section only), tables, charts, `cross_mark` | `🏆️pareto/🐍️.py`, `🧬️algorithms/🐍️.py` | diagram | diagram | none |

Index finding: the pictogram is a thermal cast, so it ignores the `color` argument (calls at 1386 pass `WARN`) and shows a heat colour next to a purple sun glyph and a grey factory glyph. A pictogram is a dark-line icon; a line cast recolours to the criterion colour. Migration: new cast `optimization-index-lamp`, pendant-lamp, pose `on`, DL elevation, scale 100 (unit 0.46 to 0.63 u/m is 1:105 to 1:76).

## 6. Open items of this batch

| id | state |
|---|---|
| OI-118 | The car clearance numbers (`clearance`, `car_path`) call `building_geometry`, which does not finish (OI-01, more than 10 minutes in the probe `🧰️r3-clipsc/car_probe.py`). The car placement in the full scene cannot be rendered. The bay-scale and vignette stills are the evidence. Blocked by OI-01, owner BIM model |
| OI-47 | Seated and lying plan postures need W08. The catalog today holds `sleeping-person` (supine, side, curled) and `seated-office-worker` with plan geometry, but both are positioned on a bed or a chair, which are fittings. The clip counts the occupant on the free floor, so a standing occupant stays correct until W08 delivers a plan posture next to a fitting. Request in `r3-requests/clipsc.md` |
| OI-46 | W11 (podium). The capsule interior is ruled model geometry (section 4) |
| exchange load | W07: forces view of a person for the imposed load, plus structural check overlay |
| catalog gap | plan view of a wetroom unit and of a built-in storage cabinet (Nakagin fittings `bath`, `storage`); owner W10 |
