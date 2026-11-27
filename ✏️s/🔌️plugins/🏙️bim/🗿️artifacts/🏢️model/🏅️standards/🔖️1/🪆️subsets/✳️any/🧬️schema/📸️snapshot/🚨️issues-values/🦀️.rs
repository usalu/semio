//! 🚨️ Rule and issue value types of `s.bim.model@1`: element classes, rules, selectors, issues and camera viewpoints.

pub use semio_s_artifact_bim_model_geometry_values::standards::v1::subsets::any::schema::snapshot::values::*;
pub use semio_s_artifact_bim_model_properties_values::standards::v1::subsets::any::schema::snapshot::values::*;

/// 🧩️ The kind of building element a selector names (the families that have a solid): walls, curtain walls, windows, doors, columns, beams, slabs, ceilings, roofs, stairs, ramps, railings, wall sweeps, placed components and routed MEP elements.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ElementClass {
    Wall,
    CurtainWall,
    Window,
    Door,
    Column,
    Beam,
    Slab,
    Ceiling,
    Roof,
    Stair,
    Ramp,
    Railing,
    WallSweep,
    Component,
    Mep,
}

/// 📏️ The numeric code checks a rule can make: the clear height of spaces (minimum), the riser height (maximum), the tread depth (minimum) and the clear width (minimum) of stairs, the clear width of doors (minimum), the slope of ramps (maximum, rise over run), the width of corridors (minimum) and the net floor area of a compartment zone (maximum).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum RuleKind {
    MinClearHeight,
    MaxRiser,
    MinTread,
    MinStairWidth,
    MinDoorWidth,
    MaxRampSlope,
    MinCorridorWidth,
    MaxCompartmentArea,
}

/// 🚦️ How serious a violation of a rule is: an error, a warning or a note.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum RuleSeverity {
    Error,
    Warning,
    Note,
}

/// 🏁️ Where an issue stands: open, in progress, resolved (fixed, not yet confirmed) or closed.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum IssueStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

/// ⏫️ How urgent an issue is.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum IssuePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// 🎯️ A selector of building elements: every element that is of one of the classes, stands on one of the storeys, is in one of the phases and is among the ids; an empty list does not restrict. The ids of elements that do not exist select nothing.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ElementSelector {
    /// The element classes selected; empty selects every class.
    #[value(default)] pub classes: Vec<ElementClass>,
    /// Ids of the storeys the elements stand on; empty selects every storey. An opening stands on the storey of its host.
    #[value(default)] pub storeys: Vec<String>,
    /// The construction phases selected; empty selects every phase. An element that carries no phase counts as new, an opening takes the phase of its host.
    #[value(default)] pub phases: Vec<Phase>,
    /// Ids of the elements selected; empty selects every element. Ids that name no element select nothing.
    #[value(default)] pub ids: Vec<String>,
}

/// 🎯️ Where a rule applies: the storeys, phases and element ids it covers (an empty list does not restrict) and, for the rules about spaces and zones, the usage of the spaces or the category of the zones it covers.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct RuleScope {
    /// Ids of the storeys the rule covers; empty covers every storey.
    #[value(default)] pub storeys: Vec<String>,
    /// The construction phases the rule covers; empty covers every phase.
    #[value(default)] pub phases: Vec<Phase>,
    /// Ids of the elements the rule covers (the stair, door, ramp, space or zone it checks); empty covers every element.
    #[value(default)] pub ids: Vec<String>,
    /// For the space rules the usage of the spaces covered (compared without regard to case), for the compartment rule the category of the zones covered; empty covers all.
    pub filter: String,
}

/// 💥️ The clash an issue was raised from: the clash set and the two elements that clash.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ClashRef {
    /// The clash set that reported the clash.
    pub set: String,
    /// The first element of the clash (side A).
    pub first: String,
    /// The second element of the clash (side B).
    pub second: String,
}

/// 📷️ A viewpoint of an issue: the orbit camera of the 3D view, an optional section box and the elements that stay visible while all others are hidden (empty shows everything).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct IssueViewpoint {
    /// The orbit camera of the 3D view: the point it looks at, its azimuth and pitch and the distance to the target.
    #[dsl(block)] pub camera: ViewCamera,
    /// Optional section box that hides everything outside it; absent shows the whole model.
    #[dsl(block)] #[value(default, skip_serializing_if = "Option::is_none")] pub section: Option<SectionBox>,
    /// Ids of the elements that stay visible while all others are hidden; empty hides nothing.
    #[value(default)] pub isolate: Vec<String>,
}

/// 🎥️ An orbit camera: the point it looks at (plan position and height above the datum), the azimuth of the viewing direction in plan, the pitch above the horizon and the distance to the target, in metres and radians.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ViewCamera {
    #[dsl(block)] pub target: Point2,
    pub target_height: f64,
    pub azimuth: f64,
    pub pitch: f64,
    pub distance: f64,
}
