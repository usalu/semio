//! 🩹️ Fem3d play app command — `patch-section`: one-field edit of a section (`name`, `area`, `iy`,
//! `iz`, `j`) → `ReplaceSection`.

use crate::standards::v1::subsets::any::schema::mutations::replace_section;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️PatchSection
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-section")]
pub struct PatchSection {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchSection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let section = doc.snapshot.sections.iter().find(|section| section.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.section-missing"))?;
    let mut new_section = section.clone();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.section-value")) };
    match payload.field.as_str() {
        "name" => new_section.name = payload.value.clone(),
        "area" => new_section.area = number()?,
        "iy" => new_section.iy = number()?,
        "iz" => new_section.iz = number()?,
        "j" => new_section.j = number()?,
        _ => return Err(Fault::from("fem3d.patch.section-field")),
    }
    if &new_section == section {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ReplaceSection(replace_section::ReplaceSection { id: payload.id.clone(), new_section })]))
}
//#endregion 🔖️PatchSection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
