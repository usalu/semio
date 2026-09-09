//! ⚙️ ⚙️ Writer play app commands command — `set-font-px`.

use crate::op::WriterMutation;
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "font-px")]
pub struct SetFontPx {
    pub value: u32,
}

pub fn handle(_payload: &SetFontPx, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("writer window settings require the retained exact-window reducer"))
}
