"""⚖️ Compare EN 1990 mutation identities with their pre-carve wire contracts."""

import hashlib
import json
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
expected = {
    "🔗️.graphql": "12691ba250b843285118a50c6e6cdfdbf3104ce77a30b4cc1c86e3d1b0edaf8c",
    "🔣️.json": "7bbcc02f802b4ef3c7f8da99b8515263605c58982ceef245d430f5a985b139b1",
    "🛰️.proto": "131c21d1aa65731a50c40dd148569aca151caa44d41fbbc73f3178f8b4904d48",
    "🟦️.ts": "3a1349a447ce2cfb532932b7021de78d33a96808f64c21dc0b7bf0be84e8f014",
}
for name, digest in expected.items():
    assert hashlib.sha256((root / name).read_bytes()).hexdigest() == digest, name
rust = (root / "🦀️.rs").read_text().split("pub enum En1990Mutation {", 1)[1].split("\n}", 1)[0]
native = re.findall(r"^\s*(\w+)\((\w+)::(\w+)\),", rust, re.M)
sequence = "\n".join("/".join(row) for row in native)
assert hashlib.sha256(sequence.encode()).hexdigest() == "dbd80b124ebd0ccd6c438c557f152ea3a542a388b276b6f904465b5acc9defba"
refs = json.loads((root / "🔣️.json").read_text())["oneOf"]
proto = (root / "🛰️.proto").read_text().split("oneof kind {", 1)[1].split("}", 1)[0]
fields = re.findall(r"^\s*(\w+)\s+\w+\s*=\s*(\d+);", proto, re.M)
assert len(native) == len(refs) == len(fields) == 134
assert [int(number) for _, number in fields] == list(range(1, 135))
wire_names = {ref["$ref"].split("/")[-2].replace("-", "").lower() for ref in refs}
assert len(wire_names) == 134
assert {name.lower() for name, _, _ in native} == wire_names == {name.lower() for name, _ in fields}
print("All 134 native variants and four wire contract files match the pre-carve baseline.")
