//! 🧩️ CAD play app commands — host-pushed `CadComputer` extension contributions.

use crate::editor::cad::config::{CadConfig, CadConfigMutation, SetContributionsEdit};
use crate::op::CadMutation;
use crate::CadSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
    #[dsl(keyword = "contributions")]
    pub struct SetContributions {
        pub json: String,
    }

    pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut crate::editor::cad::CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::config(vec![CadConfigMutation::SetContributions(SetContributionsEdit { json: payload.json.clone() })]))
    }
}
//#endregion 🔖️SetContributions
