//! 🖼️ `replace-source` mutation payload — whole-value swaps the shared figure source (a new image,
//! kind or page pick). `FigureTileSource` is a large structured sub-payload never set one field at a
//! time except for its `frame` (own `resize-source-frame` mutation), so this is `replace`, per the
//! taxonomy's rule 1 `update` exception boundary.

use crate::diff::PresentationDiff;
use crate::{FigureTileSource, PresentationSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// 🖼️ Replaces `source` with `new_source`. Diff/inverse delegate to the sibling
/// `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-source")]
pub struct ReplaceSource {
    #[dsl(block)]
    pub new_source: FigureTileSource,
}

impl<Op> MutationKind<PresentationSnapshot, Op> for ReplaceSource
where
    Op: protocol::Mutation<PresentationSnapshot, Diff = PresentationDiff> + From<ReplaceSource>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "source", kind: "replace-source", record: "ReplacedSource" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace source with \"{}\"", self.new_source.src), &format!("Quelle durch \"{}\" ersetzen", self.new_source.src))
    }

    fn target(&self) -> Vec<String> {
        vec!["source".into()]
    }
}

impl<Op> protocol::ColdMutationKind<PresentationSnapshot, Op> for ReplaceSource
where
    Op: protocol::ColdMutation<PresentationSnapshot, ReferenceDiff = PresentationDiff> + From<ReplaceSource>,
{
    fn diff(&self, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔹Payload
