//! 🧹️ `delete-drawing` — removes the entry matching `child_id` from `drawings`. Idempotent no-op
//! if absent; the inverse escrows the removed handle from BASE.

use crate::mutations::CadMutation;
use crate::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-drawing")]
pub struct DeleteDrawing {
    pub child_id: String,
}

impl MutationKind<CadSnapshot, CadMutation> for DeleteDrawing {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "drawing", kind: "delete-drawing", record: "DeletedDrawing" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete drawing child {}", self.child_id), &format!("Zeichnungs-Kind {} löschen", self.child_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}
//#endregion 🔖️Mutation
