//! 🎮️ `iz`, `j`) → `ChangeSection`.
//! 🩹️ Fem3d play app command — `patch-section`: one-field edit of a section (`name`, `area`, `iy`,

use semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::change_section::ChangeSection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSectionPatch;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️PatchSection
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-section")]
pub struct PatchSection {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchSection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let row = doc.snapshot.sections.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.section-missing"))?;
    let mut patch = Fem3dSectionPatch::default();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.section-value")) };
    match payload.field.as_str() {
        "name" => patch.name = Some(payload.value.clone()),
        "area" => patch.area = Some(number()?),
        "iy" => patch.iy = Some(number()?),
        "iz" => patch.iz = Some(number()?),
        "j" => patch.j = Some(number()?),
        _ => return Err(Fault::from("fem3d.patch.section-field")),
    }
    if patch.candidate(row) == *row { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem3dMutation::ChangeSection(ChangeSection { id: payload.id.clone(), patch })]))
}

//#endregion 🔖️PatchSection

//#region 🧪️Tests
//#endregion 🧪️Tests
