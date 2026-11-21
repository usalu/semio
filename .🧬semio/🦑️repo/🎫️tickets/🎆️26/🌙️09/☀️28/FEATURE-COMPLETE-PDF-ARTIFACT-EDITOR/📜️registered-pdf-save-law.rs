    let revision = app.document_base_revision().expect("edited document revision");
    let document = app.document_identity().expect("edited document identity");
    app.bind_instance_id(meta.instance_id).await;
    let mut submitted = None;
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        if app.submit_media_export_checked(semio_plugin::NATURAL_FILE_PORT, &document, revision, None, &mut submitted, scope.cx()).await.expect("checked PDF Save submission") { break }
        app.maintenance_step(scope.cx(), policy.close).expect("PDF Save admission maintenance");
    }
    let handle = submitted.expect("registered PDF Save handle");
    assert_eq!(handle.parent_document_id.as_str(), document.as_str());
    let mut completed = None;
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        match laws::poll_fixture_media_export(&mut app, &handle, scope.cx(), policy.close).await.expect("registered PDF Save poll") {
            semio_plugin::ArtifactMediaExportPoll::Running { .. } => {},
            semio_plugin::ArtifactMediaExportPoll::Complete(result) => { completed = Some(result); break },
            other => panic!("registered PDF Save did not produce its segmented output: {other:?}"),
        }
    }
    let result = completed.expect("registered PDF Save completes");
    assert_eq!(result.schema, codec.format_kind);
    assert_eq!(result.mime_type, codec.media_type);
    let status = laws::retain_fixture_media_export_result(&mut app, &handle, result, scope.cx(), policy.close).await.expect("PDF Save publishes download status");
    assert_eq!(status.port, semio_plugin::NATURAL_FILE_PORT);
    let mut bytes = Vec::new();
    let mut terminal = false;
    type Output = semio_plugin::ArtifactOutputChunks;
    for _ in 0..=(Output::MAXIMUM_TOTAL_BYTES / Output::CHUNK_BYTES) {
        if let Some(chunk) = laws::take_fixture_media_export_chunk(&mut app, &handle, &mut terminal, scope.cx(), policy.close).await.expect("PDF Save download page") {
            assert!(!chunk.is_empty() && chunk.len() <= Output::CHUNK_BYTES);
            bytes.extend_from_slice(&chunk);
        }
        if terminal { break }
    }
    assert!(terminal, "PDF Save downloads a terminal page sequence");
    assert_eq!(bytes.len() as u64, status.total_bytes);
    assert_eq!(app.document_base_revision().unwrap(), revision);
    assert_eq!(app.document_identity().as_deref(), Some(document.as_str()));
