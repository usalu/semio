//! 🌱️ Fem2d mutation — `CreateLoadCase` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::{Fem2dSnapshot, FemLoadCase};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemLoadCase`] into existence (empty or pre-seeded with one load — the
/// resolve-or-create gesture in `🎮️commands/📍️add-nodal-load` builds this when no matching case exists yet).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-load-case")]
pub struct CreateLoadCase {
    pub load_case: FemLoadCase,
    /// 📍 Zero-based insertion position among the siblings; `None` or past the end appends.
    pub index: Option<usize>,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateLoadCase {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "load-case", kind: "create-load-case", record: "CreatedLoadCase" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create load case \"{}\"", self.load_case.id), &format!("Lastfall \"{}\" erstellen", self.load_case.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.load_case.id.clone()]
    }
}
//#endregion 🔖️Mutation
