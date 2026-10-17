//! 🧮️ The methods of the value types of the `property_kit` kit; they live with the value types so the entity types and the model root stay above them.

use super::values::{PropertyDef, PropertyKind, PropertyValue, TemplateTarget};

const TOLERANCE: f64 = 1e-9;

impl PropertyKind {
    /// 📖️ Every kind, in declaration order.
    pub const ALL: [PropertyKind; 8] = [Self::Text, Self::Real, Self::Integer, Self::Boolean, Self::Length, Self::Area, Self::Volume, Self::Angle];

    /// 🔢️ Whether values of the kind are numbers (everything but text and boolean): the kinds a range can bound.
    pub fn is_numeric(self) -> bool {
        !matches!(self, Self::Text | Self::Boolean)
    }

    /// 🏷️ The stable lower-case name of the kind.
    pub fn name(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Real => "real",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::Length => "length",
            Self::Area => "area",
            Self::Volume => "volume",
            Self::Angle => "angle",
        }
    }
}

impl PropertyValue {
    /// 🏷️ The kind of the value.
    pub fn kind(&self) -> PropertyKind {
        match self {
            Self::Text { .. } => PropertyKind::Text,
            Self::Real { .. } => PropertyKind::Real,
            Self::Integer { .. } => PropertyKind::Integer,
            Self::Boolean { .. } => PropertyKind::Boolean,
            Self::Length { .. } => PropertyKind::Length,
            Self::Area { .. } => PropertyKind::Area,
            Self::Volume { .. } => PropertyKind::Volume,
            Self::Angle { .. } => PropertyKind::Angle,
        }
    }

    /// 🔢️ The number of a numeric value, `None` for text and boolean.
    pub fn number(&self) -> Option<f64> {
        match self {
            Self::Real { value } | Self::Length { value } | Self::Area { value } | Self::Volume { value } | Self::Angle { value } => Some(*value),
            Self::Integer { value } => Some(f64::from(*value)),
            Self::Text { .. } | Self::Boolean { .. } => None,
        }
    }

    /// 🔢️ Whether the measure is finite; text, boolean and integer always are.
    pub fn is_finite(&self) -> bool {
        self.number().is_none_or(f64::is_finite)
    }

    /// 🔁️ Whether two values are the same: the same kind and the same text, flag, integer or number within a relative tolerance of 1e-9.
    pub fn same_as(&self, other: &PropertyValue) -> bool {
        if self.kind() != other.kind() {
            return false;
        }
        match (self.number(), other.number()) {
            (Some(a), Some(b)) => (a - b).abs() <= TOLERANCE * a.abs().max(b.abs()).max(1.0),
            _ => self == other,
        }
    }

    /// 📖️ The plain text of the value: the text itself, `true` or `false`, a number without trailing zeros.
    pub fn display(&self) -> String {
        match self {
            Self::Text { value } => value.clone(),
            Self::Boolean { value } => value.to_string(),
            Self::Integer { value } => value.to_string(),
            other => other.number().map_or_else(String::new, |number| format!("{}", (number * 1e9).round() / 1e9)),
        }
    }
}

/// 🚫️ How a value breaks the definition of its property.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum Violation {
    KindMismatch,
    BelowMinimum,
    AboveMaximum,
    NotAllowed,
}

impl PropertyDef {
    /// 🚫️ How `value` breaks the definition, `None` when it conforms: another kind, below the minimum, above the maximum, or not in the enumeration of allowed values.
    pub fn violation(&self, value: &PropertyValue) -> Option<Violation> {
        if value.kind() != self.kind {
            return Some(Violation::KindMismatch);
        }
        if let (Some(number), Some(minimum)) = (value.number(), self.minimum) {
            if number < minimum - TOLERANCE * minimum.abs().max(1.0) {
                return Some(Violation::BelowMinimum);
            }
        }
        if let (Some(number), Some(maximum)) = (value.number(), self.maximum) {
            if number > maximum + TOLERANCE * maximum.abs().max(1.0) {
                return Some(Violation::AboveMaximum);
            }
        }
        if !self.allowed.is_empty() && !self.allowed.iter().any(|allowed| allowed.same_as(value)) {
            return Some(Violation::NotAllowed);
        }
        None
    }

    /// 🧐️ Why the definition itself is unusable, `None` when it is sound: the field it is about and the message. A name is needed; the default and every allowed value are of the kind, finite and distinct; the default conforms; a range bounds a numeric kind only, with finite ends and a minimum
    /// that does not exceed the maximum.
    pub fn problem(&self) -> Option<(&'static str, String)> {
        if self.name.trim().is_empty() {
            return Some(("name", "A property needs a name.".to_string()));
        }
        for bound in [self.minimum, self.maximum].into_iter().flatten() {
            if !bound.is_finite() {
                return Some(("minimum", format!("The range of property \"{}\" must be finite.", self.name)));
            }
        }
        if (self.minimum.is_some() || self.maximum.is_some()) && !self.kind.is_numeric() {
            return Some(("minimum", format!("Property \"{}\" is {} and cannot have a numeric range.", self.name, self.kind.name())));
        }
        if let (Some(minimum), Some(maximum)) = (self.minimum, self.maximum) {
            if minimum > maximum {
                return Some(("minimum", format!("The minimum of property \"{}\" exceeds its maximum.", self.name)));
            }
        }
        for (index, allowed) in self.allowed.iter().enumerate() {
            if allowed.kind() != self.kind || !allowed.is_finite() {
                return Some(("allowed", format!("Every allowed value of property \"{}\" must be a finite {}.", self.name, self.kind.name())));
            }
            if self.allowed[..index].iter().any(|earlier| earlier.same_as(allowed)) {
                return Some(("allowed", format!("The allowed values of property \"{}\" repeat {}.", self.name, allowed.display())));
            }
        }
        if let Some(default) = &self.default_value {
            if !default.is_finite() {
                return Some(("default_value", format!("The default of property \"{}\" must be finite.", self.name)));
            }
            if default.kind() != self.kind {
                return Some(("default_value", format!("The default of property \"{}\" must be a {}.", self.name, self.kind.name())));
            }
            if self.violation(default).is_some() {
                return Some(("default_value", format!("The default of property \"{}\" breaks its own range or enumeration.", self.name)));
            }
        }
        None
    }
}

impl TemplateTarget {
    /// 📖️ Every target, in declaration order.
    pub const ALL: [TemplateTarget; 26] = [
        Self::Site,
        Self::Building,
        Self::Storey,
        Self::Wall,
        Self::CurtainWall,
        Self::Column,
        Self::Beam,
        Self::Slab,
        Self::Ceiling,
        Self::Roof,
        Self::Window,
        Self::Door,
        Self::Void,
        Self::Stair,
        Self::Ramp,
        Self::Railing,
        Self::Space,
        Self::Zone,
        Self::WallType,
        Self::SlabType,
        Self::CeilingType,
        Self::RoofType,
        Self::ColumnType,
        Self::BeamType,
        Self::WindowType,
        Self::DoorType,
    ];

    /// 🏷️ The stable kebab-case name of the target.
    pub fn name(self) -> &'static str {
        match self {
            Self::Site => "site",
            Self::Building => "building",
            Self::Storey => "storey",
            Self::Wall => "wall",
            Self::CurtainWall => "curtain-wall",
            Self::Column => "column",
            Self::Beam => "beam",
            Self::Slab => "slab",
            Self::Ceiling => "ceiling",
            Self::Roof => "roof",
            Self::Window => "window",
            Self::Door => "door",
            Self::Void => "void",
            Self::Stair => "stair",
            Self::Ramp => "ramp",
            Self::Railing => "railing",
            Self::Space => "space",
            Self::Zone => "zone",
            Self::WallType => "wall-type",
            Self::SlabType => "slab-type",
            Self::CeilingType => "ceiling-type",
            Self::RoofType => "roof-type",
            Self::ColumnType => "column-type",
            Self::BeamType => "beam-type",
            Self::WindowType => "window-type",
            Self::DoorType => "door-type",
        }
    }

    /// 🏛️ Whether the target is a type kind: its properties are inherited by the instances of the type.
    pub fn is_type(self) -> bool {
        matches!(self, Self::WallType | Self::SlabType | Self::CeilingType | Self::RoofType | Self::ColumnType | Self::BeamType | Self::WindowType | Self::DoorType)
    }

    /// 🔗️ The element kind a type kind types, `None` for every other target.
    pub fn served(self) -> Option<TemplateTarget> {
        match self {
            Self::WallType => Some(Self::Wall),
            Self::SlabType => Some(Self::Slab),
            Self::CeilingType => Some(Self::Ceiling),
            Self::RoofType => Some(Self::Roof),
            Self::ColumnType => Some(Self::Column),
            Self::BeamType => Some(Self::Beam),
            Self::WindowType => Some(Self::Window),
            Self::DoorType => Some(Self::Door),
            _ => None,
        }
    }
}
