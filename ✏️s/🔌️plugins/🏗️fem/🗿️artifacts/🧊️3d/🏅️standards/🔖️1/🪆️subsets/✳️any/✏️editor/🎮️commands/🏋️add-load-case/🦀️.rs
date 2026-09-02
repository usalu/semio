//! 🏋️ 🏋️ FEM 3D app commands command — `add-load-case`.

use crate::artifacts::fem3d::mutations::create_load_case;
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemLoadCase};
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-load-case")]
pub struct AddLoadCase {
    pub name: String,
    pub self_weight: bool,
}

pub fn handle(payload: &AddLoadCase, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.load_cases.iter().map(|lc| lc.id.clone()), "case-");
    Ok(Emit::mutations(vec![Fem3dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: FemLoadCase { id, name: payload.name.clone(), loads: Vec::new(), self_weight: payload.self_weight } })]))
}
