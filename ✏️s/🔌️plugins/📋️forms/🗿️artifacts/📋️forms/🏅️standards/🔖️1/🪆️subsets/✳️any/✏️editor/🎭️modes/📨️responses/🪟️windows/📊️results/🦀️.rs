//! 📊️ Windowed submission history with original field labels and explicit export actions.
use crate::editor::forms::{forms_action, ui_admit, ui_label, ui_value_map, ui_value_text, FORMS_PLAY_APP_ID};
use crate::editor::forms::terminology::FormsLabels;
use crate::schema::response::{FormsAnswer, FormsResponse};
use crate::FormsSnapshot;
use semio_framework_plugin::row_action;
use semio_framework_plugin::row_target;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::tree_window_item;
use semio_framework_plugin::ui_node_list;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::RowActionPlacement;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiText;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
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
        semio_framework_value::DslValue::Null => labels.unanswered.as_str().to_string(),
        semio_framework_value::DslValue::String(value) => value.clone(),
        semio_framework_value::DslValue::Bool(value) => if *value { labels.yes.as_str() } else { labels.no.as_str() }.to_string(),
        value => semio_framework_pack_json::to_json_string(value),
    };
    ui_admit(ui_admit(ui::tree_item(ui_label(&answer.label)?).try_id(&answer.question_id))?.description(UiText::clipped(&value)).try_build())
}

fn response_row(response: &FormsResponse, labels: &FormsLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let submitted_at = i64::try_from(response.submitted_at).map(protocol::scalar::format_rfc3339_ms).unwrap_or_else(|_| response.submitted_at.to_string());
    let target = row_target(FORMS_PLAY_APP_ID, Some(ui_value_map([("id", ui_value_text(&response.id)?)])?), None)?;
    let item = ui_admit(ui::tree_item(ui_label(&response.id)?).try_id(&response.id))?
        .description(UiText::clipped(&format!("{}: {} · {}: {}", labels.answers.as_str(), response.answers.len(), labels.submitted_at.as_str(), submitted_at)))
        .target(target);
    let item = ui_admit(item.try_row_action(row_action("trash-2", labels.discard_response.as_str(), "discardResponse", RowActionPlacement::Menu)?))?;
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
