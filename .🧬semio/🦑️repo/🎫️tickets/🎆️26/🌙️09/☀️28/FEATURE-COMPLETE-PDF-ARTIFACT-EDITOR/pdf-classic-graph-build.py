"""🔧 Exercises the classic graph quote and writer against the real first-party lexer and writer rules."""
import os
from pathlib import Path
import runpy
import subprocess

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "classic-graph"
OUT.mkdir(parents=True, exist_ok=True)
stage = runpy.run_path(str(TICKET / "paged-jpeg-writer-stage.py"))
stage["stage"]()
previous = stage["OUT"]
for name in ("📖️dct-view.rs", "🌊️dct-object.rs", "identity.rs", "lexer.rs", "filters.rs", "xref.rs", "byte-source.rs", "hash.rs"):
    (OUT / name).write_text((previous / name).read_text(encoding="utf8"), encoding="utf8")
writer = (previous / "writer.rs").read_text(encoding="utf8")
writer += '\n#[path = "../../pdf-classic-graph-quote.rs"]\npub mod classic_graph_quote;\n'
writer += '\n#[path = "../../pdf-classic-graph-writer.rs"]\npub mod classic_graph_writer;\n'
writer += '\n#[path = "../../pdf-classic-stored-flate.rs"]\npub mod classic_stored_flate;\n'
(OUT / "writer.rs").write_text(writer, encoding="utf8")
harness = (previous / "probe.rs").read_text(encoding="utf8").split("fn main()", 1)[0]
base = stage["BASE"]
model = (base / "🧬️schema/📸️snapshot/📐️model/🔗️object-model/🦀️.rs").read_text(encoding="utf8")
decimal = stage["span"](model, "pub struct PdfDecimal", "/// 🎯 A parsed PDF object")
a = harness.index("#[derive(Clone, Copy, Debug, PartialEq)]\npub struct PdfDecimal")
b = harness.index("#[derive(Clone, Debug, PartialEq, Eq)]\npub struct PdfEncryption", a)
harness = harness[:a] + "#[derive(Clone, Debug, PartialEq)]\n" + decimal + harness[b:]
harness = harness.replace("pub use crate::{writer, filters, lexer, xref, encryption};", "pub use crate::{writer, filters, lexer, xref, encryption, lower};")
harness = harness.replace("pub mod lexer {", "pub mod lexer {\n    #[derive(Clone, Copy)] pub struct CosGrant { pub work: usize, pub allocation_bytes: usize, pub release_bytes: usize }")
lower = (base / "🔨️modules/⬆️lower/📚️catalog-and-info/🦀️.rs").read_text(encoding="utf8")
info = stage["span"](lower, "pub fn lower_info(", "// 🚫️async:")
info = info.replace("pub fn lower_info(info: &PdfInfo)", "pub fn lower_info_with(info: &PdfInfo, encode_text: impl Fn(&str) -> Vec<u8>)").replace("encode_text_string(v)", "encode_text(v)")
(OUT / "lower-info.rs").write_text(info, encoding="utf8")
harness += "\n#[path = \"../../pdf-paid-ordinary-graft.rs\"]\nmod ordinary_graft;\n\n#[derive(Default)]\npub struct PdfSnapshot { pub pages: Vec<PdfPage>, pub info: PdfInfo }\npub struct PdfPage { pub rotate: i32 }\n#[derive(Default)]\npub struct PdfInfo {\n    pub title: Option<String>, pub author: Option<String>, pub subject: Option<String>, pub keywords: Option<String>,\n    pub creator: Option<String>, pub producer: Option<String>, pub trapped: Option<String>,\n    pub creation_date: Option<String>, pub modification_date: Option<String>, pub extra: Vec<PdfDictEntry>,\n}\npub mod lower {\n    use super::*;\n    pub fn push_opt(entries: &mut Vec<PdfDictEntry>, name: &str, value: Option<PdfObject>) {\n        if let Some(value) = value { entries.push(PdfDictEntry::new(name, value)); }\n    }\n    include!(\"lower-info.rs\");\n}\n"
harness += (TICKET / "pdf-classic-graph-probe.rs").read_text(encoding="utf8").split("\n", 1)[1]
entry = OUT / "probe.rs"
entry.write_text(harness, encoding="utf8")
binary = OUT / ("classic-graph.exe" if os.name == "nt" else "classic-graph")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                "--edition=2021", "-C", "opt-level=1", str(entry), "-o", str(binary)], check=True)
subprocess.run([str(binary), str(OUT)], check=True)
