//! 💬️ 💬️ Writer play app commands command — `engagement-submit`.

use crate::op::{EditText, WriterMutation};
use crate::{writer_text, WriterSnapshot};
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use semio_framework_plugin::{engagement_token_matches, strip_engagement_prefix, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

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
    use crate::schema::format_writer_text;

    let trimmed = value.trim();
    let mut config_mutations = vec![WriterConfigMutation::SetEngagementInput(crate::editor::writer::config::SetEngagementInput { value: String::new() }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })];
    if trimmed.is_empty() {
        return WriterEngagementOutcome { text: None, config_mutations };
    }
    if engagement_token_matches(trimmed, "format") {
        config_mutations.push(WriterConfigMutation::SetFormatSignal(crate::editor::writer::config::SetFormatSignal { value: config.format_signal + 1 }));
        let formatted = format_writer_text(current_text, language_id);
        let text = (formatted != current_text).then_some(formatted);
        return WriterEngagementOutcome { text, config_mutations };
    }
    if engagement_token_matches(trimmed, "lint") {
        config_mutations.push(WriterConfigMutation::SetLintSignal(crate::editor::writer::config::SetLintSignal { value: config.lint_signal + 1 }));
        return WriterEngagementOutcome { text: None, config_mutations };
    }
    if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
        let mut settings = config.editor_settings.clone();
        settings.show_line_numbers = !settings.show_line_numbers;
        config_mutations.push(WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }));
        return WriterEngagementOutcome { text: None, config_mutations };
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
        if let Ok(px) = rest.parse::<u32>() {
            let mut settings = config.editor_settings.clone();
            settings.font_px = px;
            config_mutations.push(WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }));
        }
        return WriterEngagementOutcome { text: None, config_mutations };
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
        if let Ok(size) = rest.parse::<u32>() {
            let mut settings = config.editor_settings.clone();
            settings.tab_size = size.max(1);
            config_mutations.push(WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }));
        }
    }
    WriterEngagementOutcome { text: None, config_mutations }
}

//#endregion 🔖️EngagementSubmit

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
