//! 🌉️ Generation3d retained document-load bridge (the mounted operation registry below; the
//! wasm-bindgen `WasmBridge` submodule that used to sit between `🔖️MountedRegistry` and
//! `🧪️MountedLaws` was deleted, along with the `MountedLaws` test assertions that verified its
//! JS-facing method-name completeness — nothing ever built the bridge for `wasm32-unknown-unknown`,
//! no engine entry, no `wasm` script target — see
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).

use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type Generation3dEnvelope = ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>;
pub type Generation3dStore = ArtifactStore<Generation3dSnapshot, Generation3dMutation>;
//#endregion 🔖️Store

//#region 🔖️MountedRegistry
#[cfg(test)]
#[path = "🧪️tests/🔬️mounted-registry/🦀️.rs"]
mod mounted_registry;
