"""🔧 Exercise production lexer, stream decoding and xref under the repository Rust toolchain."""
from pathlib import Path
import subprocess,sys
l,t,f,cache,x=map(Path,sys.argv[1:6]); out=t/"🗑️generated"
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
                    pub mod text { pub mod snapshot { pub mod date {
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
raw=raw.replace("pub enum PdfEngineError { Malformed(String) }", "pub enum PdfEngineError { Malformed(String), Unsupported(String) }")
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
extra=extra.format(root=(f/"📦️packages/🦀️rust/🦀️.rs").resolve(),filter=(f/"🦀️.rs").resolve())
extra+=f'''
extern crate self as semio_framework_artifact_reference;
pub struct ArtifactRef;
#[path = "{xref.resolve()}"] pub mod xref;
#[test]
fn production_xref_imports_flate_incremental_revision() {{
    let pdf = include_bytes!("{probe}");
    let index = xref::build_xref(pdf, xref::startxref_offset(pdf));
    assert!(!index.brute_forced);
    assert!(!index.entries.contains_key(&3));
    assert!(index.entries.contains_key(&9));
    let mut resolver = xref::Resolver::new(pdf, index.entries);
    assert!(resolver.resolve(3).is_none());
    assert!(resolver.resolve(9).is_some());
}}
'''
h.write_text(raw+extra)

try:
    h.write_text(re.sub(r"pub mod schema\s*\{", "pub mod schema { pub mod graph_source { pub trait ObjectSource { fn get(&mut self, key: crate::ObjRef) -> Option<crate::PdfObject>; } }", h.read_text(), count=1))
    subprocess.run(["repo","tools","exec","python","--",str(t/"📜️flate-run-tests.py"),str(cache),str(h)],check=True)
finally:
    for file in (h,module,out/"flate-tests",out/"flate-tests.exe",out/"flate-test-args"):
        file.unlink(missing_ok=True)
