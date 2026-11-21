//! 📤️ Writes one checked export in bounded parts beside its final native destination.

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(1);

struct StagedPath {
    path: PathBuf,
    destination: PathBuf,
    committed: bool,
}

impl Drop for StagedPath {
    fn drop(&mut self) {
        if !self.committed {
            let _ = fs::remove_file(&self.path);
        }
    }
}

pub struct NativeDownloadSink {
    file: File,
    staged: StagedPath,
    expected: u64,
    written: u64,
    failed: bool,
}

pub struct PreparedNativeDownload {
    staged: StagedPath,
}

impl NativeDownloadSink {
    /// 🧳️ Opens an exclusive sibling staging file under the selected save destination.
    pub fn create(destination: &Path, expected: u64) -> io::Result<Self> {
        let parent = destination.parent().filter(|value| !value.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
        if destination.file_name().is_none() { return Err(io::ErrorKind::InvalidInput.into()); }
        for _ in 0..32 {
            let id = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(".semio-save-{}-{id:016x}", std::process::id()));
            if destination.file_name().zip(path.file_name()).is_some_and(|(a,b)| a.to_string_lossy().eq_ignore_ascii_case(&b.to_string_lossy())) { continue; }
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(file) => return Ok(Self { file, staged: StagedPath { path, destination: destination.to_path_buf(), committed: false }, expected, written: 0, failed: false }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        Err(io::ErrorKind::AlreadyExists.into())
    }

    /// 📦️ Rejects bytes beyond the admitted total before modifying the staged file.
    pub fn write_page(&mut self, bytes: &[u8]) -> io::Result<()> {
        let count = u64::try_from(bytes.len()).map_err(|_| io::ErrorKind::InvalidData)?;
        let end = self.written.checked_add(count).filter(|value| *value <= self.expected)
            .ok_or(io::ErrorKind::InvalidData)?;
        if self.failed { return Err(io::ErrorKind::BrokenPipe.into()); }
        if let Err(error) = self.file.write_all(bytes) {
            self.failed = true;
            return Err(error);
        }
        self.written = end;
        Ok(())
    }

    /// 🔒️ Settles the whole exact file before the caller rechecks the live document authority.
    pub fn prepare(self) -> io::Result<PreparedNativeDownload> {
        if self.failed { return Err(io::ErrorKind::BrokenPipe.into()); }
        if self.written != self.expected || self.file.metadata()?.len() != self.expected { return Err(io::ErrorKind::UnexpectedEof.into()); }
        self.file.sync_all()?;
        drop(self.file);
        Ok(PreparedNativeDownload { staged: self.staged })
    }
}

impl PreparedNativeDownload {
    /// 🗃️ Publishes the complete staged file after the caller's final authority check.
    pub fn commit(mut self) -> io::Result<()> {
        fs::rename(&self.staged.path, &self.staged.destination)?;
        self.staged.committed = true;
        Ok(())
    }
}
