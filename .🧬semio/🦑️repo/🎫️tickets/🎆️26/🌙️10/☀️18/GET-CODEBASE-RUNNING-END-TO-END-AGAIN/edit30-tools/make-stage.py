#!/usr/bin/env python3
"""🧳 Builds a flat staged batch from a mirror: make-stage.py <batch-dir> <mirror-root> <docstring> <rel>...
files/NN.ext = mirror bytes, base/NN.ext = live bytes now (absent = new file), manifest.json {rel:{file,base,new}}, stage.py = xplat5 mode-keeping template with this docstring."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
def digest(path):
    if not os.path.exists(path): return None
    with open(path, "rb") as f: return hashlib.sha256(f.read()).hexdigest()
batch, mirror, doc, rels = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2]), sys.argv[3], sys.argv[4:]
live_root = HERE
while not os.path.isdir(os.path.join(live_root, "🧰️framework")): live_root = os.path.dirname(live_root)
for sub in ("files", "base"): shutil.rmtree(os.path.join(batch, sub), ignore_errors=True); os.makedirs(os.path.join(batch, sub))
rows = {}
for index, rel in enumerate(rels):
    name = "%02d%s" % (index, os.path.splitext(rel)[1])
    live, new = os.path.join(live_root, rel), os.path.join(mirror, rel)
    shutil.copyfile(new, os.path.join(batch, "files", name))
    if os.path.exists(live): shutil.copyfile(live, os.path.join(batch, "base", name))
    rows[rel] = {"file": name, "base": digest(live), "new": digest(new)}
json.dump(rows, open(os.path.join(batch, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
lines = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read().split("\n")
lines[1] = '"""' + doc
open(os.path.join(batch, "stage.py"), "w", encoding="utf8").write("\n".join(lines))
for rel, row in rows.items(): print("new" if row["base"] is None else "edit", rel[-100:])
