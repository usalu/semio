"""WP-T3 helper: write legacy-shaped io leaves (`serialize_bytes`/`deserialize_bytes`) over the shared
stdio semio writers, so every owner's leaf is the same few lines.

usage (from an owner's `🚪️io` directory):
  python3 leaves.py drawing <label> <Snapshot> <projection-use-path> <projection-fn> <fmt>...
  python3 leaves.py mesh <label> <Snapshot> <projection-use-path> <projection-fn> <fmt>...
  python3 leaves.py txt <label> <Snapshot>
  python3 leaves.py zip <label> <Snapshot>
formats: drawing -> svg pdf png dxf dwg ; mesh -> stl obj ply gltf las dwg png
"""
import os
import sys

EXPORT = "📤️export/🧵️serializers/🗿️artifacts"
IMPORT = "📥️import/🧩️deserializers/🗿️artifacts"

DRAWING = {
    "svg": ("🎨️svg/🔖️1.1", "SemioDrawingFormat::Svg", "SVG 1.1"),
    "pdf": ("📖️pdf/🔖️1.4", 'SemioDrawingFormat::Pdf { version: "1.4" }', "a one-page vector PDF 1.4"),
    "png": ("📷️png/🔖️1.2", "SemioDrawingFormat::Png", "an anti-aliased PNG raster"),
    "dxf": ("📐️dxf/🔖️r12", "SemioDrawingFormat::Dxf", "DXF R12 entities"),
    "dwg": ("🖊️dwg/🔖️ac1018", "SemioDrawingFormat::Dwg", "DWG entities"),
}
MESH = {
    "stl": ("🔺️stl/🔖️ascii", "SemioMeshFormat::Stl", "ASCII STL facets"),
    "obj": ("🗿️obj/🔖️3.0", "SemioMeshFormat::Obj", "Wavefront OBJ"),
    "ply": ("🧱️ply/🔖️1.0", "SemioMeshFormat::Ply", "PLY vertex and face elements"),
    "gltf": ("🧊️gltf/🔖️2.0", "SemioMeshFormat::Gltf", "glTF 2.0 with an embedded buffer"),
    "las": ("☁️las/🔖️1.0", "SemioMeshFormat::Las", "LAS point records of its vertices"),
    "dwg": ("🖊️dwg/🔖️ac1018", "SemioMeshFormat::Dwg", "a DWG polyface mesh"),
    "png": ("📷️png/🔖️1.2", "SemioMeshFormat::Png", "a shaded isometric PNG view"),
}


def leaf_dir(base: str, rel: str) -> str:
    """Resolve a leaf directory, tolerating the owner's own emoji spelling of the format folder."""
    fmt_dir, version = rel.split("/")
    name = fmt_dir.lstrip("🎨️📖📷📐🖊🔺🗿🧱🧊☁️")
    for candidate in os.listdir(base):
        if candidate.endswith(name) or candidate.lstrip("️").endswith(name):
            path = os.path.join(base, candidate, version, "✳️any")
            if os.path.isdir(path):
                return path
    raise SystemExit(f"no leaf directory for {rel} under {base}")


def write(path: str, text: str) -> None:
    open(os.path.join(path, "🦀️.rs"), "w", encoding="utf8").write(text)


def drawing(label, snapshot, use_path, fn, formats, table, module, format_enum, encode, what_kind):
    fallible = fn.endswith("?")
    fn = fn.rstrip("?")
    for fmt in formats:
        rel, variant, what = table[fmt]
        path = leaf_dir(EXPORT, rel)
        projected = f"&{fn}(snapshot).map_err(|error| store::TextError::new(format!(\"{label}→{fmt}: {{error}}\"), dsl::TextSpan::at(1, 1)))?" if fallible else f"&{fn}(snapshot)"
        write(path, f'''//! {label} → {fmt} — the {what_kind} (`{fn}`) written as {what} by `s.stdio.semio/v1/{module}`'s own
//! export leaf, the one {fmt} writer every owner shares.
//!
//! 🔖 `IoFidelity::Lossy`: a {what_kind}, not the document — there is no {fmt} import.
use {use_path}::{fn};
use crate::{snapshot};
use semio_s_artifact_stdio_semio::standards::v1::subsets::{module}::io::{{{encode}, {format_enum}}};

pub fn register() {{}}

pub fn serialize_bytes(snapshot: &{snapshot}) -> Result<Vec<u8>, store::TextError> {{
    {encode}({projected}, {variant}).map_err(|error| store::TextError::new(format!("{label}→{fmt}: {{error}}"), dsl::TextSpan::at(1, 1)))
}}
''')
        print(f"wrote {path}")


def txt(label, snapshot):
    write(os.path.join(EXPORT, "🔤️txt/🔖️utf-8/✳️any"), f'''//! {label} → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::{snapshot};

pub fn register() {{}}

pub fn serialize_bytes(from: &{snapshot}) -> Result<Vec<u8>, store::TextError> {{
    Ok(<{snapshot} as store::ArtifactDsl>::print_dsl(from).into_bytes())
}}
''')
    write(os.path.join(IMPORT, "🔤️txt/🔖️utf-8/✳️any"), f'''//! {label} ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the inverse
//! of the sibling export leaf (`IoFidelity::Exact`).
use crate::{snapshot};

pub fn register() {{}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snapshot}, store::TextError> {{
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("{label}←txt: {{error}}"), dsl::TextSpan::at(1, 1)))?;
    <{snapshot} as store::ArtifactDsl>::parse_dsl(text)
}}
''')
    print("wrote txt pair")


def zip_pair(label, snapshot):
    write(leaf_dir(EXPORT, "🎒️zip/🔖️2.0"), f'''//! {label} → zip — the shared document archive (`encode_document_archive`): this artifact's DSL as the
//! authoritative member plus its rfc8259 rendition, both lossless (`IoFidelity::Exact`).
use crate::{snapshot};
use semio_s_artifact_stdio_zip::io::encode_document_archive;

pub fn register() {{}}

pub fn serialize_bytes(snapshot: &{snapshot}) -> Result<Vec<u8>, store::TextError> {{
    encode_document_archive(snapshot).map_err(|error| store::TextError::new(format!("{label}→zip: {{error}}"), dsl::TextSpan::at(1, 1)))
}}
''')
    write(leaf_dir(IMPORT, "🎒️zip/🔖️2.0"), f'''//! {label} ← zip — the DSL member of a document archive written by the sibling export leaf (or any
//! archive carrying that member), parsed as this artifact's own DSL (`IoFidelity::Exact`).
use crate::{snapshot};
use semio_s_artifact_stdio_zip::io::decode_document_archive;

pub fn register() {{}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snapshot}, store::TextError> {{
    decode_document_archive(bytes).map_err(|error| store::TextError::new(format!("{label}←zip: {{error}}"), dsl::TextSpan::at(1, 1)))
}}
''')
    print("wrote zip pair")


def main():
    kind, label, snapshot, *rest = sys.argv[1:]
    if kind == "drawing":
        use_path, fn, *formats = rest
        drawing(label, snapshot, use_path, fn, formats, DRAWING, "drawing", "SemioDrawingFormat", "encode_drawing", "page drawing")
    elif kind == "mesh":
        use_path, fn, *formats = rest
        drawing(label, snapshot, use_path, fn, formats, MESH, "mesh", "SemioMeshFormat", "encode_mesh", "surface mesh")
    elif kind == "txt":
        txt(label, snapshot)
    elif kind == "zip":
        zip_pair(label, snapshot)
    else:
        raise SystemExit(f"unknown kind {kind}")


main()
