//! 📋️ The vocabulary and the checks of authored schedules: the token form of every key (the text the editor and the CSV header speak), which built-in fields a category offers, the one validity rule
//! of a schedule definition (`create-schedule` and `set-schedule` refuse exactly what it names) and the presets the library creates. Everything here is a pure function of authored values.

use crate::standards::v1::subsets::any::schema::snapshot::entities::Schedule;
use crate::standards::v1::subsets::any::schema::snapshot::values::{ScheduleCategory, ScheduleColumn, ScheduleField, ScheduleFilter, ScheduleGroup, ScheduleKey, ScheduleOp, ScheduleSort};

//#region 🔖️Fields

//#endregion 🔖️Fields

//#region 🔖️Categories

//#endregion 🔖️Categories

//#region 🔖️Operators

//#endregion 🔖️Operators

//#region 🔖️Keys

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
pub const PRESETS: [&str; 7] = ["door", "window", "room", "finish", "envelope", "wall", "material"];

/// 📋️ The preset schedule `key` under `name` (`None` for an unknown key): `door` lists every door with its type, size and swing, `window` every window, `room` the spaces grouped by storey with area and volume totals, `envelope` every window with its type, size, area, U-value and g-value, `finish` the floor, wall and ceiling finish of every room grouped by storey with the finish areas summed,
/// `wall` the walls grouped by type with length, area and volume totals, `material` the material take-off collapsed per material.
pub fn preset(key: &str, name: &str) -> Option<Schedule> {
    use ScheduleField::*;
    let schedule = |category, columns: Vec<ScheduleColumn>, sort: Vec<ScheduleSort>, filter: Vec<ScheduleFilter>, group: Vec<ScheduleGroup>, itemize| Schedule { name: name.to_string(), category, columns, sort, filter, group, itemize, storeys: Vec::new(), phases: Vec::new() };
    Some(match key {
        "door" => schedule(ScheduleCategory::Door, vec![column(Name, false), column(Type, false), column(Storey, false), column(Width, false), column(Height, false), column(Leaves, false), column(Swing, false), column(Count, true)], vec![ascending(Storey), ascending(Name)], Vec::new(), Vec::new(), true),
        "window" => schedule(ScheduleCategory::Window, vec![column(Name, false), column(Type, false), column(Storey, false), column(Width, false), column(Height, false), column(Panes, false), column(Material, false), column(Count, true)], vec![ascending(Storey), ascending(Name)], Vec::new(), Vec::new(), true),
        "room" => schedule(ScheduleCategory::Space, vec![column(Number, false), column(Name, false), column(Storey, false), column(Usage, false), column(GrossArea, true), column(NetArea, true), column(Height, false), column(NetVolume, true)], vec![ascending(Number)], Vec::new(), vec![group(Storey)], true),
        "finish" => schedule(ScheduleCategory::Finish, vec![column(Number, false), column(Name, false), column(Surface, false), column(Material, false), column(FinishArea, true)], vec![ascending(Number), ascending(Surface)], Vec::new(), vec![group(Storey)], true),
        "envelope" => schedule(ScheduleCategory::Window, vec![column(Name, false), column(Type, false), column(Storey, false), column(Width, false), column(Height, false), column(GrossArea, true), column(UValue, false), column(GValue, false), column(Count, true)], vec![ascending(Storey), ascending(Name)], Vec::new(), Vec::new(), true),
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
