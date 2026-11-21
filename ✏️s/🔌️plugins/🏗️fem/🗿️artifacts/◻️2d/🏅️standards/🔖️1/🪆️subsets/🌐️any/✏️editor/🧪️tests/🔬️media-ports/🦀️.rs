//! 🚪️ FEM 2D editor media port contracts exercised through the registered editor type.

use semio_plugin::{AppOperationContext, ArtifactEditor, ArtifactView, HistoryView, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshot, Fem2dSnapshotRetiring, Fem2dSnapshotValue};
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;

pub(super) fn editing_doc<'a>(snapshot: &'a Fem2dSnapshot, history: &'a HistoryView) -> ArtifactView<'a, Fem2dSnapshot> {
    ArtifactView::with_operation(snapshot, history, AppOperationContext {
        app_instance_id: 1,
        parent_document_id: "fem2d-law".into(),
        operation_id: 1,
        generation: 1,
        canonical_base_revision: [0; 32],
        retained: Default::default(),
        authoring_seed: "fem2d-law:local-operation".into(),
    })
}

/// 🧬️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
/// see `📓️taxonomy.md`'s forbidden vocabulary), so `import_media("artifact:in")` now surfaces as a
/// `Effect::LoadDocument` carrying the replacement document's pack bytes, not an
/// `artifact_mutations` entry — asserted directly on `Emit` rather than through `app.snapshot()`.
#[semio_framework_async_macros::async_test]
async fn export_media_document_out_round_trips_via_import_media_document_in() {
    let snapshot = Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().unwrap());
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*snapshot, &history);
    let media = Fem2dPlayApp::export_media("artifact:out", &doc).expect("document:out exports");
    assert_eq!(media.media_type.class, MediaClass::TwoD);
    assert_eq!(media.media_type.form, MediaForm::Vector);
    let empty_projection = Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot());
    let empty_history = semio_plugin::HistoryView::empty();
    let empty_doc = ArtifactView::new(&*empty_projection, &empty_history);
    let emit = Fem2dPlayApp::import_media("artifact:in", &media, &empty_doc).expect("document:in imports");
    assert!(emit.artifact_mutations.is_empty(), "whole-document replace must not be an artifact_mutations entry");
    let semio_framework::kernel::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("document:in must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = Fem2dSnapshotRetiring::new(<Fem2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack"));
    assert_eq!(&*loaded, &*snapshot);
}

#[semio_framework_async_macros::async_test]
async fn export_media_results_out_returns_json_with_every_case_and_combination() {
    let snapshot = Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().unwrap());
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*snapshot, &history);
    let media = Fem2dPlayApp::export_media("results:out", &doc).expect("results:out exports");
    assert_eq!(media.media_type.class, MediaClass::Data);
    assert_eq!(media.media_type.form, MediaForm::Value);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "computation.fem2d");
            let value = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("results:out payload is valid JSON");
            for (kind, id) in [("loadCase", "dead"), ("loadCase", "live"), ("combination", "uls")] {
                let source = semio_s_fem_analyses_model::encode_result_source_parts(kind, id).expect("canonical result source");
                let result = value.get(&source).unwrap_or_else(|| panic!("missing {source} in results:out payload: {value}"));
                assert!(result.get("displacements").is_some());
                assert!(result.get("reactions").is_some());
                assert!(result.get("checks").is_some());
            }
        }
        MediaPayload::Binary { .. } | MediaPayload::Intrinsic { .. } => panic!("expected a Structured payload"),
    }
}

#[semio_framework_async_macros::async_test]
async fn export_media_results_out_errors_when_no_load_cases_are_defined() {
    let snapshot = Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot());
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*snapshot, &history);
    let error = Fem2dPlayApp::export_media("results:out", &doc).expect_err("no load cases means no results to export");
    match error {
        MediaError::Payload(port, _) => assert_eq!(port, "results:out"),
        other => panic!("expected MediaError::Payload, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn export_media_unknown_port_is_not_implemented() {
    let snapshot = Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot());
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*snapshot, &history);
    assert!(matches!(Fem2dPlayApp::export_media("bogus:out", &doc), Err(MediaError::NotImplemented)));
}

#[semio_framework_async_macros::async_test]
async fn import_media_geometry_in_builds_a_new_region_from_the_first_material() {
    let value = Fem2dSnapshotValue {
        materials: vec![semio_s_fem_material::planar::FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 2.1e11, nu: 0.3, rho: 7850.0 }],
        ..Default::default()
    };
    let snapshot = Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(value).expect("material fixture"));
    let history = semio_plugin::HistoryView::empty();
    let doc = editing_doc(&snapshot, &history);
    let payload = semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({ "outline": [[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], "holes": [] }));
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
    let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
    assert_eq!(emit.artifact_mutations.len(), 1);
    match &emit.artifact_mutations[0] {
        Fem2dMutation::CreateRegion(semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region, .. }) => {
            assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
            assert!(region.holes.is_empty());
            assert_eq!(region.material_id, "steel");
        }
        _ => panic!("expected CreateRegion"),
    }
}

#[semio_framework_async_macros::async_test]
async fn import_media_geometry_in_falls_back_to_unassigned_material_when_none_exists() {
    let snapshot = Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot());
    let history = semio_plugin::HistoryView::empty();
    let doc = editing_doc(&snapshot, &history);
    let payload = semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({ "outline": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]] }));
    let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
    let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
    match &emit.artifact_mutations[0] {
        Fem2dMutation::CreateRegion(semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion { region, .. }) => assert_eq!(region.material_id, "unassigned"),
        _ => panic!("expected CreateRegion"),
    }
}
