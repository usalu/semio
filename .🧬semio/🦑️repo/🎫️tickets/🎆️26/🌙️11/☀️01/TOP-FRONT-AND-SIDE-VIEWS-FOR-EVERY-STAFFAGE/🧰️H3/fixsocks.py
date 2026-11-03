import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *
doc = load("running-child")
for s in doc["asset"]["representations"]["500"]["geometry"]["elevation"]:
    if s["id"].startswith("sil-shank") or s["id"] in ("sil-arm-r", "sil-forearm-r"):
        print(s)
        if "shank" in s["id"]:
            s.pop("region", None); s["regions"] = ["leg-skin", "socks"]
save("running-child", doc)
