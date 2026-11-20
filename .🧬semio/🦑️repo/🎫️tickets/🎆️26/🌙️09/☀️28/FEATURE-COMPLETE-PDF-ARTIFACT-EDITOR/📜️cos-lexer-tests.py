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
module=out/"cos-object-model.rs"
module.write_text(source)
h=out/"cos-lexer-harness.rs"
h.write_text('''
extern crate self as semio_s_artifact_stdio_pdf_model;
extern crate self as semio_s_artifact_stdio_pdf_snapshot_2;
pub mod document_metadata { #[derive(Clone, Debug, PartialEq)] pub struct PdfDate; }
#[path = "cos-object-model.rs"] mod model;
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
try:
    b=out/"cos-lexer-test"
    subprocess.run(["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc","--edition=2021","-Awarnings","--test",str(h),"-o",str(b)],check=True)
    subprocess.run([str(b),"--nocapture"],check=True)
finally:
    for file in (h,module,out/"cos-lexer-test"):
        file.unlink(missing_ok=True)
