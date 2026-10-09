# r9 audit: BIM plugin feature completeness

Read-only audit by `r9-audit-completeness`. Scope: the plugin `✏️s/🔌️plugins/🏙️bim` against the core feature sets of Revit, ArchiCAD, Vectorworks and BricsCAD BIM.
Nothing in the repo was modified. No build or test was run in this pass; statements marked **(reported)** come from the r4-r8 execution reports and were not re-verified here. Statements marked **(grep)** were checked against the sources in this pass.

## 0. Aliases and sources

| Alias | Path |
|---|---|
| `P` | `✏️s/🔌️plugins/🏙️bim` |
| `S` | `P/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any` (the artifact subset; all `🧬️schema`, `✏️editor`, `🚪️io` paths below are relative to `S`) |
| `G` | `🧰️framework/🔨️modules/📐️geometry` (framework geometry: `🦴️skeleton`, `🏠️roof`, `🕸️mesh`, `🔪️section`) |
| `H` | `🌎️hub/🧩️compositions/🏙️bim` |

Read: `r2-design.md`, `📓️coordination.md`, the open-issue and limit sections of all 18 `r4-exec-*` reports, `r5-exec-u-tools.md`, `r5-exec-x-gltf-svg.md`, `r5-audit-inferences.md` (headers and P1 rows), `r6-exec-z-mutations.md`, `r7-exec-z-depth.md`, `r7-exec-z-codecs.md`, `r7-depth-inference-handoff.md`, `r8-exec-z-baseline.md`, `r8-exec-z-close.md`. `r5-audit-mutations.md` and the r1 explorations were not read in full.

## 1. Inventory of what exists

- **Hierarchy and entities** (`S/🧬️schema/📸️snapshot/🧱️entities/🦀️.rs`, `💠️values/🦀️.rs`): Project, Site, Building, Storey (`level`, `height`, `cut_height`), GridLine, Wall (layered type, `Axis` line or single arc, `LocationLine`, `TopConstraint`, `Phase`), CurtainWall, Column, Beam, Slab (boundary, holes, slope), Roof (5 shapes), Opening (window / door / void hosted in a wall), Stair (4 flight kinds, authored stringer/nosing/riser/landing), Railing (path, posts, baluster, infill), Space, Classification, Material, WallType / SlabType / RoofType (layers), ColumnType / BeamType (`Profile`), WindowType / DoorType. Element collections: 14 (walls, curtain_walls, columns, beams, slabs, roofs, openings, stairs, railings, spaces, grids, sites, buildings, storeys).
- **Mutations**: 88 kinds with a leaf (`🦠️mutation`, `🔺️diff`, `↩️inverse`, tests, fixtures), plus 7 helper directories (`🌊️cascade`, `📍️placement`, `🦉️wall-geometry`, `🧵️elements`, `🧰️kit`, `🧿️horizontal-rules`, `🧪️tests`). Sum-law tests on 88/88 leaves **(reported, r6)**.
- **Inference fields** (`S/🧬️schema/💡️inferences/`): 12 directories: `🏢️storey-levels`, `🧱️wall-layout` (with `🔗️joins`), `🪞️curtain-layout`, `🪟️opening-frames`, `🧊️element-solids` (walls, curtain walls, fillers, columns, beams, slabs, roofs, stairs, railings), `📦️bodies`, `🏠️spaces`, `🧮️quantities`, `🪜️stair-runs`, `🗺️plan-linework`, `⚠️diagnostics` (about 63 codes, incl. `💥️clashes` with 12 pairwise codes), `🕸️model-graph` (session + compute, **(grep)**).
- **Editor**: 19 utilities (select, move, rotate, wall, arc wall, curtain wall, column, beam, slab, slab-walls, roof, window, door, opening, stair, railing, space, grid, measure) in `✏️editor/🪛️utilities`; 15 command files in `✏️editor/🎮️commands`; windows plan, section, world (3D), schedule in `✏️editor/🎭️modes/✏️edit/🪟️windows`; panels outliner, properties, library in `✏️editor/📌️panels`; undo/redo in `✏️editor/🦀️.rs` **(grep)**; presence `✏️editor/👥️presence`; window transient `✏️editor/🫧️transient`.
- **Viewer**: `👁️viewer` (3D + plan, camera commands only) **(reported, r4-exec-u-viewer)**.
- **IO**: pack (binary) and DSL text (`🚪️io/💾️binary`, `🚪️io/📝️text`); IFC2x3 export and import (`🚪️io/📤️export/🏗️ifc`, `🚪️io/📥️import/🏗️ifc`); glTF 2.0 export (`🚪️io/📤️export/🧊️gltf`); SVG 1.1 plan export (`🚪️io/📤️export/🎨️svg`).
- **Examples**: `🏡️house` (30 walls, 35 openings, 12 spaces) and `🏢️office` (96 columns, 152 beams, 8 curtain walls); both replay through mutations **(reported, r4-exec-x-examples)**.
- **Registration**: hub composition `H`, bridge `P/🏭️bridge`, oracles `P/🔮️oracles` (Python shapely / ifcopenshell / lxml, TS three).

## 2. Verdict

The plugin is a sound parametric core: storey-driven heights, layered walls with arcs and joins, hosted openings, the QTO, IFC2x3 round-trip, and 2D/3D views are real and tested. It is not yet a "full-blown" BIM authoring tool. Major families are missing (ceilings, ramps, furniture, MEP placeholders, zones, reference planes, dimensions, sheets, elevations, design options, worksets, BCF, energy and structural hooks). Several parameters that the UI lets the user edit are stored but ignored by inference (section 4.1). Progress and cancellation are not implemented for expensive work (section 4.3).

Counts over the 56 areas graded below: **Present 15, Partial 23, Missing 18**.

## 3. Grading by area

Grades: **Present** = authored record, mutation(s), inference or query, UI and IO exist end to end for the core behaviour of mainstream tools. **Partial** = exists with a major gap listed. **Missing** = no kind, entity, inference or UI found.

### 3.1 Spatial hierarchy, levels, grids, constraints

| # | Area | Grade | Evidence | Gap versus mainstream |
|---|---|---|---|---|
| 1 | Spatial hierarchy (project > site > building > storey > element) | Present | `🧱️entities/🦀️.rs` (Project, Storey, Building, Site); references by id (`storey`, `building`, `site`, `host`) | No zones, groups or assemblies. No mutation moves an element to another storey (no `set-*-storey` kind, **(grep)** on the 88 kinds). |
| 2 | Levels (storeys) | Present | Storey `level`, `height`, `cut_height`; elevation inferred in `🏢️storey-levels`; kinds `create-storey`, `set-storey-height`, `set-storey-level`, `set-storey-cut-height`, `rename-storey`, `delete-storey` (now cascades, r6) | `cut_height` is not used by the plan (`🗺️plan-linework/🦀️.rs:45, 330, 339` still `CUT_HEIGHT = 1.2`). No non-storey levels (roof or ceiling levels). |
| 3 | Grids | Partial | GridLine (`label`, `start`, `end`, `building`); kinds `create-grid-line`, `set-grid-line`, `delete-grid-line`; grid tool and snapping in `✏️editor/🧵️gestures/🧲️snap` | No bubble or visibility control per storey, no arc grids, no grid-driven element creation beyond snapping. |
| 4 | Reference planes | Missing | No entity, kind or inference (**grep**: no hits) | Entire feature. |
| 5 | Parametric constraints | Partial | Parametric by inference DAG: storey to wall layout to opening frame (`🧱️wall-layout`, `🪟️opening-frames`); `TopConstraint` (Unconnected, StoreyTop, Storey); parametric tests `🧪️tests/🔗️follows-by-inference` in `🧬️mutations/*` **(reported, r4-exec-m-walls)** | No geometric or dimensional constraints (align, equal, distance, lock), no solver, no locks. Only hosted and storey-driven dependencies. |

### 3.2 Element families

| # | Area | Grade | Evidence | Gap versus mainstream |
|---|---|---|---|---|
| 6 | Walls, layered | Present | `WallType.layers` with `LayerFunction`; kinds `create-wall-type`, `set-wall-type`, `set-wall-type-of`; per-layer solids in `🧊️element-solids/🧱️walls` | No compound-structure wrap rules or layer-specific end conditions. |
| 7 | Walls, curved | Present | `Axis::Arc { bulge }`; `set-wall-axis`; arc tool in `🧱️chain` | One arc per wall; polycurves need `split-wall`. |
| 8 | Walls, profiles (non-rectangular, sloped, stepped) | Missing | Wall has no profile; `Profile` only on columns, beams, railings and mullions | Stepped or tapered walls. |
| 9 | Walls, sweeps and reveals | Missing | Reveal depth is derived from host thickness (`🪟️opening-frames/🦀️.rs:354`); reveal solids in `🧊️element-solids/🧱️walls` tests. No sweep kind, no authored reveal | Baseboards, crowns, reveal sizes and materials as authored data. |
| 10 | Walls, top attach (roof, slab, ceiling) | Partial | `TopConstraint` has only Unconnected, StoreyTop, Storey (`🛰️.proto`, `🔗️.graphql`) | No attach to roof, slab or ceiling underside; roof-trimmed walls are impossible (r4-exec-x-examples). |
| 11 | Walls, phase | Partial | `Phase` enum (`💠️values/🦀️.rs:39`) stored on walls; set at creation only; properties show it read-only (`✏️editor/🧩️entities/🦀️.rs:362`) | No `set-*-phase` kind, no phase filter, no phase-based QTO, no phase on other element kinds. |
| 12 | Curtain walls | Partial | `CurtainWall` (uniform `u_spacing`, `v_spacing`, mullion `Profile`, panel and mullion material); `create-curtain-wall`, `set-curtain-wall`; `🧊️element-solids/🪟️curtain-walls` | No panel types, no per-cell overrides (door or solid panels), no curtain-wall type library, panels not cut by hosted openings (r4-exec-i-solids-walls, item 6). |
| 13 | Columns | Present | `Column` + `ColumnType` (`Profile`, material), rotation, `TopConstraint`; `create-column` and `set-column` use `placement::rise` (pure sum, r6) | No inclined columns, no automatic placement on grid intersections, no layered or composite columns. |
| 14 | Beams | Partial | `Beam` start and end (straight only), signed `top_offset`; `create-beam`, `set-beam`, `set-beam-type`; sweep in `🧊️element-solids/➖️beams` | No arc or polyline beams, no inclined beams, no beam-column joins or framing layout. |
| 15 | Slabs | Present | `Slab` boundary loop + holes, `offset`, `slope`, layers; `create-slab`, `set-slab-boundary`, `set-slab`, slab tool and slab-from-walls tool | No edge profiles or edge types, no per-edge slope. |
| 16 | Roofs | Partial | Roof footprint and shape (Flat, Shed, Gable, Hip, Mansard), `overhang`; `create-roof`, `set-roof-footprint`, `set-roof-shape`, `set-roof-type` | Hip, gable and mansard exist only over convex footprints; curved or concave footprints fall back to a flat roof (`roof.fallback-flat.*`, `🏠️roofs/🦀️.rs`). The framework straight skeleton (`G/🦴️skeleton`, `G/🏠️roof`) is not wired into the roof inference (r7 handoff section 4, **(grep)**). No wall-roof trim, no fascia or gutter. |
| 17 | Stairs | Partial | Flights Straight, LTurn, UTurn, Spiral; riser maths in `🪜️stair-runs`; code flags (2R+T) in `⚠️diagnostics` (`StairComfort`) | Authored stringer, nosing, tread thickness and landing depth are ignored by the solids: `🧊️element-solids/🪜️stairs/🦀️.rs:6` says "Stringers and tread nosings are not modelled". No railing hosting. |
| 18 | Railings | Partial | Path, height, post spacing, profiles, baluster, infill are authored; `create-railing`, `set-railing`, `delete-railing` | Section sizes are still constants (`POST_SIZE`, `RAIL_WIDTH`, `RAIL_DEPTH` at `🛤️railings/🦀️.rs:20-24`), so the authored profiles are ignored. No hosting to stairs, ramps or slab edges. |
| 19 | Ramps | Missing | No kind or entity (**grep**: no hits) | Entire feature, including slope code checks. |
| 20 | Ceilings | Missing | No element kind. `🏠️spaces` takes the clear height from a slab found by `ceiling_of` (`🏠️spaces/🦀️.rs:196`) | Ceiling as element, ceiling type, ceiling tool. |
| 21 | Floors and finishes | Partial | Finishes are a `LayerFunction::Finish` inside slab and wall types; no finish assigned to a room | Room finish assignments, finish schedule, finish areas. |
| 22 | Doors and windows: families and types | Partial | `WindowType` and `DoorType` (width, height, sill, frame, panes, leaves, swing, material); `create-window-type`, `set-window-type`, `create-door-type`, `set-door-type`, delete kinds | No parametric families, no hardware, no frame or glazing layers, no panel layers. Types are fixed records. |
| 23 | Doors and windows: placement | Present | `Opening` (host, `offset`, `sill_override`, `width` and `height` overrides, `flip_hand`, `flip_facing`); `create-opening`, `set-opening`, `move-opening`; frames and fillers in `🪟️opening-frames` and `🧊️element-solids/🚪️fillers` | No openings in slabs or roofs (slab holes exist but are not opening kinds). |
| 24 | Furniture and equipment | Missing | No kind (**grep**). The IFC importer lists `IFCFURNISHINGELEMENT` only as an unsupported class (`🚪️io/📥️import/🏗️ifc/🏛️spatial/🦀️.rs:150`) | Entire feature. |
| 25 | MEP placeholders | Missing | No kind, no IFC distribution export | Entire feature. |
| 26 | Zones | Missing | No entity | Entire feature. |
| 27 | Spaces and rooms | Partial | `Space` (number, name, boundary `Bounded` seed or `Explicit`, usage string); `🏠️spaces` gives area, perimeter, net floor area, clear height, volume, ceiling slab; `create-space`, `set-space`, `delete-space`; IFC `IfcSpace` export | No zones, no finishes, no occupancy or conditions, ceilings limited to slabs. |
| 28 | Areas (area schemes: GFA, NSA, rentable) | Missing | Only per-space numbers | Area schemes and their rules. |

### 3.3 Views and documentation

| # | Area | Grade | Evidence | Gap versus mainstream |
|---|---|---|---|---|
| 29 | Plan view | Present | `🗺️plan` window (Canvas2d, `Viewport2d`, cut height in window config); `🗺️plan-linework` (poché, openings with swings, columns, stairs with arrows, space tags) | Cut height is not the authored storey value (section 3.1 row 2). No view range, no view-specific visibility or overrides. |
| 30 | Section view | Present | `📐️section` window; section line and storey-top handle (`set-storey-height`) | The section line lives in window config, not as an authored view element. |
| 31 | Elevation views | Missing | Window kinds are only plan, section, world, schedule (`✏️editor/🎭️modes/✏️edit/🪟️windows/`) | Elevation views as views. |
| 32 | 3D view | Present | `🧊️world` window (orbit, section box, storey isolation); `👁️viewer`; glTF export | No walkthrough, no render settings, no 3D annotation. |
| 33 | Schedules | Partial | `🧮️schedule` window renders a fixed table of quantities (**grep**: `rows`, `render` in `🧮️schedule/🦀️.rs`) | No user-defined schedules: category, fields, sort, filter, group, totals. No door, window or room schedules. |
| 34 | Sheets | Missing | SVG export is one plan per storey (`🎨️svg`); no sheet, title block or viewport | Entire feature. |
| 35 | Annotation (text, symbols, leaders) | Missing | Plan texts for space tags and storey labels only (`🗺️plan-linework`) | Entire feature. |
| 36 | Dimensions | Missing | No kind or inference (**grep**: no hits) | Entire feature. |
| 37 | Tags | Partial | Space tags in `🗺️plan-linework` | Element tags (door, window, wall marks) missing. |

### 3.4 Coordination, phasing, lifecycle

| # | Area | Grade | Evidence | Gap versus mainstream |
|---|---|---|---|---|
| 38 | Phasing | Partial | `Phase` on walls; `Project.phase_names` (one vocabulary); IFC import maps phases (`🚪️io/📥️import/🏗️ifc/🧱️walls/🦀️.rs:14`) | See row 11. No phase filters in views, no phase in QTO. |
| 39 | Design options | Missing | No entity or kind (**grep**) | Entire feature. |
| 40 | Worksets and multi-user | Partial | `BimPresence { camera, storey, engagement_input }` (`✏️editor/👥️presence`); window transient is ephemeral per window (`✏️editor/🫧️transient`); sync and history come from the framework (event-sourced) | No worksets, element ownership, check-out or locks. |
| 41 | Undo and redo | Present | Concrete inverses on all 88 kinds (sum law 88/88, r6); editor history in `✏️editor/🦀️.rs` | None material. |

### 3.5 Data, standards, analysis, interchange

| # | Area | Grade | Evidence | Gap versus mainstream |
|---|---|---|---|---|
| 42 | Materials and layers | Present | `Material` (category, colour, density, conductivity, specific heat); `create-material`, `set-material`, `delete-material` (refuses while referenced, incl. railings); layers in types | No textures, no appearance or render materials, no plan hatch patterns per material. |
| 43 | Property sets | Partial | Typed `PropertyValue` (Text, Real, Integer, Boolean, Length, Area, Volume, Angle) per element; `set-element-property`, `remove-element-property`; IFC `IfcPropertySet` export and import | No property-set definitions or templates, no type-level properties, no allowed values or enumerations. |
| 44 | Classification | Partial | One `Classification { system, code, title }` per element; `set-element-classification`, `remove-element-classification`; IFC `IfcClassification` export | One classification per element; no classification system library. |
| 45 | Quantities and QTO | Present | `🧮️quantities` per element, type, material, storey, building, project (`🧮️quantities/🦀️.rs` header); IFC `Qto_*BaseQuantities` from the take-off (r7 codecs section 4) | No user quantity definitions, no costs, no quantities for furniture or MEP (absent kinds). Storey gross height is authored, not taken off. |
| 46 | Clash detection | Partial | `⚠️diagnostics/💥️clashes/🦀️.rs`: pairwise 2D footprint overlap over shared height within a building (12 kinds of pair); joins and beam-on-column rest are excluded | No 3D solid clash, no clash sets, tolerances per pair, or result panel. |
| 47 | Validation and rule checking | Partial | About 63 diagnostic codes (`⚠️diagnostics/🦀️.rs`): references, opening placement, degenerate geometry, storey gaps, stair comfort, space enclosure, roof fallbacks. | No user-defined rules. No diagnostics panel: the `🌳️outliner`, `🔍️properties` and `🛍️library` panels are the only panels, and `diagnostics` is registered only as an inference field (`✏️editor/🔮️inference/🦀️.rs:59`) (**grep**). |
| 48 | BCF | Missing | No hits in the plugin (**grep**: `bcf`) | Issues, viewpoints and snapshots exchange. |
| 49 | IFC export | Present (IFC2x3 only) | `🚪️io/📤️export/🏗️ifc/` writes site, building, storey, walls (`🏰️walls`), openings, windows, doors, slabs, roofs, columns, beams, stairs, railings, spaces, curtain walls, grids, Psets, Qto, classifications; validated with ifcopenshell (r4-exec-x-ifc) | IFC2x3 only, no IFC4. No furniture or MEP. Export has no progress or cancel (section 4.3). |
| 50 | IFC import | Partial | `🚪️io/📥️import/🏗️ifc/` imports storeys, walls, openings, slabs, columns, beams, spaces, grids, materials, types, Psets, classifications. Roofs, stairs, railings and curtain walls are reported, not imported (`🦀️.rs` header) | IFC4 import, and the four missing families. |
| 51 | Other exports | Partial | glTF 2.0 (`🧊️gltf`), SVG 1.1 plan (`🎨️svg`), pack and DSL text | No DXF, DWG, PDF, gbXML or IFC-XML. |
| 52 | Energy analysis hooks | Missing | Data only: material density, conductivity, specific heat; `Pset_MaterialThermal` in IFC; `Space.usage` string | No occupancy, set points, ventilation, envelope surfaces, U-value inference or gbXML. |
| 53 | Structural analysis hooks | Missing | Data only: profiles and material density on beams and columns | No analytical line model, supports, loads or analysis export. |

### 3.6 Cross-cutting

| # | Area | Grade | Evidence | Gap versus mainstream |
|---|---|---|---|---|
| 54 | Progress and cancellation for expensive work | Missing | r4-exec-u-viewer section 5 item 3 (render infers synchronously, no bounded job); r4-exec-x-ifc section 5 item 4 (export is one synchronous pass); r5-exec-x-gltf-svg (no progress or cancel). Escape cancels gestures only (`🚪️canvas-escape`). | AGENTS.md requires progress and cancellation for all expensive operations. |
| 55 | Editing tools beyond placement | Partial | 19 utilities (section 1). Move and rotate (`🚚️transform`), trim via `set-wall-axis`, split via `split-wall`. | No copy, mirror, array, offset, extend or trim for slabs and beams, no align, no join control, no match properties. |
| 56 | Internationalisation | Present | `🗣️terminology` en and de (`BimLabels`, `app_labels!`); x-semio-ui en and de on every mutation field (r7) | None material. |

Note on the counts: rows 1 to 56 above yield 15 Present, 23 Partial and 18 Missing.

## 4. Cross-cutting findings

### 4.1 Authored parameters that inference ignores (P0)

The new depth parameters from z-depth (r7) are editable in the properties panel and stored in the snapshot, but no derived value reads them yet. Verified **(grep)**:

- **Stairs**: `stringer`, `nosing`, `tread_thickness`, `riser`, `landing_depth` are not read by `🧊️element-solids/🪜️stairs`. The module states that stringers and nosings are not modelled.
- **Railings**: `post_profile`, `profile` and `baluster` are not read; `🛤️railings/🦀️.rs:20-24` still uses constants.
- **Storey cut height**: `set-storey-cut-height` writes `cut_height`, but `🗺️plan-linework` still uses `CUT_HEIGHT = 1.2` (lines 45, 330, 339). The handoff (r7 section 5) confirms it is not implemented.
- **Roofs**: the skeleton and roof surfaces from `G` exist, but `🏠️roofs` still uses the convex-only envelope and flat fallback (r7 section 4).

Impact: the UI accepts edits that have no visible effect. This breaks the principle that each authored parameter has a consequence. The r7 handoff section 7 checklist is the work list.

### 4.2 Diagnostics computed but not surfaced (P0)

Sixty-odd diagnostic codes are computed (`⚠️diagnostics`) and the editor references only the inference field. There is no panel, no list, no severity filter and no click-to-select. Validation is therefore invisible to the user.

### 4.3 Progress, cancellation and inference access paths (P0)

- No progress or cancellation for render, inference, IFC export, glTF export or SVG export (rows 49, 51, 54).
- Three inference access paths still exist (r5 F10). The editor uses the per-instance `ModelInferenceSession` (`✏️editor/🔮️inference`, **(grep)**). The viewer uses a thread-local memo (`🖌️render`, **(grep)**). IFC export calls `ModelInference::infer(model)` once per export (`🚪️io/📤️export/🏗️ifc/🦀️.rs:197`, **(grep)**). The z-graph session in `🕸️model-graph/📡️session` exists, but the viewer and export do not use it.
- r5 F11 (incremental recompute has no production caller) is partly resolved by the editor session; the viewer and export are still full recomputes.

### 4.4 Missing basic mutation kinds (P1)

- No kind to move an element to another storey (no `set-*-storey`).
- No kind to set the phase of a wall after creation, nor of any other element.
- No copy, mirror or array kind, although the move and rotate kinds exist.
- `delete-elements` has an unbounded inverse and refuses above 65 536 staged rows (r5 u-tools, section 2), so the Delete key cannot use the bulk path.

### 4.5 Verification debt (P0)

- The reported library result is 3688 passed, 0 failed, 0 ignored (r8-exec-z-close section 5). It was not re-run in this pass.
- Hub `surface` tests were never run to a verdict (r7-exec-z-codecs section 5).
- Platform `subject` and `parity` roles were not run for most cases (r4 and r5 reports).
- The drawing graph does not compile at the time of the r8 report; seven framework `tool_run` tests fail and are outside the BIM scope (r8-exec-z-close section 6).
- Playground registration and taxonomy entries for `🏡️house`, `🏢️office` and `🧰️checks` are not confirmed (r4-exec-x-examples; r3-exec-r-registration).
- No browser run of the viewer or editor (r4-exec-u-viewer section 5 item 7).

### 4.6 Resolved since r4 (confirmed in reports)

- `delete-storey` cascades over the element kinds its diff lists (walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces, openings) through the shared cascade (**grep** of `🚮️delete-storey/🔺️diff`; r4 open item resolved).
- Single deletes cascade properties and classifications (r6 item 2).
- `rehost-opening` removed; `move-opening` re-hosts with validation (r6 item 3).
- Column top check no longer runs the storey engine inside a diff (r6 item 10; r5 F16).
- Cross-kind id uniqueness (r6 item 1).

## 5. Prioritised work packages

Common recipe for every package (apply to each, not repeated below): schema-first through the single generator `r3-f1-gen-model.ts` (r7); leaves follow `r3-golden-leaf.md`; every leaf has `🔺️diff` built from payload and base reads only, a concrete `↩️inverse`, and a sum-law test (R15); `x-semio-inverse-rows` bounded; en and de labels and `x-semio-ui`; approved verbs only (`create`, `set`, `delete`, `move`, `rename`, `remove`, `split`, `flip`, `place`, `rotate`; `retract` was rejected in r8); fixtures and generators `r3-f1-gen-oracle.ts`, `r3-f1-gen-feature.ts`; at least one language-agnostic `.feature` scenario and one third-party oracle per feature (AGENTS.md); gate through `🚦️gate.sh`. Laws for every package: authored fields only in the snapshot; every derived value is a `ModelInference` field with declared `reads`; diffs are sparse; inverses are concrete.

Sizes: S = under a week of one agent, M = one to two weeks, L = several weeks, XL = a wave of parallel agents.

### P0: correctness and honesty (do first)

**WP-00 Verification baseline.** Size S.
- Goal: confirm the reported state before adding features.
- Actions: `cargo test … -p semio-s-artifact-bim-model --lib`; `cargo check -p semio-hub-bim --tests` native and wasm; run the hub `surface` tests; run `bun 📜️script.ts oracle quick` for every BIM case and `parity quick` for the export and IFC cases; register `🏡️house`, `🏢️office`, `🧰️checks` in the taxonomy.
- Mutation kinds, inference fields, UI, IO: none.
- Laws: none changed. Output is a verification report, not a fix.

**WP-01 Make authored depth parameters effective.** Size M.
- Goal: every authored parameter changes derived output (section 4.1).
- Mutation kinds: none new. Possibly `set-railing-profile` split from `set-railing` for clearer inverses; optional.
- Inference fields:
  - `🧊️element-solids/🪜️stairs`: stringers (closed, open, mono), nosings, tread thickness, open or closed risers, landing depth.
  - `🧊️element-solids/🛤️railings`: profile, post profile, baluster spacing, infill (glass or panel). Remove `POST_SIZE`, `RAIL_WIDTH`, `RAIL_DEPTH`.
  - `🪜️stair-runs`: `landing_depth` in the run record.
  - `🗺️plan-linework`: `cut_height` from `Storey.cut_height.unwrap_or(DEFAULT_CUT_HEIGHT)`.
  - `🧊️element-solids/🏠️roofs`: skeleton-based surfaces from `G/🦴️skeleton` and `G/🏠️roof`; hip fallback with a warning near reflex corners (r7 open item 2).
  - `🧮️quantities`: stringer and rail volumes from the new solids.
  - `⚠️diagnostics`: stair and railing construction problems already exist (`stair_construction_problem`); wire them.
- UI: no new UI; the existing properties rows become effective. Plan cut-height input initialised from the storey value.
- IO: IFC export writes stringers as flight parts; glTF and SVG pick up the new solids.
- Laws and tests: gating test per field (`infer_field_after_diff` of `SetStoreyCutHeight` touches `storeys/<id>/cut_height` only); parametric test "raising the cut above a sill closes the window gap in the poche"; mesh volume oracle (three.js) and plan oracle (shapely) for stairs, railings, roofs; the 54-case skeleton fixture `G/🧫️fixtures/🦴️skeleton` already exists for roofs.

**WP-02 Surface diagnostics and rule results.** Size M.
- Goal: validation is visible and actionable.
- Mutation kinds: none.
- Inference fields: `⚠️diagnostics` gains a per-element index (element id to codes) and a severity and category table. Add `RoofFallback` as a diagnostic code (r5 F14).
- UI: new panel `📌️panels/🚨️diagnostics`: list grouped by storey and kind, filter by severity, click selects the element in the plan and the outliner. Status line count in `✏️editor/🎛️chrome`. Labels en and de in `🗣️terminology`.
- IO: JSON and CSV export of diagnostics (text codec in `🚪️io/📝️text`).
- Laws and tests: determinism and default laws for each code; language-agnostic `.feature` for a clean and a defective model (`🧫️fixtures/💡️inferences/⚠️diagnostics/🏡️clean` and `💥️defects` exist); third-party cross-check of clash pairs with shapely.

**WP-03 Progress, cancellation and one inference path.** Size L.
- Goal: all expensive work is a bounded job with progress and cancel, and every consumer reads one session (section 4.3).
- Mutation kinds: none.
- Inference fields: no new fields. The model-graph session becomes the only entry point for editor, viewer and export; `ModelInference::infer` calls in exports are removed.
- UI: progress bar and cancel for render (viewer and editor), export dialog, and recompute of large models. Reuse the framework job pattern from the drawing editor (`FixedOperationRegistry`), or the BIM `ArtifactCommandWork`, as decided in the design.
- IO: `Serializer::serialize` gets a progress and cancel argument (r4-exec-x-ifc item 4). glTF, SVG and IFC exports run as jobs.
- Laws and tests: cancelled job keeps finished values (r2 section 5 law); cache-transparency test for each field; determinism of the session against a fresh `infer`.

### P1: core authoring parity with mainstream tools

**WP-04 Element storey and phase authoring.** Size M.
- Mutation kinds (new): `set-element-storey` for wall, curtain wall, column, beam, slab, roof, stair, railing, space, opening host check; `set-element-phase` for every phasable element (wall, slab, roof, column, beam, stair, railing, space, curtain wall); both refuse when a top constraint or hosted opening would break.
- Inference fields: `🎭️phase-visibility` (new, per storey and view phase: which element ids are visible); `🧮️quantities` gains a per-phase breakdown; `🧱️wall-layout` and others are re-inferred only through the storey gate.
- UI: phase combobox in `🔍️properties`; view phase in the plan and world window config; storey-move action in the outliner (drag between storeys).
- IO: IFC export writes `IfcElement` phase via `Pset_...` or `IfcGroup` assignment; IFC import reads the phase instead of defaulting.
- Laws and tests: sum law for both kinds; authored `storey` and `phase` never derived; language-agnostic scenario "move a wall from ground to first, its openings follow, quantities move per storey".

**WP-05 Modify toolset kernel.** Size L.
- Mutation kinds (new): `copy-elements` (new ids minted deterministically from the op id; inverse deletes the copies), `mirror-elements` (about an axis, reflects axes, rotations and openings, inverse mirrors back), `array-elements` (linear and radial; expressed as a list of copies), `align-elements` (axis-aligned move, inverse move back), `offset-wall` (creates a parallel wall, inverse deletes it), `trim-extend-wall` (uses `set-wall-axis` with computed ends, inverse restores), `split-slab` and `split-beam` (new kinds with concrete inverses), `set-wall-end-join` (authored join preference per wall end, default auto).
- Inference fields: none new. Joins already come from `🔗️joins`; `set-wall-end-join` only feeds the join choice.
- UI: utilities copy, mirror, array, offset, trim, extend, align, split; hotkeys; previews in the window transient.
- IO: none new; examples extended.
- Laws and tests: every copy is a create diff, the inverse deletes the minted ids (sum law); determinism of minted ids; gesture tests in the style of `🧪️tests` under `✏️editor/🧵️gestures`.

**WP-06 Ceilings.** Size M.
- Mutation kinds (new): `create-ceiling`, `set-ceiling`, `delete-ceiling`, `set-ceiling-boundary`, `create-ceiling-type`, `set-ceiling-type`, `delete-ceiling-type`. Ceiling record: storey, type, boundary loop, holes, offset (below storey top), optional slope, name. Type reuses the layer stack of slab types.
- Inference fields: `🧊️element-solids/⬜️ceilings` (layers hanging below the offset); `🏠️spaces` takes the clear height from ceilings (replaces `ceiling_of` over slabs); `🧮️quantities` (area, volume); `⚠️diagnostics` (ceiling outside its storey, ceiling below a beam).
- UI: ceiling tool (pick a space, pick a rectangle or polygon); library entry; properties rows.
- IO: IFC `IfcCovering` with `PredefinedType = CEILING` in export; import of `IfcCovering`.
- Laws and tests: sum law; `🏠️spaces` clear-height test changes when a ceiling is edited (gating test); oracle for ceiling volumes (three.js).

**WP-07 Zones, room finishes and area schemes.** Size M.
- Mutation kinds (new): `create-zone`, `set-zone`, `delete-zone` (cascade clears `space.zone`); `set-space` extended with `zone: Option<id>` and finish references (`floor_finish`, `wall_finish`, `ceiling_finish` as material or type ids); `create-area-scheme`, `set-area-scheme`, `delete-area-scheme` (usage list and included zones, as authored rules).
- Inference fields: `🏠️spaces` aggregates per zone and per area scheme; `🧮️quantities` gets finish areas (floor from net floor area, wall finish from perimeter times clear height minus openings, ceiling from ceiling area).
- UI: zones in the outliner; finish pickers in the space properties; area-scheme editor; room finish schedule (`🧮️schedule` definition from WP-14).
- IO: IFC `IfcZone` and finish Psets in export; import of zones.
- Laws and tests: zone delete cascades the membership only, inverse restores it; finish areas agree with the wall solids (oracle shapely).

**WP-08 Wall depth: sweeps, reveals, attach.** Size L.
- Mutation kinds (new): `create-wall-sweep`, `set-wall-sweep`, `delete-wall-sweep` (host wall, side, profile, height above floor, material, inset); `set-opening` extended with `reveal` (authored depth override and material); `set-wall-top` gains attachment variants `Roof { id }`, `Slab { id }`, `Ceiling { id }` in `TopConstraint`.
- Inference fields: `🧱️wall-layout` resolves attached top heights with cycle detection; `🧊️element-solids/🧱️walls` adds sweep solids and authored reveal depths; `🧮️quantities` adds sweep lengths and areas.
- UI: sweep tool (pick wall side, pick profile); wall top attach picker in properties.
- IO: IFC `IfcWallStandardCase` with attach; sweeps as `IfcMember` or proxy; glTF.
- Laws and tests: attach cycle refused in diff (inference must not run in diff, r6 ruling 7: use authored reads only); gating tests for slab and roof edits; sweep solid volume oracle.

**WP-09 Ramps and railing hosting.** Size M.
- Mutation kinds (new): `create-ramp`, `set-ramp`, `delete-ramp` (path, width, landing lengths, slope limit, top constraint, optional railings on each side); `set-railing` gains `host: Option<element id>` (stair, ramp or slab edge).
- Inference fields: `🪜️ramp-runs` (new; length, rise, slope, landings, slope code check); `🧊️element-solids/🛝️ramps`; `⚠️diagnostics` `RampSlope`; `🧮️quantities`.
- UI: ramp tool (click path, width in library); railing host picker.
- IO: IFC `IfcRamp`, `IfcRampFlight` export and import.
- Laws and tests: slope check as third-party-verified formula (python); sum laws.

**WP-10 Furniture, equipment and MEP placeholders.** Size L.
- Mutation kinds (new): `create-furniture-type`, `set-furniture-type`, `delete-furniture-type` (name, category, width, depth, height, material); `create-furniture`, `set-furniture`, `delete-furniture` (storey, type, position, rotation); `create-equipment` with the same type family and a system tag; `create-mep-placeholder` (system: supply, return, exhaust, pipe, cable; size; connector points; storey); `set-mep-placeholder`, `delete-mep-placeholder`. Deletes cascade the properties and classifications (r6 rule).
- Inference fields: `🧊️element-solids/🪑️furniture` (box or type-defined solid); `🧮️quantities` (counts, volumes per type and system); `⚠️diagnostics` (placement inside a wall, outside the storey).
- UI: furniture library entries; place tool (`place-elements` kind already exists for placement); MEP placeholder tool with system colour in plan and 3D.
- IO: IFC `IfcFurniture` and `IfcDistributionElement` or `IfcBuildingElementProxy` with the system tag; glTF.
- Laws and tests: sum laws; type refusal while in use (`mutation.in-use`); language-agnostic scenario "place a chair, delete its type refused".

**WP-11 Dimensions, tags, notes and leaders.** Size M.
- Mutation kinds (new): `create-dimension` (storey, anchors as element references or points, offset, style), `set-dimension`, `delete-dimension`; `create-tag` (element reference, category), `set-tag`, `delete-tag`; `create-text-note` (storey, position, text, style), `set-text-note`, `delete-text-note`; `create-leader`.
- Inference fields: `🗺️plan-linework` gains annotation records: dimension text and extension lines computed from anchors (derived), tag text from element authored fields (derived), note geometry.
- UI: dimension tool (pick two anchors, place); tag tool; text tool; style presets in library.
- IO: annotations in SVG export (new layer `annotations`); glTF ignores; IFC annotation export as `IfcAnnotation` (optional).
- Laws and tests: dimension text is derived, never stored; anchor moves update the value through inference (gating test); oracle for dimension lengths (shapely).

**WP-12 Views as authored entities, with elevations.** Size L.
- Mutation kinds (new): `create-view`, `set-view`, `delete-view` with kinds Plan, Section, Elevation, Ortho3D; a view stores storey or plane, cut height, depth range, direction, crop, visibility categories and phase filter.
- Inference fields: `🖼️view-linework` (new): plan linework generalised to any view plane (section cut and elevation projection of elements along the view direction); reuse `🗺️plan-linework` and `🔪️section`.
- UI: view browser in the outliner; plan, section and elevation windows bind to a view id; section line becomes an authored view, not window config.
- IO: SVG per view; glTF unchanged.
- Laws and tests: a view edit touches only its own linework (gating); the section-line migration removes window config for section lines (no legacy layer, greenfield rule).

**WP-13 User-defined schedules.** Size M.
- Mutation kinds (new): `create-schedule`, `set-schedule`, `delete-schedule` (category, columns from a fixed quantity key list, sort, filter, group, totals on or off, storey scope).
- Inference fields: `🧮️schedule` becomes per schedule definition: rows and totals derived from `🧮️quantities`; no stored rows.
- UI: schedule editor in the schedule window (columns picker, sort, filter); door, window and room schedules as presets.
- IO: CSV and XLSX-like text export (CSV through `🚪️io/📝️text`); IFC `Qto` unchanged.
- Laws and tests: definitions are authored; rows are derived; a quantity edit updates the schedule through the gate.

**WP-14 Sheets and printing.** Size L.
- Mutation kinds (new): `create-sheet`, `set-sheet`, `delete-sheet` (number, name, size, title-block fields); `create-viewport` (view id, sheet id, position, scale), `set-viewport`, `delete-viewport`.
- Inference fields: `📄️sheet-layout` (new): composed sheet geometry from views and title block (derived).
- UI: sheet list in the outliner; sheet window; viewport placement.
- IO: SVG per sheet (first), PDF per sheet (needs a framework PDF writer; decision).
- Laws and tests: viewports are authored references; sheet output is derived.

### P2: coordination, interoperability, checking

**WP-15 Design options and worksets.** Size L.
- Mutation kinds (new): `create-design-option`, `set-design-option`, `delete-design-option` (group, primary flag); `set-element-option`; `create-workset`, `set-workset`, `delete-workset`, `set-element-workset`.
- Inference fields: `🧭️option-scope` (new): visible elements per option set and per workset; `🧮️quantities` per option.
- UI: option switcher in the top bar; workset panel; ownership and lock display from presence (ephemeral shared lane, not authored).
- IO: IFC `IfcGroup` for options and worksets (export); import.
- Laws and tests: lock and ownership stay ephemeral (never in the snapshot), so the snapshot law holds; sum laws for option and workset kinds.

**WP-16 Clash, rule sets and BCF.** Size XL.
- Mutation kinds (new): `create-clash-set` (group A and group B selectors by kind, workset, option; tolerance); `set-clash-set`, `delete-clash-set`; `create-rule` and `set-rule` for numeric code checks (minimum clear height, maximum riser, minimum door width, and so on) with scope selectors; `delete-rule`.
- Inference fields: `💥️clashes` rewritten on 3D solids (triangle intersection through `G/🕸️mesh` with tolerance, not footprints only); `⚠️rules` (new) per rule result; both are derived.
- UI: clash panel (run, list, select, zoom to pair); rule panel; "run" is a job with progress and cancel (WP-03).
- IO: BCF 2.1 export and import (zip container and XML; the framework needs a zip primitive, no runtime library, per AGENTS.md); issues carry clash id, viewpoint and snapshot.
- Laws and tests: results are inferred, never stored; oracle with an independent 3D intersection (three.js or trimesh-equivalent in the test group).

**WP-17 IFC4 and IFC coverage.** Size XL.
- Mutation kinds: none; this is an IO package.
- Inference fields: none new; export reads the same session (WP-03).
- IO: `s.stdio.ifc@4/*` export and import; import of roofs, stairs, railings, curtain walls (currently reported only); export of furniture, MEP placeholders, ceilings, ramps, zones, options; IFC4 property set templates; multiple classifications.
- Laws and tests: round trip byte-stable per schema; ifcopenshell validation on IFC4 (python test group).

**WP-18 Property-set templates and classification systems.** Size M.
- Mutation kinds (new): `create-property-template`, `set-property-template`, `delete-property-template` (names, types, units, allowed values); `create-classification-system`, `set-classification-system` (entries as authored table); `set-element-classification` extended to a list keyed by system (whole-list rule for one element's list, per r2 ruling 5).
- Inference fields: `⚠️diagnostics` for values outside allowed values; no derived values.
- UI: template editor in library; apply template to an element or a type; classification browser.
- IO: IFC `IfcPropertySetTemplate` and `IfcClassificationReference` (many per element).
- Laws and tests: sum laws; templates are library data and own no entries (r6 rule).

**WP-19 Beams, columns and curtain-wall depth.** Size L.
- Mutation kinds (new): `set-beam-axis` (line or arc, same `Axis` type as walls); `set-column-tilt` (authored tilt about horizontal axis, `Slope` reused); `set-curtain-wall-grid` (explicit `u` and `v` line lists, whole-list); `create-curtain-panel-override` and `set-curtain-panel-override`, keyed by (curtain id, u index, v index) with panel type or door type or solid; `create-curtain-wall-type` and related kinds.
- Inference fields: `🧊️element-solids/➖️beams` arc sweep; `🏛️columns` tilted solids; curtain panels cut by hosted openings in `🪟️curtain-walls`; `🧮️quantities` adjusted.
- UI: arc beam gesture; tilt handle; panel override in properties and by click in the 3D world window.
- IO: IFC `IfcBeam` with arc axis (IfcTrimmedCurve); curtain wall panel parts.
- Laws and tests: one keyed collection for panel overrides needs a decision (keyed collection versus whole-list field); recorded in section 6.

### P3: analysis hooks and extended features

**WP-20 Energy analysis hooks.** Size L.
- Mutation kinds (new): `set-space-conditions` (occupancy, heating and cooling set points, ventilation rate, lighting power density); `create-thermal-zone` only if spaces cannot express zones (decision).
- Inference fields: `🌡️energy-envelope` (new): envelope surfaces from element solids with boundary condition (exterior, ground, adjacent), area, orientation, U-value from layer stack (`conductivity`, `thickness`), glazing from openings.
- UI: space conditions in properties; envelope overlay in 3D.
- IO: gbXML export (`s.stdio.gbxml`) or IDF; IFC `Pset_SpaceThermalRequirements` on export.
- Laws and tests: U-values from layers only (derived); oracle for areas by orientation (shapely and numpy).

**WP-21 Structural analysis hooks.** Size L.
- Mutation kinds (new): `create-support`, `set-support`, `delete-support` (node or line, restraint set); `create-load-case`, `create-load` (point, line, area on members; authored only).
- Inference fields: `🦴️analytical-members` (new): line members for beams, columns and walls with end offsets and rigid links, derived.
- UI: structural view overlay; support and load tools.
- IO: IFC4 `IfcStructuralAnalysisModel`; JSON for solver input.
- Laws and tests: analytical members are derived from authored geometry; oracle for member lengths.

**WP-22 Costing.** Size M.
- Mutation kinds (new): `create-cost-item`, `set-cost-item`, `delete-cost-item` (unit, unit cost, rule for quantity key); `set-type-cost-link` (type to cost item).
- Inference fields: `🧮️costs` (new): cost per element, per type, per storey, derived from `🧮️quantities`.
- UI: cost panel in schedule window (WP-13).
- IO: CSV export.
- Laws and tests: costs are derived; sum laws for cost kinds.

**WP-23 Parametric families (research spike).** Size XL, spike first.
- Goal: decide whether types become parametric families (formula parameters, nested families) or stay fixed records with more fields.
- Output: decision record only; no implementation until decided.

## 6. Decisions needed from the owner

1. Geometric constraints (WP-11, row 5): an authored constraint with a derived solver result conflicts with "only authored parameters are stored", unless the solver writes nothing and the constraint is a stored driving dimension. Decide whether constraints drive positions (derived positions) or only check them (diagnostics).
2. Curtain panel overrides (WP-19): keyed collection by (curtain, u, v) versus a whole-list field. A keyed collection fits the id-uniqueness rule (r6 item 1); the whole-list fits r2 ruling 5 only if panels are not independently addressable.
3. Sheets PDF output (WP-14): framework-level PDF writer versus SVG only.
4. Parametric families (WP-23): spike before any work.
5. BCF and zip (WP-16): a framework zip primitive is needed; confirm it may be added to the framework (AGENTS.md forbids runtime external libraries).

## 7. Suggested order

1. WP-00 verification, then WP-01 (inert parameters), WP-02 (diagnostics panel), WP-03 (progress and one inference path). These close the P0 gaps and make the existing features honest.
2. WP-04 (storey and phase), WP-05 (modify toolset), WP-06 (ceilings), WP-07 (zones and finishes), WP-11 (annotations), WP-12 (views with elevations), WP-13 (schedules). These cover the daily authoring loop.
3. WP-08 (sweeps, reveals, attach), WP-09 (ramps), WP-10 (furniture and MEP), WP-14 (sheets). Family coverage and documentation output.
4. WP-15 (options and worksets), WP-16 (clash, rules, BCF), WP-17 (IFC4), WP-18 (templates and classifications), WP-19 (beams, columns, curtain depth).
5. WP-20 to WP-23 (analysis hooks, costs, families).
