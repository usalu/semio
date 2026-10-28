#!/usr/bin/env python3
"""🧭️ leaseord stage 2: the shared whole-build pins (graph snapshot, Rust compiler code) become the unordered class `pin` (rank 0), with three laws. Based on the live files after staged/ landed.
Usage: python3 -I stage.py <root> check|apply|revert   |   python3 -I stage.py <root> make <work-root>   (make: rebuild files/ and base/ from <work-root>, bases from <root>).
apply: every file must hold its base (or already its new) hash, then all are replaced atomically; exit 2 on drift. revert: every file must hold its new hash."""
import hashlib, json, os, shutil, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
RELS = [
 "🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json",
 "🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🟦️.ts",
 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts",
]

def digest(path):
    if not os.path.exists(path): return None
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""): h.update(chunk)
    return h.hexdigest()

def atomic(target, data):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".stage-")
    with os.fdopen(fd, "wb") as f: f.write(data)
    os.replace(tmp, target)

def make(root, work):
    rows = {}
    for d in ("files", "base"): os.makedirs(os.path.join(HERE, d), exist_ok=True)
    for index, rel in enumerate(RELS):
        name = "%02d%s" % (index, os.path.splitext(rel)[1])
        live, new = os.path.join(root, rel), os.path.join(work, rel)
        shutil.copyfile(new, os.path.join(HERE, "files", name))
        if os.path.exists(live): shutil.copyfile(live, os.path.join(HERE, "base", name))
        rows[rel] = {"file": name, "base": digest(live), "new": digest(new)}
    json.dump(rows, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    for rel, row in rows.items(): print(row["file"], (row["base"] or "absent")[:10], (row["new"] or "absent")[:10], rel[-60:])

def state(root):
    rows = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf8"))
    for rel, row in rows.items():
        live = digest(os.path.join(root, rel))
        yield rel, row, live, "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"

def main():
    root, verb = os.path.abspath(sys.argv[1]), sys.argv[2]
    if verb == "make": make(root, os.path.abspath(sys.argv[3])); return
    rows = list(state(root))
    if verb == "check":
        for rel, _row, live, kind in rows: print(kind, rel[-70:], (live or "absent")[:12])
        sys.exit(0 if all(kind in ("base", "applied") for *_x, kind in rows) else 1)
    if verb == "apply":
        bad = [r for r in rows if r[3] == "drifted"]
        for rel, row, live, _k in bad: print("DRIFTED", rel[-70:], "live", (live or "absent")[:12], "base", (row["base"] or "absent")[:12])
        if bad: sys.exit(2)
        for rel, row, _l, kind in rows:
            if kind == "applied": continue
            atomic(os.path.join(root, rel), open(os.path.join(HERE, "files", row["file"]), "rb").read()); print("applied", rel[-70:])
        return
    if verb == "revert":
        bad = [r for r in rows if r[3] == "drifted"]
        for rel, _row, _l, _k in bad: print("DRIFTED", rel[-70:])
        if bad: sys.exit(2)
        for rel, row, _l, kind in rows:
            if kind != "applied": continue
            atomic(os.path.join(root, rel), open(os.path.join(HERE, "base", row["file"]), "rb").read()); print("reverted", rel[-70:])
        return
    sys.exit("verb: check|apply|revert|make")

main()
