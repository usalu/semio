//! 🌉️ Process3d retained document-load bridge (the mounted operation registry below; the
//! wasm-bindgen `WasmBridge` submodule that used to sit between `🔖️MountedRegistry` and
//! `🧪️MountedLaws` was deleted — nothing ever built it for `wasm32-unknown-unknown`, no engine
//! entry, no `wasm` script target — see
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).

use crate::op::Process3dMutation;
use crate::Process3dSnapshot;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type Process3dEnvelope = ArtifactEnvelope<Process3dSnapshot, Process3dMutation>;
pub type Process3dStore = ArtifactStore<Process3dSnapshot, Process3dMutation>;
//#endregion 🔖️Store

//#region 🔖️MountedRegistry
#[cfg(test)]
#[path = "🧪️tests/🔬️mounted-registry/🦀️.rs"]
mod mounted_registry;
//#endregion 🧪️MountedLaws

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
