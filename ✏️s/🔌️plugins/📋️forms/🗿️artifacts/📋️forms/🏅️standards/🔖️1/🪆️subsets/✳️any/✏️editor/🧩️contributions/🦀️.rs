//! 🧩️ Forms contributions: the host-declared plugin contributions parsed from the configuration, the question kind routes and the extension question rendering and catalogue kinds shared by the builder, the try wizard and the inspection panel.

use crate::editor::forms::config::FormsConfig;
use crate::editor::forms::questions::extensions::{self, ExtensionSurface, QuestionKindContribution};
use crate::editor::forms::support::{ui_admit, ui_label, ui_text_value, ui_value_map, ui_value_text, FORMS_PLAY_APP_ID};
use crate::editor::forms::terminology::{forms_play_labels, FormsLabels};
use crate::{FormQuestion, FORM_BUILTIN_KINDS};
use semio_framework::ProgramContributionEntry;
use semio_framework_pack_json::Object;
use semio_framework_plugin::IconName;

//#region 🔖️Contributions
pub fn parse_contributions(config: &FormsConfig) -> Vec<ProgramContributionEntry> {
    semio_framework::parse_contributions(&config.contributions_json)
}

const FORMS_QUESTION_KIND_TOPIC: &str = "forms.questionKind";

/// 🎯️ A resolved question-kind contribution's routing fields, sourced from the open `forms.questionKind`
/// topic shape.
struct QuestionKindRoute {
    app_id: String,
    params_body_key: String,
    preview_body_key: String,
}

fn question_kind_route_from_topic(topic_contribution: &semio_framework_plugin::TopicContribution, kind: &str) -> Option<QuestionKindRoute> {
    if topic_contribution.topic != FORMS_QUESTION_KIND_TOPIC {
        return None;
    }
    let payload = topic_contribution.decode::<QuestionKindContribution>().ok()?;
    (payload.question_kind == kind).then_some(QuestionKindRoute { app_id: payload.app_id, params_body_key: payload.params_body_key, preview_body_key: payload.preview_body_key })
}

/// 🗂️ Reads the open `TopicContribution` (`"forms.questionKind"` topic) shape per entry.
fn find_question_kind_contribution<'a>(contributions: &'a [ProgramContributionEntry], kind: &str) -> Option<(&'a str, QuestionKindRoute)> {
    contributions.iter().find_map(|entry| {
        let route = entry.topic_contribution.as_ref().and_then(|topic_contribution| question_kind_route_from_topic(topic_contribution, kind))?;
        Some((entry.plugin_id.as_str(), route))
    })
}

/// 🧩️ Renders a contributed (extension) question kind as a pair of external slots (params editor +
/// preview), or an "Extension unavailable" diagnostic when no contribution is registered for it. Shared
/// by the try wizard and the inspection panel's kind-specific editor fields.
pub fn render_extension_question(question: &FormQuestion, values: &Object, contributions: &[ProgramContributionEntry], surface: ExtensionSurface<'_>, interactive: bool, labels: &FormsLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use semio_framework_ui_contract::{self as ui, Buildable, HasBase, HasChildren};
    let key = format!("{}.extension", question.id);
    let Some((plugin_id, route)) = find_question_kind_contribution(contributions, &question.kind) else {
        return ui_admit(ui_admit(ui::text(ui_label(format!("{}: {}", labels.extension_unavailable.as_str(), question.kind))?).try_id(&key))?.try_build());
    };
    let payload = extensions::render_payload(question, values, FORMS_PLAY_APP_ID, surface, interactive).to_string();
    let mut column = ui_admit(ui::column().try_id(&key))?;
    for body_key in [&route.params_body_key, &route.preview_body_key] {
        let props = ui_value_map([("bodyKey", ui_value_text(body_key)?), ("paramsJson", ui_value_text(&payload)?)])?;
        let slot = ui_admit(ui::extension(ui_text_value(format!("{plugin_id}/{}", route.app_id))?).props(props).try_id(format!("{}.{}", question.id, body_key)))?;
        column = ui_admit(column.try_child(slot))?;
    }
    ui_admit(column.try_build())
}

/// 🗂️ Every kind offered by the catalogue/inspector kind selector: the built-in kinds (labeled from
/// `labels`) followed by every contributed extension kind. Shared by the blueprint builder's palette, the
/// catalogue panel, and the inspection panel's kind select.
pub fn catalogue_kinds(contributions: &[ProgramContributionEntry], view: &semio_framework_plugin::ViewModel) -> Vec<(String, String, IconName)> {
    let labels = forms_play_labels(view);
    let mut kinds: Vec<(String, String, IconName)> = FORM_BUILTIN_KINDS
        .iter()
        .map(|kind| {
            let (label, icon): (&str, &str) = match *kind {
                "text" => (labels.kind_text.as_str(), "type"),
                "longText" => (labels.kind_long_text.as_str(), "align-left"),
                "number" => (labels.kind_number.as_str(), "hash"),
                "slider" => (labels.kind_slider.as_str(), "sliders-horizontal"),
                "boolean" => (labels.kind_boolean.as_str(), "toggle-left"),
                "single" => (labels.kind_single.as_str(), "circle-dot"),
                "multi" => (labels.kind_multi.as_str(), "list-checks"),
                "date" => (labels.kind_date.as_str(), "calendar"),
                "color" => (labels.kind_color.as_str(), "palette"),
                "image" => (labels.kind_image.as_str(), "image"),
                "file" => (labels.kind_file.as_str(), "file"),
                "vector" => (labels.kind_vector.as_str(), "move-3d"),
                "note" => (labels.kind_note.as_str(), "sticky-note"),
                other => (other, "help-circle"),
            };
            (kind.to_string(), label.into(), icon.into())
        })
        .collect();
    for entry in contributions {
        let topic_kind = entry
            .topic_contribution
            .as_ref()
            .filter(|topic_contribution| topic_contribution.topic == FORMS_QUESTION_KIND_TOPIC)
            .and_then(|topic_contribution| topic_contribution.decode::<QuestionKindContribution>().ok())
            .map(|payload| (payload.question_kind, payload.label.resolve(view.terminology, view.locale).to_string(), IconName::from(payload.icon_id.as_str())));
        if let Some(kind) = topic_kind {
            kinds.push(kind);
        }
    }
    kinds
}
//#endregion 🔖️Contributions
