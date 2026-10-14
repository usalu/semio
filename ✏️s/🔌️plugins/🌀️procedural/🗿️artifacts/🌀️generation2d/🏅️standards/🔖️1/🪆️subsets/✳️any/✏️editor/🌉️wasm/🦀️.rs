//! 🌉️ Generation2d retained document-load bridge (the mounted operation registry below; the
//! wasm-bindgen `WasmBridge` submodule that used to sit between `🔖️MountedRegistry` and
//! `🧪️MountedLaws` was deleted, along with the `MountedLaws` test assertions that verified its
//! JS-facing method-name completeness — nothing ever built the bridge for `wasm32-unknown-unknown`,
//! no engine entry, no `wasm` script target — see
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).

use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type Generation2dEnvelope = ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>;
pub type Generation2dStore = ArtifactStore<Generation2dSnapshot, Generation2dMutation>;
//#endregion 🔖️Store

//#region 🔖️MountedRegistry
#[cfg(test)]
#[path = "🧪️tests/🔬️mounted-registry/🦀️.rs"]
mod mounted_registry;
