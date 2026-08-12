//! 🦉 `change-delta-sigma-ed` payload — changes the En1999 document's `delta_sigma_ed` (fatigue design stress range [MPa]).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDeltaSigmaEd
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDeltaSigmaEd {
    pub new_delta_sigma_ed: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeDeltaSigmaEd {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "delta-sigma-ed", kind: "change-delta-sigma-ed", record: "ChangedDeltaSigmaEd" };

    fn diff(&self, base: &En1999Snapshot) -> En1999Diff {
        crate::artifacts::en1999::mutations::change_delta_sigma_ed::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_delta_sigma_ed::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fatigue design stress range [MPa] to {}", self.new_delta_sigma_ed)
    }
}
//#endregion 🔖️ChangeDeltaSigmaEd
