//! 🧩️ CAD play app commands — host-pushed `CadComputer` extension contributions.

use crate::apps::cad::config::{CadConfig, CadConfigOperation};
use crate::artifacts::cad::op::CadOperation;
use crate::artifacts::cad::CadProjection;
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

    pub fn handle(payload: &SetContributions, _doc: &DocumentView<'_, CadProjection>, _cfg: &ConfigView<'_, CadConfig>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        Ok(Emit::config(vec![CadConfigOperation::SetContributions { json: payload.json.clone() }]))
    }
}
//#endregion 🔖️SetContributions
