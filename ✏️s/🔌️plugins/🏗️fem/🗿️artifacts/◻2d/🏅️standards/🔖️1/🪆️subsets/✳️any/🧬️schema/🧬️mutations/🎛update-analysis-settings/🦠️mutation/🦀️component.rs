//! 🎛️ Fem2d mutation — `UpdateAnalysisSettings` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemAnalysisSettings};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🎛️ Atomically updates the document's inseparable analysis-settings facet (`modal_count`,
/// `buckling_count`, `deformation_scale`) — never meaningfully set one field at a time (the command
/// layer always merges partial input onto the current settings before emitting this).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-analysis-settings")]
pub struct UpdateAnalysisSettings {
    #[dsl(block)]
    pub settings: FemAnalysisSettings,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for UpdateAnalysisSettings {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "analysis-settings", kind: "update-analysis-settings", record: "UpdatedAnalysisSettings" };

    fn diff(&self, base: &Fem2dSnapshot) -> crate::artifacts::fem2d::diff::Fem2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update analysis settings".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
