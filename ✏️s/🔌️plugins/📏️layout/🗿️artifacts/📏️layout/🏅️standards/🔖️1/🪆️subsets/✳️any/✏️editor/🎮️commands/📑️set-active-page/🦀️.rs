//! 👁️ 👁️ Layout play app commands command — `set-active-page`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-page")]
pub struct SetActivePage {
    pub page_id: String,
}

pub fn handle(_payload: &SetActivePage, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
