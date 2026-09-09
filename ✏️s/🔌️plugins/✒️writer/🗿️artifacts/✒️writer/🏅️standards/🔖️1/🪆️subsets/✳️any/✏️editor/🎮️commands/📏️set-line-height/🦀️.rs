//! ⚙️ ⚙️ Writer play app commands command — `set-line-height`.

use crate::op::WriterMutation;
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "line-height")]
pub struct SetLineHeight {
    pub value: u32,
}

pub fn handle(_payload: &SetLineHeight, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("writer window settings require the retained exact-window reducer"))
}
