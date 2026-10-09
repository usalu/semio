//! 🧱️ The one place that names the typed glTF document of the `s.stdio.gltf` artifact and its GLB codec: a [`Snapshot`] in, binary glTF bytes out (and back for the tests).
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#binary-gltf-layout

pub use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::snapshot::{
    GltfAccessor as Accessor, GltfAccessorType as AccessorType, GltfAlphaMode as AlphaMode, GltfAsset as Asset, GltfBuffer as Buffer, GltfBufferView as BufferView, GltfComponentType as ComponentType, GltfDocument as Document, GltfJson as Json, GltfMaterial as Material, GltfMesh as Mesh,
    GltfNode as Node, GltfPbrMetallicRoughness as Pbr, GltfPrimitive as Primitive, GltfScene as Scene, GltfSnapshot as Snapshot, GltfSourceForm as SourceForm,
};
use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{decode_glb, encode_glb};

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
