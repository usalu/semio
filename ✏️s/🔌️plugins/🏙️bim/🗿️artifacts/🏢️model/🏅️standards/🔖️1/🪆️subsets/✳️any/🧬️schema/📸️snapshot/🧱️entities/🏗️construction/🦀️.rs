//! 🏗️ Authored construction records for BIM model schema version one.
extern crate semio_framework_value_derive as value_derive;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::*;

/// 🧷️ A wall sweep: a profile run along one face of a wall (a baseboard, a cornice, a drip rail). It follows its host by inference: along the join-trimmed face, interrupted by the openings that reach its height, and clipped where the top of the wall is attached. Its solid, length and areas are inferred, never stored.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct WallSweep {
    /// The wall the sweep runs along.
    pub host: String,
    /// The face of the wall the sweep stands on, looking along the axis from start to end: the left (interior) or the right (exterior) face.
    pub side: WallSide,
    /// Section of the sweep: the first coordinate runs out of the wall perpendicular to its face, the second runs up; the section is centred on its own origin, so a rectangle of width 0.02 and depth 0.12 is a baseboard 2 cm deep and 12 cm high.
    #[dsl(statements, block)] pub profile: Profile,
    /// Height in metres above the wall base of the lowest point of the profile.
    pub height: f64,
    /// Distance in metres the profile is moved into the wall: zero stands the inner edge of the profile on the face, a positive inset embeds it, so only the rest shows.
    pub inset: f64,
    /// Material of the sweep.
    pub material: String,
    pub name: String,
}

/// 🧱️ A material of the project library.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Material {
    pub name: String,
    pub category: MaterialCategory,
    #[dsl(block)] pub color: Rgb,
    pub density: f64,
    pub conductivity: f64,
    pub specific_heat: f64,
}

/// 🧱️ Layered wall build-up.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct WallType {
    pub name: String,
    #[dsl(table)] #[value(default)] pub layers: Vec<Layer>,
}

/// 🧱️ Layered slab build-up.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SlabType {
    pub name: String,
    #[dsl(table)] #[value(default)] pub layers: Vec<Layer>,
}

/// 🧱️ Layered ceiling build-up: the first layer is topmost, the last one is the visible underside.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct CeilingType {
    pub name: String,
    #[dsl(table)] #[value(default)] pub layers: Vec<Layer>,
}

/// 🧱️ Layered roof build-up.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct RoofType {
    pub name: String,
    #[dsl(table)] #[value(default)] pub layers: Vec<Layer>,
}

/// 🏛️ Column profile and material.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ColumnType {
    pub name: String,
    #[dsl(statements, block)] pub profile: Profile,
    pub material: String,
}

/// 🏛️ Beam profile and material.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct BeamType {
    pub name: String,
    #[dsl(statements, block)] pub profile: Profile,
    pub material: String,
}

/// 🪟️ Window family parameters.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct WindowType {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub sill: f64,
    pub frame_width: f64,
    pub frame_depth: f64,
    pub panes: u32,
    pub material: String,
    /// Thermal transmittance of the whole window (glazing and frame) in watts per square metre and kelvin; absent means the window has no thermal data.
    #[value(default, skip_serializing_if = "Option::is_none")] pub u_value: Option<f64>,
    /// Total solar energy transmittance of the glazing, from 0 to 1; absent means the window has no solar data.
    #[value(default, skip_serializing_if = "Option::is_none")] pub g_value: Option<f64>,
    /// Share of the window opening covered by the frame, from 0 to 1; the glazed share is one minus it; absent counts the whole opening as glazing.
    #[value(default, skip_serializing_if = "Option::is_none")] pub frame_fraction: Option<f64>,
}

/// 🚪️ Door family parameters.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct DoorType {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub frame_width: f64,
    pub frame_depth: f64,
    pub leaves: DoorLeaves,
    pub swing: Swing,
    pub material: String,
    /// Thermal transmittance of the door leaf in watts per square metre and kelvin; absent means the door has no thermal data.
    #[value(default, skip_serializing_if = "Option::is_none")] pub u_value: Option<f64>,
}

/// 🌍️ A site: geographic anchor and boundary.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Site {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub true_north: f64,
    #[dsl(table)] #[value(default)] pub boundary: Vec<Point2>,
}

/// 🏢️ A building on a site.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Building {
    pub site: String,
    pub name: String,
    #[dsl(block)] pub origin: Point2,
    pub rotation: f64,
    pub elevation: f64,
}

/// 🪜️ A storey: authored level index, height and optional plan cut height; its elevation is inferred.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Storey {
    pub building: String,
    pub name: String,
    pub level: i32,
    pub height: f64,
    /// Plan view convention: height in metres above the storey elevation at which the plan cuts through the storey; absent means the default of 1.2 m.
    #[value(default, skip_serializing_if = "Option::is_none")] pub cut_height: Option<f64>,
}

/// 📏️ A labelled grid line of a building.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct GridLine {
    pub building: String,
    pub label: String,
    #[dsl(block)] pub start: Point2,
    #[dsl(block)] pub end: Point2,
}

/// 🧱️ A wall: axis, type, location line and base/top constraints; never its height.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Wall {
    pub storey: String,
    pub wall_type: String,
    #[dsl(statements, block)] pub axis: Axis,
    pub location: LocationLine,
    pub base_offset: f64,
    #[dsl(statements, block)] pub top: TopConstraint,
    pub phase: Phase,
    /// Authored join preference of the end at the start of the axis; absent leaves the join to the geometry (auto).
    #[value(default, skip_serializing_if = "Option::is_none")] pub start_join: Option<EndJoin>,
    /// Authored join preference of the end at the end of the axis; absent leaves the join to the geometry (auto).
    #[value(default, skip_serializing_if = "Option::is_none")] pub end_join: Option<EndJoin>,
    pub name: String,
    /// Optional slab the base of the wall stands on: the base follows the top surface of that slab (sloped slabs included) plus the base offset; absent means the base lies on the elevation of the storey plus the base offset.
    #[value(default, skip_serializing_if = "Option::is_none")] pub base_slab: Option<String>,
}

/// 🏬️ A curtain wall type: the grid rules of both directions, the mullion sections of the interior grid lines and of the border, the default panel of every cell, the glass material and the mullion material.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct CurtainWallType {
    pub name: String,
    /// Grid rule along the wall: equal cells of about the spacing, or explicit lines in metres from the start of the axis.
    #[dsl(statements, block)] pub u_grid: CurtainGrid,
    /// Grid rule up the wall: equal cells of about the spacing, or explicit lines in metres above the base.
    #[dsl(statements, block)] pub v_grid: CurtainGrid,
    /// Section of the mullions on the interior grid lines: the first coordinate runs along the wall, the second through it.
    #[dsl(statements, block)] pub interior_mullion: Profile,
    /// Section of the mullions on the outer edges of the wall.
    #[dsl(statements, block)] pub border_mullion: Profile,
    /// The panel of every cell that has no override.
    #[dsl(statements, block)] pub panel: CurtainPanel,
    /// Material of glass panels.
    pub panel_material: String,
    /// Material of every mullion.
    pub mullion_material: String,
    /// Thermal transmittance of the whole curtain wall (glass panels and mullions) in watts per square metre and kelvin; absent means the curtain wall has no thermal data.
    #[value(default, skip_serializing_if = "Option::is_none")] pub u_value: Option<f64>,
    /// Total solar energy transmittance of the glass panels, from 0 to 1; absent means the curtain wall has no solar data.
    #[value(default, skip_serializing_if = "Option::is_none")] pub g_value: Option<f64>,
    /// Share of the curtain wall area covered by the mullions, from 0 to 1; the glazed share is one minus it; absent counts the whole area as glazing.
    #[value(default, skip_serializing_if = "Option::is_none")] pub frame_fraction: Option<f64>,
}

/// 🎯️ The panel of one cell of a curtain wall that differs from the default panel of its type; the cell is named by its indices along the wall and up it, at most one override per cell.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct CurtainPanelOverride {
    /// The curtain wall whose cell is overridden.
    pub curtain: String,
    /// Zero-based index of the cell along the wall, from the start of the axis.
    pub u: u32,
    /// Zero-based index of the cell up the wall, from the base; a door panel belongs in row zero.
    pub v: u32,
    /// What fills the cell instead of the default panel.
    #[dsl(statements, block)] pub panel: CurtainPanel,
}

/// 🪟️ A curtain wall: axis, base and top constraints, the type that carries its grid rules, mullions and panels, and optional grid rules of its own.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct CurtainWall {
    pub storey: String,
    /// The curtain wall type that carries the grid rules, mullion sections, default panel and materials.
    pub curtain_wall_type: String,
    #[dsl(statements, block)] pub axis: Axis,
    pub base_offset: f64,
    #[dsl(statements, block)] pub top: TopConstraint,
    /// Optional grid rule along the wall that replaces the rule of the type; absent follows the type.
    #[dsl(statements, block)] #[value(default, skip_serializing_if = "Option::is_none")] pub u_grid: Option<CurtainGrid>,
    /// Optional grid rule up the wall that replaces the rule of the type; absent follows the type.
    #[dsl(statements, block)] #[value(default, skip_serializing_if = "Option::is_none")] pub v_grid: Option<CurtainGrid>,
    pub phase: Phase,
    pub name: String,
}

/// 🏛️ A column placed on a storey, plumb or leaning.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Column {
    pub storey: String,
    pub column_type: String,
    #[dsl(block)] pub position: Point2,
    pub rotation: f64,
    /// Optional lean of the column: the top leans towards `direction` (radians, counter-clockwise from +x) by `angle` radians from the vertical, about the base point; absent is plumb. The cross-section perpendicular to the axis is the profile of the type, the ends are cut horizontally.
    #[dsl(block)] #[value(default, skip_serializing_if = "Option::is_none")] pub tilt: Option<Slope>,
    pub base_offset: f64,
    #[dsl(statements, block)] pub top: TopConstraint,
    pub phase: Phase,
    pub name: String,
}

/// ➖️ A beam along a line or an arc, level or inclined: its top lies `top_offset` metres above (positive) or below (negative) the storey top at the start and `end_top_offset` at the end; zero is flush with the storey top.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Beam {
    pub storey: String,
    pub beam_type: String,
    /// The line or arc of the beam in plan, from its start to its end, exactly like the axis of a wall; a curved beam is an axis whose bulge is set.
    #[dsl(statements, block)] pub axis: Axis,
    /// Signed vertical offset in metres of the top of the beam at the start of its axis from the top of its storey: positive lifts it above the storey top, negative hangs it below, zero is flush.
    pub top_offset: f64,
    /// Optional signed offset in metres of the top of the beam at the end of its axis from the top of its storey; absent means the same as at the start (a level beam), a different value inclines the beam, linearly along the axis.
    #[value(default, skip_serializing_if = "Option::is_none")] pub end_top_offset: Option<f64>,
    pub phase: Phase,
    pub name: String,
}

/// ⬜️ A slab: boundary loop, holes, offset and optional slope.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Slab {
    pub storey: String,
    pub slab_type: String,
    #[dsl(table)] #[value(default)] pub boundary: Vec<Vertex>,
    #[value(default)] pub holes: Vec<Vec<Vertex>>,
    pub offset: f64,
    #[dsl(block)] #[value(default, skip_serializing_if = "Option::is_none")] pub slope: Option<Slope>,
    pub phase: Phase,
    pub name: String,
}

/// 🔲️ A ceiling: boundary loop, holes, drop below the storey top and optional slope; the layers of its type hang downward.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Ceiling {
    pub storey: String,
    pub ceiling_type: String,
    #[dsl(table)] #[value(default)] pub boundary: Vec<Vertex>,
    #[value(default)] pub holes: Vec<Vec<Vertex>>,
    /// Distance in metres the top of the ceiling hangs below the top of its storey; the layers of the ceiling type stack downward from there. Zero lies against the underside of the floor above.
    pub offset: f64,
    /// Optional tilt of the ceiling plane: fall direction and angle; the top plane keeps its drop at the uphill edge of the boundary.
    #[dsl(block)] #[value(default, skip_serializing_if = "Option::is_none")] pub slope: Option<Slope>,
    pub name: String,
}

/// 🏠️ A roof: footprint loop and shape.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Roof {
    pub storey: String,
    pub roof_type: String,
    #[dsl(table)] #[value(default)] pub footprint: Vec<Vertex>,
    #[dsl(statements, block)] pub shape: RoofShape,
    pub overhang: f64,
    pub base_offset: f64,
    pub phase: Phase,
    pub name: String,
}

/// 🪟️ A window, door or void hosted by a wall or curtain wall; `sill_override` replaces the sill of its type (zero for doors and voids).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Opening {
    pub host: String,
    #[dsl(statements, block)] pub kind: OpeningKind,
    pub offset: f64,
    /// Sill height in metres above the host base that replaces the sill of the window type; absent means the type sill for windows and zero for doors and voids.
    #[value(default, skip_serializing_if = "Option::is_none")] pub sill_override: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")] pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")] pub height: Option<f64>,
    pub flip_hand: bool,
    pub flip_facing: bool,
    pub name: String,
    /// Optional depth in metres of the reveal on the front side of the opening: the distance from the front face of the host to the front plane of the window or door frame; absent centres the frame in the thickness of the host.
    #[value(default, skip_serializing_if = "Option::is_none")] pub reveal_depth: Option<f64>,
    /// Optional material of the reveal surfaces (jambs, head and sill) between the front face of the host and the frame; absent leaves them in the material of the wall layers.
    #[value(default, skip_serializing_if = "Option::is_none")] pub reveal_material: Option<String>,
}

/// 🪜️ A stair run: placement, flight, limits and construction (stringer, nosing, tread thickness, risers, landing depth).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Stair {
    pub storey: String,
    #[dsl(block)] pub start: Point2,
    pub direction: f64,
    pub width: f64,
    #[dsl(statements, block)] pub flight: StairFlight,
    #[dsl(statements, block)] pub top: TopConstraint,
    pub max_riser: f64,
    pub min_tread: f64,
    /// Stringer construction: kind, plus width and depth of each stringer in metres.
    #[dsl(block)] pub stringer: StairStringer,
    /// Overhang in metres of a tread beyond the riser below it; zero is flush.
    pub nosing: f64,
    /// Vertical thickness in metres of each tread slab.
    pub tread_thickness: f64,
    /// Whether riser boards close the gap between treads.
    pub riser: RiserKind,
    /// Depth in metres of the landing of a turning flight, measured along the arriving flight.
    pub landing_depth: f64,
    pub phase: Phase,
    pub name: String,
}

/// 🛤️ A railing along a path: heights, rail and post sections, optional balusters and an infill.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Railing {
    pub storey: String,
    #[dsl(table)] #[value(default)] pub path: Vec<Point2>,
    pub height: f64,
    pub post_spacing: f64,
    /// Section of the top rail.
    #[dsl(statements, block)] pub profile: Profile,
    /// Section of a post.
    #[dsl(statements, block)] pub post_profile: Profile,
    /// Optional baluster row between the posts: section and spacing.
    #[dsl(block)] #[value(default, skip_serializing_if = "Option::is_none")] pub baluster: Option<Baluster>,
    /// Fill between the posts: none, glass or a panel with a thickness in metres.
    #[dsl(statements, block)] pub infill: Infill,
    pub material: String,
    pub base_offset: f64,
    /// Optional host (stair, ramp or slab edge): a hosted railing has no path of its own, its path and its base follow the host by inference.
    #[dsl(block)] #[value(default, skip_serializing_if = "Option::is_none")] pub host: Option<RailingHost>,
    pub phase: Phase,
    pub name: String,
}

/// 🛝️ A ramp: a sloped slab of constant thickness and width along a centre-line path whose rise is resolved by its top constraint; the slope, the landings and the compliance are inferred.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Ramp {
    pub storey: String,
    /// Centre line of the ramp from its foot to its head: vertices with the bulge of the segment to the next one; the ramp climbs along it.
    #[dsl(table)] #[value(default)] pub path: Vec<Vertex>,
    /// Clear width in metres across the centre line.
    pub width: f64,
    /// Length in metres of the flat landing at the foot of the path.
    pub landing_start: f64,
    /// Length in metres of the flat landing at the head of the path.
    pub landing_end: f64,
    /// Length in metres of the flat landing centred on every corner of the path where its direction changes.
    pub landing_turn: f64,
    /// Largest allowed slope as the ratio of rise to run (for example 0.0833 for 1:12); the inferred slope is checked against it.
    pub max_slope: f64,
    /// Vertical thickness in metres of the ramp slab, measured below its walking surface.
    pub thickness: f64,
    pub material: String,
    /// Height in metres of the foot of the ramp above the elevation of its storey (negative below).
    pub base_offset: f64,
    /// Constraint that resolves the height of the head of the ramp; the rise is the head minus the foot.
    #[dsl(statements, block)] pub top: TopConstraint,
    /// Whether the ramp carries a railing along its left edge, seen in the direction of travel.
    pub railing_left: bool,
    /// Whether the ramp carries a railing along its right edge, seen in the direction of travel.
    pub railing_right: bool,
    pub name: String,
}
