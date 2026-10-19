//! 🧭️ Entity table for options, groups and worksets; shared authored parameter controls.
use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use crate::editor::bim::entities::*;
use semio_s_artifact_bim_model_viewer::editor::bim::entities::create_rows::Created;
use semio_s_artifact_bim_model_authored::editor::bim::entities::field_codecs::{first, parse_text, parse_flag};
use semio_s_artifact_bim_model_viewer::editor::bim::entities::row_types::{FieldRow, EntityKind};
use semio_s_artifact_bim_model_mut_any_116::standards::v1::subsets::any::schema::mutations::delete_design_option;
use semio_s_artifact_bim_model_mut_any_115::standards::v1::subsets::any::schema::mutations::delete_option_group;
use semio_s_artifact_bim_model_mut_any_117::standards::v1::subsets::any::schema::mutations::delete_workset;
use semio_s_artifact_bim_model_mut_any_48::standards::v1::subsets::any::schema::mutations::set_design_option;
use semio_s_artifact_bim_model_mut_any_47::standards::v1::subsets::any::schema::mutations::set_option_group;
use semio_s_artifact_bim_model_mut_any_49::standards::v1::subsets::any::schema::mutations::set_workset;

fn create_group(_: &ModelSnapshot, id: &str, _: &str, name: &str) -> Created { Ok(ModelMutation::CreateOptionGroup(semio_s_artifact_bim_model_mut_any_115::standards::v1::subsets::any::schema::mutations::create_option_group::CreateOptionGroup { id: id.into(), option_group: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::OptionGroup { name: name.into() } })) }
fn create_option(snapshot: &ModelSnapshot, id: &str, parent: &str, name: &str) -> Created {
    let group = if snapshot.option_groups.contains_key(parent) { parent.to_owned() } else { first(&snapshot.option_groups).ok_or("bim.create.option-group-missing")? };
    let primary = !snapshot.design_options.values().any(|option| option.group == group && option.primary);
    Ok(ModelMutation::CreateDesignOption(semio_s_artifact_bim_model_mut_any_116::standards::v1::subsets::any::schema::mutations::create_design_option::CreateDesignOption { id: id.into(), design_option: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::DesignOption { group, name: name.into(), primary } }))
}
fn create_workset(_: &ModelSnapshot, id: &str, _: &str, name: &str) -> Created { Ok(ModelMutation::CreateWorkset(semio_s_artifact_bim_model_mut_any_117::standards::v1::subsets::any::schema::mutations::create_workset::CreateWorkset { id: id.into(), workset: semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Workset { name: name.into(), default_visible: true } })) }
fn option_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> { std::iter::once((String::new(), String::new())).chain(snapshot.design_options.iter().map(|(id, option)| (id.clone(), option.name.clone()))).collect() }
fn workset_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> { std::iter::once((String::new(), String::new())).chain(snapshot.worksets.iter().map(|(id, workset)| (id.clone(), workset.name.clone()))).collect() }
fn group_choices(snapshot: &ModelSnapshot, _: &BimLabels) -> Vec<(String, String)> { snapshot.option_groups.iter().map(|(id, group)| (id.clone(), group.name.clone())).collect() }
fn set_option(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> { Some(ModelMutation::SetElementOption(semio_s_artifact_bim_model_mut_any_50::standards::v1::subsets::any::schema::mutations::set_element_option::SetElementOption { id: id.into(), option: (!value.is_empty()).then(|| value.to_owned()) })) }
fn set_workset(_: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> { Some(ModelMutation::SetElementWorkset(semio_s_artifact_bim_model_mut_any_51::standards::v1::subsets::any::schema::mutations::set_element_workset::SetElementWorkset { id: id.into(), workset: (!value.is_empty()).then(|| value.to_owned()) })) }
pub static MEMBERSHIP_FIELDS: &[FieldRow] = &[
    field!("design_option", field_design_option, Text, |s, id| Some(s.element_options.get(id).map(|row| row.target.clone()).unwrap_or_default()), choices: option_choices, write: set_option),
    field!("workset", field_workset, Text, |s, id| Some(s.element_worksets.get(id).map(|row| row.target.clone()).unwrap_or_default()), choices: workset_choices, write: set_workset),
];
static GROUP_FIELDS: &[FieldRow] = &[field!("name", field_name, Text, |s, id| s.option_groups.get(id).map(|row| row.name.clone()), parse_text => set_option_group::SetOptionGroup)];
static OPTION_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.design_options.get(id).map(|row| row.name.clone()), parse_text => set_design_option::SetDesignOption),
    field!("group", field_option_group, Text, |s, id| s.design_options.get(id).map(|row| row.group.clone()), choices: group_choices, parse_text => set_design_option::SetDesignOption),
    field!("primary", field_primary_option, Text, |s, id| s.design_options.get(id).map(|row| row.primary.to_string()), parse_flag => set_design_option::SetDesignOption),
];
static WORKSET_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.worksets.get(id).map(|row| row.name.clone()), parse_text => set_workset::SetWorkset),
    field!("default_visible", field_workset_visible, Text, |s, id| s.worksets.get(id).map(|row| row.default_visible.to_string()), parse_flag => set_workset::SetWorkset),
];
pub const OPTION_GROUP: EntityKind = kind!("option-group", "layers", false, kind_option_group, group_option_groups, option_groups . name, parent: |_, _| None, delete: delete!(delete_option_group::DeleteOptionGroup), rename: renaming!(set_option_group::SetOptionGroup), create: Some(create_group), fields: GROUP_FIELDS, inferred: &[]);
pub const DESIGN_OPTION: EntityKind = kind!("design-option", "git-branch", false, kind_design_option, group_design_options, design_options . name, parent: |s, id| s.design_options.get(id).map(|row| row.group.clone()), delete: delete!(delete_design_option::DeleteDesignOption), rename: renaming!(set_design_option::SetDesignOption), create: Some(create_option), fields: OPTION_FIELDS, inferred: &[]);
pub const WORKSET: EntityKind = kind!("workset", "users", false, kind_workset, group_worksets, worksets . name, parent: |_, _| None, delete: delete!(delete_workset::DeleteWorkset), rename: renaming!(set_workset::SetWorkset), create: Some(create_workset), fields: WORKSET_FIELDS, inferred: &[]);
