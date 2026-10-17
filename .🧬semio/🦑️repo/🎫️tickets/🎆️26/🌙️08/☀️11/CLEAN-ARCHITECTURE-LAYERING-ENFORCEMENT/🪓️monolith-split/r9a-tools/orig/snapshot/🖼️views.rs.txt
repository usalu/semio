//! 🖼️ Authored-view rules shared by the three view leaves, the editor and the `view-linework` inference: which fields a view kind owns, the conventions of its defaults and
//! why a view cannot be written. Pure, total and read-only: a refusal names the field below the view record and carries the message.

use super::{DetailLevel, ModelSnapshot, Phase, Point2, View, ViewCamera, ViewCategory, ViewCrop, ViewKind, ViewPlane, DEFAULT_CUT_HEIGHT};
use std::f64::consts::FRAC_PI_2;

//#region 🔖️Defaults
/// 🪑️ Ceiling plan convention: the height above the storey elevation at which a ceiling plan cuts when the view authors none.
pub const DEFAULT_CEILING_CUT_HEIGHT: f64 = 2.1;

/// 🔭️ The view depth of a new view: deep enough to see the whole model.
pub const DEFAULT_VIEW_DEPTH: f64 = 100.0;

/// 📏️ The drawing scale denominator of a new view.
pub const DEFAULT_VIEW_SCALE: u32 = 100;

/// 📏️ The largest scale denominator a view accepts (1:10000).
pub const MAX_VIEW_SCALE: u32 = 10_000;

impl ViewKind {
    /// 🗺️ Whether the kind cuts a storey horizontally.
    pub fn is_plan(self) -> bool {
        matches!(self, Self::Plan | Self::CeilingPlan)
    }

    /// 📐️ Whether the kind looks through a vertical plane.
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Section | Self::Elevation)
    }

    /// 🎥️ Whether the kind is a camera.
    pub fn is_camera(self) -> bool {
        matches!(self, Self::Orthographic | Self::Perspective)
    }
}

impl View {
    /// 🖼️ A view of `kind` with the conventions of a new view: unlimited depth, 1:100, medium detail, nothing hidden, no crop, every phase.
    pub fn standard(building: &str, name: &str, kind: ViewKind) -> Self {
        Self {
            building: building.into(),
            name: name.into(),
            kind,
            storey: None,
            plane: None,
            camera: None,
            cut_height: None,
            depth: DEFAULT_VIEW_DEPTH,
            crop: None,
            hidden: Vec::new(),
            phase: None,
            scale: DEFAULT_VIEW_SCALE,
            detail: DetailLevel::Medium,
        }
    }

    /// 🗺️ A plan or ceiling plan of `storey`.
    pub fn of_storey(building: &str, name: &str, kind: ViewKind, storey: &str) -> Self {
        Self { storey: Some(storey.into()), ..Self::standard(building, name, kind) }
    }

    /// 📐️ A section or elevation through `plane`.
    pub fn through(building: &str, name: &str, kind: ViewKind, plane: ViewPlane) -> Self {
        Self { plane: Some(plane), ..Self::standard(building, name, kind) }
    }

    /// 🎥️ Whether the view hides the category.
    pub fn hides(&self, category: ViewCategory) -> bool {
        self.hidden.contains(&category)
    }

    /// 🕰️ Whether the phase filter lets an element of `phase` through.
    pub fn shows_phase(&self, phase: Phase) -> bool {
        self.phase.is_none_or(|wanted| wanted == phase)
    }
}

/// ✂️ The height above the storey elevation at which a plan or ceiling plan cuts: the view's own, else the storey's, else the plan convention (the ceiling convention for a ceiling plan); none for every other kind or a missing storey.
pub fn view_cut_height(base: &ModelSnapshot, view: &View) -> Option<f64> {
    let storey = base.storeys.get(view.storey.as_ref()?)?;
    match view.kind {
        ViewKind::Plan => Some(view.cut_height.or(storey.cut_height).unwrap_or(DEFAULT_CUT_HEIGHT)),
        ViewKind::CeilingPlan => Some(view.cut_height.unwrap_or(DEFAULT_CEILING_CUT_HEIGHT)),
        _ => None,
    }
}
//#endregion 🔖️Defaults

//#region 🔖️Problems
/// 🚫️ Why a view cannot be written: the field below the view record, the human message and whether a referenced record is absent (else a value breaks an invariant).
#[derive(Clone, Debug, PartialEq)]
pub struct ViewProblem {
    pub missing: bool,
    pub field: &'static str,
    pub message: String,
}

fn invariant(field: &'static str, message: impl Into<String>) -> Option<ViewProblem> {
    Some(ViewProblem { missing: false, field, message: message.into() })
}

fn missing(field: &'static str, message: impl Into<String>) -> Option<ViewProblem> {
    Some(ViewProblem { missing: true, field, message: message.into() })
}

fn finite(point: Point2) -> bool {
    point.x.is_finite() && point.y.is_finite()
}

fn plane_problem(plane: &ViewPlane) -> Option<ViewProblem> {
    if !(finite(plane.start) && finite(plane.end)) {
        return invariant("plane", "A view plane needs finite end points.");
    }
    (plane.start == plane.end).then(|| ViewProblem { missing: false, field: "plane", message: "A view plane needs a length: its end points must differ.".into() })
}

fn camera_problem(camera: &ViewCamera) -> Option<ViewProblem> {
    let numbers = [camera.target.x, camera.target.y, camera.target_height, camera.azimuth, camera.pitch, camera.distance];
    if !numbers.iter().all(|value| value.is_finite()) {
        return invariant("camera", "A camera needs finite numbers.");
    }
    if !(camera.distance > 0.0) {
        return invariant("camera", "A camera needs a positive distance to its target.");
    }
    (camera.pitch.abs() >= FRAC_PI_2).then(|| ViewProblem { missing: false, field: "camera", message: "A camera pitch must stay strictly between straight down and straight up.".into() })
}

fn crop_problem(crop: &ViewCrop) -> Option<ViewProblem> {
    if !(finite(crop.min) && finite(crop.max)) {
        return invariant("crop", "A crop rectangle needs finite corners.");
    }
    (!(crop.min.x < crop.max.x && crop.min.y < crop.max.y)).then(|| ViewProblem { missing: false, field: "crop", message: "A crop rectangle needs its minimum corner below and left of its maximum corner.".into() })
}

fn category_rank(category: ViewCategory) -> u8 {
    category as u8
}

/// 🚫️ Why `view` (to be stored under `id`) cannot be written, none when it can: its building exists and the name is filled and unique within it; each kind owns exactly its fields (a plan or ceiling plan a storey of the building, a
/// section or elevation a plane, a camera a camera); the cut height is a positive length of a plan kind; depth is a positive length and scale an integer in `1..=10000`; the crop has a size; hidden categories are unique and in category order.
pub fn view_problem(base: &ModelSnapshot, id: &str, view: &View) -> Option<ViewProblem> {
    if !base.buildings.contains_key(&view.building) {
        return missing("building", format!("Building \"{}\" does not exist.", view.building));
    }
    if view.name.trim().is_empty() {
        return invariant("name", "A view name must not be blank.");
    }
    if base.views.iter().any(|(other, row)| other != id && row.building == view.building && row.name == view.name) {
        return invariant("name", format!("Building \"{}\" already has a view named \"{}\".", view.building, view.name));
    }
    let (plan, vertical, camera) = (view.kind.is_plan(), view.kind.is_vertical(), view.kind.is_camera());
    match (&view.storey, plan) {
        (None, true) => return invariant("storey", "A plan view needs a storey."),
        (Some(_), false) => return invariant("storey", "Only a plan view has a storey."),
        (Some(storey), true) => match base.storeys.get(storey) {
            None => return missing("storey", format!("Storey \"{storey}\" does not exist.")),
            Some(row) if row.building != view.building => return invariant("storey", format!("Storey \"{storey}\" belongs to another building.")),
            Some(_) => {}
        },
        (None, false) => {}
    }
    match (&view.plane, vertical) {
        (None, true) => return invariant("plane", "A section or elevation needs a plane."),
        (Some(_), false) => return invariant("plane", "Only a section or elevation has a plane."),
        (Some(plane), true) => {
            if let Some(problem) = plane_problem(plane) {
                return Some(problem);
            }
        }
        (None, false) => {}
    }
    match (&view.camera, camera) {
        (None, true) => return invariant("camera", "An orthographic or perspective view needs a camera."),
        (Some(_), false) => return invariant("camera", "Only an orthographic or perspective view has a camera."),
        (Some(row), true) => {
            if let Some(problem) = camera_problem(row) {
                return Some(problem);
            }
        }
        (None, false) => {}
    }
    if let Some(cut) = view.cut_height {
        if !plan {
            return invariant("cut_height", "Only a plan view has a cut height.");
        }
        if !(cut.is_finite() && cut > 0.0) {
            return invariant("cut_height", "A cut height must be a positive length.");
        }
    }
    if !(view.depth.is_finite() && view.depth > 0.0) {
        return invariant("depth", "A view depth must be a positive length.");
    }
    if let Some(crop) = &view.crop {
        if camera {
            return invariant("crop", "A camera has no crop rectangle.");
        }
        if let Some(problem) = crop_problem(crop) {
            return Some(problem);
        }
    }
    if view.hidden.windows(2).any(|pair| category_rank(pair[0]) >= category_rank(pair[1])) {
        return invariant("hidden", "Hidden categories must be unique and in category order.");
    }
    if !(1..=MAX_VIEW_SCALE).contains(&view.scale) {
        return invariant("scale", format!("A view scale must be between 1 and {MAX_VIEW_SCALE}."));
    }
    None
}
//#endregion 🔖️Problems

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
