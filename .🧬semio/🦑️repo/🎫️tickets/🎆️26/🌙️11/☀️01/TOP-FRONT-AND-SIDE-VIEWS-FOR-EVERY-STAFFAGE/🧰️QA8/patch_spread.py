import os

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
p = os.path.join(A2, "lib.py")
s = open(p, encoding="utf-8").read()
s = s.replace(
    "        s.evs = {}\n",
    "        s.evs = {}\n        own = set()\n        for src in [q[\"bones\"] for q in s.a[\"poses\"]] + [k.get(\"bones\", {}) for c in s.a[\"clips\"] for k in c[\"keys\"]]:\n            for b, v in src.items():\n                if v.get(\"abductionDeg\") or v.get(\"twistDeg\"):\n                    own.add(b)\n        s.spread = set()\n        for b in bones:\n            c = b\n            while c is not None:\n                if c in own:\n                    s.spread.add(b)\n                    break\n                c = bones[c].get(\"parent\")\n",
    1,
)
# the elevation solid is registered only for bones a pose or clip spreads or twists (the validator asks for it there)
s = s.replace(
    '    if kw.get("like"):\n        self.A.evs[(self.scale, kw["like"])] = (bone, r5(z0), r5(z1))\n',
    '    if kw.get("like") and bone in self.A.spread:\n        self.A.evs[(self.scale, kw["like"])] = (bone, r5(z0), r5(z1))\n',
    1,
)
s = s.replace(
    "        self.A.evs[(self.scale, like)] = (bone, -half, half)\n        kw.setdefault",
    "        if bone in self.A.spread:\n            self.A.evs[(self.scale, like)] = (bone, -half, half)\n        kw.setdefault",
    1,
)
open(p, "w", encoding="utf-8").write(s)
print(s.count("A.spread"))
