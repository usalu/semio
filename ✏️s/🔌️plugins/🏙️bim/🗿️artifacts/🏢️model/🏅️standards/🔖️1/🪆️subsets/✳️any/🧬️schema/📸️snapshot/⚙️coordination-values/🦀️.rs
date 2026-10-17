//! 🧮️ The methods of the value types of the `coordination` kit; they live with the value types so the entity types and the model root stay above them.

use super::values::{ElementClass, ElementSelector, RuleKind, RuleScope};

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

impl super::values::ViewCamera {
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
            return Self { target: super::values::Point2 { x: target[0], y: target[1] }, target_height: target[2], azimuth: 0.0, pitch: 0.0, distance: 1.0 };
        }
        let flat = look[0].hypot(look[1]);
        let azimuth = if flat < 1e-12 { 0.0 } else { look[1].atan2(look[0]) };
        Self { target: super::values::Point2 { x: target[0], y: target[1] }, target_height: target[2], azimuth, pitch: (-look[2] / distance).clamp(-1.0, 1.0).asin(), distance }
    }
}
