# 🧊️ API of `🧊️element-solids` (r4, owner `i-solids-walls`, consumers `i-solids-rest`, u-editor, u-viewer, x-gltf-svg, x-ifc)

`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. Module dir `S/🧬️schema/💡️inferences/🧊️element-solids/`,
Rust path `crate::standards::v1::subsets::any::schema::inferences::element_solids` (mounted in the artifact root `🦀️.rs` next to `storey_levels`).
Field of `ModelInference`: `element_solids: BTreeMap<String, ElementSolid>` (`#[derived]`, keyed by ELEMENT ID; ids are unique across collections because commands
generate them from document id + op id; an opening id keys its filler, a wall id keys the wall solid). Metres, radians, +Z up. Elements without geometry (zero
thickness, unknown type, invalid host, invalid placement) are ABSENT from the map. Facets: `S/🧬️schema/💡️inferences/{🟦️.ts,🔗️.graphql,🛰️.proto (field 8),🔣️.json}` and `🧊️element-solids/🟦️.ts`.

**Coordinates.** A solid is in BUILDING-LOCAL coordinates (the coordinates of the snapshot, `z` up from the building datum = level 0 of the building, the same
`z` as `wall-layout.base_z`). `ElementSolid::placement` is the instance transform into the world: rotate about +Z by `rotation`, then translate by `(x, y, z)`
(`x, y` = building origin, `z` = site elevation + building elevation). For a World3d `instances_json` entry: `position = [x, y, z]`, `rotation` = quaternion about +Z.

## 1. Value types (stable)

```rust
pub enum SolidFamily { Wall, CurtainWall, Window, Door, Column, Beam, Slab, Roof, Stair, Railing }
pub struct SolidPoint     { pub x: f64, pub y: f64, pub z: f64 }
pub struct SolidBounds    { pub min: SolidPoint, pub max: SolidPoint }
pub struct SolidPlacement { pub x: f64, pub y: f64, pub z: f64, pub rotation: f64 }
pub struct SolidGroup     { pub part: String, pub material: String, pub layer: u32 }   // one per distinct (part, material, layer)
pub struct ElementSolid {
    pub family: SolidFamily,
    pub storey: String,                 // filled by the field (family.storey(..)), not by compute
    pub placement: SolidPlacement,      // filled by the field
    pub groups: Vec<SolidGroup>,        // faces refer to these by index
    pub positions: Vec<f64>,            // flat xyz, one vertex triple per triangle (flat shading; creases smoothed on curved parts)
    pub normals: Vec<f64>,              // flat xyz, same length as positions
    pub indices: Vec<u32>,              // flat, 3 per triangle, counter-clockwise outward
    pub face_groups: Vec<u32>,          // one per triangle: index into `groups`
    pub bounds: SolidBounds,
    pub volume: f64,                    // m3, sum of the signed volumes of all parts (outward winding => positive)
    pub area: f64,                      // m2, total surface area of all triangles (internal layer interfaces included)
}
```
- `part` is a free string; constants in `element_solids::parts`: `BODY`, `LAYER`, `PANEL`, `MULLION`, `FRAME`, `MUNTIN`, `GLASS`, `LEAF` (add yours).
- `material` is a `Material` id (`""` = none, e.g. glazing); `layer` is the layer index (0 for unlayered parts). Colour is NOT stored: look up `snapshot.materials[material].color`.
  A material colour edit therefore never invalidates a solid.
- Methods: `is_empty`, `triangle_count`, `vertex_count`, `mesh() -> TriMesh` (rebuild for sections/booleans), `positions_f32()`, `normals_f32()`,
  `face_ids() -> Vec<u32>`, `vertex_colors(|&SolidGroup| [f32; 4]) -> Vec<f32>` (RGBA per vertex), `placed(storey, placement)`, `byte_size()`.
  `positions_f32()`, `normals_f32()`, `indices`, `vertex_colors(..)`, `face_ids()` are exactly the `positions`, `normals`, `indices`, `colors`, `faceIds` arrays of the framework
  `MeshData` inline entry `{ "id": <element id>, "data": MeshData }` of `World3d` `meshes_json` (no re-indexing, no welding needed).

## 2. Producing a solid (what `i-solids-rest` uses)

```rust
let mut builder = SolidBuilder::new(SolidFamily::Column);
builder.add(parts::BODY, &material_id, 0, &trimesh);       // TriMesh from semio_framework_geometry::mesh (extrude/sweep_profile/prism_between/...)
let solid: ElementSolid = builder.build();                  // bounds, volume, area computed here (volume = sum of part volumes)
```
Chord tolerance for every tessellated arc: `element_solids::CHORD_TOLERANCE = 1e-4` (0.1 mm sagitta; relative volume error of an arc wall about 3e-5).
Shared helpers in the root module: `segment_of(&Axis) -> BulgeSeg`, `resolve_vertical(base_offset, &TopConstraint, own, target) -> (base_z, top_z)`,
`storey_parents(snapshot, storey, &top) -> Vec<SolidKey>`, `levels(parents) -> (Option<StoreyLevel>, Option<StoreyLevel>)`, `profile_polygon(&Profile) -> Vec<Point>`
(centred (across, depth) outline, circles flattened), `profile_extents(&[Point]) -> (across, depth)`, `dep_object([...])`, `dep_value(&T)`.

## 3. Registering a family (the extension point)

```rust
pub trait SolidFamilyBuilder: Sync {
    fn source(&self) -> SolidSource;                                     // snapshot collection this family iterates
    fn storey(&self, snapshot: &ModelSnapshot, id: &str) -> Option<String>;          // storey the element stands on (placement + dependency)
    fn plan(&self, snapshot: &ModelSnapshot) -> Vec<InferenceStep<SolidKey>>;       // one step per element; parents = storey_parents(..): own storey first, then the constrained storey
    fn dependency(&self, snapshot: &ModelSnapshot, id: &str) -> DslValue;           // honesty contract of dep_input: everything `compute` reads for `id` besides parent values
    fn compute(&self, snapshot: &ModelSnapshot, id: &str, parents: &[SolidNode]) -> ElementSolid;   // `SolidNode::Level(StoreyLevel)` per parent, same order as `plan`; empty solid = no geometry
}
```
`SolidKey { source: SolidSource, id: String }` with `SolidSource::{Storey, Wall, CurtainWall, Opening, Column, Beam, Slab, Roof, Stair, Railing}`.
To add a family: create `🧊️element-solids/<emoji><slug>/🦀️.rs` with a unit struct implementing the trait, mount it in the artifact root `🦀️.rs` under `pub mod element_solids`,
add ONE entry to `fn builders()` and the collections you read to `READS` in `🧊️element-solids/🦀️.rs` (the `element-solids` row of `fields()` in `💡️inferences/🦀️.rs` reuses `READS`).
Everything below the storey nodes is recomputed per element only when its own `dependency` or a parent changed.

## 4. Walls, curtain walls, fillers (mine)

| Dir | Module | Source | Output |
|---|---|---|---|
| `🧱️walls` | `element_solids::walls` | `walls` | one closed body per layer (`LAYER`, the layer's material, layer index left face -> right face = interior -> exterior as in `wall-layout.layer_offsets`), join-trimmed ends from `wall-layout` (`left_face`/`right_face`, interpolated across the build-up), valid openings cut through all layers as holes with reveals, door-like openings (sill 0) as bottom notches; arc walls sliced within the chord tolerance |
| `🪟️curtain-walls` | `element_solids::curtain_walls` | `curtain_walls` | `MULLION` (mullion_material): vertical full height, horizontal between them, outer ones inside the extent; `PANEL` (panel_material): 0.024 m flat panel per cell; grid from `curtain-layout` (`u_panels` x `v_panels`) |
| `🚪️fillers` | `element_solids::fillers` | `openings` | window: `FRAME` + `MUNTIN` (type material) + `GLASS` (material `""`), `panes` panes; door: `FRAME` + `LEAF` (1 or 2 leaves); void and invalid placements: none. Placed with the `local` frame of `opening-frames` |

Openings are consumed through `opening_frames::{frame_of, HostExtent}` (cut rectangle, validity); a wall hole is made only for a `valid` opening that fits strictly inside the full-thickness extent and below the top.

## 5. Tests and oracles

- Unit: `🧊️element-solids/🧪️tests/🔬️unit`, `🧱️walls/…`, `🪟️curtain-walls/…`, `🚪️fillers/…` (analytic volumes/areas/bounds, watertightness per layer, location lines, joins vs layout footprint volume,
  parametric laws, determinism, default, gating through `infer_field_after_diff`, 500-wall bound).
- Fixtures `S/🧫️fixtures/💡️inferences/🧊️element-solids/<case>/🔣️.json` = `{ snapshot (authored), expected (closed forms, written by the python oracle), meshes (welded positions + indices per element, blessed by the Rust test with BIM_BLESS=1) }`.
- Oracles: `S/🧪️tests/🧊️infer-bim-1-wall-solids/🐍️.py` (`write|check <S/🧫️fixtures/💡️inferences>`: closed forms + IfcOpenShell kernel with `IfcOpeningElement` voids + shapely union), `S/🧪️tests/🧊️infer-bim-1-solids-three/🟦️.ts` (three.js volume/area/bounds of the blessed meshes).
