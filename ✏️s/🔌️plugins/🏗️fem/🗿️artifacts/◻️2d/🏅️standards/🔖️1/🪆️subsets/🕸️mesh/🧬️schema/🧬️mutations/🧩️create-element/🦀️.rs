//! 🌱️ Fem2d mutation — `CreateElement` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::{element_id, Fem2dSnapshot, FemElement};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemElement`] (`Bar`/`Beam`) into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-element")]
pub struct CreateElement {
    #[dsl(statements)]
    pub element: Box<FemElement>,
    /// 📍 Zero-based insertion position among the siblings; `None` or past the end appends.
    pub index: Option<usize>,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "element", kind: "create-element", record: "CreatedElement" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create element \"{}\"", element_id(&self.element)), &format!("Element \"{}\" erstellen", element_id(&self.element)))
    }
    fn target(&self) -> Vec<String> {
        vec![element_id(&self.element).to_string()]
    }
}
//#endregion 🔖️Mutation
