"""🔧 Exercise production lexer, stream decoding and xref under the repository Rust toolchain."""
from pathlib import Path
import shutil,subprocess,sys,tempfile
l,t,f,cache,x=map(Path,sys.argv[1:6])
generated=t/"🗑️generated"
generated.mkdir(exist_ok=True)
scratch=tempfile.TemporaryDirectory(prefix="xref-section-",dir=generated)
out=Path(scratch.name)
runner=out/"flate-run-tests.py"
shutil.copyfile(t/"📜️flate-run-tests.py",runner)
source=(l.parents[1]/"🧬️schema/📸️snapshot/📐️model/🔗️object-model/🦀️.rs").read_text()
source="\n".join(line for line in source.splitlines() if not line.startswith(("#[canonical_json", "#[value")) and not line.lstrip().startswith("#[value"))
import re
source=re.sub(r'#\[value\([^\]]*\)\]\s*','',source)
for name in ("semio_framework_value::RetireOwned","semio_framework_value::CanonicalJsonTree","semio_framework_value::RetainedClone","value_derive::ToValue","value_derive::FromValue"):
    source=source.replace(name+", ","").replace(", "+name,"")
module=out/"stream-test-model.rs"
module.write_text(source)
h=out/"stream-test-harness.rs"
h.write_text('''
extern crate self as semio_s_artifact_stdio_pdf_model;
extern crate self as semio_s_artifact_stdio_pdf_snapshot_2;
pub mod document_metadata { #[derive(Clone, Debug, PartialEq)] pub struct PdfDate; }
#[path = "stream-test-model.rs"] mod model;
pub use model::{ObjRef, PdfDictEntry, PdfObject, PdfDecimal};
pub mod standards {
    pub mod v1_7 {
        pub mod subsets {
            pub mod base {
                pub mod schema { pub mod snapshot { pub use crate::model::*; } }
                pub mod io {
                    pub mod error {
                        #[derive(Debug)] pub enum PdfEngineError { Malformed(String) }
                        pub type PResult<T> = Result<T, PdfEngineError>;
                        pub fn malformed<T>(reason: impl Into<String>) -> PResult<T> { Err(PdfEngineError::Malformed(reason.into())) }
                    }
                    pub mod text_string { pub fn encode_text_string(text: &str) -> Vec<u8> { text.as_bytes().to_vec() } }
                    pub mod text { pub mod snapshot { pub use crate::retained_text; pub mod date {
                        pub fn print_pdf_date(_: &crate::document_metadata::PdfDate) -> String { String::new() }
                    } } }
                }
            }
        }
    }
}
#[path = "'''+str((l/"🦀️.rs").resolve())+'''"] mod lexer;
pub use lexer::{hex_val, is_delim, is_ws, parse_indirect_at, PdfByteStringCursor, StringStep};
''')

extra='''
extern crate self as semio_s_artifact_stdio_pdf_filters;
extern crate self as semio_s_artifact_stdio_deflate;
#[path = "{root}"] mod codec;
pub use codec::*;
#[path = "{filter}"] mod filters;
'''
raw=h.read_text()
raw=raw.replace("pub enum PdfEngineError { Malformed(String) }", "pub enum PdfEngineError { Malformed(String), Unsupported(String), NotPdf }")
raw=raw.replace("pub mod standards {", """
pub mod standards {
    pub mod v_rfc1950 { pub mod subsets { pub mod any { pub mod io {
        pub fn zlib_compress_deterministic(_: &[u8]) -> Result<Vec<u8>, ()> { unimplemented!() }
    } } } }
""")
raw=raw.replace("pub mod schema", """
                pub mod modules { pub use crate::lexer; }
                pub mod schema
""").replace('"] mod lexer;','"] pub mod lexer;')

raw=raw.replace("pub mod modules { pub use crate::lexer; }", """
                pub mod modules {
                    pub use crate::{filters, lexer, xref};
                    pub mod encryption {
                        pub struct Decryptor;
                        impl Decryptor { pub fn decrypt_object(&self, value: crate::PdfObject, _: u32, _: u16) -> crate::PdfObject { value } }
                    }
                }
""").replace("pub mod schema { pub mod snapshot { pub use crate::model::*; } }", """
                pub mod schema {
                    pub mod snapshot { pub use crate::model::*; }
                    pub mod graph_source {
                        pub trait ObjectSource { fn get(&mut self, key: crate::ObjRef) -> Option<crate::PdfObject>; }
                    }
                }
""")
extra=extra.replace("] mod filters;","] pub mod filters;")
probe=(x.parents[1]/"🧪️tests/🧫️fixtures/🔗️incremental-free/🌀️flate.pdf").resolve()
xref=x/"🦀️.rs"
packed=(x.parents[1]/"🧪️tests/🧫️fixtures/📦️object-stream/🧪️compressed.pdf").resolve()
page_tree=(x.parents[1]/"🔨️modules/⬇️lift/🌳️page-tree/🦀️.rs").resolve()
section_owner=(x/"💧️stream/📦️packages/🦀️rust/🦀️.rs").resolve()
extra=extra.format(root=(f/"📦️packages/🦀️rust/🦀️.rs").resolve(),filter=(f/"🦀️.rs").resolve())
extra+=f'''
extern crate self as semio_framework_artifact_reference;
extern crate self as semio_s_artifact_stdio_pdf_xref_stream;
extern crate self as semio_s_artifact_stdio_pdf_lexer;
extern crate self as semio_s_artifact_stdio_pdf_inferences;
pub use lexer::{{CosGrant, CosLimits, CosReceipt, CosStatus, PdfCosCursor, PdfHeader, PdfHeaderCursor, PdfHeaderStep, PdfNormalObjectCursor, PdfIndirectLengthCursor, NormalObject, NormalObjectStatus, PdfStreamDictionaryCursor, IndirectFrameCursor, IndirectFrameStatus, StreamSpan, StreamSpanCursor, HeaderStep, IndirectHeaderCursor, XrefLinkCursor, XrefPrimaryCycle, XrefRevisionSchedule, XrefSectionKind, XrefSectionProbe, XrefTarget}};
#[path = "{section_owner}"] pub mod xref_stream_owner;
#[path = "{page_tree}"] pub mod page_tree;
pub use xref_stream_owner::*;
#[path = "{(x.parents[1] / "🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs").resolve()}"] pub mod controlled_graph;
pub struct ArtifactRef;
#[path = "{xref.resolve()}"] pub mod xref;
#[test]
fn graph_import_inline_sizes() {{
    eprintln!("graph import bytes {{}}; file structure {{}}",
        std::mem::size_of::<controlled_graph::GraphImportCursor>(),
        std::mem::size_of::<xref::PdfFileStructureCursor>());
    eprintln!("graph import bytes normal {{}} member {{}} COS {{}} dictionary {{}} length {{}}",
        std::mem::size_of::<xref::NormalProjectionCursor>(),std::mem::size_of::<xref::ObjectStreamCursor>(),std::mem::size_of::<PdfCosCursor>(),std::mem::size_of::<PdfStreamDictionaryCursor>(),std::mem::size_of::<PdfIndirectLengthCursor>());
}}
#[test]
fn production_xref_imports_flate_incremental_revision() {{
    let pdf = include_bytes!("{probe}");
    let index = xref::build_xref(pdf).unwrap();
    assert!(!index.brute_forced);
    assert!(!index.entries.contains_key(&3));
    assert!(index.entries.contains_key(&9));
    let mut resolver = xref::Resolver::new(pdf, index.entries);
    assert!(resolver.resolve(3).is_none());
    assert!(resolver.resolve(9).is_some());
}}
'''
extra+=f''' 
#[test]
fn production_resolver_reads_members_of_a_flate_object_stream() {{
    let pdf = include_bytes!("{packed}");
    let index = xref::build_xref(pdf).unwrap();
    assert!(!index.brute_forced);
    let mut resolver = xref::Resolver::new(pdf, index.entries);
    let info = resolver.resolve(6).unwrap();
    assert_eq!(info.dict_get("Title").unwrap().as_str_bytes(), Some(b"Compressed".as_slice()));
    let value = resolver.resolve(7).unwrap();
    assert_eq!(value.as_array().unwrap()[3], PdfObject::Ref(ObjRef {{ num: 6, gen: 0 }}));
    resolver.xref.insert_row(XrefRow {{ number: 77, entry: Some(XrefEntry::Compressed {{ stream_num: 4, index: 1 }}) }}, false).unwrap();
    assert!(resolver.resolve(77).is_none());
}}
'''
def import_model(path, destination):
    body=path.read_text()
    body="\n".join(line for line in body.splitlines() if not line.lstrip().startswith(("#[canonical_json", "#[value")))
    body=re.sub(r'#\[value\([^\]]*\)\]\s*','',body)
    derives=("semio_framework_value::RetireOwned","semio_framework_value::CanonicalJsonTree","semio_framework_value::RetainedClone","value_derive::ToValue","value_derive::FromValue")
    for derive in derives:
        body=re.sub(r',?\s*'+re.escape(derive)+r'(?:\s*,)?',lambda match: ',' if match.group(0).startswith(',') and match.group(0).endswith(',') else '',body)
    (out/destination).write_text(body)

base=x.parents[1]
models=base/"🧬️schema/📸️snapshot/📐️model"
date_model=(models/"📄️document-metadata/🦀️.rs").read_text()
date_shape="pub struct PdfDate {"+date_model.split("pub struct PdfDate {",1)[1].split("\n}",1)[0]+"\n}"
date_shape="\n".join(line for line in date_shape.splitlines() if not line.lstrip().startswith("#[value"))
raw=raw.replace("pub struct PdfDate;",date_shape).replace("pub use crate::model::*;","pub use crate::model::*; pub use crate::document_metadata::PdfDate;",1)
text_codec=(base/"🚪️io/🔤️text-string/🦀️.rs").resolve()
role_dir=base/"🚪️io/📝️text/📸️snapshot"
raw=raw.replace('pub mod text_string { pub fn encode_text_string(text: &str) -> Vec<u8> { text.as_bytes().to_vec() } }',f'#[path = "{text_codec}"] pub mod text_string;')
extra+=f'\n#[path = "{(role_dir/"📅️date/🦀️.rs").resolve()}"] pub mod date;\n'
extra+=f'#[path = "{(role_dir/"🪪️retained-text/🦀️.rs").resolve()}"] pub mod retained_text;\n'

import_model(models/"🖋️content-operators/🦀️.rs","content-operators.rs")
import_model(models/"🖼️xobjects/🦀️.rs","content-xobjects.rs")
import_model(models/"🔤️font-model/🦀️.rs","font-model.rs")
font_modules=base/"🔨️modules/🔤️fonts"
font_codec=(font_modules/"🦀️.rs").read_text().rsplit("#[cfg(test)]", 1)[0]
cmap=(font_modules/"🗺️cmap/🦀️.rs").resolve().as_posix()
(out/"font-codec.rs").write_text(font_codec.replace("🗺️cmap/🦀️.rs", cmap))
image_source=(base/"🔨️modules/🖼️images/🦀️.rs").read_text()
geometry=image_source[image_source.index("pub fn packed_image_geometry"):]
(out/"content-images.rs").write_text("use crate::standards::v1_7::subsets::base::io::error::{PResult,PdfEngineError};\n"+geometry)
content=(base/"🔨️modules/🖋️content/🦀️.rs").resolve()
content_contract=(base/"🔨️modules/🖋️content/🧪️tests/🔬️unit/🦀️.rs").resolve()
page_content=(base/"🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content/🦀️.rs").resolve()
resources=(base/"🔨️modules/⬇️lift/🪟️resources-and-fonts/🦀️.rs").read_text()
resource_entries=resources[resources.index("fn resource_entries"):resources.index("//#region 🔖️Resources")]
(out/"resource-entry-tests.rs").write_text("use crate::{ObjRef, PdfDictEntry, PdfObject};\nuse crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;\n"+resource_entries)
raw=raw.replace("pub use crate::{filters, lexer, xref};","pub use crate::{colour, content, filters, fonts, images, lexer, xref}; pub use crate::page_content as lift;")
raw=raw.replace("pub use crate::model::*;","pub use crate::model::*;pub use crate::content_operators::*;pub use crate::xobjects::*;pub use crate::font_model::*;",1)
raw=raw.replace("                pub mod io {", "                pub mod io {\n                    pub use crate::foreign_artifacts;")
extra=extra.replace("pub struct ArtifactRef;", "#[derive(Clone,Debug,PartialEq,Eq)] pub struct ArtifactRef;")
extra+='''
pub mod object_model {pub use crate::model::*;}
#[path = "content-operators.rs"] pub mod content_operators;
#[path = "font-model.rs"] pub mod font_model;
#[path = "font-codec.rs"] pub mod actual_fonts;
#[path = "@METRICS@"] pub mod metrics;
#[path = "@TABLES@"] pub mod tables;
#[path = "@GLYPH_LIST@"] pub mod glyph_list;
#[path = "@TRUETYPE@"] pub mod truetype;
extern crate self as semio_s_artifact_stdio_pdf_tables;
extern crate self as semio_s_artifact_stdio_pdf_glyph_list;
#[path = "content-xobjects.rs"] pub mod xobjects;
#[path = "content-images.rs"] pub mod images;
#[path = "@CONTENT@"] pub mod content;
#[path = "@CONTENT_CONTRACT@"] mod content_contract;
#[path = "@PAGE_CONTENT@"] pub mod page_content;
#[path = "@PAGE_CONTRACT@"] mod page_content_contracts;
#[path = "resource-entry-tests.rs"] mod resource_entries;
pub mod colour_and_graphics_state {
    #[derive(Clone,Debug,PartialEq)]
    pub enum PdfColorSpace { DeviceGray,DeviceRgb,DeviceCmyk }
    impl PdfColorSpace { pub fn components(&self)->Option<u32>{Some(match self{Self::DeviceGray=>1,Self::DeviceRgb=>3,Self::DeviceCmyk=>4})} }
}
pub mod colour {
    use crate::{colour_and_graphics_state::PdfColorSpace,PdfObject};
    use crate::standards::v1_7::subsets::base::io::error::PResult;
    pub fn lift_colour_space_inline(value:&PdfObject)->PdfColorSpace {
        match value.as_name().unwrap() { "G"|"DeviceGray"=>PdfColorSpace::DeviceGray, "RGB"|"DeviceRGB"=>PdfColorSpace::DeviceRgb, "CMYK"|"DeviceCMYK"=>PdfColorSpace::DeviceCmyk, unexpected=>panic!("fixture unexpectedly used {unexpected}") }
    }
    pub fn lower_colour_space_inline(value:&PdfColorSpace)->PResult<PdfObject>{
        Ok(PdfObject::name(match value{PdfColorSpace::DeviceGray=>"G",PdfColorSpace::DeviceRgb=>"RGB",PdfColorSpace::DeviceCmyk=>"CMYK"}))
    }
}
pub mod fonts {
    pub use crate::{metrics, tables, glyph_list, truetype};
    pub use crate::actual_fonts::*;
}
pub mod foreign_artifacts {
    use crate::{ArtifactRef,PdfObject};
    use crate::standards::v1_7::subsets::base::io::error::PResult;
    pub trait PdfArtifactResourcePort {
        fn admit(&mut self,kind:&str,value:PdfObject)->PResult<ArtifactRef>;
        fn resolve(&self,_:&ArtifactRef)->PResult<PdfObject>{unreachable!("fixture does not contain a foreign artifact")}
    }
    pub fn require_font_artifact_kind(_: &crate::font_model::PdfFontProgram) -> PResult<()> { unreachable!() }
    pub struct NativePdfArtifactResources;
    impl NativePdfArtifactResources {pub fn from_objects(_: &[()])->Self {Self}}
    impl PdfArtifactResourcePort for NativePdfArtifactResources {fn admit(&mut self,_:&str,_:PdfObject)->PResult<ArtifactRef>{unreachable!("fixture does not contain a foreign artifact")}}
}
impl std::fmt::Display for standards::v1_7::subsets::base::io::error::PdfEngineError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {match self{Self::Malformed(message)|Self::Unsupported(message)=>f.write_str(message),Self::NotPdf=>f.write_str("not a PDF")}}
}

#[test]
fn production_content_lexical_cursor_preserves_graphics_and_text_operator_roundtrip() {
    use crate::{content::{parse_content,print_content,NoFonts},content_operators::{PdfOp,PdfTextString},foreign_artifacts::NativePdfArtifactResources};
    let mut resources=NativePdfArtifactResources::from_objects(&[]);
    let input=b"q 1.25 w 20 30 m 70 90 l S BT /F1 14 Tf 1 0 0 1 30 100 Tm (Hello PDF) Tj ET Q";
    let ops=parse_content(input,&NoFonts,&mut resources).unwrap();
    assert!(matches!(&ops[1],PdfOp::SetLineWidth{width} if *width==1.25));
    assert!(ops.iter().any(|op|matches!(op,PdfOp::ShowText{text:PdfTextString::Text{text}} if text=="Hello PDF")));
    let bytes=print_content(&ops,&NoFonts).unwrap();
    assert_eq!(parse_content(&bytes,&NoFonts,&mut resources).unwrap(),ops);
}
'''.replace("@CONTENT@",content.as_posix()).replace("@CONTENT_CONTRACT@",content_contract.as_posix()).replace("@PAGE_CONTENT@",page_content.as_posix()).replace("@PAGE_CONTRACT@",(page_content.parent/"🧪️tests/🔬️unit/🦀️.rs").as_posix())
for placeholder, domain in (("METRICS","📏️metrics"),("TABLES","🅰️tables"),("GLYPH_LIST","🔡️glyph-list"),("TRUETYPE","🔠️truetype")):
    extra=extra.replace(f"@{placeholder}@", (font_modules/domain/"🦀️.rs").resolve().as_posix())

if len(sys.argv) > 6:
    large_pdf=Path(sys.argv[6]).resolve().as_posix()
    extra+='''
#[test]
fn large_file_uses_retained_graph_and_real_page_program_parser() {
    use crate::{content::{parse_content,NoFonts},content_operators::{PdfOp,PdfTextString},foreign_artifacts::NativePdfArtifactResources};
    let pdf=include_bytes!("@LARGE_PDF@");
    use crate::controlled_graph::{PdfByteGraphCursor, PdfByteGraphEvent};
    let mut original = PdfByteGraphCursor::new(196, 1_000_000);
    let turn = CosGrant { work: 1, allocation_bytes: 32 * 1024, release_bytes: 32 * 1024 };
    let mut objects = 0;
    let opened = (0..500_000).find_map(|_| {
        let step = original.advance(pdf, turn, false);
        assert!(step.work <= 1);
        match step.event.unwrap() {
            PdfByteGraphEvent::More => None,
            PdfByteGraphEvent::Object => { objects += 1; None }
            PdfByteGraphEvent::Complete => {
                assert!(original.take(CosGrant { release_bytes: 0, ..turn }).is_none());
                Some(original.take(turn).expect("paid graph ownership transfer").0)
            }
            event => panic!("unexpected large PDF byte graph state {event:?}"),
        }
    }).expect("large PDF graph completion");
    assert_eq!((opened.header.version(), opened.origin, objects), ("1.7", 0, 196));
    let root = opened.trailer.get_inheritable("Root").unwrap().as_ref().unwrap();
    let retained = opened.graph;
    assert_eq!(retained.objects.len(), 196);
    let state = xref::read_file_xref(pdf).unwrap().2;
    let mut resolver = xref::Resolver::new(pdf, state.entries);
    let catalog=resolver.resolve_reference(root).unwrap();
    let mut pages=page_tree::PageTreeCursor::new(catalog.dict_get("Pages").unwrap().as_ref().unwrap());
    let grant=CosGrant{work:1,allocation_bytes:128*1024,release_bytes:128*1024};
    let mut contents=Vec::new();
    for _ in 0..100_000 {
        let turn=pages.advance(&mut resolver,grant,false);
        assert!(turn.work<=1);
        match turn.event.unwrap() {
            page_tree::PageTreeEvent::Page(leaf)=>contents.push(leaf.reference),
            page_tree::PageTreeEvent::Complete=>break,
            page_tree::PageTreeEvent::More|page_tree::PageTreeEvent::BranchResources(_)=>{},
            other=>panic!("unexpected large PDF page tree state {other:?}"),
        }
    }
    assert_eq!(contents.len(),48);
    let leaf=contents.pop().unwrap();
    let page=retained.objects.iter().find(|object|object.id==leaf).unwrap();
    let mut resources=NativePdfArtifactResources::from_objects(&[]);
    let ops=page_content::project_content_bytes(&mut resolver,page.value.dict_get("Contents"),|data|parse_content(data,&NoFonts,&mut resources)).unwrap();
    assert_eq!(ops.iter().filter(|op|matches!(op,PdfOp::LineTo{..})).count(),48);
    assert_eq!(crate::content::extract_text(&ops,&NoFonts).trim(),"Page 48 of 48");
    pages.begin_close();
    while pages.close_step(grant).event.unwrap()!=page_tree::PageTreeEvent::Closed {}
}
'''.replace("@LARGE_PDF@",large_pdf)

testkit = (l.parents[1]/"🧬️schema/🧪️testkit/📦️packages/🦀️rust/🦀️.rs").read_text()
index_contract = testkit.index("#[cfg(test)]\nmod xref_index_contracts {")
index_contract_end = testkit.index("\n}\n", index_contract) + 3
h.write_text(raw+extra+testkit[index_contract:index_contract_end])

try:
    h.write_text(re.sub(r"pub mod schema\s*\{", "pub mod schema { pub mod graph_source { pub trait ObjectSource { fn get(&mut self, key: crate::ObjRef) -> Option<crate::PdfObject>; fn inspect(&mut self, key: crate::ObjRef, reader: &mut dyn FnMut(Option<&crate::PdfObject>)) { let value=self.get(key); reader(value.as_ref()); } } }", h.read_text(), count=1))
    subprocess.run(["repo","tools","exec","python","--",str(runner),str(cache),str(h)],check=True)
finally:
    for file in (h,module,out/"flate-tests",out/"flate-tests.exe",out/"flate-test-args"):
        file.unlink(missing_ok=True)
