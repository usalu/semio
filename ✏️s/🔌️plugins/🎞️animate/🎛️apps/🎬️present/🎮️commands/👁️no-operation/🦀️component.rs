//! 👁️ 👁️ Animate present app commands command — `no-operation`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::PresentDispatchCtx;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 👁️ Decorative no-op wired to the read-only "active source" catalogue field's `on_change` — never
/// mutates anything (mirrors the pre-B1 `"noMutation"` view action verbatim).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "no-op")]
pub struct NoOperation {}

pub fn handle(_payload: &NoOperation, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit::default())
}
