"""📋 [backlog] pairwise file overlap of the staged batches (from 🗑️generated/backlog/files/*.txt). usage: overlap.py"""
import os, itertools, collections
G = os.path.dirname(os.path.dirname(os.path.abspath(__file__))) + "/🗑️generated/backlog/files"
fs = {}
for f in sorted(os.listdir(G)):
    fs[f[:-4]] = set(l.strip() for l in open(f"{G}/{f}") if l.strip())
fs["gram4-shape-keys"] = {"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs"}
names = [n for n in fs if not n.startswith("docs3-") and n != "red-inferfix"]
guide = lambda p: p.endswith("build-guide.md") or p.endswith("README.md")
for a, b in itertools.combinations(names, 2):
    both = fs[a] & fs[b]
    s = sorted(p for p in both if not guide(p)); g = [p for p in both if guide(p)]
    if s or g:
        print(f"{a} x {b}: {len(s)} code, {len(g)} prose", ["/".join(x.split("/")[-2:]) for x in s[:3]])
