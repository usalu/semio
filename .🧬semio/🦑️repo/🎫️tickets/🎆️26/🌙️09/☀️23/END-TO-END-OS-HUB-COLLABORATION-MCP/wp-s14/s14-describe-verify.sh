#!/bin/zsh
# After activate: describe key rebuilt plugins and capture descriptor hashes.
set -u
WP=/Users/ueli/Documents/semio/.tmp-ticket/wp-s14
GEN=$WP/generated
MUTEX=/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh
OUT=$GEN/s14-describe-hashes.txt
: > "$OUT"
# activate already published guests into the vite tree; describe via nx package names when possible
# Fallback: hash the staged component-dev wasm deliverables under each plugin dist.
python3 - <<'PY' | tee -a "$OUT"
from pathlib import Path
import hashlib, os
root = Path("/Users/ueli/Documents/semio")
# find staged component-dev deliverables for s plugins
hits = []
plugins = root.glob("*/plugins")  # will miss emoji
# walk via known plugin link dir
link = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-s14/links")
for name in ["plugin-writer","plugin-sourcing","plugin-stdio","plugin-trinity","plugin-energy"]:
    p = (link/name).resolve()
    for wasm in p.rglob("*.wasm"):
        if "component-dev" in str(wasm) or "dist" in str(wasm):
            h = hashlib.sha256(wasm.read_bytes()).hexdigest()[:16]
            hits.append((str(wasm.relative_to(p)), h, wasm.stat().st_size))
hits.sort()
print(f"hashed {len(hits)} wasm deliverables")
for rel,h,sz in hits[:80]:
    print(f"{h}  {sz:10d}  {rel}")
PY
