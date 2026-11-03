import os

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
p = os.path.join(A2, "lib.py")
s = open(p, encoding="utf-8").read()
s = s.replace(
    "    q = self._shaped(bone, kw.get(\"like\"), world_pts)\n",
    "    q = self._shaped(bone, kw.get(\"like\"), world_pts)\n    if kw.get(\"like\"):\n        self.A.evs[(self.scale, kw[\"like\"])] = (bone, r5(z0), r5(z1))\n",
)
s = s.replace(
    "        s.dp = s.a[\"presentation\"][\"defaultPose\"]",
    "        s.evs = {}\n        s.dp = s.a[\"presentation\"][\"defaultPose\"]",
)
s = s.replace(
    "    rep[\"geometry\"] = {\"elevation\": rep[\"geometry\"][\"elevation\"], \"front\": front, \"plan\": plan}",
    "    for sh in rep[\"geometry\"][\"elevation\"]:\n        hit = A.evs.get((scale, sh[\"id\"]))\n        if hit is not None and hit[0] == sh[\"bone\"]:\n            sh[\"solid\"] = {\"kind\": \"extruded\", \"z0\": hit[1], \"z1\": hit[2]}\n    rep[\"geometry\"] = {\"elevation\": rep[\"geometry\"][\"elevation\"], \"front\": front, \"plan\": plan}",
)
open(p, "w", encoding="utf-8").write(s)
print("evs" in s, s.count("A.evs"))
