//! 🧱️ The one place that names the typed glTF document of the `s.stdio.gltf` artifact and its GLB codec: a [`Snapshot`] in, binary glTF bytes out (and back for the tests).
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#binary-gltf-layout

pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAccessor as Accessor;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType as AccessorType;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAlphaMode as AlphaMode;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAsset as Asset;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfBuffer as Buffer;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfBufferView as BufferView;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfComponentType as ComponentType;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfDocument as Document;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfJson as Json;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfMaterial as Material;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfMesh as Mesh;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfNode as Node;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfPbrMetallicRoughness as Pbr;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfPrimitive as Primitive;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfScene as Scene;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot as Snapshot;
pub use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSourceForm as SourceForm;
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
