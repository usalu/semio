//! 💾 Appends one sealed first-genesis archive with the folder log's existing physical authority.
use super::{archive_readback::{self, DocumentRecord}, extend_folder_checksum, FolderEventLogStorage,
    DOCUMENT_ARCHIVE_PUT_EVENT, FOLDER_EVENT_MAGIC};
use semio_framework_hash::Sha256;
use semio_os_spr::channel::{
    DOCUMENT_ARCHIVE_VERSION, FIRST_GENESIS_ARCHIVE_PAGE_BYTES, FirstGenesisArchiveOffer,
    FirstGenesisArchivePageReceipt, FirstGenesisArchiveScope, FirstGenesisArchiveSeal,
};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, ErrorKind, Read, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

type VcsError = semio_os_vcs::VcsError;
const PAGE: usize = FIRST_GENESIS_ARCHIVE_PAGE_BYTES;
static NEXT: AtomicU64 = AtomicU64::new(1);

/// 📥 Keeps the exact accepted pages in a private file until the producer seals their digest.
pub struct NativeFirstArchiveCapture {
    offer: FirstGenesisArchiveOffer,
    path: PathBuf,
    destination: PathBuf,
    file: Option<File>,
    written: u32,
    digest: Sha256,
    failed: bool,
}

impl Drop for NativeFirstArchiveCapture {
    fn drop(&mut self) {
        drop(self.file.take());
        let _ = fs::remove_file(&self.path);
    }
}

impl NativeFirstArchiveCapture {
    /// 🧾 Receipts the exact ordered page after writing it, including an exact repeated offer.
    pub fn accept_page(
        &mut self, offer: &FirstGenesisArchiveOffer, offset: u32, bytes: &[u8],
    ) -> io::Result<FirstGenesisArchivePageReceipt> {
        if self.failed || *offer != self.offer || self.offer.page_length(offset) != Some(bytes.len()) {
            return Err(invalid("unexpected first-genesis archive page"));
        }
        let length = bytes.len() as u32;
        let file = self.file.as_mut().ok_or_else(|| invalid("first genesis capture is closed"))?;
        if offset == self.written {
            if let Err(error) = file.seek(SeekFrom::Start(u64::from(offset))).and_then(|_| file.write_all(bytes)) {
                self.failed = true;
                return Err(error);
            }
            self.written += length;
            self.digest.update(bytes);
        } else if offset.checked_add(length) == Some(self.written) {
            let mut prior = [0; PAGE];
            file.seek(SeekFrom::Start(u64::from(offset)))?;
            file.read_exact(&mut prior[..bytes.len()])?;
            if &prior[..bytes.len()] != bytes { return Err(invalid("first-genesis archive retry changed bytes")); }
        } else {
            return Err(invalid("first-genesis archive page changed order"));
        }
        Ok(FirstGenesisArchivePageReceipt { offer: offer.clone(), offset, length })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirstArchiveCommit { Created, AlreadyPresent }

impl FolderEventLogStorage {
    /// 🌱 Admits only the previously validated source offer and starts its private bounded spool.
    pub fn capture_first_genesis_archive(
        &self, offer: FirstGenesisArchiveOffer, expected: &FirstGenesisArchiveScope,
    ) -> Result<NativeFirstArchiveCapture, VcsError> {
        if offer.scope != *expected || offer.page_length(0).is_none() {
            return Err(backbone("first-genesis archive does not match the accepted source"));
        }
        self.refuse_repository_root()?;
        let destination = self.event_path();
        let parent = destination.parent().expect("folder log has its own directory");
        fs::create_dir_all(parent).map_err(backbone)?;
        for _ in 0..32 {
            let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(".genesis-{}-{ordinal:016x}.stage", std::process::id()));
            match OpenOptions::new().read(true).write(true).create_new(true).open(&path) {
                Ok(file) => return Ok(NativeFirstArchiveCapture {
                    offer, path, destination, file: Some(file), written: 0, digest: Sha256::new(), failed: false,
                }),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(backbone(error)),
            }
        }
        Err(backbone("first-genesis archive spool could not choose a unique name"))
    }

    /// 🗃️ Acknowledges a sealed archive only after sync and bounded exact readback under the writer lock.
    pub async fn commit_first_genesis_archive(
        &self, capture: NativeFirstArchiveCapture, seal: &FirstGenesisArchiveSeal,
        scope: &FirstGenesisArchiveScope, schema: &str, current: impl Fn() -> bool,
    ) -> Result<FirstArchiveCommit, VcsError> {
        self.commit_first_genesis_archive_at(capture, seal, scope, schema, super::now_ms().await, current)
    }

    pub(super) fn commit_first_genesis_archive_at(
        &self, mut capture: NativeFirstArchiveCapture, seal: &FirstGenesisArchiveSeal,
        scope: &FirstGenesisArchiveScope, schema: &str, timestamp: u64, current: impl Fn() -> bool,
    ) -> Result<FirstArchiveCommit, VcsError> {
        self.refuse_repository_root()?;
        if capture.destination != self.event_path() || capture.offer != seal.offer || seal.offer.scope != *scope
            || capture.failed || capture.written != capture.offer.extent || capture.digest.clone().finalize() != seal.sha256
            || schema.is_empty() || schema.len() > 1024
        { return Err(backbone("first-genesis archive seal or authority does not match the capture")); }
        let document = scope.document().parent_document_id.as_deref()
            .ok_or_else(|| backbone("first-genesis archive document is missing"))?;
        let extent = u64::from(capture.written);
        let spool = capture.file.as_mut().ok_or_else(|| backbone("first-genesis archive spool is closed"))?;
        if spool.metadata().map_err(backbone)?.len() != extent { return Err(backbone("first-genesis archive spool extent changed")); }
        check_live(&current).map_err(backbone)?;
        validate_archive(spool, extent).map_err(backbone)?;
        let mut prefix = Vec::new();
        prefix.push(DOCUMENT_ARCHIVE_PUT_EVENT);
        prefix.extend_from_slice(&timestamp.to_le_bytes());
        FolderEventLogStorage::encode_text(&mut prefix, document)?;
        FolderEventLogStorage::encode_text(&mut prefix, schema)?;
        prefix.extend_from_slice(&extent.to_le_bytes());
        let tail = [0; 8];
        let mut checksum = extend_folder_checksum(0xcbf29ce484222325, &prefix);
        let mut captured_hash = Sha256::new();
        let mut page = [0; PAGE];
        let cancelled = || !current();
        spool.rewind().map_err(backbone)?;
        archive_readback::read_pages(spool, extent, &mut page, &cancelled, |bytes| {
            checksum = extend_folder_checksum(checksum, bytes);
            captured_hash.update(bytes);
            Ok(())
        }).map_err(backbone)?;
        if captured_hash.finalize() != seal.sha256 { return Err(backbone("first-genesis spool changed after receipt")); }
        checksum = extend_folder_checksum(checksum, &tail);

        let _guard = self.writer.lock().map_err(backbone)?;
        let mut file = OpenOptions::new().read(true).write(true).create(true).truncate(false)
            .open(&capture.destination).map_err(backbone)?;
        file.try_lock().map_err(backbone)?;
        let file_len = file.metadata().map_err(backbone)?.len();
        let scan = archive_readback::scan_folder(&mut file, file_len, document, Some(schema), &cancelled).map_err(backbone)?;
        match scan.latest {
            Some(DocumentRecord::Archive { start, length, schema_matches: true }) if length == extent => {
                if hash_range(&mut file, start, length, &cancelled).map_err(backbone)? == seal.sha256 {
                    file.sync_data().map_err(backbone)?;
                    Self::sync_directory(capture.destination.parent().expect("folder log directory"))?;
                    Self::sync_directory(&self.folder)?;
                    check_live(&current).map_err(backbone)?;
                    return Ok(FirstArchiveCommit::AlreadyPresent);
                }
                return Err(backbone("existing first-genesis archive differs from the captured bytes"));
            }
            Some(_) => return Err(backbone("first-genesis document already has a different durable record")),
            None => {}
        }
        check_live(&current).map_err(backbone)?;
        let start = scan.valid_end;
        if start != file_len { file.set_len(start).map_err(backbone)?; }
        file.seek(SeekFrom::Start(start)).map_err(backbone)?;
        spool.rewind().map_err(backbone)?;
        let body_len = prefix.len() as u64 + extent + tail.len() as u64;
        let committed = (|| {
            file.write_all(FOLDER_EVENT_MAGIC).map_err(backbone)?;
            file.write_all(&body_len.to_le_bytes()).map_err(backbone)?;
            file.write_all(&checksum.to_le_bytes()).map_err(backbone)?;
            file.write_all(&prefix).map_err(backbone)?;
            archive_readback::read_pages(spool, extent, &mut page, &cancelled, |bytes| file.write_all(bytes))
                .map_err(backbone)?;
            file.write_all(&tail).map_err(backbone)?;
            file.sync_data().map_err(backbone)?;
            let committed_end = start + 24 + body_len;
            let readback = archive_readback::scan_folder(&mut file, committed_end, document, Some(schema), &cancelled).map_err(backbone)?;
            if file.metadata().map_err(backbone)?.len() != committed_end || readback.valid_end != committed_end
                || !matches!(readback.latest, Some(DocumentRecord::Archive { start: offset, length, schema_matches: true })
                    if offset == start + 24 + prefix.len() as u64 && length == extent)
                || hash_range(&mut file, start + 24 + prefix.len() as u64, extent, &cancelled).map_err(backbone)? != seal.sha256
            { return Err(backbone("first-genesis archive durable readback differs from the sealed bytes")); }
            Self::sync_directory(capture.destination.parent().expect("folder log directory"))?;
            Self::sync_directory(&self.folder)?;
            check_live(&current).map_err(backbone)
        })();
        if let Err(error) = committed {
            return match file.set_len(start).and_then(|()| file.sync_data()) {
                Ok(()) => Err(error),
                Err(rollback) => Err(backbone(format!("first-genesis archive rollback after {error}: {rollback}"))),
            };
        }
        Ok(FirstArchiveCommit::Created)
    }
}

fn validate_archive(file: &mut File, extent: u64) -> io::Result<()> {
    file.rewind()?;
    let mut version = [0];
    file.read_exact(&mut version)?;
    if version != [DOCUMENT_ARCHIVE_VERSION] { return Err(invalid("unexpected first-genesis archive version")); }
    for _ in 0..2 {
        let length = next_varint(file)?;
        let offset = file.stream_position()?;
        if length == 0 || length >= extent.saturating_sub(offset) {
            return Err(invalid("first-genesis archive field extent is invalid"));
        }
        file.seek(SeekFrom::Current(length as i64))?;
    }
    if next_varint(file)? != 0 || file.stream_position()? != extent {
        return Err(invalid("first-genesis archive must contain exactly its parent pair"));
    }
    Ok(())
}

fn next_varint(file: &mut File) -> io::Result<u64> {
    let mut value = 0;
    for index in 0..10 {
        let mut byte = [0];
        file.read_exact(&mut byte)?;
        let [byte] = byte;
        if index == 9 && byte > 1 { break; }
        value |= u64::from(byte & 127) << (index * 7);
        if byte < 128 {
            if index != 0 && byte == 0 { break; }
            return Ok(value);
        }
    }
    Err(invalid("invalid first-genesis archive length"))
}

fn hash_range(file: &mut File, start: u64, length: u64, cancelled: &impl Fn() -> bool) -> io::Result<[u8; 32]> {
    let mut digest = Sha256::new();
    let mut page = [0; PAGE];
    file.seek(SeekFrom::Start(start))?;
    archive_readback::read_pages(file, length, &mut page, cancelled, |bytes| {
        digest.update(bytes);
        Ok(())
    })?;
    Ok(digest.finalize())
}

fn check_live(current: &impl Fn() -> bool) -> io::Result<()> {
    if current() { Ok(()) } else { Err(io::Error::new(ErrorKind::Interrupted, "first-genesis archive authority closed")) }
}

fn invalid(message: &'static str) -> io::Error { io::Error::new(ErrorKind::InvalidData, message) }
fn backbone(error: impl ToString) -> VcsError { VcsError::Backbone(error.to_string()) }
