#!/usr/bin/env python3
"""[obsrows-g] applies, reverts or checks the obsrows-g batch (batch/batch.json anchored edits with alternative variants, batch/new-files.tgz).
usage: stage.py check|apply|revert [--root DIR]    exit 0 when every file ends in the requested state, 1 on a conflict.
An edit has `variants` [{old,new}]: the variant whose `old` is present exactly once is replaced by its `new` (apply), the variant whose `new` is present exactly once is turned back (revert).
A file is `after` when every edit has a variant `new` present, `base` when every edit has a variant `old` present and none `new`, else `conflict` (a peer edited the anchor)."""
import json, os, sys, tarfile
HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]; mode = args[0]
root = args[args.index("--root") + 1] if "--root" in args else "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
batch = json.load(open(HERE + "/batch/batch.json", encoding="utf8"))
def is_after(text, edit): return any(v["new"] in text for v in edit["variants"])
def is_base(text, edit): return not is_after(text, edit) and any(text.count(v["old"]) == 1 for v in edit["variants"])
def state(text, edits):
    if all(is_after(text, e) for e in edits): return "after"
    if all(is_base(text, e) for e in edits): return "base"
    return "conflict"
def convert(text, edits, direction):
    for e in (edits if direction == "apply" else reversed(edits)):
        if direction == "apply":
            if is_after(text, e): continue
            hit = [v for v in e["variants"] if text.count(v["old"]) == 1]
            if not hit: return None
            text = text.replace(hit[0]["old"], hit[0]["new"], 1)
        else:
            hit = [v for v in e["variants"] if text.count(v["new"]) == 1]
            if not hit:
                if any(text.count(v["old"]) == 1 for v in e["variants"]): continue
                return None
            text = text.replace(hit[0]["new"], hit[0]["old"], 1)
    return text
failed = 0
for entry in batch["files"]:
    path = root + entry["file"]
    text = open(path, encoding="utf8").read() if os.path.exists(path) else ""
    now = state(text, entry["edits"])
    if mode == "check":
        print(("%-9s" % now) + entry["file"][-70:]); failed += now == "conflict"; continue
    want = "after" if mode == "apply" else "base"
    if now == want: print("already " + want + "  " + entry["file"][-60:]); continue
    out = convert(text, entry["edits"], mode)
    if out is None: print("CONFLICT " + entry["file"]); failed += 1; continue
    open(path, "w", encoding="utf8").write(out); print(mode + "d  " + entry["file"][-60:])
names = json.load(open(HERE + "/batch/new-files.json", encoding="utf8"))
if mode == "apply":
    with tarfile.open(HERE + "/batch/new-files.tgz") as tar:
        for member in tar.getmembers():
            target = root + member.name; data = tar.extractfile(member).read()
            if os.path.exists(target):
                if open(target, "rb").read() != data: print("CONFLICT new file differs " + member.name[-60:]); failed += 1
                continue
            os.makedirs(os.path.dirname(target), exist_ok=True); open(target, "wb").write(data)
    print("new files present:", len(names))
elif mode == "revert":
    for n in names:
        if os.path.exists(root + n): os.remove(root + n)
    print("new files removed:", len(names))
else:
    print("new files present:", sum(os.path.exists(root + n) for n in names), "of", len(names))
sys.exit(1 if failed else 0)
