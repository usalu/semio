/// 🎟️ Harness, pdf and dispatch laws: the original-caller helpers and close ladder drivers, the pdf structure and first export laws, the segmented download law, the factory dispatch laws and the close ladder laws.

/// 🏃️ One batch drive of a job: the generation its original caller runs under, whether the caller cancels before the first step, and the site that names the drive.
#[derive(Clone, Copy)]
struct TestRun {
    generation: Generation,
    cancelled: bool,
    site: &'static str,
}

impl TestRun {
    fn new(site: &'static str) -> Self {
        Self { generation: operation().generation, cancelled: false, site }
    }
}

fn drive_test_job<J: InteractiveJob + 'static>(job: J, run: TestRun) -> LayoutExportObserved {
    drive_job_batch(job, operation(), run.generation, run.cancelled, run.site, || ()).0
}

/// 🚪️ Closes a hand-driven job to terminal-empty on its original caller. The export ladder refuses to be the LAST owner of its
/// snapshot (`layout snapshot lost its paired original supplier`), so the caller keeps the host's witness alive across the close
/// exactly as the store or the media route's close lease does in production.
fn close_job(mut job: LayoutExportJob, caller: &mut OriginalJobFixtureCaller) {
    let witness = Arc::clone(&job.request.as_ref().expect("an unclosed job owns its request").snapshot);
    InteractiveJob::begin_close(&mut job);
    let mut slices = 0usize;
    while !InteractiveJob::terminal_is_empty(&job) {
        slices += 1;
        assert!(slices < LAYOUT_EXPORT_FIXTURE_TURN_BUDGET, "the hand-driven job's close ladder never reached terminal-empty");
        caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| InteractiveJob::close_step(&mut job, cx)).expect("hand-driven close turn");
    }
    drop(job);
    drop(witness);
}

/// 👣️ One hand-driven step of a job under a one-unit work loan; `None` while the step only advanced a retained frontier.
fn step_one_unit<J: InteractiveJob>(caller: &mut OriginalJobFixtureCaller, job: &mut J) -> Option<LayoutExportObserved> {
    let mut observed = None;
    caller
        .turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| {
            cx.with_original_work_loan(1, u64::MAX, |same| {
                observed = job.step(same)?.as_ref().map(LayoutExportObserved::of_borrow);
                Ok(())
            })
        })
        .expect("hand-driven step turn");
    observed
}

/// 👣️ Steps a job until it reports one outcome.
fn next_outcome<J: InteractiveJob>(caller: &mut OriginalJobFixtureCaller, job: &mut J) -> LayoutExportObserved {
    for _ in 0..LAYOUT_EXPORT_FIXTURE_TURN_BUDGET {
        if let Some(observed) = step_one_unit(caller, job) {
            return observed;
        }
    }
    panic!("the job never reported an outcome inside the turn budget");
}

fn operation() -> Operation {
    Operation::new(semio_framework_job::OperationId(71), RevisionId(9), Generation(3), 17)
}

#[test]
fn layout_retained_publication_zero_grant_and_exact_writer_close_are_exact() {
    let operation = operation();
    let mut caller = layout_export_fixture_caller(operation, operation.generation);
    let mut publication = LayoutExportPublication::new(LayoutExportPublicationKind::Preview, b"layout-preview").expect("bounded retained preview");
    let mut turns = 0usize;
    let observed = loop {
        turns += 1;
        assert!(turns < LAYOUT_EXPORT_FIXTURE_TURN_BUDGET, "the publication never lent its preview");
        if let Some(observed) = caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| Ok(publication.advance(cx)?.as_ref().map(LayoutExportObserved::of_borrow))).expect("publication turn") {
            break observed;
        }
    };
    assert_eq!(observed, LayoutExportObserved::Preview(b"layout-preview".to_vec()));
    let zero = RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 0 };
    assert_eq!(publication.close_step(zero).expect("zero grant close"), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
    assert!(!publication.terminal_is_empty());
    let mut slices = 0usize;
    while !publication.terminal_is_empty() {
        slices += 1;
        assert!(slices < LAYOUT_EXPORT_FIXTURE_TURN_BUDGET, "the publication's close ladder never reached terminal-empty");
        publication.close_step(LAYOUT_EXPORT_FIXTURE_TURN).expect("exact publication close");
    }
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
}

fn request(kind: LayoutExportKind) -> LayoutExportRequest {
    let snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let page_id = (!matches!(kind, LayoutExportKind::Package)).then(|| snapshot.pages[0].id.clone());
    LayoutExportRequest { kind, page_id, snapshot: Arc::new(snapshot), preflight_json: None, parent_document_id: "layout-test-document".into(), canonical_base_revision_hex: "09".repeat(32) }
}

//#region 📕️PdfDocument
fn find_bytes(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack.get(from..).and_then(|tail| tail.windows(needle.len()).position(|window| window == needle)).map(|at| from + at)
}

fn ascii_number(bytes: &[u8], from: usize) -> usize {
    let digits: String = bytes[from..].iter().take_while(|byte| byte.is_ascii_digit()).map(|byte| *byte as char).collect();
    digits.parse().unwrap_or_else(|_| panic!("ascii number at {from}: {digits:?}"))
}

/// 📕️ Parses the xref table the way a viewer does — on raw bytes, since the font stream is binary:
/// every `N 0 obj` offset must land exactly, every stream's `/Length` (direct or indirect) must equal
/// its byte count, and `startxref` must point at `xref`. Returns the stream count.
fn assert_pdf_structure(bytes: &[u8]) -> usize {
    assert!(bytes.starts_with(b"%PDF-1.4\n"), "pdf header");
    let startxref = find_bytes(bytes, b"startxref\n", 0).expect("startxref");
    let xref_offset = ascii_number(bytes, startxref + 10);
    assert!(bytes[xref_offset..].starts_with(b"xref\n0 "), "startxref lands on the xref table");
    let size = ascii_number(bytes, xref_offset + 7);
    let mut row = find_bytes(bytes, b"\n", xref_offset + 7).expect("xref rows") + 1;
    for id in 0..size {
        let offset = ascii_number(bytes, row);
        if id != 0 {
            let head = format!("{id} 0 obj\n");
            assert!(bytes[offset..].starts_with(head.as_bytes()), "object {id} offset {offset} lands on its header, found {:?}", String::from_utf8_lossy(&bytes[offset..(offset + 12).min(bytes.len())]));
        }
        row += 20;
    }
    let mut cursor = 0;
    let mut streams = 0;
    while let Some(start) = find_bytes(bytes, b">>\nstream\n", cursor) {
        let stream_start = start + 10;
        let dictionary_start = bytes[..start].windows(2).rposition(|window| window == b"<<").expect("stream dictionary");
        let dictionary = String::from_utf8_lossy(&bytes[dictionary_start..start]).to_string();
        let length_text = dictionary.split("/Length ").nth(1).expect("stream length").split_whitespace().take(3).collect::<Vec<_>>();
        let length: usize = if length_text.get(1) == Some(&"0") && length_text.get(2) == Some(&"R") {
            let id: usize = length_text[0].parse().expect("indirect length id");
            let head = format!("{id} 0 obj\n");
            let at = find_bytes(bytes, head.as_bytes(), 0).expect("length object") + head.len();
            ascii_number(bytes, at)
        } else {
            length_text[0].trim_end_matches('/').parse().expect("direct length")
        };
        assert!(bytes[stream_start + length..].starts_with(b"\nendstream"), "stream length {length} ends exactly at endstream");
        streams += 1;
        cursor = stream_start + length;
    }
    streams
}

#[test]
fn pdf_export_carries_every_page_with_the_embedded_font_and_shaped_glyphs() {
    let snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, None, None).expect("document pdf");
    assert_eq!(all.filename, "Demo.pdf");
    assert_eq!(all.mime_type, "application/pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    // 🔬️ `SEMIO_LAYOUT_PDF_DUMP=<path>` hands the bytes to an external reader (the ticket's `🐍️validate-pdf.py`).
    if let Ok(path) = std::env::var("SEMIO_LAYOUT_PDF_DUMP") {
        std::fs::write(path, &bytes).expect("pdf dump");
    }
    let text = String::from_utf8_lossy(&bytes).to_string();
    let streams = assert_pdf_structure(&bytes);
    assert_eq!(streams, 1 + snapshot.pages.len(), "one font stream plus one content stream per page");
    assert!(text.contains(&format!("/Count {} >>", snapshot.pages.len())), "every page is a kid");
    assert_eq!(text.matches("/Type /Page ").count(), snapshot.pages.len());
    assert!(text.contains("/Subtype /CIDFontType2") && text.contains("/Encoding /Identity-H") && text.contains("/FontFile2 6 0 R"), "embedded CID font");
    let font_start = find_bytes(&bytes, b"6 0 obj\n", 0).expect("font object");
    let stream_at = find_bytes(&bytes, b"stream\n", font_start).expect("font stream") + 7;
    assert!(bytes[stream_at..].starts_with(&[0x00, 0x01, 0x00, 0x00]), "the FontFile2 stream is the raw TrueType");
    let story_glyphs = snapshot.stories[0].content.chars().count();
    assert_eq!(text.matches("> Tj ET").count(), story_glyphs, "one positioned glyph per shaped character of \"{}\"", snapshot.stories[0].content);
    assert!(text.contains(" rg 10.000 450.000 40.000 40.000 re f Q"), "the rect frame is filled at its y-flipped page position");
    assert!(text.contains("0.4000 0.5000 0.7000 RG 1 w 50.000 370.000 100.000 80.000 re S Q"), "the inherited parent-page frame is stroked");
    assert!(text.contains("0.92 0.88 0.84 rg 136.000 25.000 60.000 40.000 re f Q"), "a missing image link paints its placeholder");

    let single = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-2"), None).expect("page pdf");
    let single_bytes = decode_base64(&single.data).expect("base64 pdf");
    let single_text = String::from_utf8_lossy(&single_bytes);
    assert_eq!(assert_pdf_structure(&single_bytes), 2);
    assert!(single_text.contains("/Count 1 >>"));
    assert_eq!(single_text.matches("> Tj ET").count(), 0, "page-2 carries no text");
}


#[test]
fn pdf_export_prints_a_placed_drawing_mark_and_its_kind() {
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.links[0].artifact_kind = "drawing".into();
    snapshot.links[0].artifact_ref = "drawing-1".into();
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("0.92 0.88 0.84 rg 136.000 25.000 60.000 40.000 re f Q"), "the placed frame still fills");
    assert!(text.contains("140.000 61.000 m 192.000 29.000 l S Q"), "a drawing prints its stroke mark inside the frame");
    let glyphs = snapshot.stories[0].content.chars().count() + "drawing".chars().count();
    assert_eq!(text.matches("> Tj ET").count(), glyphs, "the story and the placed kind are both shaped");
}

//#endregion 📕️PdfDocument

/// 🏃️ Dispatches one export through the exact factory and drives it on an original caller; the export is read at the terminal outcome, before the close ladder drains the shared queue.
fn drive_dispatched_worker(kind: LayoutExportKind, run: TestRun) -> (LayoutExportObserved, Result<LayoutExportCommit, String>) {
    let operation = operation();
    let bus = semio_framework::ActionBus::new();
    bus.register(LayoutExportJobFactory::new("layout-test")).expect("factory registration");
    let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
    let request = request(kind);
    let snapshot_owner = Arc::clone(&request.snapshot);
    let spec = semio_framework::ToolOperationSpec::new("layout-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, LayoutExportToolPayload { request, output_chunks: output_chunks.clone(), completion: None }, operation);
    let dispatch = bus.dispatch(spec).expect("exact factory dispatch");
    assert_eq!(bus.dispatch_count(), 1);
    let drained = output_chunks.clone();
    let outcome = drive_job_batch(dispatch.job, operation, run.generation, run.cancelled, run.site, || LayoutExportCommit::from_chunks(kind, "test", &drained));
    drop(snapshot_owner);
    outcome
}

#[test]
fn exact_factory_dispatch_is_deterministic_across_repeated_original_caller_drives() {
    for kind in [LayoutExportKind::Png, LayoutExportKind::Svg, LayoutExportKind::Pdf, LayoutExportKind::Package] {
        let mut outputs = Vec::new();
        for _ in 0..4 {
            match drive_dispatched_worker(kind, TestRun::new("layout.export.worker-test")) {
                (LayoutExportObserved::Complete { .. }, commit) => outputs.push(commit.expect("segmented output").data),
                outcome => panic!("unexpected {kind:?} outcome: {outcome:?}"),
            }
        }
        assert!(outputs.iter().all(|data| !data.is_empty()), "{kind:?} exported no bytes");
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]), "{kind:?} bytes diverged across original caller drives");
    }
}

#[test]
fn production_retained_wire_factory_decodes_before_reducer_and_closes_cancel_fault_and_success_owners() {
    /// 🪪️ Answers the dispatch together with the host's snapshot witness, which must outlive the close.
    fn dispatch(raw_verb: &str, kind: LayoutExportKind) -> (semio_framework::ToolJobDispatch, Arc<semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot>) {
        let operation = operation();
        let bus = semio_framework::ActionBus::new();
        bus.register(LayoutExportJobFactory::new("layout-retained-test")).expect("retained factory registration");
        assert!(bus.begin_exact_wire("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES + 1).is_err());
        let (admission, mut input) = bus.begin_exact_wire("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES).expect("maximum extent before encoding");
        // 🔐️ The wire is the command's `OpBinary` encoding, exactly as the host sends it.
        let command = match raw_verb {
            "exportSvg" => semio_s_artifact_layout_layout_editor::LayoutCommand::ExportSvg(semio_s_artifact_layout_layout_editor_commands::export_svg::ExportSvg { page_id: None }),
            "exportPdf" => semio_s_artifact_layout_layout_editor::LayoutCommand::ExportPdf(semio_s_artifact_layout_layout_editor_commands::export_pdf::ExportPdf { page_id: None }),
            other => panic!("fixture verb {other}"),
        };
        let raw = <semio_s_artifact_layout_layout_editor::LayoutCommand as protocol::OpBinary>::encode_op(&command).expect("fixture wire");
        for bytes in raw.chunks(semio_framework_action_bus::TOOL_WIRE_PAGE_BYTES) {
            input.admit_page(semio_framework_action_bus::ToolWirePage::try_copy_from(bytes).expect("bounded raw page")).expect("preadmitted page");
        }
        input.seal_admitted_prefix().expect("truthful encoded prefix");
        let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
        let request = request(kind);
        let witness = Arc::clone(&request.snapshot);
        let payload = LayoutExportToolPayload { request, output_chunks, completion: None };
        let spec = semio_framework::ToolOperationSpec::new("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, payload, operation);
        (bus.dispatch_wire_retained_with_spec(&admission, input, None, spec).unwrap_or_else(|_| panic!("retained production dispatch")), witness)
    }
    fn drive(dispatched: (semio_framework::ToolJobDispatch, Arc<semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot>), run: TestRun) -> LayoutExportObserved {
        let (dispatch, witness) = dispatched;
        let outcome = drive_test_job(dispatch.job, run);
        drop(witness);
        outcome
    }

    let run = TestRun::new("layout.retained-wire.worker-test");
    assert!(matches!(drive(dispatch("exportSvg", LayoutExportKind::Svg), run), LayoutExportObserved::Complete { .. }));
    assert!(matches!(drive(dispatch("exportPdf", LayoutExportKind::Svg), run), LayoutExportObserved::Fault(_)));
    assert!(matches!(drive(dispatch("exportSvg", LayoutExportKind::Svg), TestRun { cancelled: true, ..run }), LayoutExportObserved::Cancelled));
}

#[test]
fn exact_layout_out_media_factory_dispatches_a_real_reserved_job() {
    let operation = operation();
    let bus = semio_framework::ActionBus::new();
    bus.register(LayoutMediaExportJobFactory::new("layout-media-test")).expect("media factory registration");
    let request = request(LayoutExportKind::Svg);
    let snapshot_owner = Arc::clone(&request.snapshot);
    let payload = ArtifactReservedToolJob::new(fixture_export_job(operation, request));
    let spec = semio_framework::ToolOperationSpec::new("layout-media-test", LAYOUT_MEDIA_EXPORT_TOOL_ID, LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA, payload, operation);
    let dispatch = bus.dispatch(spec).expect("exact media factory dispatch");
    assert_eq!(bus.dispatch_count(), 1);
    assert!(matches!(drive_test_job(dispatch.job, TestRun::new("layout.media-export.worker-test")), LayoutExportObserved::Complete { .. }));
    drop(snapshot_owner);
}

/// 🧯️ One close slice that may release at most one item and one output chunk, so every stage proves its release is bounded.
const CHUNK_SLICE: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: OUTPUT_CHUNK_BYTES, maximum_depth: 4 };

#[test]
fn close_disposes_every_export_buffer_in_bounded_slices_and_rejects_an_unwitnessed_snapshot() {
    let operation = operation();
    let mut caller = layout_export_fixture_caller(operation, operation.generation);
    let mut job = fixture_export_job(operation, request(LayoutExportKind::Svg));
    let mut validation = JsonValidationCursor::new("[[", true).expect("json cursor");
    assert!(validation.advance("[[").is_err(), "an unfinished array leaves its containers open");
    assert_eq!(validation.stack.len(), 2);
    job.json_validation = Some(validation);
    job.typed_validation = Some(TypedJsonCursor { stack: vec![TypedJsonNode::Scalar { bytes: vec![1; 256], cursor: 0 }], ..Default::default() });
    job.package_json = Some(TypedJsonCursor { stack: vec![TypedJsonCursor::owned_string("z".repeat(OUTPUT_CHUNK_BYTES * 2))], ..Default::default() });
    job.rects.push(ExportRect { x: 0, y: 0, width: 1, height: 1, rgba: [0; 4] });
    job.output.append(&vec![1; OUTPUT_CHUNK_BYTES * 2]).expect("raw chunks");
    job.encoded.append(&vec![2; OUTPUT_CHUNK_BYTES * 2]).expect("encoded chunks");
    job.base64_tail = vec![3; 2];
    job.png_row = vec![4; OUTPUT_CHUNK_BYTES * 2];
    job.pdf_offsets.push(7);
    job.zip.entries.push(ZipEntry { name: "e".repeat(OUTPUT_CHUNK_BYTES * 2), crc: 0, size: 0, offset: 0 });
    job.zip.current_name = Some("current".into());
    job.request.as_mut().expect("request").preflight_json = Some("p".repeat(OUTPUT_CHUNK_BYTES * 2));
    job.output_chunks.as_ref().expect("shared output").push(vec![5; OUTPUT_CHUNK_BYTES]).expect("shared chunk");

    InteractiveJob::begin_close(&mut job);
    let mut slices = 0usize;
    while job.close_stage != LayoutExportCloseStage::OutputChunks {
        slices += 1;
        assert!(slices < LAYOUT_EXPORT_FIXTURE_TURN_BUDGET, "the close ladder never reached the paired output roots");
        match caller.turn(CHUNK_SLICE, |cx| InteractiveJob::close_step(&mut job, cx)).expect("bounded close turn") {
            InteractiveJobCloseStep::Pending { progress } => {
                assert!(progress.copied_items <= 1);
                assert!(progress.released_bytes <= OUTPUT_CHUNK_BYTES);
            }
            step => panic!("the buffer stages never block or complete before the paired roots: {step:?}"),
        }
    }
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
    let request = job.request.as_ref().expect("the snapshot supplier is still paired");
    assert!(request.page_id.is_none());
    assert!(request.preflight_json.is_none());
    assert!(request.parent_document_id.is_empty());
    assert!(request.canonical_base_revision_hex.is_empty());

    let failure = loop {
        slices += 1;
        assert!(slices < LAYOUT_EXPORT_FIXTURE_TURN_BUDGET, "the paired roots never reached the unwitnessed snapshot");
        match caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| InteractiveJob::close_step(&mut job, cx)) {
            Ok(InteractiveJobCloseStep::Pending { .. }) => {}
            Ok(step) => panic!("an unwitnessed close must not reach {step:?}"),
            Err(failure) => break failure,
        }
    };
    assert!(failure.primary_value().is_some_and(|error| error.message.contains("paired original supplier")), "{failure:?}");
    assert!(job.output_chunks.is_none());
    assert!(job.media_output_credit.is_none());
    drop(failure);
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
}

#[test]
fn close_zero_budget_preserves_the_exact_cursor_and_owner() {
    let operation = operation();
    let mut caller = layout_export_fixture_caller(operation, operation.generation);
    let witness = request(LayoutExportKind::Svg);
    let mut job = fixture_export_job(operation, witness.clone());
    job.output.append(&vec![1; OUTPUT_CHUNK_BYTES]).expect("raw chunk");
    InteractiveJob::begin_close(&mut job);
    job.close_stage = LayoutExportCloseStage::Output;
    let no_items = RetainedCloneGrant { maximum_items: 0, ..LAYOUT_EXPORT_FIXTURE_TURN };
    let no_bytes = RetainedCloneGrant { maximum_release_bytes: 0, ..LAYOUT_EXPORT_FIXTURE_TURN };
    for grant in [no_items, no_bytes] {
        let step = caller.turn(grant, |cx| InteractiveJob::close_step(&mut job, cx)).expect("zero budget close turn");
        assert_eq!(step, InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        assert_eq!(job.close_stage, LayoutExportCloseStage::Output);
        assert_eq!(job.output.chunks.len(), 1);
    }
    close_job(job, &mut caller);
    drop(witness);
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
}

#[test]
fn close_releases_the_witness_after_snapshot_handback() {
    let operation = operation();
    let mut caller = layout_export_fixture_caller(operation, operation.generation);
    let mut job = fixture_export_job(operation, request(LayoutExportKind::Svg));
    InteractiveJob::begin_close(&mut job);
    job.close_stage = LayoutExportCloseStage::SnapshotOwner;
    let step = caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| InteractiveJob::close_step(&mut job, cx)).expect("witness handback turn");
    assert_eq!(step, InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } });
    assert_eq!(job.close_stage, LayoutExportCloseStage::Complete);
    job.request.take();
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
}
