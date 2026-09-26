    /// 🗺️ A document keeps accepting edits however many paths it holds: 400 edits, each setting a new
    /// path, so the document's resident state grows to 400 values. What a document holds between edits
    /// must never draw on the credit its next edit's I/O needs; when every held value occupied one DB
    /// I/O operation, a document stopped accepting edits after about a hundred distinct paths.
    #[semio_framework_async_macros::async_test]
    async fn a_document_keeps_accepting_edits_as_its_paths_multiply() {
        const PATHS: usize = 400;
        let root = tempdir("many-paths").await;
        let mut database = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
        let document = protocol::ArtifactId("many-paths".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let mut previous: Option<String> = None;
        for index in 0..PATHS {
            let id = format!("path-{index}");
            let dependencies: Vec<&str> = previous.iter().map(String::as_str).collect();
            let batch = db_artifact::CommandBatch::new(vec![envelope(&id, &dependencies, "alice", &document, &[(&format!("feature-{index:04}"), serde_json::json!(index))]).await]).await.unwrap();
            handle.submit(batch, db_artifact::SubmitOptions::default()).await.unwrap_or_else(|error| panic!("edit {index} was not delivered: {error:?}")).unwrap_or_else(|error| panic!("edit {index} was refused: {error:?}"));
            previous = Some(id);
        }
        let queried = handle.query(Query::Get { path: format!("feature-{:04}", PATHS - 1) }, Consistency::Canonical).await.unwrap();
        assert_eq!(decode_query_json(queried).await, serde_json::json!(PATHS - 1));
        drop(handle);
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    }

    /// 🤝️ Many documents grown at once each get every edit answered: 24 documents on one SQLite
    /// database, every document a chain of dependent edits submitted concurrently with all the others,
    /// the way a hub serves 24 open sockets. A submit that never answers (a lost wake in shared admission
    /// or in one document's runner) aborts the law with every document's progress instead of hanging it.
    #[cfg(feature = "sqlite")]
    #[semio_framework_async_macros::async_test]
    async fn concurrently_grown_documents_each_answer_every_edit() {
        const DOCUMENTS: usize = 24;
        const EDITS: usize = 40;
        let root = tempdir("concurrent-documents").await;
        let pool = test_worker_pool();
        let storage = crate::db_storage_sqlite::SqliteStorage::open(pool.clone(), &root.join("db.sqlite3")).await.unwrap();
        let mut database = Database::open(pool, DbConfig::for_profile(Profile::Prod), Arc::new(db_storage::DbBackend::Sqlite(storage))).await.unwrap();
        let progress: Arc<Vec<std::sync::atomic::AtomicUsize>> = Arc::new((0..DOCUMENTS).map(|_| std::sync::atomic::AtomicUsize::new(0)).collect());
        let finished = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let watchdog = {
            let progress = progress.clone();
            let finished = finished.clone();
            std::thread::spawn(move || {
                let mut last = Vec::new();
                let mut unchanged_since = std::time::Instant::now();
                while !finished.load(std::sync::atomic::Ordering::Acquire) {
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    let now: Vec<usize> = progress.iter().map(|count| count.load(std::sync::atomic::Ordering::Acquire)).collect();
                    if now != last {
                        last = now;
                        unchanged_since = std::time::Instant::now();
                    } else if unchanged_since.elapsed() > std::time::Duration::from_secs(90) {
                        eprintln!("concurrently grown documents stopped answering; acknowledged edits per document: {last:?}");
                        std::process::abort();
                    }
                }
            })
        };
        let mut handles = Vec::with_capacity(DOCUMENTS);
        for index in 0..DOCUMENTS {
            let document = protocol::ArtifactId(format!("concurrent-{index:02}"));
            handles.push((document.clone(), database.create_document(ArtifactSpec::new(document).await).await.unwrap()));
        }
        let mut growing: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + '_>>> = handles
            .iter()
            .enumerate()
            .map(|(index, (document, handle))| {
                let progress = progress.clone();
                Box::pin(async move {
                    let mut previous: Option<String> = None;
                    for edit in 0..EDITS {
                        let id = format!("concurrent-{index:02}-{edit}");
                        let dependencies: Vec<&str> = previous.iter().map(String::as_str).collect();
                        let batch = db_artifact::CommandBatch::new(vec![envelope(&id, &dependencies, "alice", document, &[(&format!("path-{}", edit % 8), serde_json::json!(format!("{edit}:{}", "c".repeat(2048))))]).await]).await.unwrap();
                        handle.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }).await.unwrap_or_else(|error| panic!("document {index} edit {edit} was not delivered: {error:?}")).unwrap_or_else(|error| panic!("document {index} edit {edit} was refused: {error:?}"));
                        progress[index].fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                        previous = Some(id);
                    }
                }) as std::pin::Pin<Box<dyn std::future::Future<Output = ()> + '_>>
            })
            .collect();
        std::future::poll_fn(|context| {
            growing.retain_mut(|document| document.as_mut().poll(context).is_pending());
            if growing.is_empty() { std::task::Poll::Ready(()) } else { std::task::Poll::Pending }
        })
        .await;
        drop(growing);
        finished.store(true, std::sync::atomic::Ordering::Release);
        watchdog.join().unwrap();
        for (document, handle) in &handles {
            assert_eq!(handle.frontier().await.unwrap().head_seq, EDITS as u64, "{document:?}");
        }
        drop(handles);
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
    }

