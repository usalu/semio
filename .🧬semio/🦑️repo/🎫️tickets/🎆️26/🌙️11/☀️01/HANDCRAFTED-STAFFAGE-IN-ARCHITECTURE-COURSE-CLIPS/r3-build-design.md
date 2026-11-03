# r3-build design: building components in the course clips

W11 (`build`), round 3. A = `🎓️teaching/🏛️architecture`, S = `A/🧍️staffage`, V = `A/⚡️energy/🏗️authoring/🎨️visuals`. All line numbers are from the tree of 2026-11-03.

## 1. Rule used to split staffage from diagram

| Class | Meaning | Examples |
|---|---|---|
| Staffage (depicted component) | A real building element a viewer could buy or build: it has dimensions, materials, a U-value, a surface temperature. It must be a library cast. | window sash, glazing unit, door leaf, blind, chimney, radiator, wall build-up, roof build-up, stair |
| Section structure (diagram) | The cut envelope of the lesson's room or house, drawn once in the unique `section_material` and carrying the thermal tint or flow arrows of the lesson. It is an annotation of the lesson geometry (`room_section`, `house_section`, `_stage`). The guard already lists these as `structure:` allowances. | cut slab, cut wall strip, roof band, ground hatch |
| Teaching diagram | Arrows, dimension lines, layer stacks drawn as a data chart, parametric façade charts, envelope outline polygons, ray geometry. | `_envelope` (recap:182), optimization `facade()` (room:837), `_roof` point-and-normal helper (recap:173) |

Rule applied to the shared section constructors: the cut material that *bounds* a room is structure; anything *inside the opening* or *bolted to the wall* is a component. `window_glyph` therefore is a component (it draws a sash with a frame and a mullion inside the opening), `house_section` / `room_section` walls are structure.

## 2. Inventory of ad hoc building components (file:line)

| # | Site | What is drawn from primitives | Class | Target asset |
|---|---|---|---|---|
| 1 | V `🐍️.py:838` `window_glyph`, `:865` `window_opening`, `:890` `open_window` | oblique sash polygon, mullion line, reveal blocks | staffage (sash) + structure (reveal) | `tilt-window` (closed, tilted-open, fully-tilted) for sash; reveal stays section |
| 2 | V `:899` `house_section` (calls `window_glyph` twice at `:~960`) | two windows per house | staffage (windows) | `tilt-window`, see 1 |
| 3 | ventilation `❓️need/🐍️.py:127` `_window`, `:129` `window_glyph`, `:257-258`, `:627`; `🪟️natural/🐍️.py:124`, `:126`, `:229`, `:349`, `:805-806`, `:1005`, `:1293-1294`, `:1445`; `📦️volume/🐍️.py:256`; `⚙️mechanical/🐍️.py:786`; `🌡️heat-flow/🐍️.py:150-157` | open/close windows animated by `Transform` of a polygon | staffage | `tilt-window` poses and clips `tilt-open-close` |
| 4 | cooling `💧️transmission-humidity:462`, heating `🌬️convection:204`, `🧮️calculation:280`, `:396`, `:478`, `:1224`; demand `🔁️recap:492`, `:657`, `:973`; `⚖️regulations:372` (`_north_window`, allowed); physics `🧲️fundamentals:1290`, `:2613`, `:2622`, `:2637` | same sash glyph | staffage | `tilt-window` |
| 5 | heating `🧱️conduction/🐍️.py:319-340` `_envelope_house` | front door as `window_glyph` with the mullion hidden, door top at 0.62 s | staffage | new `entrance-door` |
| 6 | heating `☀️solar-gains/🐍️.py:585-600` `_glazing`, used `:628` | two glass panes (Rectangle) and two frame blocks in section | staffage | new `insulating-glazing-unit` |
| 7 | heating `🧱️conduction/🐍️.py:93-100` `_wall_layers`, `:113-115` `WALLS`, `:234` `_slab` | wall layer stack (plaster, solid brick, ETICS 0/12/30 cm, render) as coloured layers and a 1 m x 1 m oblique tile | staffage (the wall build-up) | new `insulated-wall-buildup`, poses `uninsulated`, `insulated-12`, `highly-insulated-30` |
| 8 | heating `🧲️fundamentals/🐍️.py:108-120` `_brick_wall`, `:1354-1410` `_wall_section` | section wall with a thermal tint overlay; layer section for the lesson | section structure + staffage layers | `insulated-wall-buildup` for the layered case, `_brick_wall` stays structure |
| 9 | demand `♨️system-losses/🐍️.py:324-330` `_chimney`, used `:470` | two vertical lines | staffage | new `flue-chimney` |
| 10 | V `:783` `radiator_wall` | a 0.15 m cut wall strip behind the radiator cast | structure (scene wall, ruling OI-34) | none, stays |
| 11 | demand `🔁️recap/🐍️.py:173` `_roof`, `:182` `_envelope` | roof slope point and normal, dashed envelope outline | teaching diagram | none |
| 12 | V `house_section` roof (`:~925`) | roof as one cut polygon | section structure; the build-up behind it is a component | new `pitched-roof-section` for roof-layer lessons (heating conduction `ENVELOPE` "Dach", `:121`) |
| 13 | optimization `🏠️room/🐍️.py:837` `facade` | WWR façade elevation: wall rectangle, glass rectangle, mullions | teaching diagram (parametric chart of the design variable `window_area`) | none (stays, parametric) |
| 14 | physics `🧲️fundamentals/🐍️.py:432` `_blind`, `:2219`, `:2260`; cooling `🌤️summer-strategy:107` `_blinds`; heating `☀️solar-gains:826` `_blind_slats`, `:982` | blinds | already casts of `external-venetian-blind` | none |
| 15 | grasshopper `🧱️model/🐍️.py:258` `window`, bim `🏛️savoye/*` walls, slabs, stairs, curtain walls | solid CAD or IFC geometry of the model being taught | model data, not staffage | none |
| 16 | nakagin `🐍️.py` podium and capsule furnishing | see section 3 | model data | see section 3 |

Items with no ad hoc component after the scan: typology, morph, parametric, fit, course, excel (UI), the BIM and Grasshopper series (model geometry).

Components that exist and need no new asset: tilt window (sash, glass, frame), external venetian blind, panel radiator, masonry wall with lintel (structural cavity wall, elevation), sliding door, curtain wall bay, stair flight, spiral stair, sectional garage door, canopy, balustrade.

### New assets (all `🧱️components`, handcrafted, `elevation` authored at 1:50, 1:100, 1:200, 1:500 unavailable)

| id | what | U or thermal numbers | views | poses |
|---|---|---|---|---|
| `insulating-glazing-unit` | 4-16-4 low-e argon unit between a head and a sill frame block, section | U_g 1.1 W/m2K; emissivity glass 0.837, low-e coating 0.04; warm inner pane under the winter case | elevation (section) | `winter`, `summer-sun`, `shaded` |
| `insulated-wall-buildup` | 1 m high section through the 1960 solid brick wall: 15 mm plaster, 365 mm brick, ETICS (0, 120, 300 mm), 20 mm render | U 1.53, 0.24, 0.11 W/m2K (EN ISO 6946, R_si 0.13, R_se 0.04) | elevation (section) | `uninsulated`, `insulated-12`, `highly-insulated-30` |
| `entrance-door` | 1.0 x 2.1 m insulated timber door leaf in a frame, front elevation | U_D 1.3 W/m2K; emissivity timber 0.9 | elevation (front) | `closed`, `ajar` |
| `flue-chimney` | masonry chimney stack with stainless liner, section | flue gas 160 degC in operation, standby 25 degC; outer surface from the scenario | elevation (section) | `standby`, `operating` |
| `pitched-roof-section` | rafter roof strip 1 m wide: tiles, battens, counter-battens, underlay, rafters with mineral wool, vapour barrier, plasterboard | U 0.20 W/m2K | elevation (section) | `insulated`, `uninsulated` |

Surface temperature comes from the scenario: each region is `conditioned` (interior 20 degC, `interiorFilmM2KW` 0.13) and the thermal solver evaluates it against the environment of the cast, so the same asset reads 19 degC inside on the highly insulated wall and about 12 degC on the uninsulated one in the winter reference case.

## 3. OI-46: Nakagin furnishing

Where the furnishing lives (`🏢️nakagin/🐍️.py`):

| Part | Lines | Content |
|---|---|---|
| podium rooms `_podium_room_parts` | 1388-1445 | reception desk, hall benches, cafe tables with legs, chairs, counter, office desks, chairs, filing as `_box` and `_cylinder` solids with roles `reception_desk`, `hall_bench`, `cafe_table`, `cafe_chair`, `cafe_counter`, `office_desk`, `office_chair`, `office_storage` |
| capsule reference interior `_reference_interior` | 2440-2518 | floor bed with drawers, wetroom, media wall, folding desk, handles: built-in joinery from the Domus 520 photographs |
| capsule study interior `_study_interior` | 2520-2590 | bed, bath, storage, desk, sofa from the authored `fittings` widths and lengths of `🔣️.json` |
| visuals `🎨️visuals/🐍️.py` | 745-755, 1674-1717, 1800 | `_cap_parts` fetch the parts, `capsule_axon` draws every part with the painter's algorithm, `roles` drive highlights |

Findings that bound the replacement:

1. These are 3D `Solid` meshes in a standard-library geometry model. The catalog assets are multi-view drawings (side elevation, front, plan) of rigged parts; they have no 3D solid export, so a library cast cannot take part in the painter's algorithm or in the fragment kernel that decides visibility (`🧪️tests/👁️occlusion`, 7 golden cases with `pillow` flags).
2. The capsule furniture is part of the verified reconstruction: 39 explicit supports, rounded-lining fitting (`_fit_reference_cavity`), entrance clearance, non-overlap and headroom tests (`🧪️tests/🐍️.py:663-665`, `:864`), and the podium test checks every role (`🧪️tests/🏬️podium/🐍️.py:86`). The bed, drawers, media wall and wetroom are built-ins; they are building fabric, not staffage.
3. The only freestanding furniture is the podium cafe and office set (about 40 solids, assumed placement, seen through glass partitions at overview scale 0.1) and the study programs (parametric footprints for the energy studies).
4. Sizes differ from the canonical assets (cafe chair seat 0.38 m against 0.44 x 0.50 m, desk 1.2 x 0.64 m against 1.6 x 0.8 m), so a swap would change footprints and the golden clearance cases.

Decision (kept apart from the owner rulings of round 1 and 2, which kept the boxes): the building model stays the source of its own 3D parts; the furniture is not replaced by 2D casts. What round 3 adds is a provenance link between the model and the library, stated in the report. Alternatives rejected: a billboard cast in the painter's algorithm (wrong occlusion, breaks golden cases), a TS `solid3d` export (new shared contract, owner W05) and regeneration of the golden cases (removes the independent oracle).
