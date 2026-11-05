//! 🧩️ Forms contributions: the host-declared plugin contributions parsed from the configuration, the question kind routes and the extension question rendering and catalogue kinds shared by the builder, the try wizard and the inspection panel.

use crate::forms_config::FormsConfig;
use crate::question_extensions::{ExtensionSurface, QuestionKindContribution};
use crate::editor::forms::support::FORMS_PLAY_APP_ID;
use crate::editor::forms::terminology::{forms_play_labels, FormsLabels};
use crate::{FormQuestion, FORM_BUILTIN_KINDS};
use semio_framework::ProgramContributionEntry;
use semio_framework_pack_json::Object;
use semio_plugin::IconName;

//#region 🔖️Contributions
pub fn parse_contributions(config: &FormsConfig) -> Vec<ProgramContributionEntry> {
    semio_framework::parse_contributions(&config.contributions_json)
}

pub use crate::question_extensions::FORMS_QUESTION_KIND_TOPIC;

/// 🧩️ Supplies editor authority and localized diagnostics to the shared question slots.
pub fn render_extension_question(question: &FormQuestion, values: &Object, contributions: &[ProgramContributionEntry], surface: ExtensionSurface<'_>, interactive: bool, labels: &FormsLabels) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    crate::question_extensions::render_extension_question(question, values, contributions, FORMS_PLAY_APP_ID, surface, interactive, labels.extension_unavailable.as_str())
}

/// 🗂️ Every kind offered by the catalogue/inspector kind selector: the built-in kinds (labeled from
/// `labels`) followed by every contributed extension kind. Shared by the blueprint builder's palette, the
/// catalogue panel, and the inspection panel's kind select.
pub fn catalogue_kinds(contributions: &[ProgramContributionEntry], view: &semio_plugin::ViewModel) -> Vec<(String, String, IconName)> {
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
