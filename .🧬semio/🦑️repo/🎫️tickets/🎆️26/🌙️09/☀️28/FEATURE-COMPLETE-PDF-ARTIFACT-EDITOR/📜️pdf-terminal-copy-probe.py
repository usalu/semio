"""📏 Measures staged terminal publication with cached WASI Rust type metadata."""

from pathlib import Path
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
base = Path(sys.argv[1])
out = ticket / "🗑️generated" / "terminal-copy"
cache = Path(".cache/b/r/m/u/l/p32ef76/wasm32-wasip2/prototype/build")
files = sorted(cache.glob("*/*/out/*.rmeta"))
host = cache.parents[2] / "prototype" / "build"
host_paths = [path.parent for pattern in ("*/*/out/*.dylib", "*/*/out/*.so") for path in host.glob(pattern)]
crates = ["semio_s_artifact_stdio_pdf_snapshot_2", "semio_s_artifact_stdio_pdf_xref_stream"]
externs = []
for name in crates:
    found = [path for path in files if path.name.startswith(f"lib{name}-")]
    assert len(found) == 1, (name, found)
    externs += ["--extern", f"{name}={found[0]}"]
    print(f"[DEBUG] Using repository WASI metadata {found[0]}", flush=True)


def block(text, marker):
    start = text.index(marker)
    end = text.index("\n}", start) + 2
    return text[start:end]


graph = (base / "🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs").read_text()
header = (base / "🔨️modules/🔤️lexer/🪪️pdf-header/🦀️.rs").read_text()
staged = (out / "controlled.rs").read_text()
copy_body = staged[staged.index("    fn copy_body("):staged.index("    fn reserves_empty_output(")]
check = re.search(r"if grant.maximum_items == 0 .*? \{ break; \}", staged).group()
guard = check.replace("{ break; }", "{ return false; }")
header_len = re.search(r"const MAXIMUM_VERSION_BYTES[^;]*;", header).group()
report = (out / "layout.rs")
report.write_text(
    "use std::mem::size_of;\n"
    "use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, PdfDictEntry, PdfIndirectObject};\n"
    "use semio_s_artifact_stdio_pdf_xref_stream::XrefIndex;\n"
    + "\n".join((header_len, block(header, "pub struct PdfHeader {"), block(graph, "pub struct ImportedGraph {"), block(graph, "pub struct PreparedPdfFile {")))
    + "\n" + re.search(r"const COPY_PER_WORK[^;]*;", staged).group() + "\n"
    + "enum Phase { TransferFile, TransferLift, Publish, Done }\nstruct NaturalPdfDecoder { phase: Phase }\nimpl NaturalPdfDecoder {\n"
    + copy_body + "}\n"
    + "struct Grant { maximum_items: usize, maximum_depth: usize, maximum_copy_bytes: usize }\n"
    + f"fn admitted(maximum_copy_bytes: usize, copy_body: usize) -> bool {{ let grant = Grant {{ maximum_items: 1, maximum_depth: 1, maximum_copy_bytes }}; {guard} true }}\n"
    + """
#[no_mangle] pub extern "C" fn snapshot_bytes() -> usize { size_of::<PdfSnapshot>() }
#[no_mangle] pub extern "C" fn prepared_bytes() -> usize { size_of::<PreparedPdfFile>() }
#[no_mangle] pub extern "C" fn terminal_bytes() -> usize { NaturalPdfDecoder { phase: Phase::Publish }.copy_body() }
#[no_mangle] pub extern "C" fn terminal_admits(grant: usize) -> bool { admitted(grant, terminal_bytes()) }
#[no_mangle] pub extern "C" fn terminal_boundaries() -> u32 {
    let quote = terminal_bytes();
    u32::from(!admitted(quote - 1, quote)) | (u32::from(admitted(quote, quote)) << 1) | (u32::from(admitted(32_768, quote)) << 2)
}
""",
    encoding="utf8",
)

# The cached artifacts only supply type metadata. LLVM folds the quotes and boundary predicate.
args = out / "rustc.args"
target = out / "layout.ll"
search = [f"-Ldependency={path}" for path in sorted({path.parent for path in files}.union(host_paths))]
args.write_text("\n".join(["--edition=2021", "--target=wasm32-wasip2", "--crate-type=lib", "-Cpanic=abort", "--emit=llvm-ir", "-Awarnings", "-O", "-o", str(target), str(report), *externs, *search]) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(args)], check=True)
ir = target.read_text()
for symbol in ("snapshot_bytes", "prepared_bytes", "terminal_bytes", "terminal_boundaries"):
    match = re.search(rf"define [^\n]*@{symbol}\([^{{]*\{{[^}}]*ret i32 (\d+)", ir)
    assert match, symbol
    value = int(match[1])
    print(f"[DEBUG] WASI {symbol}={value}")
    if symbol == "terminal_boundaries":
        assert value == 7
