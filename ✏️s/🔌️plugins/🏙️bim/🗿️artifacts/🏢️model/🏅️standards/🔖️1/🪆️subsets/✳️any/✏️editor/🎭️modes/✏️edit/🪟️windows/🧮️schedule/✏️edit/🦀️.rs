//! ✏️ The edits of a schedule definition the window offers: a column picker (add, remove, move, sum), sort keys (add, remove, move, reverse), filters (add, comparison, value, remove), grouping levels
//! (add, remove, move), itemization, and the storey and phase scope. Each edit is a pure function of the definition and yields the `set-schedule` payload that carries exactly the changed list or value;
//! whether the result stands is decided by the mutation, not here.

use crate::mutations::set_schedule::SetSchedule;
use crate::schedule_kit::{preset, PRESETS};
use crate::{ModelSnapshot, Phase, Schedule, ScheduleCategory, ScheduleColumn, ScheduleFilter, ScheduleGroup, ScheduleKey, ScheduleOp, ScheduleSort};

/// 🧩️ One edit as the command carries it: the part of the definition, the operation, the key (a key token, a filter index, a storey id or a phase token) and the value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Edit {
    pub part: String,
    pub op: String,
    pub key: String,
    pub value: String,
}

fn empty(id: &str) -> SetSchedule {
    SetSchedule { id: id.to_string(), name: None, category: None, columns: None, sort: None, filter: None, group: None, itemize: None, storeys: None, phases: None }
}

fn position<T>(items: &[T], at: impl Fn(&T) -> bool) -> Option<usize> {
    items.iter().position(at)
}

fn shift<T>(items: &mut [T], index: usize, up: bool) -> bool {
    let target = if up { index.checked_sub(1) } else { Some(index + 1).filter(|next| *next < items.len()) };
    target.map(|target| items.swap(index, target)).is_some()
}

fn phase_of(token: &str) -> Option<Phase> {
    [Phase::Existing, Phase::New, Phase::Demolished, Phase::Temporary].into_iter().find(|phase| format!("{phase:?}").eq_ignore_ascii_case(token))
}

fn cycle(op: ScheduleOp) -> ScheduleOp {
    let at = ScheduleOp::ALL.iter().position(|known| *known == op).unwrap_or(0);
    ScheduleOp::ALL[(at + 1) % ScheduleOp::ALL.len()]
}

fn columns(schedule: &Schedule, edit: &Edit) -> Result<Vec<ScheduleColumn>, &'static str> {
    let mut list = schedule.columns.clone();
    let key = ScheduleKey::parse(&edit.key).ok_or("bim.schedule.key-unknown")?;
    let at = position(&list, |column| column.key == key);
    match (edit.op.as_str(), at) {
        ("add", None) => list.push(ScheduleColumn { key, heading: None, total: false }),
        ("remove", Some(index)) => {
            list.remove(index);
        }
        ("up" | "down", Some(index)) => {
            shift(&mut list, index, edit.op == "up");
        }
        ("total", Some(index)) => list[index].total = !list[index].total,
        ("heading", Some(index)) => list[index].heading = Some(edit.value.clone()).filter(|heading| !heading.trim().is_empty()),
        _ => return Err("bim.schedule.edit-invalid"),
    }
    Ok(list)
}

fn sorts(schedule: &Schedule, edit: &Edit) -> Result<Vec<ScheduleSort>, &'static str> {
    let mut list = schedule.sort.clone();
    let key = ScheduleKey::parse(&edit.key).ok_or("bim.schedule.key-unknown")?;
    let at = position(&list, |sort| sort.key == key);
    match (edit.op.as_str(), at) {
        ("add", None) => list.push(ScheduleSort { key, descending: false }),
        ("remove", Some(index)) => {
            list.remove(index);
        }
        ("up" | "down", Some(index)) => {
            shift(&mut list, index, edit.op == "up");
        }
        ("descending", Some(index)) => list[index].descending = !list[index].descending,
        _ => return Err("bim.schedule.edit-invalid"),
    }
    Ok(list)
}

fn filters(schedule: &Schedule, edit: &Edit) -> Result<Vec<ScheduleFilter>, &'static str> {
    let mut list = schedule.filter.clone();
    if edit.op == "add" {
        list.push(ScheduleFilter { key: ScheduleKey::parse(&edit.key).ok_or("bim.schedule.key-unknown")?, op: ScheduleOp::NotEmpty, value: String::new() });
        return Ok(list);
    }
    let index: usize = edit.key.parse().map_err(|_| "bim.schedule.edit-invalid")?;
    let filter = list.get_mut(index).ok_or("bim.schedule.edit-invalid")?;
    match edit.op.as_str() {
        "remove" => {
            list.remove(index);
        }
        "op" => {
            filter.op = ScheduleOp::from_symbol(&edit.value).unwrap_or_else(|| cycle(filter.op));
            if filter.op.nullary() {
                filter.value.clear();
            } else if filter.value.trim().is_empty() {
                filter.value = "0".to_string();
            }
        }
        "value" => filter.value = edit.value.clone(),
        _ => return Err("bim.schedule.edit-invalid"),
    }
    Ok(list)
}

fn groups(schedule: &Schedule, edit: &Edit) -> Result<Vec<ScheduleGroup>, &'static str> {
    let mut list = schedule.group.clone();
    let key = ScheduleKey::parse(&edit.key).ok_or("bim.schedule.key-unknown")?;
    let at = position(&list, |group| group.key == key);
    match (edit.op.as_str(), at) {
        ("add", None) => list.push(ScheduleGroup { key }),
        ("remove", Some(index)) => {
            list.remove(index);
        }
        ("up" | "down", Some(index)) => {
            shift(&mut list, index, edit.op == "up");
        }
        _ => return Err("bim.schedule.edit-invalid"),
    }
    Ok(list)
}

fn toggled<T: PartialEq + Clone>(list: &[T], item: T) -> Vec<T> {
    if list.contains(&item) {
        list.iter().filter(|known| **known != item).cloned().collect()
    } else {
        list.iter().cloned().chain(std::iter::once(item)).collect()
    }
}

/// ✏️ The `set-schedule` payload of `edit` on schedule `id`, or the code of the reason it does not apply.
pub fn apply(snapshot: &ModelSnapshot, id: &str, edit: &Edit) -> Result<SetSchedule, &'static str> {
    let schedule = snapshot.schedules.get(id).ok_or("bim.schedule.missing")?;
    let mut payload = empty(id);
    match edit.part.as_str() {
        "name" => payload.name = Some(edit.value.clone()),
        "category" => payload.category = Some(ScheduleCategory::from_token(&edit.value).ok_or("bim.schedule.category-unknown")?),
        "column" => payload.columns = Some(columns(schedule, edit)?),
        "sort" => payload.sort = Some(sorts(schedule, edit)?),
        "filter" => payload.filter = Some(filters(schedule, edit)?),
        "group" => payload.group = Some(groups(schedule, edit)?),
        "itemize" => payload.itemize = Some(!schedule.itemize),
        "storey" => payload.storeys = Some(toggled(&schedule.storeys, edit.key.clone())),
        "phase" => payload.phases = Some(toggled(&schedule.phases, phase_of(&edit.key).ok_or("bim.schedule.phase-unknown")?)),
        _ => return Err("bim.schedule.part-unknown"),
    }
    Ok(payload)
}

/// 📋️ The preset schedule a library row creates under `name`: the preset named `key`, the wall schedule for any other key.
pub fn created(key: &str, name: &str) -> Schedule {
    preset(key, name).or_else(|| preset("wall", name)).expect("the wall preset exists")
}

/// 📋️ The preset keys the library offers.
pub fn presets() -> &'static [&'static str] {
    &PRESETS
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
