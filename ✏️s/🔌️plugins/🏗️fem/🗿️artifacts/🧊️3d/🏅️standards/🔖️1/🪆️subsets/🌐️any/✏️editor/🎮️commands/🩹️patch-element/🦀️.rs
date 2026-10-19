//! 🎮️ `materialId`, `sectionId`, `roll`) → `ChangeElement`.
//! 🩹️ Fem3d play app command — `patch-element`: one-field edit of a member (`kind`, `start`, `end`,

use semio_s_artifact_fem_3d_mesh_elements::standards::v1::subsets::any::schema::mutations::change_element::ChangeElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dElementPatch;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️PatchElement
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-element")]
pub struct PatchElement {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchElement, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let row = doc.snapshot.elements.iter().find(|row| element_id(row) == payload.id).ok_or_else(|| Fault::from("fem3d.patch.element-missing"))?;
    let mut patch = Fem3dElementPatch::default();
    let value = payload.value.trim();
    match payload.field.as_str() {
        "kind" if matches!(value, "bar" | "frame") => patch.kind = Some(value.into()),
        "kind" => return Err(Fault::from("fem3d.patch.element-value")),
        "start" => patch.start = Some(value.into()),
        "end" => patch.end = Some(value.into()),
        "materialId" => patch.material_id = Some(value.into()),
        "sectionId" => patch.section_id = Some(value.into()),
        "roll" => patch.roll = Some(value.parse().map_err(|_| Fault::from("fem3d.patch.element-value"))?),
        _ => return Err(Fault::from("fem3d.patch.element-field")),
    }
    let candidate = patch.candidate(row).map_err(|_| Fault::from("fem3d.patch.element-field"))?;
    if candidate == *row { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem3dMutation::ChangeElement(ChangeElement { id: payload.id.clone(), patch })]))
}
//#endregion 🔖️PatchElement

//#region 🧪️Tests
//#endregion 🧪️Tests
