//! 🧩️ Playbook play app commands — host-pushed plugin contributions (extension block kinds).

use crate::apps::playbook::config::{PlaybookConfig, PlaybookConfigOperation};
use crate::artifacts::playbook::{op::PlaybookOperation, PlaybookSpec};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "contributions")]
    pub struct SetContributions {
        pub json: String,
    }

    pub fn handle(payload: &SetContributions, _doc: &DocumentView<'_, PlaybookSpec>, _cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookOperation, PlaybookConfigOperation>, Fault> {
        Ok(Emit::config(vec![PlaybookConfigOperation::SetContributions { json: payload.json.clone() }]))
    }
}
//#endregion 🔖️SetContributions
