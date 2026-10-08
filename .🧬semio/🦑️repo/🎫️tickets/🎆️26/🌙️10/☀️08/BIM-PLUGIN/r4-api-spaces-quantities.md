# 🏠️🧮️ API: `spaces` and `quantities` (Wave I, label `i-spaces-quantities`)

For `u-editor` (schedule panel, properties), `x-ifc` (IfcSpace and quantity sets), `x-gltf-svg`/`i-plan-diagnostics` (room tags).
Both fields live on `ModelInference`: `spaces: BTreeMap<String /*space id*/, SpaceRoom>` and `quantities: ModelQuantities`.
`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, modules `S/🧬️schema/💡️inferences/{🏠️spaces,🧮️quantities}` =
`…::inferences::{spaces, quantities}`. TypeScript/JSON/GraphQL/proto facets are in the aggregate facets (`S/🧬️schema/💡️inferences/🔣️.json` …).

## `spaces` (`SpaceRoom`, one per space)

Fields: `status` (`Inferred | Explicit | NotEnclosed | SeedInsideWall | InvalidOutline`), `outline: Vec<Vertex>` (CCW; bulges only for explicit
outlines), `holes: Vec<Vec<Vertex>>` (islands, CW), `point` (a point inside: the seed, else the widest span at mid height), `area` (outline minus
islands), `perimeter` (outline plus islands), `net_floor_area` (area minus the columns of the storey, cut out with region booleans), `floor_z`
(storey elevation), `clear_height`, `volume = area * clear_height`, `ceiling_slab` (id, `""` when none), `bounding_walls` (ids of walls and curtain
walls that share an edge of at least 1 mm with the room, sorted).

- A bounded room is the face of `extent minus union(footprints)` that holds the seed. Footprints are exactly the join-trimmed ones of
  `wall-layout` (`wall_layout::{storey_bands, joins::{join, footprint}}`) plus curtain-wall bands of the mullion depth. Curved faces are flattened at
  `FLATTEN_TOLERANCE = 1e-6` (area error about 1e-6 relative). A face that reaches the extent (`OPEN_MARGIN = 1 m` beyond footprints and seeds) is open:
  `NotEnclosed`. A seed in wall material: `SeedInsideWall`. An explicit loop (at least 2 vertices, area > 1e-12) is exact for arcs.
- Clear height: storey height + `offset` - Σ layer thickness of the thickest slab of the storey with the next higher level in the same building
  that covers `point` and has no hole there (slab top = storey elevation + offset, layers stack downward); no slab: the storey height.
- DAG: `Rooted::Element(storey id)` per storey that has spaces, parent = the storey level node. Moving a wall of a storey re-infers the rooms
  of that storey only (`SpacesField`, `FIELD_ID s.bim.model.inference.spaces`).

## `quantities` (`ModelQuantities`)

`elements: BTreeMap<id, ElementQuantity>`, `storeys` / `buildings: BTreeMap<id, QuantityTotals>`, `project: QuantityTotals`.
`QuantityTotals { kinds, types, materials: BTreeMap<String, Totals> }`, `Totals { count, length, area, volume, mass }`; type keys are
`<kind>:<type id>` (`wall:wt-300`), kind keys `wall | curtain-wall | slab | roof | column | beam | window | door | void | stair | railing | space`.

`ElementQuantity { kind, storey, type_id, count (1), length, width, height, perimeter, gross_side_area, opening_area, net_side_area, gross_area,
net_area, surface_area, gross_volume, net_volume, mass, risers, layers: Vec<LayerQuantity { material, thickness, area, volume, mass }> }`;
measures that do not apply are 0. `ElementQuantity::area()` is the area a total sums (net side area of walls, net plan area of the rest).

| Kind | length | width | height | areas | volume | layers |
|---|---|---|---|---|---|---|
| wall | centreline | thickness | resolved | `gross_side_area = length * height` (one side, centreline), `opening_area` (hosted openings clipped to the development), `net_side_area`, `gross_area = net_area` = join-trimmed footprint | `gross_volume` = footprint * height, `net_volume` without the openings (layers: exact per-layer share of the footprint, opening cut per layer, arcs scaled by radius) | one per type layer, left face first (see `wall-layout`), `mass = volume * density` |
| curtain wall | axis | | resolved | `gross_side_area = area`, openings | solid volume | solid groups by material |
| slab | | vertical thickness | | `gross_area` boundary, `net_area` minus holes, `surface_area = net / cos(slope)` | `net_area * thickness` | one per layer |
| roof | | thickness | | eave outline (footprint grown by overhang), `surface_area` = upward faces of the outermost layer of the solid | solid volume | solid groups |
| column | height | | height | profile area, `perimeter` | area * height | one row (type material) |
| beam | axis | | | profile area | area * length | one row |
| window, door, void | | opening width | opening height | `gross_area = net_area = width * height` | solid volume | solid groups |
| stair | run | width | rise | | solid volume | `risers` = riser count |
| railing | path | | height | | solid volume | solid groups |
| space | | | clear height | `gross_area` room, `net_area` net floor | volume | |

Cross-checks in the tests: wall, slab and column `net_volume` equal the mesh volume of their `element_solids` entry (planar exactly, arcs within the
chord tolerance); the third-party oracle `🧮️infer-bim-1-quantities` (shapely) reproduces walls, slabs, columns, beams, spaces and their totals.
Entry point `compute_quantities(&ModelSnapshot, &ModelInference) -> ModelQuantities` reads the other inferred fields; `ModelInference::infer` calls it last.
