
/// 📥️ LAW (Import Document hang, ticket 26/09/23 S20 14c): the demo document's op log — ONE edit whose ops plant the
/// 25 KB `semio-emblem` asset and the carrier layers — comes back through the host document-archive door. Live,
/// Import Document of that archive answered `1/1 running` forever while an empty raster imported in 1.2 s
/// (`.tmp-ticket/wp-s20/generated/probe-raster-2.txt`). Two bounded halves pin where a stall lives: the raster store
/// initializer alone (the edit replay), then the whole archive load.
#[semio_framework_async_macros::async_test]
async fn a_demo_edit_archive_loads_back_through_the_document_archive_door() {
    use semio_framework_job::InteractiveJob;
    let mut app = mounted::mounted_app();
    mounted::dispatch(&mut app, RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::art_raster_demo::ID.into() })).await;
    let archive = PluginApp::document_archive(&*app).await.expect("the demo document reads as one archive");
    eprintln!("[DEBUG] raster demo archive pack={} spr={} members={}", archive.parent_pack.len(), archive.parent_spr.len(), archive.members.len());
    let envelope = store::parse_document_pack::<crate::RasterSnapshot, crate::op::RasterMutation>(&archive.parent_pack, &archive.parent_spr).await.expect("the archive's pack/spr pair parses").into_envelope();
    let edits = envelope.vcs.edits.len();
    let operation = semio_framework_job::OperationId(4_401);
    let generation = semio_framework_job::Generation(1);
    let mut job = crate::spr::raster_document_store_initialization_job(envelope, operation, generation);
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    let mut steps = 0usize;
    let mut terminal = None;
    while steps < 400_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        let outcome = job.step(&mut context);
        steps += 1;
        if outcome.is_terminal() {
            terminal = Some(outcome);
            break;
        }
    }
    eprintln!("[DEBUG] raster demo initializer edits={edits} steps={steps} terminal={}", terminal.is_some());
    let mut outcome = terminal.unwrap_or_else(|| panic!("the raster store initializer replaying {edits} edit(s) of the demo archive reached no terminal in {steps} steps"));
    assert!(matches!(outcome, semio_framework_job::StepOutcome::Complete(_)), "the initializer completes the demo archive");
    while !outcome.terminal_is_empty() {
        let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    for _ in 0..400_000 {
        if job.terminal_is_empty() {
            break;
        }
        let _ = job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    assert!(job.terminal_is_empty(), "the completed initializer closes to terminal-empty");
    drop(job);
    PluginApp::begin_document_archive_load(&mut *app, 91, archive).expect("archive admission");
    let started = std::time::Instant::now();
    let mut status = None;
    let mut polls = 0usize;
    let mut last = (0u64, 0u64);
    while started.elapsed() < std::time::Duration::from_secs(240) {
        let polled = PluginApp::poll_document_archive_load(&mut *app, 91).await.expect("archive status");
        polls += 1;
        last = (polled.completed, polled.total);
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        let _ = PluginApp::maintenance_step(&mut *app, 1, 4_096).expect("archive maintenance step");
    }
    eprintln!("[DEBUG] raster demo archive load polls={polls} last={last:?} ms={}", started.elapsed().as_millis());
    let status = status.unwrap_or_else(|| panic!("the demo archive load reached no terminal in {polls} polls (last progress {}/{})", last.0, last.1));
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    PluginApp::acknowledge_document_archive_load(&mut *app, 91).expect("archive acknowledgement");
    let snapshot = app.snapshot().expect("loaded snapshot");
    assert!(crate::raster_asset(&snapshot.assets, "semio-emblem").is_some(), "the emblem survives the archive door");
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}
