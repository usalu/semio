//! 🆕️ `create-tile` mutation payload — adds a new figure tile crop to `tiles`.

use crate::diff::PresentationDiff;
use crate::mutations::PresentationMutation;
use crate::{FigureTileDraft, PresentationSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// 🆕️ Inserts `tile` into `tiles` at `index` (FINAL-state, per the taxonomy's index-addressing
/// law — `tiles` is id-keyed, so `index` only determines append order, never lookup). Diff/inverse
/// delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-tile")]
pub struct CreateTile {
    pub index: usize,
    #[dsl(block)]
    pub tile: FigureTileDraft,
}

impl MutationKind<PresentationSnapshot, PresentationMutation> for CreateTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    fn diff(&self, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentationSnapshot) -> Result<Vec<PresentationMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create tile \"{}\"", self.tile.name), &format!("Kachel \"{}\" erstellen", self.tile.name))
    }

    fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.tile.id.clone()]
    }
}
//#endregion 🔹Payload
