#!/usr/bin/env python3
"""rev3-g staged-batch tool: anchored exactly-once edits + new files. Usage: batch.py <dir> check|apply|revert|status [root] [--accept-drift]
<dir>/bases.json = sha256 of every touched file at staging: a file with a pending edit whose sha moved is reported as `drift` (check/status exit 3, apply refuses unless --accept-drift, which is for files an earlier batch of the landing order legitimately changed).
<dir>/edits.json = {"edits":[{"file","old","new"}], "newFiles":[{"path","from"}]} (paths relative to the repo root; "from" relative to <dir>)."""
import hashlib, json, os, sys

def sha(b): return hashlib.sha256(b).hexdigest()

def main():
    flags = [a for a in sys.argv[1:] if a.startswith("--")]; args = [a for a in sys.argv[1:] if not a.startswith("--")]
    directory, mode = args[0], args[1]
    root = args[2] if len(args) > 2 else os.getcwd()
    spec = json.load(open(os.path.join(directory, "edits.json"), encoding="utf-8"))
    state, problems = [], []
    for edit in spec["edits"]:
        path = os.path.join(root, edit["file"])
        text = open(path, encoding="utf-8").read()
        before, after = text.count(edit["old"]), text.count(edit["new"])
        status = "after" if after == 1 else "before" if before == 1 and after == 0 else "ambiguous:%d/%d" % (before, after)
        state.append((edit, status))
    for item in spec.get("newFiles", []):
        target = os.path.join(root, item["path"]); source = open(os.path.join(directory, item["from"]), "rb").read()
        state.append((item, "after" if os.path.exists(target) and open(target, "rb").read() == source else "absent" if not os.path.exists(target) else "different"))
    for item, status in state:
        label = item.get("file") or item["path"]
        print("%-10s %s" % (status, label.split("/")[-2] + "/" + label.split("/")[-1]))
    bases_path = os.path.join(directory, "bases.json"); drift = []
    if os.path.exists(bases_path):
        bases = json.load(open(bases_path, encoding="utf-8"))
        for rel in sorted({item["file"] for item, status in state if "file" in item and status == "before"}):
            now = sha(open(os.path.join(root, rel), "rb").read())
            if bases.get(rel) != now: drift.append(rel); print("%-10s %s (staged on %s, live %s)" % ("drift", rel.split("/")[-2] + "/" + rel.split("/")[-1], (bases.get(rel) or "none")[:10], now[:10]))
    if mode in ("check", "status"): sys.exit(3 if drift else 0)
    if mode == "apply":
        if any(s.startswith(("ambiguous", "different")) for _, s in state): sys.exit("refused: an anchor is not unique or a new file differs")
        if drift and "--accept-drift" not in flags: sys.exit("refused: %d touched files moved since staging (bases.json); rebase or pass --accept-drift" % len(drift))
        for item, status in state:
            if "file" in item and status == "before":
                path = os.path.join(root, item["file"]); text = open(path, encoding="utf-8").read()
                open(path, "w", encoding="utf-8").write(text.replace(item["old"], item["new"], 1))
            elif "path" in item and status == "absent":
                target = os.path.join(root, item["path"]); os.makedirs(os.path.dirname(target), exist_ok=True)
                open(target, "wb").write(open(os.path.join(directory, item["from"]), "rb").read())
        print("applied")
    elif mode == "revert":
        for item, status in state:
            if "file" in item and status == "after":
                path = os.path.join(root, item["file"]); text = open(path, encoding="utf-8").read()
                if text.count(item["new"]) != 1: sys.exit("refused: new text not unique in " + item["file"])
                open(path, "w", encoding="utf-8").write(text.replace(item["new"], item["old"], 1))
            elif "path" in item and status == "after": os.remove(os.path.join(root, item["path"]))
        print("reverted")

main()
