//! ▶️ Forms viewer — the Try window: a read-only preview of the form as an end user would see it.
//! Built from the SAME artifact-level pure snapshot helpers the editor's own Try window
//! (the sibling editor surface's `🪟️windows/▶️try`) uses — this file itself imports nothing from
//! that sibling surface (`policyViewerPurityBreaches` forbids it outright). No wizard navigation,
//! no answer entry: a viewer has no utilities that edit and emits no mutations by construction
//! (`ViewEmit`), so every step's questions render flat, in document order, showing each question's
//! typed default value as plain text.

use crate::schema::{default_value_for_question, dsl_to_value, is_extension_question_kind, json_string_value};
use crate::{forms_steps, FormQuestion, FormsSnapshot};
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract as ui;
use ui::{Buildable, HasBase, HasChildren};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "forms-view-try";
pub const BODY_KEY: &str = "forms.view.try";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Try", "Testen"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "play".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 👁️ Read-only preview — no `.window_kind_interactions(..)` reference for this window.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn admit<T, E>(value: Result<T, E>) -> UiAssemblyResult<T> {
    value.map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "forms viewer admission failed"))
}

fn text(value: &str, emphasize: bool) -> UiAssemblyResult<ui::BuiltNode> {
    admit(ui::text(admit(ui::Label::try_from(value))?).emphasize(emphasize).try_build())
}

fn read_only_field(question: &FormQuestion, value_text: String) -> UiAssemblyResult<ui::BuiltNode> {
    let mut field = admit(ui::field(admit(ui::Label::try_from(question.label.as_str()))?).try_id(format!("forms-view-try.{}", question.id)))?;
    if let Some(description) = &question.description {
        field = field.description(ui::UiText::try_from_str(description).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "forms viewer description admission failed"))?);
    }
    admit(admit(field.try_child(text(&value_text, false)?))?.try_build())
}

fn render_view_question(question: &FormQuestion) -> UiAssemblyResult<ui::BuiltNode> {
    if is_extension_question_kind(&question.kind) {
        return read_only_field(question, format!("({})", question.kind));
    }
    let value = dsl_to_value(&default_value_for_question(question));
    read_only_field(question, json_string_value(&value))
}

pub fn render(document: &FormsSnapshot) -> UiAssemblyResult<ui::BuiltNode> {
    let steps = forms_steps(document);
    if steps.is_empty() {
        return text("No steps in this form.", false);
    }
    let mut column = admit(ui::column().try_child(text(document.title.as_deref().unwrap_or("Form"), true)?))?;
    for step in &steps {
        let mut section = admit(ui::column().try_id(format!("forms-view-try.step.{}", step.id)))?;
        section = admit(section.try_child(text(&step.title, true)?))?;
        if let Some(description) = &step.description {
            section = admit(section.try_child(text(description, false)?))?;
        }
        for question in &step.blocks {
            section = admit(section.try_child(render_view_question(question)?))?;
        }
        column = admit(column.try_child(section))?;
    }
    admit(column.try_build())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
