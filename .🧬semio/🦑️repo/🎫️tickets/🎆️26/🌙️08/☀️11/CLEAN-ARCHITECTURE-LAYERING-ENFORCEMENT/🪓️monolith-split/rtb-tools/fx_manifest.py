"""Regenerates the manifest of the fixtures crate from the identifiers its modules use."""
import sys
from build import *
import manifest
FX_DIR = T + "/🧰️fixtures"
text = ""
for root, dirs, files in os.walk(FX_DIR):
    for f in files:
        if f.endswith(".rs"):
            text += read(os.path.join(root, f)) + "\n"
idents = used_idents(text)
mt, miss = manifest.build_manifest("fixtures", "fixtures", "Shared test fixtures of the wgpu renderer test bins", idents)
mt = mt.replace("\n[dependencies]\n", "\n[features]\ntestkit = []\n\n[dependencies]\n", 1)
write(FX_DIR + "/📦️packages/🦀️rust/Cargo.toml", mt)
print("deps:", mt.count("\n") , "unmatched idents:", sorted(m for m in miss if m[0].islower() and m not in ("std", "super", "self", "crate")))
