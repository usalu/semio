//! 📋️ The vocabulary and the checks of authored schedules: the token form of every key (the text the editor and the CSV header speak), which built-in fields a category offers, the one validity rule
//! of a schedule definition (`create-schedule` and `set-schedule` refuse exactly what it names) and the presets the library creates. Everything here is a pure function of authored values.

use crate::{Schedule, ScheduleCategory, ScheduleColumn, ScheduleField, ScheduleFilter, ScheduleGroup, ScheduleKey, ScheduleOp, ScheduleSort};

//#region 🔖️Fields
impl ScheduleField {
    /// 🔢️ Every built-in field, in vocabulary order.
    pub const ALL: [ScheduleField; 35] = [
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
            Self::Swing | Self::Leaves => category == Door,
            Self::Panes => category == Window,
            Self::Risers => category == Stair,
            Self::Thickness | Self::LayerArea | Self::LayerVolume | Self::LayerMass => category == Material,
            _ => true,
        }
    }
}
//#endregion 🔖️Fields

//#region 🔖️Categories
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
//#endregion 🔖️Categories

//#region 🔖️Operators
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
//#endregion 🔖️Operators

//#region 🔖️Keys
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
//#endregion 🔖️Keys

//#region 🔖️Validity
fn repeated<T: PartialEq>(items: &[T]) -> bool {
    items.iter().enumerate().any(|(index, item)| items[..index].contains(item))
}

/// 🚫️ The first reason a schedule definition cannot stand, as the path of the offending field and the message; storey references are checked against the snapshot by the mutation that writes them.
pub fn schedule_problem(schedule: &Schedule) -> Option<(&'static str, String)> {
    let category = schedule.category;
    if schedule.name.trim().is_empty() {
        return Some(("name", "A schedule needs a name.".into()));
    }
    if schedule.columns.is_empty() {
        return Some(("columns", "A schedule needs at least one column.".into()));
    }
    let keys: Vec<&ScheduleKey> = schedule.columns.iter().map(|column| &column.key).collect();
    if repeated(&keys) {
        return Some(("columns", "A column appears once in a schedule.".into()));
    }
    if schedule.columns.iter().any(|column| column.heading.as_ref().is_some_and(|heading| heading.trim().is_empty())) {
        return Some(("columns", "A column heading must not be blank.".into()));
    }
    for (path, keys) in [("columns", keys.clone()), ("sort", schedule.sort.iter().map(|sort| &sort.key).collect()), ("filter", schedule.filter.iter().map(|filter| &filter.key).collect()), ("group", schedule.group.iter().map(|group| &group.key).collect())] {
        if let Some(problem) = keys.iter().find_map(|key| key.problem(category)) {
            return Some((path, problem.to_string()));
        }
    }
    if repeated(&schedule.sort.iter().map(|sort| &sort.key).collect::<Vec<_>>()) {
        return Some(("sort", "A sort key appears once.".into()));
    }
    if repeated(&schedule.group.iter().map(|group| &group.key).collect::<Vec<_>>()) {
        return Some(("group", "A grouping key appears once.".into()));
    }
    if schedule.filter.iter().any(|filter| filter.op.nullary() && !filter.value.is_empty()) {
        return Some(("filter", "An empty or non-empty filter compares with no value.".into()));
    }
    if schedule.filter.iter().any(|filter| !filter.op.nullary() && filter.value.trim().is_empty()) {
        return Some(("filter", "A filter needs a value to compare with.".into()));
    }
    if repeated(&schedule.storeys) {
        return Some(("storeys", "A storey appears once in the scope.".into()));
    }
    if repeated(&schedule.phases) {
        return Some(("phases", "A phase appears once in the scope.".into()));
    }
    None
}
//#endregion 🔖️Validity

//#region 🔖️Presets
fn column(field: ScheduleField, total: bool) -> ScheduleColumn {
    ScheduleColumn { key: ScheduleKey::field(field), heading: None, total }
}

fn ascending(field: ScheduleField) -> ScheduleSort {
    ScheduleSort { key: ScheduleKey::field(field), descending: false }
}

fn group(field: ScheduleField) -> ScheduleGroup {
    ScheduleGroup { key: ScheduleKey::field(field) }
}

/// 📋️ The schedules the library creates, by preset key.
pub const PRESETS: [&str; 6] = ["door", "window", "room", "finish", "wall", "material"];

/// 📋️ The preset schedule `key` under `name` (`None` for an unknown key): `door` lists every door with its type, size and swing, `window` every window, `room` the spaces grouped by storey with area and volume totals, `finish` the floor, wall and ceiling finish of every room grouped by storey with the finish areas summed,
/// `wall` the walls grouped by type with length, area and volume totals, `material` the material take-off collapsed per material.
pub fn preset(key: &str, name: &str) -> Option<Schedule> {
    use ScheduleField::*;
    let schedule = |category, columns: Vec<ScheduleColumn>, sort: Vec<ScheduleSort>, filter: Vec<ScheduleFilter>, group: Vec<ScheduleGroup>, itemize| Schedule { name: name.to_string(), category, columns, sort, filter, group, itemize, storeys: Vec::new(), phases: Vec::new() };
    Some(match key {
        "door" => schedule(ScheduleCategory::Door, vec![column(Name, false), column(Type, false), column(Storey, false), column(Width, false), column(Height, false), column(Leaves, false), column(Swing, false), column(Count, true)], vec![ascending(Storey), ascending(Name)], Vec::new(), Vec::new(), true),
        "window" => schedule(ScheduleCategory::Window, vec![column(Name, false), column(Type, false), column(Storey, false), column(Width, false), column(Height, false), column(Panes, false), column(Material, false), column(Count, true)], vec![ascending(Storey), ascending(Name)], Vec::new(), Vec::new(), true),
        "room" => schedule(ScheduleCategory::Space, vec![column(Number, false), column(Name, false), column(Storey, false), column(Usage, false), column(GrossArea, true), column(NetArea, true), column(Height, false), column(NetVolume, true)], vec![ascending(Number)], Vec::new(), vec![group(Storey)], true),
        "finish" => schedule(ScheduleCategory::Finish, vec![column(Number, false), column(Name, false), column(Surface, false), column(Material, false), column(FinishArea, true)], vec![ascending(Number), ascending(Surface)], Vec::new(), vec![group(Storey)], true),
        "wall" => schedule(ScheduleCategory::Wall, vec![column(Name, false), column(Type, false), column(Storey, false), column(Length, true), column(Height, false), column(NetSideArea, true), column(NetVolume, true)], vec![ascending(Name)], Vec::new(), vec![group(Type)], true),
        "material" => schedule(ScheduleCategory::Material, vec![column(Material, false), column(LayerArea, true), column(LayerVolume, true), column(LayerMass, true), column(Count, true)], Vec::new(), Vec::new(), vec![group(Material)], false),
        _ => return None,
    })
}
//#endregion 🔖️Presets

//#region 🔖️Text
/// 🔤️ The text of a key list: the key tokens separated by `, `.
pub fn keys_text<'a>(keys: impl IntoIterator<Item = &'a ScheduleKey>) -> String {
    keys.into_iter().map(ScheduleKey::token).collect::<Vec<_>>().join(", ")
}

fn parts(text: &str, separator: char) -> Vec<&str> {
    text.split(separator).map(str::trim).filter(|part| !part.is_empty()).collect()
}

/// 🔎️ The keys a key list text names; `None` when one token is not a key.
pub fn parse_keys(text: &str) -> Option<Vec<ScheduleKey>> {
    parts(text, ',').into_iter().map(ScheduleKey::parse).collect()
}

/// 🔤️ The text of the columns: the key token, `*` after a summed column.
pub fn columns_text(columns: &[ScheduleColumn]) -> String {
    columns.iter().map(|column| format!("{}{}", column.key.token(), if column.total { "*" } else { "" })).collect::<Vec<_>>().join(", ")
}

/// 🔎️ The columns a text names (`name, type, net_area*`); a column that stays keeps its heading from `previous`.
pub fn parse_columns(text: &str, previous: &[ScheduleColumn]) -> Option<Vec<ScheduleColumn>> {
    parts(text, ',')
        .into_iter()
        .map(|part| {
            let (token, total) = part.strip_suffix('*').map_or((part, false), |token| (token.trim(), true));
            let key = ScheduleKey::parse(token)?;
            let heading = previous.iter().find(|column| column.key == key).and_then(|column| column.heading.clone());
            Some(ScheduleColumn { key, heading, total })
        })
        .collect()
}

/// 🔤️ The text of the sort keys: the key token, ` desc` after a descending one.
pub fn sorts_text(sorts: &[ScheduleSort]) -> String {
    sorts.iter().map(|sort| format!("{}{}", sort.key.token(), if sort.descending { " desc" } else { "" })).collect::<Vec<_>>().join(", ")
}

/// 🔎️ The sort keys a text names (`length desc, name`).
pub fn parse_sorts(text: &str) -> Option<Vec<ScheduleSort>> {
    parts(text, ',')
        .into_iter()
        .map(|part| {
            let (token, descending) = part.strip_suffix(" desc").map_or((part, false), |token| (token.trim(), true));
            Some(ScheduleSort { key: ScheduleKey::parse(token)?, descending })
        })
        .collect()
}

/// 🔤️ The text of the filters: `key op value`, separated by `; `.
pub fn filters_text(filters: &[ScheduleFilter]) -> String {
    filters.iter().map(|filter| if filter.op.nullary() { format!("{} {}", filter.key.token(), filter.op.symbol()) } else { format!("{} {} {}", filter.key.token(), filter.op.symbol(), filter.value) }).collect::<Vec<_>>().join("; ")
}

/// 🔎️ The filters a text names (`length >= 6; name ~ st; usage empty`).
pub fn parse_filters(text: &str) -> Option<Vec<ScheduleFilter>> {
    parts(text, ';')
        .into_iter()
        .map(|part| {
            let mut words = part.splitn(3, ' ');
            let key = ScheduleKey::parse(words.next()?)?;
            let op = ScheduleOp::from_symbol(words.next()?)?;
            let value = words.next().unwrap_or("").trim().to_string();
            Some(ScheduleFilter { key, op, value })
        })
        .collect()
}

/// 🔎️ The grouping levels a key list text names.
pub fn parse_groups(text: &str) -> Option<Vec<ScheduleGroup>> {
    parse_keys(text).map(|keys| keys.into_iter().map(|key| ScheduleGroup { key }).collect())
}
//#endregion 🔖️Text
