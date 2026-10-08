//! ✅️ Authored-value validity rules shared by every leaf that writes a length, a profile or an outline: pure, total and base-free.

use super::{Baluster, Infill, Point2, Profile, Railing, RiserKind, Stair, StairStringer, StringerKind, Vertex};

const AREA_EPSILON: f64 = 1e-9;

/// 📏️ A finite length above zero.
pub fn is_positive_length(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// 📐️ A finite length of zero or more.
pub fn is_non_negative_length(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn segment_area(from: &Vertex, to: &Vertex) -> f64 {
    if from.bulge == 0.0 {
        return 0.0;
    }
    let sweep = 4.0 * from.bulge.atan();
    let chord = (to.point.x - from.point.x).hypot(to.point.y - from.point.y);
    let half = (sweep / 2.0).sin();
    chord * chord / (8.0 * half * half) * (sweep - sweep.sin())
}

/// 🧮️ Signed area of a closed loop of bulged segments: positive when the loop runs counter-clockwise.
pub fn signed_loop_area(outline: &[Vertex]) -> f64 {
    let count = outline.len();
    (0..count)
        .map(|index| {
            let (from, to) = (&outline[index], &outline[(index + 1) % count]);
            (from.point.x * to.point.y - to.point.x * from.point.y) / 2.0 + segment_area(from, to)
        })
        .sum()
}

fn side(origin: Point2, a: Point2, b: Point2) -> f64 {
    (a.x - origin.x) * (b.y - origin.y) - (a.y - origin.y) * (b.x - origin.x)
}

fn chords_cross(first: (Point2, Point2), second: (Point2, Point2)) -> bool {
    side(second.0, second.1, first.0) * side(second.0, second.1, first.1) < 0.0 && side(first.0, first.1, second.0) * side(first.0, first.1, second.1) < 0.0
}

/// 🔲️ Why a closed outline cannot bound a section, none when it can: at least three finite vertices, no repeated vertex (the
/// loop closes itself), no crossing chords and a counter-clockwise orientation with an area.
pub fn outline_problem(outline: &[Vertex]) -> Option<&'static str> {
    let count = outline.len();
    if count < 3 {
        return Some("A custom outline needs at least three vertices.");
    }
    if outline.iter().any(|vertex| !(vertex.point.x.is_finite() && vertex.point.y.is_finite() && vertex.bulge.is_finite())) {
        return Some("A custom outline must be made of finite coordinates and bulges.");
    }
    if (0..count).any(|index| outline[index].point == outline[(index + 1) % count].point) {
        return Some("A custom outline must not repeat a vertex; it closes itself.");
    }
    let chord = |index: usize| (outline[index].point, outline[(index + 1) % count].point);
    if (0..count).any(|first| ((first + 2)..count).filter(|second| (second + 1) % count != first).any(|second| chords_cross(chord(first), chord(second)))) {
        return Some("A custom outline must not cross itself.");
    }
    if signed_loop_area(outline) <= AREA_EPSILON {
        return Some("A custom outline must run counter-clockwise and enclose an area.");
    }
    None
}

/// ▭️ Why a profile cannot be swept, none when it can: positive dimensions, an I web and flanges that fit inside the section,
/// a valid closed outline for a custom profile.
pub fn profile_problem(profile: &Profile) -> Option<&'static str> {
    match profile {
        Profile::Rectangle { width, depth } => (!(is_positive_length(*width) && is_positive_length(*depth))).then_some("A rectangle profile needs a positive width and depth."),
        Profile::Circle { diameter } => (!is_positive_length(*diameter)).then_some("A circle profile needs a positive diameter."),
        Profile::IShape { width, depth, web, flange } => {
            if ![width, depth, web, flange].into_iter().all(|value| is_positive_length(*value)) {
                Some("An I profile needs a positive width, depth, web and flange.")
            } else if *web >= *width || 2.0 * *flange >= *depth {
                Some("The web and flanges of an I profile must fit inside its width and depth.")
            } else {
                None
            }
        }
        Profile::Custom { outline } => outline_problem(outline),
    }
}

//#region 🔖️Defaults
/// 🗺️ Plan view convention: the height above a storey elevation at which the plan cuts when the storey does not author `cut_height`.
pub const DEFAULT_CUT_HEIGHT: f64 = 1.2;

/// 🪵️ Stringer of a new stair: none; the width and depth are kept ready for a stringer kind chosen later.
pub const STANDARD_STRINGER: StairStringer = StairStringer { kind: StringerKind::None, width: 0.05, depth: 0.25 };

/// 🪜️ Vertical thickness of a new stair tread, in metres.
pub const STANDARD_TREAD_THICKNESS: f64 = 0.04;

/// 🪜️ Riser boards of a new stair.
pub const STANDARD_RISER: RiserKind = RiserKind::Closed;

/// 🛤️ Top rail section of a new railing: 60 mm wide, 40 mm deep.
pub fn standard_rail_profile() -> Profile {
    Profile::Rectangle { width: 0.06, depth: 0.04 }
}

/// 🛤️ Post section of a new railing: 50 mm square.
pub fn standard_post_profile() -> Profile {
    Profile::Rectangle { width: 0.05, depth: 0.05 }
}

/// 🛤️ Infill of a new railing: none.
pub const STANDARD_INFILL: Infill = Infill::None;
//#endregion 🔖️Defaults

//#region 🔖️StairConstruction
/// 🪵️ Why the construction of a stair is refused, as the field below the stair record and the message, none when it holds: a stringer
/// kind other than none needs a positive width and depth, the nosing is finite, non-negative and shorter than the minimum tread, the
/// tread is a positive thickness and the landing a positive depth.
pub fn stair_construction_problem(stair: &Stair) -> Option<(&'static str, &'static str)> {
    let stringer = &stair.stringer;
    let sized = is_non_negative_length(stringer.width) && is_non_negative_length(stringer.depth) && (stringer.kind == StringerKind::None || (stringer.width > 0.0 && stringer.depth > 0.0));
    if !sized {
        return Some(("stringer", "A stringer needs finite dimensions, positive ones unless its kind is none."));
    }
    if !(is_non_negative_length(stair.nosing) && stair.nosing < stair.min_tread) {
        return Some(("nosing", "A nosing must be a non-negative length shorter than the minimum tread."));
    }
    if !is_positive_length(stair.tread_thickness) {
        return Some(("tread_thickness", "A tread thickness must be a positive length."));
    }
    if !is_positive_length(stair.landing_depth) {
        return Some(("landing_depth", "A landing depth must be a positive length."));
    }
    None
}
//#endregion 🔖️StairConstruction

//#region 🔖️RailingConstruction
fn baluster_problem(baluster: &Baluster) -> Option<(&'static str, &'static str)> {
    if let Some(message) = profile_problem(&baluster.profile) {
        return Some(("baluster", message));
    }
    (!is_positive_length(baluster.spacing)).then_some(("baluster", "A baluster spacing must be a positive length."))
}

/// 🛤️ Why the construction of a railing is refused, as the field below the railing record and the message, none when it holds: sound
/// top rail and post sections, a baluster row with a sound section and a positive spacing, a positive infill thickness.
pub fn railing_construction_problem(railing: &Railing) -> Option<(&'static str, &'static str)> {
    if let Some(message) = profile_problem(&railing.profile) {
        return Some(("profile", message));
    }
    if let Some(message) = profile_problem(&railing.post_profile) {
        return Some(("post_profile", message));
    }
    if let Some(problem) = railing.baluster.as_ref().and_then(baluster_problem) {
        return Some(problem);
    }
    match railing.infill {
        Infill::Glass { thickness } | Infill::Panel { thickness } if !is_positive_length(thickness) => Some(("infill", "An infill thickness must be a positive length.")),
        _ => None,
    }
}
//#endregion 🔖️RailingConstruction

//#region 🔖️CutHeight
/// ✂️ Why a plan cut height is refused, none when it holds: a finite length above zero.
pub fn cut_height_problem(cut_height: f64) -> Option<&'static str> {
    (!is_positive_length(cut_height)).then_some("A plan cut height must be a positive length.")
}
//#endregion 🔖️CutHeight
