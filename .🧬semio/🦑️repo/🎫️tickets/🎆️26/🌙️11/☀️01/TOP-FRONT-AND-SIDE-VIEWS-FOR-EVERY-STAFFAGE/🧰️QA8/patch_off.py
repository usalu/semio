import os

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
p = os.path.join(A2, "lib.py")
s = open(p, encoding="utf-8").read()

s = s.replace(
    "        r = max((abs(y) for y in ys), default=0.0)\n        prof.append((r5(x), r5(max(r, 0.002))))\n",
    "        r = max((abs(y) for y in ys), default=0.0)\n        if ys:\n            self._rows.append((x, min(ys), max(ys)))\n        prof.append((r5(x), r5(max(r, 0.002))))\n",
)
s = s.replace("    x0, x1 = min(x for x, _ in pts), max(x for x, _ in pts)\n    prof = []", "    x0, x1 = min(x for x, _ in pts), max(x for x, _ in pts)\n    prof = []\n    self._rows = []", 1)
s = s.replace(
    "    prof = self._radial(bone, like)\n    solid = {\"kind\": \"revolved\"",
    "    prof = self._radial(bone, like)\n    rows = self._rows[2:-2] or self._rows\n    off = max(abs(a + b) / 2 / max((b - a) / 2, 1e-3) for _, a, b in rows)\n    if off > 0.6:\n        half = r5(max((b - a) / 2 for _, a, b in self._rows))\n        self.A.evs[(self.scale, like)] = (bone, -half, half)\n        kw.setdefault(\"like\", like)\n        return self.add(id, bone, {\"kind\": \"poly\", \"points\": self._shaped(bone, like, None), \"closed\": True, \"solid\": {\"kind\": \"extruded\", \"z0\": -half, \"z1\": half}}, **kw)\n    solid = {\"kind\": \"revolved\"",
    1,
)
open(p, "w", encoding="utf-8").write(s)
print(s.count("_rows"))
