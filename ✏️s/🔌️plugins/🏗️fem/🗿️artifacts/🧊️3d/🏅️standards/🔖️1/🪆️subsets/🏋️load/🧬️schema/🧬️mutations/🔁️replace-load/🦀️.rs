//! 🔁️ Fem3d mutation — `ReplaceLoad` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::{Fem3dSnapshot, FemLoad};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of one load inside an existing load case's `loads` member collection —
/// the in-place twin of the `add-load`/`remove-load` pair, addressed by the two ids that reach it.
///
/// 🎁️ `new_load` is boxed for the same reason `add-load`'s and `replace-element`'s payloads are:
/// the value codec implements `DslField` for `Box<T>`, not for a bare `DslEnum` field.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-load")]
pub struct ReplaceLoad {
    pub case_id: String,
    pub load_id: String,
    #[dsl(statements)]
    pub new_load: Box<FemLoad>,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceLoad {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "load", kind: "replace-load", record: "ReplacedLoad" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Replace load \"{}\" in case \"{}\"", self.load_id, self.case_id), &format!("Last \"{}\" in Fall \"{}\" ersetzen", self.load_id, self.case_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone(), self.load_id.clone()]
    }
}
//#endregion 🔖️Mutation
