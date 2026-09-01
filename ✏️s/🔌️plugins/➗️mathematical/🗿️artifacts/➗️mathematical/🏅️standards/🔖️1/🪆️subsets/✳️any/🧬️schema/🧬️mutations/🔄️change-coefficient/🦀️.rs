//! 🔄️ `change-coefficient` — sets a numeric leaf's value in the equation tree, addressed by
//! `EquationNodeLabel` (never a positional path — see `📸️snapshot/🦀️component.rs`'s `🔖️Equation`
//! region for why). First real mutation over `MathematicalSnapshot.equation`
//! (26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, wave M3a's `roots`
//! vertical slice).

use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::{EquationNodeKind, EquationNodeLabel};
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
/// 🪧 `numer`/`denom` are `Integer` decimal lexemes (never a bare `f64` — matches
/// `EquationNodeKind::{Integer,Rational}`'s own round-trip-exact representation); `denom == "1"`
/// is how a plain integer coefficient is expressed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeCoefficient {
    pub label: EquationNodeLabel,
    pub numer: String,
    pub denom: String,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for ChangeCoefficient {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "coefficient", kind: "change-coefficient", record: "ChangedCoefficient" };

    async fn diff(&self, base: &MathematicalSnapshot) -> protocol::MutationOutcome<<MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Change coefficient {} to {}/{}", self.label.0, self.numer, self.denom)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.label.0.to_string()]
    }
}
//#endregion 🔖️Payload
