//! 👓️ Read-only form defaults and contributed question previews.
//! Scalar values use artifact-level snapshot helpers. Extensions receive no answer lease and
//! retain their own view-only computation; the viewer cannot emit document or draft mutations.

use crate::standards::v1::subsets::any::io::text::snapshot::{dsl_to_value,json_string_value};
use crate::schema::{default_value_for_question,is_extension_question_kind};
use crate::{forms_steps, FormQuestion, FormsSnapshot};
use semio_ui_locale::LocalizedLabel;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;
use semio_ui_contract as ui;
use ui::{Buildable, HasBase, HasChildren};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "forms-view-try";
pub const BODY_KEY: &str = "forms.view.try";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
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
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn admit<T, E>(value: Result<T, E>) -> UiAssemblyResult<T> {
    value.map_err(|_| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "forms viewer admission failed"))
}

/// 🔑️ Explicit key per text row — `try_build` stamps unkeyed nodes `"#0"` and `try_child` re-keys only
/// EMPTY keys, so a step title next to its description collided (`DuplicateSiblingKey`); see the editor
/// Try window's `display` (ticket 26/09/16/FORMS-PLUGIN-END-TO-END).
pub fn text(key: &str, value: &str, emphasize: bool) -> UiAssemblyResult<ui::BuiltNode> {
    let mut node: ui::BuiltNode = ui::text(admit(ui::Label::try_from(value))?).emphasize(emphasize).into();
    node.key = ui::UiText::try_from_str(key).ok_or_else(|| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "forms viewer key admission failed"))?;
    Ok(node)
}

fn viewer_field(question: &FormQuestion, child: ui::BuiltNode) -> UiAssemblyResult<ui::BuiltNode> {
    let mut field = admit(ui::field(admit(ui::Label::try_from(question.label.as_str()))?).try_id(format!("forms-view-try.{}", question.id)))?;
    if let Some(description) = &question.description {
        field = field.description(ui::UiText::try_from_str(description).ok_or_else(|| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "forms viewer description admission failed"))?);
    }
    admit(admit(field.try_child(child))?.try_build())
}

fn read_only_field(question: &FormQuestion, value_text: &str) -> UiAssemblyResult<ui::BuiltNode> {
    let child = if value_text.len() > 512 || question.kind == "longText" {
        let mut scene = semio_plugin::TextEditorScene::base(value_text.to_owned(), None, None);
        scene.settings_json = Some(semio_framework_pack_json::to_json_string(&semio_framework_pack_json::json!({
            "readOnly": true,
            "label": question.label
        })));
        semio_plugin::scene_surface(&format!("forms-view-try.{}.value", question.id), ui::SurfaceKind::TextEditor, &scene)?
    } else {
        text(&format!("forms-view-try.{}.value", question.id), value_text, false)?
    };
    viewer_field(question, child)
}

fn render_view_question(question: &FormQuestion, contributions: &[semio_framework::ProgramContributionEntry], view: &semio_plugin::ViewModel) -> UiAssemblyResult<ui::BuiltNode> {
    if is_extension_question_kind(&question.kind) {
        let unavailable = LocalizedLabel::native("Extension unavailable", "Erweiterung nicht verfügbar");
        let child = crate::question_extensions::render_extension_question(
            question,
            &semio_framework_pack_json::Object::new(),
            contributions,
            crate::viewer::forms::FORMS_VIEW_APP_ID,
            crate::question_extensions::ExtensionSurface::View,
            false,
            unavailable.resolve(view.terminology, view.locale),
        )?;
        return viewer_field(question, child);
    }
    let value = dsl_to_value(&default_value_for_question(question));
    read_only_field(question, &json_string_value(&value))
}

pub fn render(document: &FormsSnapshot, contributions: &[semio_framework::ProgramContributionEntry], view: &semio_plugin::ViewModel) -> UiAssemblyResult<ui::BuiltNode> {
    let steps = forms_steps(document);
    if steps.is_empty() {
        return text("forms-view-try.empty", LocalizedLabel::native("No steps in this form.", "Dieses Formular hat keine Schritte.").resolve(view.terminology, view.locale), false);
    }
    let untitled = LocalizedLabel::native("Form", "Formular");
    let mut column = admit(ui::column().try_child(text("forms-view-try.title", document.title.as_deref().unwrap_or_else(|| untitled.resolve(view.terminology, view.locale)), true)?))?;
    for step in &steps {
        let mut section = admit(ui::column().try_id(format!("forms-view-try.step.{}", step.id)))?;
        section = admit(section.try_child(text(&format!("forms-view-try.step.{}.title", step.id), &step.title, true)?))?;
        if let Some(description) = &step.description {
            section = admit(section.try_child(text(&format!("forms-view-try.step.{}.description", step.id), description, false)?))?;
        }
        for question in &step.blocks {
            section = admit(section.try_child(render_view_question(question, contributions, view)?))?;
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
