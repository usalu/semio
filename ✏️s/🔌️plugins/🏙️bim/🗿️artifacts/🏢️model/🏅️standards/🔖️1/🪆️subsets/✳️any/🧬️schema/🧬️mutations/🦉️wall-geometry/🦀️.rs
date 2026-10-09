//! 📐️ Shared checks of the wall leaves: validity, flipping and splitting of a bulged wall axis, and the invariants of a curtain wall
//! record. Pure and platform-independent: split points snap to a nanometre and sub-arc bulges to 1e-12, so every device derives the
//! same diff from the same mutation. Arc geometry follows `semio-framework-geometry::bulge` (`bulge = tan(sweep / 4)`, parameters
//! proportional to arc length), which cross-checks it in the unit tests.

use crate::{Axis, CurtainGrid, CurtainPanel, CurtainWall, CurtainWallType, ModelDiff, ModelSnapshot, Point2, Profile, Slope, TopConstraint};
use protocol::{MutationOutcome, OutcomeCode};
use std::f64::consts::TAU;

//#region 🔖️Flaw
/// 🧱 Smallest length and sweep that still counts as geometry, in metres and radians.
pub const EPS: f64 = 1e-9;

/// 🚫 One reason to refuse, with the vocabulary code and the path of the offending payload field.
#[derive(Clone, Debug, PartialEq)]
pub struct Flaw {
    pub code: OutcomeCode,
    pub path: Vec<String>,
    pub message: String,
}

impl Flaw {
    /// 🏗️ A flaw at `path`.
    pub fn new(code: OutcomeCode, path: &[&str], message: impl Into<String>) -> Self {
        Self { code, path: path.iter().map(|segment| segment.to_string()).collect(), message: message.into() }
    }

    /// 🧭️ The same flaw seen from an enclosing payload field.
    pub fn under(mut self, prefix: &[&str]) -> Self {
        self.path.splice(0..0, prefix.iter().map(|segment| segment.to_string()));
        self
    }

    /// 🚫 The refusing outcome of this flaw.
    pub fn refuse(self) -> MutationOutcome<ModelDiff> {
        MutationOutcome::refuse(self.code, self.message, self.path)
    }
}

fn invariant(path: &[&str], message: &str) -> Option<Flaw> {
    Some(Flaw::new(OutcomeCode::Invariant, path, message))
}
//#endregion 🔖️Flaw

//#region 🔖️Axis
/// 📏️ Rounds to a nanometre; negative zero becomes zero.
pub fn snap(value: f64) -> f64 {
    (value * 1e9).round() / 1e9 + 0.0
}

fn snap_bulge(value: f64) -> f64 {
    (value * 1e12).round() / 1e12 + 0.0
}

/// 📍️ The start and end point of an axis.
pub fn ends(axis: &Axis) -> (Point2, Point2) {
    let (Axis::Line { start, end } | Axis::Arc { start, end, .. }) = axis;
    (*start, *end)
}

/// 📏️ Chord length of an axis.
pub fn chord(axis: &Axis) -> f64 {
    let (start, end) = ends(axis);
    (end.x - start.x).hypot(end.y - start.y)
}

/// 📏️ Arc length of an axis.
pub fn length(axis: &Axis) -> f64 {
    let sweep = match axis {
        Axis::Line { .. } => 0.0,
        Axis::Arc { bulge, .. } => 4.0 * bulge.atan().abs(),
    };
    if sweep < 1e-12 {
        chord(axis)
    } else {
        sweep * chord(axis) / (2.0 * (sweep / 2.0).sin())
    }
}

/// 🚫 Why an axis cannot carry a wall: non-finite or coinciding end points, a non-finite bulge, or an arc that is a sliver or a full turn.
pub fn flaw(axis: &Axis) -> Option<Flaw> {
    let (start, end) = ends(axis);
    if ![start.x, start.y, end.x, end.y].iter().all(|value| value.is_finite()) {
        return invariant(&[], "A wall axis needs finite coordinates.");
    }
    if chord(axis) <= EPS {
        return invariant(&[], "A wall axis must have length.");
    }
    match axis {
        Axis::Arc { bulge, .. } if !bulge.is_finite() => invariant(&["bulge"], "An arc bulge must be a finite number."),
        Axis::Arc { bulge, .. } if !(EPS..TAU - EPS).contains(&(4.0 * bulge.atan().abs())) => invariant(&["bulge"], "An arc must sweep between a sliver and a full turn; use a line for a straight wall."),
        _ => None,
    }
}

/// 🪞 The same curve run the other way: end points swap and the bulge changes sign.
pub fn flipped(axis: &Axis) -> Axis {
    match axis {
        Axis::Line { start, end } => Axis::Line { start: *end, end: *start },
        Axis::Arc { start, end, bulge } => Axis::Arc { start: *end, end: *start, bulge: 0.0 - bulge },
    }
}

fn point_at(axis: &Axis, t: f64) -> Point2 {
    match axis {
        Axis::Line { start, end } => Point2 { x: start.x + t * (end.x - start.x), y: start.y + t * (end.y - start.y) },
        Axis::Arc { start, end, bulge } => {
            let sweep = 4.0 * bulge.atan();
            let (dx, dy, chord) = (end.x - start.x, end.y - start.y, chord(axis));
            let radius = chord / (2.0 * (sweep / 2.0).sin().abs());
            let lift = chord / 2.0 / (sweep / 2.0).tan();
            let centre = Point2 { x: (start.x + end.x) / 2.0 - lift * dy / chord, y: (start.y + end.y) / 2.0 + lift * dx / chord };
            let angle = (start.y - centre.y).atan2(start.x - centre.x) + t * sweep;
            Point2 { x: centre.x + radius * angle.cos(), y: centre.y + radius * angle.sin() }
        }
    }
}

/// ✂️ The two parts of a split axis and the arc length of the first.
#[derive(Clone, Debug, PartialEq)]
pub struct Split {
    pub first: Axis,
    pub second: Axis,
    pub at: f64,
}

/// ✂️ Splits an axis at the fraction `t` of its arc length; none when `t` is not strictly inside or a part would be no valid axis.
pub fn split(axis: &Axis, t: f64) -> Option<Split> {
    if flaw(axis).is_some() || !(t > 0.0 && t < 1.0) {
        return None;
    }
    let (start, end) = ends(axis);
    let middle = point_at(axis, t);
    let middle = Point2 { x: snap(middle.x), y: snap(middle.y) };
    let part = |from: Point2, to: Point2, share: f64| match axis {
        Axis::Line { .. } => Axis::Line { start: from, end: to },
        Axis::Arc { bulge, .. } => Axis::Arc { start: from, end: to, bulge: snap_bulge((share * bulge.atan()).tan()) },
    };
    let (first, second) = (part(start, middle, t), part(middle, end, 1.0 - t));
    (flaw(&first).is_none() && flaw(&second).is_none()).then(|| Split { at: snap(t * length(axis)), first, second })
}
//#endregion 🔖️Axis

//#region 🔖️CurtainWall
fn positive(path: &[&str], value: f64, message: &str) -> Option<Flaw> {
    (!(value.is_finite() && value > 0.0)).then(|| Flaw::new(OutcomeCode::Invariant, path, message))
}

fn finite(path: &[&str], value: f64, message: &str) -> Option<Flaw> {
    (!value.is_finite()).then(|| Flaw::new(OutcomeCode::Invariant, path, message))
}

/// 🚫 Why a top constraint cannot cap an element of `storey`: a non-positive free height, a non-finite offset, or a storey of another building.
pub fn top_flaw(base: &ModelSnapshot, storey: &str, top: &TopConstraint) -> Option<Flaw> {
    match top {
        TopConstraint::Unconnected { height } => positive(&["top", "height"], *height, "A free height must be a positive length."),
        TopConstraint::StoreyTop { offset } => finite(&["top", "offset"], *offset, "A top offset must be finite."),
        TopConstraint::Roof { .. } | TopConstraint::Slab { .. } | TopConstraint::Ceiling { .. } => invariant(&["top"], "Only a wall attaches its top to a roof, a slab or a ceiling."),
        TopConstraint::Storey { storey: target, offset } => finite(&["top", "offset"], *offset, "A top offset must be finite.").or_else(|| match (base.storeys.get(target), base.storeys.get(storey)) {
            (None, _) => Some(Flaw::new(OutcomeCode::TargetMissing, &["top", "storey"], format!("Storey \"{target}\" does not exist."))),
            (Some(row), Some(own)) if row.building != own.building => invariant(&["top", "storey"], &format!("Storey \"{target}\" belongs to another building.")),
            _ => None,
        }),
    }
}

/// 🚫 Why a grid spacing is no spacing: it must be a positive length.
pub fn spacing_flaw(field: &str, value: f64) -> Option<Flaw> {
    positive(&[field], value, "A grid spacing must be a positive length.")
}

/// 🚫 Why a mullion profile cannot be swept: non-positive dimensions, a web or flange eating the section, or an outline without area.
pub fn profile_flaw(profile: &Profile) -> Option<Flaw> {
    let message = "A mullion profile needs positive dimensions.";
    match profile {
        Profile::Rectangle { width, depth } => positive(&[], *width, message).or_else(|| positive(&[], *depth, message)),
        Profile::Circle { diameter } => positive(&[], *diameter, message),
        Profile::IShape { width, depth, web, flange } => [width, depth, web, flange].into_iter().find_map(|value| positive(&[], *value, message)).or_else(|| (*web >= *width || 2.0 * *flange >= *depth).then(|| Flaw::new(OutcomeCode::Invariant, &[], "The web and flanges must fit inside the profile."))),
        Profile::Custom { outline } => (outline.len() < 3 || outline.iter().any(|vertex| !(vertex.point.x.is_finite() && vertex.point.y.is_finite() && vertex.bulge.is_finite()))).then(|| Flaw::new(OutcomeCode::Invariant, &[], "A custom profile needs an outline of at least three finite vertices.")),
        Profile::Family { family } => family.trim().is_empty().then(|| Flaw::new(OutcomeCode::Invariant, &[], "A family profile needs the id of a profile family.")),
    }
}

/// 🚫 Why a material reference is dangling.
pub fn material_flaw(base: &ModelSnapshot, field: &str, id: &str) -> Option<Flaw> {
    (!base.materials.contains_key(id)).then(|| Flaw::new(OutcomeCode::TargetMissing, &[field], format!("Material \"{id}\" does not exist.")))
}

/// 🚫 Why a grid rule is no rule: a spacing is a positive length, explicit lines lie at positive finite distances in strictly ascending order.
pub fn grid_flaw(grid: &CurtainGrid) -> Option<Flaw> {
    match grid {
        CurtainGrid::Spacing { spacing } => spacing_flaw("spacing", *spacing),
        CurtainGrid::Lines { positions } => positions
            .iter()
            .any(|position| !(position.is_finite() && *position > 0.0))
            .then(|| Flaw::new(OutcomeCode::Invariant, &["positions"], "A grid line lies at a positive, finite distance."))
            .or_else(|| positions.windows(2).any(|pair| pair[1] <= pair[0]).then(|| Flaw::new(OutcomeCode::Invariant, &["positions"], "Grid lines are listed in strictly ascending order."))),
    }
}

/// 🚫 Why a panel cannot fill a cell: a solid panel names a missing material, a door or window panel a missing type.
pub fn panel_flaw(base: &ModelSnapshot, panel: &CurtainPanel) -> Option<Flaw> {
    match panel {
        CurtainPanel::Glass | CurtainPanel::Empty => None,
        CurtainPanel::Solid { material } => material_flaw(base, "material", material),
        CurtainPanel::Door { door_type } => (!base.door_types.contains_key(door_type)).then(|| Flaw::new(OutcomeCode::TargetMissing, &["door_type"], format!("Door type \"{door_type}\" does not exist."))),
        CurtainPanel::Window { window_type } => (!base.window_types.contains_key(window_type)).then(|| Flaw::new(OutcomeCode::TargetMissing, &["window_type"], format!("Window type \"{window_type}\" does not exist."))),
    }
}

/// 🚫 The first reason a whole curtain wall type record is invalid against `base`, with paths relative to the record.
pub fn curtain_type_flaw(base: &ModelSnapshot, kind: &CurtainWallType) -> Option<Flaw> {
    grid_flaw(&kind.u_grid).map(|flaw| flaw.under(&["u_grid"]))
        .or_else(|| grid_flaw(&kind.v_grid).map(|flaw| flaw.under(&["v_grid"])))
        .or_else(|| profile_flaw(&kind.interior_mullion).map(|flaw| flaw.under(&["interior_mullion"])))
        .or_else(|| profile_flaw(&kind.border_mullion).map(|flaw| flaw.under(&["border_mullion"])))
        .or_else(|| panel_flaw(base, &kind.panel).map(|flaw| flaw.under(&["panel"])))
        .or_else(|| material_flaw(base, "panel_material", &kind.panel_material))
        .or_else(|| material_flaw(base, "mullion_material", &kind.mullion_material))
}

/// 🚫 The first reason a whole curtain wall record is invalid against `base`, with paths relative to the record.
pub fn curtain_wall_flaw(base: &ModelSnapshot, wall: &CurtainWall) -> Option<Flaw> {
    (!base.storeys.contains_key(&wall.storey)).then(|| Flaw::new(OutcomeCode::TargetMissing, &["storey"], format!("Storey \"{}\" does not exist.", wall.storey)))
        .or_else(|| (!base.curtain_wall_types.contains_key(&wall.curtain_wall_type)).then(|| Flaw::new(OutcomeCode::TargetMissing, &["curtain_wall_type"], format!("Curtain wall type \"{}\" does not exist.", wall.curtain_wall_type))))
        .or_else(|| top_flaw(base, &wall.storey, &wall.top))
        .or_else(|| flaw(&wall.axis).map(|flaw| flaw.under(&["axis"])))
        .or_else(|| finite(&["base_offset"], wall.base_offset, "A base offset must be finite."))
        .or_else(|| wall.u_grid.as_ref().and_then(grid_flaw).map(|flaw| flaw.under(&["u_grid"])))
        .or_else(|| wall.v_grid.as_ref().and_then(grid_flaw).map(|flaw| flaw.under(&["v_grid"])))
}
//#endregion 🔖️CurtainWall

//#region 🔖️Frame
pub use crate::standards::v1::subsets::any::schema::inferences::element_solids::columns::MAX_TILT;

/// 🚫 Why a lean is no lean: the direction must be finite and the angle a positive, finite angle of at most [`MAX_TILT`]; plumb is the absence of a tilt.
pub fn tilt_flaw(tilt: &Slope) -> Option<Flaw> {
    if !tilt.direction.is_finite() {
        return Some(Flaw::new(OutcomeCode::Invariant, &["direction"], "A tilt direction must be finite."));
    }
    (!(tilt.angle.is_finite() && tilt.angle > 0.0 && tilt.angle <= MAX_TILT)).then(|| Flaw::new(OutcomeCode::Invariant, &["angle"], "A tilt leans by a positive angle of at most 60 degrees; leave the tilt out for a plumb column."))
}

/// 🚫 Why an inclined beam cannot be authored: both top offsets must be finite.
pub fn offsets_flaw(top_offset: f64, end_top_offset: Option<f64>) -> Option<Flaw> {
    finite(&["top_offset"], top_offset, "A beam top offset must be finite.").or_else(|| end_top_offset.and_then(|offset| finite(&["end_top_offset"], offset, "A beam end top offset must be finite.")))
}
//#endregion 🔖️Frame

//#region 🔖️Testing
/// 🧪️ Helpers shared by the wall leaf tests: fixture decoding, tolerant comparison and the code of a refusal.
#[cfg(test)]
pub mod testing {
    use crate::standards::v1::subsets::any::schema::inferences::wall_layout::{compute_wall_layout, WallLayout};
    use crate::{ModelDiff, ModelMutation, ModelSnapshot};
    use protocol::Mutation;
    use semio_framework_diagnostic::Severity;
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

    /// 🧱️ The inferred layout of one wall.
    pub fn layout(snapshot: &ModelSnapshot, id: &str) -> WallLayout {
        compute_wall_layout(snapshot).remove(id).expect("the wall has a layout")
    }

    /// 📸️ Decodes a committed snapshot fixture.
    pub fn decode(text: &str) -> ModelSnapshot {
        from_json_str(text, JsonMemberPolicy::Reject).expect("fixture decodes")
    }

    /// 🧮️ Equal within a relative tolerance of a nanometre.
    pub fn close(left: f64, right: f64) -> bool {
        (left - right).abs() <= 1e-9 * right.abs().max(1.0)
    }

    /// 🚫 The code of the single refusal of `mutation` against `base`, asserting that it produced no diff.
    pub fn refusal(mutation: &ModelMutation, base: &ModelSnapshot) -> String {
        let (diff, messages) = mutation.diff(base).into_parts();
        assert_eq!(diff, ModelDiff::default(), "a refused mutation produces an empty diff");
        messages.iter().find(|message| matches!(message.level, Severity::Error | Severity::Fatal)).expect("the mutation is refused").code.0.to_string()
    }
}
//#endregion 🔖️Testing
