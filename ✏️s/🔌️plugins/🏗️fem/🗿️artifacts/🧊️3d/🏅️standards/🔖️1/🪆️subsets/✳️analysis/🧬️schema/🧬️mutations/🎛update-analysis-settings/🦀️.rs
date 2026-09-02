//! 🎛️ Fem3d mutation — `UpdateAnalysisSettings` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemAnalysisSettings};
use crate::artifacts::fem3d::diff::Fem3dDiff;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🎛️ Atomically updates the document's inseparable analysis-settings facet (`modal_count`,
/// `buckling_count`, `deformation_scale`) — never meaningfully set one field at a time (the command
/// layer always merges partial input onto the current settings before emitting this).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "update-analysis-settings")]
pub struct UpdateAnalysisSettings {
    #[dsl(block)]
    pub settings: FemAnalysisSettings,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for UpdateAnalysisSettings {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "analysis-settings", kind: "update-analysis-settings", record: "UpdatedAnalysisSettings" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
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
