//! 🧱️ 🧱️ Fem2d play app commands command — `add-material`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::FemMaterial;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️AddNode
//#endregion 🔖️AddNode

//#region 🔖️AddBar
//#endregion 🔖️AddBar

//#region 🔖️AddBeam
//#endregion 🔖️AddBeam

//#region 🔖️AddMaterial
//#endregion 🔖️AddMaterial

//#region 🔖️AddSection
//#endregion 🔖️AddSection

//#region 🔖️AddSupport
//#endregion 🔖️AddSupport

//#region 🔖️AddRegion
//#endregion 🔖️AddRegion

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-material")]
pub struct AddMaterial {
    pub name: String,
    pub e: f64,
}

pub fn handle(payload: &AddMaterial, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.materials.iter().map(|m| m.id.clone()), "m");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateMaterial(crate::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial { material: FemMaterial { id, name: payload.name.clone(), e: payload.e, nu: 0.3, rho: 7850.0 } })]))
}
