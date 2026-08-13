//! 👁️ 👁️ Animate present app commands command — `set-locale`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::valid_tile_ids;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit::config(vec![PresentConfigMutation::SetLocale { value: payload.value.clone() }]))
}
