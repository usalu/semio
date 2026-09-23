//! 🖱️ 🖱️ Layout play app commands command — `canvas-drop`.

use crate::editor::layout::commands::{add_frame, add_page};
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::layout::modes::edit::windows::blueprint::config::{current, LayoutWindowConfig};
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🖱️ A surface id names its blueprint/preview surface directly (`"layout.play.blueprint"` /
/// `"layout.play.preview"`); an absent id defaults to blueprint (the interactive authoring surface).
fn surface_is_blueprint(surface_id: Option<&str>) -> bool {
    surface_id.is_none_or(|surface| surface.contains("blueprint"))
}

fn screen_to_world_for_surface(config: &LayoutWindowConfig, sx: f64, sy: f64, width: f64, height: f64) -> (f64, f64) {
    let camera_runtime = &config.camera;
    let camera = infinite_canvas::camera::Camera { x: camera_runtime.x, y: camera_runtime.y, zoom: camera_runtime.zoom.max(0.0001) };
    let viewport = infinite_canvas::camera::Viewport { width: width.max(1.0) as u32, height: height.max(1.0) as u32, dpr: 1.0 };
    let world = infinite_canvas::camera::screen_to_world(&camera, &viewport, infinite_canvas::Point::new(sx, sy));
    (world.x, world.y)
}

//#endregion 🔖️Shared

//#region 🔖️CanvasPointerDown
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
//#endregion 🔖️CanvasPointerMove

//#region 🔖️CanvasPointerUp
//#endregion 🔖️CanvasPointerUp

//#region 🔖️CanvasDragOver
//#endregion 🔖️CanvasDragOver

//#region 🔖️CanvasDragLeave
//#endregion 🔖️CanvasDragLeave

//#region 🔖️SetCamera
//#endregion 🔖️SetCamera

//#region 🔖️CanvasDrop
//#endregion 🔖️CanvasDrop

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-drop")]
pub struct CanvasDrop {
    pub surface_id: Option<String>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[value(default)]
    pub artifact_ref: String,
    #[value(default)]
    pub proxy_data_url: String,
}

fn native_artifact_kind(kind: &str) -> Option<&'static str> {
    crate::editor::layout::panels::catalogue::native_artifact_kind(kind)
}

fn place_native(payload: &CanvasDrop, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>, artifact_kind: &str, x: f64, y: f64) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    use crate::mutations::create_frame::CreateFrame;
    use crate::mutations::create_link::CreateLink;
    use crate::{Frame, ImageLink};
    let config = current(cfg);
    let Some(page) = doc.snapshot.pages.iter().find(|page| page.id == config.active_page_id) else { return Ok(Emit::default()) };
    let link_id = format!("link-{}", doc.snapshot.links.len() + 1);
    let frame_id = format!("frame-{}", page.frames.len() + 1);
    let layer_id = page.layer_ids.first().cloned().unwrap_or_else(|| "layer-1".into());
    let linked = !payload.artifact_ref.is_empty() || !payload.proxy_data_url.is_empty();
    let link = ImageLink {
        id: link_id.clone(),
        path: String::new(),
        hash: String::new(),
        width: 0,
        height: 0,
        dpi: 72,
        color_profile: None,
        state: Some(if linked { "ready" } else { "missing" }.into()),
        proxy_data_url: (!payload.proxy_data_url.is_empty()).then(|| payload.proxy_data_url.clone()),
        artifact_kind: artifact_kind.into(),
        artifact_ref: payload.artifact_ref.clone(),
    };
    let frame = Frame::Image {
        id: frame_id.clone(),
        layer_id: layer_id.clone(),
        bounds: crate::LayoutBounds { x, y, width: 200.0, height: 120.0, rotation: 0.0 },
        locked: None,
        visible: None,
        link_id,
    };
    Ok(Emit {
        artifact_mutations: vec![
            LayoutMutation::CreateLink(CreateLink { link, index: None }),
            LayoutMutation::CreateFrame(CreateFrame { page_id: page.id.clone(), frame, index: Some(page.frames.len()), layer_id: Some(layer_id) }),
        ],
        effects: vec![crate::editor::layout::layout_select_effect(std::slice::from_ref(&frame_id), "replace")],
        ..Default::default()
    })
}

/// 🐛️ Delegates document creation to `add_page`/`add_frame`'s own handlers so "drop adds content"
/// has one implementation, then always clears the drag-ghost regardless of surface/outcome.
pub fn handle(payload: &CanvasDrop, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
    if !blueprint {
        return Ok(Emit::default());
    }
    let (wx, wy) = screen_to_world_for_surface(&current(cfg), payload.x, payload.y, payload.width, payload.height);
    if payload.kind == "page" {
        add_page::handle(&add_page::AddPage {}, doc, cfg)
    } else if let Some(artifact_kind) = native_artifact_kind(&payload.kind) {
        place_native(payload, doc, cfg, artifact_kind, wx, wy)
    } else {
        add_frame::handle(&add_frame::AddFrame { kind: payload.kind.clone(), x: Some(wx), y: Some(wy) }, doc, cfg)
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn dropping_a_pdf_links_the_native_artifact_on_an_image_frame() {
        let document = crate::standards::v1::subsets::any::schema::default_document();
        let config = NoConfig::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let emit = handle(
            &CanvasDrop { surface_id: Some("layout.play.blueprint".into()), kind: "pdf".into(), x: 10.0, y: 10.0, width: 800.0, height: 600.0, artifact_ref: "doc-pdf".into(), proxy_data_url: String::new() },
            &doc,
            &cfg,
        )
        .expect("drop");
        let LayoutMutation::CreateLink(created) = &emit.artifact_mutations[0] else { panic!("link") };
        assert_eq!(created.link.artifact_kind, "s.stdio.pdf");
        assert_eq!(created.link.artifact_ref, "doc-pdf");
        let LayoutMutation::CreateFrame(frame) = &emit.artifact_mutations[1] else { panic!("frame") };
        let crate::Frame::Image { link_id, .. } = &frame.frame else { panic!("image frame") };
        assert_eq!(link_id, &created.link.id);
    }

    #[test]
    fn every_placeable_artifact_kind_links_its_native_dialect() {
        let document = crate::standards::v1::subsets::any::schema::default_document();
        let config = NoConfig::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        for (kind, _, artifact_kind) in crate::editor::layout::panels::catalogue::NATIVE_PLACEMENTS {
            let emit = handle(&CanvasDrop { surface_id: Some("layout.play.blueprint".into()), kind: (*kind).into(), x: 0.0, y: 0.0, width: 100.0, height: 100.0, artifact_ref: format!("ref-{kind}"), proxy_data_url: String::new() }, &doc, &cfg).expect(kind);
            let LayoutMutation::CreateLink(created) = &emit.artifact_mutations[0] else { panic!("{kind} link") };
            assert_eq!(created.link.artifact_kind, *artifact_kind, "{kind}");
            assert_eq!(created.link.artifact_ref, format!("ref-{kind}"));
        }
    }

    #[test]
    fn a_drop_payload_keeps_the_native_preview_image() {
        use crate::editor::layout::LayoutPlayApp;
        use semio_framework_plugin::ArtifactEditor;
        let drag = r#"{"kind":"png","artifactRef":"shot","proxyDataUrl":"data:image/png;base64,AA=="}"#;
        let args = dsl::DslValue::Object(vec![
            ("dragData".into(), dsl::DslValue::String(drag.into())),
            ("x".into(), dsl::DslValue::Number(dsl::Number::Float(12.0))),
            ("y".into(), dsl::DslValue::Number(dsl::Number::Float(24.0))),
            ("width".into(), dsl::DslValue::Number(dsl::Number::Float(800.0))),
            ("height".into(), dsl::DslValue::Number(dsl::Number::Float(600.0))),
        ]);
        let command = LayoutPlayApp::command_from_action("canvasDrop", Some(&args)).expect("bridge");
        let crate::editor::layout::LayoutCommand::CanvasDrop(payload) = command else { panic!("drop") };
        assert_eq!(payload.kind, "png");
        assert_eq!(payload.artifact_ref, "shot");
        assert_eq!(payload.proxy_data_url, "data:image/png;base64,AA==");
        let document = crate::standards::v1::subsets::any::schema::default_document();
        let config = NoConfig::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let emit = handle(&payload, &doc, &cfg).expect("place");
        let LayoutMutation::CreateLink(created) = &emit.artifact_mutations[0] else { panic!("link") };
        assert_eq!(created.link.artifact_kind, "s.stdio.png");
        assert_eq!(created.link.proxy_data_url.as_deref(), Some("data:image/png;base64,AA=="));
        assert_eq!(created.link.state.as_deref(), Some("ready"));
    }
}
//#endregion 🧪️Tests
