import os, sys, collections
F = sys.argv[1]
files = [l for l in open(F + "/🗑️generated/bd8-all-files.txt", encoding="utf-8").read().split("\n") if l]
by_tail = {}
for f in files:
    parts = f.split("/")
    for k in (1, 2, 3):
        by_tail.setdefault("/".join(parts[-k:]), []).append(f)
rows = [l.split("\t") for l in open(F + "/🗑️generated/bd8-ts-enoent.txt", encoding="utf-8").read().split("\n") if l]
out = collections.defaultdict(list)
for src, line, rel, tgt in rows:
    parts = tgt.split("/")
    cand = []
    for k in (3, 2):
        c = by_tail.get("/".join(parts[-k:]), [])
        c = [x for x in c if x != tgt]
        if c: cand = c; break
    gen = any("generated" in p for p in parts)
    kind = "generated" if gen else ("moved" if cand and len(cand) <= 3 else ("ambiguous" if cand else "missing"))
    out[kind].append((src, line, rel, tgt, cand[:3]))
for kind, items in out.items():
    print(f"=== {kind}: {len(items)}")
    for src, line, rel, tgt, cand in items:
        print(f"  {src.split('/')[-4:-1]}:{line}  {rel}  ->  {tgt[-120:]}")
        for c in cand: print(f"        cand {c[-140:]}")
