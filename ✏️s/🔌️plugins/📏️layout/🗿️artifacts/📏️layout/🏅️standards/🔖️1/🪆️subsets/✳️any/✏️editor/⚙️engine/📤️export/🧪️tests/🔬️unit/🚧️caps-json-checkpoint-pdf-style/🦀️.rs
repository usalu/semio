use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
/// 🚧️ Caps, checkpoint and pdf style laws: the dimension and collection caps, the output cap, the chunk rope and crc, the preflight entry, the yield budget, the checkpoint codec, the close ladder and the pdf proxy, style and layer laws.

#[test]
fn dimension_max_plus_one_is_rejected_before_encoding() {
    let mut request = request(LayoutExportKind::Png);
    Arc::make_mut(&mut request.snapshot).pages[0].width = f64::from(MAX_LAYOUT_EXPORT_DIMENSION + 1);
    let error = run_layout_export_headless_batch(operation(), request).expect_err("max + 1 must fail");
    assert!(error.contains("dimension-limit"));
}

#[test]
fn real_worker_dispatch_observes_cancel_and_stale_generation() {
    let run = TestRun::new("layout.export.worker-test");
    assert!(matches!(drive_dispatched_worker(LayoutExportKind::Png, TestRun { cancelled: true, ..run }).0, LayoutExportObserved::Cancelled));
    assert!(matches!(drive_dispatched_worker(LayoutExportKind::Svg, TestRun { generation: Generation(operation().generation.0 + 1), ..run }).0, LayoutExportObserved::Fault(_)));
}

#[test]
fn collection_and_json_envelopes_accept_max_and_reject_max_plus_one() {
    let mut max = request(LayoutExportKind::Svg);
    let snapshot = Arc::make_mut(&mut max.snapshot);
    let page = snapshot.pages[0].clone();
    snapshot.pages.resize(MAX_LAYOUT_EXPORT_PAGES, page);
    let style = snapshot.paragraph_styles[0].clone();
    snapshot.paragraph_styles.resize(MAX_LAYOUT_EXPORT_STYLES, style);
    let character = snapshot.character_styles.first().cloned().unwrap_or(semio_s_artifact_layout_layout_model::CharacterStyle { id: "character.test".into(), name: None, font_family: None, font_size: None, font_weight: None, italic: None, color: None, tracking: None });
    snapshot.character_styles.resize(MAX_LAYOUT_EXPORT_STYLES, character);
    snapshot.data_fields = Some(dictionary_at_json_bytes(MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES));
    assert!(run_layout_export_headless_batch(operation(), max).is_ok());

    let mut plus_one = request(LayoutExportKind::Svg);
    let snapshot = Arc::make_mut(&mut plus_one.snapshot);
    let page = snapshot.pages[0].clone();
    snapshot.pages.resize(MAX_LAYOUT_EXPORT_PAGES + 1, page);
    assert!(run_layout_export_headless_batch(operation(), plus_one).expect_err("page max + 1").contains("document-envelope"));

    let mut json_plus_one = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut json_plus_one.snapshot).data_fields = Some(dictionary_at_json_bytes(MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES + 1));
    assert!(run_layout_export_headless_batch(operation(), json_plus_one).expect_err("json max + 1").contains("json-byte-limit"));
}

#[test]
fn every_top_level_collection_rejects_its_max_plus_one() {
    let mut parent_pages = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut parent_pages.snapshot).parent_pages =
        (0..=MAX_LAYOUT_EXPORT_PARENT_PAGES).map(|index| semio_s_artifact_layout_layout_model::ParentPage { id: format!("parent-{index}"), name: "Parent".into(), width: 100.0, height: 100.0, layer_ids: Vec::new(), layers: Vec::new(), frames: Vec::new() }).collect();
    assert!(run_layout_export_headless_batch(operation(), parent_pages).expect_err("parent max + 1").contains("document-envelope"));

    let mut spreads = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut spreads.snapshot).spreads = (0..=MAX_LAYOUT_EXPORT_SPREADS).map(|index| semio_s_artifact_layout_layout_model::Spread { id: format!("spread-{index}"), name: "Spread".into(), page_ids: Vec::new() }).collect();
    assert!(run_layout_export_headless_batch(operation(), spreads).expect_err("spread max + 1").contains("document-envelope"));

    let mut stories = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut stories.snapshot).stories = (0..=MAX_LAYOUT_EXPORT_STORIES).map(|index| semio_s_artifact_layout_layout_model::TextStory { id: format!("story-{index}"), content: String::new(), style_runs: Vec::new() }).collect();
    assert!(run_layout_export_headless_batch(operation(), stories).expect_err("story max + 1").contains("document-envelope"));

    let mut links = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut links.snapshot).links =
        (0..=MAX_LAYOUT_EXPORT_LINKS).map(|index| semio_s_artifact_layout_layout_model::ImageLink { id: format!("link-{index}"), path: "image.png".into(), hash: String::new(), width: 1, height: 1, dpi: 72, color_profile: None, state: None, proxy_data_url: None, artifact_kind: String::new(), artifact_ref: String::new() }).collect();
    assert!(run_layout_export_headless_batch(operation(), links).expect_err("link max + 1").contains("document-envelope"));

    for character_styles in [false, true] {
        let mut styles = request(LayoutExportKind::Svg);
        let snapshot = Arc::make_mut(&mut styles.snapshot);
        if character_styles {
            let seed = semio_s_artifact_layout_layout_model::CharacterStyle { id: "character.seed".into(), name: None, font_family: None, font_size: None, font_weight: None, italic: None, color: None, tracking: None };
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
    let guide = semio_s_artifact_layout_layout_model::LayoutRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    let page = &mut snapshot.pages[0];
    page.frames.resize(32, frame);
    page.overrides.resize(32, PageOverride { object_id: "frame-1".into(), bounds: None, visible: None, locked: None });
    page.guides.resize(MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE, guide);
    page.layers.resize(MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE, layer);
    for layer in &mut page.layers {
        layer.object_ids.resize(32, "frame-1".into());
    }
    page.layer_ids.resize(MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE, "layer-1".into());
    snapshot.spreads[0].page_ids.resize(MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS, "page-1".into());
    snapshot.name = "n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES);
    snapshot.data_fields = Some(dictionary_with_nulls(MAX_LAYOUT_EXPORT_JSON_NODES - 1));
    assert!(run_layout_export_headless_batch(operation(), max).is_ok());

    let mut frames = request(LayoutExportKind::Svg);
    let seed = frames.snapshot.pages[0].frames[0].clone();
    Arc::make_mut(&mut frames.snapshot).pages[0].frames.resize(semio_s_artifact_layout_layout_engine_json::MAX_LAYOUT_EXPORT_DECODED_ITEMS + 1, seed);
    assert!(run_layout_export_headless_batch(operation(), frames).expect_err("decoded item budget + 1").contains("item-limit"));

    let mut guides = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut guides.snapshot).pages[0].guides.resize(MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE + 1, semio_s_artifact_layout_layout_model::LayoutRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 });
    assert!(run_layout_export_headless_batch(operation(), guides).expect_err("guide max + 1").contains("page-envelope"));

    let mut spread_ids = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut spread_ids.snapshot).spreads[0].page_ids.resize(MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS + 1, "page-1".into());
    assert!(run_layout_export_headless_batch(operation(), spread_ids).expect_err("spread page id max + 1").contains("spread-envelope"));

    let mut string = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut string.snapshot).name = "n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES + 1);
    assert!(run_layout_export_headless_batch(operation(), string).expect_err("string max + 1").contains("document-envelope"));

    let mut json_nodes = request(LayoutExportKind::Svg);
    Arc::make_mut(&mut json_nodes.snapshot).data_fields = Some(dictionary_with_nulls(MAX_LAYOUT_EXPORT_JSON_NODES));
    assert!(run_layout_export_headless_batch(operation(), json_nodes).expect_err("json node max + 1").contains("json-node-limit"));

    let mut preflight_schema = request(LayoutExportKind::Package);
    preflight_schema.preflight_json = Some("{}".into());
    assert!(run_layout_export_headless_batch(operation(), preflight_schema).expect_err("preflight schema").contains("preflight-schema"));
}

#[test]
fn exact_output_cap_accepts_max_and_rejects_one_more_byte(){let operation=operation();let mut caller=layout_export_fixture_caller(operation,operation.generation);let mut rope=ChunkRope::new();assert!(rope.terminal_is_empty());let chunk=vec![0u8;OUTPUT_CHUNK_BYTES];for _ in 0..MAX_LAYOUT_EXPORT_OUTPUT_BYTES/OUTPUT_CHUNK_BYTES{append_output_fixture(&mut rope,&chunk,&mut caller);}assert_eq!(rope.len,MAX_LAYOUT_EXPORT_OUTPUT_BYTES);assert!(output_append_refused_fixture(&mut rope,&[0],&mut caller));let mut drained=0;loop{let(chunk,done)=caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|{let result=rope.transfer_one(cx.retained_grant())?;cx.consume_retained(result.2)?;Ok((result.0,result.1))}).unwrap();if chunk.is_some(){drained+=1;}if done{break;}}assert_eq!(drained,MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS);close_output_fixture(&mut rope,&mut caller);assert!(rope.terminal_is_empty());caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);}

#[test]
fn output_byte_pages_keep_original_payload_addresses_across_metadata_growth(){let operation=operation();let mut caller=layout_export_fixture_caller(operation,operation.generation);let mut rope=ChunkRope::new();assert_eq!(rope.allocated_bytes(),0);let chunk=vec![0u8;OUTPUT_CHUNK_BYTES];append_output_fixture(&mut rope,&chunk,&mut caller);let original=rope.fixture_source_pointer(0).unwrap();for _ in 1..257{append_output_fixture(&mut rope,&chunk,&mut caller);assert_eq!(rope.fixture_source_pointer(0),Some(original));}assert_eq!(rope.len,257*OUTPUT_CHUNK_BYTES);close_output_fixture(&mut rope,&mut caller);caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);}

#[test]
fn owned_crc32_is_standard_and_incremental() {
    assert_eq!(crc32_update(0, b"123456789"), 0xcbf4_3926);
    assert_eq!(crc32_update(crc32_update(0, b"1234"), b"56789"), 0xcbf4_3926);
}

#[test]
fn terminal_candidate_is_empty_and_owned_chunks_never_exceed_four_kibibytes() {
    let operation = operation();
    let job = fixture_export_job(operation, request(LayoutExportKind::Png));
    let snapshot_owner = Arc::clone(&job.request.as_ref().expect("a fresh job owns its request").snapshot);
    let chunks = job.output_chunks.as_ref().expect("a fresh job owns its output").clone();
    let (outcome, drained) = drive_job_batch(job, operation, operation.generation, false, "layout.export.segment-test", || {
        let mut drained = 0;
        while let Some(chunk) = chunks.take_chunk().expect("sealed chunks") {
            assert!(!chunk.is_empty());
            assert!(chunk.len() <= OUTPUT_CHUNK_BYTES);
            drained += chunk.len();
        }
        drained
    });
    drop(snapshot_owner);
    assert_eq!(outcome, LayoutExportObserved::Complete { state_bytes: 0, output_bytes: 0 });
    assert!(drained > 0);
}

#[test]
fn supplied_preflight_array_is_preserved_byte_for_byte_in_package_entry() {
    let snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let supplied = r#"[{"kind":"custom","severity":"warning"}]"#;
    let json = semio_framework_pack_json::to_json_string(&snapshot);
    let package = export_package_zip_headless_batch(&json, supplied).expect("package");
    assert!(package.windows(supplied.len()).any(|window| window == supplied.as_bytes()));
}

#[test]
fn one_unit_budget_forces_multiple_yields_and_stale_context_faults() {
    let operation = operation();
    let mut caller = layout_export_fixture_caller(operation, operation.generation);
    let mut job = fixture_export_job(operation, request(LayoutExportKind::Svg));
    for _ in 0..2 {
        assert_eq!(next_outcome(&mut caller, &mut job), LayoutExportObserved::Yield);
    }
    let mut stale_caller = layout_export_fixture_caller(operation, Generation(operation.generation.0 + 1));
    assert_eq!(next_outcome(&mut stale_caller, &mut job), LayoutExportObserved::Fault("layout-export-stale-operation".into()));
    close_job(job, &mut caller);
    stale_caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
}

#[test]
fn checkpoint_is_lossless_bounded_and_authority_qualified() {
    let operation = operation();
    let request_value = request(LayoutExportKind::Package);
    let mut caller = layout_export_fixture_caller(operation, operation.generation);
    let mut job = fixture_export_job(operation, request_value.clone());
    let mut outcomes = 0usize;
    let (applied_progress, checkpoint_state) = loop {
        outcomes += 1;
        assert!(outcomes < 4_096, "the package export never published a checkpoint");
        match next_outcome(&mut caller, &mut job) {
            LayoutExportObserved::Checkpoint { applied_progress, state } => break (applied_progress, state),
            terminal if terminal.is_terminal() => panic!("the package export ended before its first checkpoint: {terminal:?}"),
            _ => {}
        }
    };
    assert_eq!(applied_progress % 64, 0);
    assert_eq!(checkpoint_state.len(), MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES);
    close_job(job, &mut caller);
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
    let restored = LayoutExportJob::restore(operation, request_value.clone(), &checkpoint_state).expect("matching authority");
    let output_chunks = restored.output_chunks.as_ref().expect("a restored job owns a fresh output").clone();
    let (outcome, resumed) = drive_job_batch(restored, operation, operation.generation, false, "layout.export.restore-test", || LayoutExportCommit::from_chunks(LayoutExportKind::Package, "layout", &output_chunks).expect("drained resumed output").data.into_bytes());
    assert_eq!(outcome, LayoutExportObserved::Complete { state_bytes: 0, output_bytes: 0 });
    let uninterrupted = run_layout_export_headless_batch(operation, request_value).expect("uninterrupted").data.into_bytes();
    assert_eq!(resumed, uninterrupted);
    let stale = Operation::new(semio_framework_job::OperationId(72), RevisionId(9), Generation(3), 17);
    assert!(LayoutExportJob::restore(stale, request(LayoutExportKind::Package), &checkpoint_state).is_err());
    let mut malformed = checkpoint_state;
    malformed[0] ^= 0xff;
    assert!(LayoutExportJob::restore(operation, request(LayoutExportKind::Package), &malformed).is_err());
}

/// ⚖️ LAW: the close ladder drains the publication stage instead of refusing it forever.
///
/// 🪪️ `begin_close` parks the job on `LayoutExportCloseStage::Publication`, and `close_export_step`'s
/// `Publication` arm is a hard `Err` by design: whichever ladder runs must drain the retained
/// publication payload and advance the stage itself. A caller that closes with
/// `while !terminal_is_empty() { close_step(…) }`, which the batch driver and the framework's own
/// session ladders both do, then spins forever when the ladder keeps refusing that stage, until the
/// 30-minute test-binary watchdog SIGKILLs the binary and takes every other layout result with it.
/// Bounded on purpose: a close ladder that cannot leave one stage in a bounded number of slices is the
/// defect, so this law counts slices rather than hanging.
#[test]
fn the_close_ladder_leaves_the_publication_stage_in_bounded_slices() {
    let operation = operation();
    let mut caller = layout_export_fixture_caller(operation, operation.generation);
    let request = request(LayoutExportKind::Svg);
    let snapshot_owner = Arc::clone(&request.snapshot);
    let mut job = fixture_export_job(operation, request);
    InteractiveJob::begin_close(&mut job);
    assert_eq!(job.close_stage, LayoutExportCloseStage::Publication, "begin_close parks the job on the publication stage");
    for slice in 0..64 {
        match caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| InteractiveJob::close_step(&mut job, cx)) {
            Ok(InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. }) => {}
            Ok(step) => panic!("the publication close is blocked at slice {slice}: {step:?}"),
            Err(error) => panic!("the close ladder refused its own publication stage at slice {slice}: {error:?}"),
        }
        if job.close_stage != LayoutExportCloseStage::Publication {
            close_job(job, &mut caller);
            drop(snapshot_owner);
            caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
            return;
        }
    }
    panic!("the close ladder never left the publication stage");
}

#[test]
fn pdf_export_prints_proxy_png_pixels() {
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let image = semio_framework_pixels::RasterImage { width: 2, height: 2, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255] };
    let png = semio_framework_pixels::encode_png(&image).expect("png");
    let decoded = semio_framework_pixels::decode_png(&png).expect("png round trip");
    assert_eq!(decoded.pixels, image.pixels, "the png codec keeps the placed pixels");
    snapshot.links[0].state = Some("ready".into());
    snapshot.links[0].proxy_data_url = Some(format!("data:image/png;base64,{}", base64_encode(&png)));
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(pdf_has_operation(&bytes, "cm", &[60.0, 0.0, 0.0, 40.0, 136.0, 25.0]), "the proxy is placed in the image frame");
    assert!(pdf_rgb_images(&bytes).iter().any(|(rgb, alpha)| rgb == &[255,0,0,0,255,0,0,0,255,255,255,255] && alpha == &[255;4]));
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let a = chunk[0];
        let b = chunk.get(1).copied().unwrap_or(0);
        let c = chunk.get(2).copied().unwrap_or(0);
        out.push(TABLE[(a >> 2) as usize] as char);
        out.push(TABLE[((a & 3) << 4 | b >> 4) as usize] as char);
        out.push(if chunk.len() > 1 { TABLE[((b & 15) << 2 | c >> 6) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[(c & 63) as usize] as char } else { '=' });
    }
    out
}

#[test]
fn pdf_export_rotates_a_proxy_with_its_frame() {
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let image = semio_framework_pixels::RasterImage { width: 2, height: 2, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255] };
    let png = semio_framework_pixels::encode_png(&image).expect("png");
    snapshot.links[0].state = Some("ready".into());
    snapshot.links[0].proxy_data_url = Some(format!("data:image/png;base64,{}", base64_encode(&png)));
    let frame = snapshot.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-image-1").expect("image");
    let semio_s_artifact_layout_layout_model::Frame::Image { bounds, .. } = frame else { panic!("image") };
    bounds.rotation = std::f64::consts::FRAC_PI_2;
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(pdf_has_operation(&bytes, "cm", &[0.0, -60.0, 40.0, 0.0, 146.0, 75.0]), "the proxy turns with the frame");
    assert!(pdf_rgb_images(&bytes).iter().any(|(rgb, alpha)| rgb == &[255,0,0,0,255,0,0,0,255,255,255,255] && alpha == &[255;4]));
}

#[test]
fn pdf_export_prints_a_character_style_run() {
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.character_styles.push(semio_s_artifact_layout_layout_model::CharacterStyle { id: "character-1".into(), name: Some("Emphasis".into()), font_family: None, font_size: Some(24.0), font_weight: None, italic: None, color: Some([1.0, 0.0, 0.0, 1.0]), tracking: None });
    snapshot.stories[0].style_runs.push(semio_s_artifact_layout_layout_model::TextStyleRun { start: 0, end: 5, paragraph_style_id: None, character_style_id: Some("character-1".into()) });
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(pdf_font_size(&bytes,24.0), "the styled run retains its character size");
    assert!(pdf_has_operation(&bytes,"rg",&[1.0,0.0,0.0]));
    assert!(pdf_font_size(&bytes,12.0), "the remaining text retains its paragraph size");
    assert!(pdf_text_compact(&bytes).contains(&snapshot.stories[0].content.chars().filter(|c|!c.is_whitespace()).collect::<String>()));
}

#[test]
fn pdf_export_obliques_and_tracks_a_character_span() {
    let mut tracked = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    tracked.character_styles.push(semio_s_artifact_layout_layout_model::CharacterStyle { id: "character-1".into(), name: Some("Emphasis".into()), font_family: None, font_size: None, font_weight: Some(700), italic: Some(true), color: None, tracking: Some(10.0) });
    tracked.stories[0].style_runs.push(semio_s_artifact_layout_layout_model::TextStyleRun { start: 0, end: 5, paragraph_style_id: None, character_style_id: Some("character-1".into()) });
    let mut plain = tracked.clone();
    plain.character_styles[0].tracking = Some(0.0);
    plain.character_styles[0].italic = Some(false);
    let mut engine = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::LayoutEngine::new();
    let tracked_list = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::build_display_list_for_page(&mut engine, &tracked, &tracked.pages[0], "", &[], None, false);
    let plain_list = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::build_display_list_for_page(&mut engine, &plain, &plain.pages[0], "", &[], None, false);
    let tracked_glyphs = &tracked_list.text_runs.iter().find(|run| run.object_id == "frame-text-1").expect("story").glyphs;
    let plain_glyphs = &plain_list.text_runs.iter().find(|run| run.object_id == "frame-text-1").expect("story").glyphs;
    assert!(tracked_glyphs.iter().take(5).all(|glyph| glyph.italic));
    assert!(tracked_glyphs.iter().skip(5).all(|glyph| !glyph.italic));
    let tracked_gap = tracked_glyphs[1].x - tracked_glyphs[0].x;
    let plain_gap = plain_glyphs[1].x - plain_glyphs[0].x;
    assert!(tracked_gap > plain_gap + 5.0, "tracked {tracked_gap} plain {plain_gap}");
    let all = headless_batch_export(LayoutExportKind::Pdf, &tracked, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(pdf_has_oblique_text(&bytes,true));
    assert!(pdf_has_oblique_text(&bytes,false));
}

#[test]
fn pdf_export_prints_a_page_override() {
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.pages[0].overrides.push(semio_s_artifact_layout_layout_model::PageOverride { object_id: "frame-inherited".into(), bounds: Some(semio_s_artifact_layout_layout_model::LayoutBounds { x: 90.0, y: 50.0, width: 100.0, height: 80.0, rotation: 0.0 }), visible: None, locked: None });
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(pdf_has_operation(&bytes,"m",&[90.0,450.0]));
    assert!(!pdf_has_operation(&bytes,"m",&[50.0,450.0]));
}

#[test]
fn pdf_export_omits_a_hidden_layer() {
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.pages[0].layers[0].visible = false;
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(pdf_text_compact(&bytes).is_empty(), "the hidden content layer prints no story text");
    assert!(!pdf_has_operation(&bytes,"m",&[10.0,490.0]), "the hidden rect is omitted");
    assert!(pdf_has_operation(&bytes,"m",&[50.0,450.0]));
}

#[test]
fn pdf_export_omits_a_frame_on_a_hidden_layer() {
    use semio_s_artifact_layout_layout_mutations_frames::create_layer::CreateLayer;
    use semio_s_artifact_layout_layout_mutations_frames::set_frame_layer::SetFrameLayer;
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    use protocol::{ColdMutation, Mutation, MutationDiff};
    let base = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let created = protocol::apply_diff_cold(LayoutMutation::CreateLayer(CreateLayer { page_id: "page-1".into(), id: "layer-2".into(), name: "Notes".into(), remove: false, index: None }).diff(&base).diff(), &base).expect("layer");
    let mut snapshot = protocol::apply_diff_cold(LayoutMutation::SetFrameLayer(SetFrameLayer { page_id: "page-1".into(), frame_id: "frame-1".into(), layer_id: "layer-2".into() }).diff(&created).diff(), &created).expect("move");
    snapshot.pages[0].layers.iter_mut().find(|layer| layer.id == "layer-2").unwrap().visible = false;
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(!pdf_has_operation(&bytes,"m",&[10.0,490.0]));
    assert!(pdf_text_compact(&bytes).contains(&snapshot.stories[0].content.chars().filter(|c|!c.is_whitespace()).collect::<String>()));
}

#[test]fn checkpoint_envelope_separates_work_progress_from_committed_prefix_and_rejects_invalid_counters(){let operation=operation();let request_value=request(LayoutExportKind::Svg);let checkpoint=LayoutExportCheckpoint{work_units:17,committed_units:3,output_bytes:0,output_digest:0xcbf29ce484222325};let encoded=encode_export_checkpoint_fixture(&operation,&request_value,&checkpoint).unwrap();assert_eq!(&encoded[..4],b"LXC3");assert_eq!(encoded.len(),642);assert_eq!(decode_export_checkpoint_fixture(&operation,&request_value,&encoded).unwrap(),checkpoint);let mut invalid=encoded;let work_offset=MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES-32;invalid[work_offset..work_offset+8].copy_from_slice(&2u64.to_le_bytes());assert!(decode_export_checkpoint_fixture(&operation,&request_value,&invalid).is_err());let mut invalid=encoded;let bytes_offset=MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES-16;invalid[bytes_offset..bytes_offset+8].copy_from_slice(&u64::MAX.to_le_bytes());assert!(decode_export_checkpoint_fixture(&operation,&request_value,&invalid).is_err());assert!(encode_export_checkpoint_fixture(&operation,&request_value,&LayoutExportCheckpoint{committed_units:18,..checkpoint}).is_err());}
