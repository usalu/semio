//! 🏋️ 🏋️ FEM 3D app commands command — `set-self-weight`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-self-weight")]
pub struct SetSelfWeight {
    pub case_id: String,
    pub enabled: bool,
}

pub fn handle(payload: &SetSelfWeight, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    match snapshot.load_cases.iter().any(|lc| lc.id == payload.case_id) {
        true => Ok(Emit::mutations(vec![Fem3dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: payload.case_id.clone(), new_self_weight: payload.enabled })])),
        false => Ok(Emit::default()),
    }
}
