//! 📸 Shooting mutation payload — `CreateShot`. Brings a new shot into existence (append-only apply).

use crate::diff::ShootingDiff;
use crate::{ShootingShot, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CreateShot {
    pub shot: ShootingShot,
    pub index: Option<usize>,
}

impl<Op> MutationKind<ShootingSnapshot, Op> for CreateShot
where
    Op: protocol::Mutation<ShootingSnapshot, Diff = ShootingDiff> + From<CreateShot> + From<crate::standards::v1::subsets::any::schema::mutations::delete_shot::DeleteShot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "shot", kind: "create-shot", record: "CreatedShot" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create shot \"{}\"", self.shot.label), &format!("Aufnahme \"{}\" erstellen", self.shot.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.shot.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ShootingSnapshot, Op> for CreateShot
where
    Op: protocol::ColdMutation<ShootingSnapshot, ReferenceDiff = ShootingDiff> + From<CreateShot> + From<crate::standards::v1::subsets::any::schema::mutations::delete_shot::DeleteShot>,
{
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
