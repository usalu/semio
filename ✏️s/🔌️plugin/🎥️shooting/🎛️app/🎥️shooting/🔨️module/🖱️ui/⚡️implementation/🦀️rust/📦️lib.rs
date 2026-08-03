//! 🖥️ Shooting app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! pilot — `ShootingPlayApp` is a unit struct; every former `ShootingPlayRuntime` field (selection,
//! hover, camera, …) now lives in `shooting_engine::ShootingConfig`, written via
//! `shooting_op::ShootingConfigOperation`s (real `backwards`, no ad hoc `InverseAction`); every action
//! dispatches through the single typed `shooting_protocol::ShootingCommand` channel via
//! `DocumentApp::handle`.

use semio_framework_plugin::{SurfaceKind, PanelGroup,
    app_labels, build_icon_render_scene, build_world_3d_scene, create_default_layout,
    localized_label_map, merge_world_selection_ids, LocaleLabels,
    ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_readonly_field,
    ui_declarative_sections_to_tree, ui_text, tree_item_with_action, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_scene,
    world3d_selection_json, ActionArgDef, ActionArgOption, ActionDefinition, Emit, ActionKind,
    App, ActionDescriptor, AppIo, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DslValue,
    DocumentView, ConfigView, HostEffect, IconRenderExportItem, IconRenderScene, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MeasureSelectItem, OsMediaCapability, OsMediaFormat, PanelTreeBuilder, ArtifactKindSpec, UtilityDefinition, UiFieldNode, UiInspectorFieldGroup,
    UiNode, UiPresence, UiTreeItemNode, WindowEngagement, WindowEngagementInput,
    WindowEngagementPossible, WindowEngagementStatus, WindowMeasure, World3dScene,
    WorldSunConfig, SET_ACTIVE_UTILITY_ACTION_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use shooting::{
    shooting_asset_scale, shooting_resolve_shot_camera, ShootingAsset, ShootingAssetPatch, ShootingCamera, ShootingFixture,
    ShootingSavedCamera, ShootingScenePatch, ShootingShot, ShootingShotPatch,
    SHOOTING_FIXTURE_SCHEMA,
};
use shooting_engine::{active_asset, active_shot, default_fixture, default_fixture_json, next_shooting_id, ShootingConfig};
use shooting_op::{ShootingConfigOperation, ShootingOperation};
use shooting_protocol::ShootingCommand;
use protocol::CollectionOperation;
use store::DocumentPack;
use std::collections::{HashMap, HashSet};
use serde_json::{json, Value};

//#region 🔖️Constants
const SHOOTING_PLAY_APP_ID: &str = "shooting-play";
const SHOOTING_PLAY_CONTROLLER_ID: &str = "shooting-play";
const SHOOTING_PLAY_SURFACE_SCENE: &str = "shooting.play.scene";
const SHOOTING_PLAY_SURFACE_ICON: &str = "shooting.play.icon";
const SHOOTING_PLAY_BODY_SCENE: &str = "shooting.play.scene";
const SHOOTING_PLAY_BODY_ICON: &str = "shooting.play.icon";
const SHOOTING_PLAY_BODY_DOCUMENT: &str = "shooting.play.document";
const SHOOTING_PLAY_BODY_CATALOGUE: &str = "shooting.play.catalogue";
const SHOOTING_PLAY_BODY_INSPECTION: &str = "shooting.play.inspection";
const SHOOTING_PLAY_WINDOW_SCENE: &str = "shooting-scene";
const SHOOTING_PLAY_WINDOW_ICON: &str = "shooting-icon";
const SHOOTING_EXAMPLE_DEFAULT_ID: &str = "base-icon";

const SHOOTING_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels`.
fn is_de_locale(cfg: &ShootingConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &ShootingConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn shooting_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: SHOOTING_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
    }
}

fn camera_json(camera: &ShootingCamera) -> String {
    let mut value = json!({
        "position": camera.position,
        "target": camera.target,
        "fov": camera.fov,
        "zoom": camera.zoom,
        "projection": camera.projection.clone().unwrap_or_else(|| "perspective".into()),
    });
    if let (Some(object), Some(up)) = (value.as_object_mut(), camera.up) {
        object.insert("up".into(), json!(up));
    }
    value.to_string()
}

/// 🎯️ B1: the typed-command counterpart of the pre-B1 `mesh_selection_ids` (JSON-args) — falls back
/// to the current config selection when the command carries no explicit ids.
fn mesh_selection_ids_typed(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() { fallback.to_vec() } else { ids.to_vec() }
}

fn resolve_asset_mesh_url(asset: &ShootingAsset) -> Option<String> {
    if asset.url.is_empty() {
        None
    } else {
        Some(asset.url.clone())
    }
}

fn collect_mesh_urls(fixture: &ShootingFixture) -> Vec<String> {
    let mut urls = HashSet::new();
    for asset in &fixture.assets {
        if let Some(url) = resolve_asset_mesh_url(asset) {
            urls.insert(url);
        }
    }
    urls.into_iter().collect()
}

fn world_instances_json(fixture: &ShootingFixture, cfg: &ShootingConfig) -> String {
    let instances: Vec<Value> = fixture
        .assets
        .iter()
        .map(|asset| {
            let active = fixture.active_asset_id == asset.id
                || (fixture.active_asset_id.is_empty() && fixture.assets.first().map(|entry| &entry.id) == Some(&asset.id));
            let selected = cfg.selected_asset_ids.contains(&asset.id) || active;
            let hovered = cfg.hovered_asset_id.as_deref() == Some(asset.id.as_str());
            let mesh_id = resolve_asset_mesh_url(asset)
                .map(|url| world3d_mesh_id_from_url(&url))
                .unwrap_or_else(|| SHOOTING_FALLBACK_MESH_KIND.into());
            json!({
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
                "color": if selected { "#9aa0ab" } else { "#6b7280" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(fixture: &ShootingFixture) -> String {
    world3d_meshes_json_from_kinds_and_urls(&[SHOOTING_FALLBACK_MESH_KIND.into()], &collect_mesh_urls(fixture))
}

fn world_selection_json(fixture: &ShootingFixture, cfg: &ShootingConfig) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json(
        &cfg.selection_method,
        &cfg.selected_asset_ids,
        cfg.hovered_asset_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("targets".into(), json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        object.insert("transformMode".into(), json!(cfg.active_utility_id));
        object.insert("activeObjectId".into(), json!(fixture.active_asset_id));
        object.insert("gumballActive".into(), json!(!cfg.selected_asset_ids.is_empty()));
        if let Some(target) = selection_centroid(fixture, &cfg.selected_asset_ids) {
            object.insert("gumballTarget".into(), json!(target));
        }
    }
    value.to_string()
}

fn selection_centroid(fixture: &ShootingFixture, selected_ids: &[String]) -> Option<[f64; 3]> {
    let selected: Vec<&ShootingAsset> = fixture.assets.iter().filter(|asset| selected_ids.contains(&asset.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let count = selected.len() as f64;
    let sum = selected.iter().fold([0.0f64; 3], |acc, asset| {
        [acc[0] + asset.origin[0], acc[1] + asset.origin[1], acc[2] + asset.origin[2]]
    });
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}

fn is_transparent_shooting_background(background: &str) -> bool {
    background.is_empty() || background == "transparent"
}

fn shooting_environment_json(fixture: &ShootingFixture) -> String {
    let scene = &fixture.scene;
    let mut value = json!({
        "ambient": { "intensity": scene.ambient.intensity, "color": scene.ambient.color },
        "sun": { "enabled": scene.sun.enabled, "azimuth": scene.sun.azimuth, "elevation": scene.sun.elevation, "intensity": scene.sun.intensity, "color": scene.sun.color },
        "shadow": { "enabled": scene.shadow.enabled, "opacity": scene.shadow.opacity, "softness": scene.shadow.softness },
        "material": { "color": scene.material.color, "metalness": scene.material.metalness, "roughness": scene.material.roughness, "emissive": scene.material.emissive, "emissiveIntensity": scene.material.emissive_intensity },
    });
    if let Some(object) = value.as_object_mut() {
        if !is_transparent_shooting_background(&scene.background) {
            object.insert("background".into(), json!(scene.background));
        }
    }
    value.to_string()
}

fn shooting_frame_json(shot: &ShootingShot) -> String {
    json!({ "width": shot.width, "height": shot.height, "shape": shot.shape, "badge": true }).to_string()
}

fn shooting_fit_json(cfg: &ShootingConfig) -> String {
    json!({ "enabled": cfg.center_model, "revision": cfg.fit_revision, "padding": 1.25 }).to_string()
}

fn shooting_icon_render_request_json(fixture: &ShootingFixture, shot: &ShootingShot, asset: &ShootingAsset, fallback_camera: &ShootingCamera) -> String {
    let camera = shooting_resolve_shot_camera(fixture, shot, fallback_camera);
    let scene = &fixture.scene;
    let mut camera_value = json!({
        "position": camera.position,
        "target": camera.target,
        "zoom": camera.zoom,
        "fov": camera.fov,
    });
    if let (Some(object), Some(up)) = (camera_value.as_object_mut(), camera.up) {
        object.insert("up".into(), json!(up));
    }
    let mut value = json!({
        "assetUrl": asset.url,
        "camera": camera_value,
        "lights": {
            "ambientIntensity": scene.ambient.intensity,
            "ambientColor": scene.ambient.color,
            "sunAzimuth": scene.sun.azimuth,
            "sunElevation": scene.sun.elevation,
            "sunIntensity": scene.sun.intensity,
            "sunColor": scene.sun.color,
        },
        "width": shot.width,
        "height": shot.height,
        "format": shot.format,
        "shape": if shot.shape == "ellipse" { "ellipse" } else { "rectangle" },
        "shadowEnabled": scene.shadow.enabled,
        "material": {
            "color": scene.material.color,
            "metalness": scene.material.metalness,
            "roughness": scene.material.roughness,
            "emissive": scene.material.emissive,
            "emissiveIntensity": scene.material.emissive_intensity,
        },
    });
    if let Some(object) = value.as_object_mut() {
        let background = shot.background.clone().unwrap_or_else(|| scene.background.clone());
        if !is_transparent_shooting_background(&background) {
            object.insert("background".into(), json!(background));
        }
    }
    value.to_string()
}

/// 🩹️ Builds the `ShootingShotPatch` for a `patchShot`/`patchShots`/`setActiveShot*` field write.
fn shot_patch_for_field(field: &str, value: &Value) -> Option<ShootingShotPatch> {
    match field {
        "label" => value.as_str().map(|v| ShootingShotPatch { label: Some(v.into()), ..Default::default() }),
        "width" => value.as_u64().map(|v| ShootingShotPatch { width: Some(v as u32), ..Default::default() }),
        "height" => value.as_u64().map(|v| ShootingShotPatch { height: Some(v as u32), ..Default::default() }),
        "format" => value.as_str().map(|v| ShootingShotPatch { format: Some(v.into()), ..Default::default() }),
        "shape" => value.as_str().map(|v| ShootingShotPatch { shape: Some(v.into()), ..Default::default() }),
        _ => None,
    }
}

/// 🩹️ Builds the `ShootingAssetPatch` for a `patchAsset`/`patchAssets` field write.
fn asset_patch_for_field(field: &str, value: &Value) -> Option<ShootingAssetPatch> {
    match field {
        "name" => value.as_str().map(|v| ShootingAssetPatch { name: Some(v.into()), ..Default::default() }),
        "url" => value.as_str().map(|v| ShootingAssetPatch { url: Some(v.into()), ..Default::default() }),
        _ => None,
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the shooting app; one field per label makes every locale combination compile-checked.
app_labels! {
    struct ShootingLabels {
        shots: &'static str = en: "Shots", de: "Aufnahmen";
        assets: &'static str = en: "Assets", de: "Objekte";
        add_shot: &'static str = en: "Add Shot", de: "Aufnahme hinzufügen";
        add_asset: &'static str = en: "Add Asset", de: "Objekt hinzufügen";
        svg_rectangle: &'static str = en: "SVG Rectangle", de: "SVG Rechteck";
        png_rectangle: &'static str = en: "PNG Rectangle", de: "PNG Rechteck";
        svg_ellipse: &'static str = en: "SVG Ellipse", de: "SVG Ellipse";
        png_ellipse: &'static str = en: "PNG Ellipse", de: "PNG Ellipse";
        glb_asset: &'static str = en: "GLB Asset", de: "GLB-Objekt";
        shot: &'static str = en: "Shot", de: "Aufnahme";
        asset: &'static str = en: "Asset", de: "Objekt";
        camera_label_placeholder: &'static str = en: "Camera label", de: "Kamera-Bezeichnung";
        load_camera: &'static str = en: "Load camera", de: "Kamera laden";
        shot_label_placeholder: &'static str = en: "Shot label", de: "Aufnahme-Bezeichnung";
        no_shot: &'static str = en: "No shot", de: "Keine Aufnahme";
        format_select_label: &'static str = en: "Format", de: "Format";
        shape_select_label: &'static str = en: "Shape", de: "Form";
        format_svg: &'static str = en: "SVG", de: "SVG";
        format_png: &'static str = en: "PNG", de: "PNG";
        shape_rectangle: &'static str = en: "Rectangle", de: "Rechteck";
        shape_ellipse: &'static str = en: "Ellipse", de: "Ellipse";
        window_scene: &'static str = en: "Scene", de: "Szene";
        window_icon: &'static str = en: "Icon", de: "Symbol";
        measure_center_model: &'static str = en: "Center Model", de: "Modell zentrieren";
        measure_sun: &'static str = en: "Sun", de: "Sonne";
        measure_sun_azimuth: &'static str = en: "Sun Azimuth", de: "Sonnenazimut";
        measure_sun_elevation: &'static str = en: "Sun Elevation", de: "Sonnenhöhe";
        measure_sun_intensity: &'static str = en: "Sun Intensity", de: "Sonnenintensität";
        measure_ambient: &'static str = en: "Ambient", de: "Umgebungslicht";
        measure_shadow: &'static str = en: "Shadow", de: "Schatten";
        measure_roughness: &'static str = en: "Roughness", de: "Rauheit";
        field_label: &'static str = en: "Label", de: "Bezeichnung";
        field_format: &'static str = en: "Format", de: "Format";
        field_shape: &'static str = en: "Shape", de: "Form";
        field_width: &'static str = en: "Width", de: "Breite";
        field_height: &'static str = en: "Height", de: "Höhe";
        field_name: &'static str = en: "Name", de: "Name";
        field_url: &'static str = en: "URL", de: "URL";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action/shell-action declared in `create_shooting_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn shooting_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("setFixtureJson", "Set Fixture Json", "Fixture-JSON festlegen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("setActiveShot", "Set Active Shot", "Aktive Aufnahme festlegen"),
        ("setActiveAsset", "Set Active Asset", "Aktives Objekt festlegen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setShotCamera", "Set Shot Camera", "Aufnahmekamera festlegen"),
        ("saveCamera", "Save Camera", "Kamera speichern"),
        ("loadSavedCamera", "Load Saved Camera", "Gespeicherte Kamera laden"),
        ("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen"),
        ("setSunElevation", "Set Sun Elevation", "Sonnenhöhe festlegen"),
        ("setSunIntensity", "Set Sun Intensity", "Sonnenintensität festlegen"),
        ("setAmbientIntensity", "Set Ambient Intensity", "Umgebungslichtintensität festlegen"),
        ("setMaterialRoughness", "Set Material Roughness", "Materialrauheit festlegen"),
        ("setShadowEnabled", "Set Shadow Enabled", "Schatten aktivieren"),
        ("toggleSun", "Toggle Sun", "Sonne umschalten"),
        ("setActiveShotLabel", "Set Active Shot Label", "Bezeichnung der aktiven Aufnahme festlegen"),
        ("setActiveShotFormat", "Set Active Shot Format", "Format der aktiven Aufnahme festlegen"),
        ("setActiveShotShape", "Set Active Shot Shape", "Form der aktiven Aufnahme festlegen"),
        ("patchShot", "Patch Shot", "Aufnahme aktualisieren"),
        ("patchShots", "Patch Shots", "Aufnahmen aktualisieren"),
        ("patchAsset", "Patch Asset", "Objekt aktualisieren"),
        ("patchAssets", "Patch Assets", "Objekte aktualisieren"),
        ("addShot", "Add Shot", "Aufnahme hinzufügen"),
        ("addAsset", "Add Asset", "Objekt hinzufügen"),
        ("importAsset", "Import Asset", "Objekt importieren"),
        ("resetFixture", "Reset Fixture", "Vorgabe zurücksetzen"),
        ("translateSelection", "Translate Selection", "Auswahl verschieben"),
        ("rotateSelection", "Rotate Selection", "Auswahl drehen"),
        ("scaleSelection", "Scale Selection", "Auswahl skalieren"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("setCameraDraftLabel", "Set Camera Draft Label", "Kamera-Entwurfsbezeichnung festlegen"),
        ("setCenterModel", "Set Center Model", "Modellzentrierung festlegen"),
        ("worldSelect", "World Select", "Welt auswählen"),
        ("worldHover", "World Hover", "Überfahren (Welt)"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("worldPick", "World Pick", "Welt-Auswahl (Pick)"),
        ("setSelectionMethod", "Set Selection Method", "Auswahlmethode festlegen"),
        ("worldPointerDown", "World Pointer Down", "Welt-Zeiger gedrückt"),
        ("worldPointerMove", "World Pointer Move", "Welt-Zeiger bewegt"),
        ("saveDownload", "Save Download", "Download speichern"),
        ("loadRequest", "Load Request", "Ladeanfrage"),
        ("importAssetRequest", "Import Asset Request", "Objekt-Importanfrage"),
        ("exportActiveShot", "Export Active Shot", "Aktive Aufnahme exportieren"),
        ("exportAllShots", "Export All Shots", "Alle Aufnahmen exportieren"),
    ])
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_shooting_app`.
fn shooting_utility_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("move", "Move", "Verschieben"),
        ("rotate", "Rotate", "Drehen"),
        ("scale", "Scale", "Skalieren"),
    ])
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
/// 🌳️ Layers an `icon_id` onto the SDK's `tree_item_with_action` skeleton — the SDK primitive's third
/// parameter is `description`, not an icon, so the shooting-specific icon assignment stays local.
fn tree_item_with_icon(id: impl Into<String>, label: impl Into<String>, icon_id: &str, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon_id.into()), menu: None,
    ..tree_item_with_action(id, label, None, action) }
}

fn build_document_tree(fixture: &ShootingFixture, labels: &ShootingLabels) -> UiNode {
    let shot_items: Vec<UiTreeItemNode> = fixture
        .shots
        .iter()
        .map(|shot| {
            tree_item_with_icon(
                format!("shooting-shot:{}", shot.id),
                shot.label.clone(),
                "camera",
                shooting_action("setSelection", Some(json!({ "shotIds": [shot.id], "assetIds": [] }))),
            )
        })
        .collect();
    let asset_items: Vec<UiTreeItemNode> = fixture
        .assets
        .iter()
        .map(|asset| {
            tree_item_with_icon(
                format!("shooting-asset:{}", asset.id),
                asset.name.clone(),
                "box",
                shooting_action("setSelection", Some(json!({ "shotIds": [], "assetIds": [asset.id] }))),
            )
        })
        .collect();
    PanelTreeBuilder::new("shooting-play-document")
        .section("shooting-play-document.shots", Some(labels.shots.into()), true, shot_items)
        .section("shooting-play-document.assets", Some(labels.assets.into()), true, asset_items)
        .build()
}

fn build_catalogue_tree(labels: &ShootingLabels) -> UiNode {
    let shot_items = vec![
        catalog_shot_item("svg-rect", labels.svg_rectangle, "svg", "rectangle"),
        catalog_shot_item("png-rect", labels.png_rectangle, "png", "rectangle"),
        catalog_shot_item("svg-ellipse", labels.svg_ellipse, "svg", "ellipse"),
        catalog_shot_item("png-ellipse", labels.png_ellipse, "png", "ellipse"),
    ];
    let asset_items = vec![tree_item_with_icon(
        "shooting-play-catalogue.asset.glb",
        labels.glb_asset,
        "box",
        shooting_action("addAsset", Some(json!({ "format": "glb" }))),
    )];
    PanelTreeBuilder::new("shooting-play-catalogue")
        .section("shooting-play-catalogue.shots", Some(labels.add_shot.into()), true, shot_items)
        .section("shooting-play-catalogue.assets", Some(labels.add_asset.into()), true, asset_items)
        .build()
}

fn catalog_shot_item(id: &str, label: &str, format: &str, shape: &str) -> UiTreeItemNode {
    tree_item_with_icon(
        format!("shooting-play-catalogue.{id}"),
        label,
        "camera",
        shooting_action("addShot", Some(json!({ "format": format, "shape": shape }))),
    )
}

fn build_inspector_tree(fixture: &ShootingFixture, cfg: &ShootingConfig, labels: &ShootingLabels) -> UiNode {
    if !cfg.selected_shot_ids.is_empty() {
        let shot_id = &cfg.selected_shot_ids[0];
        if let Some(shot) = fixture.shots.iter().find(|entry| &entry.id == shot_id) {
            return ui_inspector_groups_to_tree(&[shot_inspector_group(shot, labels)]);
        }
    }
    if !cfg.selected_asset_ids.is_empty() {
        let asset_id = &cfg.selected_asset_ids[0];
        if let Some(asset) = fixture.assets.iter().find(|entry| &entry.id == asset_id) {
            return ui_inspector_groups_to_tree(&[asset_inspector_group(asset, labels)]);
        }
    }
    if let Some(shot) = active_shot(fixture) {
        return ui_inspector_groups_to_tree(&[shot_inspector_group(shot, labels)]);
    }
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
        id: "shooting-play-inspector.empty".into(),
        label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
        default_open: Some(true),
        children: vec![
            ui_text(format!("Schema: {SHOOTING_FIXTURE_SCHEMA}")),
            ui_text(format!("Shots: {}", fixture.shots.len())),
            ui_text(format!("Assets: {}", fixture.assets.len())),
        ],
        presence: UiPresence::default(),
        menu: None,
    }])
}

fn shot_inspector_group(shot: &ShootingShot, labels: &ShootingLabels) -> UiInspectorFieldGroup {
    let width_mixed = ui_inspector_mixed_number(&[shot.width as f64]);
    let height_mixed = ui_inspector_mixed_number(&[shot.height as f64]);
    UiInspectorFieldGroup {
        id: "shooting-play-inspector.shot".into(),
        label: labels.shot.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "shooting-play-inspector.shot.label".into(),
                label: labels.field_label.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
                    id: "shooting-play-inspector.shot.label.input".into(),
                    input_kind: "text".into(),
                    value: shot.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "label" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            ui_inspector_readonly_field("shooting-play-inspector.shot.format", labels.field_format, &shot.format),
            ui_inspector_readonly_field("shooting-play-inspector.shot.shape", labels.field_shape, &shot.shape),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "shooting-play-inspector.shot.width".into(),
                label: labels.field_width.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
                    id: "shooting-play-inspector.shot.width.input".into(),
                    input_kind: "number".into(),
                    value: width_mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "width" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "shooting-play-inspector.shot.height".into(),
                label: labels.field_height.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
                    id: "shooting-play-inspector.shot.height.input".into(),
                    input_kind: "number".into(),
                    value: height_mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "height" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ],
    }
}

fn asset_inspector_group(asset: &ShootingAsset, labels: &ShootingLabels) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "shooting-play-inspector.asset".into(),
        label: labels.asset.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "shooting-play-inspector.asset.name".into(),
                label: labels.field_name.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
                    id: "shooting-play-inspector.asset.name.input".into(),
                    input_kind: "text".into(),
                    value: asset.name.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchAsset",
                        Some(json!({ "assetId": asset.id, "field": "name" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "shooting-play-inspector.asset.url".into(),
                label: labels.field_url.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
                    id: "shooting-play-inspector.asset.url.input".into(),
                    input_kind: "text".into(),
                    value: asset.url.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchAsset",
                        Some(json!({ "assetId": asset.id, "field": "url" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            ui_inspector_readonly_field("shooting-play-inspector.asset.format", labels.field_format, &asset.format),
        ],
    }
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_model_scene(fixture: &ShootingFixture, cfg: &ShootingConfig) -> UiNode {
    build_world_3d_scene(
        SHOOTING_PLAY_SURFACE_SCENE,
        SHOOTING_PLAY_APP_ID,
        World3dScene {
            environment_json: Some(shooting_environment_json(fixture)),
            frame_json: active_shot(fixture).map(shooting_frame_json),
            fit_json: Some(shooting_fit_json(cfg)),
            ..world3d_scene(
                camera_json(&cfg.camera),
                world_meshes_json(fixture),
                world_instances_json(fixture, cfg),
                world_selection_json(fixture, cfg),
                &WorldSunConfig::default(),
            )
        },
    )
}

fn render_icon_scene(fixture: &ShootingFixture, cfg: &ShootingConfig) -> UiNode {
    let (request_json, footer) = match (active_shot(fixture), active_asset(fixture)) {
        (Some(shot), Some(asset)) => (
            shooting_icon_render_request_json(fixture, shot, asset, &cfg.camera),
            Some(format!("{} · {}×{} · {}", shot.label, shot.width, shot.height, shot.format.to_uppercase())),
        ),
        _ => ("null".into(), None),
    };
    build_icon_render_scene(
        SHOOTING_PLAY_SURFACE_ICON,
        SHOOTING_PLAY_APP_ID,
        IconRenderScene {
            request_json,
            footer,
            frame_json: None,
        },
    )
}
//#endregion 🔖️Render

//#region 🔖️Utilities
fn shooting_model_measures(fixture: &ShootingFixture, labels: &ShootingLabels) -> Vec<WindowMeasure> {
    let scene = &fixture.scene;
    vec![
        WindowMeasure::Toggle {
            id: "shooting.measure.center-model".into(),
            icon_id: "focus".into(),
            label: Some(labels.measure_center_model.into()),
            pressed: true,
            text: None,
            on_change: shooting_action("setCenterModel", None),
        },
        WindowMeasure::Toggle {
            id: "shooting.measure.sun-enabled".into(),
            icon_id: "sun".into(),
            label: Some(labels.measure_sun.into()),
            pressed: scene.sun.enabled,
            text: None,
            on_change: shooting_action("toggleSun", None),
        },
        WindowMeasure::Slider {
            id: "shooting.measure.sun-azimuth".into(),
            label: Some(labels.measure_sun_azimuth.into()),
            value: scene.sun.azimuth,
            min: 0.0,
            max: 360.0,
            step: Some(1.0),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: shooting_action("setSunAzimuth", None),
            },
        WindowMeasure::Slider {
            id: "shooting.measure.sun-elevation".into(),
            label: Some(labels.measure_sun_elevation.into()),
            value: scene.sun.elevation,
            min: -10.0,
            max: 90.0,
            step: Some(1.0),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: shooting_action("setSunElevation", None),
            },
        WindowMeasure::Slider {
            id: "shooting.measure.sun-intensity".into(),
            label: Some(labels.measure_sun_intensity.into()),
            value: scene.sun.intensity,
            min: 0.0,
            max: 5.0,
            step: Some(0.1),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: shooting_action("setSunIntensity", None),
            },
        WindowMeasure::Slider {
            id: "shooting.measure.ambient".into(),
            label: Some(labels.measure_ambient.into()),
            value: scene.ambient.intensity,
            min: 0.0,
            max: 3.0,
            step: Some(0.05),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: shooting_action("setAmbientIntensity", None),
            },
        WindowMeasure::Toggle {
            id: "shooting.measure.shadow".into(),
            icon_id: "sun".into(),
            label: Some(labels.measure_shadow.into()),
            pressed: scene.shadow.enabled,
            text: None,
            on_change: shooting_action("setShadowEnabled", None),
        },
        WindowMeasure::Slider {
            id: "shooting.measure.roughness".into(),
            label: Some(labels.measure_roughness.into()),
            value: scene.material.roughness,
            min: 0.0,
            max: 1.0,
            step: Some(0.05),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: shooting_action("setMaterialRoughness", None),
            },
    ]
}

fn shooting_icon_measures(fixture: &ShootingFixture, labels: &ShootingLabels) -> Vec<WindowMeasure> {
    let shot = active_shot(fixture);
    vec![
        WindowMeasure::Select {
            id: "shooting.measure.shot".into(),
            label: Some(labels.shot.into()),
            value: shot.map(|entry| entry.id.clone()).unwrap_or_default(),
            items: fixture
                .shots
                .iter()
                .map(|entry| MeasureSelectItem {
                    id: format!("shooting.measure.shot.{}", entry.id),
                    value: entry.id.clone(),
                    label: entry.label.clone(),
                })
                .collect(),
            on_change: shooting_action("setActiveShot", None),
        },
        WindowMeasure::Select {
            id: "shooting.measure.format".into(),
            label: Some(labels.format_select_label.into()),
            value: shot.map(|entry| entry.format.clone()).unwrap_or_else(|| "svg".into()),
            items: vec![
                MeasureSelectItem { id: "shooting.measure.format.svg".into(), value: "svg".into(), label: labels.format_svg.into() },
                MeasureSelectItem { id: "shooting.measure.format.png".into(), value: "png".into(), label: labels.format_png.into() },
            ],
            on_change: shooting_action("setActiveShotFormat", None),
        },
        WindowMeasure::Select {
            id: "shooting.measure.shape".into(),
            label: Some(labels.shape_select_label.into()),
            value: shot.map(|entry| entry.shape.clone()).unwrap_or_else(|| "rectangle".into()),
            items: vec![
                MeasureSelectItem { id: "shooting.measure.shape.rectangle".into(), value: "rectangle".into(), label: labels.shape_rectangle.into() },
                MeasureSelectItem { id: "shooting.measure.shape.ellipse".into(), value: "ellipse".into(), label: labels.shape_ellipse.into() },
            ],
            on_change: shooting_action("setActiveShotShape", None),
        },
    ]
}

fn shooting_model_engagement(fixture: &ShootingFixture, cfg: &ShootingConfig, labels: &ShootingLabels) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("shooting.camera-draft".into()),
            value: Some(cfg.camera_draft_label.clone()),
            placeholder: Some(labels.camera_label_placeholder.into()),
            disabled: None,
            on_change: Some(shooting_action("setCameraDraftLabel", None)),
            on_submit: Some(shooting_action("saveCamera", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "shooting.status.model".into(),
            text: format!("{} assets · {} shots", fixture.assets.len(), fixture.shots.len()),
        }]),
        possible_engagements: Some(
            fixture
                .saved_cameras
                .iter()
                .map(|saved| WindowEngagementPossible {
                    id: format!("shooting.camera.{}", saved.id),
                    label: saved.label.clone(),
                    detail: Some(labels.load_camera.into()),
                    action: Some(shooting_action("loadSavedCamera", Some(json!({ "id": saved.id })))),
                })
                .collect(),
        ),
    }
}

fn shooting_icon_engagement(fixture: &ShootingFixture, labels: &ShootingLabels) -> WindowEngagement {
    let shot = active_shot(fixture);
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("shooting.shot-label".into()),
            value: shot.map(|entry| entry.label.clone()),
            placeholder: Some(labels.shot_label_placeholder.into()),
            disabled: Some(shot.is_none()),
            on_change: Some(shooting_action("setActiveShotLabel", None)),
            on_submit: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "shooting.status.icon".into(),
            text: shot
                .map(|entry| format!("{}×{} {}", entry.width, entry.height, entry.format.to_uppercase()))
                .unwrap_or_else(|| labels.no_shot.into()),
        }]),
        possible_engagements: None,
    }
}

//#endregion 🔖️Utilities

//#region 🔖️ShootingPlayApp
/// 🧪️ B1: unit struct — every former `ShootingPlayRuntime`/`self.config` field now lives in
/// `shooting_engine::ShootingConfig` (see `DocumentApp::Config`), written through
/// `shooting_op::ShootingConfigOperation`s.
#[derive(Default)]
pub struct ShootingPlayApp;

impl DocumentApp for ShootingPlayApp {
    type Projection = ShootingFixture;
    type Operation = ShootingOperation;
    type Config = ShootingConfig;
    type ConfigOperation = ShootingConfigOperation;
    type Command = ShootingCommand;

    fn app_id(&self) -> &str {
        SHOOTING_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        SHOOTING_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> ShootingFixture {
        default_fixture()
    }

    fn io(&self) -> Option<AppIo> {
        Some(shooting_engine::shooting_io())
    }

    /// 🎞️ `photos:out` (see `shooting_engine::shooting_photo_media`) plus the inherited `document:out`
    /// default (the pack of `doc.projection`, replicated inline — overriding `export_media` shadows the
    /// trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, ShootingFixture>) -> Result<Media, MediaError> {
        match port {
            "photos:out" => shooting_engine::shooting_photo_media(doc.projection),
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media {
                    media_type,
                    payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn whole_document_operation(&self, projection: ShootingFixture) -> Option<ShootingOperation> {
        Some(ShootingOperation::SetFixture { fixture: projection })
    }

    /// 🏷️ Maps each `ShootingCommand` variant back to the action id it was declared under in
    /// `create_shooting_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &ShootingCommand) -> &str {
        match command {
            ShootingCommand::SetFixtureJson { .. } => "setFixtureJson",
            ShootingCommand::SetActiveExample { .. } => "setActiveExample",
            ShootingCommand::SetActiveShot { .. } => "setActiveShot",
            ShootingCommand::SetActiveAsset { .. } => "setActiveAsset",
            ShootingCommand::SetShotCamera { .. } => "setShotCamera",
            ShootingCommand::SaveCamera => "saveCamera",
            ShootingCommand::SetSunAzimuth { .. } => "setSunAzimuth",
            ShootingCommand::SetSunElevation { .. } => "setSunElevation",
            ShootingCommand::SetSunIntensity { .. } => "setSunIntensity",
            ShootingCommand::SetAmbientIntensity { .. } => "setAmbientIntensity",
            ShootingCommand::SetMaterialRoughness { .. } => "setMaterialRoughness",
            ShootingCommand::SetShadowEnabled { .. } => "setShadowEnabled",
            ShootingCommand::ToggleSun { .. } => "toggleSun",
            ShootingCommand::SetActiveShotLabel { .. } => "setActiveShotLabel",
            ShootingCommand::SetActiveShotFormat { .. } => "setActiveShotFormat",
            ShootingCommand::SetActiveShotShape { .. } => "setActiveShotShape",
            ShootingCommand::PatchShots { .. } => "patchShots",
            ShootingCommand::PatchAssets { .. } => "patchAssets",
            ShootingCommand::AddShot { .. } => "addShot",
            ShootingCommand::AddAsset { .. } => "addAsset",
            ShootingCommand::ImportAsset { .. } => "importAsset",
            ShootingCommand::ResetFixture => "resetFixture",
            ShootingCommand::TranslateSelection { .. } => "translateSelection",
            ShootingCommand::RotateSelection { .. } => "rotateSelection",
            ShootingCommand::ScaleSelection { .. } => "scaleSelection",
            ShootingCommand::SetCamera { .. } => "setCamera",
            ShootingCommand::LoadSavedCamera { .. } => "loadSavedCamera",
            ShootingCommand::SetCameraDraftLabel { .. } => "setCameraDraftLabel",
            ShootingCommand::SetCenterModel { .. } => "setCenterModel",
            ShootingCommand::SetActiveUtility { .. } => SET_ACTIVE_UTILITY_ACTION_ID,
            ShootingCommand::SetLocale { .. } => "setLocale",
            ShootingCommand::SetSelection { .. } => "setSelection",
            ShootingCommand::SetSelectionMethod { .. } => "setSelectionMethod",
            ShootingCommand::WorldSelect { .. } => "worldSelect",
            ShootingCommand::SetHover { .. } => "setHover",
            ShootingCommand::WorldPick { .. } => "worldPick",
            ShootingCommand::WorldPointerDown => "worldPointerDown",
            ShootingCommand::WorldPointerMove => "worldPointerMove",
            ShootingCommand::SaveDownload => "saveDownload",
            ShootingCommand::LoadRequest => "loadRequest",
            ShootingCommand::ImportAssetRequest => "importAssetRequest",
            ShootingCommand::ExportShots { all } => if *all { "exportAllShots" } else { "exportActiveShot" },
        }
    }

    fn handle(
        &self,
        command: &ShootingCommand,
        doc: &DocumentView<'_, ShootingFixture>,
        cfg: &ConfigView<'_, ShootingConfig>,
    ) -> Emit<ShootingOperation, ShootingConfigOperation> {
        let fixture = doc.projection;
        let config = cfg.projection;
        match command {
            ShootingCommand::SetFixtureJson { json } => match serde_json::from_str::<ShootingFixture>(json) {
                Ok(fixture) => Emit::operations(vec![ShootingOperation::SetFixture { fixture }]),
                Err(_) => Emit::default(),
            },
            ShootingCommand::SetActiveExample { example_id } => {
                let next = if example_id.is_empty() {
                    Some(shooting::empty_shooting_fixture())
                } else if example_id == SHOOTING_EXAMPLE_DEFAULT_ID || example_id == "base" {
                    Some(default_fixture())
                } else {
                    None
                };
                match next {
                    Some(fixture) => Emit::operations(vec![ShootingOperation::SetFixture { fixture }]),
                    None => Emit::default(),
                }
            }
            ShootingCommand::SetActiveShot { shot_id } => match shot_id.as_deref().filter(|id| !id.is_empty()) {
                Some(id) => Emit::operations(vec![ShootingOperation::SetActiveShot { shot_id: Some(id.into()) }]),
                None => Emit::default(),
            },
            ShootingCommand::SetActiveAsset { asset_id } => match asset_id.as_deref().filter(|id| !id.is_empty()) {
                Some(id) => Emit {
                    document_operations: vec![ShootingOperation::SetActiveAsset { asset_id: Some(id.into()) }],
                    config_operations: vec![ShootingConfigOperation::SetFitRevision { value: config.fit_revision + 1 }],
                    ..Default::default()
                },
                None => Emit::default(),
            },
            // 🎥️ Config-only: the free/live viewport camera never touches the document.
            ShootingCommand::SetCamera { camera } => Emit::config(vec![ShootingConfigOperation::SetCamera { camera: camera.clone() }]),
            ShootingCommand::SetShotCamera { shot_id, camera } => {
                // 🎥️ Deliberately overwrites shot_id's *saved* camera with the given pose — a real,
                // undoable document edit. A no-op when that shot has no saved camera (the free/live
                // camera is `SetCamera`'s job, above, and never reaches this operation).
                Emit::operations(vec![ShootingOperation::SetShotCamera { shot_id: shot_id.clone(), camera: camera.clone() }])
            }
            ShootingCommand::SaveCamera => {
                let draft = config.camera_draft_label.trim().to_string();
                let label = if draft.is_empty() { format!("Camera {}", fixture.saved_cameras.len() + 1) } else { draft };
                let saved_camera = ShootingSavedCamera { id: next_shooting_id("camera"), label, camera: config.camera.clone() };
                Emit {
                    document_operations: vec![ShootingOperation::SavedCameras(CollectionOperation::Add {
                        id: saved_camera.id.clone(),
                        item: saved_camera,
                        at: fixture.saved_cameras.len(),
                    })],
                    config_operations: vec![ShootingConfigOperation::SetCameraDraftLabel { value: String::new() }],
                    ..Default::default()
                }
            }
            ShootingCommand::LoadSavedCamera { id } => {
                // 🎥️ Config-only: loads a saved preset into the live viewport — never mutates the
                // saved camera itself, so this never touches the document either.
                match fixture.saved_cameras.iter().find(|entry| &entry.id == id) {
                    Some(saved) => Emit::config(vec![ShootingConfigOperation::SetCamera { camera: saved.camera.clone() }]),
                    None => Emit::default(),
                }
            }
            ShootingCommand::SetCameraDraftLabel { value } => Emit::config(vec![ShootingConfigOperation::SetCameraDraftLabel { value: value.clone() }]),
            ShootingCommand::SetCenterModel { pressed } => {
                let next = pressed.unwrap_or(!config.center_model);
                let mut config_operations = vec![ShootingConfigOperation::SetCenterModel { value: next }];
                if next && !config.center_model {
                    config_operations.push(ShootingConfigOperation::SetFitRevision { value: config.fit_revision + 1 });
                }
                Emit::config(config_operations)
            }
            ShootingCommand::SetActiveUtility { utility_id } => Emit::config(vec![
                ShootingConfigOperation::SetActiveUtility { utility_id: utility_id.clone() },
                ShootingConfigOperation::SetHoveredAsset { asset_id: None },
            ]),
            ShootingCommand::SetLocale { value } => Emit::config(vec![ShootingConfigOperation::SetLocale { value: value.clone() }]),
            ShootingCommand::SetSunAzimuth { value } => {
                Emit::operations(vec![ShootingOperation::PatchScene { patch: ShootingScenePatch { sun_azimuth: Some(*value), ..Default::default() } }])
            }
            ShootingCommand::SetSunElevation { value } => {
                Emit::operations(vec![ShootingOperation::PatchScene { patch: ShootingScenePatch { sun_elevation: Some(*value), ..Default::default() } }])
            }
            ShootingCommand::SetSunIntensity { value } => {
                Emit::operations(vec![ShootingOperation::PatchScene { patch: ShootingScenePatch { sun_intensity: Some(*value), ..Default::default() } }])
            }
            ShootingCommand::SetAmbientIntensity { value } => {
                Emit::operations(vec![ShootingOperation::PatchScene { patch: ShootingScenePatch { ambient_intensity: Some(*value), ..Default::default() } }])
            }
            ShootingCommand::SetMaterialRoughness { value } => {
                Emit::operations(vec![ShootingOperation::PatchScene { patch: ShootingScenePatch { material_roughness: Some(*value), ..Default::default() } }])
            }
            ShootingCommand::SetShadowEnabled { value } => {
                Emit::operations(vec![ShootingOperation::PatchScene { patch: ShootingScenePatch { shadow_enabled: Some(*value), ..Default::default() } }])
            }
            ShootingCommand::ToggleSun { value } => {
                Emit::operations(vec![ShootingOperation::PatchScene { patch: ShootingScenePatch { sun_enabled: Some(*value), ..Default::default() } }])
            }
            ShootingCommand::SetActiveShotLabel { value } => match active_shot(fixture).map(|shot| shot.id.clone()) {
                Some(shot_id) => Emit::operations(vec![ShootingOperation::Shots(CollectionOperation::Patch {
                    id: shot_id,
                    patch: ShootingShotPatch { label: Some(value.clone()), ..Default::default() },
                })]),
                None => Emit::default(),
            },
            ShootingCommand::SetActiveShotFormat { value } => match (active_shot(fixture).map(|shot| shot.id.clone()), shot_patch_for_field("format", &json!(value))) {
                (Some(shot_id), Some(patch)) => Emit::operations(vec![ShootingOperation::Shots(CollectionOperation::Patch { id: shot_id, patch })]),
                _ => Emit::default(),
            },
            ShootingCommand::SetActiveShotShape { value } => match (active_shot(fixture).map(|shot| shot.id.clone()), shot_patch_for_field("shape", &json!(value))) {
                (Some(shot_id), Some(patch)) => Emit::operations(vec![ShootingOperation::Shots(CollectionOperation::Patch { id: shot_id, patch })]),
                _ => Emit::default(),
            },
            ShootingCommand::ResetFixture => Emit::operations(vec![ShootingOperation::SetFixture { fixture: default_fixture() }]),
            ShootingCommand::SaveDownload => match serde_json::to_string_pretty(fixture) {
                Ok(fixture_text) => Emit::effect(HostEffect::DownloadMediaExport {
                    filename: "shooting.fixture.ops".into(),
                    mime_type: "text/plain".into(),
                    data: fixture_text,
                    encoding: None,
                }),
                Err(_) => Emit::default(),
            },
            ShootingCommand::LoadRequest => Emit::effect(HostEffect::RequestFileOpen {
                accept: ".ops,.dsl,.spk,application/octet-stream,text/plain".into(),
                read_as: None,
                import_action: "setFixtureJson".into(),
                multiple: false,
            }),
            ShootingCommand::ImportAssetRequest => Emit::effect(HostEffect::RequestFileOpen {
                accept: ".glb,model/gltf-binary".into(),
                read_as: Some("dataUrl".into()),
                import_action: "importAsset".into(),
                multiple: false,
            }),
            ShootingCommand::ImportAsset { payload, name } => {
                let id = next_shooting_id("asset");
                let resolved_name = name
                    .as_deref()
                    .map(|name| name.trim_end_matches(".glb").to_string())
                    .filter(|name| !name.is_empty())
                    .unwrap_or_else(|| format!("Asset {}", fixture.assets.len() + 1));
                let asset = ShootingAsset {
                    id: id.clone(),
                    name: resolved_name,
                    url: payload.clone(),
                    format: "glb".into(),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                };
                Emit {
                    document_operations: vec![
                        ShootingOperation::Assets(CollectionOperation::Add { id: id.clone(), item: asset, at: fixture.assets.len() }),
                        ShootingOperation::SetActiveAsset { asset_id: Some(id.clone()) },
                    ],
                    config_operations: vec![
                        ShootingConfigOperation::SetSelection { shot_ids: Vec::new(), asset_ids: vec![id] },
                        ShootingConfigOperation::SetFitRevision { value: config.fit_revision + 1 },
                    ],
                    ..Default::default()
                }
            }
            ShootingCommand::ExportShots { all } => {
                if let Some(asset) = active_asset(fixture) {
                    let shots: Vec<&ShootingShot> = if *all { fixture.shots.iter().collect() } else { active_shot(fixture).into_iter().collect() };
                    let items: Vec<IconRenderExportItem> = shots
                        .iter()
                        .map(|shot| IconRenderExportItem {
                            filename: format!("{}.{}", shot.id, if shot.format == "png" { "png" } else { "svg" }),
                            request: serde_json::from_str::<Value>(&shooting_icon_render_request_json(fixture, shot, asset, &config.camera))
                                .ok()
                                .and_then(|value| semio_framework_plugin::to_dsl_value(&value).ok())
                                .unwrap_or(DslValue::Null),
                        })
                        .collect();
                    if !items.is_empty() {
                        return Emit::effect(HostEffect::IconRenderExport { items });
                    }
                }
                Emit::default()
            }
            ShootingCommand::TranslateSelection { asset_ids, dx, dy, dz } => {
                let ids = mesh_selection_ids_typed(asset_ids, &config.selected_asset_ids);
                if ids.is_empty() { Emit::default() } else { Emit::amend(vec![ShootingOperation::TranslateAssets { asset_ids: ids, dx: *dx, dy: *dy, dz: *dz }], "gumball-translate") }
            }
            ShootingCommand::RotateSelection { asset_ids, ax, ay, az, angle } => {
                let ids = mesh_selection_ids_typed(asset_ids, &config.selected_asset_ids);
                if ids.is_empty() { Emit::default() } else { Emit::amend(vec![ShootingOperation::RotateAssets { asset_ids: ids, ax: *ax, ay: *ay, az: *az, angle: *angle }], "gumball-rotate") }
            }
            ShootingCommand::ScaleSelection { asset_ids, sx, sy, sz } => {
                let ids = mesh_selection_ids_typed(asset_ids, &config.selected_asset_ids);
                if ids.is_empty() { Emit::default() } else { Emit::amend(vec![ShootingOperation::ScaleAssets { asset_ids: ids, sx: *sx, sy: *sy, sz: *sz }], "gumball-scale") }
            }
            ShootingCommand::PatchShots { shot_ids, field, value } => match shot_patch_for_field(field, &json!(value)) {
                Some(patch) if !shot_ids.is_empty() => Emit::operations(
                    shot_ids.iter().cloned().map(|id| ShootingOperation::Shots(CollectionOperation::Patch { id, patch: patch.clone() })).collect(),
                ),
                _ => Emit::default(),
            },
            ShootingCommand::PatchAssets { asset_ids, field, value } => match asset_patch_for_field(field, &json!(value)) {
                Some(patch) if !asset_ids.is_empty() => Emit::operations(
                    asset_ids.iter().cloned().map(|id| ShootingOperation::Assets(CollectionOperation::Patch { id, patch: patch.clone() })).collect(),
                ),
                _ => Emit::default(),
            },
            ShootingCommand::AddShot { format, shape } => {
                let id = next_shooting_id("shot");
                let shot = ShootingShot {
                    id: id.clone(),
                    label: format!("Shot {}", fixture.shots.len() + 1),
                    width: 256,
                    height: 256,
                    format: format.clone(),
                    shape: shape.clone(),
                    background: None,
                    camera_id: None,
                };
                Emit {
                    document_operations: vec![
                        ShootingOperation::Shots(CollectionOperation::Add { id: id.clone(), item: shot, at: fixture.shots.len() }),
                        ShootingOperation::SetActiveShot { shot_id: Some(id.clone()) },
                    ],
                    config_operations: vec![ShootingConfigOperation::SetSelection { shot_ids: vec![id], asset_ids: Vec::new() }],
                    ..Default::default()
                }
            }
            ShootingCommand::AddAsset { format } => {
                let id = next_shooting_id("asset");
                let asset = ShootingAsset {
                    id: id.clone(),
                    name: format!("Asset {}", fixture.assets.len() + 1),
                    url: format!("/mesh/placeholder.{format}"),
                    format: format.clone(),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                };
                Emit {
                    document_operations: vec![
                        ShootingOperation::Assets(CollectionOperation::Add { id: id.clone(), item: asset, at: fixture.assets.len() }),
                        ShootingOperation::SetActiveAsset { asset_id: Some(id.clone()) },
                    ],
                    config_operations: vec![ShootingConfigOperation::SetSelection { shot_ids: Vec::new(), asset_ids: vec![id] }],
                    ..Default::default()
                }
            }
            ShootingCommand::SetSelection { shot_ids, asset_ids } => {
                Emit::config(vec![ShootingConfigOperation::SetSelection { shot_ids: shot_ids.clone(), asset_ids: asset_ids.clone() }])
            }
            ShootingCommand::SetSelectionMethod { method } => Emit::config(vec![ShootingConfigOperation::SetSelectionMethod { method: method.clone() }]),
            ShootingCommand::WorldSelect { ids, merge } => {
                let merged = merge_world_selection_ids(&semio_framework_plugin::SelectionSet::from_ids(config.selected_asset_ids.clone()), ids, merge).to_vec();
                Emit::config(vec![ShootingConfigOperation::SetSelection { shot_ids: config.selected_shot_ids.clone(), asset_ids: merged }])
            }
            ShootingCommand::SetHover { asset_id } => Emit::config(vec![ShootingConfigOperation::SetHoveredAsset { asset_id: asset_id.clone() }]),
            ShootingCommand::WorldPick { asset_id, asset_index, merge } => {
                let resolved = asset_index
                    .and_then(|index| fixture.assets.get(index as usize))
                    .map(|asset| asset.id.clone())
                    .or_else(|| asset_id.clone());
                match resolved {
                    None if merge == "replace" => Emit::config(vec![ShootingConfigOperation::SetSelection { shot_ids: config.selected_shot_ids.clone(), asset_ids: Vec::new() }]),
                    None => Emit::default(),
                    Some(id) => {
                        let merged = merge_world_selection_ids(&semio_framework_plugin::SelectionSet::from_ids(config.selected_asset_ids.clone()), &[id], merge).to_vec();
                        Emit::config(vec![ShootingConfigOperation::SetSelection { shot_ids: config.selected_shot_ids.clone(), asset_ids: merged }])
                    }
                }
            }
            ShootingCommand::WorldPointerDown | ShootingCommand::WorldPointerMove => Emit::default(),
        }
    }

    /// 🧮️ This app's typed configuration spec — mirrors `shooting_engine::ShootingConfig`'s three
    /// sticky-default fields, each grounded in an existing `.action_args` default (see that struct's doc).
    fn config_spec(&self) -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec {
            fields: vec![
                semio_framework_plugin::ConfigFieldSpec {
                    key: "defaultShotFormat".into(),
                    label: "Default Shot Format".into(),
                    shape: semio_framework_plugin::ConfigFieldShape::Select { options: vec!["svg".into(), "png".into()] },
                    default: Some(DslValue::String("png".into())),
                },
                semio_framework_plugin::ConfigFieldSpec {
                    key: "defaultShotShape".into(),
                    label: "Default Shot Shape".into(),
                    shape: semio_framework_plugin::ConfigFieldShape::Select { options: vec!["rectangle".into(), "ellipse".into()] },
                    default: Some(DslValue::String("rectangle".into())),
                },
                semio_framework_plugin::ConfigFieldSpec {
                    key: "defaultAssetFormat".into(),
                    label: "Default Asset Format".into(),
                    shape: semio_framework_plugin::ConfigFieldShape::Select { options: vec!["glb".into()] },
                    default: Some(DslValue::String("glb".into())),
                },
            ],
        }
    }

    //#endregion 🔖️TypedChannel

    fn render(&self, body_key: &str, doc: &DocumentView<'_, ShootingFixture>, cfg: &ConfigView<'_, ShootingConfig>) -> UiNode {
        let fixture = doc.projection;
        let labels = resolve_labels::<ShootingLabels>(cfg.projection);
        match body_key {
            SHOOTING_PLAY_BODY_SCENE => render_model_scene(fixture, cfg.projection),
            SHOOTING_PLAY_BODY_ICON => render_icon_scene(fixture, cfg.projection),
            SHOOTING_PLAY_BODY_DOCUMENT => build_document_tree(fixture, labels),
            SHOOTING_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            SHOOTING_PLAY_BODY_INSPECTION => build_inspector_tree(fixture, cfg.projection, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, ShootingFixture>, cfg: &ConfigView<'_, ShootingConfig>) -> HashMap<String, WindowEngagement> {
        let labels = resolve_labels::<ShootingLabels>(cfg.projection);
        HashMap::from([
            (SHOOTING_PLAY_WINDOW_SCENE.into(), shooting_model_engagement(doc.projection, cfg.projection, labels)),
            (SHOOTING_PLAY_WINDOW_ICON.into(), shooting_icon_engagement(doc.projection, labels)),
        ])
    }

    fn window_measures(&self, doc: &DocumentView<'_, ShootingFixture>, cfg: &ConfigView<'_, ShootingConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = resolve_labels::<ShootingLabels>(cfg.projection);
        HashMap::from([
            (SHOOTING_PLAY_WINDOW_SCENE.into(), shooting_model_measures(doc.projection, labels)),
            (SHOOTING_PLAY_WINDOW_ICON.into(), shooting_icon_measures(doc.projection, labels)),
        ])
    }

    fn app_labels(&self, cfg: &ConfigView<'_, ShootingConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<ShootingLabels>(cfg.projection);
        AppLabelsOverlay::default()
            .window_kind_label(SHOOTING_PLAY_WINDOW_SCENE, labels.window_scene)
            .window_kind_label(SHOOTING_PLAY_WINDOW_ICON, labels.window_icon)
            .action_labels(shooting_action_labels(is_de_locale(cfg.projection)))
            .utility_labels(shooting_utility_labels(is_de_locale(cfg.projection)))
    }
}
//#endregion 🔖️ShootingPlayApp

//#region 🔖️Manifest
pub fn create_shooting_app() -> App {
    App::from_builder(
        App::builder(SHOOTING_PLAY_APP_ID, "Shooting").document(["semio", "shooting"])
            .artifact_kind(ArtifactKindSpec {
                id: "2d.shooting".into(),
                name: "2D Shooting".into(),
                source_format: "shooting.scene".into(),
                component_kind: "shooting".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "shooting.scene".into(),
                export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
            })
            // 🖼️ `2d.image` — the interchange kind `photos:out` produces (WORKFLOWS-END-TO-END-TYPED-PORTS
            // Wave 2 port recipe); a sibling agent may declare the identical shape on the raster app too
            // — identical-shape duplicates are harmless (registry dedupes by id).
            .artifact_kind(ArtifactKindSpec {
                id: "2d.image".into(),
                name: "2D Image".into(),
                source_format: "2d.image".into(),
                component_kind: "image".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "2d.image".into(),
                export_formats: vec![OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Png],
            })
            .media_output(shooting_engine::shooting_photos_out_port())
            .icon_id("camera")
            .mode("edit", "Edit", "square-pen")
            .default_mode_id("edit")
            .window_kind(SHOOTING_PLAY_WINDOW_SCENE, "Scene", SHOOTING_PLAY_BODY_SCENE, SurfaceKind::World3d, "shooting-scene")
            .window_kind(SHOOTING_PLAY_WINDOW_ICON, "Icon", SHOOTING_PLAY_BODY_ICON, SurfaceKind::IconRender, "image")
            .default_layout(create_default_layout(
                &[SHOOTING_PLAY_WINDOW_SCENE.into(), SHOOTING_PLAY_WINDOW_ICON.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Model".into(), "Icon".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                SHOOTING_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                SHOOTING_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                SHOOTING_PLAY_BODY_INSPECTION,
            )
            // 🔧️ Document-mutating: dispatched as VCS operations with a true inverse.
            // 🛠️ Dev-only whole-fixture import — kept out of the command palette.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setFixtureJson", "Set Fixture Json", ActionKind::Operation) })
            .operation("setActiveExample", "Set Active Example")
            .operation("setActiveShot", "Set Active Shot")
            .operation("setActiveAsset", "Set Active Asset")
            .view_action("setCamera", "Set Camera")
            .operation("setShotCamera", "Set Shot Camera")
            .operation("saveCamera", "Save Camera")
            .view_action("loadSavedCamera", "Load Saved Camera")
            .operation("setSunAzimuth", "Set Sun Azimuth")
            .operation("setSunElevation", "Set Sun Elevation")
            .operation("setSunIntensity", "Set Sun Intensity")
            .operation("setAmbientIntensity", "Set Ambient Intensity")
            .operation("setMaterialRoughness", "Set Material Roughness")
            .operation("setShadowEnabled", "Set Shadow Enabled")
            .operation("toggleSun", "Toggle Sun")
            .operation("setActiveShotLabel", "Set Active Shot Label")
            .operation("setActiveShotFormat", "Set Active Shot Format")
            .operation("setActiveShotShape", "Set Active Shot Shape")
            .operation("patchShots", "Patch Shots")
            .operation("patchAssets", "Patch Assets")
            .operation("addShot", "Add Shot")
            .operation("addAsset", "Add Asset")
            .operation("importAsset", "Import Asset")
            .operation("resetFixture", "Reset Fixture")
            .operation("translateSelection", "Translate Selection")
            .operation("rotateSelection", "Rotate Selection")
            .operation("scaleSelection", "Scale Selection")
            // 👁️ Ephemeral view state — selection, camera draft label, world picking.
            .view_action("setSelection", "Set Selection")
            .view_action("setCameraDraftLabel", "Set Camera Draft Label")
            .view_action("setCenterModel", "Set Center Model")
            .view_action("worldSelect", "World Select")
            .view_action("setHover", "Set Hover")
            .view_action("worldPick", "World Pick")
            .view_action("setSelectionMethod", "Set Selection Method")
            .view_action("worldPointerDown", "World Pointer Down")
            .view_action("worldPointerMove", "World Pointer Move")
            // 🐚️ Shell effects — export/import round-trips through the host.
            .shell_action("saveDownload", "Save Download")
            .shell_action("loadRequest", "Load Request")
            .shell_action("importAssetRequest", "Import Asset Request")
            .shell_action("exportActiveShot", "Export Active Shot")
            .shell_action("exportAllShots", "Export All Shots")
            // 📝️ Staged argument forms for the panel-visible create actions (defaults materialized host-side).
            .action_args("addShot", vec![
                ActionArgDef::select("format", "Format", vec![ActionArgOption::new("svg", "SVG"), ActionArgOption::new("png", "PNG")]).default_value("png"),
                ActionArgDef::select("shape", "Shape", vec![ActionArgOption::new("rectangle", "Rectangle"), ActionArgOption::new("ellipse", "Ellipse")]).default_value("rectangle"),
            ])
            .action_args("addAsset", vec![
                ActionArgDef::select("format", "Format", vec![ActionArgOption::new("glb", "GLB")]).default_value("glb"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new(SHOOTING_EXAMPLE_DEFAULT_ID, "Default Base Icon"),
                ]).required(),
            ])
            // 🧰️ Transform gumball — an exclusive utility group scoped to the scene window (active utility is host-owned).
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", "Move", "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", "Rotate", "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", "Scale", "maximize-2") })
            .window_kind_utilities(SHOOTING_PLAY_WINDOW_SCENE, vec!["move".into(), "rotate".into(), "scale".into()])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // `config_spec()`/`shooting_io()` are this same information's single source of truth, reused
            // here rather than duplicated (`command_grammar` stays `CommandGrammar::empty()`: this app's
            // typed commands are dispatched via `ShootingCommand`'s `OpBinary` codec directly, not a
            // keyword-parsed text grammar).
            .config(ShootingPlayApp::default().config_spec())
            .io(shooting_engine::shooting_io()),
    )
    .example(
        SHOOTING_EXAMPLE_DEFAULT_ID,
        "Default Base Icon",
        default_fixture_json(),
        "camera",
    )
    .workflow("shooting", "Shooting", "icon")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, ViewState, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<ShootingPlayApp> {
        testkit::new_app::<ShootingPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so kind discipline runs.
    fn new_app_with_registry() -> VcsDocumentApp<ShootingPlayApp> {
        testkit::new_app_with_registry::<ShootingPlayApp>(create_shooting_app)
    }

    fn default_camera(position: [f64; 3]) -> ShootingCamera {
        ShootingCamera { position, target: [0.0, 0.0, 0.0], zoom: 1.0, fov: 50.0, up: None, projection: None }
    }

    #[test]
    fn renders_world_model_scene() {
        let mut app = new_app();
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
        let payload: Value = serde_json::from_str(&json).unwrap();
        let environment: Value = serde_json::from_str(payload["world3d"]["environmentJson"].as_str().unwrap()).unwrap();
        assert_eq!(environment["sun"]["azimuth"], json!(45.0));
        assert_eq!(environment["material"]["roughness"], json!(1.0));
        let frame: Value = serde_json::from_str(payload["world3d"]["frameJson"].as_str().unwrap()).unwrap();
        assert_eq!(frame["width"], json!(256));
        assert_eq!(frame["shape"], json!("rectangle"));
        let fit: Value = serde_json::from_str(payload["world3d"]["fitJson"].as_str().unwrap()).unwrap();
        assert_eq!(fit["enabled"], json!(true));
        let camera: Value = serde_json::from_str(payload["world3d"]["cameraJson"].as_str().unwrap()).unwrap();
        assert_eq!(camera["zoom"], json!(1.0));
        assert_eq!(camera["projection"], json!("perspective"));
    }

    #[test]
    fn renders_icon_render_scene_with_real_request() {
        let mut app = new_app();
        let node = app.render(SHOOTING_PLAY_BODY_ICON, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("icon-render"));
        let payload: Value = serde_json::from_str(&json).unwrap();
        let request: Value = serde_json::from_str(payload["iconRender"]["requestJson"].as_str().unwrap()).unwrap();
        assert_eq!(request["assetUrl"], json!("/mesh/🧊️base.glb"));
        assert_eq!(request["format"], json!("svg"));
        assert_eq!(request["shape"], json!("rectangle"));
        assert!(request.get("background").is_none(), "transparent default fixture background is omitted");
        assert_eq!(request["lights"]["sunAzimuth"], json!(45.0));
        assert!(payload["iconRender"]["footer"].as_str().unwrap().contains("256×256"));
    }

    #[test]
    fn save_and_load_camera_round_trip() {
        let mut app = new_app();
        app.dispatch_typed(ShootingCommand::SetCameraDraftLabel { value: "Hero".into() }, &testkit::meta("local")).expect("draft");
        let result = app.dispatch_typed(ShootingCommand::SaveCamera, &testkit::meta("local")).expect("save");
        assert_eq!(result.operations.len(), 1);
        let engagements = app.window_engagements();
        let possible = engagements[SHOOTING_PLAY_WINDOW_SCENE].possible_engagements.as_ref().unwrap();
        assert!(possible.iter().any(|entry| entry.label == "Hero"));
        let saved_id = possible[0].id.trim_start_matches("shooting.camera.").to_string();

        app.dispatch_typed(ShootingCommand::SetCamera { camera: default_camera([1.0, 2.0, 3.0]) }, &testkit::meta("local")).expect("move camera away");
        app.dispatch_typed(ShootingCommand::LoadSavedCamera { id: saved_id }, &testkit::meta("local")).expect("load");
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let camera: Value = serde_json::from_str(payload["world3d"]["cameraJson"].as_str().unwrap()).unwrap();
        // restored via the saved camera, not the position we moved to.
        assert_ne!(camera["position"], json!([1.0, 2.0, 3.0]));
    }

    #[test]
    fn scene_setters_mutate_lighting_and_measures_reflect_them() {
        let mut app = new_app();
        app.dispatch_typed(ShootingCommand::SetSunAzimuth { value: 90.0 }, &testkit::meta("local")).expect("azimuth");
        app.dispatch_typed(ShootingCommand::SetShadowEnabled { value: false }, &testkit::meta("local")).expect("shadow");
        let measures = app.window_measures();
        let model_measures = &measures[SHOOTING_PLAY_WINDOW_SCENE];
        assert!(model_measures.iter().any(|measure| matches!(measure, WindowMeasure::Slider { value, .. } if *value == 90.0)));
        assert!(measures[SHOOTING_PLAY_WINDOW_ICON].iter().any(|measure| matches!(measure, WindowMeasure::Select { .. })));
    }

    #[test]
    fn toggle_sun_round_trips_through_ops_and_defaults_off() {
        let mut app = new_app();
        let measures = app.window_measures();
        assert!(measures[SHOOTING_PLAY_WINDOW_SCENE].iter().any(|measure| matches!(measure, WindowMeasure::Toggle { id, pressed, .. } if id == "shooting.measure.sun-enabled" && !*pressed)));
        app.dispatch_typed(ShootingCommand::ToggleSun { value: true }, &testkit::meta("local")).expect("toggle");
        let measures = app.window_measures();
        assert!(measures[SHOOTING_PLAY_WINDOW_SCENE].iter().any(|measure| matches!(measure, WindowMeasure::Toggle { id, pressed, .. } if id == "shooting.measure.sun-enabled" && *pressed)));
    }

    #[test]
    fn center_model_and_asset_activation_bump_fit_revision() {
        let mut app = new_app();
        app.dispatch_typed(ShootingCommand::SetCenterModel { pressed: Some(false) }, &testkit::meta("local")).expect("off");
        app.dispatch_typed(ShootingCommand::SetCenterModel { pressed: Some(true) }, &testkit::meta("local")).expect("on");
        let fit_json_before = {
            let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
            let payload: Value = serde_json::to_value(&node).unwrap();
            let fit: Value = serde_json::from_str(payload["world3d"]["fitJson"].as_str().unwrap()).unwrap();
            fit["revision"].as_u64().unwrap()
        };
        assert_eq!(fit_json_before, 1);
        let asset_id = app.projection().expect("materialize projection").assets[0].id.clone();
        app.dispatch_typed(ShootingCommand::SetActiveAsset { asset_id: Some(asset_id) }, &testkit::meta("local")).expect("activate");
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let fit: Value = serde_json::from_str(payload["world3d"]["fitJson"].as_str().unwrap()).unwrap();
        assert_eq!(fit["revision"].as_u64().unwrap(), 2);
    }

    #[test]
    fn world_pick_and_hover_drive_selection_protocol() {
        let mut app = new_app();
        // worldPick is a View command: it drives config selection only, emitting no document operations.
        let result = app
            .dispatch_typed(ShootingCommand::WorldPick { asset_id: None, asset_index: Some(0), merge: "replace".into() }, &testkit::meta("local"))
            .expect("pick");
        assert!(result.operations.is_empty(), "worldPick mutates only ephemeral selection, never the document");
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
        let selection: Value = serde_json::from_str(serde_json::to_value(&node).unwrap()["world3d"]["selectionJson"].as_str().unwrap()).unwrap();
        assert_eq!(selection["ids"], json!(["base"]), "the picked asset becomes the config selection");
        app.dispatch_typed(ShootingCommand::SetHover { asset_id: Some("base".into()) }, &testkit::meta("local")).expect("hover");
        app.dispatch_typed(ShootingCommand::WorldPick { asset_id: None, asset_index: None, merge: "replace".into() }, &testkit::meta("local")).expect("clear pick");
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        let selection: Value = serde_json::from_str(payload["world3d"]["selectionJson"].as_str().unwrap()).unwrap();
        assert_eq!(selection["ids"], json!([]));
    }

    #[test]
    fn export_import_and_download_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(ShootingCommand::ExportShots { all: false }, &testkit::meta("local")).expect("export active");
        assert_eq!(result.requested_effects.len(), 1);
        match &result.requested_effects[0] {
            HostEffect::IconRenderExport { items } => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].filename, "overview-svg.svg");
                let request: Value = dsl::from_dsl_value(items[0].request.clone()).expect("decode request DslValue");
                assert_eq!(request["assetUrl"], json!("/mesh/🧊️base.glb"));
            }
            other => panic!("expected IconRenderExport, got {other:?}"),
        }
        let result = app.dispatch_typed(ShootingCommand::ExportShots { all: true }, &testkit::meta("local")).expect("export all");
        match &result.requested_effects[0] {
            HostEffect::IconRenderExport { items } => assert_eq!(items.len(), 2),
            other => panic!("expected IconRenderExport, got {other:?}"),
        }
        let result = app.dispatch_typed(ShootingCommand::SaveDownload, &testkit::meta("local")).expect("save download");
        match &result.requested_effects[0] {
            HostEffect::DownloadMediaExport { filename, data, .. } => {
                assert_eq!(filename, "shooting.fixture.ops");
                let round_trip: ShootingFixture = serde_json::from_str(data).unwrap();
                assert_eq!(round_trip.schema, SHOOTING_FIXTURE_SCHEMA);
            }
            other => panic!("expected DownloadMediaExport, got {other:?}"),
        }
        let result = app.dispatch_typed(ShootingCommand::LoadRequest, &testkit::meta("local")).expect("load request");
        match &result.requested_effects[0] {
            HostEffect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "setFixtureJson"),
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
        let result = app.dispatch_typed(ShootingCommand::ImportAssetRequest, &testkit::meta("local")).expect("import asset request");
        match &result.requested_effects[0] {
            HostEffect::RequestFileOpen { read_as, import_action, .. } => {
                assert_eq!(read_as.as_deref(), Some("dataUrl"));
                assert_eq!(import_action, "importAsset");
            }
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
        app.dispatch_typed(
            ShootingCommand::ImportAsset { payload: "data:model/gltf-binary;base64,AAAA".into(), name: Some("chair.glb".into()) },
            &testkit::meta("local"),
        )
        .expect("import asset");
        let projection = app.projection().expect("materialize projection");
        let imported = projection.assets.last().unwrap();
        assert_eq!(imported.name, "chair");
        assert!(imported.url.starts_with("data:"));
        assert_eq!(projection.active_asset_id, imported.id);
    }

    #[test]
    fn utility_registry_scopes_transform_gumball_and_actions_are_declared() {
        let definition = create_shooting_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["move", "rotate", "scale"], "gumball utilities declared in registry order");
        assert!(definition.utilities.iter().all(|utility| utility.group.as_deref() == Some("transform")), "one exclusive transform group");
        let scene = definition.window_kinds.iter().find(|window| window.id == SHOOTING_PLAY_WINDOW_SCENE).expect("scene window");
        let scoped: Vec<&str> = scene.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(scoped, ["move", "rotate", "scale"], "utilities scoped to the scene window kind");
        for command in ["loadRequest", "importAssetRequest", "saveDownload", "exportActiveShot", "exportAllShots", "resetFixture", "saveCamera"] {
            assert!(definition.actions.iter().any(|action| action.id == command), "registry declares {command}");
        }
        assert!(!definition.actions.iter().any(|action| action.id == "setTransformTool"), "the custom setTransformTool action is gone");
        let mut app = new_app();
        let engagements = app.window_engagements();
        assert!(engagements[SHOOTING_PLAY_WINDOW_SCENE].options.is_none(), "the gumball selector moved to the host-derived utility bar");
        assert!(engagements[SHOOTING_PLAY_WINDOW_SCENE].status.as_ref().unwrap()[0].text.contains("assets"));
        assert!(engagements[SHOOTING_PLAY_WINDOW_ICON].status.as_ref().unwrap()[0].text.contains("256×256"));
    }

    #[test]
    fn shooting_labels_resolve_native_english_by_default() {
        let mut app = new_app();
        let document_tree = app.render(SHOOTING_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Shots"));
        assert!(document_json.contains("Assets"));
        let catalogue = app.render(SHOOTING_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Add Shot"));
        assert!(catalogue_json.contains("Add Asset"));
        assert!(catalogue_json.contains("SVG Rectangle"));
        assert!(catalogue_json.contains("GLB Asset"));
        let inspector = app.render(SHOOTING_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&inspector).unwrap().contains("Shot"));
        let engagements = app.window_engagements();
        let model = &engagements[SHOOTING_PLAY_WINDOW_SCENE];
        assert_eq!(model.input.as_ref().unwrap().placeholder.as_deref(), Some("Camera label"));
        let icon = &engagements[SHOOTING_PLAY_WINDOW_ICON];
        assert_eq!(icon.input.as_ref().unwrap().placeholder.as_deref(), Some("Shot label"));
        let measures = app.window_measures();
        let icon_measures_json = serde_json::to_string(&measures[SHOOTING_PLAY_WINDOW_ICON]).unwrap();
        assert!(icon_measures_json.contains("Rectangle"));
        assert!(icon_measures_json.contains("SVG"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
    /// passing a `ViewState` into `render`/`window_engagements`/`window_measures` for this purpose.
    #[test]
    fn shooting_labels_resolve_native_german() {
        let mut app = new_app();
        app.dispatch_typed(ShootingCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let document_tree = app.render(SHOOTING_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Aufnahmen"));
        assert!(document_json.contains("Objekte"));
        let catalogue = app.render(SHOOTING_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Aufnahme hinzufügen"));
        assert!(catalogue_json.contains("Objekt hinzufügen"));
        assert!(catalogue_json.contains("SVG Rechteck"));
        assert!(catalogue_json.contains("GLB-Objekt"));
        let inspector = app.render(SHOOTING_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&inspector).unwrap().contains("Aufnahme"));
        let engagements = app.window_engagements();
        let model = &engagements[SHOOTING_PLAY_WINDOW_SCENE];
        assert_eq!(model.input.as_ref().unwrap().placeholder.as_deref(), Some("Kamera-Bezeichnung"));
        let icon = &engagements[SHOOTING_PLAY_WINDOW_ICON];
        assert_eq!(icon.input.as_ref().unwrap().placeholder.as_deref(), Some("Aufnahme-Bezeichnung"));
        let measures = app.window_measures();
        let icon_measures_json = serde_json::to_string(&measures[SHOOTING_PLAY_WINDOW_ICON]).unwrap();
        assert!(icon_measures_json.contains("Rechteck"));
    }

    #[test]
    fn set_active_shot_label_patches_active_shot() {
        let mut app = new_app();
        app.dispatch_typed(ShootingCommand::SetActiveShotLabel { value: "Hero Shot".into() }, &testkit::meta("local")).expect("label");
        assert_eq!(active_shot(&app.projection().expect("materialize projection")).unwrap().label, "Hero Shot");
    }

    #[test]
    fn reset_fixture_restores_default_fixture() {
        let mut app = new_app();
        app.dispatch_typed(ShootingCommand::AddShot { format: "svg".into(), shape: "ellipse".into() }, &testkit::meta("local")).expect("add shot");
        assert_eq!(app.projection().expect("materialize projection").shots.len(), 3);
        app.dispatch_typed(ShootingCommand::ResetFixture, &testkit::meta("local")).expect("reset");
        assert_eq!(app.projection().expect("materialize projection").shots.len(), 2);
    }

    #[test]
    fn model_scene_uses_asset_mesh_urls() {
        let mut app = new_app();
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("mesh:🧊️base"));
        assert!(json.contains("/mesh/🧊️base.glb"));
    }

    #[test]
    fn document_lists_shots_and_assets() {
        let mut app = new_app();
        let node = app.render(SHOOTING_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Overview Svg"));
        assert!(json.contains("Base"));
    }

    #[test]
    fn add_shot_action_appends_shot() {
        let mut app = new_app();
        app.dispatch_typed(ShootingCommand::AddShot { format: "svg".into(), shape: "ellipse".into() }, &testkit::meta("local")).expect("add shot");
        assert!(app.projection().expect("materialize projection").shots.iter().any(|shot| shot.format == "svg" && shot.shape == "ellipse"));
    }

    #[test]
    fn set_active_shot_updates_fixture() {
        let mut app = new_app();
        let second_id = app.projection().expect("materialize projection").shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
        app.dispatch_typed(ShootingCommand::SetActiveShot { shot_id: Some(second_id.clone()) }, &testkit::meta("local")).expect("set active");
        assert_eq!(app.projection().expect("materialize projection").active_shot_id, second_id);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            ShootingCommand::AddShot { format: "png".into(), shape: "rectangle".into() },
            |app| app.projection().expect("materialize projection").shots.len(),
            2,
            3,
        );
    }

    /// 🎥️ `SetCamera` is config-only — dragging the viewport camera through several ticks must never
    /// create a VCS edit/undo step on the DOCUMENT store at all (the config store gets its own separate
    /// edits, but `undo` here targets the document store and has nothing to revert).
    #[test]
    fn camera_drag_never_creates_a_document_undo_step() {
        let mut app = new_app();
        for position in [[1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0]] {
            app.dispatch_typed(ShootingCommand::SetCamera { camera: default_camera(position) }, &testkit::meta("local")).expect("drag tick");
        }
        let camera_position = |app: &mut VcsDocumentApp<ShootingPlayApp>| -> Value {
            let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
            let payload: Value = serde_json::to_value(&node).unwrap();
            let camera: Value = serde_json::from_str(payload["world3d"]["cameraJson"].as_str().unwrap()).unwrap();
            camera["position"].clone()
        };
        assert_eq!(camera_position(&mut app), json!([3.0, 0.0, 0.0]), "config camera reflects the last drag tick");
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo (no-op: nothing on the document store to undo)");
        assert_eq!(camera_position(&mut app), json!([3.0, 0.0, 0.0]), "document undo has nothing to revert — the drag never touched the document");
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
    /// apply DISJOINT edits (A renames a shot, B translates an asset), and exchanging operations over a
    /// `MemoryBackbone` converges both sides to contain BOTH edits — impossible with whole-document
    /// `setDocument` snapshots, which would have one side's write clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<ShootingPlayApp, (String, [f64; 3])>(
            "mem://shooting-convergence",
            ShootingCommand::SetActiveShotLabel { value: "Renamed By A".into() },
            ShootingCommand::TranslateSelection { asset_ids: vec!["base".into()], dx: 5.0, dy: 6.0, dz: 7.0 },
            |app| {
                let projection = app.projection().expect("materialize projection");
                (active_shot(&projection).unwrap().label.clone(), projection.assets[0].origin)
            },
        );
    }

    #[test]
    fn ingest_operations_is_idempotent_for_shooting() {
        testkit::assert_ingest_idempotent::<ShootingPlayApp, String>(
            ShootingCommand::SetActiveShotLabel { value: "Hero".into() },
            |app| active_shot(&app.projection().expect("materialize projection")).unwrap().label.clone(),
        );
    }

    #[test]
    fn set_active_utility_clears_hover_and_emits_no_document_operations() {
        let mut app = new_app();
        app.dispatch_typed(ShootingCommand::SetHover { asset_id: Some("base".into()) }, &testkit::meta("local")).expect("hover");
        // Switching utilities is a config-only command: it clears in-progress hover and must produce no
        // document operations (zero document history entries, nothing to sync).
        let result = app.dispatch_typed(ShootingCommand::SetActiveUtility { utility_id: "rotate".into() }, &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewState::default()).expect("render");
        let selection: Value = serde_json::from_str(serde_json::to_value(&node).unwrap()["world3d"]["selectionJson"].as_str().unwrap()).unwrap();
        assert_eq!(selection["transformMode"], json!("rotate"), "the gumball follows cfg.active_utility_id");
    }

    #[test]
    fn gumball_transform_drag_coalesces_into_one_edit() {
        let mut app = new_app();
        let asset_id = app.projection().expect("materialize projection").assets[0].id.clone();
        for dx in [1.0, 2.0, 3.0] {
            app.dispatch_typed(ShootingCommand::TranslateSelection { asset_ids: vec![asset_id.clone()], dx, dy: 0.0, dz: 0.0 }, &testkit::meta("local"))
                .expect("drag tick");
        }
        // A whole gumball drag (three ticks, same coalesce key) is ONE undo step, not one-operation-per-tick.
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        let restored = app.projection().expect("materialize projection");
        let original = default_fixture().assets.iter().find(|asset| asset.id == asset_id).map(|asset| asset.origin).expect("original origin");
        assert_eq!(restored.assets.iter().find(|asset| asset.id == asset_id).unwrap().origin, original, "undoing the coalesced drag restores the pre-drag origin");
    }

    #[test]
    fn world_pick_is_declared_as_a_view_action_and_emits_no_operations() {
        let definition = create_shooting_app().definition;
        let world_pick = definition.actions.iter().find(|action| action.id == "worldPick").expect("worldPick declared");
        assert!(matches!(world_pick.kind, ActionKind::View), "worldPick is a View action");
        let mut app = new_app_with_registry();
        let result = app
            .dispatch_typed(ShootingCommand::WorldPick { asset_id: None, asset_index: Some(0), merge: "replace".into() }, &testkit::meta("local"))
            .expect("pick");
        assert!(result.operations.is_empty(), "worldPick (View) emits no operations even under registry enforcement");
    }
}
//#endregion 🧪️Tests
