"""🔧 Stage the clear PDF raw-plane writer and link a tiny probe to the real paged byte package."""
import os
from pathlib import Path
import runpy
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "png-placement"
CACHE = Path(sys.argv[1])
OUT.mkdir(parents=True, exist_ok=True)

writer_stage = runpy.run_path(str(TICKET / "paged-jpeg-writer-stage.py"))
writer_stage["stage"]()
writer_files = writer_stage["OUT"]

for name in ("📖️dct-view.rs", "🌊️dct-object.rs", "identity.rs", "lexer.rs", "filters.rs", "xref.rs", "byte-source.rs", "hash.rs"):
    (OUT / name).write_text((writer_files / name).read_text(encoding="utf8"), encoding="utf8")
writer = (writer_files / "writer.rs").read_text(encoding="utf8")
writer += '\n#[path = "../../paged-png-plane-writer.rs"]\npub mod plane_object;\n'
(OUT / "writer.rs").write_text(writer, encoding="utf8")
harness = (writer_files / "probe.rs").read_text(encoding="utf8")
end = harness.index("fn main()")
harness = harness[:end]
first = harness.index("pub mod paged {")
last = harness.index("\n}", first) + len("\n}")
harness = harness[:first] + """pub mod paged {
    pub use probe_paged_value::paged::{ByteReadSource, PAGED_BYTES_CHUNK_BYTES};
}""" + harness[last:]
harness = harness.replace("    pub use crate::dct_view;", "    pub use crate::{dct_view, plane_identity};")
harness += '\n#[path = "../../paged-png-placement-identity.rs"]\npub mod plane_identity;\n'
harness += (TICKET / "paged-png-plane-writer-probe.rs").read_text(encoding="utf8").split("\n", 1)[1]
entry = OUT / "probe.rs"
entry.write_text(harness, encoding="utf8")

value = next(CACHE.glob("semio-framework-value/*/out/libsemio_framework_value-*.rlib"))
dependencies = sorted({file.parent for file in CACHE.glob("*/*/out/*") if file.suffix in (".rmeta", ".rlib", ".so", ".dylib", ".dll")})
binary = OUT / ("plane-writer.exe" if os.name == "nt" else "plane-writer")
response = OUT / "writer.args"
arguments = [
    "--edition=2021", "-C", "opt-level=1", "-o", str(binary), str(entry),
    "--extern", f"probe_paged_value={value.with_suffix('.rmeta')}", "--extern", f"probe_paged_value={value}",
    *(f"-Ldependency={path}" for path in dependencies),
]
response.write_text("\n".join(arguments) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
subprocess.run([str(binary), str(GENERATED / "paged-png"), str(OUT)], check=True)
