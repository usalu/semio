//! 🩹️ Fem3d play app command — `patch-load-case`: one-field edit of a load case (`name` →
//! `ChangeLoadCaseName`, `selfWeight` → `ChangeLoadCaseSelfWeight`).

use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{change_load_case_name, change_load_case_self_weight};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️PatchLoadCase
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-load-case")]
pub struct PatchLoadCase {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchLoadCase, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let case = doc.snapshot.load_cases.iter().find(|case| case.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.load-case-missing"))?;
    let mutation = match payload.field.as_str() {
        "name" => {
            if case.name == payload.value {
                return Ok(Emit::default());
            }
            Fem3dMutation::ChangeLoadCaseName(change_load_case_name::ChangeLoadCaseName { case_id: payload.id.clone(), new_name: payload.value.clone() })
        }
        "selfWeight" => {
            let enabled = match payload.value.trim() {
                "true" | "1" | "on" => true,
                "false" | "0" | "off" | "" => false,
                _ => return Err(Fault::from("fem3d.patch.load-case-value")),
            };
            if case.self_weight == enabled {
                return Ok(Emit::default());
            }
            Fem3dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: payload.id.clone(), new_self_weight: enabled })
        }
        _ => return Err(Fault::from("fem3d.patch.load-case-field")),
    };
    Ok(Emit::mutations(vec![mutation]))
}
//#endregion 🔖️PatchLoadCase

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
