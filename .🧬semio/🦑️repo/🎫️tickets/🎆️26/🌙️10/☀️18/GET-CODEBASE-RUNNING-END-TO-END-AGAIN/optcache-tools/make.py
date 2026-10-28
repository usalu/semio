#!/usr/bin/env python3
"""📦 [optcache] builds the flat staged batch `store` from the CURRENT live files: copies the two live files to a scratch tree, replays edit_optcache.py on it (exit 2 when an anchor is gone), writes staged/store/{manifest.json,files/,base/,stage.py} (verify.sh is kept). usage: make.py"""
import hashlib, json, os, shutil, subprocess, sys
T = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(T); L = os.path.abspath(os.path.join(C, *[".."] * 7))
G = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/"
rels = [G + "🧰️support/🟦️.ts", G + "🧊️optional-cache/🟦️.ts"]
work = os.path.join(C, "🗑️generated", "optcache", "new"); shutil.rmtree(work, ignore_errors=True)
for rel in rels:
    os.makedirs(os.path.dirname(os.path.join(work, rel)), exist_ok=True); shutil.copyfile(os.path.join(L, rel), os.path.join(work, rel))
if subprocess.call([sys.executable, "-I", os.path.join(T, "edit_optcache.py"), work]) != 0: sys.exit(2)
out = os.path.join(T, "staged", "store"); keep = open(out + "/verify.sh", encoding="utf8").read() if os.path.exists(out + "/verify.sh") else None
shutil.rmtree(out, ignore_errors=True); os.makedirs(out + "/files"); os.makedirs(out + "/base")
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
manifest = {}
for i, rel in enumerate(rels):
    name = "f%03d" % i; shutil.copyfile(os.path.join(work, rel), f"{out}/files/{name}"); shutil.copyfile(os.path.join(L, rel), f"{out}/base/{name}")
    manifest[rel] = {"file": name, "base": sha(os.path.join(L, rel)), "new": sha(os.path.join(work, rel))}
json.dump(manifest, open(out + "/manifest.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
shutil.copyfile(os.path.join(T, "stage.template.py"), out + "/stage.py")
if keep is not None: open(out + "/verify.sh", "w", encoding="utf8").write(keep)
print("store", len(manifest), "files")
