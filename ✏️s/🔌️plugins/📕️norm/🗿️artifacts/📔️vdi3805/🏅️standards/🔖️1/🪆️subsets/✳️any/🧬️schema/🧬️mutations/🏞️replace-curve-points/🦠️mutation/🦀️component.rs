//! 📉️ `replace-curve-points` — whole-value swap of a curve's interpolation point list, addressed
//! by id.

use crate::artifacts::vdi3805::{CurvePoint, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceCurvePoints {
    pub id: String,
    pub new_points: Vec<CurvePoint>,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ReplaceCurvePoints {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "curve-points", kind: "replace-curve-points", record: "ReplacedCurvePoints" };

    fn diff(&self, base: &Vdi3805Snapshot) -> <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace points for curve \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
