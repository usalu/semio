use super::*;
use crate::editor::fem2d::testkit::{fem2d_app, render as render_body};

fn visual_freshness(generation: u64) -> Fem2dVisualFreshness {
    Fem2dVisualFreshness { app_instance_id: 7, model_revision: 11, document_generation: generation, operation: 13, numerical_preview_sequence: 17, surface_generation: generation, renderer_scene_generation: generation }
}

fn sealed_visual(doc: &Fem2dSnapshot, visual: &Fem2dLiveVisual) -> Fem2dMountedVisualLease {
    let mut job = Fem2dVisualJob::new(visual_freshness(19));
    let mut turns = 0;
    while !job.step_one(doc, visual, visual_freshness(19)).expect("bounded production step") && turns < 2_048 {
        turns += 1;
    }
    job.take_complete().expect("sealed visual")
}

fn packet_contains(lease: &Fem2dMountedVisualLease, needle: &[u8]) -> bool {
    (0..lease.snapshot.page_count).any(|page| canvas2d_snapshot_with_page(lease.snapshot, page, |owner| owner.bytes().windows(needle.len()).any(|window| window == needle)).unwrap_or(false))
}

fn packet_equal(left: &Fem2dMountedVisualLease, right: &Fem2dMountedVisualLease) -> bool {
    left.snapshot.page_count == right.snapshot.page_count
        && (0..left.snapshot.page_count).all(|page| {
            let left_hash = canvas2d_snapshot_with_page(left.snapshot, page, |owner| owner.bytes().iter().fold((0xcbf2_9ce4_8422_2325_u64, 0_usize), |(hash, len), byte| ((hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3), len + 1))).ok();
            let right_hash = canvas2d_snapshot_with_page(right.snapshot, page, |owner| owner.bytes().iter().fold((0xcbf2_9ce4_8422_2325_u64, 0_usize), |(hash, len), byte| ((hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3), len + 1))).ok();
            left_hash == right_hash
        })
}

#[semio_framework_async_macros::async_test]
async fn renders_fem2d_model_scene() {
    let mut app = fem2d_app();
    assert!(render_body(&mut app, BODY_KEY).contains("canvas-2d"));
}

#[semio_framework_async_macros::async_test]
async fn mesh_preview_renders_region_edges() {
    let mut app = fem2d_app();
    crate::editor::fem2d::testkit::dispatch(&mut app, crate::editor::fem2d::Fem2dCommand::SetActiveExample(crate::editor::fem2d::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = render(&snapshot, &FemCamera::default()).expect("fixture surface admission");
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
    let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("decode canvas scene");
    assert!(scene.layers_json.contains("mesh-edge-"), "expected mesh-edge preview layers in the model scene");
}

#[semio_framework_async_macros::async_test]
async fn fem2d_model_extent_degenerate_model_returns_one() {
    assert_eq!(fem2d_model_extent(&crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot()), 1.0);
}

#[semio_framework_async_macros::async_test]
async fn live_visual_language_distinguishes_every_progress_state() {
    use store::ArtifactDsl;
    let doc = Fem2dSnapshot::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM2D_EXAMPLE_TEXT).expect("parse example");
    let region_id = doc.regions.first().expect("example region").id.clone();
    let element_id = element_id(doc.elements.first().expect("example element")).to_string();
    let node_id = doc.nodes.first().expect("example node").id.clone();
    for quality in [RegionVisualQuality::Unmeshed, RegionVisualQuality::Coarse, RegionVisualQuality::Refined, RegionVisualQuality::Final] {
        let visual = Fem2dLiveVisual {
            region_quality: [(region_id.clone(), quality)].into_iter().collect(),
            assembling_element_ids: vec![element_id.clone()],
            fields: vec![NodeLiveField { node_id: node_id.clone(), displacement: [0.01, -0.02], residual: [3.0, -4.0], reaction: [-3.0, 4.0], contour: 5.0, mode_shape: [0.2, -0.1] }],
            state: if quality == RegionVisualQuality::Final { FemVisualState::ValidatedFinal } else { FemVisualState::Coarse },
            residual_norm: 5.0,
            tolerance: 1e-8,
            progress_completed: 1,
            progress_total: 2,
            converged: quality == RegionVisualQuality::Final,
            validated_final: quality == RegionVisualQuality::Final,
        };
        let encoded = dsl::json::to_string(&dsl::json::Value::Array(fem2d_live_visual_layers(&doc, &visual)));
        assert!(encoded.contains(&format!("region-quality-{}", quality.id())));
        assert!(encoded.contains("assembling-"));
        assert!(encoded.contains("displacement-field-"));
        assert!(encoded.contains("residual-field-"));
        assert!(encoded.contains(if quality == RegionVisualQuality::Final { "solve-status-validated-final" } else { "solve-status-unconverged" }));
    }
}

#[semio_framework_async_macros::async_test]
async fn model_visual_language_includes_load_and_support_glyphs() {
    use store::ArtifactDsl;
    let doc = Fem2dSnapshot::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM2D_EXAMPLE_TEXT).expect("parse example");
    let encoded = dsl::json::to_string(&dsl::json::Value::Array(fem2d_structure_layers(&doc, "#38bdf8", "#94a3b8", "#f97316")));
    assert!(encoded.contains("support-"));
    assert!(encoded.contains("load-"));
}

#[semio_framework_async_macros::async_test]
async fn live_visual_replay_is_deterministic_and_bounded() {
    use std::time::Instant;
    use store::ArtifactDsl;

    let doc = Fem2dSnapshot::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM2D_EXAMPLE_TEXT).expect("parse example");
    let node_id = doc.nodes.first().expect("example node").id.clone();
    let visual = Fem2dLiveVisual {
        region_quality: doc.regions.iter().rev().enumerate().map(|(index, region)| (region.id.clone(), if index % 2 == 0 { RegionVisualQuality::Coarse } else { RegionVisualQuality::Refined })).collect(),
        assembling_element_ids: doc.elements.iter().rev().map(|element| element_id(element).to_string()).collect(),
        fields: (0..256)
            .rev()
            .map(|index| NodeLiveField {
                node_id: node_id.clone(),
                displacement: [index as f64 * 1e-6, -1e-4],
                residual: [index as f64 * 1e-3, -0.5],
                reaction: [0.5, -(index as f64) * 1e-3],
                contour: index as f64,
                mode_shape: [index as f64 * 1e-4, -1e-4],
            })
            .collect(),
        state: FemVisualState::SolvingUnconverged,
        residual_norm: 0.5,
        tolerance: 1e-8,
        progress_completed: 128,
        progress_total: 256,
        converged: false,
        validated_final: false,
    };
    let started = Instant::now();
    let first = fem2d_live_visual_layers(&doc, &visual);
    let elapsed = started.elapsed();
    let second = fem2d_live_visual_layers(&doc, &visual);
    assert_eq!(first, second, "the same accepted preview must replay byte-stably");
    assert!(elapsed.as_micros() < 8_000, "live overlay step took {} us", elapsed.as_micros());
}

#[test]
fn mounted_visual_output_exact_maximum_plus_one_and_page_handback() {
    let mut output = Fem2dFixedPacketPages::new();
    output.admit_page(0).expect("page zero");
    output.admit_page(1).expect("page one");
    output.admit_page(2).expect("page two");
    output.admit_page(3).expect("page three");
    let exact = "x".repeat(FEM2D_MOUNTED_VISUAL_PAGE_BYTES);
    output.write_str(&exact).expect("page zero exact");
    output.write_str(&exact).expect("page one exact");
    output.write_str(&exact).expect("page two exact");
    output.write_str(&exact).expect("page three exact");
    let before = output.pages[0].as_ref().expect("page retained").backing_identity();
    assert!(output.write_char('x').is_err(), "maximum plus one must reject before allocating");
    assert_eq!(output.pages[0].as_ref().expect("same page").backing_identity(), before);

    let mut released = 0;
    while !output.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0 {
        released += 1;
    }
    assert_eq!(released + 1, FEM2D_MOUNTED_VISUAL_PAGE_COUNT);
    assert!(output.terminal_is_empty());
}

#[test]
fn fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer() {
    let mut doc = Fem2dSnapshot::default();
    doc.nodes.resize_with(FEM2D_VISUAL_MAXIMUM_NODES + 1, || crate::FemNode { id: "n".into(), x: 0.0, y: 0.0 });
    let before = doc.nodes.as_ptr();
    let mut job = Fem2dVisualJob::new(visual_freshness(19));
    assert!(job.step_one(&doc, &Fem2dLiveVisual::default(), visual_freshness(19)).is_err());
    assert_eq!(before, doc.nodes.as_ptr());
    assert_eq!(job.stage(), Fem2dVisualJobStage::ReserveSnapshot);
}

#[test]
fn fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid() {
    let doc = Fem2dSnapshot::default();
    let visual = Fem2dLiveVisual::default();
    let mut stale = Fem2dVisualJob::new(visual_freshness(19));
    let mut turns = 0;
    while stale.stage() != Fem2dVisualJobStage::ValidateFreshness && turns < 512 {
        stale.step_one(&doc, &visual, visual_freshness(19)).expect("bounded production step");
        turns += 1;
    }
    assert!(stale.step_one(&doc, &visual, visual_freshness(23)).is_err());
    assert!(stale.take_complete().is_none());
    while !stale.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0 && turns < 2_048 {
        turns += 1;
    }
    assert!(stale.terminal_is_empty());

    let current = sealed_visual(&doc, &visual);
    let current_snapshot = current.snapshot();
    let mut rejected = Fem2dVisualJob::new(visual_freshness(29));
    assert!(!rejected.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0);
    assert!(canvas2d_snapshot_with_page(current_snapshot, 0, |_| ()).is_ok());
}

#[test]
fn fem2d_visual_job_replay_accessibility_and_each_step_are_bounded() {
    let doc = Fem2dSnapshot::default();
    let visual = Fem2dLiveVisual { state: FemVisualState::ValidatedFinal, validated_final: true, progress_completed: 1, progress_total: 1, tolerance: 1e-8, ..Default::default() };
    let first = sealed_visual(&doc, &visual);
    let second = sealed_visual(&doc, &visual);
    assert!(packet_equal(&first, &second));
    assert!(packet_contains(&first, b"accessible-en"));
    assert!(packet_contains(&first, b"accessible-de"));
    assert!(packet_contains(&first, b"Cancel Retry Discard"));
    assert!(packet_contains(&first, "Abbrechen Wiederholen Verwerfen".as_bytes()));

    let mut job = Fem2dVisualJob::new(visual_freshness(19));
    let started = std::time::Instant::now();
    let _ = job.step_one(&doc, &visual, visual_freshness(19));
    assert!(started.elapsed().as_micros() < 8_000);
}
