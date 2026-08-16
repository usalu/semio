//! ▶️ Forms viewer — the Try window: a read-only preview of the form as an end user would see it.
//! Built from the SAME artifact-level pure snapshot helpers the editor's own Try window
//! (the sibling editor surface's `🪟️windows/▶️try`) uses — this file itself imports nothing from
//! that sibling surface (`policyViewerPurityBreaches` forbids it outright). No wizard navigation,
//! no answer entry: a viewer has no utilities that edit and emits no mutations by construction
//! (`ViewEmit`), so every step's questions render flat, in document order, showing each question's
//! typed default value as plain text.

use crate::artifacts::forms::schema::{default_value_for_question, dsl_to_value, is_extension_question_kind, json_string_value};
use crate::artifacts::forms::{forms_steps, FormQuestion, FormsSnapshot};
use semio_framework_plugin::{Label, LocalizedLabel, SurfaceKind, UiFieldNode, UiNode, UiPresence, UiStackNode, UiTextNode, WindowKindDefinition, WindowOptions};

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
fn ui_text_emphasized(value: impl Into<Label>) -> UiNode {
    UiNode::Text(UiTextNode { value: value.into(), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn read_only_field(question: &FormQuestion, value_text: String) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: format!("forms-view-try.{}", question.id),
        label: Label::data(question.label.clone()),
        description: question.description.clone(),
        required: None,
        error: None,
        child: Box::new(semio_framework_plugin::ui_text(Label::data(value_text))),
        presence: UiPresence::default(),
        menu: None,
    })
}

/// 👁️ One question's typed default rendered as plain, non-interactive text — the read-only twin of
/// the editor Try window's per-kind input widgets. Extension question kinds (a host-side contribution
/// the editor resolves through the sibling editor surface's own contribution plumbing) fall back to a
/// plain "kind" label here rather than resolving any contribution, since a viewer declares no config
/// lane to carry `contributions_json`.
fn render_view_question(question: &FormQuestion) -> UiNode {
    if is_extension_question_kind(&question.kind) {
        return read_only_field(question, format!("({})", question.kind));
    }
    let value = dsl_to_value(&default_value_for_question(question));
    read_only_field(question, json_string_value(&value))
}

/// 👁️ Pure `FormsSnapshot -> UiNode` read: every step's questions rendered flat, in document order,
/// each showing its typed default value as plain text. No step-by-step wizard state (no `Config`),
/// no answer entry, no navigation — see this file's own doc comment.
pub fn render(document: &FormsSnapshot) -> UiNode {
    let steps = forms_steps(document);
    if steps.is_empty() {
        return semio_framework_plugin::ui_text(Label::data("No steps in this form."));
    }
    let mut children = vec![ui_text_emphasized(Label::data(document.title.clone().unwrap_or_else(|| "Form".into())))];
    for step in &steps {
        children.push(ui_text_emphasized(Label::data(step.title.clone())));
        if let Some(description) = &step.description {
            children.push(semio_framework_plugin::ui_text(Label::data(description.clone())));
        }
        for question in &step.blocks {
            children.push(render_view_question(question));
        }
    }
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_canvas2d_try_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(matches!(def.surface_kind, SurfaceKind::Canvas2d));
    }

    #[test]
    fn render_produces_a_node_for_the_default_document() {
        let document = crate::artifacts::forms::schema::building_component_spec();
        let node = render(&document);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"stack\""));
    }

    #[test]
    fn render_falls_back_to_a_placeholder_for_an_empty_document() {
        let document = crate::artifacts::forms::schema::empty_forms_snapshot();
        let node = render(&document);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("No steps"));
    }
}
//#endregion 🧪️Tests
