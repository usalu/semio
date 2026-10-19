//! 🏋️ 🏋️ FEM 3D app commands command — `add-combination`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_combination;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    /// 📦️ A JSON-encoded `[[caseId, factor], ...]` array — `semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination`'s
    /// `terms` is a `BTreeMap<String, f64>`, not a dedicated record type, so this stays a JSON-string
    /// blob (parsed the same way the pre-migration `handle_action` channel used to) rather than
    /// requiring the DSL engine to grow a `Vec<(String, f64)>` primitive.
    pub terms: String,
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    match semio_framework_pack_json::from_json_str::<Vec<(String, f64)>>(&payload.terms, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(parsed) => {
            let terms: std::collections::BTreeMap<String, f64> = parsed.into_iter().collect();
            let id = doc.operation()?.entity_id("c", "combinations")?;
            Ok(Emit::mutations(vec![Fem3dMutation::CreateCombination(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination { combination: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination { id, name: payload.name.clone(), terms }, index: None })]))
        }
        Err(_) => Ok(Emit::default()),
    }
}
