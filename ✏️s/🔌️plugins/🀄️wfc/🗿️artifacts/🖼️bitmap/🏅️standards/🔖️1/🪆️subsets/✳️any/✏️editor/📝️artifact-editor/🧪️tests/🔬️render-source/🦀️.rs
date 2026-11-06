use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::encode_base64;

/// 🎨️ Mounted render authority, rather than command authority, admits a matching preview.
#[semio_framework_async_macros::async_test]
async fn a_mounted_render_uses_its_own_source_revision() {
    let snapshot = crate::examples::rooms_16::snapshot();
    let history = semio_plugin::HistoryView::empty();
    let context = semio_plugin::AppRenderOperationContext {
        app_instance_id: 1,
        base_revision: semio_framework_job::RevisionId(1),
        generation: semio_framework_job::Generation(1),
        canonical_base_revision: [1; 32],
        mounted_policy: semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,
    };
    let doc = ArtifactView::with_render_context(&snapshot, &history, semio_plugin::ChildContentView::EMPTY, context, "bitmap", None).await;
    assert!(doc.operation_optional().is_none());
    let no_config = NoConfig::default();
    let config = ConfigView { snapshot: &no_config, window: None };
    let cache = BitmapTransient {
        output_pixels: Some(encode_base64(&vec![1; (snapshot.output.width * snapshot.output.height) as usize])),
        contradiction: false,
        output_width: snapshot.output.width,
        output_height: snapshot.output.height,
        source_revision: encode_base64(&[1; 32]),
    };
    let empty = BitmapEditor::render_bodies(output::BODY_KEY, &doc, &config, &BitmapTransient::default(), None).expect("empty output");
    let fresh = BitmapEditor::render_bodies(output::BODY_KEY, &doc, &config, &cache, None).expect("matching output");
    let empty = serde_json::to_value(&empty.root).expect("empty output tree");
    assert_ne!(serde_json::to_value(&fresh.root).expect("fresh output tree"), empty);
    let changed = ArtifactView::with_render_context(&snapshot, &history, semio_plugin::ChildContentView::EMPTY, semio_plugin::AppRenderOperationContext { canonical_base_revision: [2; 32], ..context }, "bitmap", None).await;
    let stale = BitmapEditor::render_bodies(output::BODY_KEY, &changed, &config, &cache, None).expect("stale output");
    assert_eq!(serde_json::to_value(&stale.root).expect("stale output tree"), empty);
}
