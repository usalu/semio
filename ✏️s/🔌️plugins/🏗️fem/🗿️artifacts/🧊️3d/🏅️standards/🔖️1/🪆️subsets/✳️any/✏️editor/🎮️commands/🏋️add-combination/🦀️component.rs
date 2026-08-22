//! 🏋️ 🏋️ FEM 3D app commands command — `add-combination`.

use crate::artifacts::fem3d::mutations::create_combination;
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    /// 📦️ A JSON-encoded `[[caseId, factor], ...]` array — `crate::artifacts::fem3d::FemCombination`'s
    /// `terms` is a `BTreeMap<String, f64>`, not a dedicated record type, so this stays a JSON-string
    /// blob (parsed the same way the pre-migration `handle_action` channel used to) rather than
    /// requiring the DSL engine to grow a `Vec<(String, f64)>` primitive.
    pub terms: String,
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    match serde_json::from_str::<Vec<(String, f64)>>(&payload.terms) {
        Ok(parsed) => {
            let terms: std::collections::BTreeMap<String, f64> = parsed.into_iter().collect();
            let id = crate::app_surface::next_id(snapshot.combinations.iter().map(|c| c.id.clone()), "c");
            Ok(Emit::mutations(vec![Fem3dMutation::CreateCombination(create_combination::mutation::CreateCombination { combination: crate::artifacts::fem3d::FemCombination { id, name: payload.name.clone(), terms } })]))
        }
        Err(_) => Ok(Emit::default()),
    }
}
