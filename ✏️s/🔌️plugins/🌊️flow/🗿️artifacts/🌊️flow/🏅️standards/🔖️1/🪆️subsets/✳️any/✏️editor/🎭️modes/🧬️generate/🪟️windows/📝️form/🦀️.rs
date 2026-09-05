//! 📝️ Generate-mode window — the input form for the active generation.

use crate::artifacts::flow::FlowSnapshot;
use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::terminology::flow_play_labels;
use crate::editor::flow::{flow_action, ui_value_map, ui_value_text};
use crate::playbook::{default_value_for_block, is_block_visible, selected_generation, PlaybookBlock, PlaybookValues};
use flow::forms_bridge::flow_fixture_to_form_spec;
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{ActionId, Buildable, BuiltNode, HasBase, HasChildren, LocalizedLabel, PluginAssemblyError, SurfaceKind, Trigger, UiAssemblyResult, UiFixedList, UiText, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::{self as ui, InputKind};

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_GENERATE_FORM: &str = "flow-generate-form";
pub const FLOW_PLAY_BODY_GENERATE_FORM: &str = "flow.play.generate-form";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FLOW_PLAY_WINDOW_GENERATE_FORM.into(),
        label: LocalizedLabel::native("Form", "Formular"),
        body_key: FLOW_PLAY_BODY_GENERATE_FORM.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "clipboard-list".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn form_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.generate-form", format!("fixed UI admission failed at {stage}"))
}

fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<Label> {
    Label::try_from(value.as_ref().to_string()).map_err(|error| PluginAssemblyError::new("ui.generate-form", error))
}

fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| form_error("text"))
}

/// 🧩️ Builds the one interactive control a question's kind maps to (`📓️recipe-plugin.md` §2's
/// `Input`/`Select`/`Slider` rows). `flow_fixture_to_form_spec` only ever emits `"slider"`/`"note"`/
/// `"image"`/`"text"`/`"single"` (see `forms_bridge::widget_to_playbook_block`'s exhaustive match) — the
/// `_` arm below covers `"text"` and any future addition defensively as a plain text input, and `"note"`/
/// `"image"` never reach this function (handled directly in [`question_field`], unwrapped, no control).
fn question_control(question: &PlaybookBlock, value: &dsl::DslValue, field_id: &str, action: (ActionId, Option<semio_framework_plugin::UiValue>)) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = action;
    match question.kind.as_str() {
        "slider" => {
            let mut builder = ui::slider(value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0))).min(question.min.unwrap_or(0.0)).max(question.max.unwrap_or(100.0)).step(question.step.unwrap_or(1.0));
            builder = builder.try_id(format!("{field_id}.slider")).map_err(|_| form_error("slider-id"))?;
            builder = match args {
                Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| form_error("slider-binding"))?,
                None => builder.try_on(Trigger::Change, action).map_err(|_| form_error("slider-binding"))?,
            };
            builder.try_build().map_err(|_| form_error("slider-build"))
        }
        "single" => {
            let mut builder = ui::select(ui_text(value.as_str().unwrap_or_default())?);
            for option in question.options.iter().flatten() {
                builder = builder.try_item(ui_text(&option.value)?, ui_label(&option.label)?).map_err(|_| form_error("select-item"))?;
            }
            if let Some(placeholder) = question.placeholder.as_deref() {
                builder = builder.placeholder(ui_label(placeholder)?);
            }
            builder = builder.try_id(format!("{field_id}.select")).map_err(|_| form_error("select-id"))?;
            builder = match args {
                Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| form_error("select-binding"))?,
                None => builder.try_on(Trigger::Change, action).map_err(|_| form_error("select-binding"))?,
            };
            builder.try_build().map_err(|_| form_error("select-build"))
        }
        _ => {
            let mut builder = ui::input(InputKind::Text).value(ui_text(value.as_str().unwrap_or_default())?);
            if let Some(placeholder) = question.placeholder.as_deref() {
                builder = builder.placeholder(ui_label(placeholder)?);
            }
            builder = builder.try_id(format!("{field_id}.input")).map_err(|_| form_error("input-id"))?;
            builder = match args {
                Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| form_error("input-binding"))?,
                None => builder.try_on(Trigger::Change, action).map_err(|_| form_error("input-binding"))?,
            };
            builder.try_build().map_err(|_| form_error("input-build"))
        }
    }
}

/// 🏷️ One question, wrapped in its labeled `Field` row — `None` when the question is conditionally
/// hidden. `"note"`/`"image"` questions render as bare text (no field wrapper, no control), matching the
/// retired `render_question_field`'s identical early-return shape.
fn question_field(question: &PlaybookBlock, values: &PlaybookValues, patch_action: &str, generation_id: &str) -> UiAssemblyResult<Option<BuiltNode>> {
    if !is_block_visible(question, values) {
        return Ok(None);
    }
    match question.kind.as_str() {
        "note" => return Ok(Some(ui::text(ui_label(question.text.clone().unwrap_or_default())?).try_build().map_err(|_| form_error("note-build"))?)),
        "image" => return Ok(Some(ui::text(ui_label(question.src.clone().unwrap_or_else(|| "(no image)".into()))?).try_build().map_err(|_| form_error("image-build"))?)),
        _ => {}
    }
    let value = values.get(&question.id).cloned().unwrap_or_else(|| default_value_for_block(question));
    let field_id = format!("generate.form.{}", question.id);
    let args = ui_value_map([("generationId", ui_value_text(generation_id)?), ("questionId", ui_value_text(&question.id)?)])?;
    let action = flow_action(patch_action, Some(args))?;
    let control = question_control(question, &value, &field_id, action)?;
    let builder = ui::field(ui_label(&question.label)?);
    let builder = builder.try_id(&field_id).map_err(|_| form_error("field-id"))?;
    let builder = builder.try_child(control).map_err(|_| form_error("field-child"))?;
    Ok(Some(builder.try_build().map_err(|_| form_error("field-build"))?))
}

pub fn render(fixture: &FlowSnapshot, config: &FlowConfig) -> UiAssemblyResult<BuiltNode> {
    let spec = flow_fixture_to_form_spec(&fixture.to_fixture());
    let generation = config.generation();
    let Some(active) = selected_generation(&generation) else {
        return ui::text(ui_label(flow_play_labels(config).generation_needed.as_str())?).try_build().map_err(|_| form_error("placeholder-build"));
    };
    let mut children = UiFixedList::<BuiltNode>::default();
    for step in &spec.steps {
        if !step.blocks.is_empty() {
            children.try_push(ui::text(ui_label(&step.title)?).try_build().map_err(|_| form_error("step-title-build"))?).map_err(|_| form_error("children"))?;
        }
        for question in &step.blocks {
            if let Some(field) = question_field(question, &active.values, "updateGenerationValues", &active.id)? {
                children.try_push(field).map_err(|_| form_error("children"))?;
            }
        }
    }
    if children.is_empty() {
        return ui::text(ui_label("No input widgets to generate from.")?).try_build().map_err(|_| form_error("empty-build"));
    }
    let builder = ui::column().try_children(children).map_err(|_| form_error("root-children"))?;
    builder.try_build().map_err(|_| form_error("root-build"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn without_a_generation_the_form_shows_the_placeholder_copy() {
        let mut app = flow_app().await;
        assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
    }
}
//#endregion 🧪️Tests
