//! 📁️ Db filesystem storage backend.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, check_len};
use db_io_core::{CatalogStorage, DB_IO_LEDGER_ITEMS, DB_IO_MAX_READ_BYTES, DB_IO_PAGE_BYTES, DB_IO_TEXT_BYTES, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoExecutionStep, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DbIoWriterReleaseStep, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage, WalWriterPermit, db_io_operation_owner_slot};
use db_io_driver::{DbStorageOpenRejected, WalWriterGuard as _, close_db_io_backend, open_db_io_backend_admitted, register_db_io_backend_prepared_with_use, retire_db_io_backend, submit_db_io_task_admitted};
use db_io_registry::{DbIoBackendKind, DbIoBackendRollbackReservation};
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use std::{sync::Arc, sync::Mutex, sync::atomic::Ordering, task::Context};
#[cfg(test)]
use db_io_driver::{block_on_ready, db_io_test_pool, poll_once};
#[cfg(test)]
use std::sync::atomic::AtomicU64;

//#region 🔖️Fs
/// 📁️ The zero-touch default `DbStorage`: pure files under a root directory, no new C
/// dependency (see module doc for the wasm32/native-only gating rationale). Layout under `root`:
/// `wal/<doc>/segment-<index>.bin` (+ `.sealed` marker once sealed), `snapshot/<doc>/gen-<n>.pack`,
/// `payload/<hash[0..2]>/<hash>.bin` (blake3 CAS, two-hex-char sharding), `catalog/root.bin`
/// (8-byte LE epoch prefix + opaque bytes), `index/<doc>/run-<id>.bin`, `lease/<resource>.bin`.
/// Every multi-step write (`cas_root`, lease grants, snapshot/index/payload writes) goes through
/// `pack::write_atomic` (temp file + `fsync` + `rename`) so a reader never observes a torn file.
/// Every blocking filesystem step runs as one typed task grant on the process I/O lane.
#[cfg(not(target_arch = "wasm32"))]
mod fs_storage {
    use super::check_len;
    use db_io_driver::{WalFileWriterGuard, WalWriterTable};
    use super::{
        close_db_io_backend, register_db_io_backend_prepared_with_use, retire_db_io_backend, submit_db_io_task_admitted, ArtifactId, ByteRange, ContentHash, DbError, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoBackendKind, DbIoBackendRollbackReservation,
        DbIoExecutionStep, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DbStorageOpenRejected, DurabilityClass, EpochFence, LeaseInfo, DB_IO_MAX_READ_BYTES,
    };
    use super::{CatalogStorage, DbIoWriterReleaseStep, IndexStorage, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage, WalWriterPermit};
    use semio_framework_async::WorkerPool;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};

    /// 🚨️ Wraps a `std::io::Error` into `DbError::Io` — the only place `std::io::Error` is
    /// allowed to appear, per the contract's no-`std::io::Error`-in-public-signatures rule.
    #[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                             // 🚫️async: E4 fn-pointer slot
    fn io_err(err: std::io::Error) -> DbError {
        DbError::Io(err.to_string())
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum FsLifecyclePhase {
        DirectoryCreated,
        DirectoryParentSynced,
        SegmentCreated,
        SegmentFileSynced,
        SegmentParentSynced,
        MarkerCreated,
        MarkerFileSynced,
        MarkerParentSynced,
        SegmentDeleted,
        SegmentDeleteParentSynced,
        MarkerDeleted,
        MarkerDeleteParentSynced,
        ReplacementRenamed,
        ReplacementParentSynced,
        WalFileSynced,
        WalParentSynced,
    }

    #[derive(Default)]
    struct FsLifecycle {
        durable_directories: Mutex<FsDurableDirectories>,
        full_sync: FsFullSyncGroup,
        #[cfg(test)]
        state: Mutex<FsLifecycleState>,
    }

    /// 🚰️ One drive write-cache flush that every concurrent durable file sync of this root
    /// shares. On Apple platforms a durable sync is the file's own `fsync` (its pages reach the
    /// drive) followed by a device-wide `F_FULLFSYNC` (the drive's cache reaches the media): a flush
    /// that starts after a member's `fsync` returned covers that member, so two dozen documents
    /// committing at once pay a few cache flushes instead of one each, all serialised by the drive.
    #[derive(Default)]
    struct FsFullSyncGroup {
        epochs: Mutex<FsFullSyncEpochs>,
        flushed: std::sync::Condvar,
    }

    #[derive(Default)]
    struct FsFullSyncEpochs {
        requested: u64,
        completed: u64,
        flushing: bool,
    }

    /// 📁️ How many directories one executor remembers as durable before it forgets them all
    /// and walks their ancestors again: a document owns four (wal, index, snapshot, payload), so the
    /// bound covers every document a hub keeps mounted.
    const FS_DURABLE_DIRECTORIES: usize = 4_096;

    /// 📁️ Directories this executor already made durable (created, every ancestor's entry
    /// synced): a WAL sync or run replacement in one of them skips the whole-ancestry fsync walk,
    /// which cost one directory fsync per path component on every durable write.
    #[derive(Default)]
    struct FsDurableDirectories {
        paths: std::collections::HashSet<PathBuf>,
    }

    #[cfg(test)]
    struct FsLifecycleState {
        trace: [Option<FsLifecyclePhase>; 256],
        count: usize,
        fail: Option<FsLifecyclePhase>,
    }

    #[cfg(test)]
    impl Default for FsLifecycleState {
        fn default() -> Self {
            Self { trace: [None; 256], count: 0, fail: None }
        }
    }

    impl FsLifecycle {
        fn hit(&self, phase: FsLifecyclePhase) -> Result<(), DbError> {
            #[cfg(test)]
            {
                let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let count = state.count;
                if count < state.trace.len() {
                    state.trace[count] = Some(phase);
                    state.count += 1;
                }
                if state.fail == Some(phase) {
                    state.fail = None;
                    return Err(DbError::Io(format!("filesystem lifecycle injected after {phase:?}")));
                }
            }
            #[cfg(not(test))]
            let _ = phase;
            Ok(())
        }
    }

    /// 🧱️ A directory-name durability barrier on the admitted filesystem lane; unsupported flushes fail closed.
    fn sync_directory(path: &Path) -> Result<(), DbError> {
        let path = if path.as_os_str().is_empty() { Path::new(".") } else { path };
        #[cfg(windows)]
        let file = {
            use std::os::windows::fs::OpenOptionsExt as _;
            std::fs::OpenOptions::new().read(true).write(true).custom_flags(0x0200_0000).open(path).map_err(io_err)?
        };
        #[cfg(not(windows))]
        let file = std::fs::File::open(path).map_err(io_err)?;
        #[cfg(target_vendor = "apple")]
        {
            use std::os::fd::AsRawFd as _;
            unsafe extern "C" {
                fn fsync(fd: std::ffi::c_int) -> std::ffi::c_int;
            }
            if unsafe { fsync(file.as_raw_fd()) } != 0 {
                return Err(io_err(std::io::Error::last_os_error()));
            }
            Ok(())
        }
        #[cfg(not(target_vendor = "apple"))]
        file.sync_all().map_err(io_err)
    }

    /// 💾️ Makes `file`'s bytes durable on the medium (see `FsFullSyncGroup`); elsewhere the
    /// platform's own full `fsync`.
    fn durable_file_sync(file: &std::fs::File, lifecycle: &FsLifecycle) -> Result<(), DbError> {
        #[cfg(target_vendor = "apple")]
        {
            use std::os::fd::AsRawFd as _;
            const F_FULLFSYNC: std::ffi::c_int = 51;
            unsafe extern "C" {
                fn fsync(fd: std::ffi::c_int) -> std::ffi::c_int;
                fn fcntl(fd: std::ffi::c_int, command: std::ffi::c_int, ...) -> std::ffi::c_int;
            }
            if unsafe { fsync(file.as_raw_fd()) } != 0 {
                return Err(io_err(std::io::Error::last_os_error()));
            }
            let group = &lifecycle.full_sync;
            let mut epochs = group.epochs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            epochs.requested += 1;
            let ticket = epochs.requested;
            loop {
                if epochs.completed >= ticket {
                    return Ok(());
                }
                if epochs.flushing {
                    epochs = group.flushed.wait(epochs).unwrap_or_else(std::sync::PoisonError::into_inner);
                    continue;
                }
                epochs.flushing = true;
                let target = epochs.requested;
                drop(epochs);
                let failure = (unsafe { fcntl(file.as_raw_fd(), F_FULLFSYNC) } == -1).then(std::io::Error::last_os_error);
                epochs = group.epochs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                epochs.flushing = false;
                if failure.is_none() {
                    epochs.completed = epochs.completed.max(target);
                }
                group.flushed.notify_all();
                if let Some(error) = failure {
                    return Err(io_err(error));
                }
            }
        }
        #[cfg(not(target_vendor = "apple"))]
        {
            let _ = lifecycle;
            file.sync_all().map_err(io_err)
        }
    }

    fn durable_directory(path: &Path, lifecycle: &FsLifecycle) -> Result<(), DbError> {
        if path.as_os_str().is_empty() {
            return Ok(());
        }
        if lifecycle.durable_directories.lock().unwrap_or_else(std::sync::PoisonError::into_inner).paths.contains(path) && path.is_dir() {
            return Ok(());
        }
        let parent = path.parent();
        if let Some(parent) = parent {
            durable_directory(parent, lifecycle)?;
        }
        match std::fs::create_dir(path) {
            Ok(()) => lifecycle.hit(FsLifecyclePhase::DirectoryCreated)?,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists && path.is_dir() => {}
            Err(error) => return Err(io_err(error)),
        }
        sync_directory(parent.unwrap_or(path))?;
        lifecycle.hit(FsLifecyclePhase::DirectoryParentSynced)?;
        let mut durable = lifecycle.durable_directories.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if durable.paths.len() == FS_DURABLE_DIRECTORIES {
            durable.paths.clear();
        }
        durable.paths.insert(path.to_path_buf());
        Ok(())
    }

    fn durable_wal_create(dir: &Path, index: u64, lifecycle: &FsLifecycle) -> Result<(), DbError> {
        durable_directory(dir, lifecycle)?;
        let file = std::fs::OpenOptions::new().write(true).create_new(true).open(segment_path(dir, index)).map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                DbError::AlreadyExists(format!("wal segment {index} already exists"))
            } else {
                io_err(error)
            }
        })?;
        lifecycle.hit(FsLifecyclePhase::SegmentCreated)?;
        durable_file_sync(&file, lifecycle)?;
        lifecycle.hit(FsLifecyclePhase::SegmentFileSynced)?;
        drop(file);
        sync_directory(dir)?;
        lifecycle.hit(FsLifecyclePhase::SegmentParentSynced)
    }

    fn durable_wal_seal(dir: &Path, index: u64, lifecycle: &FsLifecycle) -> Result<(), DbError> {
        std::fs::metadata(segment_path(dir, index)).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?;
        let marker = sealed_marker_path(dir, index);
        let file = match std::fs::OpenOptions::new().write(true).create_new(true).open(&marker) {
            Ok(file) => {
                lifecycle.hit(FsLifecyclePhase::MarkerCreated)?;
                file
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                let file = std::fs::OpenOptions::new().write(true).open(&marker).map_err(io_err)?;
                let metadata = file.metadata().map_err(io_err)?;
                if !metadata.is_file() || metadata.len() != 0 {
                    return Err(DbError::Corrupt("WAL seal marker must be an empty regular file".to_string()));
                }
                file
            }
            Err(error) => return Err(io_err(error)),
        };
        durable_file_sync(&file, lifecycle)?;
        lifecycle.hit(FsLifecyclePhase::MarkerFileSynced)?;
        drop(file);
        sync_directory(dir)?;
        lifecycle.hit(FsLifecyclePhase::MarkerParentSynced)
    }

    fn durable_wal_delete(dir: &Path, index: u64, lifecycle: &FsLifecycle) -> Result<(), DbError> {
        match std::fs::symlink_metadata(dir) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => return Err(DbError::Corrupt("WAL directory must be a directory, not a link".to_string())),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(io_err(error)),
        }
        for (path, removed, synced) in
            [(segment_path(dir, index), FsLifecyclePhase::SegmentDeleted, FsLifecyclePhase::SegmentDeleteParentSynced), (sealed_marker_path(dir, index), FsLifecyclePhase::MarkerDeleted, FsLifecyclePhase::MarkerDeleteParentSynced)]
        {
            match std::fs::remove_file(path) {
                Ok(()) => lifecycle.hit(removed)?,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(io_err(error)),
            }
            sync_directory(dir)?;
            lifecycle.hit(synced)?;
        }
        Ok(())
    }

    #[cfg(test)]
    include!("../🧪️tests/🔬️fs-storage-directory-durability/🦀️.rs");

    /// 🧭️ Maps a `std::io::Error` to `DbError::NotFound(missing())` when it's a missing-file
    /// error, or `DbError::Io` otherwise — used everywhere an open/read/stat is expected to find a
    /// caller-addressed blob that might legitimately not exist yet.
    // 🚫️async: E1 pure accessor called from sync `.map_err(|err| open_err(...))` closures — see R9
    fn open_err(err: std::io::Error, missing: impl FnOnce() -> String) -> DbError {
        if err.kind() == std::io::ErrorKind::NotFound {
            DbError::NotFound(missing())
        } else {
            io_err(err)
        }
    }

    /// 🛡️ Rejects a path component that could escape `root` (empty, `.`, `..`, or
    /// containing a path separator/NUL) — every document id, resource name, etc. that becomes a
    /// filesystem path component is validated through this before use.
    fn safe_component(raw: &str) -> Result<&str, DbError> {
        if raw.is_empty() || raw == "." || raw == ".." || raw.contains('/') || raw.contains('\\') || raw.contains('\0') {
            return Err(DbError::InvalidArgument(format!("unsafe storage path component: {raw:?}")));
        }
        Ok(raw)
    }

    fn wal_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("wal").join(safe_component(&document.0)?))
    }

    fn snapshot_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("snapshot").join(safe_component(&document.0)?))
    }

    fn index_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("index").join(safe_component(&document.0)?))
    }

    fn payload_path(root: &Path, hash: &ContentHash) -> PathBuf {
        let hex = hash.to_string();
        root.join("payload").join(&hex[0..2]).join(format!("{hex}.bin"))
    }

    fn catalog_path(root: &Path) -> PathBuf {
        root.join("catalog").join("root.bin")
    }

    fn lease_path(root: &Path, resource: &str) -> Result<PathBuf, DbError> {
        Ok(root.join("lease").join(format!("{}.bin", safe_component(resource)?)))
    }

    fn segment_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.bin"))
    }

    fn sealed_marker_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.sealed"))
    }

    fn generation_path(dir: &Path, generation: u64) -> PathBuf {
        dir.join(format!("gen-{generation:020}.pack"))
    }

    fn run_path(dir: &Path, run_id: u64) -> PathBuf {
        dir.join(format!("run-{run_id:020}.bin"))
    }

    fn read_lease_file(path: &Path) -> Result<Option<(EpochFence, u64, DbIoText)>, DbError> {
        if !path.exists() {
            return Ok(None);
        }
        let mut file = std::fs::File::open(path).map_err(io_err)?;
        let mut header = [0u8; 18];
        file.read_exact(&mut header).map_err(io_err)?;
        let mut epoch_bytes = [0u8; 8];
        epoch_bytes.copy_from_slice(&header[..8]);
        let mut expiry_bytes = [0u8; 8];
        expiry_bytes.copy_from_slice(&header[8..16]);
        let mut holder_len_bytes = [0u8; 2];
        holder_len_bytes.copy_from_slice(&header[16..18]);
        let epoch = u64::from_le_bytes(epoch_bytes);
        let expires_at_ms = u64::from_le_bytes(expiry_bytes);
        let holder_len = usize::from(u16::from_le_bytes(holder_len_bytes));
        if holder_len > super::DB_IO_TEXT_BYTES || file.metadata().map_err(io_err)?.len() != (18 + holder_len) as u64 {
            return Err(DbError::Corrupt("lease record exceeds its fixed authority".to_string()));
        }
        let mut holder = [0u8; super::DB_IO_TEXT_BYTES];
        file.read_exact(&mut holder[..holder_len]).map_err(io_err)?;
        let holder = std::str::from_utf8(&holder[..holder_len]).map_err(|_| DbError::Corrupt("lease holder is not valid UTF-8".to_string()))?;
        Ok(Some((EpochFence { epoch }, expires_at_ms, DbIoText::try_from_str(holder)?)))
    }

    fn write_lease_file(path: &Path, fence: EpochFence, expires_at_ms: u64, holder: &str) -> Result<(), DbError> {
        let holder = DbIoText::try_from_str(holder)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(io_err)?;
        }
        let mut record = [0u8; 18 + super::DB_IO_TEXT_BYTES];
        record[..8].copy_from_slice(&fence.epoch.to_le_bytes());
        record[8..16].copy_from_slice(&expires_at_ms.to_le_bytes());
        record[16..18].copy_from_slice(&(holder.as_str().len() as u16).to_le_bytes());
        record[18..18 + holder.as_str().len()].copy_from_slice(holder.as_str().as_bytes());
        pack::write_atomic(path, &record[..18 + holder.as_str().len()])?;
        Ok(())
    }

    struct FsDbIoExecutor {
        root: DbIoText,
        lifecycle: FsLifecycle,
        catalog_lock: Mutex<()>,
        lease_lock: Mutex<()>,
        writers: Mutex<Option<Box<WalWriterTable<WalFileWriterGuard>>>>,
        payload_hashes: [Mutex<Option<(u64, semio_framework_hash::Hasher)>>; super::DB_IO_LEDGER_ITEMS],
        readers: [Mutex<Option<FsReadState>>; super::DB_IO_LEDGER_ITEMS],
        backend_close_cursor: std::sync::atomic::AtomicUsize,
        backend_terminal: std::sync::atomic::AtomicBool,
    }

    struct FsReadState {
        operation: u64,
        file: std::fs::File,
        offset: u64,
        total: usize,
        fence: Option<EpochFence>,
    }

    impl FsDbIoExecutor {
        fn new(root: DbIoText) -> Self {
            Self {
                root,
                lifecycle: FsLifecycle::default(),
                catalog_lock: Mutex::new(()),
                lease_lock: Mutex::new(()),
                writers: Mutex::new(Some(Box::new(WalWriterTable::unbound()))),
                payload_hashes: [const { Mutex::new(None) }; super::DB_IO_LEDGER_ITEMS],
                readers: [const { Mutex::new(None) }; super::DB_IO_LEDGER_ITEMS],
                backend_close_cursor: std::sync::atomic::AtomicUsize::new(0),
                backend_terminal: std::sync::atomic::AtomicBool::new(false),
            }
        }

        fn root(&self) -> &Path {
            Path::new(self.root.as_str())
        }

        fn document_dir(&self, family: &str, document: &DbIoText) -> Result<PathBuf, DbError> {
            Ok(self.root().join(family).join(safe_component(document.as_str())?))
        }

        fn writer_sidecar(&self, document: &DbIoText) -> Result<PathBuf, DbError> {
            let canonical = std::fs::canonicalize(self.root()).map_err(io_err)?;
            let canonical = canonical.to_str().ok_or_else(|| DbError::InvalidArgument("canonical filesystem storage root is not UTF-8".to_string()))?;
            let directory = Path::new(canonical).join(".semio-wal-writer");
            let mut hash = semio_framework_hash::Sha256::new();
            hash.update(canonical.as_bytes());
            hash.update(&[0]);
            hash.update(document.as_str().as_bytes());
            Ok(directory.join(format!("{}.lock", semio_framework_hash::hex_lower(&hash.finalize()))))
        }

        fn read_step(&self, operation: u64, path: &Path, offset: u64, exact_len: Option<u64>, catalog: bool, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<(DbIoPages, Option<EpochFence>)>), DbError> {
            let slot = super::db_io_operation_owner_slot(operation)?;
            let mut owner = self.readers[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if owner.as_ref().is_some_and(|reader| reader.operation != operation) {
                return Err(DbError::Unavailable("filesystem read cursor capacity exhausted".to_string()));
            }
            if owner.is_none() {
                let mut file = std::fs::File::open(path).map_err(|error| open_err(error, || format!("database I/O object {} not found", path.display())))?;
                let available = file.metadata().map_err(io_err)?.len().checked_sub(offset).ok_or_else(|| DbError::InvalidArgument("DB I/O read offset exceeds object length".to_string()))?;
                let total = exact_len.unwrap_or(available);
                check_len(total, DB_IO_MAX_READ_BYTES, "db_io typed filesystem read")?;
                if total > available {
                    return Err(DbError::InvalidArgument("DB I/O read range exceeds object length".to_string()));
                }
                let fence = if catalog {
                    let mut header = [0u8; 8];
                    file.read_exact(&mut header).map_err(io_err)?;
                    Some(EpochFence { epoch: u64::from_le_bytes(header) })
                } else {
                    None
                };
                *owner = Some(FsReadState { operation, file, offset, total: total as usize, fence });
            }
            let Some(reader) = owner.as_mut() else {
                return Err(DbError::Internal("filesystem read cursor lost its retained owner".to_string()));
            };
            if output.len() == reader.total {
                let fence = reader.fence;
                return match output.seal_retained_step()? {
                    Some(pages) => {
                        owner.take();
                        Ok((DbIoExecutionStep::Complete, Some((pages, fence))))
                    }
                    None => Ok((DbIoExecutionStep::Yield, None)),
                };
            }
            let mut fragment = [0u8; super::DB_IO_PAGE_BYTES];
            let fragment_len = (reader.total - output.len()).min(fragment.len());
            reader.file.seek(SeekFrom::Start(reader.offset + output.len() as u64)).map_err(io_err)?;
            reader.file.read_exact(&mut fragment[..fragment_len]).map_err(io_err)?;
            if output.write_fragment(&fragment[..fragment_len])? != fragment_len {
                return Err(DbError::Internal("DB I/O output writer accepted a partial fixed fragment".to_string()));
            }
            Ok((DbIoExecutionStep::Yield, None))
        }

        fn read_pages_step(&self, operation: u64, path: &Path, offset: u64, exact_len: Option<u64>, output: &mut DbIoPageWriter) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            let (step, result) = self.read_step(operation, path, offset, exact_len, false, output)?;
            Ok((step, result.map(|(pages, _)| DbIoResult::Pages(pages))))
        }

        fn replace_step(&self, path: &Path, operation: u64, input: &mut DbIoPages, header: &[u8]) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            let parent = path.parent().ok_or_else(|| DbError::InvalidArgument("DB I/O replacement path has no parent".to_string()))?;
            durable_directory(parent, &self.lifecycle)?;
            let name = path.file_name().and_then(std::ffi::OsStr::to_str).ok_or_else(|| DbError::InvalidArgument("DB I/O replacement path is not UTF-8".to_string()))?;
            let temporary = parent.join(format!(".{name}.{operation:016x}.dbio"));
            if !temporary.exists() {
                let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary).map_err(io_err)?;
                if !header.is_empty() {
                    file.write_all(header).map_err(io_err)?;
                }
            }
            if let Some(fragment) = input.page(0) {
                let fragment_len = fragment.len();
                let mut file = std::fs::OpenOptions::new().append(true).open(&temporary).map_err(io_err)?;
                file.write_all(fragment).map_err(io_err)?;
                input.advance(fragment_len)?;
                if !input.is_empty() {
                    return Ok((DbIoExecutionStep::Yield, None));
                }
            }
            let file = std::fs::OpenOptions::new().write(true).open(&temporary).map_err(io_err)?;
            durable_file_sync(&file, &self.lifecycle)?;
            drop(file);
            std::fs::rename(temporary, path).map_err(io_err)?;
            self.lifecycle.hit(FsLifecyclePhase::ReplacementRenamed)?;
            sync_directory(parent)?;
            self.lifecycle.hit(FsLifecyclePhase::ReplacementParentSynced)?;
            Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
        }

        fn list_step(&self, dir: &Path, prefix: &str, suffix: &str, list: &mut DbIoU64List) -> Result<bool, DbError> {
            if !dir.exists() {
                return Ok(true);
            }
            let after = list.as_slice().last().copied();
            for entry in std::fs::read_dir(dir).map_err(io_err)? {
                let name = entry.map_err(io_err)?.file_name();
                let name = name.to_string_lossy();
                let Some(number) = name.strip_prefix(prefix).and_then(|rest| rest.strip_suffix(suffix)) else { continue };
                let Ok(value) = number.parse::<u64>() else { continue };
                if after.is_none_or(|after| value > after) {
                    list.push(value)?;
                }
            }
            list.sort_ascending();
            Ok(true)
        }

        fn payload_hash_step(&self, operation: u64, input: &mut DbIoPages) -> Result<Option<ContentHash>, DbError> {
            let slot = super::db_io_operation_owner_slot(operation)?;
            let mut state = self.payload_hashes[slot].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.as_ref().is_some_and(|(owner, _)| *owner != operation) {
                return Err(DbError::Unavailable("DB I/O filesystem hash cursor capacity exhausted".to_string()));
            }
            let (_, hasher) = state.get_or_insert_with(|| (operation, semio_framework_hash::Hasher::new()));
            if let Some(fragment) = input.page(0) {
                let len = fragment.len();
                hasher.update(fragment);
                input.advance(len)?;
            }
            if !input.is_empty() {
                return Ok(None);
            }
            let Some((_, hasher)) = state.take() else {
                return Err(DbError::Internal("DB I/O payload hash cursor lost its retained owner".to_string()));
            };
            Ok(Some(ContentHash(*hasher.finalize().as_bytes())))
        }
    }

    impl DbIoTaskExecutor for FsDbIoExecutor {
        fn supports_writer_authority(&self) -> bool {
            true
        }

        fn bind_writer_control(&mut self, control: DbIoBackendControl) -> Result<(), DbError> {
            let mut owner = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            owner.as_deref_mut().ok_or(DbError::Closed)?.bind(control)
        }

        fn writer_release_step(&self, _context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
            self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().map_or(Ok(DbIoWriterReleaseStep::Idle), WalWriterTable::release_requested_step)
        }

        fn pin_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
            let Some((key, backend, document)) = task.writer_stamp() else { return Ok(()) };
            self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut().ok_or(DbError::Closed)?.pin_operation(key, backend, document, operation).map(|_| ())
        }

        fn finish_writer_operation(&self, operation: u64, task: &DbIoTask) -> Result<(), DbError> {
            let Some((key, backend, document)) = task.writer_stamp() else { return Ok(()) };
            if let Some(table) = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_deref_mut() {
                table.finish_operation_if_pinned(key, backend, document, operation)?;
            }
            Ok(())
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }

        fn owner_backing_bytes(&self) -> u64 {
            (size_of::<Self>() + size_of::<WalWriterTable<WalFileWriterGuard>>()) as u64
        }

        fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
            Box::pin(async move {
                let executor: Box<dyn DbIoTaskExecutor> = self;
                (executor, task, Err(DbError::Internal("filesystem backend has no async-native driver".to_string())))
            })
        }

        fn execute_step(&self, operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
            match task {
                DbIoTask::BackendOpen { path, .. } => {
                    if path.as_str() != self.root.as_str() {
                        return Err(DbError::InvalidArgument("DB I/O filesystem root authority mismatch".to_string()));
                    }
                    durable_directory(self.root(), &self.lifecycle)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalWriterAcquire { document, .. } => {
                    let mut writers = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let table = writers.as_deref_mut().ok_or(DbError::Closed)?;
                    let permit = table.acquire_with(document, || {
                        let path = self.writer_sidecar(document)?;
                        std::fs::create_dir_all(path.parent().expect("filesystem writer sidecar has a parent")).map_err(io_err)?;
                        WalFileWriterGuard::try_acquire(&path)
                    })?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::WalWriter(permit))))
                }
                DbIoTask::WalCreate { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    durable_wal_create(&dir, *index, &self.lifecycle)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalAppend { document, index, input, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    if sealed_marker_path(&dir, *index).exists() {
                        return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
                    }
                    let path = segment_path(&dir, *index);
                    if let Some(fragment) = input.page(0) {
                        let len = fragment.len();
                        let mut file = std::fs::OpenOptions::new().append(true).open(&path).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?;
                        file.write_all(fragment).map_err(io_err)?;
                        input.advance(len)?;
                        if !input.is_empty() {
                            return Ok((DbIoExecutionStep::Yield, None));
                        }
                    }
                    let len = std::fs::metadata(path).map_err(io_err)?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::WalSync { document, index, class, .. } => {
                    if matches!(class, DurabilityClass::Fsync | DurabilityClass::Quorum(_)) {
                        let dir = self.document_dir("wal", document)?;
                        let path = segment_path(&dir, *index);
                        durable_file_sync(&std::fs::OpenOptions::new().write(true).open(path).map_err(io_err)?, &self.lifecycle)?;
                        self.lifecycle.hit(FsLifecyclePhase::WalFileSynced)?;
                        durable_directory(&dir, &self.lifecycle)?;
                        sync_directory(&dir)?;
                        self.lifecycle.hit(FsLifecyclePhase::WalParentSynced)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalSeal { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    durable_wal_seal(&dir, *index, &self.lifecycle)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalRead { document, index, range, output, .. } => {
                    let path = segment_path(&self.document_dir("wal", document)?, *index);
                    self.read_pages_step(operation, &path, range.offset, Some(range.len), output)
                }
                DbIoTask::WalLength { document, index, .. } => {
                    let path = segment_path(&self.document_dir("wal", document)?, *index);
                    let len = std::fs::metadata(path).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::WalState { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    std::fs::metadata(segment_path(&dir, *index)).map_err(|error| open_err(error, || format!("wal segment {index} not found")))?;
                    let state = match std::fs::metadata(sealed_marker_path(&dir, *index)) {
                        Ok(_) => WalSegmentState::Sealed,
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound => WalSegmentState::Active,
                        Err(error) => return Err(io_err(error)),
                    };
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::WalSegmentState(state))))
                }
                DbIoTask::WalList { document, output, .. } => {
                    if self.list_step(&self.document_dir("wal", document)?, "segment-", ".bin", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::WalTruncate { document, index, new_len, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    if sealed_marker_path(&dir, *index).exists() {
                        return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
                    }
                    let path = segment_path(&dir, *index);
                    let file = std::fs::OpenOptions::new().write(true).open(path).map_err(io_err)?;
                    if *new_len > file.metadata().map_err(io_err)?.len() {
                        return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
                    }
                    file.set_len(*new_len).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::WalDelete { document, index, .. } => {
                    let dir = self.document_dir("wal", document)?;
                    durable_wal_delete(&dir, *index, &self.lifecycle)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::SnapshotWrite { document, generation, input, .. } => self.replace_step(&generation_path(&self.document_dir("snapshot", document)?, *generation), operation, input, &[]),
                DbIoTask::SnapshotRead { document, generation, output, .. } => self.read_pages_step(operation, &generation_path(&self.document_dir("snapshot", document)?, *generation), 0, None, output),
                DbIoTask::SnapshotLatest { document, output, .. } => {
                    if self.list_step(&self.document_dir("snapshot", document)?, "gen-", ".pack", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLength(output.as_slice().last().copied()))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::SnapshotList { document, output, .. } => {
                    if self.list_step(&self.document_dir("snapshot", document)?, "gen-", ".pack", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::SnapshotDelete { document, generation, .. } => {
                    let path = generation_path(&self.document_dir("snapshot", document)?, *generation);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::PayloadPut { input, .. } => {
                    let temporary = self.root().join("payload").join(format!(".{operation:016x}.dbio"));
                    if !temporary.exists() {
                        if let Some(parent) = temporary.parent() {
                            std::fs::create_dir_all(parent).map_err(io_err)?;
                        }
                        std::fs::File::create(&temporary).map_err(io_err)?;
                    }
                    if let Some(fragment) = input.page(0) {
                        let mut file = std::fs::OpenOptions::new().create(true).append(true).open(&temporary).map_err(io_err)?;
                        file.write_all(fragment).map_err(io_err)?;
                    }
                    let Some(hash) = self.payload_hash_step(operation, input)? else { return Ok((DbIoExecutionStep::Yield, None)) };
                    let path = payload_path(self.root(), &hash);
                    if temporary.exists() {
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent).map_err(io_err)?;
                        }
                        if !path.exists() {
                            std::fs::rename(temporary, path).map_err(io_err)?;
                        } else {
                            std::fs::remove_file(temporary).map_err(io_err)?;
                        }
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Hash(hash))))
                }
                DbIoTask::PayloadGet { hash, output, .. } => self.read_pages_step(operation, &payload_path(self.root(), hash), 0, None, output),
                DbIoTask::PayloadExists { hash, .. } => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Exists(payload_path(self.root(), hash).exists())))),
                DbIoTask::PayloadLength { hash, .. } => {
                    let len = std::fs::metadata(payload_path(self.root(), hash)).map_err(|error| open_err(error, || format!("payload {hash} not found")))?.len();
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Length(len))))
                }
                DbIoTask::PayloadDelete { hash, .. } => {
                    let path = payload_path(self.root(), hash);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::CatalogRead { output, .. } => {
                    let path = catalog_path(self.root());
                    if !path.exists() {
                        return Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalCatalog(None))));
                    }
                    let (step, result) = self.read_step(operation, &path, 8, None, true, output)?;
                    let result = match result {
                        Some((pages, Some(fence))) => Some(DbIoResult::OptionalCatalog(Some((pages, fence)))),
                        Some((pages, None)) => {
                            drop(pages);
                            return Err(DbError::Internal("catalog read cursor lost its retained fence".to_string()));
                        }
                        None => None,
                    };
                    Ok((step, result))
                }
                DbIoTask::CatalogCas { expected, input, .. } => {
                    let _guard = self.catalog_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let current = read_root_fence(self.root())?.unwrap_or(EpochFence::INITIAL);
                    expected.check(current)?;
                    let next = expected.next();
                    let (step, result) = self.replace_step(&catalog_path(self.root()), operation, input, &next.epoch.to_le_bytes())?;
                    Ok((step, result.map(|_| DbIoResult::Fence(next))))
                }
                DbIoTask::IndexWrite { document, run_id, input, .. } => self.replace_step(&run_path(&self.document_dir("index", document)?, *run_id), operation, input, &[]),
                DbIoTask::IndexRead { document, run_id, output, .. } => self.read_pages_step(operation, &run_path(&self.document_dir("index", document)?, *run_id), 0, None, output),
                DbIoTask::IndexList { document, output, .. } => {
                    if self.list_step(&self.document_dir("index", document)?, "run-", ".bin", output)? {
                        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::List(std::mem::take(output)))))
                    } else {
                        Ok((DbIoExecutionStep::Yield, None))
                    }
                }
                DbIoTask::IndexDelete { document, run_id, .. } => {
                    let path = run_path(&self.document_dir("index", document)?, *run_id);
                    if path.exists() {
                        std::fs::remove_file(path).map_err(io_err)?;
                    }
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseAcquire { document, holder, now_ms, ttl_ms, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let fence = match read_lease_file(&path)? {
                        Some((_, expires, existing)) if *now_ms < expires && existing.as_str() != holder.as_str() => return Err(DbError::Conflict("resource is leased by another holder".to_string())),
                        Some((fence, expires, _)) if *now_ms < expires => fence,
                        Some((fence, _, _)) => fence.next(),
                        None => EpochFence::INITIAL,
                    };
                    let expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("filesystem lease expiry"))?;
                    write_lease_file(&path, fence, expires_at_ms, holder.as_str())?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Fence(fence))))
                }
                DbIoTask::LeaseRenew { document, holder, fence, now_ms, ttl_ms, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let (current, expires, existing) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                    if *now_ms >= expires || existing.as_str() != holder.as_str() {
                        return Err(DbError::Unauthorized("lease owner or expiry mismatch".to_string()));
                    }
                    fence.check(current)?;
                    let expires_at_ms = now_ms.checked_add(*ttl_ms).ok_or(DbError::LimitExceeded("filesystem lease expiry"))?;
                    write_lease_file(&path, *fence, expires_at_ms, holder.as_str())?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                    let _guard = self.lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let path = lease_path(self.root(), document.as_str())?;
                    let (current, _, existing) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound("lease not found".to_string()))?;
                    if existing.as_str() != holder.as_str() {
                        return Err(DbError::Unauthorized("lease owner mismatch".to_string()));
                    }
                    fence.check(current)?;
                    std::fs::remove_file(path).map_err(io_err)?;
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
                }
                DbIoTask::LeaseGet { document, now_ms, .. } => {
                    let lease = read_lease_file(&lease_path(self.root(), document.as_str())?)?;
                    let result = match lease {
                        Some((fence, expires_at_ms, holder)) if *now_ms < expires_at_ms => Some(super::DbIoLeaseResult::new(document.clone(), holder, fence, expires_at_ms)),
                        _ => None,
                    };
                    Ok((DbIoExecutionStep::Complete, Some(DbIoResult::OptionalLease(result))))
                }
                DbIoTask::BackendClose { .. } => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit))),
            }
        }

        fn close_operation_step(&self, operation: u64, task: &DbIoTask) -> Result<bool, DbError> {
            let owner_slot = super::db_io_operation_owner_slot(operation).ok();
            for reader in owner_slot.map_or(&self.readers[..], |slot| std::slice::from_ref(&self.readers[slot])) {
                let mut reader = reader.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if reader.as_ref().is_some_and(|owner| owner.operation == operation) {
                    reader.take();
                    return Ok(false);
                }
            }
            let temporary = match task {
                DbIoTask::SnapshotWrite { document, generation, .. } => {
                    let path = generation_path(&self.document_dir("snapshot", document)?, *generation);
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::IndexWrite { document, run_id, .. } => {
                    let path = run_path(&self.document_dir("index", document)?, *run_id);
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::CatalogCas { .. } => {
                    let path = catalog_path(self.root());
                    path.parent().zip(path.file_name().and_then(std::ffi::OsStr::to_str)).map(|(parent, name)| parent.join(format!(".{name}.{operation:016x}.dbio")))
                }
                DbIoTask::PayloadPut { .. } => Some(self.root().join("payload").join(format!(".{operation:016x}.dbio"))),
                _ => None,
            };
            if let Some(temporary) = temporary {
                if temporary.exists() {
                    std::fs::remove_file(temporary).map_err(io_err)?;
                    return Ok(false);
                }
            }
            for state in owner_slot.map_or(&self.payload_hashes[..], |slot| std::slice::from_ref(&self.payload_hashes[slot])) {
                let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if state.as_ref().is_some_and(|(owner, _)| *owner == operation) {
                    state.take();
                    return Ok(false);
                }
            }
            Ok(true)
        }

        fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
            {
                let mut owner = self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if let Some(table) = owner.as_deref_mut() {
                    if table.close_step()? {
                        return Ok(false);
                    }
                    if !table.terminal_is_empty() {
                        return Err(DbError::Internal("filesystem WAL writer table returned a false terminal witness".to_string()));
                    }
                    owner.take();
                    return Ok(false);
                }
            }
            let cursor = self.backend_close_cursor.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            if cursor < self.readers.len() {
                self.readers[cursor].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                return Ok(false);
            }
            let hash = cursor - self.readers.len();
            if hash < self.payload_hashes.len() {
                self.payload_hashes[hash].lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                return Ok(false);
            }
            self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
            Ok(true)
        }

        fn backend_terminal_is_empty(&self) -> bool {
            self.backend_terminal.load(std::sync::atomic::Ordering::Acquire)
                && self.writers.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
                && self.readers.iter().all(|owner| owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
                && self.payload_hashes.iter().all(|owner| owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none())
        }
    }

    async fn execute(task: DbIoTask) -> Result<DbIoResult, DbError> {
        submit_db_io_task_admitted(task).await?.finish().await
    }

    fn document_text(document: &ArtifactId) -> Result<DbIoText, DbError> {
        DbIoText::try_from_str(&document.0)
    }

    fn output_writer(bytes: u64) -> Result<DbIoPageWriter, DbError> {
        let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("db_io output writer bytes"))?.div_ceil(super::DB_IO_PAGE_BYTES);
        DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
    }

    fn typed_result_fault(expected: &'static str) -> DbError {
        DbError::Internal(format!("DB I/O filesystem executor did not return {expected}"))
    }

    fn unit(result: DbIoResult) -> Result<(), DbError> {
        match result {
            DbIoResult::Unit => Ok(()),
            _ => Err(typed_result_fault("unit")),
        }
    }

    fn length(result: DbIoResult) -> Result<u64, DbError> {
        match result {
            DbIoResult::Length(value) => Ok(value),
            _ => Err(typed_result_fault("length")),
        }
    }

    fn optional_length(result: DbIoResult) -> Result<Option<u64>, DbError> {
        match result {
            DbIoResult::OptionalLength(value) => Ok(value),
            _ => Err(typed_result_fault("optional length")),
        }
    }

    fn pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
        match result {
            DbIoResult::Pages(value) => Ok(value),
            _ => Err(typed_result_fault("pages")),
        }
    }

    fn list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
        match result {
            DbIoResult::List(value) => Ok(value),
            _ => Err(typed_result_fault("list")),
        }
    }

    fn wal_writer(result: DbIoResult) -> Result<WalWriterPermit, DbError> {
        match result {
            DbIoResult::WalWriter(writer) => Ok(writer),
            _ => Err(typed_result_fault("WAL writer")),
        }
    }

    /// 📁️ The zero-touch default `DbStorage` backend — see module doc for the on-disk
    /// layout. `catalog_lock`/`lease_lock` serialize this-process's compare-and-swap operations;
    /// see `CatalogStorage`/`LeaseStorage` impls below for why a bare read-verify-write over
    /// `write_atomic` isn't itself enough across OS processes (documented extension seam). `pool`
    /// is what every trait method dispatches its blocking body through (see module doc); its
    /// `Arc` is cloned into each method's `'static` blocking closure.
    pub struct FsStorage {
        control: DbIoBackendControl,
        pool: Arc<WorkerPool>,
        closed: std::sync::atomic::AtomicBool,
    }

    impl FsStorage {
        /// 🚀️ Opens (creating if absent) a `FsStorage` rooted at `root`, dispatching every
        /// subsequent trait call's blocking body through the typed task owner onto `pool`'s
        /// `Lane::Io`. The constructor's directory creation uses that same retained authority;
        /// callers never prepare the root synchronously or through a pool-less fallback.
        pub async fn open(pool: Arc<WorkerPool>, root: &Path) -> Result<Self, DbStorageOpenRejected> {
            super::open_db_io_backend_admitted(&pool, || Self::open_once(pool.clone(), root)).await
        }

        /// 📊️ This backend's writer admission line so far — how many acquisitions had to wait for a writer, how many
        /// waits were refused, and how many wait now.
        pub(crate) fn writer_admission_census(&self) -> db_io_core::WalWriterAdmissionCensus {
            db_io_driver::admission_census(self.control)
        }

        /// 🎯️ One filesystem backend open attempt, refused at once when the backend capacity is taken.
        async fn open_once(pool: Arc<WorkerPool>, root: &Path) -> Result<Self, DbStorageOpenRejected> {
            let root = root.to_str().ok_or_else(|| DbError::InvalidArgument("filesystem storage root is not UTF-8".to_string())).and_then(DbIoText::try_from_str)?;
            let rollback = DbIoBackendRollbackReservation::try_reserve()?;
            let pool_use = pool.acquire_use().map_err(|error| DbError::Unavailable(format!("filesystem DB I/O backend WorkerPool use rejected: {error:?}")))?;
            let control = register_db_io_backend_prepared_with_use(DbIoBackendKind::Filesystem, Box::new(FsDbIoExecutor::new(root.clone())), pool.clone(), pool_use, rollback)?;
            let storage = Self { control, pool, closed: std::sync::atomic::AtomicBool::new(false) };
            let task = DbIoTask::BackendOpen { backend: control, path: root };
            if let Err(error) = execute(task).await {
                return Err(DbStorageOpenRejected::registered(error, control));
            }
            Ok(storage)
        }

        pub async fn close(&self) -> Result<(), DbError> {
            let result = match execute(DbIoTask::BackendClose { backend: self.control }).await? {
                DbIoResult::Unit => Ok(()),
                _ => Err(DbError::Internal("filesystem backend close returned the wrong typed result".to_string())),
            };
            if result.is_ok() {
                close_db_io_backend(self.control).await?;
                self.closed.store(true, std::sync::atomic::Ordering::Release);
            }
            result
        }

        /// 🎚️ Always durable, `fsync`-capable, CAS-capable — the on-disk default.
        pub async fn capabilities(&self) -> StorageCapabilities {
            StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
        }
    }

    impl Drop for FsStorage {
        fn drop(&mut self) {
            if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
                let _ = retire_db_io_backend(self.control);
            }
        }
    }

    impl WalStorage for FsStorage {
        async fn acquire_writer(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError> {
            let document = document_text(document)?;
            db_io_driver::admitted_acquire(self.control, std::time::Duration::from_millis(db_io_driver::WAL_WRITER_ADMISSION_WAIT_MS), || async {
                wal_writer(execute(DbIoTask::WalWriterAcquire { backend: self.control, document: document.clone() }).await?)
            })
            .await
        }

        async fn create_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
            unit(execute(DbIoTask::WalCreate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
        }

        async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
            check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "wal_storage::append")?;
            length(execute(DbIoTask::WalAppend { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, input: bytes }).await?)
        }

        async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
            unit(execute(DbIoTask::WalSync { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, class }).await?)
        }

        async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
            unit(execute(DbIoTask::WalSeal { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
        }

        async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
            if let Err(err) = check_len(range.len, DB_IO_MAX_READ_BYTES, "wal_storage::read") {
                return Err(err);
            }
            let output = output_writer(range.len)?;
            pages(execute(DbIoTask::WalRead { backend: self.control, document: document_text(document)?, index, range, output }).await?)
        }

        async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
            length(execute(DbIoTask::WalLength { backend: self.control, document: document_text(document)?, index }).await?)
        }

        async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
            match execute(DbIoTask::WalState { backend: self.control, document: document_text(document)?, index }).await? {
                DbIoResult::WalSegmentState(state) => Ok(state),
                _ => Err(DbError::Internal("filesystem WAL state result taxonomy".to_string())),
            }
        }

        async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(DbIoTask::WalList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn truncate_tail(&self, writer: &WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
            unit(execute(DbIoTask::WalTruncate { backend: self.control, document: writer.document().clone(), writer: writer.key(), index, new_len }).await?)
        }

        async fn delete_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
            unit(execute(DbIoTask::WalDelete { backend: self.control, document: writer.document().clone(), writer: writer.key(), index }).await?)
        }
    }

    impl SnapshotStorage for FsStorage {
        fn publication_scope(&self) -> usize {
            std::ptr::from_ref(self).addr()
        }

        async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
            check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "snapshot_storage::write_generation")?;
            unit(execute(DbIoTask::SnapshotWrite { backend: self.control, document: document_text(document)?, generation, input: bytes }).await?)
        }

        async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
            pages(execute(DbIoTask::SnapshotRead { backend: self.control, document: document_text(document)?, generation, output: output_writer(DB_IO_MAX_READ_BYTES)? }).await?)
        }

        async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
            optional_length(execute(DbIoTask::SnapshotLatest { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(DbIoTask::SnapshotList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
            unit(execute(DbIoTask::SnapshotDelete { backend: self.control, document: document_text(document)?, generation }).await?)
        }
    }

    impl PayloadStorage for FsStorage {
        async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
            if let Err(err) = check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "payload_storage::put") {
                return Err(err);
            }
            match execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
                DbIoResult::Hash(hash) => Ok(hash),
                _ => Err(typed_result_fault("content hash")),
            }
        }

        async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
            pages(execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: output_writer(DB_IO_MAX_READ_BYTES)? }).await?)
        }

        async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
            match execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
                DbIoResult::Exists(exists) => Ok(exists),
                _ => Err(typed_result_fault("existence")),
            }
        }

        async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
            unit(execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
        }

        async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
            length(execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await?)
        }
    }

    impl CatalogStorage for FsStorage {
        async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
            match execute(DbIoTask::CatalogRead { backend: self.control, output: output_writer(DB_IO_MAX_READ_BYTES)? }).await? {
                DbIoResult::OptionalCatalog(root) => Ok(root),
                _ => Err(typed_result_fault("optional catalog root")),
            }
        }

        async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
            if let Err(err) = check_len(new_bytes.len() as u64, DB_IO_MAX_READ_BYTES, "catalog_storage::cas_root") {
                return Err(err);
            }
            match execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
                DbIoResult::Fence(fence) => Ok(fence),
                _ => Err(typed_result_fault("catalog fence")),
            }
        }
    }

    /// 📖️ The blocking body behind `CatalogStorage::read_root` — factored out so
    /// `cas_root` can reuse it under `catalog_lock` without recursive task submission.
    // 🚫️async: E1 pure-shaped typed-task accessor.
    fn read_root_fence(root: &Path) -> Result<Option<EpochFence>, DbError> {
        let path = catalog_path(root);
        if !path.exists() {
            return Ok(None);
        }
        let metadata = std::fs::metadata(&path).map_err(io_err)?;
        if metadata.len() < 8 {
            return Err(DbError::Corrupt("catalog root file is shorter than its 8-byte epoch header".to_string()));
        }
        let mut epoch_bytes = [0u8; 8];
        std::fs::File::open(&path).map_err(io_err)?.read_exact(&mut epoch_bytes).map_err(io_err)?;
        Ok(Some(EpochFence { epoch: u64::from_le_bytes(epoch_bytes) }))
    }

    impl IndexStorage for FsStorage {
        async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
            check_len(bytes.len() as u64, DB_IO_MAX_READ_BYTES, "index_storage::write_run")?;
            unit(execute(DbIoTask::IndexWrite { backend: self.control, document: document_text(document)?, run_id, input: bytes }).await?)
        }

        async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
            pages(execute(DbIoTask::IndexRead { backend: self.control, document: document_text(document)?, run_id, output: output_writer(DB_IO_MAX_READ_BYTES)? }).await?)
        }

        async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
            list(execute(DbIoTask::IndexList { backend: self.control, document: document_text(document)?, output: DbIoU64List::new() }).await?)
        }

        async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
            unit(execute(DbIoTask::IndexDelete { backend: self.control, document: document_text(document)?, run_id }).await?)
        }
    }

    impl LeaseStorage for FsStorage {
        async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
            match execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
                DbIoResult::Fence(fence) => Ok(fence),
                _ => Err(typed_result_fault("lease fence")),
            }
        }

        async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
            unit(execute(DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
        }

        async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
            unit(execute(DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
        }

        async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
            match execute(DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
                DbIoResult::OptionalLease(lease) => Ok(lease),
                _ => Err(typed_result_fault("optional lease")),
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use fs_storage::FsStorage;
//#endregion 🔖️Fs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
