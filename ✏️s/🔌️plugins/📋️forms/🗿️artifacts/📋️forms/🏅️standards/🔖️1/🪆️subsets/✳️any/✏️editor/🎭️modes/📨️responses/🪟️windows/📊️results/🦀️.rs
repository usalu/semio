//! 📊️ Windowed submission history with original field labels and explicit export actions.
use crate::editor::forms::{forms_action, ui_admit, ui_label, ui_text_value, ui_value_map, ui_value_text};
use crate::editor::forms::terminology::FormsLabels;
use crate::schema::response::{FormsAnswer, FormsResponse};
use crate::FormsSnapshot;
use semio_framework_plugin::{tree_item_with_action, tree_window_item, ui_node_list, BuiltNode, LocalizedLabel, PanelTreeBuilder, SurfaceKind, TreeWindows, UiAssemblyResult, UiText, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract as ui;
use ui::{Buildable, HasBase};

pub const WINDOW: &str = "forms-responses";
pub const BODY: &str = "forms.play.responses";
pub const ROOT: &str = "forms-responses";
pub const RESPONSES: &str = "forms-responses.items";

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW.into(), label: LocalizedLabel::native("Responses", "Antworten"), body_key: BODY.into(), surface_kind: SurfaceKind::Canvas2d,
        icon_id: "list-checks".into(), options: WindowOptions::default(), actions: Vec::new(), utilities: Vec::new(),
        params_schema: None, artifact_snapshot_schema: None, input_event_schema: None, output_schema: None, capabilities: Vec::new(), interactions: Vec::new(),
    }
}

fn answer_row(answer: &FormsAnswer, labels: &FormsLabels) -> UiAssemblyResult<BuiltNode> {
    let value = match &answer.value {
        dsl::DslValue::Null => labels.unanswered.as_str().to_string(),
        dsl::DslValue::String(value) => value.clone(),
        dsl::DslValue::Bool(value) => if *value { labels.yes.as_str() } else { labels.no.as_str() }.to_string(),
        value => dsl::os_pack::json::to_json_string(value),
    };
    ui_admit(ui_admit(ui::tree_item(ui_label(&answer.label)?).try_id(&answer.question_id))?.description(UiText::clipped(&value)).try_build())
}

fn response_row(response: &FormsResponse, labels: &FormsLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let submitted_at = i64::try_from(response.submitted_at).map(protocol::scalar::format_rfc3339_ms).unwrap_or_else(|_| response.submitted_at.to_string());
    let (action, args) = forms_action("discardResponse", Some(ui_value_map([("id", ui_value_text(&response.id)?)])?))?;
    let item = ui_admit(ui::tree_item(ui_label(&response.id)?).try_id(&response.id))?
        .description(UiText::clipped(&format!("{}: {} · {}: {}", labels.answers.as_str(), response.answers.len(), labels.submitted_at.as_str(), submitted_at)));
    let item = ui_admit(item.try_row_action(ui::RowAction {
        icon: ui_text_value("trash-2")?, label: Some(ui_label(labels.discard_response.as_str())?),
        action: ui::ActionBinding { trigger: ui::Trigger::Activate, action, args, capability: None }, placement: ui::RowActionPlacement::Menu,
    }))?;
    tree_window_item(windows, item, &response.id, true, &response.answers, |answer| answer_row(answer, labels))
}

pub fn render(spec: &FormsSnapshot, labels: &FormsLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let export = |format: &str, label: &str| tree_item_with_action(format!("{ROOT}.export-{format}"), ui_label(label)?, None,
        forms_action("exportResponses", Some(ui_value_map([("format", ui_value_text(format)?)])?))?);
    PanelTreeBuilder::new(ROOT)?
        .section(format!("{ROOT}.actions"), Some(ui_label(labels.actions.as_str())?), true, ui_node_list([
            export("json", labels.export_json.as_str()), export("csv", labels.export_csv.as_str()),
        ])?)?
        .window_section_or_placeholder(windows, RESPONSES, Some(ui_label(format!("{} ({})", labels.responses.as_str(), spec.responses.len()))?), true, &spec.responses, |response| response_row(response, labels, windows), labels.no_responses.as_str())?
        .build()
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
