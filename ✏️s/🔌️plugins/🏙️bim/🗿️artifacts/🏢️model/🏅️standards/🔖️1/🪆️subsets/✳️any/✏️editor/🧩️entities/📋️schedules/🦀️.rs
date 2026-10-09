//! 📋️ The schedule rows of the entity table: how an authored schedule reads off the snapshot as text (columns `name, type, net_area*`, sort `length desc, name`, filters `length >= 6; usage empty`, grouping `storey`),
//! how an edited text becomes a `set-schedule` mutation, and what a new schedule is: the preset a library row names (`parent` is its key), the wall schedule for any other.

use super::{parse_flag, parse_text, partial, variant, Created, FieldRow};
use crate::schedule_kit::{columns_text, filters_text, keys_text, parse_columns, parse_filters, parse_groups, parse_sorts, sorts_text};
use crate::{ModelMutation, ModelSnapshot, Phase, ScheduleCategory};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

fn parse_category(text: &str) -> Option<ScheduleCategory> {
    ScheduleCategory::from_token(text)
}

fn ids(text: &str) -> Option<Vec<String>> {
    Some(text.split(',').map(str::trim).filter(|part| !part.is_empty()).map(str::to_string).collect())
}

fn phases(text: &str) -> Option<Vec<Phase>> {
    text.split(',').map(str::trim).filter(|part| !part.is_empty()).map(|part| variant(part, &[Phase::Existing, Phase::New, Phase::Demolished, Phase::Temporary])).collect()
}

fn categories(_: &ModelSnapshot, _: &crate::editor::bim::terminology::BimLabels) -> Vec<(String, String)> {
    ScheduleCategory::ALL.into_iter().map(|category| (category.token().to_string(), category.token().to_string())).collect()
}

/// 🧾️ The authored definition of a schedule.
pub static SCHEDULE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.schedules.get(id).map(|row| row.name.clone()), parse_text => set_schedule::SetSchedule),
    field!("category", field_category, Text, |s, id| s.schedules.get(id).map(|row| row.category.token().to_string()), choices: categories, parse_category => set_schedule::SetSchedule),
    field!("columns", field_columns, LongText, |s, id| s.schedules.get(id).map(|row| columns_text(&row.columns)), |s, id, value| partial::<crate::mutations::set_schedule::SetSchedule, _>(id, "columns", &parse_columns(value, &s.schedules.get(id)?.columns)?).map(ModelMutation::SetSchedule)),
    field!("sort", field_sort, Text, |s, id| s.schedules.get(id).map(|row| sorts_text(&row.sort)), parse_sorts => set_schedule::SetSchedule),
    field!("filter", field_filter, LongText, |s, id| s.schedules.get(id).map(|row| filters_text(&row.filter)), parse_filters => set_schedule::SetSchedule),
    field!("group", field_group, Text, |s, id| s.schedules.get(id).map(|row| keys_text(row.group.iter().map(|group| &group.key))), parse_groups => set_schedule::SetSchedule),
    field!("itemize", field_itemize, Text, |s, id| s.schedules.get(id).map(|row| row.itemize.to_string()), parse_flag => set_schedule::SetSchedule),
    field!("storeys", field_scope_storeys, Text, |s, id| s.schedules.get(id).map(|row| row.storeys.join(", ")), ids => set_schedule::SetSchedule),
    field!("phases", field_scope_phases, Text, |s, id| s.schedules.get(id).map(|row| row.phases.iter().map(|phase| format!("{phase:?}")).collect::<Vec<_>>().join(", ")), phases => set_schedule::SetSchedule),
];

/// 📋️ A new schedule: the preset named by `parent`, else the wall schedule.
pub fn create_schedule(_: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let schedule = crate::editor::bim::modes::edit::windows::schedule::edit::created(parent, name);
    Ok(ModelMutation::CreateSchedule(crate::mutations::create_schedule::CreateSchedule { id: id.into(), schedule }))
}
