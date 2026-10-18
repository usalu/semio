//! 📋️ Manifest of the BIM editor: the action argument folding, the keybindings and the app manifest stitch of `create_bim_app`. See [`super`].

use crate::editor::bim::commands::gesture_keys;
use crate::editor::bim::modes::edit;
use crate::editor::bim::modes::edit::windows::{family, plan, schedule, section, sheet, world};
use crate::editor::bim::panels::{families as families_panel, classification as classification_panel, diagnostics as diagnostics_panel, coordination as coordination_panel, library as library_panel, outliner as outliner_panel, properties as properties_panel};
use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::BIM_MODEL_DIALECT;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::{PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::Editor;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::LocalizedLabel;
use crate::editor::bim::*;

//#region 🔖️Manifest
fn bim_action_args(id: &str) -> Vec<ActionArgDef> {
    let label = BimLabels::localized;
    let text = |name: &'static str, pick: fn(&BimLabels) -> semio_framework_ui_locale::LabelText| ActionArgDef::text(name, label(pick));
    let ids = || ActionArgDef::text_list("ids", label(|labels| labels.arg_entities));
    match id {
        "createEntity" => vec![text("kind", |labels| labels.arg_kind).required(), text("parent", |labels| labels.arg_container), text("name", |labels| labels.arg_name)],
        "createView" => vec![text("kind", |labels| labels.arg_kind).required(), text("parent", |labels| labels.arg_container), text("name", |labels| labels.arg_name)],
        "deleteSelection" | "flipWalls" | "storeyUp" | "storeyDown" | "clearConditions" => vec![ids()],
        "applyConditions" => vec![ids(), text("source", |labels| labels.arg_source_space)],
        "renameEntity" => vec![text("id", |labels| labels.arg_entity).required(), text("name", |labels| labels.arg_name).required()],
        "setField" => vec![ids(), text("field", |labels| labels.arg_parameter).required(), text("value", |labels| labels.arg_value).required()],
        "setView" => vec![text("field", |labels| labels.arg_setting).required(), text("value", |labels| labels.arg_value)],
        "editFamily" => vec![text("id", |labels| labels.arg_entity).required(), text("part", |labels| labels.arg_part).required(), text("op", |labels| labels.arg_operation).required(), text("key", |labels| labels.arg_key), text("value", |labels| labels.arg_value)],
        "editSchedule" => vec![text("id", |labels| labels.arg_entity).required(), text("part", |labels| labels.arg_part).required(), text("op", |labels| labels.arg_operation).required(), text("key", |labels| labels.arg_key), text("value", |labels| labels.arg_value)],
        "placeComponent" | "openFamily" => vec![text("family", |labels| labels.arg_family).required()],
        "searchFamilies" => vec![text("query", |labels| labels.arg_query), text("category", |labels| labels.field_category)],
        "setOverride" => vec![text("component", |labels| labels.arg_component).required(), text("name", |labels| labels.arg_parameter).required(), text("value", |labels| labels.arg_value)],
        "splitWall" => vec![ids(), text("at", |labels| labels.arg_fraction)],
        "placeGridColumns" => vec![ids(), text("storey", |labels| labels.arg_container)],
        "attachWalls" => vec![ids(), text("target", |labels| labels.arg_target)],
        "setProperty" => vec![ids(), text("pset", |labels| labels.arg_property_set), text("property", |labels| labels.arg_property), text("value", |labels| labels.arg_value).required()],
        "removeProperty" => vec![ids(), text("pset", |labels| labels.arg_property_set).required(), text("property", |labels| labels.arg_property).required()],
        "selectFindings" => vec![ids()],
        "viewClash" => vec![text("first", |labels| labels.arg_first).required(), text("second", |labels| labels.arg_second).required(), text("mode", |labels| labels.arg_clash_mode)],
        "raiseIssue" => vec![text("set", |labels| labels.arg_clash_set), text("first", |labels| labels.arg_first).required(), text("second", |labels| labels.arg_second).required()],
        "captureViewpoint" | "restoreViewpoint" => vec![text("issue", |labels| labels.arg_issue).required()],
        "removeClassification" => vec![ids(), text("system", |labels| labels.arg_classification_system).required()],
        "setClassification" => vec![ids(), text("system", |labels| labels.arg_classification_system).required(), text("code", |labels| labels.arg_classification_code).required()],
        "applyTemplate" => vec![ids(), text("template", |labels| labels.arg_template).required()],
        "editTemplate" => vec![text("id", |labels| labels.arg_entity).required(), text("op", |labels| labels.arg_operation).required(), text("index", |labels| labels.arg_index), text("field", |labels| labels.arg_parameter), text("value", |labels| labels.arg_value)],
        "editClassification" => vec![text("id", |labels| labels.arg_entity).required(), text("op", |labels| labels.arg_operation).required(), text("code", |labels| labels.arg_classification_code), text("title", |labels| labels.arg_classification_title), text("parent", |labels| labels.arg_parent)],
        "searchClassification" => vec![text("system", |labels| labels.arg_classification_system).required(), text("query", |labels| labels.arg_query)],
        "placeElements" => vec![ids(), text("at", |labels| labels.arg_position).required()],
        "engagementInput" | "engagementSubmit" => vec![text("value", |labels| labels.arg_typed_line)],
        _ => Vec::new(),
    }
}

/// ⌨️ The keys of the commands that are no utility: history, the delete keys and the wall flip.
pub const COMMAND_KEYBINDINGS: &[(&str, &str)] = &[("mod+z", "undo"), ("mod+shift+z", "redo"), ("delete", "deleteSelection"), ("backspace", "deleteSelection"), ("shift+f", "flipWalls"), ("shift+r", "attachWalls"), ("alt+arrowup", "storeyUp"), ("alt+arrowdown", "storeyDown"), ("shift+h", "applyConditions"), ("alt+h", "clearConditions")];

/// ⌨️ Every keybinding of the editor in registration order: the commands, the utilities and the keys of a gesture in progress. No key is bound twice.
pub fn all_keybindings() -> Vec<(&'static str, &'static str)> {
    COMMAND_KEYBINDINGS.iter().copied().chain(crate::editor::bim::utilities::keybindings()).chain(crate::editor::bim::gestures::GESTURE_KEYBINDINGS.iter().copied()).chain(gesture_keys::KEYBINDINGS.iter().copied()).collect()
}

/// 🗂️ One framework-predefined panel tab whose nested tabs carry the editor's unique panel ids.
fn framework_panel_tab(id: &str, label: LocalizedLabel, group: PanelGroup, children: Vec<PanelTabDefinition>) -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(id.into()), label, group, body_key: None, children }
}

/// 🧱️ The editor manifest: one definition per taxonomy node.
pub fn create_bim_app() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(BIM_MODEL_DIALECT)
        .document(["semio", "bim"])
        .artifact_kind(crate::standards::v1::subsets::any::schema::artifact_kind())
        .icon_id("building")
        .mode_def(edit::definition())
        .default_mode_id(edit::BIM_EDIT_MODE_EDIT)
        .window_kind_def(plan::definition())
        .window_kind_def(world::definition())
        .window_kind_def(section::definition())
        .window_kind_def(schedule::definition())
        .window_kind_def(family::definition())
        .window_kind_def(family::view_definition())
        .window_kind_def(sheet::definition())
        .default_layout(edit::layout())
        .panel_tab_def(outliner_panel::definition())
        .panel_tab_def(framework_panel_tab(
            FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
            LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
            PanelGroup::Workbench,
            vec![library_panel::definition(), families_panel::definition(), classification_panel::definition()],
        ))
        .panel_tab_def(framework_panel_tab(
            FRAMEWORK_PANEL_TAB_INSPECTION_ID,
            LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
            PanelGroup::Details,
            vec![
                properties_panel::definition(),
                diagnostics_panel::definition(),
                crate::editor::bim::panels::options::definition(),
                coordination_panel::clashes_definition(),
                coordination_panel::rules_definition(),
                coordination_panel::issues_definition(),
            ],
        ))
        .io(bim_io());
    for definition in crate::editor::bim::interaction::definitions() {
        builder = builder.interaction(definition);
    }
    for utility in crate::editor::bim::utilities::definitions() {
        builder = builder.utility(utility);
    }
    for action in command_actions() {
        let id = action.id.clone();
        let pointer = crate::editor::bim::gestures::POINTER_COMMAND_IDS.contains(&id.as_str());
        let described = ActionDefinition { in_palette: !pointer, ..action.with_args(bim_action_args(&id)) };
        builder = builder.action_with(described).action_interactive_job(&id, InteractiveJobClassification::Migrated);
        if pointer {
            builder = builder.action_audience(&id, semio_framework_plugin::CapabilityAudience::Input);
        }
    }
    builder = builder.action_destructive("deleteSelection");
    for (keys, action) in all_keybindings() {
        builder = builder.keybinding(keys, action);
    }
    builder.build_definition()
}
//#endregion 🔖️Manifest
