//! 🎥️ Shooting viewer — the Scene window: a read-only world-3d render of the icon-studio scene, built
//! from the same artifact-level `crate::artifacts::shooting::schema` pure snapshot helpers the editor's
//! own Scene window (`✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene`) uses — this file itself imports
//! nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No
//! camera drag, no gumball, no shot-format chrome: a viewer has no utilities that edit and emits no
//! mutations by construction (`ViewEmit`). Camera/sun use hardcoded defaults — a viewer has no
//! persisted per-session camera (`Config = NoConfig`), an intentional first-pass simplification, not a
//! bug (mirrors the cad pilot's identical viewer-window simplification).

use crate::artifacts::shooting::schema::{active_shot, is_transparent_shooting_background};
use crate::artifacts::shooting::{shooting_asset_scale, ShootingAsset, ShootingCamera, ShootingShot, ShootingSnapshot};
use semio_framework_plugin::{build_world_3d_scene, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions, World3dScene, WorldSunConfig};
use std::collections::HashSet;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "shooting-view-scene";
pub const BODY_KEY: &str = "shooting.view.scene";
const SURFACE_ID: &str = "shooting.view.scene3d/scene";
/// 👁️ Read-only counterpart of the editor's `SHOOTING_PLAY_APP_ID` controller id — kept distinct so a
/// viewer session's world-3d controller can never be mistaken for an editor session's.
const SHOOTING_VIEW_CONTROLLER_ID: &str = "shooting-view";
/// 👁️ Matches the editor's `SHOOTING_FALLBACK_MESH_KIND` literal ("box") — duplicated on purpose rather
/// than imported through the sibling editor module, which `policyViewerPurityBreaches` forbids outright.
const SHOOTING_VIEW_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::shooting::create_shooting_viewer`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Scene", "Szene"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "shooting-scene".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn camera_json(camera: &ShootingCamera) -> String {
    let mut value = serde_json::json!({
        "position": camera.position,
        "target": camera.target,
        "fov": camera.fov,
        "zoom": camera.zoom,
        "projection": camera.projection.clone().unwrap_or_else(|| "perspective".into()),
    });
    if let (Some(object), Some(up)) = (value.as_object_mut(), camera.up) {
        object.insert("up".into(), serde_json::json!(up));
    }
    value.to_string()
}

async fn resolve_asset_mesh_url(asset: &ShootingAsset) -> Option<String> {
    if asset.url.is_empty() {
        None
    } else {
        Some(asset.url.clone())
    }
}

async fn collect_mesh_urls(snapshot: &ShootingSnapshot) -> Vec<String> {
    let mut urls = HashSet::new();
    for asset in &snapshot.assets {
        if let Some(url) = resolve_asset_mesh_url(asset) {
            urls.insert(url);
        }
    }
    urls.into_iter().collect()
}

/// 👁️ Read-only twin of the editor's `world_instances_json`: no selection/hover highlight at all (a
/// viewer has no interaction domain bound to this window), just each asset's real placed mesh.
async fn world_instances_json(snapshot: &ShootingSnapshot) -> String {
    let instances: Vec<serde_json::Value> = snapshot
        .assets
        .iter()
        .map(|asset| {
            let mesh_id = resolve_asset_mesh_url(asset).map_or_else(|| SHOOTING_VIEW_FALLBACK_MESH_KIND.into(), |url| world3d_mesh_id_from_url(&url));
            serde_json::json!({
                "id": asset.id,
                "meshId": mesh_id,
                "position": [
                    asset.origin.first().copied().unwrap_or(0.0),
                    asset.origin.get(1).copied().unwrap_or(0.0),
                    asset.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": shooting_asset_scale(asset),
                "label": asset.name,
                "color": "#6b7280",
                "selected": false,
                "hovered": false,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

async fn world_meshes_json(snapshot: &ShootingSnapshot) -> String {
    world3d_meshes_json_from_kinds_and_urls(&[SHOOTING_VIEW_FALLBACK_MESH_KIND.into()], &collect_mesh_urls(snapshot))
}

async fn shooting_environment_json(snapshot: &ShootingSnapshot) -> String {
    let scene = &snapshot.scene;
    let mut value = serde_json::json!({
        "ambient": { "intensity": scene.ambient.intensity, "color": scene.ambient.color },
        "sun": { "enabled": scene.sun.enabled, "azimuth": scene.sun.azimuth, "elevation": scene.sun.elevation, "intensity": scene.sun.intensity, "color": scene.sun.color },
        "shadow": { "enabled": scene.shadow.enabled, "opacity": scene.shadow.opacity, "softness": scene.shadow.softness },
        "material": { "color": scene.material.color, "metalness": scene.material.metalness, "roughness": scene.material.roughness, "emissive": scene.material.emissive, "emissiveIntensity": scene.material.emissive_intensity },
    });
    if let Some(object) = value.as_object_mut() {
        if !is_transparent_shooting_background(&scene.background) {
            object.insert("background".into(), serde_json::json!(scene.background));
        }
    }
    value.to_string()
}

async fn shooting_frame_json(shot: &ShootingShot) -> String {
    serde_json::json!({ "width": shot.width, "height": shot.height, "shape": shot.shape, "badge": true }).to_string()
}

/// 👁️ Pure `ShootingSnapshot -> UiNode` read: default camera (a viewer has no persisted per-session
/// camera), real scene lighting/asset placement/active-shot frame straight off the document.
pub async fn render(snapshot: &ShootingSnapshot) -> UiNode {
    let camera = ShootingCamera::default();
    build_world_3d_scene(
        SURFACE_ID,
        SHOOTING_VIEW_CONTROLLER_ID,
        World3dScene {
            environment_json: Some(shooting_environment_json(snapshot)),
            frame_json: active_shot(snapshot).map(shooting_frame_json),
            ..world3d_scene(camera_json(&camera), world_meshes_json(snapshot), world_instances_json(snapshot), world3d_selection_json("pick", &[], None), &WorldSunConfig::default())
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_the_world_3d_surface_and_body_key() {
        let def = definition();
        assert_eq!(def.body_key, BODY_KEY);
        assert!(matches!(def.surface_kind, SurfaceKind::World3d));
    }

    #[test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let node = render(&snapshot);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }
}
//#endregion 🧪️Tests
