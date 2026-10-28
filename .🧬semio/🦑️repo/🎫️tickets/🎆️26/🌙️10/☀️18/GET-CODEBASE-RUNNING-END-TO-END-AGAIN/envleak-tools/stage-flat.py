#!/usr/bin/env python3
"""Flat staged batches (short paths, the manifest format `mirror.py --batch` also reads).
make  : stage-flat.py make <batch-dir> <work-root> <rel>...   `files/NN.ext` = the new bytes, `base/NN.ext` = the live bytes now (absent = a new file), `manifest.json` = {rel: {file, base, new}} with sha256 or null; plus apply/check/revert wrappers.
apply : every live file must hold its base hash (or already the new hash), then it is replaced atomically; exit 2 on drift.
check : exit 0 when every live file is `base` or `applied`.
revert: every live file must hold the new hash, then it is replaced by the base bytes (or removed).
Paths are relative to the repository root, found by walking up from the batch directory until `🧰️framework` exists."""
import hashlib, json, os, shutil, sys, tempfile

def digest(path):
    if not os.path.exists(path): return None
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""): h.update(chunk)
    return h.hexdigest()

def root_of(start):
    here = os.path.abspath(start)
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and here != "/": here = os.path.dirname(here)
    return here

def atomic(target, data):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".stage-")
    with os.fdopen(fd, "wb") as f: f.write(data)
    os.replace(tmp, target)

def make(batch, work, rels, root):
    rows = {}
    os.makedirs(os.path.join(batch, "files"), exist_ok=True); os.makedirs(os.path.join(batch, "base"), exist_ok=True)
    for index, rel in enumerate(rels):
        ext = os.path.splitext(rel)[1]; name = "%02d%s" % (index, ext)
        live, new = os.path.join(root, rel), os.path.join(work, rel)
        if os.path.exists(new): shutil.copyfile(new, os.path.join(batch, "files", name))
        if os.path.exists(live): shutil.copyfile(live, os.path.join(batch, "base", name))
        rows[rel] = {"file": name, "base": digest(live), "new": digest(new)}
    json.dump(rows, open(os.path.join(batch, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    here = os.path.abspath(__file__)
    for verb in ("apply", "check", "revert"):
        path = os.path.join(batch, verb + ".sh")
        open(path, "w").write('#!/bin/sh\nexec python3 -I "%s" "$(dirname "$0")" %s\n' % (here, verb)); os.chmod(path, 0o755)
    print(json.dumps({k: [v["base"] and v["base"][:10], v["new"] and v["new"][:10]] for k, v in rows.items()}, ensure_ascii=False))

def state(batch, root):
    rows = json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8"))
    out = []
    for rel, row in rows.items():
        live = digest(os.path.join(root, rel))
        out.append((rel, row, live, "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"))
    return out

def main():
    if sys.argv[1] == "make":
        batch, work, rels = os.path.abspath(sys.argv[2]), os.path.abspath(sys.argv[3]), sys.argv[4:]
        make(batch, work, rels, root_of(batch)); return
    batch, verb = os.path.abspath(sys.argv[1]), sys.argv[2]
    root = os.environ.get("STAGE_ROOT") or root_of(batch)
    rows = state(batch, root)
    if verb in ("check", "status"):
        for rel, row, live, kind in rows: print(kind, rel[-80:], (live or "absent")[:12])
        sys.exit(0 if all(k in ("base", "applied") for *_x, k in rows) else 1)
    if verb == "apply":
        bad = [r for r in rows if r[3] == "drifted"]
        if bad:
            for rel, row, live, _k in bad: print("DRIFTED", rel[-80:], "live", (live or "absent")[:12], "base", (row["base"] or "absent")[:12])
            sys.exit(2)
        for rel, row, _l, kind in rows:
            if kind == "applied": continue
            atomic(os.path.join(root, rel), open(os.path.join(batch, "files", row["file"]), "rb").read()); print("applied", rel[-80:])
        return
    if verb == "revert":
        bad = [r for r in rows if r[3] not in ("applied",)]
        if any(r[3] == "drifted" for r in bad):
            for rel, row, live, k in bad:
                if k == "drifted": print("DRIFTED", rel[-80:])
            sys.exit(2)
        for rel, row, _l, kind in rows:
            if kind != "applied": continue
            target = os.path.join(root, rel)
            if row["base"] is None: os.remove(target)
            else: atomic(target, open(os.path.join(batch, "base", row["file"]), "rb").read())
            print("reverted", rel[-80:])
        return
    sys.exit("verb")
main()
