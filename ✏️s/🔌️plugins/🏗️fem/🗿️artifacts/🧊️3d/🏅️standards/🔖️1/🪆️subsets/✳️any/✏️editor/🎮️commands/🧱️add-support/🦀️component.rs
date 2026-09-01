//! 🧱️ 🧱️ FEM 3D app commands command — `add-support`.

use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-support")]
pub struct AddSupport {
    pub node_id: String,
    pub fixed: Vec<crate::artifacts::fem3d::FemDof>,
}

pub fn handle(payload: &AddSupport, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.supports.iter().map(|s| s.id.clone()), "sup");
    Ok(Emit::mutations(vec![Fem3dMutation::CreateSupport(crate::artifacts::fem3d::mutations::create_support::mutation::CreateSupport {
        support: crate::artifacts::fem3d::FemSupport { id, node_id: payload.node_id.clone(), fixed: payload.fixed.clone() },
    })]))
}
