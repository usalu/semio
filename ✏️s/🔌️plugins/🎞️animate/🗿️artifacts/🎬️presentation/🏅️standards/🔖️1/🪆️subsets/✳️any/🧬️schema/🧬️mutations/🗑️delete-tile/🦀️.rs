//! 🗑️ `delete-tile` mutation payload — removes a figure tile crop from `tiles` by id.

use crate::diff::PresentationDiff;
use crate::PresentationSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// 🗑️ Removes the `tiles` entry addressed by `id` (BASE-state, per the taxonomy's index/id
/// addressing law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-tile")]
pub struct DeleteTile {
    pub id: String,
}

impl<Op> MutationKind<PresentationSnapshot, Op> for DeleteTile
where
    Op: protocol::Mutation<PresentationSnapshot, Diff = PresentationDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "tile", kind: "delete-tile", record: "DeletedTile" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete tile \"{}\"", self.id), &format!("Kachel \"{}\" löschen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PresentationSnapshot, Op> for DeleteTile
where
    Op: protocol::ColdMutation<PresentationSnapshot, ReferenceDiff = PresentationDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile>,
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
