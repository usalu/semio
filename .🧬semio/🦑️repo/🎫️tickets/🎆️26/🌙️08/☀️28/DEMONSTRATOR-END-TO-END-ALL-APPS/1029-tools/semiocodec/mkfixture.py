#!/usr/bin/env python3
"""Writes `🧿️semio/🧪️tests/🎒️record-document/🧫️fixtures/🔣️.json`: the language-agnostic vectors both test files read.

BLAKE3 hashes come from `@noble/hashes` (run by `blake3_vectors.ts`), CRC-32C vectors are RFC 3720 appendix B.4, the deflate vectors are segments the Rust writer produced, and the pack/snapshot lists are discovered
from the committed files.
"""
import importlib.util
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio")
spec = importlib.util.spec_from_file_location("rd", ROOT / "🚪️io/💾️binary/🎒️record-document/🐍️.py")
codec = importlib.util.module_from_spec(spec)
sys.modules["rd"] = codec
spec.loader.exec_module(codec)
SUBSETS = ROOT / "🏅️standards/🔖️v1/🪆️subsets"
HERE = pathlib.Path(__file__).parent


def rel(path: pathlib.Path) -> str:
    return "/".join(path.relative_to(ROOT).parts)


specs = {}
for folder in sorted(SUBSETS.iterdir()):
    path = folder / "🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json"
    if path.exists():
        loaded = codec.load_record_spec(str(path))
        specs[loaded.token] = (folder, path, loaded)
packs = {token: [] for token in specs}
for file in sorted(ROOT.rglob("*pack.semio")):
    if "node_modules" in file.parts or "dist" in file.parts:
        continue
    try:
        token, _ = codec.unwrap_binary(file.read_bytes())
    except ValueError:
        continue
    if token in specs:
        packs[token].append(file)

rows = []
for token, (folder, path, loaded) in specs.items():
    vectors = [file for file in folder.rglob("🔣️.json") if file.parent.name.endswith(("before", "after")) and any(part.endswith("fixtures") for part in file.parts)]
    pairs = []
    for pack in packs[token]:
        document = codec.decode_pack(loaded, pack.read_bytes(), narrow_f32=True)
        for vector in vectors:
            if json.loads(vector.read_text(encoding="utf-8")) == document:
                pairs.append({"pack": rel(pack), "json": rel(vector)})
                break
    row = {"folder": folder.name, "spec": rel(path), "envelope": loaded.envelope, "packs": [rel(pack) for pack in packs[token]]}
    if pairs:
        row["snapshots"] = pairs
    rows.append(row)

blake_sizes = [0, 1, 3, 63, 64, 65, 1000, 1023, 1024, 1025, 2047, 2048, 2049, 3072, 4096, 8191, 8192, 100000]
noble = subprocess.run(["bun", "run", str(HERE / "blake3_vectors.ts"), json.dumps(blake_sizes)], check=True, capture_output=True, text=True).stdout
deflate = json.load(open(sys.argv[1]))
small = [row for row in deflate if len(row["rawHex"]) // 2 <= 500]
fixture = {
    "schema": "semio.stdio.record-document-laws/v1",
    "packCount": sum(len(row["packs"]) for row in rows),
    "crc32c": [
        {"hex": "", "crc": 0},
        {"hex": "313233343536373839", "crc": 3808858755},
        {"hex": "00" * 32, "crc": 0x8A9136AA},
        {"hex": "ff" * 32, "crc": 0x62A8AB43},
        {"hex": bytes(range(32)).hex(), "crc": 0x46DD794E},
        {"hex": bytes(range(31, -1, -1)).hex(), "crc": 0x113FDB5C},
    ],
    "blake3": json.loads(noble),
    "deflate": small,
    "subsets": rows,
}
target = ROOT / "🧪️tests/🎒️record-document/🧫️fixtures/🔣️.json"
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print("packs", fixture["packCount"], "snapshot pairs", sum(len(row.get("snapshots", [])) for row in rows), "deflate", len(small))
