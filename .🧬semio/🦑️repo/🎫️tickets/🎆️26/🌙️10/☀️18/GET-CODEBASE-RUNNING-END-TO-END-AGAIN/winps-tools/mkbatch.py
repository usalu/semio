#!/usr/bin/env python3
"""🧱 Builds (or rebases) a staged batch of lane `winps` on the CURRENT live files: python3 -I mkbatch.py <batch> [--live ROOT]. Copies the files named in `staged/<batch>/rels.txt` from live into a scratch root, runs `staged/<batch>/edit.py <scratch>`, and writes `files/`, `base/`, `manifest.json` ({rel:{file,base,new}}, base null for a new file) plus `stage.py` from the xplat5 template (keeps file modes). Re-run right before the gate lands the batch; the edits are anchored, so a moved anchor stops the build instead of guessing."""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
name = sys.argv[1]
live = os.path.abspath(sys.argv[sys.argv.index("--live") + 1]) if "--live" in sys.argv else "/Users/ueli/Documents/semio"
batch = os.path.join(HERE, "staged", name)
scratch = os.path.join(HERE, "..", "🗑️generated", "winps", "scratch", name)
rels = [line.strip() for line in open(os.path.join(batch, "rels.txt"), encoding="utf-8") if line.strip()]
shutil.rmtree(scratch, ignore_errors=True)
for rel in rels:
    source = os.path.join(live, rel)
    if os.path.exists(source):
        os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True)
        shutil.copyfile(source, os.path.join(scratch, rel))
subprocess.run([sys.executable, "-I", os.path.join(batch, "edit.py"), scratch], check=True)
for side in ("files", "base"):
    shutil.rmtree(os.path.join(batch, side), ignore_errors=True); os.makedirs(os.path.join(batch, side))
sha = lambda data: hashlib.sha256(data).hexdigest()
rows = {}
for index, rel in enumerate(rels):
    key = "%02d%s" % (index, os.path.splitext(rel)[1])
    base = open(os.path.join(live, rel), "rb").read() if os.path.exists(os.path.join(live, rel)) else None
    new = open(os.path.join(scratch, rel), "rb").read()
    if base == new: sys.exit("unchanged by the edit: " + rel)
    open(os.path.join(batch, "files", key), "wb").write(new)
    if base is not None: open(os.path.join(batch, "base", key), "wb").write(base)
    rows[rel] = {"file": key, "base": None if base is None else sha(base), "new": sha(new)}
json.dump(rows, open(os.path.join(batch, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf-8").read()
doc = open(os.path.join(batch, "edit.py"), encoding="utf-8").read().split('"""')[1].split(":")[0]
template = template.replace(template.split('"""')[1].splitlines()[0], "🪟 Staged batch `%s` (lane winps): %s" % (name, doc.strip()))
open(os.path.join(batch, "stage.py"), "w", encoding="utf-8").write(template)
shutil.rmtree(scratch, ignore_errors=True)
print(name, len(rows), "rows,", sum(1 for r in rows.values() if r["base"] is None), "new")
