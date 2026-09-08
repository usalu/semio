//! 🧱️ 🧱️ FEM 3D app commands command — `add-material`.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-material")]
pub struct AddMaterial {
    pub name: String,
    pub e: f64,
    pub g: f64,
}

/// 🧱️ New materials default to `nu = 0.3`/`rho = 7850.0` (mild steel) — the manifest's `addMaterial`
/// arg form only stages `name`/`e`/`g`, matching the pre-migration `handle_action` behavior verbatim.
pub fn handle(payload: &AddMaterial, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.materials.iter().map(|m| m.id.clone()), "m");
    Ok(Emit::mutations(vec![Fem3dMutation::CreateMaterial(crate::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial {
        material: crate::FemMaterial { id, name: payload.name.clone(), e: payload.e, g: payload.g, nu: 0.3, rho: 7850.0 },
    })]))
}
