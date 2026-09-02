//! 🖼️ `replace-source` mutation payload — whole-value swaps the shared figure source (a new image,
//! kind or page pick). `FigureTileSource` is a large structured sub-payload never set one field at a
//! time except for its `frame` (own `resize-source-frame` mutation), so this is `replace`, per the
//! taxonomy's rule 1 `update` exception boundary.

use crate::artifacts::presentation::{FigureTileSource, PresentationSnapshot};
use crate::artifacts::presentation::diff::PresentationDiff;
use crate::artifacts::presentation::mutations::PresentationMutation;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// 🖼️ Replaces `source` with `new_source`. Diff/inverse delegate to the sibling
/// `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-source")]
pub struct ReplaceSource {
    #[dsl(block)]
    pub new_source: FigureTileSource,
}

impl MutationKind<PresentationSnapshot, PresentationMutation> for ReplaceSource {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "source", kind: "replace-source", record: "ReplacedSource" };

    fn diff(&self, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace source with \"{}\"", self.new_source.src)
    }

    fn target(&self) -> Vec<String> {
        vec!["source".into()]
    }
}
//#endregion 🔹Payload
