//! 🎯️ Assigns a per-shot auto-fit override.

use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SetShotAutoFit {
    pub id: String,
    #[value(required)]
    pub auto_fit: Option<bool>,
}

impl<Op> MutationKind<ShootingSnapshot, Op> for SetShotAutoFit
where
    Op: protocol::Mutation<ShootingSnapshot, Diff = ShootingDiff> + From<SetShotAutoFit>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "shot-auto-fit", kind: "set-shot-auto-fit", record: "SetShotAutoFit" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Set auto-fit for shot \"{}\"", self.id), &format!("Automatische Einpassung für Aufnahme \"{}\" festlegen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ShootingSnapshot, Op> for SetShotAutoFit
where
    Op: protocol::ColdMutation<ShootingSnapshot, ReferenceDiff = ShootingDiff> + From<SetShotAutoFit>,
{
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
