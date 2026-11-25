#!/usr/bin/env python3
"""🗺️ The verdicts of done.txt (last one per asset and current document) with the running batch that owns a failing asset: owners.py"""
import glob, hashlib, os, sys
U = os.path.realpath(os.path.join(os.path.dirname(__file__), "..", ".."))
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
RUNNING = {"MA": "MA", **{f"MH-{k}": f"MH-{k}" for k in range(1, 7)}, "MV-2": "MV-2", "MV-3": "MV-3"}
owner = {}
for b in RUNNING:
    for line in open(f"{U}/🧰️matrix/batches/{b}.tsv", encoding="utf-8"):
        f = line.rstrip("\n").split("\t")
        for x in f[:2]: owner.setdefault(x, b)
for x in ("black-alder", "field-maple", "hawthorn"): owner[x] = "MV-L1"
files = {os.path.basename(os.path.dirname(p)): p for p in glob.glob(f"{S}/*/*/🔣️.json")}
sha = {a: hashlib.sha256(open(p, "rb").read()).hexdigest()[:16] for a, p in files.items()}
last = {}
for line in open(f"{U}/🗑️generated/matrix/VOL/catalog/done.txt", encoding="utf-8"):
    a, s, v, *rest = line.rstrip("\n").split(" ")
    last[a] = (s, v, " ".join(rest))
ok = [a for a in last if last[a][1] == "pass"]
print(f"assets {len(files)}, measured {len(last)}, pass {len(ok)}, pass on the current document {sum(last[a][0] == sha.get(a) for a in ok)}")
for a in sorted(last):
    s, v, m = last[a]
    if v != "pass": print(f"{v} {a} [{owner.get(a, 'no batch')}] {'current' if s == sha[a] else 'document changed'} {m[:int(sys.argv[1]) if len(sys.argv) > 1 else 400]}")
