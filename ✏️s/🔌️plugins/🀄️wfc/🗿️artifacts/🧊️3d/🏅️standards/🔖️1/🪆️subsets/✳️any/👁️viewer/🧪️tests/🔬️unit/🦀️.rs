//! 🧪️ Viewer surface laws: the app id is canonical, the surface is structurally read-only, and it
//! boots on the same example the editor does.

use crate::viewer::wfc3d::modes::view::windows::preview;
use crate::{WFC3D_DIALECT, WFC3D_DOCUMENT_SCHEMA};
use crate::viewer::wfc3d::*;
use semio_plugin::ArtifactViewer;

#[test]
fn the_viewer_app_id_is_the_canonical_surface_id() {
    let definition = create_wfc3d_viewer();
    assert_eq!(definition.id, "s.wfc.wfc3d@1/*#viewer");
    assert_eq!(definition.breadcrumb, vec!["semio".to_string(), "wfc".to_string(), "3d".to_string()]);
}

#[test]
fn the_viewer_declares_exactly_one_read_only_window() {
    let definition = create_wfc3d_viewer();
    assert_eq!(definition.window_kinds.len(), 1);
    assert_eq!(definition.window_kinds[0].id, preview::WFC_3D_VIEW_WINDOW);
    assert!(
        definition.window_kinds[0].actions.iter().all(|action| action.kind != semio_plugin::ActionKind::Mutation),
        "a viewer window declares no MUTATING action; the shell's own chrome verbs are injected by the builder and are not this surface's"
    );
}

#[test]
fn the_viewer_and_the_editor_share_one_dialect() {
    assert_eq!(<Wfc3dViewer as ArtifactViewer>::DIALECT, WFC3D_DIALECT);
    assert_eq!(<Wfc3dViewer as ArtifactViewer>::DOCUMENT_SCHEMA, WFC3D_DOCUMENT_SCHEMA);
}

#[test]
fn the_viewer_boots_on_a_real_document() {
    assert_eq!(<Wfc3dViewer as ArtifactViewer>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births"), crate::examples::two_room_corridor::snapshot());
}

#[test]
fn an_unknown_body_key_renders_a_label_instead_of_failing() {
    assert!(render_body("nope", &crate::examples::two_room_corridor::snapshot(), &Default::default(), None, semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).is_ok());
}

/// 👁️ The read-only body renders for every bundled example — the same statement the editor makes for
/// its own two windows.
#[test]
fn the_view_body_renders_for_every_example() {
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        let tree = render_body(preview::WFC_3D_VIEW_BODY, &document, &Default::default(), None, semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).expect("the viewer body renders");
        assert!(!format!("{tree:?}").is_empty());
    }
}

/// 🪪️ The mounted viewer only projects canonical current cache and authored pins, with explicit language axes.
#[semio_framework_async_macros::async_test]
async fn mounted_viewer_uses_current_cache_and_authored_pins() {
    use crate::editor::wfc3d::transient::{Wfc3dAssignment, Wfc3dTransient};
    use semio_ui_locale::{Locale, Terminology};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).unwrap();
    let mut document = crate::examples::two_room_corridor::snapshot();
    for slot in &mut document.slots { slot.pinned_tile_id = None; }
    let history = semio_plugin::HistoryView::empty();
    let context = semio_plugin::AppRenderOperationContext { app_instance_id: 1, base_revision: semio_framework_job::RevisionId(1), generation: semio_framework_job::Generation(1), canonical_base_revision: [7;32], mounted_policy: semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT };
    let project = |doc: &semio_plugin::ArtifactView<'_, crate::Wfc3dSnapshot>, cache: &Wfc3dTransient, locale| {
        let tree = crate::viewer::wfc3d::render_with_transient(preview::WFC_3D_VIEW_BODY, doc, cache, locale, Terminology::Native).unwrap();
        serde_json::to_value(&tree.root).unwrap()
    };
    for case in fixture["cases"].as_array().unwrap() {
        let current: Option<[u8;32]> = serde_json::from_value(case["currentRevision"].clone()).unwrap();
        let doc = if let Some(revision) = current {
            semio_plugin::ArtifactView::with_render_context(&document, &history, semio_plugin::ChildContentView::EMPTY, semio_plugin::AppRenderOperationContext { canonical_base_revision: revision, ..context }, "wfc3d-viewer", None).await
        } else { semio_plugin::ArtifactView::new(&document, &history) };
        let cache = Wfc3dTransient { source_revision: serde_json::from_value(case["sourceRevision"].clone()).unwrap(), assignments: vec![Wfc3dAssignment { slot_id: document.slots[0].id.clone(), tile_id: document.tiles[0].id.clone() }], contradiction: false };
        let empty = project(&doc, &Wfc3dTransient::default(), Locale::En);
        assert_eq!(project(&doc, &cache, Locale::En) != empty, case["visible"].as_bool().unwrap(), "{}", case["id"]);
    }
    let presentation: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🧫️viewer-preview/🔣️.json")).unwrap();
    for case in presentation["cases"].as_array().unwrap() {
        document.slots[0].pinned_tile_id = case["authored"].as_bool().unwrap().then(|| document.tiles[0].id.clone());
        let doc = semio_plugin::ArtifactView::new(&document, &history);
        let locale = match case["locale"].as_str().unwrap() { "en" => Locale::En, "de" => Locale::De, _ => panic!("unsupported fixture locale") };
        let actual = project(&doc, &Wfc3dTransient::default(), locale).to_string();
        assert!(actual.contains(case["message"].as_str().unwrap()), "{}", case["id"]);
        let instance_label = format!("{} · {}", document.slots[0].id, document.tiles[0].id);
        assert_eq!(actual.contains(&instance_label), case["authored"].as_bool().unwrap(), "{}", case["id"]);
    }
}
