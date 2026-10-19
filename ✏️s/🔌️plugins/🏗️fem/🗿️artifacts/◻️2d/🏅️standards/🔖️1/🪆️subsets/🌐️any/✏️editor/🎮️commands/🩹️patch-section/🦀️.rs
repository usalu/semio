//! 🩹️ Fem2d play app command — `patch-section`: one-field edit of a cross-section (`name`, `area`, `iy`) → `ChangeSection`.

use semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::change_section::ChangeSection;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSectionPatch;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️PatchSection
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-section")]
pub struct PatchSection {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Edits one cross-section property. `area` is in m² and `iy` in m⁴ — the inspector's number
/// inputs carry the raw SI magnitude, never a display-scaled one, so no conversion happens here.
pub fn handle(payload: &PatchSection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let row = doc.snapshot.sections.iter().find(|row| row.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.section-missing"))?;
    let mut patch = Fem2dSectionPatch::default();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.section-value")) };
    match payload.field.as_str() {
        "name" => patch.name = Some(payload.value.clone()),
        "area" => patch.area = Some(number()?),
        "iy" => patch.iy = Some(number()?),
        _ => return Err(Fault::from("fem2d.patch.section-field")),
    }
    if patch.candidate(row) == *row { return Ok(Emit::default()); }
    Ok(Emit::mutations(vec![Fem2dMutation::ChangeSection(ChangeSection { id: payload.id.clone(), patch })]))
}

//#endregion 🔖️PatchSection

//#region 🧪️Tests
//#endregion 🧪️Tests
