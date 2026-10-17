//! ↩ Inverse constructor for `ChangeAssetUrl` — reconstructed from BASE state.

use super::ChangeAssetUrl;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ChangeAssetUrl, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_asset_url::ChangeAssetUrl>,
{
    Ok((|| {
    match base.assets.iter().find(|asset| asset.id == payload.id) {
        Some(asset) => vec![Op::from(ChangeAssetUrl { id: payload.id.clone(), new_url: asset.url.clone() })],
        None => Vec::new(),
    }

    })())
}
