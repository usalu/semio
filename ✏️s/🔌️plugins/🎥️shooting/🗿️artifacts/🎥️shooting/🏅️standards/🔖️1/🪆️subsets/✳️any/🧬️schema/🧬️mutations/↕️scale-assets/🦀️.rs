//! ↕️ Shooting mutation payload — `ScaleAssets`. The bulk multiplicative-scale gesture. Multiplies every asset in `asset_ids`' current per-axis scale by `(sx, sy, sz)`.

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ScaleAssets {
    pub asset_ids: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ScaleAssets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "scale", entity: "assets", kind: "scale-assets", record: "ScaledAssets" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.asset_ids.len() {
            1 => semio_framework_ui_locale::LocalizedLabel::native("Scale 1 asset", "1 Asset skalieren"),
            count => semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {count} assets"), &format!("{count} Assets skalieren")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.asset_ids.clone()
    }
}
