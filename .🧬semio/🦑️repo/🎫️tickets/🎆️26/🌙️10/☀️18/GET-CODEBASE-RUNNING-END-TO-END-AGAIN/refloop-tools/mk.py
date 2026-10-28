#!/usr/bin/env python3
"""🧱 Builds the flat staged batch `refloop` from refloop-tools/work and the CURRENT live bases. usage: python3 -I mk.py [root]"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
ROWS = [
    ("00.mjs", LIB + "🕸️dependencies/🟦️typescript/🟨️.mjs", "deps.mjs"),
    ("01.d.mts", LIB + "🕸️dependencies/🟦️typescript/🟨️.d.mts", "deps.d.mts"),
    ("02.mjs", LIB + "🏗️build/🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs", "boot.mjs"),
    ("03.ts", LIB + "🕸️dependencies/🟦️typescript/🧪️tests/🔁️verifier/🟦️.ts", "law.ts"),
    ("04.json", LIB + "🕸️dependencies/🟦️typescript/🧪️tests/🧫️fixtures/🔁️verifier/🔣️.json", "fixture.json"),
]
def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None
out = os.path.join(HERE, "staged")
for sub in ("files", "base"):
    shutil.rmtree(os.path.join(out, sub), ignore_errors=True); os.makedirs(os.path.join(out, sub))
manifest = {}
for name, rel, work in ROWS:
    live = os.path.join(ROOT, rel)
    shutil.copyfile(os.path.join(HERE, "work", work), os.path.join(out, "files", name))
    if os.path.exists(live): shutil.copyfile(live, os.path.join(out, "base", name))
    manifest[rel] = {"file": name, "base": sha(live), "new": sha(os.path.join(out, "files", name))}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
for rel, row in manifest.items(): print(row["file"], (row["base"] or "new")[:12], "->", row["new"][:12], rel[-70:])
