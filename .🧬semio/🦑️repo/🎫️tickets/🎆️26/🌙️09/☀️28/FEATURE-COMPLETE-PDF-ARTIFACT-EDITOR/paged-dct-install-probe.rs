#![allow(dead_code)]

use pack::PackRefusal;
use semio_framework_artifact_reference::{ArtifactRef, ArtifactDialect};
use semio_framework_pack_source::{PackSink, PackSource};
use semio_framework_value::{RetainedCloneGrant, paged::{ByteReadSource, PagedBytes}};
use std::{
    fs::{self, File}, future::Future, io::{Read, Seek, SeekFrom, Write}, path::Path, sync::Mutex, task::{Context, Poll, Waker},
};

include!("identity.rs");
#[path = "📖️dct-view.rs"]
mod dct_view;
#[path = "🗃️install-pack.rs"]
mod install_pack;

use dct_view::{PdfDctImageColor, PdfDctImageView};
use install_pack::{NativeDctInstallPack, PagedDctInstallPayload, write_dct_install_pack};

#[derive(Debug)]
enum TestError { Pack(PackRefusal), Io(std::io::Error), Canceled }
impl From<PackRefusal> for TestError { fn from(value: PackRefusal) -> Self { Self::Pack(value) } }
impl From<std::io::Error> for TestError { fn from(value: std::io::Error) -> Self { Self::Io(value) } }

struct FilePayload { file: Mutex<File>, length: usize }
impl FilePayload {
    fn open(path: &Path) -> Result<Self, TestError> {
        let file = File::open(path)?;
        let length = file.metadata()?.len() as usize;
        Ok(Self { file: Mutex::new(file), length })
    }
}
impl ByteReadSource for FilePayload {
    fn byte_len(&self) -> usize { self.length }
    fn byte_at(&self, index: usize) -> Option<u8> {
        let mut data = [0; 1];
        (ByteReadSource::read_bytes(self, index, &mut data) == 1).then_some(data[0])
    }
    fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
        assert!(output.len() <= 4096);
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        file.read(output).unwrap()
    }
}
impl PackSource for FilePayload {
    type Error = TestError;
    async fn len(&self) -> u64 { self.length as u64 }
    async fn read_at(&self, offset: u64, output: &mut [u8]) -> Result<usize, Self::Error> {
        let limit = output.len().min(4096);
        Ok(ByteReadSource::read_bytes(self, offset as usize, &mut output[..limit]))
    }
}

struct FileSink { file: File, length: u64, peak: usize }
impl FileSink {
    fn create(path: &Path) -> Result<Self, TestError> {
        Ok(Self { file: File::create(path)?, length: 0, peak: 0 })
    }
}
impl PackSink for FileSink {
    type Error = TestError;
    async fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        assert!(data.len() <= 4096);
        self.file.write_all(data)?;
        self.length += data.len() as u64;
        self.peak = self.peak.max(data.len());
        Ok(())
    }
    async fn position(&self) -> u64 { self.length }
    async fn flush(&mut self) -> Result<(), Self::Error> { self.file.flush().map_err(Into::into) }
}

fn block<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("file-backed harness yielded unexpectedly"),
    }
}

fn read_paged_file(mut file: File) -> Result<PagedBytes<{ isize::MAX as usize }>, TestError> {
    let mut payload = PagedBytes::default();
    let mut fragment = [0; 4096];
    loop {
        let count = file.read(&mut fragment)?;
        if count == 0 { return Ok(payload); }
        for &byte in &fragment[..count] {
            loop {
                let demand = payload.byte_backing_demand().unwrap();
                if demand.capacity_bytes == 0 && demand.copy_bytes == 0 && demand.depth == 0 { break; }
                payload.reserve_byte_backing(RetainedCloneGrant {
                    maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes,
                    maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth,
                }).unwrap();
            }
            payload.push_reserved_byte(byte).unwrap();
        }
    }
}

async fn run(dir: &Path) -> Result<(), TestError> {
    let input = dir.join("large.jpg");
    let source = read_paged_file(File::open(&input)?)?;
    let mut hash = PdfDctResourceIdentity::new(Some(1)).unwrap();
    let mut window = [0; 4096];
    while !hash.complete(source.byte_len()) {
        let used = source.read_bytes(hash.position(), &mut window);
        assert!(used != 0);
        hash.append(&window[..used]);
    }
    let view = PdfDctImageView::from_admitted(
        &source, (1024, 1024), PdfDctImageColor::Rgb { transform: Some(true) }, hash.digest(),
    ).unwrap();
    let reference = view.reference();
    let archive = dir.join("large.dct-install.pack");
    let partial = dir.join("canceled.dct-install.pack");
    let mut fragments = 0;
    let canceled = write_dct_install_pack(FileSink::create(&partial)?, view, async |count| {
        assert!(count <= 4096);
        fragments += 1;
        if fragments == 4 { return Err(TestError::Canceled); }
        Ok(())
    }).await;
    assert!(matches!(canceled, Err(TestError::Canceled)));
    assert!(NativeDctInstallPack::open(FilePayload::open(&partial)?, &reference).await.is_err());
    fs::remove_file(partial)?;

    let sink = write_dct_install_pack(FileSink::create(&archive)?, view, async |count| {
        assert!(count <= 4096);
        Ok(())
    }).await?;
    assert_eq!(sink.peak, 4096);
    drop(sink);
    drop(source);

    let payload = read_paged_file(File::open(&archive)?)?;
    let unrelated = PdfDctResourceIdentity::reference_for_digest([42; 32]);
    assert!(NativeDctInstallPack::open(PagedDctInstallPayload(&payload), &unrelated).await.is_err());
    let reopened = NativeDctInstallPack::open(PagedDctInstallPayload(&payload), &reference).await?;
    let facts = reopened.facts();
    assert_eq!(facts.reference(), reference);
    assert_eq!(facts.dimensions, (1024, 1024));
    assert_eq!(facts.color.color_transform(), Some(1));

    let mut reader = reopened.verify();
    let mut extracted = File::create(dir.join("large.dct-install.jpg"))?;
    let mut streamed = 0;
    while !reader.complete() {
        let received = reader.step(&mut window).await?;
        extracted.write_all(&window[..received])?;
        streamed += received;
    }
    assert_eq!(streamed, facts.length);
    extracted.flush()?;
    fs::write(dir.join("large.dct-install.ref"), &reference.artifact_id)?;
    let file_pack = pack::format::PackFile::open_manifest(FilePayload::open(&archive)?,
        &pack::codec::PackLimits::default(), pack::format::VerificationLevel::Full).await?;
    let data = file_pack.chunk_range(pack::codec::ChunkId(1))?.offset;
    drop(file_pack);
    let mut file = File::options().read(true).write(true).open(&archive)?;
    file.seek(SeekFrom::Start(data + 12))?;
    let mut byte = [0];
    file.read_exact(&mut byte)?;
    file.seek(SeekFrom::Start(data + 12))?;
    file.write_all(&[byte[0] ^ 1])?;
    file.flush()?;
    let damaged = NativeDctInstallPack::open(FilePayload::open(&archive)?, &reference).await?;
    let mut rejected = damaged.verify();
    let corruption = loop {
        match rejected.step(&mut window).await {
            Ok(_) if rejected.complete() => break false,
            Ok(_) => {}
            Err(TestError::Pack(_)) => break true,
            Err(error) => return Err(error),
        }
    };
    assert!(corruption);
    file.seek(SeekFrom::Start(data + 12))?;
    file.write_all(&byte)?;
    file.flush()?;
    println!("[DEBUG] install Pack persisted and reopened through actual PagedBytes, {streamed} original bytes and native DCT digest verified");
    Ok(())
}

fn main() -> Result<(), TestError> {
    let dir = std::env::args_os().nth(1).unwrap();
    block(run(Path::new(&dir)))
}
