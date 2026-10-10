//! 🏷️ The vocabulary of property templates and classification systems: what a property value is made of, which violations a definition can find in a value, the checks every authored template and classification table must pass, and the
//! navigation of an entry table. Pure and base-free: whether the elements a template applies to exist, and which values they carry, is decided by the leaves and by the inference, never here.

use super::{ClassificationItem, ClassificationSystem, PropertyDef, PropertyKind, PropertyValue, TemplateTarget};
use std::collections::BTreeSet;

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

/// 🧐️ Why a property set template cannot be stored, `None` when it can: the field of the template it is about and the message. A name; every kind at most once in `applies_to`; sound property definitions with unique names.
pub fn template_problem(name: &str, applies_to: &[TemplateTarget], properties: &[PropertyDef]) -> Option<(&'static str, String)> {
    if name.trim().is_empty() {
        return Some(("name", "A property set template needs a name.".to_string()));
    }
    let mut kinds = BTreeSet::new();
    if let Some(repeated) = applies_to.iter().find(|target| !kinds.insert(**target as u32)) {
        return Some(("applies_to", format!("The template lists the kind {repeated:?} twice.")));
    }
    let mut names = BTreeSet::new();
    for definition in properties {
        if let Some(problem) = definition.problem() {
            return Some(("properties", problem.1));
        }
        if !names.insert(definition.name.as_str()) {
            return Some(("properties", format!("The template defines the property \"{}\" twice.", definition.name)));
        }
    }
    None
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

/// 🧐️ Why an entry table cannot be stored, `None` when it can: a code is needed and unique, a parent is another row of the table, and the parents form a forest (no row is its own ancestor).
pub fn entries_problem(entries: &[ClassificationItem]) -> Option<String> {
    let mut codes = BTreeSet::new();
    for entry in entries {
        if entry.code.trim().is_empty() {
            return Some("A classification entry needs a code.".to_string());
        }
        if !codes.insert(entry.code.as_str()) {
            return Some(format!("The classification code \"{}\" is used by more than one entry.", entry.code));
        }
    }
    for entry in entries {
        let Some(parent) = &entry.parent else { continue };
        if !codes.contains(parent.as_str()) {
            return Some(format!("The parent \"{parent}\" of entry \"{}\" is not an entry of the table.", entry.code));
        }
    }
    for entry in entries {
        let mut at = entry;
        for _ in 0..=entries.len() {
            match at.parent.as_deref().and_then(|parent| entries.iter().find(|row| row.code == parent)) {
                Some(parent) if parent.code == entry.code => return Some(format!("Entry \"{}\" is its own ancestor.", entry.code)),
                Some(parent) => at = parent,
                None => break,
            }
        }
        if at.parent.is_some() {
            return Some(format!("Entry \"{}\" is its own ancestor.", entry.code));
        }
    }
    None
}

impl ClassificationSystem {
    /// 🔎️ The row of a code.
    pub fn entry(&self, code: &str) -> Option<&ClassificationItem> {
        self.entries.iter().find(|entry| entry.code == code)
    }

    /// 🌳️ The rows whose parent is `parent` (the roots for `None`), in table order.
    pub fn children(&self, parent: Option<&str>) -> Vec<&ClassificationItem> {
        self.entries.iter().filter(|entry| entry.parent.as_deref() == parent).collect()
    }

    /// 🧭️ The rows from the root down to the row of `code`, empty when the code is no row.
    pub fn lineage(&self, code: &str) -> Vec<&ClassificationItem> {
        let mut path = Vec::new();
        let mut at = self.entry(code);
        while let Some(entry) = at {
            if path.len() > self.entries.len() {
                break;
            }
            path.push(entry);
            at = entry.parent.as_deref().and_then(|parent| self.entry(parent));
        }
        path.reverse();
        path
    }

    /// 🔢️ How deep the row of `code` stands: zero for a root, none for an unknown code.
    pub fn depth(&self, code: &str) -> Option<usize> {
        self.entry(code).map(|_| self.lineage(code).len().saturating_sub(1))
    }

    /// 📖️ The rows in tree order: every root with its descendants below it, depth first, each branch in table order.
    pub fn tree(&self) -> Vec<(usize, &ClassificationItem)> {
        fn walk<'a>(system: &'a ClassificationSystem, parent: Option<&str>, depth: usize, out: &mut Vec<(usize, &'a ClassificationItem)>) {
            if depth > system.entries.len() {
                return;
            }
            for entry in system.children(parent) {
                out.push((depth, entry));
                walk(system, Some(entry.code.as_str()), depth + 1, out);
            }
        }
        let mut out = Vec::new();
        walk(self, None, 0, &mut out);
        out
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
