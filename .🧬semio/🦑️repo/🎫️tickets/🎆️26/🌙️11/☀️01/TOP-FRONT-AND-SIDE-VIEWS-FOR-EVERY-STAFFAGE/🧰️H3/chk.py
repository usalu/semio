import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from humanlib import *
a = load("seated-reader")["asset"]
for sc in ("100", "500"):
    for s in a["representations"][sc]["geometry"]["front"]:
        if s["id"] in ("hair-bun", "head", "hair"): print(sc, s["id"], s["bone"], pts_bbox(s))
R = Rest(a); print(R.o("head"), R.t("head"))
