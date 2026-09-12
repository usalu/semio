//! 🧱️ 🧱️ Fem2d play app commands command — `add-section`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::FemSection;
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
#[dsl(keyword = "add-section")]
pub struct AddSection {
    pub name: String,
    pub area: f64,
    pub iy: f64,
}

pub fn handle(payload: &AddSection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.sections.iter().map(|s| s.id.clone()), "s");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateSection(crate::standards::v1::subsets::any::schema::mutations::create_section::CreateSection { section: FemSection { id, name: payload.name.clone(), area: payload.area, iy: payload.iy } })]))
}
