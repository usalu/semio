//! 🔄️ `change-coefficient` — sets a numeric leaf's value in the equation tree, addressed by
//! `EquationNodeLabel` (never a positional path — see `📸️snapshot/🦀️.rs`'s `🔖️Equation`
//! region for why). First real mutation over `EquationSnapshot.equation`
//! (26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, wave M3a's `roots`
//! vertical slice).

use crate::standards::v1::subsets::any::schema::snapshot::EquationNodeLabel;
use crate::EquationSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
/// 🪧 `numer`/`denom` are `Integer` decimal lexemes (never a bare `f64` — matches
/// `EquationNodeKind::{Integer,Rational}`'s own round-trip-exact representation); `denom == "1"`
/// is how a plain integer coefficient is expressed.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeCoefficient {
    pub label: EquationNodeLabel,
    pub numer: String,
    pub denom: String,
}

impl<Op> protocol::MutationKind<EquationSnapshot, Op> for ChangeCoefficient
where
    Op: protocol::Mutation<EquationSnapshot, Diff = crate::EquationDiff> + From<ChangeCoefficient>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "coefficient", kind: "change-coefficient", record: "ChangedCoefficient" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change coefficient {} to {}/{}", self.label.0, self.numer, self.denom), &format!("Koeffizient {} auf {}/{} ändern", self.label.0, self.numer, self.denom))
    }
    fn target(&self) -> Vec<String> {
        vec![self.label.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EquationSnapshot, Op> for ChangeCoefficient
where
    Op: protocol::ColdMutation<EquationSnapshot, ReferenceDiff = crate::EquationDiff> + From<ChangeCoefficient>,
{
    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<EquationSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
