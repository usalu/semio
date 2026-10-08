//! 🌱️ Fem2d mutation — `CreateSupport` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::{Fem2dSnapshot, FemSupport};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSupport`] into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-support")]
pub struct CreateSupport {
    pub support: FemSupport,
    /// 📍 Zero-based insertion position among the siblings; `None` or past the end appends.
    pub index: Option<usize>,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateSupport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "support", kind: "create-support", record: "CreatedSupport" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create support \"{}\"", self.support.id), &format!("Auflager \"{}\" erstellen", self.support.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.support.id.clone()]
    }
}
//#endregion 🔖️Mutation
