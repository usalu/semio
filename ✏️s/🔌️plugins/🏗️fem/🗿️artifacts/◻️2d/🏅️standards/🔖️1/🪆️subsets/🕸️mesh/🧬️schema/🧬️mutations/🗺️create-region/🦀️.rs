//! 🌱️ Fem2d mutation — `CreateRegion` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::{Fem2dSnapshot, FemRegion};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemRegion`] meshed continuum region into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-region")]
pub struct CreateRegion {
    pub region: FemRegion,
    /// 📍 Zero-based insertion position among the siblings; `None` or past the end appends.
    pub index: Option<usize>,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateRegion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "region", kind: "create-region", record: "CreatedRegion" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create region \"{}\"", self.region.id), &format!("Bereich \"{}\" erstellen", self.region.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.region.id.clone()]
    }
}
//#endregion 🔖️Mutation
