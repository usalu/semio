"""Lists the new folders (repo-relative) of every installed or staged target, one per line, for the taxonomy check and the report.
usage: folders.py [installed|stage] [name...]"""
import json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *

mode = sys.argv[1] if len(sys.argv) > 1 else "installed"
names = sys.argv[2:] or list(TARGETS)
out = []
for name in names:
    t = TARGETS[name]
    if mode == "installed":
        mk = f"{W}/{name}/installed.json"
        if not os.path.exists(mk):
            continue
        files = json.loads(read(mk))["files"]
    else:
        files = []
        sd = stage_dir(name)
        for dp, dn, fn in os.walk(sd):
            for f in fn:
                files.append(os.path.relpath(os.path.join(dp, f), sd))
    for rel in files:
        if rel != t["file"]:
            out.append(os.path.dirname(rel))
for d in sorted(set(out)):
    print(d)
