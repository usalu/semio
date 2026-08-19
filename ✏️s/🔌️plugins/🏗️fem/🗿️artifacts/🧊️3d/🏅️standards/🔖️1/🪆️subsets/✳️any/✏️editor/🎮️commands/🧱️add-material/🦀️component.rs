//! 🧱️ 🧱️ FEM 3D app commands command — `add-material`.

use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-material")]
pub struct AddMaterial {
    pub name: String,
    pub e: f64,
    pub g: f64,
}

/// 🧱️ New materials default to `nu = 0.3`/`rho = 7850.0` (mild steel) — the manifest's `addMaterial`
/// arg form only stages `name`/`e`/`g`, matching the pre-migration `handle_action` behavior verbatim.
pub async fn handle(payload: &AddMaterial, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.materials.iter().map(|m| m.id.clone()), "m");
    Ok(Emit::mutations(vec![Fem3dMutation::CreateMaterial(crate::artifacts::fem3d::mutations::create_material::mutation::CreateMaterial { material: crate::artifacts::fem3d::FemMaterial { id, name: payload.name.clone(), e: payload.e, g: payload.g, nu: 0.3, rho: 7850.0 } })]))
}
