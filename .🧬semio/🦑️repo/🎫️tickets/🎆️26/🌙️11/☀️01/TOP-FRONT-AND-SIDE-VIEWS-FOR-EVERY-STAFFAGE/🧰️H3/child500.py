import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *
doc = load("running-child"); a = doc["asset"]
rep = a["representations"]; el = rep["500"]["geometry"]["elevation"]
m100 = {s["id"]: s for s in rep["100"]["geometry"]["elevation"]}
MAP = {"sil-arm-r": ("arm-skin-r", .055), "sil-forearm-r": ("forearm-r", .055), "sil-shank-l": ("shank-l", .05), "sil-shank-r": ("shank-r", .05)}
for s in el:
    if s["id"] in MAP and s.get("kind") in ("poly", "capsule"):
        if s["kind"] == "capsule": s.update(kind="poly", points=[[0, 0], [s["x1"] + s["r1"], 0]])
        print(s)
        src, r = MAP[s["id"]]; L = max(p[0] for p in s["points"])
        old = {k: s[k] for k in ("id", "bone", "z")}
        s.clear()
        s.update({"kind": "capsule", "x0": 0.0, "y0": 0.0, "x1": round(L - r * .85, 4), "y1": 0.0, "r0": r, "r1": round(r * .85, 4), **old, "role": "body", "filled": True, "tone": m100[src]["tone"], **({"regions": ["leg-skin", "socks"]} if "shank" in s["id"] else {"region": m100[src]["region"]})})
save("running-child", doc)
