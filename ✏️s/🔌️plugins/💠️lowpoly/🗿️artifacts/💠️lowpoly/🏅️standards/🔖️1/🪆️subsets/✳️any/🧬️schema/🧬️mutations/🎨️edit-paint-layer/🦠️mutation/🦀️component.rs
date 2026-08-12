//! 🎨️ `edit-paint-layer` — writes a sparse set of RGBA pixel runs into a paint layer's authored
//! pixel-buffer content (one committed brush stroke or fill). Domain-native content edit — taxonomy's
//! `edit` verb ("replace an authored content body"), self-inverse via the overwritten bytes captured
//! from base. NOTE: `📓️taxonomy.md`'s "Domain verbs" section names a bespoke `paint-stroke` verb for
//! this exact gesture, but `paint-stroke` is not present in `command::APPROVED_VERBS` (framework-owned,
//! out of this ticket's writable scope — see this report's `notes`), so `edit` is used instead; a
//! future framework spine change could register `paint-stroke` and this triad would rename to match.

use crate::artifacts::lowpoly::mutations::PixelRun;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditPaintLayer {
    pub object_id: String,
    pub layer_index: usize,
    pub runs: Vec<PixelRun>,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for EditPaintLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "paint-layer", kind: "edit-paint-layer", record: "EditedPaintLayer" };

    fn diff(&self, base: &LowpolySnapshot) -> <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Paint on layer {} of object \"{}\"", self.layer_index, self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload
