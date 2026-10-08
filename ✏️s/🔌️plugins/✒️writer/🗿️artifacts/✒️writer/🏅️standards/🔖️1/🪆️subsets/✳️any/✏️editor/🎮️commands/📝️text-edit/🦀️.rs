//! ✍️ ✍️ Writer play app commands command — `text-edit`.

use crate::mutations::{EditText, WriterMutation};
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "text-edit")]
pub struct TextEdit {
    pub text: String,
}

/// ⌨️ The whole text a host delivers: one `edit-text`. A live typing delivery (`typing` argument) folds into its window's typing
/// run, which commits as ONE edit (design §13.2); a one-shot dispatch is one edit.
pub fn handle(payload: &TextEdit, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![WriterMutation::EditText(EditText { text: payload.text.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
