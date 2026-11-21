"""🧪 Builds the staged grammar and exercises named native SetImage projection."""

import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "set-image"
FRAMEWORK = Path("🧰️framework/🔨️modules/🎒️pack/🔤️json").resolve()
CACHE = Path(".cache/b/r/m/u/l/pc5147e/prototype/build").resolve()
RUSTC = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
LAW = json.loads((TICKET / "set-image-shape.json").read_text())


def arguments(names):
    args = []
    for name in names:
        [path] = CACHE.glob(f"*/*/out/lib{name}-*.rlib")
        args += ["--extern", f"{name}={path.with_suffix('.rmeta')}", "--extern", f"{name}={path}"]
    return args


def run(name, options):
    path = OUT / f"{name}.args"
    libraries = {p.parent for p in CACHE.glob("*/*/out/*") if p.suffix in (".rlib", ".rmeta", ".dylib")}
    path.write_text("\n".join([*options, *(f"-Ldependency={p}" for p in sorted(libraries))]) + "\n", encoding="utf8")
    subprocess.run([*RUSTC, f"@{path}"], check=True)


def varint(length):
    prefix = []
    while length >= 128:
        prefix.append(128 | (length & 127))
        length >>= 7
    return bytes([*prefix, length])


for lane, row in enumerate(LAW["mutations"]):
    samples = [(index * row["stride"]) % 256 for index in range(row["width"] * row["height"] * row["components"])]
    record = {"mutation": "setImage", "image": {
        "id": row["id"], "width": row["width"], "height": row["height"], "colorSpace": {"kind": row["color"]},
        "bitsPerComponent": 8, "imageMask": False, "interpolate": False, "body": {"kind": "samples", "values": samples},
    }}
    if "softMask" in row:
        record["image"].update({"softMask": row["softMask"], "decode": [0.5, 1.0], "mask": {"kind": "colorKey", "ranges": [1, 2, 3, 4, 5, 6]},
            "softMaskInData": 1, "matte": [0.1, 0.2, 0.3], "intent": "RelativeColorimetric", "optionalContent": "FixtureGroup", "structParent": 17,
            "extra": [{"key": "Name", "value": {"kind": "int", "value": 7}}]})
        record["index"] = 3
    payload = json.dumps(record, separators=(",", ":")).encode()
    frame = bytes([LAW["format"], LAW["tag"]]) + varint(len(payload)) + payload
    (OUT / f"fixture-{lane}.bin").write_bytes(frame)
    print(f"[DEBUG] lane {lane}: {len(frame)} frame bytes, {len(samples)} samples, SHA-256 {hashlib.sha256(frame).hexdigest()}")

overlaid = {
    "🧮️parser-and-grammar-cursor/🦀️.rs": OUT / "grammar-load.rs",
    "📥️decode/🔢️array/🦀️.rs": OUT / "2.rs",
}
leaf = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
root = (OUT / "0.rs").read_text()
root = leaf.sub(lambda m: '#[path = "' + str(overlaid.get(m[1], FRAMEWORK / m[1])) + '"]', root)
grammar = (OUT / "1.rs").read_text()
grammar = leaf.sub(lambda m: '#[path = "' + str(FRAMEWORK / "🧮️parser-and-grammar-cursor" / m[1]) + '"]', grammar)
assert not any("#[path" in line for line in (OUT / "2.rs").read_text().splitlines())
(OUT / "grammar-load.rs").write_text(grammar, encoding="utf8")
source = OUT / "json.rs"
source.write_text(root, encoding="utf8")
library = OUT / "libsemio_framework_pack_json.rlib"
if "--probe-only" not in sys.argv:
    run("json", ["--crate-name=semio_framework_pack_json", "--crate-type=rlib", "--edition=2021", "-Copt-level=1",
        str(source), "-o", str(library), *arguments(("semio_framework_pack_json_canonical", "semio_framework_pack_json_float",
            "semio_framework_value", "semio_framework_value_ordered", "semio_framework_value_persistent",
            "semio_framework_value_native_owned", "semio_framework_value_native_receiving_original", "semio_framework_value_factory"))])
bank = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧪️tests/🧰️bank.rs").resolve()
probe = OUT / "probe.rs"
probe.write_text(f'#[path = "3.rs"]\nmod mixed;\n#[allow(dead_code)]\n#[path = "{bank}"]\nmod bank;\n'
    + (TICKET / "set-image-check.rs").read_text(encoding="utf8"), encoding="utf8")
run("probe", ["--test", "--edition=2024", "-Copt-level=2", str(probe), "-o", str(OUT / "probe"),
    "--extern", f"semio_framework_pack_json={library}",
    *arguments(("semio_framework_value", "semio_framework_value_native_owned", "semio_framework_dsl_record",
        "semio_framework_replication_codec", "serde_json"))])
subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(OUT / "probe"), "--nocapture"], check=True)
