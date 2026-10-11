//! 🗑️ Fem2d mutation — `DeleteSection` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🗑️ Removes an existing cross-section by id, capturing nothing itself (the removed payload is
/// recovered from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-section")]
pub struct DeleteSection {
    pub id: String,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for DeleteSection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "section", kind: "delete-section", record: "DeletedSection" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete section \"{}\"", self.id), &format!("Querschnitt \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
