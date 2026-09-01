//! 🧱️ 🧱️ FEM 3D app commands command — `add-frame`.

use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-frame")]
pub struct AddFrame {
    pub start: String,
    pub end: String,
    pub material_id: String,
    pub section_id: String,
    pub roll: f64,
}

pub fn handle(payload: &AddFrame, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.elements.iter().map(|e| crate::artifacts::fem3d::element_id(e).to_string()), "e");
    let element = crate::artifacts::fem3d::FemElement::Frame { id, start: payload.start.clone(), end: payload.end.clone(), material_id: payload.material_id.clone(), section_id: payload.section_id.clone(), roll: payload.roll };
    Ok(Emit::mutations(vec![Fem3dMutation::CreateElement(crate::artifacts::fem3d::mutations::create_element::mutation::CreateElement { element: Box::new(element) })]))
}
