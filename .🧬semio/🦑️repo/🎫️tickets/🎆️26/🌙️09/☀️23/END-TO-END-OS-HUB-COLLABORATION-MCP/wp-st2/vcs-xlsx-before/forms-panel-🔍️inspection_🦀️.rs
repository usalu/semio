//! 🔍️ Forms metadata, field and step editing driven by framework-owned selection.

use crate::editor::forms::ui_label;
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
#[path = "🧱️controls/🦀️.rs"]
mod controls;
#[path = "🫥️visibility/🦀️.rs"]
mod visibility;

/// 🎛️ Editable question fields in their presentation order.
pub fn question_fields(kind: &str) -> Vec<&'static str> {
    let mut fields = vec!["label", "kind", "description", "required"];
    fields.extend_from_slice(match kind {
        "text" | "longText" => &["placeholder", "default"],
        "number" | "slider" => &["default", "min", "max", "step", "unit"],
        "boolean" | "date" | "color" => &["default"],
        "single" | "multi" => &["default", "options"],
        "vector" => &["schema", "step", "fields"],
        "note" => &["text"],
        "image" => &["src"],
        "file" => &["accept"],
        _ => &["fixtureSlug", "params"],
    });
    fields.push("condition");
    fields
}

/// 🧭️ Resolves stale and transitive framework selection in document order.
pub fn inspection_model(steps: &[crate::FormStep], selected: &[String]) -> schema::FormsInspection {
    let selected: std::collections::HashSet<&str> = selected.iter().map(String::as_str).collect();
    if let Some(step) = steps.iter().find(|step| selected.contains(crate::schema::forms_play_step_tree_id(&step.id).as_str())) {
        return schema::FormsInspection { scope: "step".into(), ids: vec![step.id.clone()], fields: vec!["title".into(), "description".into()] };
    }
    let questions: Vec<&crate::FormQuestion> = steps.iter().flat_map(|step| &step.blocks).filter(|question| selected.contains(question.id.as_str())).collect();
    let Some(first) = questions.first() else {
        return schema::FormsInspection { scope: "form".into(), ids: Vec::new(), fields: vec!["title".into()] };
    };
    let fields = question_fields(&first.kind).into_iter().filter(|field| questions.iter().all(|question| question_fields(&question.kind).contains(field))).map(str::to_owned).collect();
    schema::FormsInspection { scope: "question".into(), ids: questions.iter().map(|question| question.id.clone()).collect(), fields }
}
use crate::{forms_steps, FormsSnapshot};
use semio_framework_plugin::{tree_item_desc, ui_node_list, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_INSPECTION: &str = "forms.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(FORMS_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🛠️ Every editable row emits an existing typed command; selection stays ephemeral and framework-owned.
pub fn render(
    spec: &FormsSnapshot,
    config: &crate::editor::forms::config::FormsConfig,
    selected: &[String],
    view: &semio_framework_plugin::ViewModel,
    windows: &semio_framework_plugin::TreeWindows<'_>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use controls::*;
    use crate::editor::forms::{ui_value_map, ui_value_text};
    let labels = crate::editor::forms::forms_play_labels(view);
    const ROOT: &str = "forms-play-inspector";
    let steps = forms_steps(spec);
    let model = inspection_model(&steps, selected);
    let summary = ui_node_list([
        text_row(&format!("{ROOT}.title"), labels.title.as_str(), spec.title.as_deref().unwrap_or(""), "updateForm", ui_value_map([])?),
        tree_item_desc(format!("{ROOT}.steps"), labels.steps.as_str(), Some(steps.len().to_string())),
        tree_item_desc(format!("{ROOT}.questions"), labels.questions.as_str(), Some(steps.iter().map(|step| step.blocks.len()).sum::<usize>().to_string())),
    ])?;
    let mut panel = PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.summary"), Some(ui_label(labels.form_fallback_title.as_str())?), true, summary)?;
    if model.scope == "step" {
        let step = steps.iter().find(|step| step.id == model.ids[0]).unwrap();
        let args = |field: &str| arguments(vec![("field", ui_value_text(field)?), ("stepId", ui_value_text(&step.id)?)]);
        let rows = ui_node_list([
            text_row(&format!("{ROOT}.step.title"), labels.title.as_str(), &step.title, "patchStep", args("title")?),
            text_row(&format!("{ROOT}.step.description"), labels.description.as_str(), step.description.as_deref().unwrap_or(""), "patchStep", args("description")?),
            button(&format!("{ROOT}.step.remove"), labels.remove.as_str(), "removeStep", arguments(vec![("stepId", ui_value_text(&step.id)?)])?),
        ])?;
        panel = panel.section(format!("{ROOT}.step"), Some(ui_label(&step.title)?), true, rows)?;
    } else if model.scope == "question" {
        let question = steps.iter().flat_map(|step| &step.blocks).find(|question| question.id == model.ids[0]).unwrap();
        let fields: Vec<&str> = model.fields.iter().map(String::as_str).filter(|field| !["options", "fields", "condition", "params"].contains(field)).collect();
        panel = panel.window_section(windows, &format!("{ROOT}.question"), Some(ui_label(&question.label)?), true, &fields, |field| scalar_row(question, &model.ids, field, view, config))?;
        if model.ids.len() == 1 {
            panel = panel.section(format!("{ROOT}.visibility"), Some(ui_label(labels.visibility.as_str())?), true, ui_node_list([visibility::render(question, &steps, labels)])?)?;
            if let Some(options) = &question.options {
                panel = panel.window_section(windows, &format!("{ROOT}.options"), Some(ui_label(labels.options.as_str())?), true, options, |option| option_row(question, option, labels))?;
                panel = panel.section(format!("{ROOT}.options.actions"), None, true, ui_node_list([button(&format!("{ROOT}.options.add"), labels.add_option.as_str(), "addQuestionOption", arguments(vec![("label", ui_value_text(labels.option.as_str())?), ("questionId", ui_value_text(&question.id)?)])?)])?)?;
            }
            if let Some(fields) = &question.fields {
                panel = panel.window_section(windows, &format!("{ROOT}.vector"), Some(ui_label(labels.kind_vector.as_str())?), true, fields, |field| vector_row(question, field, labels))?;
                let key = (1..).map(|index| format!("v{index}")).find(|key| !fields.iter().any(|field| &field.key == key)).unwrap();
                panel = panel.section(format!("{ROOT}.vector.actions"), None, true, ui_node_list([button(&format!("{ROOT}.vector.add"), labels.add_vector_field.as_str(), "addVectorField", arguments(vec![("fieldKey", ui_value_text(key)?), ("questionId", ui_value_text(&question.id)?)])?)])?)?;
            }
            if crate::schema::is_extension_question_kind(&question.kind) {
                let contributions = crate::editor::forms::parse_contributions(config);
                let extension = crate::editor::forms::render_extension_question(question, &dsl::os_pack::json::Object::new(), &contributions, crate::editor::forms::questions::extensions::ExtensionSurface::Blueprint, true, labels)?;
                panel = panel.section(format!("{ROOT}.extension"), Some(ui_label(labels.parameters.as_str())?), true, ui_node_list([Ok(extension)])?)?;
            }
            panel = panel.section(format!("{ROOT}.question.actions"), None, true, ui_node_list([button(&format!("{ROOT}.question.remove"), labels.remove.as_str(), "removeBlock", arguments(vec![("questionId", ui_value_text(&question.id)?)])?)])?)?;
        }
    }
    panel.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️authoring/🦀️.rs"]
mod authoring_tests;
//#endregion 🧪️Tests
