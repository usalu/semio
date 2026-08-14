//! 💬️ 💬️ Writer play app commands command — `engagement-submit`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::{engagement_token_matches, strip_engagement_prefix, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️EngagementInput
//#endregion 🔖️EngagementInput

//#region 🔖️EngagementSubmit
/// 📤️ What `apply_engagement` computes: an optional document text replacement plus the config
/// operations the interaction always produces (mirrors `Emit`'s document/config split — `handle` below
/// wires this straight into one `Emit`).
struct WriterEngagementOutcome {
    text: Option<String>,
    config_mutations: Vec<WriterConfigMutation>,
}

/// 💬️ Natural-language engagement parsing (premigration `applyEngagement`). Accepts both the spaced
/// form (wgpu REPL) and the React shell's PascalCased, separator-stripped drafts (e.g. `"Font16"`,
/// `"LineNumbers"` — see `strip_engagement_prefix`).
fn apply_engagement(config: &WriterConfig, current_text: &str, language_id: &str, value: &str) -> WriterEngagementOutcome {
    use crate::artifacts::writer::schema::format_writer_text;

    let trimmed = value.trim();
    let mut config_mutations = vec![WriterConfigMutation::SetEngagementInput { value: String::new() }, WriterConfigMutation::SetRevision { value: config.revision + 1 }];
    if trimmed.is_empty() {
        return WriterEngagementOutcome { text: None, config_mutations };
    }
    if engagement_token_matches(trimmed, "format") {
        config_mutations.push(WriterConfigMutation::SetFormatSignal { value: config.format_signal + 1 });
        let formatted = format_writer_text(current_text, language_id);
        let text = (formatted != current_text).then_some(formatted);
        return WriterEngagementOutcome { text, config_mutations };
    }
    if engagement_token_matches(trimmed, "lint") {
        config_mutations.push(WriterConfigMutation::SetLintSignal { value: config.lint_signal + 1 });
        return WriterEngagementOutcome { text: None, config_mutations };
    }
    if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
        let mut settings = config.editor_settings.clone();
        settings.show_line_numbers = !settings.show_line_numbers;
        config_mutations.push(WriterConfigMutation::SetEditorSettings { settings });
        return WriterEngagementOutcome { text: None, config_mutations };
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
        if let Ok(px) = rest.parse::<u32>() {
            let mut settings = config.editor_settings.clone();
            settings.font_px = px;
            config_mutations.push(WriterConfigMutation::SetEditorSettings { settings });
        }
        return WriterEngagementOutcome { text: None, config_mutations };
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
        if let Ok(size) = rest.parse::<u32>() {
            let mut settings = config.editor_settings.clone();
            settings.tab_size = size.max(1);
            config_mutations.push(WriterConfigMutation::SetEditorSettings { settings });
        }
    }
    WriterEngagementOutcome { text: None, config_mutations }
}

//#endregion 🔖️EngagementSubmit

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: Option<String>,
}

pub fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let value = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
    let current_text = writer_text(document);
    let outcome = apply_engagement(config, &current_text, &document.language_id, &value);
    Ok(Emit { artifact_mutations: outcome.text.map(|text| vec![WriterMutation::EditText(EditText { text })]).unwrap_or_default(), config_mutations: outcome.config_mutations, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::EngagementSubmit;
        use crate::apps::writer::testkit::new_app;
    use crate::apps::writer::{WriterCommand, WRITER_PLAY_WINDOW_KIND};
    use semio_framework_plugin::{PluginApp, WindowMeasure};

    #[test]
    fn engagement_submit_parses_font_size() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("font 16".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("submit");
        // Font size is ephemeral config state — no history entry.
        assert!(result.mutations.is_empty());
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

        app.dispatch_typed(WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("Font16".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("font");
        app.dispatch_typed(WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("Tab4".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("tab");
        app.dispatch_typed(WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("LineNumbers".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("line numbers");

        let measures = app.window_measures();
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-tab-size-measure" && *value == 4.0)));

        let after_toggle = app.window_engagements().get(WRITER_PLAY_WINDOW_KIND).and_then(|engagement| engagement.options.as_ref()).and_then(|options| options.first()).and_then(|option| option.pressed).expect("line-numbers pressed state");
        assert_eq!(after_toggle, !before_toggle);
    }
}
//#endregion 🧪️Tests
