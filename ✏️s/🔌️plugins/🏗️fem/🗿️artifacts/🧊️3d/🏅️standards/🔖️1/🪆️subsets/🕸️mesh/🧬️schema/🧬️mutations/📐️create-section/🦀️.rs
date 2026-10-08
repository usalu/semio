//! 🌱️ Fem3d mutation — `CreateSection` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::{Fem3dSnapshot, FemSection};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSection`] cross-section into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-section")]
pub struct CreateSection {
    pub section: FemSection,
    /// 📍 Zero-based insertion position among the siblings; `None` or past the end appends.
    pub index: Option<usize>,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateSection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "section", kind: "create-section", record: "CreatedSection" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create section \"{}\"", self.section.id), &format!("Querschnitt \"{}\" erstellen", self.section.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.section.id.clone()]
    }
}
//#endregion 🔖️Mutation
