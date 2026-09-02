//! 🌉️ Shooting play app — store type aliases (the wasm-bindgen `ShootingArtifactVcs` VCS bridge
//! that used to live here was deleted — nothing ever built it for `wasm32-unknown-unknown`, no
//! engine entry, no `wasm` script target — see
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).

use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 🔖️Store
pub type ShootingEnvelope = store::ArtifactEnvelope<ShootingSnapshot, ShootingMutation>;
pub type ShootingStore = store::ArtifactStore<ShootingSnapshot, ShootingMutation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn shooting_store_type_alias_constructs_from_an_empty_envelope() {
        let store = ShootingStore::new(store::create_document_envelope(crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::artifacts::shooting::empty_shooting_snapshot(), None));
        assert!(store.snapshot().expect("snapshot").assets.is_empty());
    }
}
//#endregion 🧪️Tests
