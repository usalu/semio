#!/usr/bin/env python3
"""🧱️ usage: stage.py <check|apply|revert|status> [--root DIR] : lands (or removes) the rust-mounts batch of pathmod-g.
Every edit is ANCHORED (the old text occurs exactly once) and idempotent; new files come from `files/` and are removed by `revert` only when still byte-equal to the staged copy. Exit 0 ok, 2 drift (an anchor changed or a new file differs), nothing is written on drift. `base` in edits.json is the sha256 of every touched file when the batch was authored (informational: drift is judged by the anchors)."""
import hashlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv: ROOT = sys.argv[sys.argv.index("--root") + 1]
mode = sys.argv[1] if len(sys.argv) > 1 else "check"
if mode not in ("check", "apply", "revert", "status"): sys.exit("usage: stage.py check|apply|revert|status [--root DIR]")
batch = json.load(open(HERE + "/edits.json", encoding="utf-8"))
def path(rel): return os.path.join(ROOT, rel)
def read(rel): return open(path(rel), encoding="utf-8").read()
state, drift, plan = [], [], {}
texts = {}
for e in batch["edits"]:
    text = texts.setdefault(e["file"], read(e["file"]))
    applied, fresh = text.count(e["new"]), text.count(e["old"])
    if applied == 1 and e["new"] != e["old"]: state.append(("APPLIED", e)); continue
    if applied == 0 and fresh == 1: state.append(("ABSENT", e)); continue
    drift.append((e, "old occurs %d, new occurs %d" % (fresh, applied)))
staged_files = []
for base, _, names in os.walk(HERE + "/files"):
    for n in names:
        rel = os.path.relpath(os.path.join(base, n), HERE + "/files")
        staged_files.append(rel)
for rel in sorted(staged_files):
    want = open(HERE + "/files/" + rel, "rb").read()
    if not os.path.exists(path(rel)): state.append(("ABSENT-FILE", {"file": rel, "note": "new file"})); continue
    if open(path(rel), "rb").read() == want: state.append(("APPLIED-FILE", {"file": rel, "note": "new file"})); continue
    drift.append(({"file": rel, "note": "new file"}, "exists and differs from the staged copy"))
for kind, e in state: print("%-12s %s  %s" % (kind, e["file"].split("/")[-2] if "/" in e["file"] else e["file"], e["note"]))
for e, why in drift: print("DRIFT        %s  %s  (%s)" % (e["file"], e["note"], why))
if drift: print("DRIFT: nothing written"); sys.exit(2)
if mode in ("check", "status"): sys.exit(0)
if mode == "apply":
    for kind, e in state:
        if kind == "ABSENT": texts[e["file"]] = texts[e["file"]].replace(e["old"], e["new"], 1)
    for rel, text in texts.items():
        tmp = path(rel) + ".pathmod-g.tmp"; open(tmp, "w", encoding="utf-8").write(text); os.replace(tmp, path(rel))
    for kind, e in state:
        if kind == "ABSENT-FILE":
            os.makedirs(os.path.dirname(path(e["file"])), exist_ok=True)
            tmp = path(e["file"]) + ".pathmod-g.tmp"; open(tmp, "wb").write(open(HERE + "/files/" + e["file"], "rb").read()); os.replace(tmp, path(e["file"]))
else:
    for kind, e in state:
        if kind == "APPLIED": texts[e["file"]] = texts[e["file"]].replace(e["new"], e["old"], 1)
    for rel, text in texts.items():
        tmp = path(rel) + ".pathmod-g.tmp"; open(tmp, "w", encoding="utf-8").write(text); os.replace(tmp, path(rel))
    for kind, e in state:
        if kind == "APPLIED-FILE": os.remove(path(e["file"]))
print(mode, "done")
