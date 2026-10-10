# r12 API: component and MEP inference (label `w2-f3-graph`, for `w2-f3-editor`, `w2-f3-assets`, `w2-f3-leaves`)

S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, I = `S/🧬️schema/💡️inferences`. Module paths: `semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::{components, mep, element_solids::{components, mep}}`.
Status: the types below are fixed (names will not change, bodies and tests are still being filled in; this file is updated when something changes). Everything is read from the session inference (`ModelInferenceSession::inference()` / `ModelInference`), never re-derived.

## ModelInference fields (new)

```rust
pub components: BTreeMap<String, ComponentValue>,   // by component id
pub mep: BTreeMap<String, MepValue>,                // by MEP element id
// existing fields that now also cover the new elements:
element_solids["<id>"]   // ElementSolid, family SolidFamily::Component | SolidFamily::Mep (building frame, like columns)
plan_linework[<storey>]  // + PlanKind::{ComponentOutline, ComponentFront, ComponentConnector, MepAxis, MepBand, MepDrop}; primitives carry element = the component/mep id
quantities.elements["<id>"]  // ElementQuantity with kind QuantityKind::{Component, Mep}, + new `groups: Vec<String>`; quantities.*.groups: BTreeMap<String, Totals>
diagnostics              // + DiagnosticCode::{ComponentOutsideStorey, ComponentInWall, RefComponentFamily, RefComponentHost, ComponentOverride, MepDegenerate, MepClash, TerminalUnconnected}
```

## `ComponentValue` (`inferences::components`)

```rust
pub struct ComponentValue {
    pub storey: String, pub family: String, pub category: Option<FamilyCategory>,
    pub placement: ComponentPlacement,     // where the family origin stands and how the family is turned
    pub footprint: Vec<Point2>,            // oriented rectangle (CCW, building frame) of the bounds of the visible solids in the family frame; empty without geometry
    pub footprint_area: f64,               // (max_x - min_x) * (max_y - min_y) in the family frame
    pub bounds: SolidBounds,               // AABB of the footprint x [origin z + local min z, origin z + local max z]; building frame
    pub volume: f64,                       // sum of the volumes of the visible family solids
    pub connector: Option<Connector>,      // Some for a terminal (component.system set)
    pub overridden: Vec<String>,           // names of the overrides that name a parameter of the family (sorted)
    pub parameters: BTreeMap<String, ResolvedParameter>,   // the family parameters under the overrides (kind, canonical formula, value)
    pub issues: Vec<ComponentIssue>,
}
pub struct ComponentPlacement { pub x: f64, pub y: f64, pub z: f64 /* storey elevation + component.elevation, building datum */, pub yaw: f64 /* rad, includes the host turn and `rotation` */, pub mirrored: bool, pub host: Option<HostFit> }
impl ComponentPlacement { pub fn apply(&self, local: [f64; 3]) -> [f64; 3]; pub fn rotate(&self, local: [f64; 3]) -> [f64; 3] }   // family frame -> building frame (mirror x, turn by yaw, translate)
pub struct HostFit { pub wall: String, pub station: f64 /* arc length of the projection on the axis */, pub side: f64 /* +1 left, -1 right along the axis */, pub face: Point2, pub normal: Point2 /* unit, away from the wall */ }
pub struct Connector { pub system: MepSystem, pub colour: String /* "#rrggbb" */, pub position: Point3 }
pub enum ComponentIssueCode { FamilyMissing, FamilyProfile, HostMissing, HostOtherStorey, HostDegenerate, Override, NonFinite }   // .slug()
pub struct ComponentIssue { pub code: ComponentIssueCode, pub subject: String /* family | wall | parameter or solid id */, pub detail: String /* EN */, pub family_issue: Option<FamilyIssue> /* Override: message via families::issues::message(.., "en"|"de") */ }
impl ComponentValue { pub fn solid(&self) -> bool; pub fn terminal(&self) -> bool; pub fn has(&self, ComponentIssueCode) -> bool }
pub fn overrides_of(snapshot: &ModelSnapshot, id: &str) -> BTreeMap<String, String>   // formula texts by parameter name (key range scan, no collection scan)
pub fn fit_to_wall(wall: &str, axis: &Axis, left: f64, right: f64, position: Point2) -> Option<HostFit>   // the host rule alone (editor ghost preview, host suggestion)
pub fn component_of(snapshot, id, level: &StoreyLevel, base: Option<&Arc<FamilyValue>>, layout: Option<&WallLayout>) -> ComponentEntry   // ComponentEntry { value: ComponentValue, family: Arc<FamilyValue> }
```

Host rule (binding, as in the contract): the authored `position` is projected onto the wall axis, the face on the side where `position` lies (left of the axis direction = `offset_left` of the wall layout, right = `offset_right`) is the plane `y = 0` of the family, the family turns so that its local `+y` points away from the wall, `rotation` is added. A host that is missing / on another storey / of zero length is an issue and the component falls back to the authored position (unhosted).
Family frame -> building frame: `x' = x0 + cos(yaw) * sx - sin(yaw) * y`, `y' = y0 + sin(yaw) * sx + cos(yaw) * y`, `z' = z0 + z`, with `sx = -x` when mirrored (triangle winding is reversed in the solid).

## `MepValue` (`inferences::mep`)

```rust
pub struct MepValue {
    pub storey: String, pub system: MepSystem, pub colour: String /* "#rrggbb" of the system */,
    pub section: MepSection,               // kind Duct|Pipe|Tray, width, height (pipe: both = diameter), area & perimeter (closed form), label "300x200" (mm) | "Ø100"
    pub path: Vec<Point3>,                 // building frame, z = storey elevation + authored z, consecutive duplicates dropped
    pub segments: Vec<MepSegment>,         // { from, to, length }; .vertical()
    pub length: f64, pub volume: f64 /* area * length */, pub surface_area: f64 /* perimeter * length */,
    pub bounds: SolidBounds,               // centre line box grown by the reach (max(width, height) / 2) on every side
    pub issues: Vec<MepIssue>,             // NonFinite | SectionDegenerate | PathDegenerate
}
impl MepValue { pub fn ends(&self) -> Option<(Point3, Point3)>; pub fn buildable(&self) -> bool }
pub fn key(MepSystem) -> &'static str          // "supply" "return" "exhaust" "domestic-water" "waste" "gas" "power" "data" "lighting"
pub fn colour(MepSystem) -> &'static str       // sRGB: supply #1f77d4 blue, return #f08a24 orange, exhaust #8b5a2b brown, domestic water #1ec8d6 cyan, waste #808000 olive, gas #f2d11e yellow, power #d62728 red, data #8e44ad purple, lighting #ffbf00 amber
pub fn rgb(MepSystem) -> [f32; 3]; pub fn part_colour(part: &str) -> Option<[f32; 3]>; pub const SYSTEMS: [MepSystem; 9]
pub fn section_of(&MepShape) -> MepSection; pub fn mep_of(snapshot, id, level: &StoreyLevel) -> MepValue
mep::clash::{MepClash { a, b, distance, reach }, MepClashes { pairs }, clashes_of(..), segment_distance(..)}
```
Colouring: the solid of a MEP element has one group named after its system key (`ElementSolid.groups[0].part == "supply"`); `render::world::group_color` and the glTF material key `MaterialKey::System` already turn that into the system colour. Plan primitives of MEP (`MepAxis`, `MepBand`, `MepDrop`) and the `ComponentConnector` cross carry the element id: colour them with `inference.mep[id].colour` / `inference.components[id].connector.colour` (SVG: class `kind_class` + a `stroke` from that value).

## Solids

* Component: `element_solids::components::component_solid(&ComponentEntry)`; groups `body` with the evaluated material id of each family solid; hidden family solids are left out; empty without a footprint (missing family, profile family, non-finite).
* MEP: `element_solids::mep::mep_solid(&MepValue)`: one mitred prism per segment (closed per prism), pipe tessellated with the families `Circle` tolerance (1e-4 chord). The solid volume equals `area * length` exactly for duct/tray and within the chord tolerance (about 3e-3 relative at 100 mm) below the closed form for a pipe.
* `SolidFamily::Component` / `SolidFamily::Mep` (kind strings `component` / `mep` in glTF extras).

## Quantities

`ElementQuantity` (kind `Component`): `type_id` = family id, `width`/`length` = footprint extents across/along the family frame (x / y), `height`, `perimeter`, `gross_area` = `net_area` = footprint area, `gross_volume` = family volume, `net_volume` = solid volume, material rows from the solid, `groups = ["component-category:<category>", "component-system:<system>" (terminal)]`.
`ElementQuantity` (kind `Mep`): `type_id` = system key, `length`, `width`/`height` = section, `perimeter` = section perimeter, `gross_area` = section area, `net_area` = `surface_area` = lateral surface, `gross_volume` = `area * length`, `net_volume` = solid volume, `groups = ["mep-system:<system>", "mep-size:<label>"]`.
`QuantityTotals.kinds["component"|"mep"]`, `types["component:<family>"|"mep:<system>"]`, new `QuantityTotals.groups[<group key>]` (per category, per system, per size). No `ScheduleCategory` entries (see the report).

## Diagnostics (slug, severity, values)

`component.outside-storey` W (`height`, `storey_height`, `distance`), `component.in-wall` W (`overlap_area`, `overlap_height`; elements `[component, wall]`), `reference.component-family` E (`missing` = family), `reference.component-host` E (`missing` = wall), `component.override` E (`missing` = the parameters), `mep.degenerate` E, `mep.clash` W (`distance`, `reach`; elements `[a, b]`), `mep.terminal-unconnected` I (`distance` = 0.05 m). A component or MEP element on a missing storey is `reference.element-storey`.

## Graph (for tests and the session report)

Node kinds `component`, `mep`, `mep-clash` in `UpdateReport.computed_by_kind`. `Component(id)` parents: its storey, the `Family` node of its family (shared as is while there is no override), the layout of its host wall. `Mep(id)`: its storey. `MepClash(storey)`: the `Mep` nodes of the storey (the only expensive scan, once per storey, cancellable between nodes by the stepped driver). `Solid(Component|Mep, id)`, `Quantity(id)`, `Plan(storey)` and `Diagnostics(Storey)` read them. A family parameter edit recomputes the `Family` node and exactly the components of that family (and their solids, quantities, plan, diagnostics); an unrelated edit computes none.
