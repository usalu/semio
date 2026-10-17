use super::*;

use semio_framework_ui_prepared::testing::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../♻️physical-job-close/🎟️prepared/🦀️.rs"]
mod physical_job_close_laws;
use semio_framework_job::{root_cancel_token, Generation, InteractiveStage, JobOutcomeBorrow, JobOutcomeKind, OperationId, StepBudget};

#[test]
fn prepared_animation_receipt_measures_actual_normal_and_overlay_primitives() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../../🖌️render/⏱️schedule/🧪️tests/🧫️fixtures/🎞️animation/🔣️.json")).unwrap();
    for overlay_route in [false, true] {
        for row in fixture["primitives"].as_array().unwrap() {
            let mut draw = DrawList::default();
            let mut overlay = DrawList::default();
            for (list, field) in [(&mut draw, "draw"), (&mut overlay, "overlay")] {
                for kind in row[field].as_array().unwrap() {
                    let mut instance = semio_framework_ui_draw_contract::UiInstance::solid([0.0, 0.0, 24.0, 24.0], semio_framework_ui_theme::Rgba::new(1.0, 1.0, 1.0, 1.0));
                    instance.params[2] = kind.as_f64().unwrap() as f32;
                    if overlay_route {
                        list.layers[0].overlay_ui_instances.push(instance);
                    } else {
                        list.layers[0].ui_instances.push(instance);
                    }
                }
            }
            let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, Some(overlay), 0.0), 1);
            assert!(matches!(drive_preparation_until_terminal(&mut job), Some(JobOutcomeKind::Complete)));
            let mut packet = job.take_packet().expect("measured packet");
            assert_eq!(packet.has_animated_primitives(), row["active"].as_bool().unwrap(), "{}", row["id"]);
            while !close_packet_step(&mut packet) {}
            while !close_job_step(&mut job) {}
        }
    }
}

fn packet(revision: u64, generation: u64) -> PreparedRenderPacket {
    let mut directives = PreparedRenderDirectives::default();
    assert!(directives.try_push(RenderDirective::PreservePreviousOnFailure).is_ok());
    let packet = PreparedRenderPacket {
        has_animated_primitives: false,
        scene_revision: revision,
        preview_generation: generation,
        damage: PreparedRenderScissors::default(),
        clips: PreparedRenderScissors::default(),
        directives,
        uploads: PreparedRenderUploads::default(),
        evictions: PreparedRenderEvictions::default(),
        draw: DrawList::default(),
        overlay: None,
        commands: PreparedRenderCommandPages::default(),
        time_seconds: 0.0,
        usage: PreparedRenderUsage::default(),
        limits: PreparedRenderLimits::default(),
        permit: PreparedRenderProcessPermit::try_reserve(0, 0),
        retirement_phase: 0,
        abandonment_slot: u8::MAX,
    };
    match packet.try_arm_abandonment() {
        Ok(packet) => packet,
        Err(_) => panic!("test packet abandonment slot"),
    }
}
















/// 🌑️ LAW: the bounded caster prepass completes before every receiver, then the color pass
/// keeps React's textured underlay → opaque → lines → translucent order.
#[test]
fn an_enabled_shadow_pass_measures_every_caster_before_its_receivers() {
    let _guard = prepared_process_guard();
    use semio_framework_ui_scene::math::{Instance3d, LineDraw3d, LineVertex3d, SceneCurvilinear3d, SceneDraw3d, SceneMaterialDraw3d, SceneMaterialKind3d, ScenePass3d, SceneShadowRole3d, TexturedDraw3d, TexturedInstance3d};
    let mut draw = DrawList::default();
    let instance = Instance3d { component_source: None, id: String::new(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() };
    draw.push_scene_pass(ScenePass3d {
        viewport: [0.0, 0.0, 100.0, 40.0],
        shadow: semio_framework_ui_scene::math::SceneShadow3d { enabled: true, ..Default::default() },
        shadow_draws: vec![SceneDraw3d { mesh_key: "caster".into(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: SceneShadowRole3d { casts: true, receives: true } }],
        draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: Default::default() }],
        translucent_draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance.clone()], shadow_role: Default::default() }],
        material_draws: vec![
            SceneMaterialDraw3d { mesh_key: "painted".into(), mesh_version: 3, first_index: 0, index_count: u32::MAX, instances: vec![instance.clone()], material: SceneMaterialKind3d::Painted { texture_key: "paint-map".into() }, translucent: false },
            SceneMaterialDraw3d {
                mesh_key: "celebrated".into(),
                mesh_version: 4,
                first_index: 0,
                index_count: u32::MAX,
                instances: vec![instance.clone()],
                material: SceneMaterialKind3d::Celebration { stops: [[1.0; 4]; 3], angle: 0.5 },
                translucent: true,
            },
        ],
        line_draws: vec![LineDraw3d { vertices: vec![LineVertex3d { position: [0.0, 0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] }] }],
        textured_draws: vec![TexturedDraw3d { instances: vec![TexturedInstance3d { texture_key: String::new(), model: instance.model, background: [0.0; 4], appearance: [0.85, 0.0, 0.0, 0.0] }] }],
        curvilinear: Some(SceneCurvilinear3d { fov_radians: 2.0, strength: 0.75 }),
        ..Default::default()
    });

    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut order = Vec::new();
    for _ in 0..64 {
        let label = match cursor {
            DrawMeasureCursor::PassShadowBegin(..) | DrawMeasureCursor::PassShadowInstance { .. } => "shadow",
            DrawMeasureCursor::PassTextured { .. } | DrawMeasureCursor::PassTexturedInstance { .. } | DrawMeasureCursor::PassTexturedKey { .. } => "textured",
            DrawMeasureCursor::PassDraw { translucent, .. } | DrawMeasureCursor::PassDrawKey { translucent, .. } | DrawMeasureCursor::PassInstance { translucent, .. } | DrawMeasureCursor::PassInstanceKey { translucent, .. } => {
                if translucent {
                    "translucent"
                } else {
                    "opaque"
                }
            }
            DrawMeasureCursor::PassMaterial { translucent, .. }
            | DrawMeasureCursor::PassMaterialMeshKey { translucent, .. }
            | DrawMeasureCursor::PassMaterialTextureKey { translucent, .. }
            | DrawMeasureCursor::PassMaterialInstance { translucent, .. }
            | DrawMeasureCursor::PassMaterialInstanceKey { translucent, .. } => {
                if translucent {
                    "material-translucent"
                } else {
                    "material-opaque"
                }
            }
            DrawMeasureCursor::PassLine { .. } | DrawMeasureCursor::PassLineVertex { .. } => "lines",
            DrawMeasureCursor::PassPostprocess { .. } => "world-postprocess",
            _ => "other",
        };
        if order.last() != Some(&label) && label != "other" {
            order.push(label);
        }
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() || matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    assert_eq!(order, vec!["shadow", "textured", "material-opaque", "opaque", "lines", "translucent", "material-translucent", "world-postprocess"], "the image-space remap runs after every world color receiver");
}

/// 🌐️ LAW: the prepared scalar ladder owns exactly one grid item after textured references
/// and before opaque geometry, while ordinary overlays retain the line-vertex lane.
#[test]
fn a_procedural_grid_is_one_prepared_scalar_between_textures_and_opaque_geometry() {
    let _guard = prepared_process_guard();
    use semio_framework_ui_scene::math::{Instance3d, LineDraw3d, LineVertex3d, ProceduralGrid3d, SceneDraw3d, ScenePass3d, TexturedDraw3d, TexturedInstance3d};
    let instance = Instance3d { component_source: None, id: String::new(), model: Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() };
    let grid = ProceduralGrid3d { plane_z: 0.001, camera_plane_projection: [2.0, -3.0, 0.001], cell_size: 1.0, fade_distance: 8.0, cell_color: [0.2, 0.3, 0.4] };
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d {
        procedural_grid: Some(grid),
        textured_draws: vec![TexturedDraw3d { instances: vec![TexturedInstance3d { texture_key: String::new(), model: instance.model, background: [0.0; 4], appearance: [1.0, 0.0, 0.0, 0.0] }] }],
        draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance], shadow_role: Default::default() }],
        line_draws: vec![LineDraw3d { vertices: vec![LineVertex3d { position: [0.0; 3], color: [1.0; 4] }, LineVertex3d { position: [1.0, 0.0, 0.0], color: [1.0; 4] }] }],
        ..Default::default()
    });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut measured = Vec::new();
    for _ in 0..64 {
        let current = cursor;
        let Some(usage) = PreparedRenderJob::next_draw_usage(&draw, &mut cursor) else { break };
        measured.push((current, usage));
        if matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    let grid_rows: Vec<_> = measured.iter().enumerate().filter(|(_, (cursor, _))| matches!(cursor, DrawMeasureCursor::PassGrid { pass: 0 })).collect();
    assert_eq!(grid_rows.len(), 1, "one retained grid becomes exactly one prepared scalar");
    assert_eq!(grid_rows[0].1 .1, PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<ProceduralGrid3d>(), ..Default::default() }, "the grid scalar owns its exact retained bytes in one cancellable step");
    let textured = measured.iter().position(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassTexturedInstance { .. })).expect("textured reference scalar");
    let grid_index = grid_rows[0].0;
    let opaque = measured.iter().position(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassInstance { translucent: false, .. })).expect("opaque instance scalar");
    let line_vertices = measured.iter().filter(|(cursor, _)| matches!(cursor, DrawMeasureCursor::PassLineVertex { .. })).count();
    assert!(textured < grid_index && grid_index < opaque, "textured references precede the grid and opaque geometry follows it");
    assert_eq!(line_vertices, 2, "ordinary overlays retain the line-vertex lane");
}

#[test]
fn ellipse_scene_pass_snapshots_backdrop_clears_inside_then_composites_last() {
    use semio_framework_ui_scene::math::{ScenePass3d, SceneViewportMask3d};
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d { viewport_mask: SceneViewportMask3d::Ellipse, clear_color: Some([0.1, 0.2, 0.3, 0.7]), ..Default::default() });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut scalars = Vec::new();
    for _ in 0..8 {
        let Some(_) = PreparedRenderJob::next_draw_usage(&draw, &mut cursor) else { break };
        scalars.push(cursor);
        if matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
    assert_eq!(scalars[0], DrawMeasureCursor::PassBackdropSnapshot { pass: 0 });
    assert_eq!(scalars[1], DrawMeasureCursor::PassSceneClear { pass: 0 });
    assert!(scalars.contains(&DrawMeasureCursor::PassPostprocess { pass: 0 }));
    assert_eq!(scalars.iter().filter(|cursor| matches!(cursor, DrawMeasureCursor::PassPostprocess { .. })).count(), 1);

    let mut rectangular = DrawList::default();
    rectangular.push_scene_pass(ScenePass3d::default());
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    let mut postprocess = 0;
    while PreparedRenderJob::next_draw_usage(&rectangular, &mut cursor).is_some() && !matches!(cursor, DrawMeasureCursor::Complete) {
        postprocess += usize::from(matches!(cursor, DrawMeasureCursor::PassPostprocess { .. } | DrawMeasureCursor::PassBackdropSnapshot { .. } | DrawMeasureCursor::PassSceneClear { .. }));
    }
    assert_eq!(postprocess, 0, "ordinary rectangular worlds add no capture, clear, or postprocess scalar");
}



#[test]
fn a_disabled_shadow_never_publishes_a_gpu_shadow_scalar() {
    let _guard = prepared_process_guard();
    use semio_framework_ui_scene::math::{Instance3d, SceneDraw3d, ScenePass3d};
    let instance = Instance3d { component_source: None, id: String::new(), model: Instance3d::model_from_trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() };
    let mut draw = DrawList::default();
    draw.push_scene_pass(ScenePass3d { draws: vec![SceneDraw3d { mesh_key: String::new(), mesh_version: 0, instances: vec![instance], shadow_role: Default::default() }], ..Default::default() });
    let mut cursor = DrawMeasureCursor::PassHeader(0);
    for _ in 0..32 {
        assert!(!matches!(cursor, DrawMeasureCursor::PassShadowBegin(..) | DrawMeasureCursor::PassShadowInstance { .. }));
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() || matches!(cursor, DrawMeasureCursor::Complete) {
            break;
        }
    }
}

/// 🎟️ A raster producer must be bound to the frame it is published into BEFORE it is pushed, and the
/// job's refusal must be readable — the held fault loan carries empty native detail pages here, so
/// [`PreparedRenderJob::fault`] is the only way a driver can name it
/// (`📓️w7b-presenter-one-frame-per-boot.md` §1).
#[test]
fn an_unbound_raster_producer_refuses_its_prepared_job_with_a_readable_fault() {
    let _guard = prepared_process_guard();
    let (unbound, _) = PreparedRasterProducer::try_admit("unbound".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(unbound).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    let (outcome,original_retained_receipt) = observe_prepared_terminal(&mut job, OperationId(1), Generation(3), StepBudget::new(4, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(outcome, Some(JobOutcomeKind::Fault)));
    assert_eq!(job.fault(), Some("raster producer generation is stale"));
    while !close_job_step(&mut job) {}

    let (mut bound, _) = PreparedRasterProducer::try_admit("bound".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    assert!(bound.bind_frame_generation(3));
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(bound).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let (outcome,original_retained_receipt) = observe_prepared_step(&mut job, OperationId(1), Generation(3), StepBudget::new(4, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(outcome, None | Some(JobOutcomeKind::Yield)));
    assert_eq!(job.fault(), None);
    while !close_job_step(&mut job) {}
}



#[test]
fn zero_fuel_and_expired_deadline_advance_no_raster_page_or_allocation() {
    let _guard = prepared_process_guard();
    let (mut producer, _) = PreparedRasterProducer::try_admit("governed".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
    assert!(producer.bind_frame_generation(3));
    let source_pointer = producer.original_source().as_ptr();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(producer).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;

    let (zero,original_retained_receipt) = observe_prepared_step(&mut job, OperationId(1), Generation(3), StepBudget::new(0, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);

    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(zero, None | Some(JobOutcomeKind::Yield)));
    let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
    assert_eq!(retained.original_source().as_ptr(), source_pointer);
    assert_eq!(retained.original_page_count(), 0);

    let (expired,original_retained_receipt) = observe_prepared_step(&mut job, OperationId(1), Generation(3), StepBudget::new(1, 1,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);

    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(expired, None | Some(JobOutcomeKind::Yield)));
    let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
    assert_eq!(retained.original_source().as_ptr(), source_pointer);
    assert_eq!(retained.original_page_count(), 0);

    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn cancellation_retires_large_upload_incrementally_before_terminal_empty() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4_096], width: 64, height: 64 }).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    assert!(!close_job_step(&mut job));
    assert!(!job.terminal_is_empty());
    let mut turns = 1;
    while !close_job_step(&mut job) {
        turns += 1;
        assert!(turns < 5_000);
    }
    assert!(turns > 4_096);
    assert!(job.terminal_is_empty());
}

fn assert_send<T: Send>() {}

#[test]
fn prepared_packet_is_send_owned_data() {
    let _guard = prepared_process_guard();
    assert_send::<PreparedRenderPacket>();
    assert_send::<PreparedRenderJob>();
    assert_send::<PreparedRenderReceiver>();
    assert_send::<PreparedRenderGate>();
    let packet = packet(7, 3);
    let identity = std::thread::spawn(move || (packet.scene_revision, packet.preview_generation)).join().expect("worker packet");
    assert_eq!(identity, (7, 3));
}

#[test]
fn receiver_survives_worker_ownership_of_the_job() {
    let _guard = prepared_process_guard();
    let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
    let receiver = job.receiver().expect("prepared receiver clone");
    std::thread::spawn(move || {
        let mut job = job;
        let mut preview = 0;
        loop {
            let (outcome,original_retained_receipt) = observe_prepared_step(&mut job, OperationId(1), Generation(3), StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
            assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
            if outcome.is_some_and(|kind| matches!(kind, JobOutcomeKind::Complete | JobOutcomeKind::Cancelled | JobOutcomeKind::Fault)) {
                assert!(matches!(outcome, Some(JobOutcomeKind::Complete)));
                break;
            }
        }
    })
    .join()
    .expect("worker preparation");
    let packet = receiver.take_latest().expect("prepared packet handoff");
    assert_eq!((packet.scene_revision(), packet.preview_generation()), (7, 3));
    assert!(receiver.take_latest().is_none());
    drop(packet);
    drain_abandoned_preparations();
}

#[test]
fn preparation_yields_at_the_configured_item_budget() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers.extend((0..4).map(|_| DrawLayer::default()));
    let input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    let (first,original_retained_receipt) = observe_prepared_step(&mut job, OperationId(1), Generation(3), StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(first, None | Some(JobOutcomeKind::Yield)));
    drop(job);
    drain_abandoned_preparations();
}

#[test]
fn preparation_completes_across_bounded_steps() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers.extend((0..2).map(|_| DrawLayer::default()));
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, Some(JobOutcomeKind::Complete)));
    let mut packet = job.take_packet().expect("prepared packet");
    assert_eq!((packet.scene_revision, packet.preview_generation), (7, 3));
    while !close_packet_step(&mut packet) {}
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn preparation_rejects_a_stale_generation_before_publication() {
    let _guard = prepared_process_guard();
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 2, DrawList::default(), None, 0.0), 8);
    let mut preview = 0;
    let (outcome,original_retained_receipt) = observe_prepared_terminal(&mut job, OperationId(1), Generation(3), StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(outcome, Some(JobOutcomeKind::Fault)));
    assert!(job.take_packet().is_none());
    drop(job);
    drain_abandoned_preparations();
}

#[semio_framework_async_macros::async_test]
async fn preparation_observes_cancellation_without_replacing_a_packet() {
    let cancel = root_cancel_token();
    cancel.cancel().await;
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 8);
    let mut preview = 0;
    let (outcome,original_retained_receipt) = observe_prepared_step(&mut job, OperationId(1), Generation(3), StepBudget::new(100, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel, now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
    assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(matches!(outcome, Some(JobOutcomeKind::Cancelled)));
    assert!(job.take_packet().is_none());
    drop(job);
    drain_abandoned_preparations();
}

#[test]
fn stale_packet_rejection_preserves_the_last_valid_packet() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    assert!(gate.acknowledge_presented(witness).expect("first presenter acknowledgement").is_empty());
    let stale = packet(6, 3);
    assert!(matches!(gate.validate(&stale, 7, 3), Err(PreparedRenderRejection::StaleRevision { .. })));
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
}

#[test]
fn generation_rejection_happens_before_presentation() {
    let _guard = prepared_process_guard();
    let gate = PreparedRenderGate::default();
    assert!(matches!(gate.validate(&packet(7, 2), 7, 3), Err(PreparedRenderRejection::StaleGeneration { .. })));
}

#[test]
fn device_loss_retains_the_last_valid_packet() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
    let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
    assert_eq!(gate.retain_after_device_loss(), Some((7, 3)));
}

#[test]
fn presenter_ack_is_exact_one_shot_and_preserves_old_until_acknowledged() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
    let second = gate.stage_presented(packet(8, 4)).ok().expect("second presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "candidate is not visible before acknowledgement");
    let stale = PreparedPresenterWitness { sequence: second.sequence.saturating_add(1), scene_revision: second.scene_revision, preview_generation: second.preview_generation };
    assert!(gate.acknowledge_presented(stale).is_err());
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    let duplicate = PreparedPresenterWitness { sequence: second.sequence, scene_revision: second.scene_revision, preview_generation: second.preview_generation };
    let mut replacement = gate.acknowledge_presented(second).expect("exact second acknowledgement");
    assert_eq!(gate.last_valid_identity(), Some((8, 4)));
    assert_eq!(replacement.previous.as_ref().map(|packet| (packet.scene_revision, packet.preview_generation)), Some((7, 3)));
    assert!(gate.acknowledge_presented(duplicate).is_err(), "duplicate acknowledgement is stale after publication");
    let mut previous = replacement.take_previous().expect("old last-valid owner");
    while !close_packet_step(&mut previous) {}
    assert!(previous.retirement_is_empty());
}

#[test]
fn accepted_a_stale_b_abort_and_accepted_c_preserve_exact_presenter_owners() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
    let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
    let _missing = gate.stage_presented(packet(8, 4)).ok().expect("pending presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    let mut stale = gate.abort_pending().expect("exact stale B packet handback");
    assert_eq!((stale.scene_revision, stale.preview_generation), (8, 4));
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "aborting B preserves accepted A");

    let third = gate.stage_presented(packet(9, 5)).ok().expect("successor C presenter witness");
    assert_eq!(gate.last_valid_identity(), Some((7, 3)), "C remains private before acknowledgement");
    let mut replacement = gate.acknowledge_presented(third).expect("successor C acknowledgement");
    assert_eq!(gate.last_valid_identity(), Some((9, 5)), "C alone replaces A after exact acknowledgement");
    let mut first = replacement.take_previous().expect("accepted A owner handback");
    assert_eq!((first.scene_revision, first.preview_generation), (7, 3));
    while !close_packet_step(&mut stale) {}
    while !close_packet_step(&mut first) {}
    let mut third = gate.take_last_valid().expect("accepted C owner handback");
    while !close_packet_step(&mut third) {}
    while !gate.close_step() {}
    assert!(gate.terminal_is_empty());
}

#[test]
fn pending_presenter_witness_rejects_superseding_packet_with_exact_owner() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let _pending = gate.stage_presented(packet(7, 3)).ok().expect("pending presenter witness");
    let mut superseding = packet(8, 4);
    assert!(superseding.uploads.try_push(PreparedRenderUpload::GlyphAtlas { pixels: vec![7; 16_385], width: 1, height: 1 }).is_ok());
    let pixels = match superseding.uploads.get(0) {
        Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) => pixels.as_ptr(),
        _ => unreachable!(),
    };
    let mut returned = match gate.stage_presented(superseding) {
        Ok(_) => panic!("second presenter witness must fail closed"),
        Err(packet) => packet,
    };
    assert_eq!((returned.scene_revision, returned.preview_generation), (8, 4));
    assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels: returned_pixels, .. }) if returned_pixels.as_ptr() == pixels));
    assert!(!close_packet_step(&mut returned), "one close grant retires only one admitted pixel page");
    assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) if pixels.len() == 1));
}

#[test]
fn gate_close_requires_pending_and_last_valid_packet_handback_before_terminal_scalars() {
    let _guard = prepared_process_guard();
    let mut gate = PreparedRenderGate::default();
    let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
    let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
    assert!(!gate.close_step(), "last-valid owner prevents gate terminalization");
    let mut last = gate.take_last_valid().expect("last-valid owner handback");
    while !close_packet_step(&mut last) {}
    assert!(!gate.close_step(), "first scalar grant retires only the sequence");
    assert!(gate.close_step(), "second scalar grant publishes the terminal witness");
    assert!(gate.terminal_is_empty());
    assert!(matches!(gate.validate(&packet(8, 4), 8, 4), Err(PreparedRenderRejection::Closing)));
}

#[test]
fn upload_byte_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    input.limits.max_upload_bytes = 3;
    assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4], width: 2, height: 2 }).is_ok());
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, Some(JobOutcomeKind::Fault)));
    assert!(job.take_packet().is_none());
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn eviction_byte_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    input.limits.max_upload_bytes = 3;
    assert!(input.try_push_eviction(PreparedRenderEviction::Mesh { key: "mesh".into() }).is_ok());
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, Some(JobOutcomeKind::Fault)));
    assert!(job.take_packet().is_none());
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn draw_item_cap_faults_before_packet_publication() {
    let _guard = prepared_process_guard();
    let mut draw = DrawList::default();
    draw.layers[0].ui_instances.push(semio_framework_ui_draw_contract::UiInstance::solid([0.0; 4], semio_framework_ui_theme::Rgba::new(0.0, 0.0, 0.0, 0.0)));
    let mut input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
    input.limits.max_draw_items = 0;
    let mut job = PreparedRenderJob::new(input, 64);
    let outcome = drive_preparation_until_terminal(&mut job);
    assert!(matches!(outcome, Some(JobOutcomeKind::Fault)));
    assert!(job.take_packet().is_none());
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

#[test]
fn input_drop_hands_back_exact_process_permits_for_incremental_close() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    drop(input);
    assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    let mut turns = 0;
    while !close_abandoned_input_step() {
        turns += 1;
        assert!(turns < 128);
    }
    assert!(turns > 4);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn worker_panic_hands_back_the_exact_job_and_mailbox_owners() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _owner = job;
        panic!("hostile worker interruption");
    }));
    assert!(result.is_err());
    let mut turns = 0;
    while !close_abandoned_job_step() {
        turns += 1;
        assert!(turns < 256);
    }
    assert!(turns > 4);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    assert!(PREPARED_RENDER_MAILBOX.iter().all(|slot| slot.packet.load(Ordering::Acquire).is_null()));
}

#[test]
fn packet_drop_retires_nested_backings_and_permit_scalars_separately() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let mut owner = packet(7, 3);
    owner.draw.push_solid([0.0, 0.0, 8.0, 8.0], semio_framework_ui_theme::Rgba::new(1.0, 0.0, 0.0, 1.0));
    drop(owner);
    let mut turns = 0;
    while !close_abandoned_packet_step() {
        turns += 1;
        assert!(turns < 256);
    }
    assert!(turns > 8);
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
}

#[test]
fn fixed_command_pages_reject_max_plus_one_without_consuming_the_owner() {
    let _guard = prepared_process_guard();
    let mut commands = PreparedRenderCommandPages::default();
    for source in 0..PREPARED_RENDER_COMMAND_PAGES * PREPARED_RENDER_COMMAND_PAGE_ITEMS {
        let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source, digest: source as u64, draw_cursor: Some(DrawMeasureCursor::Complete), packet_overlay: false };
        assert!(commands.try_push(command).is_ok());
    }
    let rejected = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source: usize::MAX, digest: u64::MAX, draw_cursor: Some(DrawMeasureCursor::Complete), packet_overlay: true };
    let returned = match commands.try_push(rejected) {
        Ok(()) => panic!("command cap plus one must refuse"),
        Err(returned) => returned,
    };
    assert_eq!((returned.source, returned.digest, returned.packet_overlay), (usize::MAX, u64::MAX, true));
    let mut turns = 0;
    while !commands.close_step() {
        turns += 1;
    }
    assert!(turns >= PREPARED_RENDER_COMMAND_PAGES * PREPARED_RENDER_COMMAND_PAGE_ITEMS);
    assert!(commands.terminal_is_empty());
}

#[test]
fn tessellation_commands_retain_exact_scalar_and_overlay_cursors() {
    let _guard = prepared_process_guard();
    drain_abandoned_preparations();
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 4.0, 4.0], semio_framework_ui_theme::Rgba::new(0.0, 1.0, 0.0, 1.0));
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    let mut preview = 0;
    let mut steps = 0;
    loop {
        let (outcome,original_retained_receipt) = observe_prepared_step(&mut job, OperationId(41), Generation(3), StepBudget::new(1, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
        assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
        steps += 1;
        if outcome.is_some_and(|kind| matches!(kind, JobOutcomeKind::Complete | JobOutcomeKind::Cancelled | JobOutcomeKind::Fault)) {
            assert!(matches!(outcome, Some(JobOutcomeKind::Complete)));
            break;
        }
        assert!(steps < 128);
    }
    let mut packet = match job.take_packet() {
        Some(packet) => packet,
        None => panic!("prepared packet handoff"),
    };
    assert!((0..packet.commands.len())
        .filter_map(|index| packet.commands.get(index))
        .any(|command| { command.kind == PreparedRenderCommandKind::Tessellate && command.draw_cursor == Some(DrawMeasureCursor::LayerUi { layer: 0, item: 0, overlay: false }) && !command.packet_overlay }));
    while !close_packet_step(&mut packet) {}
    while !close_job_step(&mut job) {}
}

#[test]
fn prepared_measurement_retains_scene_and_overlay_raster_cursors() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).expect("retained Select/overlay raster fixture");
    let key = fixture["image"]["sharedKey"].as_str().expect("shared raster key");
    let mut draw = DrawList::default();
    draw.push_raster_quad(key, [0.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.begin_overlay_route();
    draw.push_raster_quad(key, [4.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.end_overlay_route();
    let mut cursor = DrawMeasureCursor::LayerHeader(0);
    let mut measured = Vec::new();
    for _ in 0..64 {
        let current = cursor;
        if PreparedRenderJob::next_draw_usage(&draw, &mut cursor).is_none() {
            break;
        }
        measured.push(current);
        if cursor == DrawMeasureCursor::Complete {
            break;
        }
    }
    assert!(measured.contains(&DrawMeasureCursor::LayerRaster { layer: 0, raster: 0, overlay: false }));
    assert!(measured.contains(&DrawMeasureCursor::LayerRaster { layer: 0, raster: 0, overlay: true }));
    let raster_order: Vec<_> = measured
        .iter()
        .filter_map(|cursor| match cursor {
            DrawMeasureCursor::LayerRaster { overlay, .. } => Some(*overlay),
            _ => None,
        })
        .collect();
    for row in fixture["image"]["draws"].as_array().expect("draw order rows") {
        let ordinal = usize::try_from(row["expectedOrdinal"].as_u64().expect("expected ordinal")).expect("ordinal fits");
        assert_eq!(raster_order.get(ordinal).copied(), Some(row["route"].as_str() == Some("overlay")));
    }
}

#[test]
fn prepared_packet_publishes_main_overlay_and_top_overlay_raster_owners() {
    let _guard = prepared_process_guard();
    let mut packet = packet(7, 3);
    packet.draw.push_raster_quad("main-image", [0.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.draw.begin_overlay_route();
    packet.draw.push_raster_quad("inline-overlay-image", [4.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.draw.end_overlay_route();
    let mut overlay = DrawList::default();
    overlay.push_raster_quad("top-overlay-image", [8.0, 0.0, 4.0, 4.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    packet.overlay = Some(overlay);
    let mut cursor = PreparedRasterKeepCursorV1::default();
    let mut keys = Vec::new();
    for _ in 0..64 {
        match packet.raster_keep_step(&mut cursor) {
            PreparedRasterKeepStepV1::Pending => {}
            PreparedRasterKeepStepV1::Key(key) => keys.push(key.to_owned()),
            PreparedRasterKeepStepV1::Complete => break,
        }
    }
    assert_eq!(keys, ["main-image", "inline-overlay-image", "top-overlay-image"]);
    while !close_packet_step(&mut packet) {}
}








#[test]
fn prepared_render_common_close_funds_declared_frontiers_without_losing_original_owners() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/📦️prepared-close/🔣️.json")).unwrap();
    for row in fixture["grants"].as_array().unwrap() {
        let grant = semio_framework_job::RetainedCloneGrant {
            maximum_items: row["items"].as_u64().unwrap() as usize,
            maximum_copy_bytes: row["copy"].as_u64().unwrap() as usize,
            maximum_capacity_bytes: row["capacity"].as_u64().unwrap() as usize,
            maximum_release_bytes: row["release"].as_u64().unwrap() as usize,
            maximum_depth: row["depth"].as_u64().unwrap() as usize,
        };
        let atlas = &fixture["atlas"];
        let pixels = vec![7; atlas["byteLength"].as_u64().unwrap() as usize];
        let mut pages = atlas_fixture_owner(atlas["width"].as_u64().unwrap() as u32, atlas["height"].as_u64().unwrap() as u32, atlas["channels"].as_u64().unwrap() as u8, pixels.len()).unwrap();
        atlas_fixture_page(&mut pages, &pixels, 0).unwrap();
        let mut retained_input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        retained_input.uploads.try_push(PreparedRenderUpload::GlyphAtlasPages { pixels: pages }).unwrap();
        let mut job = PreparedRenderJob::new(retained_input, 1);
        let input = job.input.as_ref().unwrap() as *const PreparedRenderInput;
        assert!(job.commands.take().unwrap().terminal_is_empty());
        assert_eq!(InteractiveJob::close_step(&mut job, grant), semio_framework_job::InteractiveJobCloseStep::Blocked);
        InteractiveJob::begin_close(&mut job);
        let outcome = InteractiveJob::close_step(&mut job, grant);
        let expected = semio_framework_job::RetainedCloneProgress { copied_items: row["expectedItems"].as_u64().unwrap() as usize, ..Default::default() };
        assert_eq!(outcome, semio_framework_job::InteractiveJobCloseStep::Pending { progress: expected });
        assert!(outcome.progress().fits(grant));
        assert_eq!(job.input.as_ref().unwrap() as *const PreparedRenderInput, input);
        assert_eq!(job.commands.is_some(), row["commandsRetained"].as_bool().unwrap());
        assert!(!PreparedRenderJob::terminal_is_empty(&job));
        assert!(InteractiveJob::next_close_release_byte_demand(&job).is_ok());
        let mut complete = false;
        for _ in 0..10_000 { if close_job_step(&mut job) { complete = true; break; } }
        assert!(complete);
        assert!(PreparedRenderJob::terminal_is_empty(&job));
        assert_eq!(InteractiveJob::close_step(&mut job, grant), semio_framework_job::InteractiveJobCloseStep::Complete { progress: semio_framework_job::RetainedCloneProgress::default() });
    }
}

#[test]
fn retained_codec_source_moves_once_and_retires_one_page_per_governed_step() {
    let _guard = prepared_process_guard();
    let decoded = vec![7; PREPARED_RASTER_PAGE_BYTES];
    let retained_source = vec![9; PREPARED_RASTER_PAGE_BYTES * 2];
    let retained_pointer = retained_source.as_ptr();
    let reservation = PreparedRasterReservation::try_reserve_source("retained-source".into(), retained_source.capacity()).expect("source workspace admitted before decode");
    let reservation = reservation.claim(4_096, 1).expect("decoded owner credited in addition to retained source");
    let (mut producer, _) = reservation.finalize(decoded, retained_source, 4_096, 1).expect("exact retained source owner");
    assert_eq!(producer.original_retained_source().as_ptr(), retained_pointer);
    assert!(producer.bind_frame_generation(3));

    let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
    assert!(input.try_push_raster_producer(producer).is_ok());
    let mut job = PreparedRenderJob::new(input, 1);
    let mut preview = 0;
    for expected in [PREPARED_RASTER_PAGE_BYTES * 2, PREPARED_RASTER_PAGE_BYTES * 2, PREPARED_RASTER_PAGE_BYTES, 0] {
        let (outcome,original_retained_receipt) = observe_prepared_step(&mut job, OperationId(11), Generation(3), StepBudget::new(1, 10,ui_contract::UI_WORKER_RETIREMENT_POLICY), root_cancel_token(), now_ms, &mut preview, ui_contract::UI_WORKER_RETIREMENT_POLICY);
        assert!(original_retained_receipt.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
        assert!(matches!(outcome, None | Some(JobOutcomeKind::Yield)));
        assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().original_retained_source().len(), expected);
    }
    assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().original_retained_source().as_ptr(), retained_pointer);
    while !close_job_step(&mut job) {}
    assert!(job.terminal_is_empty());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
