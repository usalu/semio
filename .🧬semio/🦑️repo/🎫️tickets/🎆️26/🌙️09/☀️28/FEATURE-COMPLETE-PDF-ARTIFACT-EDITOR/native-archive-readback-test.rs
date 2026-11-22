//! 🧪 Measures the actual folder readback and archive encoders at their format boundaries.
extern crate self as semio_framework_value;
use std::{
    alloc::{GlobalAlloc, Layout, System}, cell::Cell, fs::File, io::{self, Read, Seek, SeekFrom},
    path::Path, sync::atomic::{AtomicUsize, Ordering},
};
include!(env!("SEMIO_ARCHIVE_RECORDS"));

mod readback { include!(env!("SEMIO_ARCHIVE_READBACK")); }
mod legacy { use super::*; include!(env!("SEMIO_ARCHIVE_LEGACY")); }
mod preflight { use super::*; include!(env!("SEMIO_ARCHIVE_PREFLIGHT")); }
mod protocol {
    pub type ProtocolError = &'static str;
    include!(env!("SEMIO_ARCHIVE_PRIMITIVES"));
}

fn malformed(_: &'static str, _: u64, detail: &'static str) -> protocol::ProtocolError { detail }

pub trait ByteReadSource {
    fn byte_len(&self) -> usize;
    fn read_bytes(&self, offset: usize, buffer: &mut [u8]) -> usize;
}
impl ByteReadSource for [u8] {
    fn byte_len(&self) -> usize { self.len() }
    fn read_bytes(&self, offset: usize, buffer: &mut [u8]) -> usize {
        let count = (self.len() - offset).min(buffer.len());
        buffer[..count].copy_from_slice(&self[offset..offset + count]);
        count
    }
}
impl ByteReadSource for Vec<u8> {
    fn byte_len(&self) -> usize { self.len() }
    fn read_bytes(&self, offset: usize, buffer: &mut [u8]) -> usize { self.as_slice().read_bytes(offset, buffer) }
}

struct Meter;
static LARGEST: AtomicUsize = AtomicUsize::new(0);
#[global_allocator]
static ALLOCATOR: Meter = Meter;

unsafe impl GlobalAlloc for Meter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        LARGEST.fetch_max(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout); }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        LARGEST.fetch_max(size, Ordering::Relaxed);
        unsafe { System.realloc(pointer, layout, size) }
    }
}

struct MeasuredFile {
    file: File,
    reads: usize,
    maximum: usize,
}
impl Read for MeasuredFile {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.maximum = self.maximum.max(buffer.len());
        let count = self.file.read(buffer)?;
        self.reads += count;
        Ok(count)
    }
}
impl Seek for MeasuredFile {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> { self.file.seek(position) }
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let out = args.next().unwrap();
    let out = Path::new(&out);
    let source = std::fs::read(args.next().unwrap()).unwrap();
    let scannable = DocumentArchivePack {
        parent_pack: source, parent_spr: Vec::new(), members: Vec::new(),
    };
    LARGEST.store(0, Ordering::Relaxed);
    assert!(legacy::encode_document_archive_bytes(&scannable).is_err());
    let baseline = LARGEST.load(Ordering::Relaxed);
    assert!(baseline >= scannable.parent_pack.len());
    LARGEST.store(0, Ordering::Relaxed);
    assert!(preflight::encode_document_archive_bytes(&scannable).is_err());
    let staged = LARGEST.load(Ordering::Relaxed);
    assert_eq!(staged, 0);
    println!("oversized archive baseline maximum allocation {baseline}; preflight {staged}");

    let mut boundary = DocumentArchivePack {
        parent_pack: vec![0x69; DOCUMENT_ARCHIVE_MAXIMUM_BYTES - 7], parent_spr: Vec::new(), members: Vec::new(),
    };
    let encoded = preflight::encode_document_archive_bytes(&boundary).unwrap();
    assert_eq!(encoded.len(), DOCUMENT_ARCHIVE_MAXIMUM_BYTES);
    assert_eq!(encoded, legacy::encode_document_archive_bytes(&boundary).unwrap());
    boundary.parent_pack.push(0);
    LARGEST.store(0, Ordering::Relaxed);
    assert!(preflight::encode_document_archive_bytes(&boundary).is_err());
    assert_eq!(LARGEST.load(Ordering::Relaxed), 0);
    let mut member_archive = DocumentArchivePack::default();
    let mut member = OwnedDocumentMemberPackEntry::default();
    member.ordinal = 128;
    member.reference.artifact_id = "ä".repeat(64);
    member.owner.slot = "slot".into();
    member.envelope_pack = vec![1, 2, 3];
    member_archive.members.push(member);
    assert_eq!(legacy::encode_document_archive_bytes(&member_archive), preflight::encode_document_archive_bytes(&member_archive));

    let expected = std::fs::read(out.join("expected.semio")).unwrap();
    for case in std::fs::read_to_string(out.join("cases.tsv")).unwrap().lines() {
        let (name, oracle) = case.split_once('\t').unwrap();
        let file = File::open(out.join(format!("{name}.semio"))).unwrap();
        let length = file.metadata().unwrap().len();
        let mut file = MeasuredFile { file, reads: 0, maximum: 0 };
        let calls = Cell::new(0);
        LARGEST.store(0, Ordering::Relaxed);
        let result = readback::archive_matches(&mut file, length, "pdf-document", &expected, || {
            calls.set(calls.get() + 1);
            name == "cancelled" && calls.get() > 5
        });
        let allocation = LARGEST.load(Ordering::Relaxed);
        let actual = match result {
            Ok(true) => "equal",
            Ok(false) => "different",
            Err(error) if error.kind() == io::ErrorKind::Interrupted => "cancelled",
            Err(error) if error.kind() == io::ErrorKind::InvalidData => "invalid",
            Err(error) => panic!("{error}"),
        };
        assert_eq!(actual, oracle, "{name}");
        assert!(file.maximum <= 4096 && allocation < 4096, "{name}");
        if name == "exact-after-large-blob" {
            assert!(file.reads > 9_437_877);
            assert_eq!(file.maximum, 4096);
            assert_eq!(allocation, 0);
        }
        if name == "cancelled" { assert!(file.reads < length as usize); }
        println!("{name}: {actual}, requested read at most {}, allocation {allocation}, scanned {} bytes", file.maximum, file.reads);
    }
}
