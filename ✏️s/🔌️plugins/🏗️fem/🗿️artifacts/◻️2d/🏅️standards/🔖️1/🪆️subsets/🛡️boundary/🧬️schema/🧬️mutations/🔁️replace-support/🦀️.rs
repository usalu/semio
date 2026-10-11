//! 🔁️ Fem2d mutation — `ReplaceSupport` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::{Fem2dSnapshot, FemSupport};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing support's payload (structured, no field-by-field editor
/// gesture exists for this entity — every real caller sets the whole record at once).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-support")]
pub struct ReplaceSupport {
    pub id: String,
    pub new_support: FemSupport,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for ReplaceSupport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "support", kind: "replace-support", record: "ReplacedSupport" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace support \"{}\"", self.id), &format!("Auflager \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
