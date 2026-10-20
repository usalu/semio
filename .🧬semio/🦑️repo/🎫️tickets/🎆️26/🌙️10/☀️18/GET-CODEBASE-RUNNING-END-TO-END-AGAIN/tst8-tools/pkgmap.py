#!/usr/bin/env python3
import json, os, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
files = subprocess.run(["git", "ls-files", "-z", "-co", "--exclude-standard", "--", "package.json", "**/package.json"], cwd=ROOT, capture_output=True).stdout.decode("utf-8").split("\0")
linked = set(os.listdir(ROOT + "/node_modules/@semio-tech"))
out = {}
for f in files:
    if not f or f.startswith(".🧬semio/") : continue
    try: m = json.load(open(os.path.join(ROOT, f), encoding="utf-8"))
    except Exception: continue
    n = m.get("name", "")
    if not n.startswith("@semio-tech/") or n[12:] in linked: continue
    out[n] = dict(dir=os.path.dirname(os.path.join(ROOT, f)), exports=m.get("exports"), main=m.get("main"))
json.dump(out, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False)
print(len(out), "unlinked workspace packages")
