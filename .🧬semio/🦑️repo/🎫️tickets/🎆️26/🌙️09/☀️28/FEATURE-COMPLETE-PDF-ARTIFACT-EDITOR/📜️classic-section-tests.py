"""🔧 Test the production PDF lexer and native object grammar without the workspace build graph."""
from pathlib import Path
import subprocess,sys
l,t=map(Path,sys.argv[1:3]); out=t/"🗑️generated"
source=(l.parents[1]/"🧬️schema/📸️snapshot/📐️model/🔗️object-model/🦀️.rs").read_text()
source="\n".join(line for line in source.splitlines() if not line.startswith(("#[canonical_json", "#[value")) and not line.lstrip().startswith("#[value"))
import re
source=re.sub(r'#\[value\([^\]]*\)\]\s*','',source)
for name in ("semio_framework_value::RetireOwned","semio_framework_value::CanonicalJsonTree","semio_framework_value::RetainedClone","value_derive::ToValue","value_derive::FromValue"):
    source=source.replace(name+", ","").replace(", "+name,"")
module=out/"classic-section-model.rs"
module.write_text(source)
h=out/"classic-section-harness.rs"
h.write_text('''
extern crate self as semio_s_artifact_stdio_pdf_model;
extern crate self as semio_s_artifact_stdio_pdf_lexer;
extern crate self as semio_s_artifact_stdio_pdf_snapshot_2;
pub mod document_metadata { #[derive(Clone, Debug, PartialEq)] pub struct PdfDate; }
#[path = "classic-section-model.rs"] mod model;
pub use model::{ObjRef, PdfDictEntry, PdfObject, PdfDecimal};
pub mod standards {
    pub mod v1_7 {
        pub mod subsets {
            pub mod base {
                pub mod modules { pub mod lexer {pub use crate::lexer::*;} }
                pub mod schema { pub mod snapshot { pub use crate::model::*; } }
                pub mod io {
                    pub mod error {
                        #[derive(Debug)] pub enum PdfEngineError { Malformed(String) }
                        pub type PResult<T> = Result<T, PdfEngineError>;
                        pub fn malformed<T>(reason: impl Into<String>) -> PResult<T> { Err(PdfEngineError::Malformed(reason.into())) }
                    }
                    pub mod text_string { pub fn encode_text_string(text: &str) -> Vec<u8> { text.as_bytes().to_vec() } }
                    pub mod text { pub mod snapshot { pub mod date {
                        pub fn print_pdf_date(_: &crate::document_metadata::PdfDate) -> String { String::new() }
                    } } }
                }
            }
        }
    }
}
#[path = "'''+str((l/"🦀️.rs").resolve())+'''"] mod lexer;
pub use lexer::*;

#[path = "'''+(l.parents[1]/"🔨️modules/🔗️xref/📑️classic/🦀️.rs").resolve().as_posix()+'''"] mod classic;
#[path = "'''+(l.parents[1]/"🔨️modules/🔗️xref/📋️section/🦀️.rs").resolve().as_posix()+'''"] mod section;
#[path = "'''+(l.parents[1]/"🔨️modules/🔗️xref/🔎️footer/🦀️.rs").resolve().as_posix()+'''"] mod footer;
use section::{ClassicSectionCursor, ClassicSectionEvent};
fn startxref_offset(source:&[u8])->Option<usize>{ let mut cursor=footer::FooterCursor::new(source.len());loop{if let footer::FooterStep::Complete(offset)=cursor.advance(source,1){return offset}}}
#[test]
fn latest_classic_section_yields_free_entry_and_trailer_using_single_work_grants() {
    use crate::lexer::{CosGrant, CosStatus};
    let source = include_bytes!("'''+(l.parents[1]/"🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf").resolve().as_posix()+'''");
    let mut cursor = ClassicSectionCursor::new(startxref_offset(source).unwrap());
    let mut rows = Vec::new();
    let grant = CosGrant { work: 1, allocation_bytes: 8192, release_bytes: 8192 };
    loop {
        let turn = cursor.advance(source, grant, false);
        assert!(turn.work <= 1);
        match turn.event.unwrap() {
            ClassicSectionEvent::Row { number, live, .. } => rows.push((number, live)),
            ClassicSectionEvent::Progress(CosStatus::Complete { .. }) => break,
            ClassicSectionEvent::Progress(CosStatus::More) => {}
            _ => panic!("expected xref progress"),
        }
    }
    assert_eq!(rows, [(0, false), (3, false), (9, true)]);
    let trailer = PdfObject::Dict(cursor.take_trailer().unwrap());
    assert_eq!(trailer.dict_get("Info").and_then(PdfObject::as_ref), Some(ObjRef { num: 9, gen: 0 }));
}
''')
base = l.parents[1]
root = base / "🔨️modules/🔗️xref"
extra = r'''
pub use section::ClassicSectionReceipt;
pub use footer::{FooterCursor, FooterStep};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefEntry { Normal { offset: usize, gen: u16 }, Compressed { stream_num: u32, index: u32 } }
#[derive(Debug)]
pub enum XrefStreamError { Structure(&'static str), Decode(()) }
pub enum XrefStreamEvent {
    Row { number: u32, entry: Option<XrefEntry> }, Complete, More,
    NeedsAllocation { bytes: usize }, NeedsRelease { bytes: usize }, Cancelled, Closed,
}
pub struct XrefStreamReceipt {
    pub event: Result<XrefStreamEvent, XrefStreamError>, pub work: usize,
    pub allocated_bytes: usize, pub released_bytes: usize,
}
pub struct XrefStreamSectionCursor;
impl XrefStreamSectionCursor {
    pub fn new(_: usize, _: CosLimits, _: usize, _: usize) -> Self { Self }
    pub fn ready_trailer(&self) -> Option<&[PdfDictEntry]> { None }
    pub fn take_trailer(&mut self) -> Option<Vec<PdfDictEntry>> { None }
    pub fn advance(&mut self, _: &[u8], _: CosGrant, _: bool) -> XrefStreamReceipt {
        panic!("only classic xref sections are linked into this test")
    }
    pub fn recover_compatible(&mut self, _: &[u8]) -> standards::v1_7::subsets::base::io::error::PResult<bool> { Ok(false) }
    pub fn begin_close(&mut self) {}
    pub fn close_step(&mut self, _: CosGrant) -> XrefStreamReceipt {
        panic!("only classic xref sections are linked into this test")
    }
}
#[path = "@index@"] mod index;
pub use index::*;
#[path = "@revisions@"] mod revisions;
pub use revisions::*;
#[path = "@assembly@"] mod assembly;
use assembly::*;
#[path = "@offset@"] mod offset_validation;
pub use offset_validation::{XrefOffsetCursor, XrefOffsetStatus};
#[path = "@trailers@"] mod trailer_fold;
pub use trailer_fold::{StructuredTrailer, XrefTrailerCursor};
#[path = "@structured@"] mod structured_file;

#[test]
fn classic_revision_assembly_orders_trailers_and_releases_partial_stage() {
    let pdf = include_bytes!("@fixture@");
    let grant = CosGrant { work: 1, allocation_bytes: 1024 * 1024, release_bytes: 1024 * 1024 };
    for cancel in [false, true] {
        let mut cursor = XrefAssemblyCursor::new(startxref_offset(pdf), CosLimits::default(), 4096);
        let mut allocated = 0;
        let mut released = 0;
        let mut trailers = 0;
        let mut complete = false;
        for _ in 0..50_000 {
            let step = cursor.advance(pdf, grant, cancel && allocated > 0);
            assert!(step.work <= 1);
            allocated += step.allocated_bytes;
            released += step.released_bytes;
            match step.event.unwrap() {
                XrefAssemblyEvent::More => {}
                XrefAssemblyEvent::PrimaryTrailer => {
                    let entries = cursor.take_primary_trailer().unwrap();
                    if trailers == 0 {
                        let info = entries.iter().find(|entry| entry.key == "Info").unwrap();
                        assert_eq!(info.value.as_ref(), Some(ObjRef { num: 9, gen: 0 }));
                    }
                    trailers += 1;
                }
                XrefAssemblyEvent::Complete => {
                    assert!(!cancel);
                    let index = cursor.take_index().unwrap();
                    assert!(index.is_free(3));
                    assert!(index.contains_key(&9));
                    assert_eq!(trailers, 2);
                    complete = true;
                    break;
                }
                XrefAssemblyEvent::Cancelled => {
                    assert!(cancel);
                    assert_eq!(trailers, 0);
                    complete = true;
                    break;
                }
                status => panic!("unexpected xref assembly status: {status:?}"),
            }
        }
        assert!(complete);
        cursor.begin_close();
        loop {
            let step = cursor.close_step(grant);
            assert!(step.work <= 1);
            released += step.released_bytes;
            if step.event.unwrap() == XrefAssemblyEvent::Closed { break; }
        }
        if cancel { assert_eq!(allocated, released); }
    }
}
'''
for label, path in {
    "@index@": root / "💧️stream/📇️index/🦀️.rs",
    "@revisions@": root / "🧭️revisions/🦀️.rs",
    "@assembly@": root / "🧵️assembly/🦀️.rs",
    "@offset@": root / "🔎️offset-validation/🦀️.rs",
    "@trailers@": root / "📨️trailer-fold/🦀️.rs",
    "@structured@": root / "🛤️structured-file/🦀️.rs",
    "@fixture@": base / "🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf",
}.items():
    extra = extra.replace(label, path.resolve().as_posix())
h.write_text(h.read_text() + extra)
try:
    b=out/"classic-section-test"
    subprocess.run(["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc","--edition=2021","-Awarnings","--test",str(h),"-o",str(b)],check=True)
    subprocess.run([str(b), "--nocapture", "--skip", "assembled_revisions_publish_newest_rows", "--skip", "drives_mixed_revisions"], check=True)
finally:
    for file in (h,module,out/"classic-section-test"):
        file.unlink(missing_ok=True)
