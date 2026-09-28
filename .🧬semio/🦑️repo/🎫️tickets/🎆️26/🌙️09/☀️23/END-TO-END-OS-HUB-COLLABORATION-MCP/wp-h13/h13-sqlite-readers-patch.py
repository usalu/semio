#!/usr/bin/env python3
"""📖️ H13 session 14c: SQLite WAL reader connections for the db storage backend (kernel-db, hub-closure-only).

Reads of a file database run on up to SQLITE_READERS read-only WAL connections beside the writer and each other: one-statement
and listing reads on any free reader, multi-step blob reads pinned to one reader inside one read transaction (one snapshot,
SQLite incremental blob I/O, no `db_io_stage` write). Writes, the catalog root, leases and in-memory databases stay on the one
writer connection unchanged. Adds the law `file_reads_run_on_wal_readers_beside_a_held_write_lock_and_never_write` and the
rusqlite `blob` feature. Operation cleanup deletes a `db_io_stage` row only when one exists, so a read's close never takes
the write lock. Idempotent; `--dry-run` reports pending hunks without writing.
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
DB = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db"
SQLITE = DB / "🗄️storage/🪶️sqlite/🦀️.rs"
TESTS = DB / "🗄️storage/🪶️sqlite/🧪️tests/🔬️sqlite-storage-unit/🦀️.rs"
CARGO = DB / "📦️packages/🦀️rust/Cargo.toml"

QUERY_ARMS_OLD = [
    """                DbIoTask::WalLength { document, index, .. } => {
                    let index = to_sql_i64(*index, "sqlite WAL index")?;
                    let length: Option<i64> = connection.query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(length.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))? as u64))))
                }
                DbIoTask::WalState { document, index, .. } => {
                    let index = to_sql_i64(*index, "sqlite WAL index")?;
                    let sealed: Option<i64> = connection.query_row("SELECT sealed FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    let state = decode_wal_segment_state(sealed.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))?)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::WalSegmentState(state))))
                }
                DbIoTask::WalList { document, output, .. } => Self::list_step(connection, "SELECT segment_index FROM wal_segment WHERE document = ?1 ORDER BY segment_index ASC LIMIT 1 OFFSET ?2", document, output),
""",
    """                DbIoTask::SnapshotLatest { document, .. } => {
                    let latest: Option<i64> = connection.query_row("SELECT MAX(generation) FROM snapshot_generation WHERE document = ?1", params![document.as_str()], |row| row.get(0)).map_err(sqlite_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLength(latest.map(|value| value as u64)))))
                }
                DbIoTask::SnapshotList { document, output, .. } => Self::list_step(connection, "SELECT generation FROM snapshot_generation WHERE document = ?1 ORDER BY generation ASC LIMIT 1 OFFSET ?2", document, output),
""",
    """                DbIoTask::PayloadExists { hash, .. } => {
                    let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM payload WHERE hash = ?1)", params![hash.to_string()], |row| row.get(0)).map_err(sqlite_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Exists(exists))))
                }
                DbIoTask::PayloadLength { hash, .. } => {
                    let length: Option<i64> = connection.query_row("SELECT len FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(length.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))? as u64))))
                }
""",
    """                DbIoTask::IndexList { document, output, .. } => Self::list_step(connection, "SELECT run_id FROM index_run WHERE document = ?1 ORDER BY run_id ASC LIMIT 1 OFFSET ?2", document, output),
""",
]

HELPERS = r'''
        /// 🔎️ The reads that never stage — one statement, or one listed row per step — answered alike on the writer and on a
        /// reader; `None` for every other task.
        fn query_step(connection: &Connection, task: &mut DbIoTask) -> Result<Option<(DbIoExecutionStep, Option<DbIoResult>)>, DbError> {
            let step = match task {
                DbIoTask::WalLength { document, index, .. } => {
                    let index = to_sql_i64(*index, "sqlite WAL index")?;
                    let length: Option<i64> = connection.query_row("SELECT length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    (DbIoExecutionStep::Complete, Some(DbIoResult::Length(length.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))? as u64)))
                }
                DbIoTask::WalState { document, index, .. } => {
                    let index = to_sql_i64(*index, "sqlite WAL index")?;
                    let sealed: Option<i64> = connection.query_row("SELECT sealed FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    let state = decode_wal_segment_state(sealed.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))?)?;
                    (DbIoExecutionStep::Complete, Some(DbIoResult::WalSegmentState(state)))
                }
                DbIoTask::WalList { document, output, .. } => Self::list_step(connection, "SELECT segment_index FROM wal_segment WHERE document = ?1 ORDER BY segment_index ASC LIMIT 1 OFFSET ?2", document, output)?,
                DbIoTask::SnapshotLatest { document, .. } => {
                    let latest: Option<i64> = connection.query_row("SELECT MAX(generation) FROM snapshot_generation WHERE document = ?1", params![document.as_str()], |row| row.get(0)).map_err(sqlite_err)?;
                    (DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLength(latest.map(|value| value as u64))))
                }
                DbIoTask::SnapshotList { document, output, .. } => Self::list_step(connection, "SELECT generation FROM snapshot_generation WHERE document = ?1 ORDER BY generation ASC LIMIT 1 OFFSET ?2", document, output)?,
                DbIoTask::PayloadExists { hash, .. } => {
                    let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM payload WHERE hash = ?1)", params![hash.to_string()], |row| row.get(0)).map_err(sqlite_err)?;
                    (DbIoExecutionStep::Complete, Some(DbIoResult::Exists(exists)))
                }
                DbIoTask::PayloadLength { hash, .. } => {
                    let length: Option<i64> = connection.query_row("SELECT len FROM payload WHERE hash = ?1", params![hash.to_string()], |row| row.get(0)).optional().map_err(sqlite_err)?;
                    (DbIoExecutionStep::Complete, Some(DbIoResult::Length(length.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))? as u64)))
                }
                DbIoTask::IndexList { document, output, .. } => Self::list_step(connection, "SELECT run_id FROM index_run WHERE document = ?1 ORDER BY run_id ASC LIMIT 1 OFFSET ?2", document, output)?,
                _ => return Ok(None),
            };
            Ok(Some(step))
        }

        /// 📍️ The rowid and byte length of the one row `sql` selects as `(rowid, length(bytes))`, inside the caller's snapshot.
        fn locate_blob(connection: &Connection, sql: &str, params: impl rusqlite::Params) -> Result<Option<(i64, u64)>, DbError> {
            connection.query_row(sql, params, |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)? as u64))).optional().map_err(sqlite_err)
        }

        /// 📖️ One page of a blob read on a reader, straight from its read transaction's snapshot: the row is located again in
        /// the same snapshot every step and only the pages the range covers are read (SQLite incremental blob I/O), so a
        /// multi-page read never mixes two states and never stages through `db_io_stage`.
        fn snapshot_blob_step(connection: &Connection, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            match task {
                DbIoTask::WalRead { document, index, range, output, .. } => {
                    let index = to_sql_i64(*index, "sqlite WAL index")?;
                    let (row, actual) = Self::locate_blob(connection, "SELECT rowid, length(bytes) FROM wal_segment WHERE document = ?1 AND segment_index = ?2", params![document.as_str(), index])?.ok_or_else(|| DbError::NotFound(format!("WAL segment {index} not found")))?;
                    let end = range.offset.checked_add(range.len).ok_or(DbError::LimitExceeded("sqlite WAL range"))?;
                    if end > actual {
                        return Err(DbError::InvalidArgument("WAL read range exceeds segment length".to_string()));
                    }
                    Self::blob_page_step(connection, c"wal_segment", row, range.offset, range.len, output)
                }
                DbIoTask::SnapshotRead { document, generation, output, .. } => {
                    let generation = to_sql_i64(*generation, "sqlite snapshot generation")?;
                    let (row, total) = Self::locate_blob(connection, "SELECT rowid, length(bytes) FROM snapshot_generation WHERE document = ?1 AND generation = ?2", params![document.as_str(), generation])?.ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} not found")))?;
                    Self::blob_page_step(connection, c"snapshot_generation", row, 0, total, output)
                }
                DbIoTask::PayloadGet { hash, output, .. } => {
                    let (row, total) = Self::locate_blob(connection, "SELECT rowid, length(bytes) FROM payload WHERE hash = ?1", params![hash.to_string()])?.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
                    Self::blob_page_step(connection, c"payload", row, 0, total, output)
                }
                DbIoTask::IndexRead { document, run_id, output, .. } => {
                    let run_id = to_sql_i64(*run_id, "sqlite index run")?;
                    let (row, total) = Self::locate_blob(connection, "SELECT rowid, length(bytes) FROM index_run WHERE document = ?1 AND run_id = ?2", params![document.as_str(), run_id])?.ok_or_else(|| DbError::NotFound(format!("index run {run_id} not found")))?;
                    Self::blob_page_step(connection, c"index_run", row, 0, total, output)
                }
                _ => Err(DbError::Internal("SQLite reader received a task that is not a blob read".to_string())),
            }
        }

        /// 📄️ Fills the output's current page from `[start + written, …)` of the blob in `table` row `row`, or seals the output
        /// once all `total` bytes are in.
        fn blob_page_step(connection: &Connection, table: &std::ffi::CStr, row: i64, start: u64, total: u64, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            check_len(total, MAX_BLOB_BYTES, "sqlite retained stage read")?;
            let written = output.len();
            let remaining = total.checked_sub(written as u64).ok_or_else(|| DbError::Corrupt("SQLite blob read passed the end of its row".to_string()))?;
            if remaining == 0 {
                return match output.seal_retained_step()? {
                    Some(pages) => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Pages(pages)))),
                    None => Ok((DbIoExecutionStep::Yield, None)),
                };
            }
            let len = (DB_IO_PAGE_BYTES - written % DB_IO_PAGE_BYTES).min(remaining as usize);
            let offset = usize::try_from(start + written as u64).map_err(|_| DbError::LimitExceeded("sqlite blob offset"))?;
            let mut fragment = [0_u8; DB_IO_PAGE_BYTES];
            connection.blob_open(c"main", table, c"bytes", row, true).map_err(sqlite_err)?.read_at_exact(&mut fragment[..len], offset).map_err(sqlite_err)?;
            output.write_fragment(&fragment[..len])?;
            Ok((DbIoExecutionStep::Yield, None))
        }

        /// 📖️ The reader `operation` already holds, else — when `may_claim` (the read has made no progress on the writer) — a
        /// free one; `None` sends the step to the writer connection.
        fn claim_reader(&self, operation: u64, may_claim: bool) -> Option<usize> {
            let mut owners = self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(index) = owners.iter().position(|owner| *owner == Some(operation)) {
                return Some(index);
            }
            let index = owners.iter().position(Option::is_none).filter(|_| may_claim)?;
            owners[index] = Some(operation);
            Some(index)
        }

        /// 🔚️ Ends the read transaction of the reader at `index` — committed after a completed read, rolled back otherwise; a
        /// connection that cannot end it is dropped (which rolls back) — and frees the reader. Only a failed commit is an error.
        fn end_read(&self, index: usize, operation: u64, reader: &mut Option<Connection>, completed: bool) -> Result<(), DbError> {
            let ended = match reader.as_ref() {
                Some(connection) if !connection.is_autocommit() => connection.execute_batch(if completed { "COMMIT" } else { "ROLLBACK" }).map_err(sqlite_err),
                _ => Ok(()),
            };
            if ended.is_err() {
                reader.take();
            }
            let mut owners = self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if owners[index] == Some(operation) {
                owners[index] = None;
            }
            if completed {
                ended
            } else {
                Ok(())
            }
        }

        /// 📖️ One read step of a file database on a WAL reader: opened read-only on first use, a read transaction per
        /// operation (the snapshot every step of a multi-step read sees), ended when the read completes or fails. `None` for
        /// in-memory databases (one connection is the database), for tasks that are not reads, and when every reader is held.
        fn reader_step(&self, operation: u64, task: &mut DbIoTask) -> Result<Option<(DbIoExecutionStep, Option<DbIoResult>)>, DbError> {
            let may_claim = match task {
                DbIoTask::WalRead { output, .. } | DbIoTask::SnapshotRead { output, .. } | DbIoTask::PayloadGet { output, .. } | DbIoTask::IndexRead { output, .. } => output.is_empty(),
                DbIoTask::WalList { output, .. } | DbIoTask::SnapshotList { output, .. } | DbIoTask::IndexList { output, .. } => output.is_empty(),
                DbIoTask::WalLength { .. } | DbIoTask::WalState { .. } | DbIoTask::SnapshotLatest { .. } | DbIoTask::PayloadExists { .. } | DbIoTask::PayloadLength { .. } => true,
                _ => return Ok(None),
            };
            if self.in_memory {
                return Ok(None);
            }
            let Some(index) = self.claim_reader(operation, may_claim) else { return Ok(None) };
            let mut reader = self.readers[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let outcome = Self::read_on(&mut reader, &self.path, task);
            if matches!(outcome, Ok((DbIoExecutionStep::Yield, _))) {
                return outcome.map(Some);
            }
            let ended = self.end_read(index, operation, &mut reader, outcome.is_ok());
            let step = outcome?;
            ended?;
            Ok(Some(step))
        }

        fn read_on(reader: &mut Option<Connection>, path: &DbIoText, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            if reader.is_none() {
                let connection = Connection::open_with_flags(path.as_str(), OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX | OpenFlags::SQLITE_OPEN_URI).map_err(sqlite_err)?;
                connection.pragma_update(None, "cache_size", -SQLITE_READER_CACHE_KIB).map_err(sqlite_err)?;
                *reader = Some(connection);
            }
            let connection = reader.as_ref().expect("SQLite reader connection opened");
            if connection.is_autocommit() {
                connection.execute_batch("BEGIN").map_err(sqlite_err)?;
            }
            match Self::query_step(connection, task)? {
                Some(step) => Ok(step),
                None => Self::snapshot_blob_step(connection, task),
            }
        }
'''

LAW = r'''
    /// 📖️ Reads of a file database run on WAL readers: while another connection holds the database's write lock, concurrent
    /// reads of every blob family (WAL range, snapshot generation, index run, payload; the neutral page-lifecycle lengths)
    /// complete with exactly the committed bytes an independent SQLite connection reads (third-party oracle), and they write
    /// nothing — the oracle's `data_version` and the `-wal` file length are unchanged across them. The staging reads they
    /// replace wrote `db_io_stage` rows and blocked behind that lock.
    #[semio_framework_async_macros::async_test]
    async fn file_reads_run_on_wal_readers_beside_a_held_write_lock_and_never_write() {
        let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
        let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = base.join(format!("sqlite-readers-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let database = root.join("readers.sqlite3");
        let storage = SqliteStorage::open(crate::db_storage::db_io_test_pool(), &database).await.unwrap();
        let document: ArtifactId = "sqlite-readers".into();
        let writer = storage.acquire_writer(&document).await.unwrap();
        let blobs: Vec<Vec<u8>> = fixture.lengths.iter().map(|length| (0..*length).map(|index| (index % fixture.pattern_modulo + fixture.pattern_addend) as u8).collect()).collect();
        let mut hashes = Vec::with_capacity(blobs.len());
        for (index, bytes) in blobs.iter().enumerate() {
            let key = index as u64 + 1;
            storage.create_segment(&writer, key).await.unwrap();
            storage.append(&writer, key, pages(bytes).await).await.unwrap();
            storage.write_generation(&document, key, pages(bytes).await).await.unwrap();
            storage.write_run(&document, key, pages(bytes).await).await.unwrap();
            hashes.push(storage.put(pages(bytes).await).await.unwrap());
        }

        let oracle = Connection::open(&database).unwrap();
        let data_version = |connection: &Connection| connection.query_row("PRAGMA data_version", [], |row| row.get::<_, i64>(0)).unwrap();
        let wal_len = || std::fs::metadata(root.join("readers.sqlite3-wal")).map(|metadata| metadata.len()).unwrap_or(0);
        let version_before = data_version(&oracle);
        let wal_before = wal_len();
        oracle.execute_batch("BEGIN IMMEDIATE").unwrap();
        let oracle_bytes = |sql: &str, key: i64| -> Vec<u8> { oracle.query_row(sql, params!["sqlite-readers", key], |row| row.get(0)).unwrap() };
        for (index, bytes) in blobs.iter().enumerate() {
            let key = index as i64 + 1;
            assert_eq!(&oracle_bytes("SELECT bytes FROM wal_segment WHERE document = ?1 AND segment_index = ?2", key), bytes);
            assert_eq!(&oracle_bytes("SELECT bytes FROM snapshot_generation WHERE document = ?1 AND generation = ?2", key), bytes);
            assert_eq!(&oracle_bytes("SELECT bytes FROM index_run WHERE document = ?1 AND run_id = ?2", key), bytes);
            let payload: Vec<u8> = oracle.query_row("SELECT bytes FROM payload WHERE hash = ?1", params![hashes[index].to_string()], |row| row.get(0)).unwrap();
            assert_eq!(&payload, bytes);
        }

        let storage_ref = &storage;
        let document_ref = &document;
        let reads: Vec<Result<(), String>> = std::thread::scope(|scope| {
            let threads: Vec<_> = blobs
                .iter()
                .enumerate()
                .flat_map(|(index, bytes)| ["wal", "snapshot", "index", "payload"].map(|family| (index, bytes, family)))
                .map(|(index, bytes, family)| {
                    let hash = &hashes[index];
                    scope.spawn(move || {
                        crate::db_actor::block_on(async move {
                            let key = index as u64 + 1;
                            let read = match family {
                                "wal" => storage_ref.read(document_ref, key, ByteRange { offset: 0, len: bytes.len() as u64 }).await,
                                "snapshot" => storage_ref.read_generation(document_ref, key).await,
                                "index" => storage_ref.read_run(document_ref, key).await,
                                _ => storage_ref.get(hash).await,
                            };
                            let mut pages = read.map_err(|error| format!("{family} read of blob {index} beside the held write lock failed: {error:?}"))?;
                            let equal = pages == *bytes;
                            while pages.close_step().map_err(|error| format!("{family} pages of blob {index} did not close: {error:?}"))?.is_some() {}
                            if equal {
                                Ok(())
                            } else {
                                Err(format!("{family} read of blob {index} differs from the committed bytes"))
                            }
                        })
                    })
                })
                .collect();
            threads.into_iter().map(|thread| thread.join().unwrap()).collect()
        });
        oracle.execute_batch("ROLLBACK").unwrap();
        for read in reads {
            read.unwrap();
        }
        assert_eq!(data_version(&oracle), version_before, "a read committed a write");
        assert_eq!(wal_len(), wal_before, "a read grew the SQLite WAL");

        writer.release().await.unwrap();
        storage.close().await.unwrap();
        drop(oracle);
        let _ = std::fs::remove_dir_all(root);
    }
'''

EDITS = [
    (SQLITE, "use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};", "use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};"),
    (
        SQLITE,
        "    const SQLITE_OPERATION_OWNERS: usize = crate::db_storage::DB_IO_LEDGER_ITEMS;\n",
        "    const SQLITE_OPERATION_OWNERS: usize = crate::db_storage::DB_IO_LEDGER_ITEMS;\n"
        "    /// 📖️ WAL reader connections of a file database: a read that starts while one is free runs on it, beside the writer and\n"
        "    /// the other readers, inside its own read transaction, and never writes.\n"
        "    const SQLITE_READERS: usize = 16;\n"
        "    /// 📖️ Page-cache ceiling of one reader in KiB (`PRAGMA cache_size` takes it negated): blob reads stream each page once.\n"
        "    const SQLITE_READER_CACHE_KIB: i64 = 1024;\n",
    ),
    (
        SQLITE,
        "        connection: Mutex<Option<Connection>>,\n        path: DbIoText,\n",
        "        connection: Mutex<Option<Connection>>,\n        readers: [Mutex<Option<Connection>>; SQLITE_READERS],\n        reader_owners: Mutex<[Option<u64>; SQLITE_READERS]>,\n        path: DbIoText,\n",
    ),
    (
        SQLITE,
        "                connection: Mutex::new(None),\n                path,\n",
        "                connection: Mutex::new(None),\n                readers: [const { Mutex::new(None) }; SQLITE_READERS],\n                reader_owners: Mutex::new([None; SQLITE_READERS]),\n                path,\n",
    ),
    (
        SQLITE,
        "            if !exists && insert(connection, operation).map_err(sqlite_err)? == 0 {\n                return Err(missing());\n            }\n            Ok(())\n        }\n    }\n    //#endregion 🔖️Authority",
        "            if !exists && insert(connection, operation).map_err(sqlite_err)? == 0 {\n                return Err(missing());\n            }\n            Ok(())\n        }\n" + HELPERS + "    }\n    //#endregion 🔖️Authority",
    ),
    (
        SQLITE,
        "            if matches!(task, DbIoTask::BackendClose { .. }) {\n                return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)));\n            }\n            let mut owner = self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n            let connection = owner.as_mut().ok_or(DbError::Closed)?;\n            let sql_operation = Self::operation(operation)?;\n            match task {\n",
        "            if matches!(task, DbIoTask::BackendClose { .. }) {\n                return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)));\n            }\n            if let Some(step) = self.reader_step(operation, task)? {\n                return Ok(step);\n            }\n            let mut owner = self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n            let connection = owner.as_mut().ok_or(DbError::Closed)?;\n            if let Some(step) = Self::query_step(connection, task)? {\n                return Ok(step);\n            }\n            let sql_operation = Self::operation(operation)?;\n            match task {\n",
    ),
    *[(SQLITE, arm, "") for arm in QUERY_ARMS_OLD],
    (
        SQLITE,
        '                DbIoTask::BackendOpen { .. } | DbIoTask::BackendClose { .. } => unreachable!("SQLite control tasks handled before connection lock"),\n',
        "                DbIoTask::WalLength { .. }\n"
        "                | DbIoTask::WalState { .. }\n"
        "                | DbIoTask::WalList { .. }\n"
        "                | DbIoTask::SnapshotLatest { .. }\n"
        "                | DbIoTask::SnapshotList { .. }\n"
        "                | DbIoTask::PayloadExists { .. }\n"
        "                | DbIoTask::PayloadLength { .. }\n"
        "                | DbIoTask::IndexList { .. }\n"
        "                | DbIoTask::BackendOpen { .. }\n"
        '                | DbIoTask::BackendClose { .. } => unreachable!("SQLite query and control tasks are answered before the writer match"),\n',
    ),
    (
        SQLITE,
        "        fn close_operation_step(&self, operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {\n            let mut owner = self.connection.lock()",
        "        fn close_operation_step(&self, operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {\n"
        "            let held = self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().position(|owner| *owner == Some(operation));\n"
        "            if let Some(index) = held {\n"
        "                let mut reader = self.readers[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n"
        "                self.end_read(index, operation, &mut reader, false)?;\n"
        "                return Ok(false);\n"
        "            }\n"
        "            let mut owner = self.connection.lock()",
    ),
    (
        SQLITE,
        "            if cursor == self.payload_hashes.len() {\n                self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();\n                return Ok(false);\n            }\n            if cursor == self.payload_hashes.len() + 1 {\n",
        "            let readers_end = self.payload_hashes.len() + SQLITE_READERS;\n"
        "            if cursor < readers_end {\n"
        "                let index = cursor - self.payload_hashes.len();\n"
        "                self.readers[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();\n"
        "                self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index] = None;\n"
        "                return Ok(false);\n"
        "            }\n"
        "            if cursor == readers_end {\n                self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();\n                return Ok(false);\n            }\n            if cursor == readers_end + 1 {\n",
    ),
    (
        SQLITE,
        "                && self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()\n                && self.canonical_database",
        "                && self.connection.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()\n"
        "                && self.readers.iter().all(|reader| reader.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())\n"
        "                && self.reader_owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().all(Option::is_none)\n"
        "                && self.canonical_database",
    ),
    (
        SQLITE,
        "            if let Some(connection) = owner.as_mut() {\n                if connection.execute(\"DELETE FROM db_io_stage WHERE operation = ?1\", params![Self::operation(operation)?]).map_err(sqlite_err)? != 0 {\n                    return Ok(false);\n                }\n            }\n",
        "            if let Some(connection) = owner.as_mut() {\n"
        "                let sql_operation = Self::operation(operation)?;\n"
        "                let staged: bool = connection.query_row(\"SELECT EXISTS(SELECT 1 FROM db_io_stage WHERE operation = ?1)\", params![sql_operation], |row| row.get(0)).map_err(sqlite_err)?;\n"
        "                if staged {\n"
        "                    connection.execute(\"DELETE FROM db_io_stage WHERE operation = ?1\", params![sql_operation]).map_err(sqlite_err)?;\n"
        "                    return Ok(false);\n"
        "                }\n"
        "            }\n",
    ),
    (
        TESTS,
        "    #[test]\n    fn wal_segment_state_decoder_rejects_non_boolean_storage_values() {",
        LAW.lstrip("\n") + "\n    #[test]\n    fn wal_segment_state_decoder_rejects_non_boolean_storage_values() {",
    ),
    (
        CARGO,
        'rusqlite = { version = "0.38.0", features = ["bundled"], optional = true }',
        'rusqlite = { version = "0.38.0", features = ["blob", "bundled"], optional = true }',
    ),
]


def main() -> int:
    dry = "--dry-run" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in {SQLITE, TESTS, CARGO}}
    pending = problems = 0
    for path, old, new in EDITS:
        text = texts[path]
        applied = (new in text) if new else (old not in text)
        if applied:
            print(f"applied already: {path.parent.name}/{path.name} :: {old.strip()[:70]}")
            continue
        count = text.count(old)
        if count != 1:
            print(f"PROBLEM ({count} matches): {path.parent.name}/{path.name} :: {old.strip()[:70]}")
            problems += 1
            continue
        pending += 1
        print(f"{'would apply' if dry else 'apply'}: {path.parent.name}/{path.name} :: {old.strip()[:70]}")
        texts[path] = text.replace(old, new, 1)
    if problems:
        print(f"{problems} problems — nothing written")
        return 1
    if not dry:
        for path, text in texts.items():
            if text != path.read_text(encoding="utf-8"):
                path.write_text(text, encoding="utf-8")
    print(f"{'dry run: ' if dry else ''}{pending} pending")
    return 0


if __name__ == "__main__":
    sys.exit(main())
