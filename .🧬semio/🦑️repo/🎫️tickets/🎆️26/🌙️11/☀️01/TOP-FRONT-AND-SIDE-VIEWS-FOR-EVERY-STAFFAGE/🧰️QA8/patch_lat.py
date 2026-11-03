import os, re

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
LAT = {
    "gen_duck.py": {"neck-1": 0.043, "neck-2": 0.034, "neck-3": 0.031, "neck-lower": 0.043, "neck-upper": 0.032, "body": 0.098},
    "gen_pigeon.py": {"neck-1": 0.034, "neck-2": 0.03, "neck-3": 0.028, "neck": 0.032},
    "gen_deer.py": {"neck": 0.045},
    "gen_sheep.py": {"neck": 0.085},
    "gen_fox.py": {"brush-root": 0.07, "brush-mid": 0.078, "brush-tip": 0.05, "brush": 0.07, "brush-2": 0.078},
    "gen_squirrel.py": {"rump": 0.06, "chest": 0.05, "neck": 0.032, "head": 0.032, "tail-1": 0.045, "tail-2": 0.05},
}
for name, parts in LAT.items():
    p = os.path.join(A2, name)
    out = []
    for l in open(p, encoding="utf-8").read().split("\n"):
        m = re.match(r'^(\s*[FP]\.rev\("([^"]+)", "[^"]+", )(like=.*)$', l)
        if m and m.group(2) in parts and "lat=" not in l:
            l = "%slat=%s, %s" % (m.group(1), parts[m.group(2)], m.group(3))
        out.append(l)
    open(p, "w", encoding="utf-8").write("\n".join(out))

p = os.path.join(A2, "lib.py")
s = open(p, encoding="utf-8").read()
s = s.replace("def _rev(self, id, bone, like, **kw):", "def _rev(self, id, bone, like, lat=None, **kw):", 1)
s = s.replace(
    "    if off > 0.6:\n        half = r5(max((b - a) / 2 for _, a, b in self._rows))\n",
    "    rmax = max(r for _, r in prof[1:-1])\n    if off > 0.6 or (lat is not None and not 0.8 <= lat / rmax <= 1.25):\n        half = r5(lat if lat is not None else max((b - a) / 2 for _, a, b in self._rows))\n",
    1,
)
open(p, "w", encoding="utf-8").write(s)
print(s.count("lat is not None"))
