//! 🏷️ VCS editor command — `rename-vcs`: one concrete `rename_vcs` mutation for the demo document.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "rename-vcs")]
pub struct RenameVcs {
    pub title: String,
}

pub fn handle(payload: &RenameVcs, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    use crate::mutations::rename_vcs;
    Ok(Emit::mutations(vec![rename_vcs(payload.title.clone())]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
