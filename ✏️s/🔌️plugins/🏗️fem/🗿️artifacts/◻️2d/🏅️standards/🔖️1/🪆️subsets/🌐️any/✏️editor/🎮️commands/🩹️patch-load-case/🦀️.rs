//! 🩹️ Fem2d play app command — `patch-load-case`: one-field edit of a load case (`name`, `selfWeight`) → `ChangeLoadCaseName` / `ChangeLoadCaseSelfWeight`.

use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight;

use semio_framework_plugin::ArtifactView;

use semio_framework_plugin::ConfigView;

use semio_framework_plugin::Emit;

use semio_framework_plugin::Fault;

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️PatchLoadCase
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
            Ok(Emit::mutations(vec![Fem2dMutation::ChangeLoadCaseName(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name::ChangeLoadCaseName { case_id: payload.id.clone(), new_name: payload.value.clone() })]))
        }
        "selfWeight" => {
            let new_self_weight: bool = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.load-case-value"))?;
            if case.self_weight == new_self_weight {
                return Ok(Emit::default());
            }
            Ok(Emit::mutations(vec![Fem2dMutation::ChangeLoadCaseSelfWeight(semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight::ChangeLoadCaseSelfWeight { case_id: payload.id.clone(), new_self_weight })]))
        }
        _ => Err(Fault::from("fem2d.patch.load-case-field")),
    }
}
//#endregion 🔖️PatchLoadCase

//#region 🧪️Tests
//#endregion 🧪️Tests
