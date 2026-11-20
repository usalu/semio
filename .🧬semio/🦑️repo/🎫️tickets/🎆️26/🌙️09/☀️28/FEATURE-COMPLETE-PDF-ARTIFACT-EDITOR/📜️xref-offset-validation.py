"""Exercise exact production PDF xref-offset roles without a whole-workspace Cargo lease."""
import json
from pathlib import Path
import subprocess
import sys

root = Path(sys.argv[1]).resolve()
output = Path(sys.argv[2]).resolve()
output.mkdir(parents=True, exist_ok=True)
source = output / "xref-offset-validation.rs"
binary = output / ("xref-offset-validation.exe" if sys.platform == "win32" else "xref-offset-validation")
quote = lambda path: json.dumps(str(path), ensure_ascii=False)
header = quote(root / "🔨️modules/🔤️lexer/🪪️indirect-header/🦀️.rs")
index = quote(root / "🔨️modules/🔗️xref/💧️stream/📇️index/🦀️.rs")
validation = quote(root / "🔨️modules/🔗️xref/🔎️offset-validation/🦀️.rs")
pdf = quote(root / "🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf")
source.write_text(f"""
extern crate self as semio_s_artifact_stdio_pdf_snapshot_2;
extern crate self as semio_s_artifact_stdio_pdf_lexer;

pub mod standards {{ pub mod v1_7 {{ pub mod subsets {{ pub mod base {{ pub mod schema {{ pub mod snapshot {{
    pub use crate::ObjRef;
}} }} }} }} }} }}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjRef {{ pub num: u32, pub gen: u16 }}
#[derive(Clone, Copy)]
pub struct CosGrant {{ pub work: usize, pub allocation_bytes: usize, pub release_bytes: usize }}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefEntry {{ Normal {{ offset: usize, gen: u16 }}, Compressed {{ stream_num: u32, index: u32 }} }}
pub fn is_ws(byte: u8) -> bool {{ matches!(byte, b' ' | b'\\t' | b'\\r' | b'\\n' | 0x0c | 0) }}
pub fn is_delim(byte: u8) -> bool {{ b"()<>[]{{}}/%".contains(&byte) }}

#[path={header}] mod header;
pub use header::*;
#[path={index}] mod index;
pub use index::*;
#[path={validation}] mod validation;
use validation::*;

#[test]
fn actual_rows_validate_and_wrong_generation_is_refused() {{
    let source = include_bytes!({pdf});
    let mut entries = XrefIndex::default();
    for number in [1, 2, 4, 9] {{
        let declaration = format!("{{number}} 0 obj");
        let offset = source.windows(declaration.len()).position(|part| part == declaration.as_bytes()).unwrap();
        entries.insert_row(XrefRow {{ number, entry: Some(XrefEntry::Normal {{ offset, gen: 0 }}) }}, false).unwrap();
    }}
    entries.insert_row(XrefRow {{ number: 3, entry: None }}, false).unwrap();
    let offset = match entries.get(&9).unwrap() {{ XrefEntry::Normal {{ offset, .. }} => *offset, _ => unreachable!() }};
    for expected in [XrefOffsetStatus::Complete, XrefOffsetStatus::Invalid {{ id: ObjRef {{ num: 9, gen: 5 }}, offset }}] {{
        if expected != XrefOffsetStatus::Complete {{
            entries.insert_row(XrefRow {{ number: 9, entry: Some(XrefEntry::Normal {{ offset, gen: 5 }}) }}, true).unwrap();
        }}
        let mut cursor = XrefOffsetCursor::default();
        let actual = loop {{
            let (work, status) = cursor.advance(source, &entries, 1, false);
            assert!(work <= 1);
            if status != XrefOffsetStatus::More {{ break status; }}
        }};
        assert_eq!(actual, expected);
        assert_eq!(cursor.advance(source, &entries, 1, false), (0, expected));
    }}
}}
""", encoding="utf-8")
try:
    subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                    "--edition=2021", "-Awarnings", "--test", str(source), "-o", str(binary)], check=True)
    subprocess.run([str(binary), "actual_rows_validate_and_wrong_generation_is_refused"], check=True)
finally:
    source.unlink(missing_ok=True)
    binary.unlink(missing_ok=True)
