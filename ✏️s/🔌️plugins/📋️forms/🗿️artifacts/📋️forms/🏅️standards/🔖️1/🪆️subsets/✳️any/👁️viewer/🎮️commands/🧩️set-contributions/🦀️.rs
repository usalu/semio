//! 🧩️ Installs the host's question-kind projection into read-only Forms configuration.
pub use crate::forms_config::SetContributions;
use crate::forms_config::{FormsConfig, FormsConfigMutation};
use crate::FormsSnapshot;
use semio_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};

pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<ViewEmit<FormsConfigMutation>, Fault> {
    Ok(ViewEmit::config(vec![FormsConfigMutation::SetContributions(payload.clone())]))
}
