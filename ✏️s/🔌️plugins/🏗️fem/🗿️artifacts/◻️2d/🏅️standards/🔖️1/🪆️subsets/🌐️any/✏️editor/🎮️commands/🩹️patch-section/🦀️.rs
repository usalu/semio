//! 🩹️ Fem2d play app command — `patch-section`: one-field edit of a cross-section (`name`, `area`, `iy`) → `ReplaceSection`.

use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::replace_section;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

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
    let section = doc.snapshot.sections.iter().find(|section| section.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.section-missing"))?;
    let mut new_section = section.clone();
    match payload.field.as_str() {
        "name" => new_section.name = payload.value.clone(),
        "area" => new_section.area = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.section-value"))?,
        "iy" => new_section.iy = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.section-value"))?,
        _ => return Err(Fault::from("fem2d.patch.section-field")),
    }
    if &new_section == section {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ReplaceSection(replace_section::ReplaceSection { id: payload.id.clone(), new_section })]))
}
//#endregion 🔖️PatchSection

//#region 🧪️Tests
//#endregion 🧪️Tests
