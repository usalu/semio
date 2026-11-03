import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *
L = []
for a in ("pregnant-person", "runner-with-blade-prosthesis", "running-child", "seated-guitarist", "seated-office-worker", "seated-reader", "skateboarder"):
    d = load(a)["asset"]
    L.append("## " + a)
    for p in d["poses"]:
        if p.get("key"): continue
        ab = {k: (v.get("abductionDeg"), v.get("twistDeg")) for k, v in p["bones"].items() if v.get("abductionDeg") or v.get("twistDeg")}
        L.append(f"P {p['id']}: {p['description']['en']} {ab or ''}")
    for c in d["clips"]:
        L.append(f"C {c['id']}: {c['description']['en']}")
open(r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\H3\texts.txt", "w", encoding="utf8").write("\n".join(L))
