    #[test]
    fn folder_sqlite_storage_round_trips_by_document_id() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderSqliteStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read("doc-a").expect("read empty"), None, "absent document reads as None");
        let env_a: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "doc-a", DemoProjection { n: 3 }, None);
        let env_b: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "doc-b", DemoProjection { n: 7 }, None);
        storage
            .write("doc-a", "demo/v1", &serde_json::to_string(&env_a).expect("json a"))
            .expect("write a");
        storage
            .write("doc-b", "demo/v1", &serde_json::to_string(&env_b).expect("json b"))
            .expect("write b");
        let loaded_a: DocumentEnvelope<DemoProjection, DemoOperation> =
            serde_json::from_str(&storage.read("doc-a").expect("read a").expect("some a")).expect("parse a");
        let loaded_b: DocumentEnvelope<DemoProjection, DemoOperation> =
            serde_json::from_str(&storage.read("doc-b").expect("read b").expect("some b")).expect("parse b");
        assert_eq!(loaded_a.vcs.initial_projection.n, 3, "documents are keyed independently");
        assert_eq!(loaded_b.vcs.initial_projection.n, 7);

        let env_a2: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "doc-a", DemoProjection { n: 5 }, None);
        storage
            .write("doc-a", "demo/v1", &serde_json::to_string(&env_a2).expect("json a2"))
            .expect("upsert a");
        let reloaded_a: DocumentEnvelope<DemoProjection, DemoOperation> =
            serde_json::from_str(&storage.read("doc-a").expect("reread a").expect("some a2")).expect("parse a2");
        assert_eq!(reloaded_a.vcs.initial_projection.n, 5, "writing the same id upserts in place");

        let mut ids = storage.document_ids().expect("document ids");
        ids.sort();
        assert_eq!(ids, vec!["doc-a".to_string(), "doc-b".to_string()], "folder indexes every document");
    }

    #[test]
    fn folder_text_storage_round_trips_dsl_and_appends_ops() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderTextStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read("demo", "demo").expect("read empty"), None, "absent document reads as None");

        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        let files = print_document_text(store.envelope()).expect("print document text");
        storage.write("demo", "demo", &files).expect("write");

        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply 2");
        let second_edit = store.envelope().vcs.edits.last().expect("second edit");
        storage
            .append_ops("demo", "demo", &print_edit_lines(second_edit).expect("print edit lines"))
            .expect("append ops");

        let reloaded = storage.read("demo", "demo").expect("read").expect("some");
        let parsed: ParsedDocumentText<DemoProjection, DemoOperation> =
            parse_document_text(&reloaded.dsl, &reloaded.ops).unwrap_or_else(|error| panic!("parse: {error}"));
        assert_eq!(parsed.projection.n, 2, "write + append reconstructs every edit in order");

        assert_eq!(storage.document_ids("demo").expect("document ids"), vec!["demo".to_string()]);
    }

    #[test]
    fn folder_text_storage_round_trips_pack() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderTextStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read_pack("demo", "demo").expect("read empty"), None, "absent pack reads as None");

        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        let files = print_document_pack(store.envelope()).expect("print document pack");
        let dsl_mirror = store.envelope().vcs.initial_projection.print_dsl();
        storage.write_pack("demo", "demo", &files, &dsl_mirror).expect("write pack");

        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply 2");
        let second_edit = store.envelope().vcs.edits.last().expect("second edit");
        storage
            .append_ops("demo", "demo", &print_edit_lines(second_edit).expect("print edit lines"))
            .expect("append ops");

        let reloaded = storage.read_pack("demo", "demo").expect("read pack").expect("some");
        let parsed: ParsedDocumentText<DemoProjection, DemoOperation> =
            parse_document_pack(&reloaded.pack, &reloaded.ops).unwrap_or_else(|error| panic!("parse: {error}"));
        assert_eq!(parsed.projection.n, 2, "pack write + append reconstructs every edit in order");

        // The always-written DSL mirror must also be on disk and agree with the pack path.
        let mirror = std::fs::read_to_string(storage.pack_path("demo", "demo").with_extension("")).expect("dsl mirror on disk");
        assert_eq!(DemoProjection::parse_dsl(&mirror).expect("parse mirror").n, 0, "mirror captures the initial projection, not later edits");
    }

    #[test]
    fn folder_sqlite_storage_round_trips_pack() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderSqliteStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read_pack("doc-a").expect("read empty"), None, "absent pack reads as None");

        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "doc-a", DemoProjection { n: 3 }, None);
        let pack_bytes = envelope.vcs.initial_projection.encode_pack();
        storage
            .write_pack("doc-a", "demo/v1", &serde_json::to_string(&envelope).expect("json"), &pack_bytes)
            .expect("write pack");

        let reloaded = storage.read_pack("doc-a").expect("read pack").expect("some");
        assert_eq!(reloaded, pack_bytes, "sqlite pack column round trips exact bytes");

        // `write` (JSON-only, no pack argument) must not clobber a previously-written pack.
        storage
            .write("doc-a", "demo/v1", &serde_json::to_string(&envelope).expect("json again"))
            .expect("plain write");
        assert_eq!(
            storage.read_pack("doc-a").expect("read pack after plain write"),
            Some(pack_bytes),
            "plain write preserves the existing pack column (upsert only touches schema/json/updated_at)"
        );
    }

    #[test]
    fn blob_store_put_get_dedupes_idempotently() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderSqliteStorage::new(dir.path().to_path_buf());
        let bytes = b"hello content-addressed world";
        assert!(!storage.has("not-a-real-hash").expect("has on empty store"));

        let first = storage.put(bytes, "text/plain").expect("first put");
        let second = storage.put(bytes, "text/plain").expect("second put");
        assert_eq!(first, second, "putting identical bytes twice is idempotent and dedupes by hash");
        assert_eq!(first.size, bytes.len() as u64);
        assert_eq!(first.media_type, "text/plain");

        assert!(storage.has(&first.hash).expect("has after put"));
        let fetched = storage.get(&first.hash).expect("get").expect("blob present");
        assert_eq!(fetched, bytes);

        let other = storage.put(b"different content", "text/plain").expect("put other");
        assert_ne!(other.hash, first.hash, "different bytes hash differently");

        storage.delete(&first.hash).expect("delete");
        assert!(!storage.has(&first.hash).expect("has after delete"));
        assert_eq!(storage.get(&first.hash).expect("get after delete"), None);
    }