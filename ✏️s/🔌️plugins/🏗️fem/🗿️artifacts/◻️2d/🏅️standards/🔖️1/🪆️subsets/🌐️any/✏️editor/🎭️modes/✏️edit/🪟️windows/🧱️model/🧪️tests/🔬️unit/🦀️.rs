use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::*;
use semio_framework_pack_json::{JsonMemberPolicy, parse_bytes};
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::{
    Fem2dInteractionSnapshot, canvas_gesture::FEM2D_UTILITY_SELECT_DIRECT,
};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::{
    default_fem2d_snapshot, empty_fem2d_snapshot,
};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{
    Fem2dSnapshot, Fem2dSnapshotRetiring, Fem2dSnapshotValue,
};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_fem_node::planar::FemNodeValue;
use semio_os_kernel::Viewport2d;
use semio_plugin::{BuiltNode, Canvas2dScene};
use semio_ui_scene::{CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY, Canvas2dSnapshotLease, SceneDoc, canvas2d_snapshot_with_page};
use std::fmt::Write;
use crate::context::fem2d_app;
use crate::context::render as render_body;

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

fn demo_snapshot() -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(default_fem2d_snapshot().expect("bundled demo loads"))
}

fn empty_snapshot() -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(empty_fem2d_snapshot())
}

fn append_lane_text(node: &BuiltNode, payload: &mut String) {
    if let semio_ui_contract::Component::Text(text) = &node.component {
        payload.push_str(&text.packed_payload());
    }
    for child in node.children.iter() {
        append_lane_text(child, payload);
    }
}

fn canvas_scene(node: &BuiltNode) -> Canvas2dScene {
    let semio_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
    assert_eq!(props.kind, semio_ui_contract::SurfaceKind::Canvas2d);
    let mut scene: Canvas2dScene = semio_ui_scene::decode(props).expect("decode canvas scene spine");
    for carrier in node.children.iter() {
        let mut payload = String::new();
        append_lane_text(carrier, &mut payload);
        assert!(scene.merge_lane(carrier.key.as_str(), payload));
    }
    scene
}

fn published_snapshot(doc: &Fem2dSnapshot, lease: &Fem2dMountedVisualLease) -> Canvas2dSnapshotLease {
    let node = render_with_progress(
        doc, &Viewport2d::default(), Some(lease), &Fem2dInteractionSnapshot::default(), None, FEM2D_UTILITY_SELECT_DIRECT, &Default::default(), semio_plugin::Locale::En,
    ).expect("publish the visual through the model surface");
    canvas_scene(&node).snapshot.expect("surface carries the visual snapshot lease")
}

fn packet_pages(snapshot: Canvas2dSnapshotLease) -> Vec<Vec<u8>> {
    (0..snapshot.page_count)
        .map(|index| canvas2d_snapshot_with_page(snapshot, index, |page| page.bytes().to_vec()).expect("published packet page"))
        .collect()
}

fn close_visual(mut lease: Fem2dMountedVisualLease, pages: usize) {
    let mut pages_returned = 0;
    for _ in 0..512 {
        let (terminal, items, bytes) = lease.close_step(CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        assert!(items <= 1 && bytes <= CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        pages_returned += usize::from(bytes == CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY);
        if terminal {
            assert_eq!(pages_returned, pages);
            assert!(lease.terminal_is_empty());
            return;
        }
    }
    panic!("visual lease did not return every retained page");
}

#[semio_framework_async_macros::async_test]
async fn renders_fem2d_model_scene() {
    let mut app = fem2d_app();
    assert!(render_body(&mut app, BODY_KEY).contains("canvas-2d"));
}

#[semio_framework_async_macros::async_test]
async fn mesh_preview_renders_region_edges() {
    let doc = demo_snapshot();
    let node = render(&doc, &Viewport2d::default(), semio_plugin::Locale::En).expect("fixture surface admission");
    let scene = canvas_scene(&node);
    assert!(scene.layers_json.contains("mesh-edge-"), "expected mesh-edge preview layers in the model scene");
}

#[semio_framework_async_macros::async_test]
async fn fem2d_model_extent_degenerate_model_returns_one() {
    assert_eq!(fem2d_model_extent(&empty_snapshot()), 1.0);
}

#[semio_framework_async_macros::async_test]
async fn live_visual_language_distinguishes_every_progress_state() {
    let doc = demo_snapshot();
    let region_id = doc.regions.get(0).expect("example region").id.as_str().to_owned();
    let element_id = element_id(doc.elements.get(0).expect("example element")).to_owned();
    let node_id = doc.nodes.iter().next().expect("example node").id.as_str().to_owned();
    for (quality, expected_id) in [
        (RegionVisualQuality::Unmeshed, "unmeshed"),
        (RegionVisualQuality::Coarse, "coarse"),
        (RegionVisualQuality::Refined, "refined"),
        (RegionVisualQuality::Final, "final"),
    ] {
        let visual = Fem2dLiveVisual {
            region_quality: [(region_id.clone(), quality)].into_iter().collect(),
            assembling_element_ids: vec![element_id.clone()],
            fields: [NodeLiveField { node_id: node_id.clone(), displacement: [0.01, -0.02], residual: [3.0, -4.0], reaction: [-3.0, 4.0], contour: 5.0, mode_shape: [0.2, -0.1] }].into_iter().collect(),
            state: if quality == RegionVisualQuality::Final { FemVisualState::ValidatedFinal } else { FemVisualState::Coarse },
            residual_norm: 5.0,
            tolerance: 1e-8,
            progress_completed: 1,
            progress_total: 2,
            converged: quality == RegionVisualQuality::Final,
            validated_final: quality == RegionVisualQuality::Final,
        };
        let encoded = semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(fem2d_live_visual_layers(&doc, &visual)));
        assert!(encoded.contains(&format!("region-quality-{expected_id}-{region_id}")));
        assert!(encoded.contains("assembling-"));
        assert!(encoded.contains("displacement-field-"));
        assert!(encoded.contains("residual-field-"));
        assert!(encoded.contains(if quality == RegionVisualQuality::Final { "solve-status-validated-final" } else { "solve-status-unconverged" }));
    }
}

#[semio_framework_async_macros::async_test]
async fn model_visual_language_includes_load_and_support_glyphs() {
    let doc = demo_snapshot();
    let encoded = semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(fem2d_structure_layers(&doc, "#38bdf8", "#94a3b8", "#f97316")));
    assert!(encoded.contains("support-"));
    assert!(encoded.contains("load-"));
}

#[semio_framework_async_macros::async_test]
async fn live_visual_replay_is_deterministic_and_bounded() {
    use std::time::Instant;
    let doc = demo_snapshot();
    let node_id = doc.nodes.iter().next().expect("example node").id.as_str().to_owned();
    let visual = Fem2dLiveVisual {
        region_quality: doc.regions.iter().rev().enumerate().map(|(index, region)| (region.id.as_str().to_owned(), if index % 2 == 0 { RegionVisualQuality::Coarse } else { RegionVisualQuality::Refined })).collect(),
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
    let exact = "x".repeat(FEM2D_MOUNTED_VISUAL_PAGE_BYTES);
    for index in 0..FEM2D_MOUNTED_VISUAL_PAGE_COUNT {
        output.admit_page(index).expect("admit packet page");
        output.write_str(&exact).expect("fill the admitted page exactly");
    }
    let page_identities = |packet: &Fem2dFixedPacketPages| {
        packet.pages.each_ref().map(|page| page.as_ref().expect("page retained").backing_identity())
    };
    let before = page_identities(&output);
    assert!(output.write_char('x').is_err(), "maximum plus one must reject before allocating");
    assert_eq!(page_identities(&output), before);
    assert_eq!(output.len, FEM2D_MOUNTED_VISUAL_PAGE_COUNT * FEM2D_MOUNTED_VISUAL_PAGE_BYTES);

    for returned in 1..=FEM2D_MOUNTED_VISUAL_PAGE_COUNT {
        assert_eq!(
            output.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES),
            (returned == FEM2D_MOUNTED_VISUAL_PAGE_COUNT, 1, FEM2D_MOUNTED_VISUAL_PAGE_BYTES),
        );
    }
    assert!(output.terminal_is_empty());
}

#[test]
fn fem2d_visual_job_maximum_plus_one_rejects_before_owner_transfer() {
    let value = Fem2dSnapshotValue {
        nodes: (0..=FEM2D_VISUAL_MAXIMUM_NODES).map(|index| FemNodeValue { id: format!("n{index}"), x: index as f64, y: 0.0 }).collect(),
        ..Default::default()
    };
    let doc = Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(value).expect("admit a document exceeding only the visual limit"));
    assert_eq!(doc.nodes.len(), FEM2D_VISUAL_MAXIMUM_NODES + 1);
    let before = doc.nodes.identity();
    let mut job = Fem2dVisualJob::new(visual_freshness(19));
    assert_eq!(
        job.step_one(&doc, &Fem2dLiveVisual::default(), visual_freshness(19)),
        Err(b"fem2d.visual-maximum-plus-one".to_vec()),
    );
    assert_eq!(before, doc.nodes.identity());
    assert_eq!(job.stage(), Fem2dVisualJobStage::ReserveSnapshot);
    assert!(job.terminal_is_empty(), "refusal precedes snapshot or page acquisition");
}

#[test]
fn mounted_visual_complete_field_pages_match_independent_json_and_close_one_page() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧵️session/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let count = fixture["visual"]["fields"].as_u64().unwrap() as usize;
    let page_count = fixture["visual"]["pageCapacity"].as_u64().unwrap() as usize;
    assert_eq!(semio_ui_scene::CANVAS2D_SNAPSHOT_PAGE_CAPACITY, page_count);
    assert_eq!(CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY, fixture["visual"]["pageBytes"].as_u64().unwrap() as usize);
    let doc = empty_snapshot();
    let visual = Fem2dLiveVisual { fields: (0..count).map(|index| NodeLiveField { node_id: format!("n{index}"), displacement: [index as f64 * 1e-6, -1e-6], residual: [0.0; 2], reaction: [0.0; 2], contour: index as f64 * 1e-6, mode_shape: [0.0; 2] }).collect(), state: FemVisualState::ValidatedFinal, converged: true, validated_final: true, ..Default::default() };
    let freshness = visual_freshness(791);
    let mut job = Fem2dVisualJob::new(freshness);
    assert!((0..100_000).any(|_| job.step_one(&doc, &visual, freshness).unwrap()));
    let mut lease = job.take_complete().unwrap();
    let snapshot = published_snapshot(&doc, &lease);
    assert_eq!(usize::from(snapshot.page_count), page_count);
    let packet = packet_pages(snapshot).concat();
    assert!(packet.len() <= fixture["visual"]["maximumBytes"].as_u64().unwrap() as usize);
    let mut independent_ids = std::collections::HashSet::new();
    for line in std::str::from_utf8(&packet).expect("UTF-8 packet").lines() {
        let independent: serde_json::Value = serde_json::from_str(line).expect("independent layer decode");
        let own = parse_bytes(line.as_bytes(), JsonMemberPolicy::Reject).expect("production layer decode");
        let id = independent["id"].as_str().expect("layer id");
        assert_eq!(own.get("id").and_then(semio_framework_pack_json::Value::as_str), Some(id));
        assert!(independent_ids.insert(id.to_owned()), "packet repeats layer id {id}");
    }
    let fields_per_node = fixture["visual"]["entriesPerField"].as_u64().unwrap() as usize;
    assert_eq!(independent_ids.len(), count * fields_per_node + 4, "the empty structure contributes only four accessibility labels");
    for kind in ["displacement-field-", "residual-field-", "reaction-field-", "contour-field-", "mode-field-"] {
        assert_eq!(independent_ids.iter().filter(|id| id.starts_with(kind)).count(), count, "every admitted field publishes {kind}");
        assert!((0..count).all(|index| independent_ids.contains(&format!("{kind}n{index}"))));
    }
    assert_eq!(lease.close_step(0), (false, 0, 0));
    assert!(canvas2d_snapshot_with_page(snapshot, 0, |_| ()).is_ok());
    close_visual(lease, page_count);
    assert!(canvas2d_snapshot_with_page(snapshot, 0, |_| ()).is_err());
}

#[test]
fn fem2d_visual_job_stale_cancel_fault_and_device_close_preserve_last_valid() {
    let doc = empty_snapshot();
    let visual = Fem2dLiveVisual::default();
    let mut stale = Fem2dVisualJob::new(visual_freshness(19));
    let mut turns = 0;
    while stale.stage() != Fem2dVisualJobStage::ValidateFreshness && turns < 512 {
        stale.step_one(&doc, &visual, visual_freshness(19)).expect("bounded production step");
        turns += 1;
    }
    assert_eq!(stale.stage(), Fem2dVisualJobStage::ValidateFreshness);
    assert_eq!(stale.step_one(&doc, &visual, visual_freshness(23)), Err(b"fem2d.visual-stale-before-publication".to_vec()));
    assert!(stale.take_complete().is_none());
    while !stale.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0 && turns < 2_048 {
        turns += 1;
    }
    assert!(stale.terminal_is_empty());

    let current = sealed_visual(&doc, &visual);
    let current_snapshot = published_snapshot(&doc, &current);
    let mut rejected = Fem2dVisualJob::new(visual_freshness(29));
    let mut closes = 0;
    while !rejected.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0 && closes < 2_048 {
        closes += 1;
    }
    assert!(rejected.terminal_is_empty(), "a never-started visual job closes down to its empty shell");
    assert!(canvas2d_snapshot_with_page(current_snapshot, 0, |_| ()).is_ok());
    close_visual(current, usize::from(current_snapshot.page_count));
    assert!(canvas2d_snapshot_with_page(current_snapshot, 0, |_| ()).is_err());
}

#[test]
fn fem2d_visual_job_replay_accessibility_and_each_step_are_bounded() {
    let doc = empty_snapshot();
    let visual = Fem2dLiveVisual { state: FemVisualState::ValidatedFinal, validated_final: true, progress_completed: 1, progress_total: 1, tolerance: 1e-8, ..Default::default() };
    let first = sealed_visual(&doc, &visual);
    let second = sealed_visual(&doc, &visual);
    let first_snapshot = published_snapshot(&doc, &first);
    let second_snapshot = published_snapshot(&doc, &second);
    let pages = packet_pages(first_snapshot);
    assert_eq!(pages, packet_pages(second_snapshot));
    let payload = String::from_utf8(pages.concat()).expect("UTF-8 packet");
    for text in ["accessible-en", "accessible-de", r#""fontFamily":"Share Tech Mono""#, r#""fill":{"color":["#, "Cancel Retry Discard", "Abbrechen Wiederholen Verwerfen"] {
        assert!(payload.contains(text), "visual packet is missing {text}");
    }
    close_visual(first, usize::from(first_snapshot.page_count));
    close_visual(second, usize::from(second_snapshot.page_count));

    let mut job = Fem2dVisualJob::new(visual_freshness(19));
    assert!(!job.step_one(&doc, &visual, visual_freshness(19)).expect("bounded reservation step"));
    assert!((0..2_048).any(|_| job.close_step(FEM2D_MOUNTED_VISUAL_PAGE_BYTES).0));
    assert!(job.terminal_is_empty());
}

/// ⚖️ LAW: the model body draws the gumball its window config describes: a handle switched off in the config is off in the `meta:gumball` layer the host paints, and a second window config draws its own set.
#[test]
fn the_model_body_draws_the_gumball_its_window_config_describes() {
    use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::canvas_gesture::FEM2D_UTILITY_TRANSFORM;
    use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::Fem2dGumballConfig;
    let doc = demo_snapshot();
    let interaction = Fem2dInteractionSnapshot { selected_ids: vec!["scope:1:4:node2:n1".into()], ..Default::default() };
    let flags = |config: &Fem2dGumballConfig| {
        let node = render_with_progress(&doc, &Viewport2d::default(), None, &interaction, Some("model-left"), FEM2D_UTILITY_TRANSFORM, config, semio_plugin::Locale::En).expect("model surface");
        let layers: serde_json::Value = serde_json::from_str(&canvas_scene(&node).layers_json).expect("layers json");
        let meta = layers.as_array().expect("layers").iter().find(|layer| layer["id"] == "meta:gumball").expect("gumball meta layer").clone();
        ["moveAxes", "rotate", "scaleAxes", "scaleUniform"].map(|key| meta["gumball"]["config"][key].as_bool().expect("flag"))
    };
    assert_eq!(flags(&Fem2dGumballConfig::default()), [true; 4]);
    let mut off = Fem2dGumballConfig::default();
    off.set_flag("rotate", Some(false)).expect("rotate is a flag");
    off.set_flag("scaleUniform", Some(false)).expect("scaleUniform is a flag");
    assert_eq!(flags(&off), [true, false, true, false]);
}
