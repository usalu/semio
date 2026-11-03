import glob, os, re, json, sys
from collections import defaultdict
H = os.path.dirname(os.path.abspath(__file__))
D = os.path.join(H, "..", "🗑️generated", "QA12", "prev")
S = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"
res = defaultdict(set)
for f in sorted(glob.glob(os.path.join(D, "*.svg"))):
    m = re.match(r"(.+?)--(.+?)--views-dark-line-1-(\d+)\.svg", os.path.basename(f))
    a, q, sc = m[1], m[2], m[3]
    s = open(f, encoding="utf8").read()
    parts = s.split('id="staffage-background"')[1:]
    asset = json.load(open(os.path.join(S, a, "🔣️.json"), encoding="utf8"))["asset"]
    geo = asset["representations"][sc]["geometry"]
    for v, part in zip(("elevation", "front", "plan"), parts):
        outl = set(re.findall(r'id="outline-(.+?)-\d+"', part))
        filled = set(re.findall(r'id="shape-(.+?)"', part))
        for sh in geo[v]:
            if sh["role"] == "ground": continue
            if sh["id"] not in outl:
                res[(a, sc, v)].add(sh["id"] + (" (filled)" if sh["id"] in filled else " (line)"))
for k, v in sorted(res.items()):
    print(k, sorted(v))
