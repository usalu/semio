# 🗺️⚠️ API of `plan-linework` and `diagnostics` (r4, label `i-plan-diagnostics`)

Consumers: `u-editor` (plan window, problems panel), `u-viewer`, `x-gltf-svg` (SVG plan export). `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.
Leaf dirs `S/🧬️schema/💡️inferences/{🗺️plan-linework,⚠️diagnostics,📦️bodies}`; Rust paths `…::schema::inferences::{plan_linework, diagnostics, bodies}`.
Fields of `ModelInference`: `plan_linework: BTreeMap<String /*storey id*/, PlanLinework>` and `diagnostics: Vec<Diagnostic>` (sorted: most severe first, then code, storey, element ids; de-duplicated).
Pure entry points: `compute_plan_linework(&snapshot)`, `plan_of(&snapshot, storey, levels)`, `compute_diagnostics(&snapshot)`, `storey_findings`, `building_findings`, `model_findings`.

## 🗺️ `plan-linework`

Coordinates are the **building coordinates** of the snapshot (no building origin or rotation applied: the plan window applies `Building.origin`/`rotation` like the opening frames' `world` frames do), metres, +Y north, counter-clockwise positive. The plan of a storey is cut at `CUT_HEIGHT = 1.2` m above the storey elevation (plan convention constant; there is no authored per-storey override yet): `cut_height = 1.2`, `cut_elevation = storey elevation + 1.2` (building datum, same frame as `wall-layout.base_z`).

```rust
pub struct PlanLinework { storey, cut_height, cut_elevation, regions: Vec<PlanRegion>, polylines: Vec<PlanPolyline>, texts: Vec<PlanText>, bounds: PlanBounds }
pub struct PlanVertex   { x, y, bulge }                  // bulge = tan(sweep / 4) of the segment leaving the vertex (0 = straight, > 0 counter-clockwise arc)
pub struct PlanRegion   { id, element, kind: PlanKind, style: PlanStyle, outer: Vec<PlanVertex>, holes: Vec<Vec<PlanVertex>> }   // filled, closed
pub struct PlanPolyline { id, element, kind, style, closed: bool, vertices: Vec<PlanVertex> }                                    // stroked
pub struct PlanText     { id, element, kind, style, x, y, rotation, label, detail, measure: Option<f64> }                        // space tag: label = number, detail = name, measure = area m2; grid label: label
pub struct PlanBounds   { min_x, min_y, max_x, max_y }   // exact (arcs included) rectangle of every primitive
```

* `id` is unique within a plan: `<element>/<PlanKind>/<n>`. `element` is the id of the snapshot entity (wall, opening, column, beam, slab, roof, stair, railing, space, grid line, curtain wall) so a pick on any primitive resolves to a selectable element.
* `PlanStyle`: `Cut` (the plane cuts it: poché, cut outlines, door leaf), `Projection` (below the cut, solid thin: slab edges, low walls, railings, sills, door swing arcs, glazing, layer lines, risers below the cut), `Hidden` (above the cut, dashed: beams, roofs, windows above the plane, demolished walls, risers above the cut), `Annotation` (grid lines/bubbles/labels, space outline/tag, stair arrow).
* A region or polyline maps one to one onto a `Canvas2d` layer record (`r3-recipe-ui.md` section 7.2): `outer`/`vertices` become `Move` + `Line`/`Arc` path segments (the bulge `b` is the arc: sweep = 4 atan b), `closed`/regions end with `Close`, `id` is the record id, `role` = `overlay`/`node` as the window decides, `fill` = poché colour only for regions, `stroke` weight and dash from `style`, `text` records from `PlanText`. Holes go into the same record with `fillRule: "evenodd"`.

`PlanKind` (what a primitive depicts): `WallCut` (region, one piece per run between opening gaps, join-trimmed exactly like `wall-layout.footprint`), `WallLayer` (layer interface line, interior layers only, stops at gaps and ends on the join trims), `WallOutline` (closed outline of a wall that is not cut: projection when it ends below the cut, hidden when it starts above it or is `Phase::Demolished`), `CurtainAxis`, `CurtainMullion` (region, `u_panels + 1` mullions at `panel_width` spacing), `WindowFrame` (closed `frame_depth` wide rectangle across the gap), `WindowGlazing` (`opening-frames` stroke), `WindowSill` (two lines along the wall faces, only where the window crosses the cut), `DoorLeaf`, `DoorSwing` (`opening-frames` strokes; swing arcs carry the bulge `tan(pi / 8)`), `ColumnCut` (region) / `ColumnOutline`, `BeamOutline` (closed, dashed above the cut), `SlabEdge` / `SlabHole` (closed; a region only if the plane cuts the slab), `RoofOutline` (footprint grown by the overhang), `StairOutline` (per flight, exact annular sector for a spiral), `StairRiser` (one line per riser: solid below the cut, dashed above), `StairCutLine` (the zig-zag at the riser the plane passes through), `StairArrow` (shaft through flights and landings plus the arrow head), `StairLanding`, `RailingPath`, `SpaceOutline` (room outline and islands from `spaces`), `SpaceTag`, `GridLine`, `GridBubble` (two half circles, bulge 1, radius `GRID_BUBBLE_RADIUS = 0.3`), `GridLabel`.

Cutting rule: an element with vertical span `[low, high)` is **cut** when `low <= cut < high`, **below** when `high <= cut`, **above** when `low > cut`. Spans: wall/column/curtain wall from `wall-layout` / `bodies::vertical`; beam `[storey top + top_offset - depth, storey top + top_offset]`; slab `[elevation + offset - thickness, elevation + offset (+ slope rise)]`; railing `[base_offset, base_offset + height]`; stair `[base_z, top_z]` of `stair-runs`; roof: hidden unless its base is below the cut. An opening cuts the plane when `host base + sill <= cut < host base + sill + height` (its gap removes that `s` range from the poché; the symbol is drawn in `Cut`/`Projection` style, `Hidden` if it does not cross).

Measures for oracles/status bars: `PlanLinework::{area_of(kind), length_of(kind), count_of(kind)}`.

DAG: key `Rooted` (`storey-levels::{Rooted, RootedValue, rooted_plan, …}`): `Storey(id)` roots in stacking order, `Element(storey id)` is the plan with parents `[own storey, the storeys its elements' top constraints target]` (`bodies::level_storeys`). `dep_input` of a plan is `bodies::storey_scope`: every element of the storey, the openings of its hosts, the grid lines of its building, all element/opening types, the material ids and the building placement. `FIELD_ID = s.bim.model.inference.plan-linework`, `reads = READS` (storeys, buildings, sites, walls, wall_types, curtain_walls, openings, window/door types, columns, column types, beams, beam types, slabs, slab types, roofs, stairs, railings, spaces, grids, materials).

## ⚠️ `diagnostics`

```rust
pub struct Diagnostic { code: DiagnosticCode, severity: Severity, message_key: String, elements: Vec<String>, missing: Vec<String>, storey: Option<String>, values: BTreeMap<String, f64> }
```
`Severity = Info | Warning | Error`. `message_key = "bim.diagnostic.<slug>"`; `row_of(code) -> Row {slug, severity, en, de}` is the single table (54 codes); `Diagnostic::text("en" | "de") -> Option<String>` renders the template (`{elements}` = ids joined by `, `, `{missing}` = referenced ids that do not exist, `{name}` = `values[name]`); any other locale gives `None` (no default language). Consumers that own a terminology table can key on `message_key` and `values`.

| Family | Codes (slug) | Severity | `values` |
|---|---|---|---|
| Clashes | `clash.wall-wall`, `wall-column`, `column-column` (Error), `wall-beam`, `beam-column`, `beam-beam`, `beam-slab`, `stair-wall`, `stair-column`, `stair-beam`, `stair-stair`, `slab-slab` | Warning | `overlap_area` m2, `overlap_height` m, `overlap_volume` m3 |
| References | `reference.wall-type`, `column-type`, `beam-type`, `slab-type`, `roof-type`, `window-type`, `door-type`, `top-storey`, `opening-host`, `element-storey`, `storey-building`, `building-site`, `grid-building`, `layer-material`, `duplicate-id` | Error | none; `missing` names the dangling id |
| | `reference.type-material`, `property-element` | Warning | |
| Openings (from `opening-frames`) | `opening.outside-host`, `below-base`, `above-top`, `overlap` (elements = the pair), `non-positive-size` | Error | |
| Degenerate | `degenerate.axis-length`, `wall-thickness`, `height`, `profile`, `loop`, `self-intersection`, `railing-path`, `non-finite`, `curtain-spacing`, `storey-height` | Error | `length`, `height`, `area`, `crossings` |
| Storeys | `storey.level-gap` (Warning, `from`/`to`), `level-duplicate` (Error, `level`), `no-datum` (Info, element = building) | | |
| Stairs (from `stair-runs`) | `stair.no-rise` (Error), `riser-too-high` (Error), `tread-too-shallow` (Warning), `comfort-rule` (Warning, Blondel 2R+T) | | `riser_height`/`tread`/`stride`, `limit`, `minimum`, `maximum` |
| Spaces (from `spaces`) | `space.not-enclosed`, `seed-in-wall`, `duplicate-number` | Warning | |

Clash model: bodies are prisms (`bodies::Body`: flattened footprint regions between `z_min` and `z_max`, chord tolerance 0.1 mm). A bounding-box sweep prefilters pairs, heights must overlap by more than 1 um, the pair of kinds must be in the table (slabs legitimately meet walls and columns, so those pairs are not reported), and the exact `region_boolean` intersection area must exceed `AREA_EPS = 1e-3` m2 (10 cm2). Crossing walls (an X join) are never a clash; a beam whose end lies in its partner wall, column or beam rests on it. Stair bodies are the flights and landings of `stair-runs`, curved or sloped parts are bounded by their extent.

DAG: `DiagnosticKey::{Level(storey), Storey(storey), Building(building), Model}`. Levels are the roots (stacking chain); `Storey` findings (references of its elements, validity: degenerate geometry, openings, stairs, spaces) have parents = the levels `bodies::level_storeys` names; `Building` findings (storey levels, space numbers, all clashes) have the levels of all its storeys as parents; `Model` findings (dangling spatial references, elements on missing storeys, orphaned hosts, type materials, duplicate ids, orphaned properties) have no parents and depend on the whole snapshot. `FIELD_ID = s.bim.model.inference.diagnostics`, `reads = READS` (all collections). Parametric: moving a wall into a column creates the clash and moving it back removes it; a storey height edit recomputes the findings of the storeys above only.

## 📦️ `bodies` (shared helper leaf, not a field)

`Body {id, kind: Wall|CurtainWall|Column|Beam|Slab|Stair, storey, z_min, z_max, regions, bounds}`, `storey_bodies(&snapshot, storey, &levels)`, `profile_outline/profile_extent`, `column_outline`, `beam_outline`, `beam_span`, `slab_span`, `slab_thickness`, `stair_outlines`, `vertical(base_offset, &top, &own, target)`, `target_of`, `level_storeys`, `storey_scope`, conversions `seg/point/mark/corners/ring/region/placed`. Slab convention (shared with `spaces`): the slab top is the storey elevation plus `offset`, the layers hang below it.

## Oracle

`S/🧪️tests/🗺️infer-bim-1-plan-and-diagnostics/{🥒️.feature,🐍️.py,🦀️.rs}`, oracle `bim-1-shapely-geometry`; projections `encode_inference_projection_json(&snapshot, "plan-metrics" | "diagnostics")`; fixtures `S/🧫️fixtures/💡️inferences/{🗺️plan-linework/{🏡️house,🌀️curved},⚠️diagnostics/{🏡️clean,💥️defects}}`.
