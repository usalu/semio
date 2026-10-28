#!/usr/bin/env python3
"""🧰 Builds one flat staged batch (house style) from a mirror: base = the LIVE file now (absent = new), new = the mirror file.
usage: python3 -I make_stage.py <batch-dir> <mirror-root> <live-root> <rel>..."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
def digest(path):
    if not os.path.exists(path): return None
    return hashlib.sha256(open(path, "rb").read()).hexdigest()
batch, mirror, live, rels = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]
for sub in ("files", "base"): shutil.rmtree(os.path.join(batch, sub), ignore_errors=True); os.makedirs(os.path.join(batch, sub))
rows = {}
for index, rel in enumerate(rels):
    ext = os.path.splitext(rel)[1] or ".bin"; name = f"{index:02d}{ext}"
    shutil.copyfile(os.path.join(mirror, rel), os.path.join(batch, "files", name))
    base = os.path.join(live, rel)
    if os.path.exists(base): shutil.copyfile(base, os.path.join(batch, "base", name))
    rows[rel] = {"file": name, "base": digest(base), "new": digest(os.path.join(mirror, rel))}
    if rows[rel]["base"] == rows[rel]["new"]: sys.exit(f"unchanged: {rel}")
json.dump(rows, open(os.path.join(batch, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
shutil.copyfile(os.path.join(HERE, "stage_template.py"), os.path.join(batch, "stage.py"))
print(len(rows), "files ->", batch)
