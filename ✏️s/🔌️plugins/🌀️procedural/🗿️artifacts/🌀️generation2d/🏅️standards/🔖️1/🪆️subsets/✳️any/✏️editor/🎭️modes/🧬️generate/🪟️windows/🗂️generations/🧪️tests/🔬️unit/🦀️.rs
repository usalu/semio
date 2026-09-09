use super::*;
use crate::editor::generation2d::testkit::{app, close, render as render_body};

/// 🗂️ The generate-mode window renders BOTH of its sections against an untouched document: the
/// generations list (empty placeholder) and the actions section carrying the add row. The fixture
/// projection serializes keys and components only — action bindings ride on `BuiltNode::on` and are
/// not projected — so the add route is measured by its stable surface key, and its bridge to
/// `addGeneration` by `declared_actions_bridge_to_commands`.
#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_surfaces() {
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION2D_PLAY_BODY_GENERATIONS).await;
    close(app);
    assert!(rendered.contains("procedural2d-play-generate.generations"), "{rendered}");
    assert!(rendered.contains("procedural2d-play-generate.add-generation"), "{rendered}");
    assert!(rendered.contains("Add Generation"), "{rendered}");
}
