//! 🌱️ Fem3d mutation — `CreateSolid` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::{Fem3dSnapshot, FemSolid};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSolid`] meshed solid into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-solid")]
pub struct CreateSolid {
    pub solid: FemSolid,
    /// 📍 Zero-based insertion position among the siblings; `None` or past the end appends.
    pub index: Option<usize>,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateSolid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "solid", kind: "create-solid", record: "CreatedSolid" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create solid \"{}\"", self.solid.id), &format!("Körper \"{}\" erstellen", self.solid.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.solid.id.clone()]
    }
}
//#endregion 🔖️Mutation
