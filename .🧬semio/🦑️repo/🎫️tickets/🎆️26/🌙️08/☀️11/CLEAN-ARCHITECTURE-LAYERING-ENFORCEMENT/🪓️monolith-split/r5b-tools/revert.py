#!/usr/bin/env python3
"""Reverts one `new crates ... --backup=B` apply from its log: restores updated and deleted files from the backup, deletes created files, removes the owner rows."""
import sys, os, re, json, shutil
ROOT = "/Users/ueli/Documents/semio"
log, backup = sys.argv[1], sys.argv[2]
go = len(sys.argv) > 3 and sys.argv[3] == "go"
created, restored = [], []
for line in open(log, errors="replace"):
    m = re.match(r"^  ([+~-]) (.+)$", line.rstrip("\n"))
    if not m: continue
    kind, path = m.groups()
    if kind == "+": created.append(path)
    else: restored.append((kind, path))
missing = [p for k, p in restored if not os.path.exists(os.path.join(backup, p))]
print("created", len(created), "restore", len(restored), "backup missing", len(missing))
for p in missing[:10]: print("  missing backup", p)
rows = json.load(open(backup + "-owner-rows.json"))
if go:
    assert not missing
    for kind, p in restored:
        src = os.path.join(backup, p); dst = os.path.join(ROOT, p)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copyfile(src, dst)
    for p in created:
        f = os.path.join(ROOT, p)
        if os.path.exists(f): os.remove(f)
        d = os.path.dirname(f)
        while d.startswith(ROOT + "/") and os.path.isdir(d) and not os.listdir(d):
            os.rmdir(d); d = os.path.dirname(d)
    ob = ROOT + "/📋️build-owners.json"
    data = json.load(open(ob))
    before = len(data["owners"])
    data["owners"] = [o for o in data["owners"] if o["name"] not in set(rows)]
    open(ob, "w").write(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    print("owners", before, "->", len(data["owners"]))
    print("reverted")
