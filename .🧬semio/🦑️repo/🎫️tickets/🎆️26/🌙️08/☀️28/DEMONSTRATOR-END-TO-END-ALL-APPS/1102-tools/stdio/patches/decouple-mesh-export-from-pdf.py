#!/usr/bin/env python3
# usage: python3 -I decouple-mesh-export-from-pdf.py [--apply]   (dry-run by default; fail-closed)
# Cuts stdio-semio-mesh-export -> stdio-semio-drawing-export -> stdio-pdf by extracting the drawing->png
# rasterizer (the ONLY thing mesh export uses from drawing export) into its own crate
# `semio-s-artifact-stdio-semio-drawing-png-export` (lib path = the existing component file; nothing is moved but its test).
import sys, re, pathlib
APPLY = "--apply" in sys.argv
ROOT = next((p for p in pathlib.Path(__file__).resolve().parents if (p / "✏️s" / "🔌️plugins").is_dir()), None)
if ROOT is None: sys.exit("FAIL-CLOSED repo root (a parent holding ✏️s/🔌️plugins) not found above the script")
R = ROOT / "✏️s" / "🔌️plugins"
S = R / "🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
DX = S / "🖊️drawing/🚪️io/📤️export"
PNG = DX / "🧵️serializers/🗿️artifacts/📷️png"
LEAF = PNG / "🔖️1.2/✳️any"
MX = S / "🔺️mesh/🚪️io/📤️export"
NAME = "semio-s-artifact-stdio-semio-drawing-png-export"
MOD = NAME.replace("-", "_")
edits = []
def sub(path, old, new, count=1):
    cur = {p: x for p, x in edits}
    t = cur.get(path) or path.read_text()
    if t.count(old) != count: sys.exit(f"FAIL-CLOSED {path}: expected {count}x {old[:70]!r}, found {t.count(old)}")
    edits[:] = [e for e in edits if e[0] != path] + [(path, t.replace(old, new))]
def new(path, text):
    if path.exists(): sys.exit(f"FAIL-CLOSED exists {path}")
    edits.append((path, text))
dm = (DX / "📦️packages/🦀️rust/Cargo.toml").read_text()
pack = re.search(r'semio-framework-pack = \{ path = "((?:\.\./)+)(🧰️framework[^"]*)" \}', dm)
ups = pack.group(1) + "../../../"
new(PNG / "📦️packages/🦀️rust/Cargo.toml", f'''[package]
workspace = "{"../" * 11}"
name = "{NAME}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Stdio semio `drawing` subset anti-aliased png rasterizer"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🔖️1.2/✳️any/🦀️.rs"

[dependencies]
semio-framework-artifact-reference = {{ workspace = true }}
semio-framework-os-kernel = {{ workspace = true }}
semio-framework-plugin = {{ workspace = true }}
semio-framework-value = {{ workspace = true }}
semio-s-artifact-stdio-png = {{ workspace = true }}
semio-s-artifact-stdio-semio-base = {{ workspace = true }}
semio-s-artifact-stdio-semio-drawing = {{ workspace = true }}

[dev-dependencies]
semio-framework-os-kernel = {{ workspace = true, features = ["protocol-laws", "mutation-testing"] }}
semio-framework-plugin = {{ workspace = true, features = ["artifact-app-testing"] }}
semio-framework-async = {{ workspace = true }}
''')
leaf = LEAF / "🦀️.rs"
sub(leaf, "//! @see https://www.w3.org/TR/SVG11/painting.html\n", "//! @see https://www.w3.org/TR/SVG11/painting.html\n", 1)
sub(leaf, "use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;\n", "extern crate semio_framework_os_kernel as store;\n\nuse semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;\n", 1)
sub(leaf, '\n//#region 🔖️Tests\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n//#endregion 🔖️Tests\n', "\n", 1)
t = DX / "🦀️.rs"
sub(t, '#[path = "🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]\n                        mod component;\n                        pub use component::*;', 'pub use ' + MOD + '::*;', 1)
sub(t, "crate::serializers::artifacts::png::v1_2::any::SemioDrawingToPng", MOD + "::SemioDrawingToPng", 1)
sub(t, "extern crate semio_framework_os_kernel as store;\n", 'extern crate semio_framework_os_kernel as store;\n\n#[cfg(test)]\n#[path = "🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🧪️tests/🔬️unit/🦀️.rs"]\nmod png_raster_oracle;\n', 1)
sub(DX / "📦️packages/🦀️rust/Cargo.toml", "semio-s-artifact-stdio-png = { workspace = true }\n", f"semio-s-artifact-stdio-png = {{ workspace = true }}\n{NAME} = {{ workspace = true }}\n", 1)
sub(MX / "📦️packages/🦀️rust/Cargo.toml", "semio-s-artifact-stdio-semio-drawing-export = { workspace = true }\n", f"{NAME} = {{ workspace = true }}\n", 1)
sub(MX / "🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs", "use semio_s_artifact_stdio_semio_drawing_export::serializers::artifacts::png::v1_2::any::SemioDrawingToPng;", f"use {MOD}::SemioDrawingToPng;", 1)
sub(DX / "🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🧪️tests/🔬️unit/🦀️.rs", "use crate::serializers::artifacts::png::v1_2::any::rasterize_drawing;", f"use {MOD}::rasterize_drawing;", 1)
otest = LEAF / "🧪️tests/🔬️unit/🦀️.rs"
sub(otest, "use super::*;\n", f"use {MOD}::*;\nuse semio_framework_plugin::ArtifactSerializer;\nuse semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;\nuse semio_s_artifact_stdio_semio_drawing::schema::snapshot::{{DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot}};\nuse semio_s_artifact_stdio_semio_base::schema::geometry::{{SemioPoint2, SemioTransform}};\n", 1)
dep = f"[workspace.dependencies.{NAME}]\npath = "
for m in R.glob("*/🗿️artifacts/*/Cargo.toml"):
    t0 = m.read_text()
    h = "[workspace.dependencies.semio-s-artifact-stdio-semio-drawing-export]\npath = \""
    if h not in t0: continue
    mm = re.search(re.escape(h) + r'([^"]*)📤️export/📦️packages/🦀️rust"\n', t0)
    if not mm: sys.exit(f"FAIL-CLOSED path shape {m}")
    entry = h + mm.group(1) + '📤️export/📦️packages/🦀️rust"\n'
    sub(m, entry, entry + "\n" + dep + '"' + mm.group(1) + '📤️export/🧵️serializers/🗿️artifacts/📷️png/📦️packages/🦀️rust"\n', 1)
semio_toml = S.parents[2] / "Cargo.toml"
sub(semio_toml, '  "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/📦️packages/🦀️rust",\n', '  "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/📦️packages/🦀️rust",\n  "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/📦️packages/🦀️rust",\n', 1)
print(len(edits), "edits planned"); [print(" ", p) for p, _ in edits]
if APPLY:
    for p, txt in edits:
        p.parent.mkdir(parents=True, exist_ok=True); p.write_text(txt)
    print("applied; then: coordinator `dependencies lock` (new crate + mesh-export/drawing-export dep arrays in every lockfile), wasm --lib check mesh-export + drawing-export, native tests of the new crate and drawing-export")
