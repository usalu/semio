//! 🧱️ 🧱️ FEM 3D app commands command — `add-section`.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-section")]
pub struct AddSection {
    pub name: String,
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
}

pub fn handle(payload: &AddSection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.sections.iter().map(|s| s.id.clone()), "s");
    Ok(Emit::mutations(vec![Fem3dMutation::CreateSection(crate::artifacts::fem3d::mutations::create_section::mutation::CreateSection { section: crate::artifacts::fem3d::FemSection { id, name: payload.name.clone(), area: payload.area, iy: payload.iy, iz: payload.iz, j: payload.j } })]))
}
