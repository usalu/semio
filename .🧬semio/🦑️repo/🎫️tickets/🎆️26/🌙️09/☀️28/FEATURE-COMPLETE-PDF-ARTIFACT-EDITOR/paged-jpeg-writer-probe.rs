//! 🔬️ Exercises actual classic-writer framing with file-backed JPEG windows and injected sink backpressure.
#![allow(dead_code)]

extern crate self as semio_framework_artifact_reference;
extern crate self as semio_framework_value;
extern crate self as semio_framework_hash;
extern crate self as semio_s_artifact_stdio_pdf_model;
extern crate self as semio_s_artifact_stdio_pdf_snapshot_2;

use std::{
    fmt, fs::{self, File}, io::{self, Read, Seek, SeekFrom, Write}, path::Path, sync::Mutex,
};
#[path = "hash.rs"]
mod hash;
pub use hash::Sha256;

#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactRef { pub artifact_id: String, pub dialect: ArtifactDialect }
#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactDialect { pub artifact_kind: String, pub standard: String, pub subset: String }

#[path = "byte-source.rs"]
mod byte_source;

pub mod paged {
    pub const PAGED_BYTES_CHUNK_BYTES: usize = 4096;
    pub use super::byte_source::ByteReadSource;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjRef { pub num: u32, pub gen: u16 }

#[derive(Clone, Debug, PartialEq)]
pub struct PdfDictEntry { pub key: String, pub value: PdfObject }
impl PdfDictEntry {
    pub fn new(key: impl Into<String>, value: PdfObject) -> Self { Self { key: key.into(), value } }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PdfObject {
    Null, Bool(bool), Int(i64), Real(PdfDecimal), Str(Vec<u8>), Text(String), Date(String), Name(String),
    Array(Vec<PdfObject>), Dict(Vec<PdfDictEntry>), Ref(ObjRef),
    Stream { dict: Vec<PdfDictEntry>, data: Vec<u8>, filters: Vec<PdfStreamFilter> },
}
impl PdfObject {
    pub fn name(name: impl Into<String>) -> Self { Self::Name(name.into()) }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PdfDecimal(f64);
impl PdfDecimal {
    fn from_f64(value: f64) -> Self { Self(value) }
    fn to_f64(&self) -> Option<f64> { Some(self.0) }
}
impl fmt::Display for PdfDecimal {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(output) }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdfEncryption;

pub struct PdfIndirectObject { pub id: ObjRef, pub value: PdfObject }

#[derive(Clone, Debug, PartialEq)]
pub enum PdfStreamFilter {
    Flate { predictor: Option<PdfPredictor> },
    Lzw { predictor: Option<PdfPredictor>, early_change: bool },
    AsciiHex, Ascii85, RunLength, Dct { color_transform: Option<u32> }, Jpx,
    Ccitt { parameters: PdfCcittParameters }, Jbig2 { globals: Option<Vec<u8>> }, Crypt { name: Option<String> },
}
impl PdfStreamFilter {
    pub fn name(&self) -> &'static str {
        match self { Self::Dct { .. } => "DCTDecode", _ => unreachable!() }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PdfPredictor { pub predictor: u32, pub colors: u32, pub bits_per_component: u32, pub columns: u32 }
#[derive(Clone, Debug, PartialEq)]
pub struct PdfCcittParameters {
    pub k: i32, pub columns: u32, pub rows: u32, pub black_is_1: bool, pub encoded_byte_align: bool,
    pub end_of_line: bool, pub end_of_block: bool, pub damaged_rows_before_error: u32,
}

pub mod standards {
    pub mod v1_7 {
        pub mod subsets {
            pub mod base {
                pub mod schema {
                    pub mod snapshot { pub use crate::*; }
                }
                pub mod modules {
                    pub use crate::{writer, filters, lexer, xref, encryption};
                }
                pub mod io {
                    pub use crate::foreign_artifacts;
                    pub mod text_string {
                        pub fn encode_text_string(text: &str) -> Vec<u8> { text.as_bytes().to_vec() }
                    }
                    pub mod text {
                        pub mod snapshot {
                            pub mod date { pub fn print_pdf_date(_: &str) -> String { unreachable!() } }
                        }
                    }
                }
            }
        }
    }
}

#[path = "writer.rs"]
pub mod writer;

pub mod foreign_artifacts {
    use super::{semio_framework_hash, ArtifactRef, ArtifactDialect};
    include!("identity.rs");
    pub use crate::dct_view;
}
#[path = "📖️dct-view.rs"]
pub mod image_view;
pub use image_view as dct_view;

pub mod xref { include!("xref.rs"); }
pub mod lexer {
    use super::{PdfObject, PdfDictEntry, PdfDecimal};
    include!("lexer.rs");
}
pub mod filters {
    use super::{PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter};
    pub fn flate_encode(_: &[u8]) -> Vec<u8> { unreachable!() }
    fn lzw_encode(_: &[u8], _: bool) -> Vec<u8> { unreachable!() }
    fn predictor_encode(_: &[u8], _: &PdfPredictor) -> Vec<u8> { unreachable!() }
    fn ascii_hex_encode(_: &[u8]) -> Vec<u8> { unreachable!() }
    fn ascii85_encode(_: &[u8]) -> Vec<u8> { unreachable!() }
    fn run_length_encode(_: &[u8]) -> Vec<u8> { unreachable!() }
    include!("filters.rs");
}
pub mod encryption {
    use super::{PdfDictEntry, PdfEncryption, PdfObject};
    pub struct Decryptor;
    impl Decryptor {
        pub fn encrypt_object(&self, _: PdfObject, _: u32, _: u16) -> PdfObject { unreachable!() }
    }
    pub fn seal_standard_security(_: &PdfEncryption, _: &[u8], _: &[u8]) -> (Decryptor, Vec<PdfDictEntry>) {
        unreachable!()
    }
}

use dct_view::{PdfDctImageColor, PdfDctImageView};
use foreign_artifacts::PdfDctResourceIdentity;
use paged::{ByteReadSource, PAGED_BYTES_CHUNK_BYTES};
use writer::{DocumentTrailer, PdfWriter, WriteOptions};
use writer::dct_object::{DctObjectStartError, DctObjectWriteError};

struct FileSource { file: Mutex<File>, length: usize }
impl ByteReadSource for FileSource {
    fn byte_len(&self) -> usize { self.length }
    fn byte_at(&self, index: usize) -> Option<u8> {
        let mut byte = [0; 1];
        (self.read_bytes(index, &mut byte) == 1).then_some(byte[0])
    }
    fn read_bytes(&self, index: usize, output: &mut [u8]) -> usize {
        assert!(output.len() <= PAGED_BYTES_CHUNK_BYTES);
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(index as u64)).unwrap();
        file.read_exact(output).unwrap();
        output.len()
    }
}

struct ShortSink { output: File, calls: usize, written: usize }
impl Write for ShortSink {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.calls += 1;
        match self.calls % 23 {
            0 => return Err(io::Error::from(io::ErrorKind::Interrupted)),
            1 => return Err(io::Error::from(io::ErrorKind::WouldBlock)),
            2 => return Ok(0),
            _ => {}
        }
        let count = self.output.write(&bytes[..bytes.len().min(1501)])?;
        self.written += count;
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> { self.output.flush() }
}

fn object(num: u32, value: PdfObject) -> PdfIndirectObject { PdfIndirectObject { id: id(num), value } }
fn id(num: u32) -> ObjRef { ObjRef { num, gen: 0 } }
fn dict(entries: &[(&str, PdfObject)]) -> PdfObject {
    PdfObject::Dict(entries.iter().map(|(key, value)| PdfDictEntry::new(*key, value.clone())).collect())
}
fn pdf_ref(num: u32) -> PdfObject { PdfObject::Ref(id(num)) }

fn write_pdf(dir: &Path, name: &str, dimensions: (u16, u16), color: PdfDctImageColor) -> io::Result<()> {
    let jpeg = File::open(dir.join(format!("{name}.jpg")))?;
    let length = jpeg.metadata()?.len() as usize;
    let source = FileSource { file: Mutex::new(jpeg), length };
    let mut identity = PdfDctResourceIdentity::new(color.color_transform()).unwrap();
    let mut page = [0; PAGED_BYTES_CHUNK_BYTES];
    while !identity.complete(length) {
        let offset = identity.position();
        let count = (length - offset).min(page.len());
        assert_eq!(source.read_bytes(offset, &mut page[..count]), count);
        identity.append(&page[..count]);
    }
    let view = PdfDctImageView::from_admitted(&source, dimensions, color, identity.digest()).unwrap();
    emit_pdf(dir, name, view)
}

fn emit_pdf(dir: &Path, name: &str, view: PdfDctImageView<'_>) -> io::Result<()> {
    let dimensions = view.dimensions();
    let color = view.color();
    let length = view.encoded().byte_len();
    let source = view.encoded();
    fs::write(dir.join(format!("{name}.stream-ref")), view.reference().artifact_id)?;

    let (width, height) = dimensions;
    let (mut writer, header) = PdfWriter::begin("1.7", &WriteOptions::default(), &[], &[]);
    let parts = [
        object(1, dict(&[("Type", PdfObject::name("Catalog")), ("Pages", pdf_ref(2))])),
        object(2, dict(&[("Type", PdfObject::name("Pages")), ("Kids", PdfObject::Array(vec![pdf_ref(3)])), ("Count", PdfObject::Int(1))])),
        object(3, dict(&[
            ("Type", PdfObject::name("Page")),
            ("Parent", pdf_ref(2)),
            ("MediaBox", PdfObject::Array(vec![0, 0, width.into(), height.into()].into_iter().map(PdfObject::Int).collect())),
            ("Resources", dict(&[("XObject", dict(&[("Im", pdf_ref(5))]))])),
            ("Contents", pdf_ref(4)),
        ])),
        object(4, PdfObject::Stream {
            dict: Vec::new(), data: format!("q {width} 0 0 {height} 0 0 cm /Im Do Q").into_bytes(), filters: Vec::new(),
        }),
    ];
    let mut output = File::create(dir.join(format!("{name}.stream.pdf")))?;
    output.write_all(&header)?;
    for part in parts { output.write_all(&writer.object(&part))?; }
    let duplicate = writer.begin_dct_object(id(4), view);
    let writer = match duplicate {
        Err((DctObjectStartError::InvalidObject, writer)) => writer,
        _ => panic!("already-written object number was accepted"),
    };
    let mut image_writer = writer.begin_dct_object(id(5), view).ok().expect("begin DCT object");
    let mut sink = ShortSink { output, calls: 0, written: 0 };
    const UNRELATED_DIGEST: [u8; 32] = [1; 32];
    let unrelated = PdfDctImageView::from_admitted(source, dimensions, color, UNRELATED_DIGEST).unwrap();
    let refused = image_writer.step(unrelated, &mut sink, 8192);
    assert!(matches!(refused.result, Err(DctObjectWriteError::ResourceMismatch)));
    assert_eq!(refused.counts.read + refused.counts.written, 0);
    let idle = image_writer.step(view, &mut sink, 0);
    assert!(matches!(idle.result, Ok(false)) && idle.counts.written == 0);
    image_writer = match image_writer.finish() {
        Err(pending) => pending,
        Ok(_) => panic!("unfinished object returned writer"),
    };
    let (mut peak_read, mut peak_write) = (0, 0);
    loop {
        let turn = image_writer.step(view, &mut sink, 8192);
        assert!(turn.counts.read + turn.counts.written <= 8192);
        peak_read = peak_read.max(turn.counts.read);
        peak_write = peak_write.max(turn.counts.written);
        match turn.result {
            Ok(true) => break,
            Ok(false) => {}
            Err(DctObjectWriteError::Sink(error)) if matches!(error.kind(), io::ErrorKind::Interrupted | io::ErrorKind::WouldBlock) => {
                assert_eq!(turn.counts.written, 0);
            }
            Err(error) => panic!("{error:?}"),
        }
    }
    assert!(sink.written > length);
    assert!(peak_read <= PAGED_BYTES_CHUNK_BYTES && peak_write <= 1501);
    let mut writer = image_writer.finish().ok().expect("finished DCT object");
    sink.output.write_all(&writer.object(&object(6, dict(&[("Producer", PdfObject::Str(b"Bounded PDF writer probe".to_vec()))]))))?;
    sink.output.write_all(&writer.finish(&DocumentTrailer { root: id(1), info: Some(id(6)), id: None, extra: Vec::new() }))?;
    sink.output.flush()?;
    println!("[DEBUG] {name} emitted {length} JPEG bytes, peak source read {peak_read}, peak sink transfer {peak_write}");
    Ok(())
}

fn main() -> io::Result<()> {
    let dir = std::env::args_os().nth(1).unwrap();
    let dir = Path::new(&dir);
    use PdfDctImageColor::{Gray, Rgb};
    let (writer, _) = PdfWriter::begin("1.7", &WriteOptions { xref_stream: true, encryption: None }, &[], &[]);
    assert_eq!(writer.version(), "1.7");
    write_pdf(dir, "large", (1024, 1024), Rgb { transform: Some(true) })?;
    write_pdf(dir, "gray-restart", (29, 24), Gray { transform: None })?;
    write_pdf(dir, "adobe-rgb", (29, 24), Rgb { transform: Some(false) })?;
    write_pdf(dir, "unknown-metadata", (29, 24), Gray { transform: None })
}
