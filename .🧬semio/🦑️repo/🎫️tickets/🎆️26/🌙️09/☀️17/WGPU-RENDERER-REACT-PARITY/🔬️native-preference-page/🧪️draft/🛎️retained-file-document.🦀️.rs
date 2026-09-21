use semio_framework_job::{FixedOperationKey, InteractiveJobCloseStep, JobPayloadCloseStep, RetainedJobPayload, StepContext};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

pub const STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES: usize = 64 * 1024;
pub const STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS: usize = 4;

static FIXED_FILE_DOCUMENT_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(1);
static FIXED_FILE_DOCUMENT_WRITERS: LazyLock<Mutex<BTreeMap<PathBuf, FixedOperationKey>>> = LazyLock::new(|| Mutex::new(BTreeMap::new()));

#[derive(Debug, PartialEq, Eq)]
pub enum FixedFileDocumentWriteStep {
    Yield,
    Published,
    Cancelled,
    Superseded,
    Fault(String),
}

enum FixedFileDocumentWriteTerminal {
    Cancelled,
    Superseded,
    Fault(String),
}

enum FixedFileDocumentWriteState {
    Pending { destination: PathBuf, payload: RetainedJobPayload, maximum_bytes: usize },
    Writing {
        owner: FixedOperationKey,
        destination: PathBuf,
        temporary: PathBuf,
        file: std::fs::File,
        payload: RetainedJobPayload,
        expected_bytes: usize,
        written_bytes: usize,
    },
    Ready { owner: FixedOperationKey, destination: PathBuf, temporary: PathBuf, file: std::fs::File, expected_bytes: usize },
    Closing {
        owner: Option<FixedOperationKey>,
        destination: Option<PathBuf>,
        temporary: Option<PathBuf>,
        file: Option<std::fs::File>,
        payload: Option<RetainedJobPayload>,
    },
    Terminal,
    Published,
}

pub struct FixedFileDocumentWriteCursor {
    state: FixedFileDocumentWriteState,
    terminal: Option<FixedFileDocumentWriteTerminal>,
}

impl FixedFileDocumentWriteCursor {
    pub fn new(destination: PathBuf, payload: RetainedJobPayload, maximum_bytes: usize) -> Self {
        Self { state: FixedFileDocumentWriteState::Pending { destination, payload, maximum_bytes }, terminal: None }
    }

    pub fn step(&mut self, cx: &mut StepContext<'_>) -> FixedFileDocumentWriteStep {
        if cx.is_cancelled() && self.terminal.is_none() {
            self.terminal = Some(FixedFileDocumentWriteTerminal::Cancelled);
            self.begin_close();
        }
        if matches!(self.state, FixedFileDocumentWriteState::Closing { .. }) {
            return match self.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                InteractiveJobCloseStep::Complete => self.take_terminal(),
                _ => FixedFileDocumentWriteStep::Yield,
            };
        }
        if cx.should_yield() {
            return FixedFileDocumentWriteStep::Yield;
        }
        cx.set_stage("FixedFileDocumentWrite");
        let state = std::mem::replace(&mut self.state, FixedFileDocumentWriteState::Terminal);
        match state {
            FixedFileDocumentWriteState::Pending { destination, payload, maximum_bytes } => self.open(cx, destination, payload, maximum_bytes),
            FixedFileDocumentWriteState::Writing { owner, destination, temporary, mut file, mut payload, expected_bytes, written_bytes } => {
                if !fixed_file_document_owner_is_current(&destination, owner) {
                    self.state = FixedFileDocumentWriteState::Writing { owner, destination, temporary, file, payload, expected_bytes, written_bytes };
                    self.terminal = Some(FixedFileDocumentWriteTerminal::Superseded);
                    self.begin_close();
                    return FixedFileDocumentWriteStep::Yield;
                }
                let Some(page) = (0..STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS).find_map(|index| payload.page(index)) else {
                    self.state = FixedFileDocumentWriteState::Ready { owner, destination, temporary, file, expected_bytes };
                    return FixedFileDocumentWriteStep::Yield;
                };
                let page_bytes = page.len();
                if page_bytes > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES || written_bytes.saturating_add(page_bytes) > expected_bytes {
                    self.state = FixedFileDocumentWriteState::Writing { owner, destination, temporary, file, payload, expected_bytes, written_bytes };
                    return self.fail("fixed file document page exceeds its admitted length".into());
                }
                if let Err(error) = file.write_all(page) {
                    self.state = FixedFileDocumentWriteState::Writing { owner, destination, temporary, file, payload, expected_bytes, written_bytes };
                    return self.fail(error.to_string());
                }
                match payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                    JobPayloadCloseStep::Pending { released_items: 1, .. } => {}
                    _ => {
                        self.state = FixedFileDocumentWriteState::Writing { owner, destination, temporary, file, payload, expected_bytes, written_bytes };
                        return self.fail("fixed file document retained page did not retire exactly".into());
                    }
                }
                cx.consume_fuel(1);
                self.state = FixedFileDocumentWriteState::Writing { owner, destination, temporary, file, payload, expected_bytes, written_bytes: written_bytes + page_bytes };
                FixedFileDocumentWriteStep::Yield
            }
            FixedFileDocumentWriteState::Ready { owner, destination, temporary, mut file, expected_bytes } => {
                if !fixed_file_document_owner_is_current(&destination, owner) {
                    self.state = FixedFileDocumentWriteState::Ready { owner, destination, temporary, file, expected_bytes };
                    self.terminal = Some(FixedFileDocumentWriteTerminal::Superseded);
                    self.begin_close();
                    return FixedFileDocumentWriteStep::Yield;
                }
                let publish = file
                    .flush()
                    .and_then(|()| file.sync_all())
                    .and_then(|()| file.metadata())
                    .and_then(|metadata| {
                        if metadata.len() == expected_bytes as u64 {
                            Ok(())
                        } else {
                            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "inactive fixed file document length changed before publication"))
                        }
                    });
                if let Err(error) = publish {
                    self.state = FixedFileDocumentWriteState::Ready { owner, destination, temporary, file, expected_bytes };
                    return self.fail(error.to_string());
                }
                drop(file);
                if !fixed_file_document_owner_is_current(&destination, owner) {
                    self.state = FixedFileDocumentWriteState::Closing { owner: Some(owner), destination: Some(destination), temporary: Some(temporary), file: None, payload: None };
                    self.terminal = Some(FixedFileDocumentWriteTerminal::Superseded);
                    return FixedFileDocumentWriteStep::Yield;
                }
                if let Err(error) = std::fs::rename(&temporary, &destination) {
                    self.state = FixedFileDocumentWriteState::Closing { owner: Some(owner), destination: Some(destination), temporary: Some(temporary), file: None, payload: None };
                    self.terminal = Some(FixedFileDocumentWriteTerminal::Fault(error.to_string()));
                    return FixedFileDocumentWriteStep::Yield;
                }
                fixed_file_document_release_owner(&destination, owner);
                self.state = FixedFileDocumentWriteState::Published;
                FixedFileDocumentWriteStep::Published
            }
            FixedFileDocumentWriteState::Terminal => self.take_terminal(),
            FixedFileDocumentWriteState::Published => {
                self.state = FixedFileDocumentWriteState::Published;
                FixedFileDocumentWriteStep::Fault("fixed file document write polled after publication".into())
            }
            closing @ FixedFileDocumentWriteState::Closing { .. } => {
                self.state = closing;
                FixedFileDocumentWriteStep::Yield
            }
        }
    }

    fn open(&mut self, cx: &StepContext<'_>, destination: PathBuf, payload: RetainedJobPayload, maximum_bytes: usize) -> FixedFileDocumentWriteStep {
        if maximum_bytes > STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES || payload.len() > maximum_bytes || payload.page_count() > STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS {
            self.state = FixedFileDocumentWriteState::Pending { destination, payload, maximum_bytes };
            return self.fail("fixed file document exceeds retained service authority".into());
        }
        let Some(parent) = destination.parent() else {
            self.state = FixedFileDocumentWriteState::Pending { destination, payload, maximum_bytes };
            return self.fail("fixed file document has no parent".into());
        };
        if let Err(error) = std::fs::create_dir_all(parent) {
            self.state = FixedFileDocumentWriteState::Pending { destination, payload, maximum_bytes };
            return self.fail(error.to_string());
        }
        let owner = FixedOperationKey::new(cx.operation(), cx.generation());
        let sequence = FIXED_FILE_DOCUMENT_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let Some(name) = destination.file_name() else {
            self.state = FixedFileDocumentWriteState::Pending { destination, payload, maximum_bytes };
            return self.fail("fixed file document has no file name".into());
        };
        let temporary = destination.with_file_name(format!("{}.pending-{}-{}-{sequence}", name.to_string_lossy(), owner.operation.0, owner.generation.0));
        let file = match std::fs::OpenOptions::new().create_new(true).write(true).open(&temporary) {
            Ok(file) => file,
            Err(error) => {
                self.state = FixedFileDocumentWriteState::Pending { destination, payload, maximum_bytes };
                return self.fail(error.to_string());
            }
        };
        FIXED_FILE_DOCUMENT_WRITERS.lock().expect("fixed file document owner mutex poisoned").insert(destination.clone(), owner);
        let expected_bytes = payload.len();
        self.state = FixedFileDocumentWriteState::Writing { owner, destination, temporary, file, payload, expected_bytes, written_bytes: 0 };
        FixedFileDocumentWriteStep::Yield
    }

    fn fail(&mut self, error: String) -> FixedFileDocumentWriteStep {
        self.terminal = Some(FixedFileDocumentWriteTerminal::Fault(error));
        self.begin_close();
        FixedFileDocumentWriteStep::Yield
    }

    fn take_terminal(&mut self) -> FixedFileDocumentWriteStep {
        self.state = FixedFileDocumentWriteState::Terminal;
        match self.terminal.take() {
            Some(FixedFileDocumentWriteTerminal::Cancelled) => FixedFileDocumentWriteStep::Cancelled,
            Some(FixedFileDocumentWriteTerminal::Superseded) => FixedFileDocumentWriteStep::Superseded,
            Some(FixedFileDocumentWriteTerminal::Fault(error)) => FixedFileDocumentWriteStep::Fault(error),
            None => FixedFileDocumentWriteStep::Fault("fixed file document terminal has no outcome".into()),
        }
    }

    pub fn begin_close(&mut self) {
        if matches!(self.state, FixedFileDocumentWriteState::Closing { .. } | FixedFileDocumentWriteState::Terminal | FixedFileDocumentWriteState::Published) {
            return;
        }
        let state = std::mem::replace(&mut self.state, FixedFileDocumentWriteState::Terminal);
        self.state = match state {
            FixedFileDocumentWriteState::Pending { destination, payload, .. } => FixedFileDocumentWriteState::Closing { owner: None, destination: Some(destination), temporary: None, file: None, payload: Some(payload) },
            FixedFileDocumentWriteState::Writing { owner, destination, temporary, file, payload, .. } => FixedFileDocumentWriteState::Closing {
                owner: Some(owner),
                destination: Some(destination),
                temporary: Some(temporary),
                file: Some(file),
                payload: Some(payload),
            },
            FixedFileDocumentWriteState::Ready { owner, destination, temporary, file, .. } => FixedFileDocumentWriteState::Closing {
                owner: Some(owner),
                destination: Some(destination),
                temporary: Some(temporary),
                file: Some(file),
                payload: None,
            },
            other => other,
        };
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        let FixedFileDocumentWriteState::Closing { owner, destination, temporary, file, payload } = &mut self.state else {
            return if matches!(self.state, FixedFileDocumentWriteState::Terminal | FixedFileDocumentWriteState::Published) {
                InteractiveJobCloseStep::Complete
            } else {
                InteractiveJobCloseStep::Blocked
            };
        };
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if file.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(path) = temporary.as_ref() {
            let bytes = path.as_os_str().len();
            if maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            match std::fs::remove_file(path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => return InteractiveJobCloseStep::Blocked,
            }
            drop(temporary.take());
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(retained) = payload.as_mut() {
            match retained.close_step(maximum_items.min(1), maximum_bytes) {
                JobPayloadCloseStep::Pending { released_items, released_bytes } => return InteractiveJobCloseStep::Pending { released_items, released_bytes },
                JobPayloadCloseStep::Complete if !retained.terminal_is_empty() => return InteractiveJobCloseStep::Blocked,
                JobPayloadCloseStep::Complete => {}
            }
            *payload = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let (Some(destination), Some(owner)) = (destination.as_ref(), *owner) {
            fixed_file_document_release_owner(destination, owner);
            *owner = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(path) = destination.as_ref() {
            let bytes = path.as_os_str().len();
            if maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            drop(destination.take());
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        self.state = FixedFileDocumentWriteState::Terminal;
        InteractiveJobCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(self.state, FixedFileDocumentWriteState::Terminal | FixedFileDocumentWriteState::Published)
    }
}

impl Drop for FixedFileDocumentWriteCursor {
    fn drop(&mut self) {
        debug_assert!(self.terminal_is_empty(), "fixed file document cursor requires bounded close to terminal before drop");
    }
}

fn fixed_file_document_owner_is_current(destination: &Path, owner: FixedOperationKey) -> bool {
    FIXED_FILE_DOCUMENT_WRITERS.lock().ok().and_then(|writers| writers.get(destination).copied()).is_some_and(|current| current == owner)
}

fn fixed_file_document_release_owner(destination: &Path, owner: FixedOperationKey) {
    let Ok(mut writers) = FIXED_FILE_DOCUMENT_WRITERS.lock() else { return };
    if writers.get(destination).is_some_and(|current| *current == owner) {
        writers.remove(destination);
    }
}

pub fn storage_worker_read_fixed_file_document(path: &Path, maximum_bytes: usize) -> std::io::Result<Vec<u8>> {
    if maximum_bytes > STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "fixed file document limit exceeds host-storage authority"));
    }
    let mut file = std::fs::File::open(path)?;
    let length = usize::try_from(file.metadata()?.len()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file document length is not representable"))?;
    if length > maximum_bytes {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file document exceeds admitted bytes"));
    }
    let mut bytes = vec![0; length];
    for chunk in bytes.chunks_mut(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
        file.read_exact(chunk)?;
    }
    let mut trailing = [0u8; 1];
    if file.read(&mut trailing)? != 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "fixed file document grew beyond admitted bytes"));
    }
    Ok(bytes)
}
