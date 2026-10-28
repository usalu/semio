#!/usr/bin/env python3
"""🔁 Puts a MIRROR into the state `base` and applies the batch again (after a rebuild of the batch): `python3 -I mirror-apply.py <batch dir> <mirror root>`. Refuses a root that is not below a `/sm/` mirror directory."""
import json, os, shutil, subprocess, sys
batch, root = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
if "/sm/" not in root: sys.exit(f"refusing: {root} is not a mirror")
rows = json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8"))
for rel, row in rows.items():
    target = os.path.join(root, rel)
    if row["base"] is None:
        if os.path.exists(target): os.remove(target)
    else:
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(os.path.join(batch, "base", row["file"]), target)
sys.exit(subprocess.run([sys.executable, "-I", os.path.join(batch, "stage.py"), "apply", "--root", root]).returncode)
