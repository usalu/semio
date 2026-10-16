"""🔁 Duplicate-mount scan over every touched crate and companion: a source file reached by more than one mount edge in one closure, or compiled by both a library and its companion."""
import collections, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftclib import *
import ftcunion as U

T = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "ftc-touched.txt")
pk = []
for line in open(T, encoding="utf-8"):
    line = line.strip()
    if line.endswith("Cargo.toml") and os.path.exists(os.path.join(ROOT, line)):
        m = re.search(r'^name\s*=\s*"([^"]+)"', open(os.path.join(ROOT, line), encoding="utf-8").read(), re.M)
        if m and m.group(1) not in pk:
            pk.append(m.group(1))
pk += [a for a in sys.argv[1:] if a not in pk]
src = Source(None)
mans = Manifests(src)
mans.build_index()
owner = collections.defaultdict(list)
bad = 0
for p in pk:
    man = mans.manifest(p, src)
    if man is None:
        print("NOMANIFEST", p)
        continue
    files, code, missing = U.reach(src, mans.lib_rel(man), man["dir"])
    edges = collections.Counter()
    for rel in code:
        text = files[rel].decode("utf-8", "replace")
        d = os.path.dirname(rel)
        for m in U.PATH_RX.finditer(text):
            edges[U.norm(os.path.join(d, m.group(1)))] += 1
        for m in U.INC_CODE_RX.finditer(text):
            edges[U.norm(os.path.join(d, m.group(1)))] += 1
    for t, n in edges.items():
        if n > 1:
            bad += 1
            print("MULTI-EDGE", p, n, t)
    for rel in code:
        owner[rel].append(p)
    if missing:
        bad += 1
        print("MISSING", p, missing[:3])
for rel, ps in owner.items():
    if len(ps) > 1:
        bad += 1
        print("SHARED-CODE", rel, ps)
print(f"scanned {len(pk)} crates, {len(owner)} code files, findings {bad}")
