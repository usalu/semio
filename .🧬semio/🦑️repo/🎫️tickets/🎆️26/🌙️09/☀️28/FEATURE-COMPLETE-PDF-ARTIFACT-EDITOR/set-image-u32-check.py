"""🧪 Executes the staged receiver against the actual PDF Samples declaration."""

from hashlib import sha256
import json
from pathlib import Path
import re
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "set-u32"
FRAMEWORK = Path("🧰️framework/🔨️modules").resolve()
JSON = FRAMEWORK / "🎒️pack/🔤️json"
REFERENCE = FRAMEWORK / "🧬️schema/🗿️artifact-reference"
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base").resolve()
CACHE = Path(".cache/b/r/m/u/l/pc5147e/prototype/build").resolve()
RUSTC = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]


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
    run(name, [f"--crate-name={name}", "--crate-type=rlib", "--edition=2021", "-Copt-level=1", str(source), "-o", str(library), *args])
    return library


def varint(size):
    parts = []
    while size >= 128:
        parts.append(128 | (size & 127))
        size >>= 7
    return bytes([*parts, size])


def fixtures():
    law = json.loads((TICKET / "set-image-shape.json").read_text(encoding="utf8"))
    expected = ["3555dddb4d33a1d4f0134d306d4a6406f8df333b68c64e18bf6a08d681a54149",
                "7eaf21da2b29bef6f0993dcabbc88d329889da705ed669448bbb9cbb4a9d4773"]
    for lane, row in enumerate(law["mutations"]):
        count = row["width"] * row["height"] * row["components"]
        samples = [(index * row["stride"]) % 256 for index in range(count)]
        image = {"id": row["id"], "width": row["width"], "height": row["height"], "colorSpace": {"kind": row["color"]},
                 "bitsPerComponent": 8, "imageMask": False, "interpolate": False, "body": {"kind": "samples", "values": samples}}
        record = {"mutation": "setImage", "image": image}
        if "softMask" in row:
            image.update({"softMask": row["softMask"], "decode": [0.5, 1.0], "mask": {"kind": "colorKey", "ranges": [1, 2, 3, 4, 5, 6]},
                "softMaskInData": 1, "matte": [0.1, 0.2, 0.3], "intent": "RelativeColorimetric", "optionalContent": "FixtureGroup",
                "structParent": 17, "extra": [{"key": "Name", "value": {"kind": "int", "value": 7}}]})
            record["index"] = 3
        payload = json.dumps(record, separators=(",", ":")).encode()
        frame = bytes([law["format"], law["tag"]]) + varint(len(payload)) + payload
        assert sha256(frame).hexdigest() == expected[lane]
        (OUT / f"fixture-{lane}.bin").write_bytes(frame)


fixtures()
name = "semio_framework_pack_json"
library = OUT / f"lib{name}.rlib"
reference = OUT / "libsemio_framework_artifact_reference.rlib"
if "--probe-only" not in sys.argv:
    attribute = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
    overlaid = {"🧮️parser-and-grammar-cursor/🦀️.rs": OUT / "grammar.rs", "📥️decode/🔢️u32/🦀️.rs": OUT / "2.rs"}
    root = attribute.sub(lambda m: f'#[path = "{overlaid.get(m[1], JSON / m[1])}"]', (OUT / "0.rs").read_text())
    grammar = attribute.sub(lambda m: f'#[path = "{JSON / "🧮️parser-and-grammar-cursor" / m[1]}"]', (OUT / "1.rs").read_text())
    (OUT / "grammar.rs").write_text(grammar)
    json_source = OUT / "json.rs"
    json_source.write_text(root)
    library = lib(name, json_source, cached("semio_framework_pack_json_canonical", "semio_framework_pack_json_float",
        "semio_framework_value", "semio_framework_value_ordered", "semio_framework_value_persistent",
        "semio_framework_value_native_owned", "semio_framework_value_native_receiving_original", "semio_framework_value_factory"))
    borrow = lib("semio_framework_artifact_reference_borrow", REFERENCE / "🔗️borrow/🦀️.rs", [])
    reference = lib("semio_framework_artifact_reference", REFERENCE / "🦀️.rs",
        extern("semio_framework_artifact_reference_borrow", borrow) + extern(name, library) +
        cached("semio_framework_value", "semio_framework_value_persistent", "semio_framework_value_native_owned"))

model = (PDF / "🧬️schema/📸️snapshot/📐️model/🖼️xobjects/🦀️.rs").read_text()
begin = model.index('/// 🖼️ A logical image body')
end = model.index('\n}', begin) + 2
declaration = model[begin:end]
assert "pub enum PdfImageBody" in declaration and "values:Vec<u32>" in declaration
source = OUT / "probe.rs"
source.write_text(
    '#[path = "3.rs"] mod projection;\n'
    'mod standards { pub mod v1_7 { pub mod subsets { pub mod base { pub mod schema { pub mod snapshot {\n'
    + declaration + '\n} } } } } }\n' + (TICKET / "set-image-u32-check.rs").read_text(), encoding="utf8",
)
[derive] = CACHE.glob("*/*/out/libsemio_framework_value_derive-*.dylib")
probe = OUT / "probe"
run("probe", ["--test", "--edition=2024", "-Copt-level=2", str(source), "-o", str(probe),
    *extern(name, library), *extern("semio_framework_artifact_reference", reference), *extern("value_derive", derive),
    *cached("semio_framework_value", "semio_framework_value_native_owned", "semio_framework_dsl_record",
            "semio_framework_replication_codec", "serde_json")])
subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(probe), "--nocapture"], check=True)
