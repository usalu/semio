"""🧪 Exercises staged aggregate receiving with the actual PDF model and codec bodies."""

from pathlib import Path
import os
import re
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = Path(os.environ.get("SEMIO_TICKET_DONATION_OVERLAY", TICKET / "🗑️generated" / "set-donation"))
FRAMEWORK = Path("🧰️framework/🔨️modules").resolve()
JSON = FRAMEWORK / "🎒️pack/🔤️json"
REFERENCE = FRAMEWORK / "🧬️schema/🗿️artifact-reference"
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base").resolve()
CACHE = Path(".cache/b/r/m/u/l/pc5147e/prototype/build").resolve()
RUSTC = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
VALUE = "semio_framework_value"
NATIVE = "semio_framework_value_native_owned"
PACK_JSON = "semio_framework_pack_json"


def cached(*names):
    args = []
    for name in names:
        [path] = CACHE.glob(f"*/*/out/lib{name}-*.rlib")
        args += ["--extern", f"{name}={path.with_suffix('.rmeta')}", "--extern", f"{name}={path}"]
    return args


def extern(name, path):
    return ["--extern", f"{name}={path}"]


def run(name, options):
    path = OUT / f"{name}.args"
    directories = {p.parent for p in CACHE.glob("*/*/out/*") if p.suffix in (".rlib", ".rmeta", ".dylib")}
    path.write_text("\n".join([*options, f"-Ldependency={OUT}", *(f"-Ldependency={p}" for p in sorted(directories))]) + "\n", encoding="utf8")
    subprocess.run([*RUSTC, f"@{path}"], check=True)


def lib(name, source, args):
    library = OUT / f"lib{name}.rlib"
    run(name, [f"--crate-name={name}", "--crate-type=rlib", "--edition=2021", "-Copt-level=0", str(source), "-o", str(library), *args])
    return library


def declaration(path, name, kind="struct"):
    source = path.read_text(encoding="utf8")
    start = source.rindex("#[derive(", 0, source.index(f"pub {kind} {name}"))
    end = source.index("\n}", start) + 2
    return source[start:end].replace("dsl::MutationLeaf, ", "").replace("#[mutation_leaf(contract = ::protocol)]\n", "")


def original_function(source, label):
    start = source.index(label)
    indentation = source[source.rindex("\n", 0, start) + 1:start]
    end = source.index(f"\n{indentation}}}", start) + 2 + len(indentation)
    return indentation + source[start:end]


def object_declarations():
    schema = PDF / "🧬️schema"
    insert = declaration(schema / "🧬️mutations/📦️insert-object/🦀️.rs", "InsertObject")
    remove = declaration(schema / "🧬️mutations/🧹️remove-object/🦀️.rs", "RemoveObject")
    roles = (schema / "🪪️stream-roles/🦀️.rs").read_text(encoding="utf8")
    roles = roles[:roles.index("/// 🎯️ Resolves")]
    diff = schema / "🔺️diff/🔀️tri-state-indexed-and-keyed/🦀️.rs"
    diff_source = diff.read_text(encoding="utf8")
    types = "\n".join(declaration(diff, name) for name in ("PdfIndexedItem", "PdfIndexedDiff"))
    start = diff_source.index("impl<T> Default for PdfIndexedDiff<T>")
    defaults = diff_source[start:diff_source.index("\n}", start) + 2]
    patched = "\n".join(original_function(diff_source, method) for method in ("pub fn apply(&self, base: &[T])", "pub fn inverse(&self, base: &[T])"))
    start = diff_source.index("#[derive(Clone, Copy)]\npub(super) enum Origin")
    layout = diff_source[start:diff_source.index("\nimpl<T: Clone + PartialEq> PdfIndexedDiff<T>", start)]
    value = schema / "🔺️diff/🌳️value-trees/🦀️.rs"
    value_source = value.read_text(encoding="utf8")
    value_types = "\n".join(declaration(value, name) for name in (
        "PdfDictModified", "PdfDictAdded", "PdfDictDiff", "PdfArrayModified", "PdfArrayAdded", "PdfArrayDiff",
        "PdfObjectModified", "PdfObjectAdded", "PdfObjectsDiff",
    )) + "\n" + declaration(value, "PdfValueDiff", "enum")
    value_apply = "\n".join(original_function(value_source, signature) for signature in (
        "pub(super) fn apply_dict_diff", "fn apply_array_diff", "pub fn apply_value_diff", "pub(super) fn apply_objects_diff",
    )).replace("pub(super) fn apply_objects_diff", "pub fn apply_objects_diff")
    return (
        f"pub mod stream_roles {{\n{roles}\n}}\n"
        + f"pub mod diff {{\n{types}\n{defaults}\n{layout}\nimpl<T: Clone + PartialEq> PdfIndexedDiff<T> {{\n{patched}\n}}\n}}\n"
        + f"pub mod object_mutations {{\nuse super::{{diff, snapshot::{{ObjRef, PdfObject}}}};\n{insert}\n{remove}\n}}\n"
        + f"pub mod object_state {{\nuse super::snapshot::{{ObjRef, PdfObject, PdfIndirectObject, PdfDictEntry, PdfDecimal, PdfStreamFilter}};\n{value_types}\n{value_apply}\n}}\n"
    )


libraries = Path(os.environ.get("SEMIO_TICKET_DONATION_LIBRARIES", OUT))
if libraries != OUT:
    assert "--probe-only" in sys.argv
    for index in range(3):
        assert (OUT / f"{index}.rs").read_bytes() == (libraries / f"{index}.rs").read_bytes()
library = libraries / f"lib{PACK_JSON}.rlib"
reference = libraries / "libsemio_framework_artifact_reference.rlib"
model = libraries / "libpdf_model_probe.rlib"
[derive] = CACHE.glob("*/*/out/libsemio_framework_value_derive-*.dylib")
if "--probe-only" not in sys.argv:
    path_attr = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
    overlaid = {"🧮️parser-and-grammar-cursor/🦀️.rs": OUT / "grammar.rs", "📥️decode/🔢️u32/🦀️.rs": OUT / "2.rs"}
    root = path_attr.sub(lambda m: f'#[path = "{overlaid.get(m[1], JSON / m[1])}"]', (OUT / "0.rs").read_text())
    grammar = path_attr.sub(lambda m: f'#[path = "{JSON / "🧮️parser-and-grammar-cursor" / m[1]}"]', (OUT / "1.rs").read_text())
    (OUT / "grammar.rs").write_text(grammar)
    json_source = OUT / "json.rs"
    json_source.write_text(root)
    library = lib(PACK_JSON, json_source, cached("semio_framework_pack_json_canonical", "semio_framework_pack_json_float",
        VALUE, "semio_framework_value_ordered", "semio_framework_value_persistent",
        NATIVE, "semio_framework_value_native_receiving_original", "semio_framework_value_factory"))
    borrow = lib("semio_framework_artifact_reference_borrow", REFERENCE / "🔗️borrow/🦀️.rs", [])
    reference = lib("semio_framework_artifact_reference", REFERENCE / "🦀️.rs",
        extern("semio_framework_artifact_reference_borrow", borrow) + extern(PACK_JSON, library) +
        cached(VALUE, "semio_framework_value_persistent", NATIVE))
    model_source = OUT / "model.rs"
    model_source.write_text('extern crate self as pack;\npub use semio_framework_value as value;\n'
        f'#[path = "{PDF / "🧬️schema/📸️snapshot/📐️model/🦀️.rs"}"] pub mod model;\n')
    model = lib("pdf_model_probe", model_source, extern(PACK_JSON, library) +
        extern("semio_framework_artifact_reference", reference) + extern("value_derive", derive) + cached(VALUE, NATIVE))

varint_source = (FRAMEWORK / "🎒️pack/📐️format/🎮️retained/🧱️segment-and-varint-cursors/🦀️.rs").read_text(encoding="utf8")
start = varint_source.index("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n/// 🔢")
varint = varint_source[start:varint_source.index("/// 🚦", start)]
start = varint.index("impl RetainedVarintFault")
varint = varint[:start] + varint[varint.index("#[derive(Clone, Copy, Debug, Default", start):]

receiving_path = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs")
receiving_src = receiving_path.read_text()
start = receiving_src.index("/// 🧪️ Explicit physical stock")
receiving = receiving_src[start:receiving_src.index("/// ⚖️ Law", start)]
receiving = ("pub use semio_framework_replication_codec::{OpBinaryReceiving, OpBinaryReceivingCursor};\n"
    "pub use semio_framework_value_native_owned::OriginalReceivingPrimary;\n"
    "use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, ValueError, ValueRefusalKind};\n") + receiving

set_image_src = (PDF / "🧬️schema/🧬️mutations/🏞️set-image/🦀️.rs").read_text()
start = set_image_src.index("#[derive(")
set_image = set_image_src[start:set_image_src.index("\n}", start) + 2]
set_image = set_image.replace("dsl::MutationLeaf, ", "").replace("#[mutation_leaf(contract = ::protocol)]\n", "")

aggregate_source = (OUT / "4.rs").read_text()
start = aggregate_source.index("#[derive(", aggregate_source.index("//#region 🔖️Aggregate"))
end = aggregate_source.index("pub enum PdfMutation {", start) + len("pub enum PdfMutation {")
aggregate = aggregate_source[start:end].replace("dsl::Mutations, ", "")
aggregate = re.sub(r"(?m)^#\[mutations\(.*\)\]\n", "", aggregate)
has_objects = (OUT / "9.rs").exists()
has_leaves = (OUT / "10.rs").exists()
variants = [("SetImage", "setImage", 26)]
if has_objects:
    variants += [("InsertObject", "insertObject", 6), ("RemoveObject", "removeObject", 7)]
leaf_variants = [
    ("SetPageRotation", "setPageRotation", 15, "🔄️set-page-rotation"),
    ("SetInfo", "setInfo", 5, "ℹ️set-info"),
    ("SetLanguage", "setLanguage", 53, "🗣️set-language"),
] if has_leaves else []
variants += [(name, kind, tag) for name, kind, tag, _ in leaf_variants]
for variant, _, _ in variants:
    entry = f"    {variant}({variant}),"
    assert entry in aggregate_source
    aggregate += f"\n{entry}"
aggregate += "\n}\n"

install = os.environ.get("SEMIO_TICKET_DONATION_INSTALL")
binary_source = (Path(install or OUT) / "3.rs").read_text()
start = binary_source.index("const MAX_PAYLOAD_BYTES")
binary = binary_source[start:binary_source.index("//#endregion 🧱️Framing", start)]
tests = (OUT / "6.rs").read_text()
tests = tests[tests.index("fn image_archive_receiving_stock"):].replace('include_str!("set-image-donation.json")', 'include_str!("7.json")')
tests = tests.split("// Registered snapshot replay test.", 1)[0]
receiving_module = "5.rs"
installed_receiving = None
if install:
    start = binary_source.index("mod receiving {\n")
    end = binary_source.index("\npub use receiving::", start)
    installed_receiving = binary_source[start:end] + "\n"
if (OUT / "8.rs").exists():
    receiver = (OUT / receiving_module).read_text().replace('path = "🧾️metadata/🦀️.rs"', 'path = "8.rs"')
    if has_objects:
        receiver = receiver.replace('path = "📦️object/🦀️.rs"', 'path = "9.rs"')
    if has_leaves:
        receiver = receiver.replace('path = "🌿️leaves/🦀️.rs"', 'path = "10.rs"')
    (OUT / "receiving.rs").write_text(receiver)
    receiving_module = "receiving.rs"
program = OUT / "probe.rs"
capture = r'''
#[test]
fn capture_source_wire_for_independent_oracle() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("7.json")).unwrap();
    let directory = std::path::PathBuf::from(std::env::var_os("SEMIO_SETTLED_PDF_WIRE").unwrap());
    for lane in ["image", "alpha"] {
        let bytes = archive_image(&fixtures[lane]).encode_op().unwrap();
        std::fs::write(directory.join(format!("{lane}.bin")), &bytes).unwrap();
        println!("[DEBUG] {lane}: {} bytes crc32c {:08x} sha256 {}", bytes.len(), crc32c(&bytes), semio_framework_hash::sha256_hex(&bytes));
    }
}
'''
if has_objects:
    capture += (TICKET / "archive-object-state-probe.rs").read_text(encoding="utf8")
leaf_declarations = "use super::snapshot::PdfInfo;\n" if has_leaves else ""
for name, _, _, directory in leaf_variants:
    path = PDF / "🧬️schema/🧬️mutations" / directory / "🦀️.rs"
    leaf_declarations += declaration(path, name) + "\n"
    inverse = path.read_text().split("    fn inverse(&self, base:", 1)[1].split("\n}\n}", 1)[0]
    inverse = ("fn inverse(&self, base:" + inverse + "\n}").replace("&PdfSnapshot", "&crate::FixtureState").replace("Vec<Op>", "Vec<crate::PdfMutation>").replace("Op::from(", f"crate::PdfMutation::{name}(")
    leaf_declarations += f"impl {name} {{ pub(crate) {inverse} }}\n"
if has_leaves:
    capture += (TICKET / "archive-leaves-state-probe.rs").read_text(encoding="utf8")
leaf_imports = "{" + ", ".join(name for name, _, _, _ in leaf_variants) + "}"
program.write_text(
    '''extern crate self as dsl;
extern crate self as pack;
extern crate self as protocol;
extern crate self as store;
pub use semio_framework_replication_codec::{ByteReader, ByteWriter, OpBinary, crc32c};
pub use semio_framework_replication_error::ProtocolError;
pub mod pack_rt { pub const OP_BINARY_FORMAT: u8 = 1; }
mod standards { pub mod v1_7 { pub mod subsets { pub mod base { pub mod schema { pub mod snapshot {
    pub use pdf_model_probe::model;
    pub use pdf_model_probe::model::*;
}
'''
    + (object_declarations() if has_objects else "")
    + (f"mod ordinary_leaves {{ {leaf_declarations} }}\n" if has_leaves else "")
    + "pub mod mutations { pub use crate::set_image::SetImage;"
    + ("pub use super::object_mutations::{InsertObject, RemoveObject};" if has_objects else "")
    + (f"pub use super::ordinary_leaves::{leaf_imports};" if has_leaves else "")
    + " } } } } } }\n"
    + '''
use standards::v1_7::subsets::base::schema::snapshot::*;
'''
    + ("use standards::v1_7::subsets::base::schema::mutations::{InsertObject, RemoveObject};\n" if has_objects else "")
    + (f"use standards::v1_7::subsets::base::schema::mutations::{leaf_imports};\n" if has_leaves else "")
    + 'const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &['
    + ", ".join(f'("{name}", "{kind}", {tag})' for name, kind, tag in variants) + '];\n'
    + (installed_receiving or f'#[path = "{receiving_module}"] mod receiving;\n')
    + "pub mod format {\n" + varint + "\n}\n"
    + "pub mod original_receiving {\n" + receiving + "\n}\n"
    + "mod set_image {\nuse super::PdfImage;\n" + set_image + "\n}\nuse set_image::SetImage;\n"
    + aggregate + binary + tests + capture, encoding="utf8")
probe = OUT / "probe"
run("probe", ["--test", "--edition=2024", "-Copt-level=1", str(program), "-o", str(probe),
    f"-Ldependency={libraries}",
    *extern(PACK_JSON, library), *extern("pdf_model_probe", model), *extern("value_derive", derive),
    *extern("semio_framework_artifact_reference", reference),
    *cached(VALUE, NATIVE, "semio_framework_dsl_record", "semio_framework_hash",
            "semio_framework_replication_codec", "semio_framework_replication_error", "serde_json")])
subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(probe), "--nocapture"], check=True,
    env={**os.environ, "SEMIO_SETTLED_PDF_WIRE": str(OUT)})
