//! ⚙️ ⚙️ Writer play app commands command — `toggle-line-numbers`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "toggle-line-numbers")]
pub struct ToggleLineNumbers {}

pub fn handle(_payload: &ToggleLineNumbers, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let mut settings = config.editor_settings.clone();
    settings.show_line_numbers = !settings.show_line_numbers;
    Ok(Emit::config(vec![WriterConfigMutation::SetEditorSettings { settings }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::ToggleLineNumbers;
        use crate::apps::writer::testkit::new_app;
    use crate::apps::writer::WriterCommand;

    #[test]
    fn view_action_emits_no_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::ToggleLineNumbers(ToggleLineNumbers {}), &semio_framework_plugin::testkit::meta("local")).expect("toggle");
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
