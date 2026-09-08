//! 🏋️ 🏋️ Fem2d play app commands command — `set-self-weight`.

use crate::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️AddNodalLoad
//#endregion 🔖️AddNodalLoad

//#region 🔖️AddMemberUdl
//#endregion 🔖️AddMemberUdl

//#region 🔖️AddAreaLoad
//#endregion 🔖️AddAreaLoad

//#region 🔖️AddLoadCase
//#endregion 🔖️AddLoadCase

//#region 🔖️AddCombination
//#endregion 🔖️AddCombination

//#region 🔖️SetSelfWeight
//#endregion 🔖️SetSelfWeight

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-self-weight")]
pub struct SetSelfWeight {
    pub case_id: String,
    pub enabled: bool,
}

pub fn handle(payload: &SetSelfWeight, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    match snapshot.load_cases.iter().any(|lc| lc.id == payload.case_id) {
        true => Ok(Emit::mutations(vec![Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: payload.case_id.clone(), new_self_weight: payload.enabled })])),
        false => Ok(Emit::default()),
    }
}
