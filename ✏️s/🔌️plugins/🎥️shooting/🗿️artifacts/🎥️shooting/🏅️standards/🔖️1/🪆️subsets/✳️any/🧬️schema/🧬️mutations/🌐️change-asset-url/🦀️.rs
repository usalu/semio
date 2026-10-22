//! 🔗 Shooting mutation payload — `ChangeAssetUrl`. Sets an asset's mesh `url`.

use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAssetUrl {
    pub id: String,
    pub new_url: String,
}

impl<Op> MutationKind<ShootingSnapshot, Op> for ChangeAssetUrl
where
    Op: protocol::Mutation<ShootingSnapshot, Diff = ShootingDiff> + From<ChangeAssetUrl>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "asset-url", kind: "change-asset-url", record: "ChangedAssetUrl" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change asset \"{}\" url", self.id), &format!("URL von Asset \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ShootingSnapshot, Op> for ChangeAssetUrl
where
    Op: protocol::ColdMutation<ShootingSnapshot, ReferenceDiff = ShootingDiff> + From<ChangeAssetUrl>,
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
