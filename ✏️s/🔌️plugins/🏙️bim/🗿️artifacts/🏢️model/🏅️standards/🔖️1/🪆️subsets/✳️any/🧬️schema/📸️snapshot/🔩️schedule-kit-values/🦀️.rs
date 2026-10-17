//! 🧮️ The methods of the value types of the `schedule_kit` kit; they live with the value types so the entity types and the model root stay above them.

use crate::standards::v1::subsets::any::schema::snapshot::values::{ScheduleCategory, ScheduleField, ScheduleKey, ScheduleOp};

impl ScheduleField {
    /// 🔢️ Every built-in field, in vocabulary order.
    pub const ALL: [ScheduleField; 37] = [
        Self::Id,
        Self::Name,
        Self::Kind,
        Self::Storey,
        Self::Level,
        Self::Type,
        Self::Phase,
        Self::Material,
        Self::Host,
        Self::Number,
        Self::Usage,
        Self::Surface,
        Self::Swing,
        Self::Leaves,
        Self::Panes,
        Self::Count,
        Self::Length,
        Self::Width,
        Self::Height,
        Self::Perimeter,
        Self::GrossSideArea,
        Self::OpeningArea,
        Self::NetSideArea,
        Self::GrossArea,
        Self::NetArea,
        Self::SurfaceArea,
        Self::GrossVolume,
        Self::NetVolume,
        Self::Mass,
        Self::Risers,
        Self::Thickness,
        Self::LayerArea,
        Self::LayerVolume,
        Self::LayerMass,
        Self::FinishArea,
        Self::UValue,
        Self::GValue,
    ];

    /// 🏷️ The stable token of the field (snake case).
    pub const fn token(self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
            Self::Kind => "kind",
            Self::Storey => "storey",
            Self::Level => "level",
            Self::Type => "type",
            Self::Phase => "phase",
            Self::Material => "material",
            Self::Host => "host",
            Self::Number => "number",
            Self::Usage => "usage",
            Self::Surface => "surface",
            Self::Swing => "swing",
            Self::Leaves => "leaves",
            Self::Panes => "panes",
            Self::Count => "count",
            Self::Length => "length",
            Self::Width => "width",
            Self::Height => "height",
            Self::Perimeter => "perimeter",
            Self::GrossSideArea => "gross_side_area",
            Self::OpeningArea => "opening_area",
            Self::NetSideArea => "net_side_area",
            Self::GrossArea => "gross_area",
            Self::NetArea => "net_area",
            Self::SurfaceArea => "surface_area",
            Self::GrossVolume => "gross_volume",
            Self::NetVolume => "net_volume",
            Self::Mass => "mass",
            Self::Risers => "risers",
            Self::Thickness => "thickness",
            Self::LayerArea => "layer_area",
            Self::LayerVolume => "layer_volume",
            Self::LayerMass => "layer_mass",
            Self::FinishArea => "finish_area",
            Self::UValue => "u_value",
            Self::GValue => "g_value",
        }
    }

    /// 🔎️ The field a token names.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|field| field.token() == token.trim())
    }

    /// 🔢️ Whether the cells of the field are numbers (summable, compared as numbers).
    pub const fn numeric(self) -> bool {
        !matches!(self, Self::Id | Self::Name | Self::Kind | Self::Storey | Self::Type | Self::Phase | Self::Material | Self::Host | Self::Number | Self::Usage | Self::Surface | Self::Swing | Self::Leaves)
    }

    /// 📏️ The unit of a numeric field (`m`, `m²`, `m³`, `kg`), empty for counts and texts.
    pub const fn unit(self) -> &'static str {
        match self {
            Self::Length | Self::Width | Self::Height | Self::Perimeter | Self::Thickness => "m",
            Self::GrossSideArea | Self::OpeningArea | Self::NetSideArea | Self::GrossArea | Self::NetArea | Self::SurfaceArea | Self::LayerArea | Self::FinishArea => "m²",
            Self::GrossVolume | Self::NetVolume | Self::LayerVolume => "m³",
            Self::Mass | Self::LayerMass => "kg",
            Self::UValue => "W/(m²·K)",
            _ => "",
        }
    }

    /// ✔️ Whether a schedule of `category` offers the field: every category offers the common fields and the take-off measures, the others belong to the category that has them.
    pub fn available(self, category: ScheduleCategory) -> bool {
        use ScheduleCategory::*;
        match self {
            Self::Host => matches!(category, Window | Door | Void),
            Self::Number | Self::Usage => matches!(category, Space | Finish),
            Self::Surface | Self::FinishArea => category == Finish,
            Self::UValue => matches!(category, Window | Door),
            Self::GValue => category == Window,
            Self::Swing | Self::Leaves => category == Door,
            Self::Panes => category == Window,
            Self::Risers => category == Stair,
            Self::Thickness | Self::LayerArea | Self::LayerVolume | Self::LayerMass => category == Material,
            _ => true,
        }
    }
}

impl ScheduleCategory {
    /// 🔢️ Every category, in vocabulary order.
    pub const ALL: [ScheduleCategory; 14] = [Self::Wall, Self::CurtainWall, Self::Slab, Self::Roof, Self::Column, Self::Beam, Self::Window, Self::Door, Self::Void, Self::Stair, Self::Railing, Self::Space, Self::Finish, Self::Material];

    /// 🏷️ The stable token of the category: the key of the quantity kind it lists (kebab case), `finish` for the room finishes, `material` for the material take-off.
    pub const fn token(self) -> &'static str {
        match self {
            Self::Wall => "wall",
            Self::CurtainWall => "curtain-wall",
            Self::Slab => "slab",
            Self::Roof => "roof",
            Self::Column => "column",
            Self::Beam => "beam",
            Self::Window => "window",
            Self::Door => "door",
            Self::Void => "void",
            Self::Stair => "stair",
            Self::Railing => "railing",
            Self::Space => "space",
            Self::Finish => "finish",
            Self::Material => "material",
        }
    }

    /// 🔎️ The category a token names.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|category| category.token() == token.trim())
    }

    /// 🎨️ Whether the rows of the category are the finished surfaces (floor, walls, ceiling) of rooms.
    pub const fn is_finish(self) -> bool {
        matches!(self, Self::Finish)
    }

    /// 🧱️ Whether the rows of the category are layers or material runs of elements of any kind.
    pub const fn is_material(self) -> bool {
        matches!(self, Self::Material)
    }
}

impl ScheduleOp {
    /// 🔢️ Every operator, in vocabulary order.
    pub const ALL: [ScheduleOp; 9] = [Self::Equals, Self::NotEquals, Self::Contains, Self::Greater, Self::GreaterOrEqual, Self::Less, Self::LessOrEqual, Self::Empty, Self::NotEmpty];

    /// 🏷️ The symbol of the operator in filter text.
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Equals => "=",
            Self::NotEquals => "!=",
            Self::Contains => "~",
            Self::Greater => ">",
            Self::GreaterOrEqual => ">=",
            Self::Less => "<",
            Self::LessOrEqual => "<=",
            Self::Empty => "empty",
            Self::NotEmpty => "!empty",
        }
    }

    /// 🔎️ The operator a symbol names.
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.symbol() == symbol.trim())
    }

    /// 🕳️ Whether the operator compares with no value.
    pub const fn nullary(self) -> bool {
        matches!(self, Self::Empty | Self::NotEmpty)
    }
}

impl ScheduleKey {
    /// 🏗️ The key of a built-in field.
    pub const fn field(field: ScheduleField) -> Self {
        Self::Field { field }
    }

    /// 🏗️ The key of property `name` of the property set `set`.
    pub fn property(set: impl Into<String>, name: impl Into<String>) -> Self {
        Self::Property { set: set.into(), name: name.into() }
    }

    /// 🏷️ The token of the key: the field token, or `set.name` for a property.
    pub fn token(&self) -> String {
        match self {
            Self::Field { field } => field.token().to_string(),
            Self::Property { set, name } => format!("{set}.{name}"),
        }
    }

    /// 🔎️ The key a token names: a field token, or `set.name` split at the first dot; `None` for an unknown field token.
    pub fn parse(token: &str) -> Option<Self> {
        let token = token.trim();
        match token.split_once('.') {
            Some((set, name)) => Some(Self::property(set.trim(), name.trim())),
            None => ScheduleField::from_token(token).map(Self::field),
        }
    }

    /// 🔢️ Whether the cells of the key are numbers; a property is decided by its values.
    pub fn numeric(&self) -> bool {
        matches!(self, Self::Field { field } if field.numeric())
    }

    /// 🚫️ Why the key cannot be used in a schedule of `category`, `None` when it can.
    pub fn problem(&self, category: ScheduleCategory) -> Option<&'static str> {
        match self {
            Self::Field { field } => (!field.available(category)).then_some("A schedule of this category does not offer this field."),
            Self::Property { set, name } => {
                if set.trim().is_empty() || name.trim().is_empty() {
                    Some("A property key needs a property set and a property name.")
                } else if set.contains('.') || set.contains(',') || name.contains(',') {
                    Some("A property set must not contain a dot, and neither part of a property key may contain a comma.")
                } else {
                    None
                }
            }
        }
    }
}
