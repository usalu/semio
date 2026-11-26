"""📄 Build the drawing exporter's PDF reference tests without enabling unrelated plugin test facilities."""
from pathlib import Path
import json
import os
import subprocess

ticket = Path(__file__).resolve().parent
repo = ticket.parents[6]
owner = repo / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
sources = json.loads(subprocess.check_output(["repo", "tools", "exec", "--light", "bun", "--", "-e", "console.log(JSON.stringify(Bun.TOML.parse(await Bun.file(process.argv[1]).text()).workspace.dependencies))", str(owner / "Cargo.toml")], text=True))
names = ["semio-framework-async", "semio-framework-async-macros", "semio-plugin",
         "semio-s-artifact-stdio-semio-base", "semio-s-artifact-stdio-semio-drawing",
         "semio-s-artifact-stdio-semio-drawing-export", "semio-s-artifact-stdio-semio-drawing-png-export",
         "semio-s-artifact-stdio-pdf", "semio-s-artifact-stdio-pdf-lower"]
crate = ticket / "🗑️generated" / "pdfref"
crate.mkdir(parents=True, exist_ok=True)
manifest = ['[package]', 'name = "demonstrator-drawing-reference"', 'version = "0.1.0"', 'edition = "2024"',
            '[workspace]', 'resolver = "3"', '[lib]', 'path = "lib.rs"', '[dev-dependencies]']
for name in names:
    dep = (owner / sources[name]["path"]).resolve()
    manifest.append(f'{name} = {{ path = {json.dumps(os.path.relpath(dep, crate).replace(os.sep, "/"), ensure_ascii=False)} }}')
hayro = sources["hayro"]
manifest.append('hayro = { version = "=0.4.0"' + (', default-features = false' if hayro.get("default-features") is False else '') + ' }')
(crate / "Cargo.toml").write_text("\n".join(manifest) + "\n")
lock = crate / "Cargo.lock"
if not lock.exists():
    lock.write_bytes((owner / "Cargo.lock").read_bytes())
tests = next(owner.glob("**/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/*/*/🧪️tests/🔬️unit/🦀️.rs"))
path = os.path.relpath(tests, crate).replace(os.sep, "/")
(crate / "lib.rs").write_text('''#![cfg(test)]
pub use semio_s_artifact_stdio_semio_drawing_export::{encode_drawing, SemioDrawingFormat};
use semio_s_artifact_stdio_semio_drawing_export::serializers::artifacts::pdf::v1_7::any::{drawing_to_pdf, SemioDrawingToPdf};
use semio_s_artifact_stdio_pdf_lower::standards::v1_7::subsets::base::schema::snapshot::PdfOp;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use semio_plugin::ArtifactSerializer;
''' + '#[path = ' + json.dumps(path, ensure_ascii=False) + ']\nmod reference;\n')
print(crate / "Cargo.toml")
