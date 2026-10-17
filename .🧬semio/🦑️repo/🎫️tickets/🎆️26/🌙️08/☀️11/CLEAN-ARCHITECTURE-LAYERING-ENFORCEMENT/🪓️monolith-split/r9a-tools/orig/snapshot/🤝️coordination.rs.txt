//! 🤝️ Authored coordination rules shared by the twelve clash-set, rule, issue and comment leaves, the `clash-sets` and `rule-results` inferences, the BCF exchange and the editor: which class an element is,
//! which storey and phase it has, whether a selector picks it, and why a clash set, a rule, an issue or a comment cannot be written. Pure, total and read-only over AUTHORED data: a refusal names the field below
//! the record and carries the message.
//! 📎 https://github.com/buildingSMART/BCF-XML

use super::{ClashRef, ClashSet, ElementClass, ElementSelector, Issue, IssueComment, IssueViewpoint, ModelSnapshot, OpeningKind, Phase, Rule, RuleKind, RuleScope};

//#region 🔖️Defaults
/// 📏️ The largest tolerance of a clash set in metres.
pub const MAX_TOLERANCE: f64 = 1.0;

/// 📏️ The largest clearance of a clash set in metres.
pub const MAX_CLEARANCE: f64 = 10.0;

/// 📏️ The tolerance of a new clash set in metres: an overlap of a millimetre is a modelling artefact, not a clash.
pub const DEFAULT_TOLERANCE: f64 = 0.001;

/// 📐️ The largest ramp slope a rule accepts as a limit (rise over run, 45 degrees).
pub const MAX_SLOPE_LIMIT: f64 = 1.0;

/// 📐️ The largest limit of any other rule (metres or square metres).
pub const MAX_LIMIT: f64 = 1_000_000.0;

/// 🧩️ Every class of the selectors in the order the editor offers them.
pub const CLASSES: [ElementClass; 15] = [
    ElementClass::Wall,
    ElementClass::CurtainWall,
    ElementClass::Window,
    ElementClass::Door,
    ElementClass::Column,
    ElementClass::Beam,
    ElementClass::Slab,
    ElementClass::Ceiling,
    ElementClass::Roof,
    ElementClass::Stair,
    ElementClass::Ramp,
    ElementClass::Railing,
    ElementClass::WallSweep,
    ElementClass::Component,
    ElementClass::Mep,
];

/// 📏️ Every rule kind in the order the editor offers them.
pub const RULE_KINDS: [RuleKind; 8] = [
    RuleKind::MinClearHeight,
    RuleKind::MaxRiser,
    RuleKind::MinTread,
    RuleKind::MinStairWidth,
    RuleKind::MinDoorWidth,
    RuleKind::MaxRampSlope,
    RuleKind::MinCorridorWidth,
    RuleKind::MaxCompartmentArea,
];

impl ElementSelector {
    /// 🎯️ The selector that picks every element.
    pub fn all() -> Self {
        Self { classes: Vec::new(), storeys: Vec::new(), phases: Vec::new(), ids: Vec::new() }
    }

    /// 🎯️ The selector of the given classes.
    pub fn of_classes(classes: &[ElementClass]) -> Self {
        Self { classes: classes.to_vec(), ..Self::all() }
    }
}

impl RuleScope {
    /// 🎯️ The scope that covers everything.
    pub fn all() -> Self {
        Self { storeys: Vec::new(), phases: Vec::new(), ids: Vec::new(), filter: String::new() }
    }
}

impl ClashSet {
    /// 💥️ A clash set of `a` against `b` with the conventions of a new one: a tolerance of a millimetre and no clearance.
    pub fn standard(name: &str, a: ElementSelector, b: ElementSelector) -> Self {
        Self { name: name.into(), a, b, tolerance: DEFAULT_TOLERANCE, clearance: 0.0 }
    }
}

impl ElementClass {
    /// 🏷️ The stable lower-case name of the class (the heading of the clash panel and the key of the BCF labels).
    pub fn name(self) -> &'static str {
        match self {
            Self::Wall => "wall",
            Self::CurtainWall => "curtain-wall",
            Self::Window => "window",
            Self::Door => "door",
            Self::Column => "column",
            Self::Beam => "beam",
            Self::Slab => "slab",
            Self::Ceiling => "ceiling",
            Self::Roof => "roof",
            Self::Stair => "stair",
            Self::Ramp => "ramp",
            Self::Railing => "railing",
            Self::WallSweep => "wall-sweep",
            Self::Component => "component",
            Self::Mep => "mep",
        }
    }

    /// 🔎️ The class a name spells, ignoring case.
    pub fn parse(text: &str) -> Option<Self> {
        CLASSES.into_iter().find(|class| class.name().eq_ignore_ascii_case(text.trim()))
    }
}

impl RuleKind {
    /// 🏷️ The stable name of the rule kind.
    pub fn name(self) -> &'static str {
        match self {
            Self::MinClearHeight => "min-clear-height",
            Self::MaxRiser => "max-riser",
            Self::MinTread => "min-tread",
            Self::MinStairWidth => "min-stair-width",
            Self::MinDoorWidth => "min-door-width",
            Self::MaxRampSlope => "max-ramp-slope",
            Self::MinCorridorWidth => "min-corridor-width",
            Self::MaxCompartmentArea => "max-compartment-area",
        }
    }

    /// 🔎️ The rule kind a name spells, ignoring case.
    pub fn parse(text: &str) -> Option<Self> {
        RULE_KINDS.into_iter().find(|kind| kind.name().eq_ignore_ascii_case(text.trim()))
    }

    /// ⬆️ Whether the limit is a maximum (the measure must not exceed it) and not a minimum.
    pub fn is_maximum(self) -> bool {
        matches!(self, Self::MaxRiser | Self::MaxRampSlope | Self::MaxCompartmentArea)
    }

    /// 📏️ The largest limit the kind accepts.
    pub fn largest_limit(self) -> f64 {
        if self == Self::MaxRampSlope {
            MAX_SLOPE_LIMIT
        } else {
            MAX_LIMIT
        }
    }
}
//#endregion 🔖️Defaults

//#region 🔖️Elements
/// 🧩️ The class of the element `id`, none for an id that is no element with a solid. An opening is a window or a door by its kind; a void has no class.
pub fn class_of(base: &ModelSnapshot, id: &str) -> Option<ElementClass> {
    if base.walls.contains_key(id) {
        return Some(ElementClass::Wall);
    }
    if base.curtain_walls.contains_key(id) {
        return Some(ElementClass::CurtainWall);
    }
    if let Some(opening) = base.openings.get(id) {
        return match opening.kind {
            OpeningKind::Window { .. } => Some(ElementClass::Window),
            OpeningKind::Door { .. } => Some(ElementClass::Door),
            OpeningKind::Void { .. } => None,
        };
    }
    [
        (base.columns.contains_key(id), ElementClass::Column),
        (base.beams.contains_key(id), ElementClass::Beam),
        (base.slabs.contains_key(id), ElementClass::Slab),
        (base.ceilings.contains_key(id), ElementClass::Ceiling),
        (base.roofs.contains_key(id), ElementClass::Roof),
        (base.stairs.contains_key(id), ElementClass::Stair),
        (base.ramps.contains_key(id), ElementClass::Ramp),
        (base.railings.contains_key(id), ElementClass::Railing),
        (base.wall_sweeps.contains_key(id), ElementClass::WallSweep),
        (base.components.contains_key(id), ElementClass::Component),
        (base.mep_elements.contains_key(id), ElementClass::Mep),
    ]
    .into_iter()
    .find_map(|(found, class)| found.then_some(class))
}

/// 🪜️ The storey the element `id` stands on: its own, the storey of the host for an opening or a wall sweep; none for an id without a storey.
pub fn storey_of<'a>(base: &'a ModelSnapshot, id: &str) -> Option<&'a String> {
    let host = |host: &str| base.walls.get(host).map(|row| &row.storey).or_else(|| base.curtain_walls.get(host).map(|row| &row.storey));
    base.walls
        .get(id)
        .map(|row| &row.storey)
        .or_else(|| base.curtain_walls.get(id).map(|row| &row.storey))
        .or_else(|| base.columns.get(id).map(|row| &row.storey))
        .or_else(|| base.beams.get(id).map(|row| &row.storey))
        .or_else(|| base.slabs.get(id).map(|row| &row.storey))
        .or_else(|| base.ceilings.get(id).map(|row| &row.storey))
        .or_else(|| base.roofs.get(id).map(|row| &row.storey))
        .or_else(|| base.stairs.get(id).map(|row| &row.storey))
        .or_else(|| base.ramps.get(id).map(|row| &row.storey))
        .or_else(|| base.railings.get(id).map(|row| &row.storey))
        .or_else(|| base.spaces.get(id).map(|row| &row.storey))
        .or_else(|| base.components.get(id).map(|row| &row.storey))
        .or_else(|| base.mep_elements.get(id).map(|row| &row.storey))
        .or_else(|| base.openings.get(id).and_then(|opening| host(&opening.host)))
        .or_else(|| base.wall_sweeps.get(id).and_then(|sweep| host(&sweep.host)))
}

/// 🕰️ The phase the element `id` effectively has: its own, its host's for an opening or a wall sweep, new for a kind without a phase.
pub fn phase_of(base: &ModelSnapshot, id: &str) -> Phase {
    let host = |host: &str| base.walls.get(host).map(|row| row.phase).or_else(|| base.curtain_walls.get(host).map(|row| row.phase));
    base.walls
        .get(id)
        .map(|row| row.phase)
        .or_else(|| base.curtain_walls.get(id).map(|row| row.phase))
        .or_else(|| base.columns.get(id).map(|row| row.phase))
        .or_else(|| base.beams.get(id).map(|row| row.phase))
        .or_else(|| base.slabs.get(id).map(|row| row.phase))
        .or_else(|| base.roofs.get(id).map(|row| row.phase))
        .or_else(|| base.stairs.get(id).map(|row| row.phase))
        .or_else(|| base.railings.get(id).map(|row| row.phase))
        .or_else(|| base.spaces.get(id).map(|row| row.phase))
        .or_else(|| base.openings.get(id).and_then(|opening| host(&opening.host)))
        .or_else(|| base.wall_sweeps.get(id).and_then(|sweep| host(&sweep.host)))
        .unwrap_or(Phase::New)
}

impl ElementSelector {
    /// 🎯️ Whether the selector picks the element `id` of class `class`: every restriction that is not empty must hold.
    pub fn picks(&self, base: &ModelSnapshot, id: &str, class: ElementClass) -> bool {
        (self.classes.is_empty() || self.classes.contains(&class))
            && (self.ids.is_empty() || self.ids.iter().any(|picked| picked == id))
            && (self.storeys.is_empty() || storey_of(base, id).is_some_and(|storey| self.storeys.contains(storey)))
            && (self.phases.is_empty() || self.phases.contains(&phase_of(base, id)))
    }
}

impl RuleScope {
    /// 🎯️ Whether the scope covers the element `id`, a space or a zone (`storey` is none for a zone, which stands on no storey and is covered by every storey restriction).
    pub fn covers(&self, base: &ModelSnapshot, id: &str, storey: Option<&String>) -> bool {
        (self.ids.is_empty() || self.ids.iter().any(|covered| covered == id))
            && (self.storeys.is_empty() || storey.is_none_or(|storey| self.storeys.contains(storey)))
            && (self.phases.is_empty() || self.phases.contains(&phase_of(base, id)))
    }
}

/// 🔎️ Whether `id` names a record a selector, a scope or an issue can refer to: an element with a solid, a space, a zone or a storey.
pub fn is_referable(base: &ModelSnapshot, id: &str) -> bool {
    class_of(base, id).is_some() || base.spaces.contains_key(id) || base.zones.contains_key(id) || base.storeys.contains_key(id) || base.openings.contains_key(id)
}

/// 🧱️ The ids of the elements that have a class, in id order, with the class.
pub fn classified(base: &ModelSnapshot) -> Vec<(String, ElementClass)> {
    let mut rows: Vec<(String, ElementClass)> = Vec::new();
    macro_rules! collect {
        ($($collection:ident),+) => { $( rows.extend(base.$collection.keys().filter_map(|id| class_of(base, id).map(|class| (id.clone(), class)))); )+ };
    }
    collect!(walls, curtain_walls, openings, columns, beams, slabs, ceilings, roofs, stairs, ramps, railings, wall_sweeps, components, mep_elements);
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows
}
//#endregion 🔖️Elements

//#region 🔖️Problems
/// 🚫️ Why a clash set, a rule, an issue or a comment cannot be written: the field below the record, the human message and whether a referenced record is absent (else a value breaks an invariant).
#[derive(Clone, Debug, PartialEq)]
pub struct CoordinationProblem {
    pub missing: bool,
    pub field: &'static str,
    pub message: String,
}

fn invariant(field: &'static str, message: impl Into<String>) -> Option<CoordinationProblem> {
    Some(CoordinationProblem { missing: false, field, message: message.into() })
}

fn missing(field: &'static str, message: impl Into<String>) -> Option<CoordinationProblem> {
    Some(CoordinationProblem { missing: true, field, message: message.into() })
}

/// 📅️ Whether `text` is a moment: a calendar date as `year-month-day`, optionally followed by `Thour:minute:second`, an optional fraction and `Z` or an offset `+hour:minute` / `-hour:minute` (the `xs:dateTime` of BCF).
pub fn valid_timestamp(text: &str) -> bool {
    let (date, time) = match text.split_once('T') {
        Some((date, time)) => (date, Some(time)),
        None => (text, None),
    };
    if date.is_empty() || !super::sheets::valid_date(date) {
        return false;
    }
    let Some(time) = time else { return true };
    let clock_end = time.find(['Z', '+', '-']).unwrap_or(time.len());
    let (clock, zone) = time.split_at(clock_end);
    let digits = |part: &str, width: usize, last: u32| part.len() == width && part.bytes().all(|byte| byte.is_ascii_digit()) && part.parse::<u32>().is_ok_and(|number| number <= last);
    let (seconds, fraction) = match clock.split_once('.') {
        Some((seconds, fraction)) => (seconds, Some(fraction)),
        None => (clock, None),
    };
    let parts: Vec<&str> = seconds.split(':').collect();
    let clock_ok = parts.len() == 3 && digits(parts[0], 2, 23) && digits(parts[1], 2, 59) && digits(parts[2], 2, 60) && fraction.is_none_or(|fraction| !fraction.is_empty() && fraction.bytes().all(|byte| byte.is_ascii_digit()));
    let zone_ok = match zone {
        "" | "Z" => true,
        offset => {
            let mut chars = offset.chars();
            let sign = chars.next();
            let rest: Vec<&str> = chars.as_str().split(':').collect();
            matches!(sign, Some('+' | '-')) && rest.len() == 2 && digits(rest[0], 2, 23) && digits(rest[1], 2, 59)
        }
    };
    clock_ok && zone_ok
}

fn selector_problem(base: &ModelSnapshot, field: &'static str, selector: &ElementSelector) -> Option<CoordinationProblem> {
    if let Some(storey) = selector.storeys.iter().find(|storey| !base.storeys.contains_key(*storey)) {
        return missing(field, format!("Storey \"{storey}\" does not exist."));
    }
    if let Some(id) = selector.ids.iter().find(|id| !is_referable(base, id)) {
        return missing(field, format!("Element \"{id}\" does not exist."));
    }
    let unique = |items: &[String]| items.iter().enumerate().all(|(at, item)| !items[..at].contains(item));
    if !(unique(&selector.storeys) && unique(&selector.ids) && selector.classes.iter().enumerate().all(|(at, class)| !selector.classes[..at].contains(class)) && selector.phases.iter().enumerate().all(|(at, phase)| !selector.phases[..at].contains(phase))) {
        return invariant(field, "A selector names each class, storey, phase and element at most once.");
    }
    None
}

/// 🚫️ Why `clash_set` cannot be written, none when it can: the name is filled, the tolerance is between 0 and 1 m, the clearance between 0 and 10 m and both selectors name only storeys and elements that exist, each at most once.
pub fn clash_set_problem(base: &ModelSnapshot, _id: &str, clash_set: &ClashSet) -> Option<CoordinationProblem> {
    if clash_set.name.trim().is_empty() {
        return invariant("name", "A clash set name must not be blank.");
    }
    if !(clash_set.tolerance.is_finite() && (0.0..=MAX_TOLERANCE).contains(&clash_set.tolerance)) {
        return invariant("tolerance", format!("A tolerance must be between 0 and {MAX_TOLERANCE} metres."));
    }
    if !(clash_set.clearance.is_finite() && (0.0..=MAX_CLEARANCE).contains(&clash_set.clearance)) {
        return invariant("clearance", format!("A clearance must be between 0 and {MAX_CLEARANCE} metres."));
    }
    selector_problem(base, "a", &clash_set.a).or_else(|| selector_problem(base, "b", &clash_set.b))
}

fn scope_problem(base: &ModelSnapshot, scope: &RuleScope) -> Option<CoordinationProblem> {
    if let Some(storey) = scope.storeys.iter().find(|storey| !base.storeys.contains_key(*storey)) {
        return missing("scope", format!("Storey \"{storey}\" does not exist."));
    }
    if let Some(id) = scope.ids.iter().find(|id| !is_referable(base, id)) {
        return missing("scope", format!("Element \"{id}\" does not exist."));
    }
    let unique = |items: &[String]| items.iter().enumerate().all(|(at, item)| !items[..at].contains(item));
    if !(unique(&scope.storeys) && unique(&scope.ids) && scope.phases.iter().enumerate().all(|(at, phase)| !scope.phases[..at].contains(phase))) {
        return invariant("scope", "A scope names each storey, phase and element at most once.");
    }
    None
}

/// 🚫️ Why `rule` cannot be written, none when it can: the name is filled, the limit is a positive finite number the kind accepts and the scope names only storeys and elements that exist, each at most once.
pub fn rule_problem(base: &ModelSnapshot, _id: &str, rule: &Rule) -> Option<CoordinationProblem> {
    if rule.name.trim().is_empty() {
        return invariant("name", "A rule name must not be blank.");
    }
    if !(rule.limit.is_finite() && rule.limit > 0.0 && rule.limit <= rule.kind.largest_limit()) {
        return invariant("limit", format!("The limit of this rule must be greater than 0 and at most {}.", rule.kind.largest_limit()));
    }
    scope_problem(base, &rule.scope)
}

fn point_finite(point: &super::Point3) -> bool {
    point.x.is_finite() && point.y.is_finite() && point.z.is_finite()
}

fn viewpoint_problem(base: &ModelSnapshot, viewpoint: &IssueViewpoint) -> Option<CoordinationProblem> {
    let camera = &viewpoint.camera;
    if ![camera.target.x, camera.target.y, camera.target_height, camera.azimuth, camera.pitch, camera.distance].iter().all(|value| value.is_finite()) {
        return invariant("viewpoint", "A viewpoint camera needs finite values.");
    }
    if camera.distance <= 0.0 {
        return invariant("viewpoint", "A viewpoint camera needs a distance greater than 0.");
    }
    if camera.pitch.abs() > std::f64::consts::FRAC_PI_2 {
        return invariant("viewpoint", "A viewpoint camera pitch must lie between -90 and 90 degrees.");
    }
    if let Some(section) = &viewpoint.section {
        if !(point_finite(&section.min) && point_finite(&section.max) && section.min.x < section.max.x && section.min.y < section.max.y && section.min.z < section.max.z) {
            return invariant("viewpoint", "A section box needs finite corners with the minimum below the maximum on every axis.");
        }
    }
    if let Some(id) = viewpoint.isolate.iter().find(|id| !is_referable(base, id)) {
        return missing("viewpoint", format!("Element \"{id}\" does not exist."));
    }
    None
}

fn clash_ref_problem(base: &ModelSnapshot, clash: &ClashRef) -> Option<CoordinationProblem> {
    if !base.clash_sets.contains_key(&clash.set) {
        return missing("clash", format!("Clash set \"{}\" does not exist.", clash.set));
    }
    if let Some(id) = [&clash.first, &clash.second].into_iter().find(|id| !is_referable(base, id)) {
        return missing("clash", format!("Element \"{id}\" does not exist."));
    }
    (clash.first == clash.second).then(|| CoordinationProblem { missing: false, field: "clash", message: "A clash names two different elements.".into() })
}

/// 🚫️ Why `issue` cannot be written, none when it can: the title and the author are filled, the creation moment is readable, labels are filled and unique, the elements exist and are named once, the clash it was raised from and the
/// viewpoint it shows are well formed. Elements that are deleted later stay named as history.
pub fn issue_problem(base: &ModelSnapshot, _id: &str, issue: &Issue) -> Option<CoordinationProblem> {
    if issue.title.trim().is_empty() {
        return invariant("title", "An issue title must not be blank.");
    }
    if issue.author.trim().is_empty() {
        return invariant("author", "An issue author must not be blank.");
    }
    if !valid_timestamp(&issue.created) {
        return invariant("created", "An issue creation moment must be a date as year-month-day, optionally with a time (for example 2026-10-09T08:30:00Z).");
    }
    if issue.labels.iter().any(|label| label.trim().is_empty()) || issue.labels.iter().enumerate().any(|(at, label)| issue.labels[..at].contains(label)) {
        return invariant("labels", "Issue labels must be filled and unique.");
    }
    if let Some(id) = issue.elements.iter().find(|id| !is_referable(base, id)) {
        return missing("elements", format!("Element \"{id}\" does not exist."));
    }
    if issue.elements.iter().enumerate().any(|(at, id)| issue.elements[..at].contains(id)) {
        return invariant("elements", "An issue names each element at most once.");
    }
    if let Some(problem) = issue.clash.as_ref().and_then(|clash| clash_ref_problem(base, clash)) {
        return Some(problem);
    }
    issue.viewpoint.as_ref().and_then(|viewpoint| viewpoint_problem(base, viewpoint))
}

/// 🚫️ Why `comment` cannot be written, none when it can: its issue exists, the author and the text are filled and the moment is readable.
pub fn comment_problem(base: &ModelSnapshot, _id: &str, comment: &IssueComment) -> Option<CoordinationProblem> {
    if !base.issues.contains_key(&comment.issue) {
        return missing("issue", format!("Issue \"{}\" does not exist.", comment.issue));
    }
    if comment.author.trim().is_empty() {
        return invariant("author", "A comment author must not be blank.");
    }
    if comment.text.trim().is_empty() {
        return invariant("text", "A comment must not be blank.");
    }
    if !valid_timestamp(&comment.date) {
        return invariant("date", "A comment moment must be a date as year-month-day, optionally with a time (for example 2026-10-09T08:30:00Z).");
    }
    None
}
//#endregion 🔖️Problems

//#region 🔖️Camera
impl super::ViewCamera {
    /// 🎥️ The eye and the target of the orbit camera in building-datum metres: the camera looks from `distance` metres away along the azimuth, above the target by the pitch.
    pub fn eye_and_target(&self) -> ([f64; 3], [f64; 3]) {
        let target = [self.target.x, self.target.y, self.target_height];
        let look = [self.pitch.cos() * self.azimuth.cos(), self.pitch.cos() * self.azimuth.sin(), -self.pitch.sin()];
        (std::array::from_fn(|axis| target[axis] - self.distance * look[axis]), target)
    }

    /// 🎥️ The orbit camera that looks from `eye` at `target` (a zero distance becomes a metre, straight down or up keeps the azimuth zero).
    pub fn from_eye_and_target(eye: [f64; 3], target: [f64; 3]) -> Self {
        let look = [target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]];
        let distance = look.iter().map(|part| part * part).sum::<f64>().sqrt();
        if distance < 1e-9 {
            return Self { target: super::Point2 { x: target[0], y: target[1] }, target_height: target[2], azimuth: 0.0, pitch: 0.0, distance: 1.0 };
        }
        let flat = look[0].hypot(look[1]);
        let azimuth = if flat < 1e-12 { 0.0 } else { look[1].atan2(look[0]) };
        Self { target: super::Point2 { x: target[0], y: target[1] }, target_height: target[2], azimuth, pitch: (-look[2] / distance).clamp(-1.0, 1.0).asin(), distance }
    }
}
//#endregion 🔖️Camera

/// 🧾️ The comments of `issue` in writing order: by moment, then id.
pub fn comments_of<'a>(base: &'a ModelSnapshot, issue: &str) -> Vec<(&'a String, &'a IssueComment)> {
    let mut rows: Vec<(&String, &IssueComment)> = base.issue_comments.iter().filter(|(_, row)| row.issue == issue).collect();
    rows.sort_by(|a, b| (&a.1.date, a.0).cmp(&(&b.1.date, b.0)));
    rows
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
