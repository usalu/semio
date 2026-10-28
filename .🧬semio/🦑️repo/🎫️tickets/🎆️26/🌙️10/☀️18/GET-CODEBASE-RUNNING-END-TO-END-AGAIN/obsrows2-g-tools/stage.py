#!/usr/bin/env python3
"""[obsrows2-g] anchored text edits for the obsrows2 add-on batch (adapted from red-g-tools/stage.py).
usage: stage.py make   <batch> --base-root DIR   edits.json from <batch>/spec.json against DIR (the tree AFTER obsrows-g and red-g inferfix); records base/applied sha256 per file
       stage.py check  <batch> [--root DIR]       one line per file: base | applied | DRIFT-ANCHORED (live differs, every anchor still unique: apply --rebase) | DRIFT-BROKEN | MISSING
       stage.py apply  <batch> [--root DIR] [--rebase]   all-or-nothing; refuses on DRIFT unless --rebase and every anchor is unique
       stage.py revert <batch> [--root DIR]       reverse replacement of the new texts, all-or-nothing
Root is the repository root (default: the live checkout). ORDER: obsrows-g batch, red-g inferfix, then this batch (anchors contain text those batches add)."""
import hashlib, json, os, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = os.path.abspath(os.path.join(HERE, *[".."] * 8))
sha = lambda data: hashlib.sha256(data).hexdigest()
def read(path):
    with open(path, "rb") as handle: return handle.read()
def write_atomic(path, data):
    handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".obsrows2-")
    with os.fdopen(handle, "wb") as out: out.write(data)
    os.chmod(temporary, os.stat(path).st_mode & 0o7777); os.replace(temporary, path)
def forward(text, edits):
    for edit in edits:
        if text.count(edit["old"]) != 1: return None, edit["old"][:80]
        text = text.replace(edit["old"], edit["new"])
    return text, None
def backward(text, edits):
    for edit in reversed(edits):
        if text.count(edit["new"]) != 1: return None, edit["new"][:80]
        text = text.replace(edit["new"], edit["old"])
    return text, None
def load(batch):
    with open(os.path.join(HERE, "staged", batch, "edits.json"), encoding="utf8") as handle: return json.load(handle)
def state(root, entry):
    path = os.path.join(root, entry["file"])
    if not os.path.exists(path): return "MISSING", None
    data = read(path); digest = sha(data)
    if digest == entry["applied"]: return "applied", data
    if digest == entry["base"]: return "base", data
    text = data.decode("utf8")
    if forward(text, entry["edits"])[0] is not None: return "DRIFT-ANCHORED", data
    return ("DRIFT-APPLIED" if backward(text, entry["edits"])[0] is not None else "DRIFT-BROKEN"), data
def make(batch, base_root):
    folder = os.path.join(HERE, "staged", batch)
    spec = json.load(open(os.path.join(folder, "spec.json"), encoding="utf8"))
    out = {"batch": batch, "files": []}
    for file, edits in spec["files"].items():
        data = read(os.path.join(base_root, file)); result, bad = forward(data.decode("utf8"), edits)
        if result is None: sys.exit("anchor not unique or missing in %s: %r" % (file, bad))
        if backward(result, edits)[0] != data.decode("utf8"): sys.exit("new text not unique (revert would be ambiguous) in " + file)
        out["files"].append({"file": file, "base": sha(data), "applied": sha(result.encode("utf8")), "edits": edits})
    json.dump(out, open(os.path.join(folder, "edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("%d files staged for %s" % (len(out["files"]), batch))
def run(command, batch, root, rebase):
    rows = [(entry, *state(root, entry)) for entry in load(batch)["files"]]
    if command == "check":
        for entry, status, _ in rows: print(status, entry["file"].replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/", "R/"))
        return 1 if any(status.startswith(("DRIFT-BROKEN", "MISSING")) for _, status, _ in rows) else 0
    plan = []
    for entry, status, data in rows:
        path = os.path.join(root, entry["file"])
        if command == "apply":
            if status == "applied": continue
            if status not in ("base", "DRIFT-ANCHORED") or (status == "DRIFT-ANCHORED" and not rebase): sys.exit("refuse: %s is %s" % (entry["file"], status))
            result, bad = forward(data.decode("utf8"), entry["edits"])
            if result is None: sys.exit("refuse: anchor %r not unique in %s" % (bad, entry["file"]))
            plan.append((path, result.encode("utf8")))
        else:
            if status == "base": continue
            result, bad = backward(data.decode("utf8"), entry["edits"])
            if result is None: sys.exit("refuse: %s is %s and its new text %r is not unique" % (entry["file"], status, bad))
            plan.append((path, result.encode("utf8")))
    for path, content in plan: write_atomic(path, content)
    print("%s: %d files changed" % (command, len(plan))); return 0
if __name__ == "__main__":
    arguments = sys.argv[1:]; command, batch = arguments[0], arguments[1]
    option = lambda name: os.path.abspath(arguments[arguments.index(name) + 1]) if name in arguments else None
    if command == "make": make(batch, option("--base-root") or sys.exit("--base-root required"))
    else: sys.exit(run(command, batch, option("--root") or LIVE, "--rebase" in arguments))
