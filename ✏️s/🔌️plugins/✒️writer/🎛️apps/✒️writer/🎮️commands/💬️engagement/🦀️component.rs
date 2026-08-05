//! 💬️ Writer play app commands — the engagement bar: draft input and natural-language submit.

use crate::apps::writer::config::{WriterConfig, WriterConfigOperation};
use crate::artifacts::writer::op::WriterOperation;
use crate::artifacts::writer::WriterProjection;
use semio_framework_plugin::{engagement_token_matches, strip_engagement_prefix, ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
    }

    pub fn handle(payload: &EngagementInput, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterOperation, WriterConfigOperation>, Fault> {
        let config = cfg.projection;
        if payload.value != config.engagement_input {
            Ok(Emit::config(vec![WriterConfigOperation::SetEngagementInput { value: payload.value.clone() }, WriterConfigOperation::SetRevision { value: config.revision + 1 }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementSubmit
/// 📤️ What `apply_engagement` computes: an optional document text replacement plus the config
/// operations the interaction always produces (mirrors `Emit`'s document/config split — `handle` below
/// wires this straight into one `Emit`).
struct WriterEngagementOutcome {
    text: Option<String>,
    config_operations: Vec<WriterConfigOperation>,
}

/// 💬️ Natural-language engagement parsing (premigration `applyEngagement`). Accepts both the spaced
/// form (wgpu REPL) and the React shell's PascalCased, separator-stripped drafts (e.g. `"Font16"`,
/// `"LineNumbers"` — see `strip_engagement_prefix`).
fn apply_engagement(config: &WriterConfig, current_text: &str, language_id: &str, value: &str) -> WriterEngagementOutcome {
    use crate::artifacts::writer::engine::format_writer_text;

    let trimmed = value.trim();
    let mut config_operations = vec![WriterConfigOperation::SetEngagementInput { value: String::new() }, WriterConfigOperation::SetRevision { value: config.revision + 1 }];
    if trimmed.is_empty() {
        return WriterEngagementOutcome { text: None, config_operations };
    }
    if engagement_token_matches(trimmed, "format") {
        config_operations.push(WriterConfigOperation::SetFormatSignal { value: config.format_signal + 1 });
        let formatted = format_writer_text(current_text, language_id);
        let text = (formatted != current_text).then_some(formatted);
        return WriterEngagementOutcome { text, config_operations };
    }
    if engagement_token_matches(trimmed, "lint") {
        config_operations.push(WriterConfigOperation::SetLintSignal { value: config.lint_signal + 1 });
        return WriterEngagementOutcome { text: None, config_operations };
    }
    if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
        let mut settings = config.editor_settings.clone();
        settings.show_line_numbers = !settings.show_line_numbers;
        config_operations.push(WriterConfigOperation::SetEditorSettings { settings });
        return WriterEngagementOutcome { text: None, config_operations };
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
        if let Ok(px) = rest.parse::<u32>() {
            let mut settings = config.editor_settings.clone();
            settings.font_px = px;
            config_operations.push(WriterConfigOperation::SetEditorSettings { settings });
        }
        return WriterEngagementOutcome { text: None, config_operations };
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
        if let Ok(size) = rest.parse::<u32>() {
            let mut settings = config.editor_settings.clone();
            settings.tab_size = size.max(1);
            config_operations.push(WriterConfigOperation::SetEditorSettings { settings });
        }
    }
    WriterEngagementOutcome { text: None, config_operations }
}

pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {
        pub value: Option<String>,
    }

    pub fn handle(payload: &EngagementSubmit, doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterOperation, WriterConfigOperation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
        let value = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
        let outcome = apply_engagement(config, &document.text, &document.language_id, &value);
        Ok(Emit { document_operations: outcome.text.map(|text| vec![WriterOperation::SetText { text }]).unwrap_or_default(), config_operations: outcome.config_operations, ..Default::default() })
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::engagement_submit;
    use crate::apps::writer::testkit::new_app;
    use crate::apps::writer::{WriterCommand, WRITER_PLAY_WINDOW_KIND};
    use semio_framework_plugin::{PluginApp, WindowMeasure};

    #[test]
    fn engagement_submit_parses_font_size() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("font 16".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("submit");
        // Font size is ephemeral config state — no history entry.
        assert!(result.operations.is_empty());
        let measures = app.window_measures();
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
    }

    #[test]
    fn engagement_submit_parses_normalized_shell_drafts() {
        // The React shell PascalCases and strips separators from every draft before submitting it
        // (`normalizeEngagementActionText`), so "font 16" arrives as "Font16", "tab 4" as "Tab4",
        // and "line numbers" as "LineNumbers".
        let mut app = new_app();
        let before_toggle = app.window_engagements().get(WRITER_PLAY_WINDOW_KIND).and_then(|engagement| engagement.options.as_ref()).and_then(|options| options.first()).and_then(|option| option.pressed).expect("line-numbers pressed state");

        app.dispatch_typed(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Font16".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("font");
        app.dispatch_typed(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Tab4".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("tab");
        app.dispatch_typed(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("LineNumbers".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("line numbers");

        let measures = app.window_measures();
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-tab-size-measure" && *value == 4.0)));

        let after_toggle = app.window_engagements().get(WRITER_PLAY_WINDOW_KIND).and_then(|engagement| engagement.options.as_ref()).and_then(|options| options.first()).and_then(|option| option.pressed).expect("line-numbers pressed state");
        assert_eq!(after_toggle, !before_toggle);
    }
}
//#endregion 🧪️Tests
