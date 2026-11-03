import sys, json
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *
a = load("seated-office-worker")["asset"]
L = []
for sc in ("50", "500"):
    L.append("== plan " + sc)
    for s in a["representations"][sc]["geometry"]["plan"]:
        if s["id"].startswith(("chair", "desk", "laptop", "sho", "head")):
            L.append(json.dumps({k: v for k, v in s.items() if k not in ("role",)}, ensure_ascii=False, separators=(",", ":")))
L.append("== elev 50 chair/desk")
for s in a["representations"]["50"]["geometry"]["elevation"]:
    if s["id"].startswith(("chair-seat-cushion", "chair-backrest-shell", "chair-column", "desk", "laptop-base", "laptop-lid", "chair-base", "chair-hub")):
        L.append(json.dumps({k: v for k, v in s.items() if k not in ("role",)}, ensure_ascii=False, separators=(",", ":"))[:420])
open(r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\H3\office-dump.txt", "w", encoding="utf8").write("\n".join(L))
print("ok")
