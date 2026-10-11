//! 💡️ Lowpoly inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::{LowpolySnapshot, LowpolyTransform};
use framework_schema::ArtifactSchema;

use super::bounds::scene_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from a lowpoly snapshot. Today: object count and the 3d bounding box
/// across every object's `transform.position` (see `📦bounds/🦀️.rs`). A simple
/// whole-snapshot scalar — no `InferredField` caching, the object list is small.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly.inference")]
pub struct LowpolyInference {
    #[derived]
    pub object_count: usize,
    #[derived]
    pub bounds: Option<LowpolyBounds>,
}

impl protocol::Inference<LowpolySnapshot> for LowpolyInference {
    fn infer(snapshot: &LowpolySnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { object_count: snapshot.objects.len(), bounds: scene_bounds(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<LowpolySnapshot> for LowpolyInference {
    fn inference_schema_id() -> &'static str {
        "s.lowpoly.lowpoly.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.lowpoly.lowpoly.inference.objectCount", reads: &["objects"] }, protocol::InferenceFieldSpec { id: "s.lowpoly.lowpoly.inference.bounds", reads: &["objects"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.lowpoly.lowpoly.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `lowpoly_artifact_schema_descriptor`'s registration.
pub fn lowpoly_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.lowpoly.lowpoly.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::bounds::LowpolyBounds;
//#endregion 🔁️Re-exports

/// 🔄️ Semantic XYZ Euler degrees → quaternion `[x,y,z,w]`.

pub fn euler_degrees_to_quaternion(rotation: [f32; 3]) -> [f64; 4] {
    let to_rad = std::f64::consts::PI / 180.0;
    let (sx, cx) = (f64::from(rotation[0]) * to_rad * 0.5).sin_cos();
    let (sy, cy) = (f64::from(rotation[1]) * to_rad * 0.5).sin_cos();
    let (sz, cz) = (f64::from(rotation[2]) * to_rad * 0.5).sin_cos();
    [sx * cy * cz + cx * sy * sz, cx * sy * cz - sx * cy * sz, cx * cy * sz + sx * sy * cz, cx * cy * cz - sx * sy * sz]
}

/// 🧭️ Applies a unit quaternion to a local vector.
pub fn rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = q;
    let t = [2.0 * (y * v[2] - z * v[1]), 2.0 * (z * v[0] - x * v[2]), 2.0 * (x * v[1] - y * v[0])];
    [v[0] + w * t[0] + (y * t[2] - z * t[1]), v[1] + w * t[1] + (z * t[0] - x * t[2]), v[2] + w * t[2] + (x * t[1] - y * t[0])]
}

/// 🌍 Local position → world: scale, then rotation, then translation.
pub fn apply_transform(transform: &LowpolyTransform, local: [f32; 3]) -> [f64; 3] {
    let scaled = [f64::from(local[0] * transform.scale[0]), f64::from(local[1] * transform.scale[1]), f64::from(local[2] * transform.scale[2])];
    let rotated = rotate(euler_degrees_to_quaternion(transform.rotation), scaled);
    [rotated[0] + f64::from(transform.position[0]), rotated[1] + f64::from(transform.position[1]), rotated[2] + f64::from(transform.position[2])]
}


/// 📐 Unit normal of triangle `(a, b, c)` (zero for a degenerate triangle).
pub fn triangle_normal(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> [f64; 3] {
    let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if length < 1e-12 {
        [0.0; 3]
    } else {
        [n[0] / length, n[1] / length, n[2] / length]
    }
}
