//! 📦️ `pack_io` — native file I/O for the `pack` binary document container family:
//! `FilePackSource`/`FilePackSink` implementing `pack_core`'s `PackSource`/`PackSink` traits over
//! `std::fs::File`, `write_atomic` (temp-file + fsync + rename, no partial file ever visible),
//! `StreamingPackWriter` (an incremental `crate::format::PackWriter<FilePackSink>` that flushes
//! segments to disk as produced rather than buffering the whole file in memory), and
//! `recover_file` (opens a file and forward-scans it via `crate::format::recover`).
//!
//! Everything here is native-only (`std::fs`, `std::sync::Mutex`) and gated behind
//! `#[cfg(not(target_arch = "wasm32"))]` so the crate still compiles — as an effectively-empty
//! crate — for a `wasm32-unknown-unknown` target check. See the `## pack_io` section of the
//! wave-0 contract at `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md`.

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::io::Write;
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    use crate::format::{Manifest, PackWriter, RecoveryReport, WriteOptions};
    use crate::{PackLimits, PackSink, PackSource};
    use semio_framework_pack_error::{PackError,PackTransportContext,PackTransportPhase,PackTransportCategory,PackRetryDisposition};
    use semio_framework_value::ValueRefusalKind;

    /// 🔌️ Native operations require an admitted concrete error cell before touching the filesystem.
    fn io_operation<T>(context:&PackTransportContext,bytes:usize,operation:impl FnOnce()->std::io::Result<(T,usize)>)->Result<T,PackError>{context.operation(PackTransportCategory::NativeIo,PackRetryDisposition::Never,bytes,operation)}

    /// 📤️ One pre-reserved source cell covers bounded writes and every policy checkpoint.
    fn write_bounded(file:&mut std::fs::File,bytes:&[u8],context:&PackTransportContext)->Result<(),PackError>{
        let source=context.reserve::<std::io::Error>(PackTransportCategory::NativeIo,PackRetryDisposition::Never)?;
        for page in bytes.chunks(4096){
            context.checkpoint(PackTransportPhase::BeforeOperation,PackTransportCategory::NativeIo,page.len())?;
            if let Err(error)=file.write_all(page){return Err(PackError::TransportFailure(source.publish(error)));}
            context.checkpoint(PackTransportPhase::AfterOperation,PackTransportCategory::NativeIo,page.len())?;
        }
        Ok(())
    }

    //#region 🔖️File
    /// 📂️ A read-only, random-access file source. Positional reads go through
    /// `std::os::unix::fs::FileExt::read_at`/`std::os::windows::fs::FileExt::seek_read` (both
    /// take `&self`, no locking needed) on unix/windows, and a `Mutex`-guarded seek+read fallback
    /// on any other native target — kept behind one `Mutex<File>` field for a single code path.
    pub struct FilePackSource {
        file: Mutex<std::fs::File>,
        len: u64,
        context:PackTransportContext,
    }

    impl FilePackSource {
        /// 📖️ Opens `path` for reading and stat's its length up front.
        pub fn open(path: &Path,context:PackTransportContext) -> Result<Self, PackError> {
            let file = io_operation(&context,0,||std::fs::File::open(path).map(|file|(file,0)))?;
            let len = io_operation(&context,0,||file.metadata().map(|metadata|(metadata,0)))?.len();
            Ok(Self { file: Mutex::new(file), len, context })
        }
    }

    // 🧮️ `PackSource`/`PackSink` (owned by `semio-framework-replication`, not this packet) are
    // already plain-AFIT `async` traits — see `📓️terra-pack-finish-report.md`
    // §"pure-computation-made-async: the recipe". These impls do genuinely-blocking `std::fs`
    // I/O in an `async fn` body (no `.await` inside): that mirrors the crate's existing idiom of
    // spawning a dedicated thread for blocking work rather than pretending file I/O suspends.
    impl PackSource for FilePackSource {
        async fn len(&self) -> u64 {
            self.len
        }

        async fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError> {
            if offset > self.len {
                return Err(PackError::Truncated(offset));
            }
            let maximum=buf.len().min(4096);let buf=&mut buf[..maximum];
            #[cfg(unix)]
            {
                use std::os::unix::fs::FileExt;
                io_operation(&self.context,buf.len(),||{let guard=self.file.lock().unwrap_or_else(std::sync::PoisonError::into_inner);guard.read_at(buf,offset).map(|bytes|(bytes,bytes))})
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::FileExt;
                io_operation(&self.context,buf.len(),||{let guard=self.file.lock().unwrap_or_else(std::sync::PoisonError::into_inner);guard.seek_read(buf,offset).map(|bytes|(bytes,bytes))})
            }
            #[cfg(not(any(unix, windows)))]
            {
                use std::io::{Read, Seek, SeekFrom};
                io_operation(&self.context,buf.len(),||{let mut guard=self.file.lock().unwrap_or_else(std::sync::PoisonError::into_inner);guard.seek(SeekFrom::Start(offset))?;guard.read(buf).map(|bytes|(bytes,bytes))})
            }
        }
    }

    /// 📤️ A write-only file sink opened truncate-on-create; tracks its own write position
    /// since `std::fs::File` exposes none without a `&mut self` seek.
    pub struct FilePackSink {
        file: std::fs::File,
        position: u64,
        context:PackTransportContext,
    }

    impl FilePackSink {
        /// 🆕️ Creates (truncating any existing file) `path` for writing.
        pub fn create(path: &Path,context:PackTransportContext) -> Result<Self, PackError> {
            let file = io_operation(&context,0,||std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path).map(|file|(file,0)))?;
            Ok(Self { file, position: 0,context })
        }
    }

    impl PackSink for FilePackSink {
        async fn write_all(&mut self, bytes: &[u8]) -> Result<(), PackError> {
            write_bounded(&mut self.file,bytes,&self.context)?;
            self.position += bytes.len() as u64;
            Ok(())
        }

        async fn position(&self) -> u64 {
            self.position
        }

        async fn flush(&mut self) -> Result<(), PackError> {
            io_operation(&self.context,0,||self.file.flush().map(|value|(value,0)))?;
            io_operation(&self.context,0,||self.file.sync_all().map(|value|(value,0)))
        }
    }
    //#endregion 🔖️File

    //#region 🔖️Atomic
    /// 🔢️ Per-process monotonic counter mixed into temp-file names so concurrent
    /// `write_atomic` calls (even to the same `path`, even on the same PID) never collide.
    static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    /// 🛟️ Writes `bytes` to `path` atomically: writes to a sibling temp file, `fsync`s it,
    /// then `rename`s it into place. `rename` is atomic on every platform this targets, so a
    /// reader can never observe a partially-written `path` — it sees either the old content or
    /// the fully-written new content, never a torn write.
    pub fn write_atomic(path: &Path, bytes: &[u8],context:&PackTransportContext) -> Result<(), PackError> {
        let file_name = path.file_name().ok_or(PackError::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"native-path",offset:0,detail:"write_atomic: path has no file name"})?;
        let pid = std::process::id();
        let counter = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let tmp_name = format!("{}.tmp-{pid}-{counter}", file_name.to_string_lossy());
        let tmp_path = path.with_file_name(tmp_name);
        {
            let mut tmp_file = io_operation(context,0,||std::fs::File::create(&tmp_path).map(|file|(file,0)))?;
            write_bounded(&mut tmp_file,bytes,context)?;
            io_operation(context,0,||tmp_file.sync_all().map(|value|(value,0)))?;
        }
        io_operation(context,0,||std::fs::rename(&tmp_path,path).map(|value|(value,0)))?;
        Ok(())
    }
    //#endregion 🔖️Atomic

    //#region 🔖️Stream
    /// 🌊️ An incremental pack file writer: segments/chunks are framed and flushed straight
    /// to disk as they're written rather than buffered in memory for the whole file, wrapping
    /// `crate::format::PackWriter<FilePackSink>`.
    pub struct StreamingPackWriter {
        inner: PackWriter<FilePackSink>,
    }

    impl StreamingPackWriter {
        /// 🚀️ Creates `path` and writes the 32-byte header.
        pub async fn create(path: &Path, options: &WriteOptions,context:PackTransportContext) -> Result<Self, PackError> {
            let sink = FilePackSink::create(path,context)?;
            let inner = PackWriter::begin(sink, options).await?;
            Ok(Self { inner })
        }

        /// 🖇️ Frames, compresses, CRCs, and flushes one segment to disk.
        pub async fn write_segment(&mut self, kind: u8, payload: &[u8]) -> Result<(), PackError> {
            self.inner.write_segment(kind, payload).await
        }

        pub async fn begin_identity_chunk(&mut self, payload_len: usize) -> Result<crate::format::PackIdentityChunk<'_, FilePackSink>, PackError> {
            self.inner.begin_identity_chunk(payload_len).await
        }

        /// 🏁️ Writes the chunk table, manifest, end marker, and footer, `fsync`s (via
        /// `FilePackSink::flush`, called internally by `PackWriter::finish`), and closes the file.
        pub async fn finish(self, manifest: &Manifest) -> Result<(), PackError> {
            self.inner.finish(manifest).await?;
            Ok(())
        }
    }
    //#endregion 🔖️Stream

    //#region 🔖️Recover
    /// 🩺️ Opens `path` and forward-scans it via `crate::format::recover` — for use when a
    /// file's footer fails to parse/validate and the caller wants to salvage whatever valid
    /// segments precede the corruption.
    pub async fn recover_file(path: &Path, limits: &PackLimits,context:PackTransportContext) -> Result<RecoveryReport, PackError> {
        let source = FilePackSource::open(path,context)?;
        crate::format::recover(&source, limits).await
    }
    //#endregion 🔖️Recover

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️native-unit/🦀️.rs");
    //#endregion 🧪️Tests
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path = "🧪️tests/🧭️producer-authority/🦀️.rs"]
mod producer_authority_tests;
