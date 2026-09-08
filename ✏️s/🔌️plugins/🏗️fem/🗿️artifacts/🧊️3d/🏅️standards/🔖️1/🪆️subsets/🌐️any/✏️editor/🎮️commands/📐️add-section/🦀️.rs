//! 🧱️ 🧱️ FEM 3D app commands command — `add-section`.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
    Ok(Emit::mutations(vec![Fem3dMutation::CreateSection(crate::standards::v1::subsets::any::schema::mutations::create_section::CreateSection {
        section: crate::FemSection { id, name: payload.name.clone(), area: payload.area, iy: payload.iy, iz: payload.iz, j: payload.j },
    })]))
}
