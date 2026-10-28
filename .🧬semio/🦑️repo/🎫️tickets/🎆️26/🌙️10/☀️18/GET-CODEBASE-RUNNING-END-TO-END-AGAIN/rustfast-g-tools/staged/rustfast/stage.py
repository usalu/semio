#!/usr/bin/env python3
"""[rustfast-g] applies, reverts or checks the rustfast batch: anchored edits (batch.json) plus new files (new.json maps flat names in new/ to repository paths).
usage: stage.py check|apply|revert [--root DIR]   (default root: the repository above this file)  exit 0 = requested state reached, 2 = drift (a peer edited an anchor)."""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
mode = args[0]
def find_root(path):
    while path != os.path.dirname(path):
        if os.path.exists(path + "/AGENTS.md"): return path
        path = os.path.dirname(path)
    sys.exit("repository root not found")
root = args[args.index("--root") + 1] if "--root" in args else find_root(HERE)
if not root.endswith("/"): root += "/"
batch = json.load(open(HERE + "/batch.json", encoding="utf8"))
def state(text, edits):
    if all(e["new"] in text for e in edits): return "after"
    if all(text.count(e["old"]) == 1 and e["new"] not in text for e in edits): return "base"
    return "conflict"
failed = 0
for entry in batch["files"]:
    path = root + entry["file"]
    if not os.path.exists(path): print("MISSING   " + entry["file"][-70:]); failed += 1; continue
    text = open(path, encoding="utf8").read(); now = state(text, entry["edits"])
    if mode == "check": print("%-9s %s" % (now, entry["file"][-70:])); failed += now == "conflict"; continue
    want = "after" if mode == "apply" else "base"
    if now == want: print("already %-6s %s" % (want, entry["file"][-60:])); continue
    if now == "conflict": print("CONFLICT  " + entry["file"]); failed += 1; continue
    for e in entry["edits"]:
        text = text.replace(e["old"], e["new"], 1) if mode == "apply" else text.replace(e["new"], e["old"], 1)
    open(path, "w", encoding="utf8").write(text); print("%s  %s" % ("applied" if mode == "apply" else "reverted", entry["file"][-60:]))
names = json.load(open(HERE + "/new.json", encoding="utf8"))
for flat, rel in sorted(names.items()):
    target = root + rel; data = open(HERE + "/new/" + flat, "rb").read(); exists = os.path.exists(target)
    same = exists and open(target, "rb").read() == data
    if mode == "check": print("%-9s %s" % ("after" if same else "base" if not exists else "conflict", rel[-70:])); failed += exists and not same
    elif mode == "apply":
        if exists and not same: print("CONFLICT  new file differs " + rel[-60:]); failed += 1
        elif not exists: os.makedirs(os.path.dirname(target), exist_ok=True); open(target, "wb").write(data)
    elif same: os.remove(target)
print("new files: %d" % len(names))
sys.exit(2 if failed else 0)
