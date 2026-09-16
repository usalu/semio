//! 🩹️ Fem2d play app command — `patch-load-case`: one-field edit of a load case (`name`, `selfWeight`) → `ChangeLoadCaseName` / `ChangeLoadCaseSelfWeight`.

use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{change_load_case_name, change_load_case_self_weight};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️PatchLoadCase
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-load-case")]
pub struct PatchLoadCase {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ The one patch command that does NOT emit a whole-record replace: a load case owns its
/// `loads` collection, and re-sending that collection to rename the case would make every
/// concurrent `add-load` on the same case a lost update. The vocabulary therefore carries two
/// narrow changes and this handler picks the one the edited field names.
pub fn handle(payload: &PatchLoadCase, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let case = doc.snapshot.load_cases.iter().find(|case| case.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.load-case-missing"))?;
    match payload.field.as_str() {
        "name" => {
            if case.name == payload.value {
                return Ok(Emit::default());
            }
            Ok(Emit::mutations(vec![Fem2dMutation::ChangeLoadCaseName(change_load_case_name::ChangeLoadCaseName { case_id: payload.id.clone(), new_name: payload.value.clone() })]))
        }
        "selfWeight" => {
            let new_self_weight: bool = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.load-case-value"))?;
            if case.self_weight == new_self_weight {
                return Ok(Emit::default());
            }
            Ok(Emit::mutations(vec![Fem2dMutation::ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: payload.id.clone(), new_self_weight })]))
        }
        _ => Err(Fault::from("fem2d.patch.load-case-field")),
    }
}
//#endregion 🔖️PatchLoadCase

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
