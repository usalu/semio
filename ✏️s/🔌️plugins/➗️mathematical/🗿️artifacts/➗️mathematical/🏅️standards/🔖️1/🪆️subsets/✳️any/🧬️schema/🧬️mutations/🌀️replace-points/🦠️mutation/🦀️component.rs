//! 🌀️ `replace-points` — whole-value swap of the geometry playground's point cloud — the semantic
//! replacement for the old generic `SetGeometry`, used by gestures that load/paste an entire point
//! set (the app's `SetPoints` command) rather than editing one point.

use crate::artifacts::mathematical::{MathematicalMutation, MathematicalPoint, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplacePoints {
    pub points: Vec<MathematicalPoint>,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for ReplacePoints {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "points", kind: "replace-points", record: "ReplacedPoints" };

    fn diff(&self, base: &MathematicalSnapshot) -> protocol::MutationOutcome<<MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Replace points".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into()]
    }
}
//#endregion 🔖️Payload
