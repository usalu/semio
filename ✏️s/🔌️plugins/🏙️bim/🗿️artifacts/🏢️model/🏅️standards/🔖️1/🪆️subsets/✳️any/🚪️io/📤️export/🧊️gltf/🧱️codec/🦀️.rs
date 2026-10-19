//! 🧱️ The one place that names the typed glTF document of the `s.stdio.gltf` artifact and its GLB codec: a [`Snapshot`] in, binary glTF bytes out (and back for the tests).
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#binary-gltf-layout

use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot as Snapshot;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::decode_glb;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::encode_glb;

/// 📦️ The GLB bytes of a snapshot: the JSON chunk of its document and the first buffer as the binary chunk.
pub fn encode(snapshot: &Snapshot) -> Result<Vec<u8>, String> {
    encode_glb(snapshot)
}

/// 🔍️ The snapshot of GLB bytes.
pub fn decode(bytes: &[u8]) -> Result<Snapshot, String> {
    decode_glb(bytes)
}


