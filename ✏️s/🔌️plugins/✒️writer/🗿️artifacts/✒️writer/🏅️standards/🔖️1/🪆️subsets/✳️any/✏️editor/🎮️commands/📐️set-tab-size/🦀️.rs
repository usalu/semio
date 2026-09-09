//! ⚙️ ⚙️ Writer play app commands command — `set-tab-size`.

use crate::op::WriterMutation;
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "tab-size")]
pub struct SetTabSize {
    pub value: u32,
}

pub fn handle(_payload: &SetTabSize, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("writer window settings require the retained exact-window reducer"))
}
