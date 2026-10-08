# 🪟️ API: `opening-frames` (Wave I, label `i-openings`)

Field of `ModelInference`: `opening_frames: BTreeMap<String /*opening id*/, OpeningFrame>`, schema `s.bim.model.inference.opening-frames` (version 1).
Module path: `semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::opening_frames::*`.
Source: `S/🧬️schema/💡️inferences/🪟️opening-frames/🦀️.rs` (`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`).
Consumers (solids, plan linework, IFC, diagnostics): read `inferred.opening_frames[id]`; everything below is metres/radians, f64.

## Conventions

- **Building coordinates** = the snapshot's plan coordinates (wall axes, `Point2`); `z` is up from the building datum (level 0 at the building elevation), exactly like `wall-layout.base_z`.
- **World coordinates** = building coordinates rotated by `building.rotation` about z, translated by `building.origin` in plan and by `site.elevation + building.elevation` in z (`storey-levels.absolute_elevation - elevation`).
- **Host** = a wall or, when no wall has the id, a curtain wall. The host length is the axis arc length, its height `top_z - base_z` as `wall-layout`/`curtain-layout` resolve them (`top_of`, `thickness_of`, `axis_length`, `offsets_of`, `curtain_layout_of` are called, not copied).
- **Size/sill**: `width`/`height` = `Opening.width`/`height` if set, else the type (window/door type, or the void's own size). `sill` = type sill + `Opening.sill` for a window (so `Opening.sill = 0` means "the type default"), `Opening.sill` for a door and a void (door default 0). The sill is measured up from the host base (`base_z`), not from the storey floor.
- **Position**: `Opening.offset` is the arc length of the opening CENTRE from the host axis start. Point and unit tangent come from `BulgeSeg::point_at_length` / `tangent_at_length` (lines and arcs, bulge convention of the geometry API). Beyond the axis the carrier is extrapolated and the issue `OutsideHostExtent` is raised.
- **Local frame** (`local`, building coordinates): origin = centre point on the host axis (the location line) at `base_z + sill`; `z_axis = (0,0,1)`; `x_axis` = tangent of travel, `y_axis` = left normal of the host (`perp(tangent)`), both multiplied by -1 when `flip_facing` (a half turn about z, so the frame stays right-handed: `x × y = z`). `+y` is the side the opening "faces"; doors swing towards `+y`.
- **Faces**: `face_front` / `face_back` = distance from the location line to the host's face on the `+y` / `-y` side (`wall-layout` `offsets_of`: `offset_left` / `offset_right`, swapped by `flip_facing`; curtain wall: mullion depth / 2 each). `reveal_depth = face_front + face_back` = host thickness (curtain wall: mullion depth).
- **Cut** (`cut`, host development): `s_min..s_max` = `offset ∓ width/2` (arc-length interval along the axis, also on arcs), `z_min..z_max` = `sill .. sill + height`, z up from the host base. This is the hole (window/void) or notch (door, `z_min = 0`) in the `(s, z)` faces of `geometry::mesh::extrude_between_faces`.
- **Door hand**: `hand = Some(Left|Right)` for doors with a known type (`DoorType.swing`, inverted by `flip_hand`), `None` for windows/voids/untyped doors. Seen from the `+y` side, `Left` hinges at the `+x` jamb, `Right` at `-x`.
- **Plan strokes** (`plan`, building plan coordinates, ready for `plan-linework`): window → one `Glazing` line along `x_axis` through the centre on the location line, from `-width/2` to `+width/2`; single door → a `Leaf` line from the hinge (on the jamb at the `+y` face: `centre ± x·width/2 + y·face_front`) perpendicular to the host towards `+y` with length `width`, and a `Swing` arc centred at the hinge, radius `width`, starting at the closed-leaf direction (towards the other jamb), sweeping a signed quarter turn (`±π/2`) to the open leaf; double door → two leaves of `width/2`, hinges at both jambs, mirrored; void → none. `PlanShape::Arc { centre, radius, start_angle, sweep }` angles are radians from +x, counter-clockwise positive.

## Value type (serde-free `ToValue`/`FromValue`; JSON shape = field names, unit enums as strings, `PlanShape` externally tagged `{"Line":{…}}`/`{"Arc":{…}}`, `hand` omitted when `None`)

```rust
pub struct OpeningFrame {
    pub width: f64, pub height: f64, pub sill: f64, pub offset: f64,
    pub cut: OpeningCut,                    // { s_min, s_max, z_min, z_max }
    pub reveal_depth: f64, pub face_front: f64, pub face_back: f64,
    pub host_length: f64, pub host_height: f64,
    pub point: Point2,                      // centre on the host axis (building plan)
    pub local: Frame, pub world: Frame,     // { origin: Vec3, x_axis: Vec3, y_axis: Vec3, z_axis: Vec3 }
    pub hand: Option<Swing>,                // snapshot enum Left | Right
    pub plan: Vec<PlanStroke>,              // { role: Leaf | Swing | Glazing, shape: PlanShape }
    pub issues: Vec<OpeningIssue>,          // HostMissing, TypeMissing, NonPositiveSize, OutsideHostExtent, BelowHostBase, AboveHostTop, OverlapsSibling (in this order)
    pub overlaps: Vec<String>,              // ids of the siblings whose cut rectangle shares positive area with this one (sorted)
    pub valid: bool,                        // issues.is_empty()
}
impl Frame { pub fn affine(&self) -> semio_framework_geometry::placement::Affine3 }   // opening-local -> frame space
```

Helpers (same module): `resolve_size(&ModelSnapshot, &Opening) -> Resolved { width, height, sill, type_found }`, `cut_of(&ModelSnapshot, &Opening) -> OpeningCut`, `sibling_cuts`, `host_extent`, `frame_of`, `plan_strokes`, `compute_opening_frames`.

## Validity (`issues`)

An unresolvable host (id neither wall nor curtain wall, or its storey missing) gives `HostMissing` and an otherwise empty frame (`Default` placement, cut/size still resolved). `TypeMissing`: the window/door type id is unknown (size falls back to the overrides, else 0). `NonPositiveSize`: width or height ≤ 1 nm. `OutsideHostExtent`: `s_min < 0` or `s_max > host_length` (1 nm tolerance). `BelowHostBase`: `z_min < 0`. `AboveHostTop`: `z_max > host_height` (this is what a lowered storey height or a lowered wall top triggers). `OverlapsSibling`: another opening on the same host shares > 1 nm in both `s` and `z`.

## DAG

`FrameKey::{Storey, Host, Opening}`: storeys (stacking chain as `storey-levels`) → hosts referenced by an opening (parents: the host's storey and the storey its `TopConstraint::Storey` targets) → openings (parent: their host; none when the host does not resolve). An opening's `dep_input` = its record, its window/door type record and the cut rectangles of its siblings on the same host; a host's = the wall (or curtain wall) record, its layers and the building origin/rotation. `reads` = `openings, walls, curtain_walls, wall_types, window_types, door_types, storeys, buildings, sites`. Parametric: `set-wall-axis` (move/curve), `set-wall-top`, `set-storey-height`, wall type/location edits, window/door type edits, `move-opening` and `set-opening` re-derive exactly the dependent frames.

## Not provided (yet)

Frame/reveal geometry inside curtain-wall grid cells (an opening on a curtain wall is a plain cut; the grid layout does not avoid it), sill/lintel solids and glass (belongs to `element-solids` which consumes `cut`, `local`, `face_front/back`, `plan`).
