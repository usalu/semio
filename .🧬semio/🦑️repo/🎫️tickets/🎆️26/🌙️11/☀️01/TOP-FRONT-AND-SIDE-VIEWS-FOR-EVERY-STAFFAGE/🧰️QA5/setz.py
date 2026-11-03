import sys
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️H4")
from lib import load, save
for aid in ["sleeping-person","standing-adult","standing-phone-user","tour-guide","walking-commuter","wheelchair-user","worker-carrying-box"]:
    doc = load(aid); a = doc["asset"]
    plan = a["representations"]["50"]["geometry"]["plan"]
    res = {}
    for b in a["rig"]["bones"]:
        bid = b["id"]
        if bid.split("-")[0] in ("arm", "upper") and bid.endswith(("-l", "-r")) and (bid.startswith("arm-") or bid.startswith("upper-arm-")) or bid.startswith("thigh-"):
            sh = [s for s in plan if s["bone"] == bid and "y0" in s]
            if not sh:
                continue
            y = abs(sh[0]["y0"]) * (1 if bid.endswith("-l") else -1)
            if y == 0:
                b.pop("z", None); continue
            b["z"] = round(y, 4); res[bid] = b["z"]
    save(aid, doc); print(aid, res)
