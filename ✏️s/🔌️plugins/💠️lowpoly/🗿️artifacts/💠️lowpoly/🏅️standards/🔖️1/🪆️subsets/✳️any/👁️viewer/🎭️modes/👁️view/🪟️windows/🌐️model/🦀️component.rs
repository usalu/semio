//! 🌐️ Lowpoly viewer — the Model window: a read-only world-3d mesh scene built with the shared
//! `MeshWindowKit` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6), from the
//! same artifact-level `LowpolySnapshot` the editor's own Model window renders — this file itself
//! imports nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright).
//! No selection, no engagement, no gumball/utility chrome: a viewer emits no mutations by construction
//! (`ViewEmit`). Object geometry renders the same fallback-box placeholder the editor's own
//! `world_meshes_json` falls back to while composed-child mesh resolution is unimplemented (ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave-3 gap, pre-existing, not introduced here) — real
//! parity with the editor's CURRENT behavior for that gap, not a regression.

use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_framework_plugin::{mesh_from_kind, world3d_camera_json, world3d_selection_json, WindowKindDefinition};
// 🚧️ SDK GAP: `MeshWindowKit`/`MeshView`/`WindowKit` (contract §2.6) are declared inside
// `semio_framework_plugin`'s `app` module but are not in the curated crate-root `pub use app::{ … };`
// re-export list (W0-F's Gap-1 fix added the surface traits/adapters, not the window kits) — only
// reachable through `app::` today. Flagged for W1-A in the migration report, same class of gap as
// `ArtifactEditor`/`Dialect` before W0-F closed it.
use semio_framework_plugin::app::{MeshView, MeshWindowKit, WindowKit};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = MeshWindowKit::KIND_ID;
pub const BODY_KEY: &str = MeshWindowKit::KIND_ID;
/// 👁️ Matches the editor's fallback mesh-kind literal ("box") — duplicated on purpose rather than
/// imported through the sibling editor module, which `policyViewerPurityBreaches` forbids outright.
const LOWPOLY_VIEW_FALLBACK_MESH_KIND: &str = "box";
/// 👁️ Read-only default camera — matches `LowpolyConfig::default()`'s own `world_camera_*` literals
/// (duplicated, not imported: `Config = NoConfig`, a viewer has no persisted per-session camera).
const LOWPOLY_VIEW_DEFAULT_CAMERA_POSITION: [f64; 3] = [18.0, -18.0, 12.0];
const LOWPOLY_VIEW_DEFAULT_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const LOWPOLY_VIEW_DEFAULT_CAMERA_FOV: f64 = 45.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::lowpoly::create_lowpoly_viewer`.
pub fn definition() -> WindowKindDefinition {
    MeshWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Read-only twin of the editor's `world_instances_json` — real per-object transform/label,
/// duplicated (not imported) per `policyViewerPurityBreaches`.
fn euler_degrees_to_quaternion(rotation: [f32; 3]) -> [f64; 4] {
    let to_rad = std::f32::consts::PI / 180.0;
    let (sx, cx) = (rotation[0] * to_rad * 0.5).sin_cos();
    let (sy, cy) = (rotation[1] * to_rad * 0.5).sin_cos();
    let (sz, cz) = (rotation[2] * to_rad * 0.5).sin_cos();
    [(sx * cy * cz + cx * sy * sz) as f64, (cx * sy * cz - sx * cy * sz) as f64, (cx * cy * sz + sx * sy * cz) as f64, (cx * cy * cz - sx * sy * sz) as f64]
}

fn world_instances_json(snapshot: &LowpolySnapshot) -> String {
    let instances: Vec<serde_json::Value> = snapshot
        .objects
        .iter()
        .map(|object| {
            let rotation = euler_degrees_to_quaternion(object.transform.rotation);
            serde_json::json!({
                "id": object.id,
                "meshId": LOWPOLY_VIEW_FALLBACK_MESH_KIND,
                "position": [object.transform.position[0] as f64, object.transform.position[1] as f64, object.transform.position[2] as f64],
                "rotation": rotation,
                "scale": [object.transform.scale[0] as f64, object.transform.scale[1] as f64, object.transform.scale[2] as f64],
                "label": object.name,
                "smoothShading": object.smooth_shading,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Pure `LowpolySnapshot -> UiNode` read: default camera (a viewer has no persisted per-session
/// camera — `Config = NoConfig`), no selection/gumball/engagement overlay, real object transforms read
/// straight off the document. Every object renders the same fallback-box placeholder mesh geometry
/// (real composed-child mesh resolution is an editor-side, engine-backed pipeline out of scope for a
/// pure read).
pub fn render(document: &LowpolySnapshot) -> UiNode {
    let meshes_json = serde_json::to_string(&[serde_json::json!({ "id": LOWPOLY_VIEW_FALLBACK_MESH_KIND, "data": mesh_from_kind(LOWPOLY_VIEW_FALLBACK_MESH_KIND) })]).unwrap_or_else(|_| "[]".into());
    let view = MeshView {
        camera_json: world3d_camera_json(LOWPOLY_VIEW_DEFAULT_CAMERA_POSITION, LOWPOLY_VIEW_DEFAULT_CAMERA_TARGET, LOWPOLY_VIEW_DEFAULT_CAMERA_FOV),
        meshes_json,
        instances_json: world_instances_json(document),
        selection_json: world3d_selection_json("rectangle", &[], None),
    };
    MeshWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_the_shared_mesh_window_kit() {
        let def = definition();
        assert_eq!(def.id, MeshWindowKit::KIND_ID);
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::lowpoly::schema::default_snapshot();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
