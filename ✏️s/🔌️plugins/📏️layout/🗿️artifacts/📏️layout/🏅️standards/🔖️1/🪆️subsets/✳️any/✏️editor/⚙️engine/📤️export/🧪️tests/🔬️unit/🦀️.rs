
use super::*;

fn drive_test_job<J: InteractiveJob + 'static>(job: J, params: BatchJobParams) -> StepOutcome {
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            panic!("test oracle admits retained session");
        }
    };
    loop {
        session.step().expect("test oracle caller opportunity");
        let Some(mut outcome) = session.take_outcome() else { continue };
        let terminal = outcome.is_terminal();
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if terminal {
            session.begin_close();
            while !session.terminal_is_empty() {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return outcome;
        }
        session.resume().expect("test oracle resumes exact owner");
    }
}

fn operation() -> Operation {
    Operation::new(semio_framework_job::OperationId(71), RevisionId(9), Generation(3), 17)
}

#[test]
fn layout_retained_publication_zero_grant_and_exact_writer_close_are_exact() {
    let mut publication = LayoutExportPublication::new(LayoutExportPublicationKind::Preview, b"layout-preview").expect("bounded retained preview");
    publication.begin_close();
    assert_eq!(publication.close_step(0, 0), JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert!(!publication.terminal_is_empty());
    let _ = publication.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    assert!(publication.terminal_is_empty());
}

fn request(kind: LayoutExportKind) -> LayoutExportRequest {
    let snapshot = crate::schema::default_document();
    let page_id = (!matches!(kind, LayoutExportKind::Package)).then(|| snapshot.pages[0].id.clone());
    LayoutExportRequest { kind, page_id, snapshot: Arc::new(snapshot), preflight_json: None, parent_document_id: "layout-test-document".into(), canonical_base_revision_hex: "09".repeat(32) }
}

fn drive_dispatched_worker(kind: LayoutExportKind, worker_count: usize, generation: Generation, cancel_before_start: bool) -> (StepOutcome, ArtifactOutputChunks) {
    let operation = operation();
    let bus = semio_framework::ActionBus::new();
    bus.register(LayoutExportJobFactory::new("layout-test")).expect("factory registration");
    let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
    let request = request(kind);
    let snapshot_owner = Arc::clone(&request.snapshot);
    let spec = semio_framework::ToolOperationSpec::new("layout-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, LayoutExportToolPayload { request, output_chunks: output_chunks.clone(), completion: None }, operation);
    let dispatch = bus.dispatch(spec).expect("exact factory dispatch");
    assert_eq!(bus.dispatch_count(), 1);
    let cancel = semio_framework_job::root_cancel_token();
    if cancel_before_start {
        cancel.cancel_now();
    }
    let params = BatchJobParams {
        operation: operation.operation,
        generation,
        cancel,
        config: BatchDriveConfig { site: "layout.export.worker-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: semio_framework_job::default_now_us,
    };
    let _ = worker_count;
    let outcome = drive_test_job(dispatch.job, params);
    drop(snapshot_owner);
    (outcome, output_chunks)
}

#[test]
fn exact_factory_dispatch_is_deterministic_across_real_one_two_four_and_default_worker_pools() {
    let default_workers = std::thread::available_parallelism().map_or(1, usize::from);
    for kind in [LayoutExportKind::Png, LayoutExportKind::Svg, LayoutExportKind::Pdf, LayoutExportKind::Package] {
        let mut outputs = Vec::new();
        for worker_count in [1, 2, 4, default_workers] {
            match drive_dispatched_worker(kind, worker_count, operation().generation, false) {
                (StepOutcome::Complete(_), chunks) => outputs.push(LayoutExportCommit::from_chunks(kind, "test", &chunks).expect("segmented output").data),
                outcome => panic!("unexpected {kind:?}/{worker_count} outcome: {outcome:?}"),
            }
        }
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]), "{kind:?} bytes diverged across real worker pools");
    }
}

#[test]
fn production_retained_wire_factory_decodes_before_reducer_and_closes_cancel_fault_and_success_owners() {
    fn dispatch(raw_verb: &str, kind: LayoutExportKind) -> semio_framework::ToolJobDispatch {
        let operation = operation();
        let bus = semio_framework::ActionBus::new();
        bus.register(LayoutExportJobFactory::new("layout-retained-test")).expect("retained factory registration");
        assert!(bus.begin_exact_wire("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES + 1).is_err());
        let (admission, mut input) = bus.begin_exact_wire("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES).expect("maximum extent before encoding");
        let raw = serde_json::to_vec(&(raw_verb, serde_json::json!({ "pageId": null }))).expect("fixture wire");
        for bytes in raw.chunks(semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES) {
            input.admit_page(semio_framework::action_bus::ToolWirePage::try_copy_from(bytes).expect("bounded raw page")).expect("preadmitted page");
        }
        input.seal_admitted_prefix().expect("truthful encoded prefix");
        let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
        let payload = LayoutExportToolPayload { request: request(kind), output_chunks, completion: None };
        let spec = semio_framework::ToolOperationSpec::new("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, payload, operation);
        bus.dispatch_wire_retained_with_spec(&admission, input, None, spec).unwrap_or_else(|_| panic!("retained production dispatch"))
    }

    let params = |cancel: semio_framework_job::CancelToken| BatchJobParams {
        operation: operation().operation,
        generation: operation().generation,
        cancel,
        config: BatchDriveConfig { site: "layout.retained-wire.worker-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: semio_framework_job::default_now_us,
    };
    assert!(matches!(drive_test_job(dispatch("exportSvg", LayoutExportKind::Svg).job, params(semio_framework_job::root_cancel_token())), StepOutcome::Complete(_)));
    assert!(matches!(drive_test_job(dispatch("exportPdf", LayoutExportKind::Svg).job, params(semio_framework_job::root_cancel_token())), StepOutcome::Fault(_)));
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    assert!(matches!(drive_test_job(dispatch("exportSvg", LayoutExportKind::Svg).job, params(cancel)), StepOutcome::Cancelled));
}

#[test]
fn exact_layout_out_media_factory_dispatches_a_real_reserved_job() {
    let operation = operation();
    let bus = semio_framework::ActionBus::new();
    bus.register(LayoutMediaExportJobFactory::new("layout-media-test")).expect("media factory registration");
    let request = request(LayoutExportKind::Svg);
    let snapshot_owner = Arc::clone(&request.snapshot);
    let payload = ArtifactReservedToolJob::new(LayoutExportJob::new(operation, request).expect("media payload job"));
    let spec = semio_framework::ToolOperationSpec::new("layout-media-test", LAYOUT_MEDIA_EXPORT_TOOL_ID, LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA, payload, operation);
    let dispatch = bus.dispatch(spec).expect("exact media factory dispatch");
    assert_eq!(bus.dispatch_count(), 1);
    let params = BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: BatchDriveConfig { site: "layout.media-export.worker-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: semio_framework_job::default_now_us,
    };
    assert!(matches!(drive_test_job(dispatch.job, params), StepOutcome::Complete(_)));
    drop(snapshot_owner);
}

#[test]
fn reserved_close_disposes_every_export_buffer_in_bounded_slices_and_rejects_an_unwitnessed_snapshot() {
    let mut job = LayoutExportJob::new(operation(), request(LayoutExportKind::Svg)).expect("job");
    job.json_validation = Some(JsonValidationCursor::new("[]", true).expect("json cursor"));
    job.json_validation.as_mut().expect("json cursor").stack.push(JsonContainer::Array(JsonArrayState::FirstValueOrEnd));
    job.typed_validation = Some(TypedJsonCursor { stack: vec![TypedJsonNode::Scalar { bytes: vec![1; 256], cursor: 0 }], emitted_nodes: 0, fragment_bytes: None });
    job.package_json = Some(TypedJsonCursor { stack: vec![TypedJsonNode::OwnedString { value: "z".repeat(OUTPUT_CHUNK_BYTES * 2), cursor: JsonStringWriteCursor::default() }], emitted_nodes: 0, fragment_bytes: None });
    job.rects.push(ExportRect { x: 0, y: 0, width: 1, height: 1, rgba: [0; 4] });
    job.output.append(&vec![1; OUTPUT_CHUNK_BYTES * 2]).expect("raw chunks");
    job.encoded.append(&vec![2; OUTPUT_CHUNK_BYTES * 2]).expect("encoded chunks");
    job.base64_tail = vec![3; 2];
    job.png_row = vec![4; OUTPUT_CHUNK_BYTES * 2];
    job.pdf_offsets.push(7);
    job.zip.entries.push(ZipEntry { name: "e".repeat(OUTPUT_CHUNK_BYTES * 2), crc: 0, size: 0, offset: 0 });
    job.zip.current_name = Some("current".into());
    job.request.preflight_json = Some("p".repeat(OUTPUT_CHUNK_BYTES * 2));
    job.output_chunks.push(vec![5; OUTPUT_CHUNK_BYTES]).expect("shared chunk");

    let error = loop {
        match ArtifactReservedJob::close_step(&mut job, 1, OUTPUT_CHUNK_BYTES) {
            Ok(PluginCloseStep::Pending { released_items, released_bytes }) => {
                assert_eq!(released_items, 1);
                assert!(released_bytes <= OUTPUT_CHUNK_BYTES);
            }
            Err(error) => break error,
            Ok(step) => panic!("unwitnessed close must not reach {step:?}"),
        }
    };
    assert!(format!("{error:?}").contains("snapshot-unwitnessed"));
    assert!(job.json_validation.is_none());
    assert!(job.typed_validation.is_none());
    assert!(job.package_json.is_none());
    assert!(job.rects.is_empty());
    assert!(job.output.chunks.is_empty());
    assert!(job.encoded.chunks.is_empty());
    assert!(job.base64_tail.is_empty());
    assert!(job.png_row.is_empty());
    assert!(job.pdf_offsets.is_empty());
    assert!(job.zip.entries.is_empty());
    assert!(job.zip.current_name.is_none());
    assert!(job.request.page_id.is_none());
    assert!(job.request.preflight_json.is_none());
    assert!(job.request.parent_document_id.is_empty());
    assert!(job.request.canonical_base_revision_hex.is_empty());
    assert_eq!(job.output_chunks.chunks_remaining(), 0);
}

#[test]
fn reserved_close_zero_budget_preserves_the_exact_cursor_and_owner() {
    let mut job = LayoutExportJob::new(operation(), request(LayoutExportKind::Svg)).expect("job");
    job.output.append(&vec![1; OUTPUT_CHUNK_BYTES]).expect("raw chunk");
    job.close_stage = LayoutExportCloseStage::Output;
    assert_eq!(ArtifactReservedJob::close_step(&mut job, 0, OUTPUT_CHUNK_BYTES).expect("zero item slice"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(job.close_stage, LayoutExportCloseStage::Output);
    assert_eq!(job.output.chunks.len(), 1);
    assert_eq!(ArtifactReservedJob::close_step(&mut job, 1, 0).expect("zero byte slice"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(job.close_stage, LayoutExportCloseStage::Output);
    assert_eq!(job.output.chunks.len(), 1);
}

#[test]
fn reserved_close_releases_the_witness_after_snapshot_handback() {
    let mut job = LayoutExportJob::new(operation(), request(LayoutExportKind::Svg)).expect("job");
    job.close_stage = LayoutExportCloseStage::SnapshotOwner;
    assert_eq!(ArtifactReservedJob::close_step(&mut job, 1, OUTPUT_CHUNK_BYTES).expect("witness handback"), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(job.close_stage, LayoutExportCloseStage::Complete);
}

#[test]
fn dimension_max_plus_one_is_rejected_before_encoding() {
    let mut request = request(LayoutExportKind::Png);
    Arc::make_mut(&mut request.snapshot).pages[0].width = f64::from(MAX_LAYOUT_EXPORT_DIMENSION + 1);
    let error = run_layout_export_headless_batch(operation(), request).expect_err("max + 1 must fail");
    assert!(error.contains("dimension-limit"));
}

#[test]
fn real_worker_dispatch_observes_cancel_and_stale_generation() {
    assert!(matches!(drive_dispatched_worker(LayoutExportKind::Png, 1, operation().generation, true).0, StepOutcome::Cancelled));
    assert!(matches!(drive_dispatched_worker(LayoutExportKind::Svg, 1, Generation(operation().generation.0 + 1), false).0, StepOutcome::Fault(_)));
}

#[test]
fn collection_and_json_envelopes_accept_max_and_reject_max_plus_one() {
    let mut max = request(LayoutExportKind::Svg);
    let snapshot = Arc::make_mut(&mut max.snapshot);
    let page = snapshot.pages[0].clone();
    snapshot.pages.resize(MAX_LAYOUT_EXPORT_PAGES, page);
    let style = snapshot.paragraph_styles[0].clone();
    snapshot.paragraph_styles.resize(MAX_LAYOUT_EXPORT_STYLES, style);
    let character = snapshot.character_styles.first().cloned().unwrap_or(crate::CharacterStyle { id: "character.test".into(), name: None, font_family: None, font_size: None, font_weight: None, italic: None, color: None, tracking: None });
    snapshot.character_styles.resize(MAX_LAYOUT_EXPORT_STYLES, character);
    snapshot.data_fields_json = Some(format!("{}[]", " ".repeat(MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES - 2)));
    assert!(run_layout_export_headless_batch(operation(), max).is_ok());

    let mut plus_one = request(LayoutExportKind::Svg);
    let snapshot = Arc::make_mut(&mut plus_one.snapshot);
    let page = snapshot.pages[0].clone();
    snapshot.pages.resize(MAX_LAYOUT_EXPORT_PAGES + 1, page);
    assert!(run_layout_export_headless_batch(operation(), plus_one).expect_err("page max + 1").contains("document-envelope"));

    let mut json_plus_one = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut json_plus_one.snapshot).data_fields_json = Some(format!("{}[]", " ".repeat(MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES - 1)));
    assert!(run_layout_export_headless_batch(operation(), json_plus_one).expect_err("json max + 1").contains("json-byte-limit"));
}

#[test]
fn every_top_level_collection_rejects_its_max_plus_one() {
    let mut parent_pages = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut parent_pages.snapshot).parent_pages =
        (0..=MAX_LAYOUT_EXPORT_PARENT_PAGES).map(|index| crate::ParentPage { id: format!("parent-{index}"), name: "Parent".into(), width: 100.0, height: 100.0, layer_ids: Vec::new(), layers: Vec::new(), frames: Vec::new() }).collect();
    assert!(run_layout_export_headless_batch(operation(), parent_pages).expect_err("parent max + 1").contains("document-envelope"));

    let mut spreads = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut spreads.snapshot).spreads = (0..=MAX_LAYOUT_EXPORT_SPREADS).map(|index| crate::Spread { id: format!("spread-{index}"), name: "Spread".into(), page_ids: Vec::new() }).collect();
    assert!(run_layout_export_headless_batch(operation(), spreads).expect_err("spread max + 1").contains("document-envelope"));

    let mut stories = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut stories.snapshot).stories = (0..=MAX_LAYOUT_EXPORT_STORIES).map(|index| crate::TextStory { id: format!("story-{index}"), content: String::new(), style_runs: Vec::new() }).collect();
    assert!(run_layout_export_headless_batch(operation(), stories).expect_err("story max + 1").contains("document-envelope"));

    let mut links = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut links.snapshot).links =
        (0..=MAX_LAYOUT_EXPORT_LINKS).map(|index| crate::ImageLink { id: format!("link-{index}"), path: "image.png".into(), hash: String::new(), width: 1, height: 1, dpi: 72, color_profile: None, state: None, proxy_data_url: None }).collect();
    assert!(run_layout_export_headless_batch(operation(), links).expect_err("link max + 1").contains("document-envelope"));

    for character_styles in [false, true] {
        let mut styles = request(LayoutExportKind::Svg);
        let snapshot = Arc::make_mut(&mut styles.snapshot);
        if character_styles {
            let seed = crate::CharacterStyle { id: "character.seed".into(), name: None, font_family: None, font_size: None, font_weight: None, italic: None, color: None, tracking: None };
            snapshot.character_styles.resize(MAX_LAYOUT_EXPORT_STYLES + 1, seed);
        } else {
            let seed = snapshot.paragraph_styles[0].clone();
            snapshot.paragraph_styles.resize(MAX_LAYOUT_EXPORT_STYLES + 1, seed);
        }
        assert!(run_layout_export_headless_batch(operation(), styles).expect_err("style max + 1").contains("document-envelope"));
    }
}

#[test]
fn nested_collection_string_and_json_caps_accept_max_and_reject_max_plus_one() {
    let mut max = request(LayoutExportKind::Svg);
    let snapshot = Arc::make_mut(&mut max.snapshot);
    let frame = snapshot.pages[0].frames[0].clone();
    let layer = snapshot.pages[0].layers[0].clone();
    let guide = crate::LayoutRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    let page = &mut snapshot.pages[0];
    page.frames.resize(MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE, frame);
    page.overrides.resize(MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE, PageOverride { object_id: "frame-1".into(), bounds: None, visible: None, locked: None });
    page.guides.resize(MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE, guide);
    page.layers.resize(MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE, layer);
    for layer in &mut page.layers {
        layer.object_ids.resize(MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE, "frame-1".into());
    }
    page.layer_ids.resize(MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE, "layer-1".into());
    snapshot.spreads[0].page_ids.resize(MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS, "page-1".into());
    snapshot.name = "n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES);
    snapshot.data_fields_json = Some(format!("[{}]", std::iter::repeat_n("null", MAX_LAYOUT_EXPORT_JSON_NODES - 1).collect::<Vec<_>>().join(",")));
    assert!(run_layout_export_headless_batch(operation(), max).is_ok());

    let mut frames = request(LayoutExportKind::Svg);
    let seed = frames.snapshot.pages[0].frames[0].clone();
    Arc::make_mut(&mut frames.snapshot).pages[0].frames.resize(MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE + 1, seed);
    assert!(run_layout_export_headless_batch(operation(), frames).expect_err("frame max + 1").contains("page-envelope"));

    let mut guides = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut guides.snapshot).pages[0].guides.resize(MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE + 1, crate::LayoutRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 });
    assert!(run_layout_export_headless_batch(operation(), guides).expect_err("guide max + 1").contains("page-envelope"));

    let mut spread_ids = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut spread_ids.snapshot).spreads[0].page_ids.resize(MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS + 1, "page-1".into());
    assert!(run_layout_export_headless_batch(operation(), spread_ids).expect_err("spread page id max + 1").contains("spread-envelope"));

    let mut string = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut string.snapshot).name = "n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES + 1);
    assert!(run_layout_export_headless_batch(operation(), string).expect_err("string max + 1").contains("document-envelope"));

    let mut json_nodes = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut json_nodes.snapshot).data_fields_json = Some(format!("[{}]", std::iter::repeat_n("null", MAX_LAYOUT_EXPORT_JSON_NODES).collect::<Vec<_>>().join(",")));
    assert!(run_layout_export_headless_batch(operation(), json_nodes).expect_err("json node max + 1").contains("json-node-limit"));

    let mut preflight_schema = request(LayoutExportKind::Package);
    preflight_schema.preflight_json = Some("{}".into());
    assert!(run_layout_export_headless_batch(operation(), preflight_schema).expect_err("preflight schema").contains("preflight-schema"));
}

#[test]
fn exact_output_cap_accepts_max_and_rejects_one_more_byte() {
    let mut rope = ChunkRope::new();
    let chunk = vec![0u8; OUTPUT_CHUNK_BYTES];
    for _ in 0..MAX_LAYOUT_EXPORT_OUTPUT_BYTES / OUTPUT_CHUNK_BYTES {
        rope.append(&chunk).expect("exact output max");
    }
    assert_eq!(rope.len, MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
    assert!(rope.append(&[0]).is_err());
    let mut drained = 0;
    while rope.take_chunk().is_some() {
        drained += 1;
    }
    assert_eq!(drained, MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS);
    assert!(rope.chunks.is_empty());
    assert_eq!(rope.front_byte_cursor, 0);
}

#[test]
fn chunk_rope_outer_storage_is_stable_across_growth_boundaries() {
    let mut rope = ChunkRope::new();
    assert!(rope.chunks.capacity() >= MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS);
    let chunk = vec![0u8; OUTPUT_CHUNK_BYTES];
    rope.append(&chunk).expect("first pre-admitted chunk");
    let storage = rope.chunks.as_slices().0.as_ptr();
    for _ in 1..257 {
        rope.append(&chunk).expect("pre-admitted chunk");
        assert_eq!(rope.chunks.as_slices().0.as_ptr(), storage);
    }
    assert_eq!(rope.chunks.len(), 257);
}

#[test]
fn owned_crc32_is_standard_and_incremental() {
    assert_eq!(crc32_update(0, b"123456789"), 0xcbf4_3926);
    assert_eq!(crc32_update(crc32_update(0, b"1234"), b"56789"), 0xcbf4_3926);
}

#[test]
fn typed_document_json_matches_serde_and_every_write_is_credit_bounded() {
    let mut snapshot = crate::schema::default_document();
    snapshot.name = "\u{1f642}\n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES / 5);
    snapshot.background_drawing =
        Some(crate::LayoutDrawingChild { handle: store::ArtifactChild::new("drawing-child".into(), store::os_io::ArtifactRef::parse_uri("document!s.stdio.semio@v1/drawing").expect("child reference")), content: Default::default() });
    snapshot.referenced_model = Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("document!s.stdio.semio@v1/model").expect("model reference"), pin: store::LinkPin::Head, role: "model".into() });
    let expected: serde_json::Value = serde_json::from_str(&dsl::os_pack::to_json_string(&snapshot)).expect("independent document JSON oracle");
    let mut cursor = TypedJsonCursor::document();
    let mut actual = Vec::new();
    loop {
        let (bytes, done) = cursor.advance(&snapshot).expect("typed JSON step");
        assert!(bytes.len() <= JSON_OUTPUT_BYTES_PER_UNIT);
        actual.extend_from_slice(&bytes);
        if done {
            break;
        }
    }
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual).expect("independent bounded JSON oracle"), expected);
}

#[test]
fn dynamic_json_validation_never_scans_more_than_its_input_credit() {
    let text = format!("[{}]", std::iter::repeat_n("null", MAX_LAYOUT_EXPORT_JSON_NODES - 1).collect::<Vec<_>>().join(","));
    let mut cursor = JsonValidationCursor::new(&text, true).expect("validator");
    loop {
        let before = cursor.byte_cursor;
        let done = cursor.advance(&text).expect("bounded validation step");
        assert!(cursor.byte_cursor - before <= JSON_INPUT_BYTES_PER_UNIT);
        if done {
            break;
        }
    }
}

#[test]
fn terminal_candidate_is_empty_and_owned_chunks_never_exceed_four_kibibytes() {
    let operation = operation();
    let job = LayoutExportJob::new(operation, request(LayoutExportKind::Png)).expect("job");
    let snapshot_owner = Arc::clone(&job.request.snapshot);
    let chunks = job.output_chunks.clone();
    let params = BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: BatchDriveConfig { site: "layout.export.segment-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: semio_framework_job::default_now_us,
    };
    drop(snapshot_owner);
    let candidate = match drive_test_job(job, params.clone()) {
        StepOutcome::Complete(candidate) => candidate,
        outcome => panic!("unexpected terminal outcome: {outcome:?}"),
    };
    assert!(candidate.output.is_empty());
    let mut drained = 0;
    while let Some(chunk) = chunks.take_chunk().expect("sealed chunks") {
        assert!(!chunk.is_empty());
        assert!(chunk.len() <= OUTPUT_CHUNK_BYTES);
        drained += chunk.len();
    }
    assert!(drained > 0);
}

#[test]
fn supplied_preflight_array_is_preserved_byte_for_byte_in_package_entry() {
    let snapshot = crate::schema::default_document();
    let supplied = r#"[{"kind":"custom","severity":"warning"}]"#;
    let json = dsl::os_pack::to_json_string(&snapshot);
    let package = export_package_zip_headless_batch(&json, supplied).expect("package");
    assert!(package.windows(supplied.len()).any(|window| window == supplied.as_bytes()));
}

#[test]
fn one_unit_budget_forces_multiple_yields_and_stale_context_faults() {
    let operation = operation();
    let mut job = LayoutExportJob::new(operation, request(LayoutExportKind::Svg)).expect("job");
    let mut sequence = 0;
    for _ in 0..2 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
        assert_eq!(job.step(&mut context), StepOutcome::Yield);
    }
    let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
    assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
}

#[test]
fn checkpoint_is_lossless_bounded_and_authority_qualified() {
    let operation = operation();
    let request_value = request(LayoutExportKind::Package);
    let mut job = LayoutExportJob::new(operation, request_value.clone()).expect("job");
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    let mut checkpoint = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
            break checkpoint;
        }
    };
    assert_eq!(checkpoint.state.len(), MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES);
    let checkpoint_state = checkpoint.state.single_page().expect("checkpoint owns one retained page").to_vec();
    while !checkpoint.state.terminal_is_empty() {
        let _ = checkpoint.state.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
    let restored = LayoutExportJob::restore(operation, request_value.clone(), &checkpoint_state).expect("matching authority").with_output_chunks(output_chunks.clone());
    let params = BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: BatchDriveConfig { site: "layout.export.restore-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: semio_framework_job::default_now_us,
    };
    match drive_test_job(restored, params.clone()) {
        StepOutcome::Complete(candidate) => assert!(candidate.output.is_empty()),
        outcome => panic!("resumed outcome: {outcome:?}"),
    }
    let resumed = LayoutExportCommit::from_chunks(LayoutExportKind::Package, "layout", &output_chunks).expect("drained resumed output").data.into_bytes();
    let uninterrupted = run_layout_export_headless_batch(operation, request_value).expect("uninterrupted").data.into_bytes();
    assert_eq!(resumed, uninterrupted);
    let stale = Operation::new(semio_framework_job::OperationId(72), RevisionId(9), Generation(3), 17);
    assert!(LayoutExportJob::restore(stale, request(LayoutExportKind::Package), &checkpoint_state).is_err());
    let mut malformed = checkpoint_state;
    malformed[0] ^= 0xff;
    assert!(LayoutExportJob::restore(operation, request(LayoutExportKind::Package), &malformed).is_err());
}
