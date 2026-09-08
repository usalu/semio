//! 🏋️ 🏋️ FEM 3D app commands command — `add-combination`.

use crate::standards::v1::subsets::any::schema::mutations::create_combination;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    /// 📦️ A JSON-encoded `[[caseId, factor], ...]` array — `crate::FemCombination`'s
    /// `terms` is a `BTreeMap<String, f64>`, not a dedicated record type, so this stays a JSON-string
    /// blob (parsed the same way the pre-migration `handle_action` channel used to) rather than
    /// requiring the DSL engine to grow a `Vec<(String, f64)>` primitive.
    pub terms: String,
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    match dsl::json::from_json_str::<Vec<(String, f64)>>(&payload.terms) {
        Ok(parsed) => {
            let terms: std::collections::BTreeMap<String, f64> = parsed.into_iter().collect();
            let id = crate::app_surface::next_id(snapshot.combinations.iter().map(|c| c.id.clone()), "c");
            Ok(Emit::mutations(vec![Fem3dMutation::CreateCombination(create_combination::CreateCombination { combination: crate::FemCombination { id, name: payload.name.clone(), terms } })]))
        }
        Err(_) => Ok(Emit::default()),
    }
}
