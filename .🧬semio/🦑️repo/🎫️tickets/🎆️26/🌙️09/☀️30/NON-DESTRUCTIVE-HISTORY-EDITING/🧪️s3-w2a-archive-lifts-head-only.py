"""🛬️ S3-W2A (S3-LOAD §6 item 5): the pure head-only law reaches its history again through the stepped archive load."""
import pathlib

LAW = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs")
plan = [
    ("/// query refuse with the same code, and a load with history lifts it.\n",
     "/// query refuse with the same code, and the whole-document archive load of the history (the only load path) lifts it over\n/// real polls, landing the source head and all 240 edits.\n"),
    ("    pure.load_document_pack(&files).await.expect(\"a load with history\");\n    assert!(!pure.time_travel.history_unavailable() && pure.history_snapshot().await.is_ok(), \"a load with history lifts head-only mode\");\n",
     "    let envelope = pure.store.envelope();\n"
     "    let dialect: ArtifactDialect = <ToyHistoryApp as ArtifactApp>::DIALECT.into();\n"
     "    let parent_spr = store::stamp_document_spr_identity(&files.spr, &envelope.id, <ToyHistoryApp as ArtifactApp>::DOCUMENT_SCHEMA, &dialect, envelope.owner.as_ref()).await.expect(\"the load identity stamp\");\n"
     "    let operation = 0x51;\n"
     "    PluginApp::begin_document_archive_load(&mut pure, operation, protocol::DocumentArchivePack { parent_pack: files.pack.clone(), parent_spr, members: Vec::new() }).expect(\"the archive load is admitted\");\n"
     "    let mut polls = 0;\n"
     "    let status = loop {\n"
     "        let status = PluginApp::poll_document_archive_load(&mut pure, operation).await.expect(\"the archive load status\");\n"
     "        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {\n"
     "            break status;\n"
     "        }\n"
     "        polls += 1;\n"
     "        assert!(polls < 1_000_000, \"the archive load reaches a terminal state\");\n"
     "    };\n"
     "    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, \"the archive load lands: {}\", if status.fault.is_empty() { String::new() } else { semio_framework_diagnostic::decode_fault_bytes(&status.fault).describe() });\n"
     "    PluginApp::acknowledge_document_archive_load(&mut pure, operation).expect(\"the terminal load is released\");\n"
     "    assert!(!pure.time_travel.history_unavailable() && pure.history_snapshot().await.is_ok(), \"the archive load with history lifts head-only mode\");\n"
     "    assert_eq!((pure.store.snapshot().expect(\"the loaded head\"), pure.store.envelope().vcs.edits.len()), (head, 240), \"the archive load lands the source head and its whole history\");\n"),
]
text = LAW.read_text(encoding="utf-8")
for old, new in plan:
    if text.count(old) != 1:
        raise SystemExit(f"anchor count {text.count(old)}: {old[:90]!r}")
    text = text.replace(old, new)
LAW.write_text(text, encoding="utf-8")
print(f"archive lifts head-only: {len(plan)} edits")
