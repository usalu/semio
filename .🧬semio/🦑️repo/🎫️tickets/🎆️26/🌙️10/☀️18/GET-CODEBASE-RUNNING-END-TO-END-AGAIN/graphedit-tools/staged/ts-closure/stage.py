#!/usr/bin/env python3
"""🎫 Flat staged batch (manifest.json {rel: {file, base, new}} + files/ + base/), root always given as an argument.
make  : stage.py make <batch-dir> <live-root> <work-root> <rel>...   (copies this script into the batch as its stage.py)
check : stage.py check <root>    exit 0 when every file is `base` or `applied`, 1 otherwise
apply : stage.py apply <root>    every file must be `base` or `applied`, else exit 2 (drift); writes atomically
revert: stage.py revert <root>   every file must be `applied` or `base`, else exit 2; restores base bytes or removes new files
rebase: stage.py rebase <root>   re-derives files/ and base/ of every file named in anchors.json ({rel: [[old, new], ...]}, each old exactly once) from the bytes at <root> now"""
import hashlib, json, os, shutil, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))


def digest(path):
    if not os.path.isfile(path): return None
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""): h.update(chunk)
    return h.hexdigest()


def atomic(target, data):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".stage-")
    with os.fdopen(fd, "wb") as f: f.write(data)
    os.replace(tmp, target)


def make(batch, live, work, rels):
    for sub in ("files", "base"): os.makedirs(os.path.join(batch, sub), exist_ok=True)
    rows = {}
    for index, rel in enumerate(rels):
        name = "%02d%s" % (index, os.path.splitext(rel)[1])
        if os.path.exists(os.path.join(work, rel)): shutil.copyfile(os.path.join(work, rel), os.path.join(batch, "files", name))
        if os.path.exists(os.path.join(live, rel)): shutil.copyfile(os.path.join(live, rel), os.path.join(batch, "base", name))
        rows[rel] = {"file": name, "base": digest(os.path.join(live, rel)), "new": digest(os.path.join(work, rel))}
    with open(os.path.join(batch, "manifest.json"), "w", encoding="utf8") as f: json.dump(rows, f, ensure_ascii=False, indent=1)
    shutil.copyfile(os.path.abspath(__file__), os.path.join(batch, "stage.py"))
    for rel, row in rows.items(): print((row["base"] or "new")[:10], (row["new"] or "gone")[:10], rel[-90:])


def states(root):
    rows = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf8"))
    for rel, row in rows.items():
        live = digest(os.path.join(root, rel))
        yield rel, row, "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"


def rebase(root):
    manifest = os.path.join(HERE, "manifest.json")
    rows, anchors = json.load(open(manifest, encoding="utf8")), json.load(open(os.path.join(HERE, "anchors.json"), encoding="utf8"))
    for rel, pairs in anchors.items():
        text = open(os.path.join(root, rel), encoding="utf8").read()
        if rows[rel]["new"] and digest(os.path.join(root, rel)) == rows[rel]["new"]: print("applied", rel[-90:]); continue
        out = text
        for old, new in pairs:
            if out.count(old) != 1: sys.exit("anchor not unique (%d): %s" % (out.count(old), old[:80]))
            out = out.replace(old, new)
        row = rows[rel]
        open(os.path.join(HERE, "base", row["file"]), "w", encoding="utf8").write(text)
        open(os.path.join(HERE, "files", row["file"]), "w", encoding="utf8").write(out)
        row["base"], row["new"] = hashlib.sha256(text.encode()).hexdigest(), hashlib.sha256(out.encode()).hexdigest()
        print("rebased", rel[-90:])
    with open(manifest, "w", encoding="utf8") as f: json.dump(rows, f, ensure_ascii=False, indent=1)


def main():
    verb = sys.argv[1]
    if verb == "make": return make(os.path.abspath(sys.argv[2]), sys.argv[3], sys.argv[4], sys.argv[5:])
    root = os.path.abspath(sys.argv[2])
    if not os.path.isdir(os.path.join(root, "🧰️framework")): sys.exit("root without 🧰️framework: " + root)
    if verb == "rebase": return rebase(root)
    rows = list(states(root))
    for rel, _row, kind in rows: print(kind, rel[-90:])
    if verb == "check": sys.exit(0 if all(kind != "drifted" for *_x, kind in rows) else 1)
    if any(kind == "drifted" for *_x, kind in rows): sys.exit(2)
    for rel, row, kind in rows:
        target = os.path.join(root, rel)
        if verb == "apply" and kind == "base":
            atomic(target, open(os.path.join(HERE, "files", row["file"]), "rb").read())
        elif verb == "revert" and kind == "applied":
            if row["base"] is None:
                os.remove(target)
                parent = os.path.dirname(target)
                while parent != root and not os.listdir(parent): os.rmdir(parent); parent = os.path.dirname(parent)
            else: atomic(target, open(os.path.join(HERE, "base", row["file"]), "rb").read())
    print(verb, "done")


main()
