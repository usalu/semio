//! 🌉️ Shooting play app — store type aliases (the wasm-bindgen `ShootingArtifactVcs` VCS bridge
//! that used to live here was deleted — nothing ever built it for `wasm32-unknown-unknown`, no
//! engine entry, no `wasm` script target — see
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).

use crate::op::ShootingMutation;
use crate::ShootingSnapshot;

//#region 🔖️Store
pub type ShootingEnvelope = store::ArtifactEnvelope<ShootingSnapshot, ShootingMutation>;
pub type ShootingStore = store::ArtifactStore<ShootingSnapshot, ShootingMutation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
