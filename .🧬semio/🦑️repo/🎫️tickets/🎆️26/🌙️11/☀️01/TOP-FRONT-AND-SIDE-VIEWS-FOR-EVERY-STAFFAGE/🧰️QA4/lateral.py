import json, os
H = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"


def edit(asset, fn):
    p = H + asset + "/🔣️.json"
    doc = json.load(open(p, encoding="utf-8"))
    fn(doc["asset"])
    tmp = p + ".qa4tmp"
    open(tmp, "w", encoding="utf-8", newline="\n").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
    os.replace(tmp, p)


ARMS = {"upper-arm-l": 30, "forearm-l": -30, "upper-arm-r": -30, "forearm-r": 30}
NEW = {"upper-arm": -30, "forearm": 80, "hand": 0}
OLD = {"upper-arm": -12, "forearm": 105, "hand": 10}


def pivots(a, zs):
    for b in a["rig"]["bones"]:
        if b["id"] in zs:
            b["z"] = zs[b["id"]]


def jogger(a):
    """Hands on the hips (grid search `hips.ts`): elbows back and out, forearms folded to the hips. Run once; the clip keys keep their small offsets from the stand pose."""
    pivots(a, {"upper-arm-l": 0.175, "upper-arm-r": -0.175})
    for p in a["poses"]:
        if p["id"] == "stretch-stand":
            for bone, v in ARMS.items():
                p["bones"][bone]["abductionDeg"] = v
            for s in ("l", "r"):
                for part, ang in NEW.items():
                    p["bones"][f"{part}-{s}"]["angleDeg"] = ang
    for c in a["clips"]:
        if c["id"] != "stretch":
            continue
        for k in c["keys"]:
            if k.get("pose") == "stretch-stand" and "bones" in k:
                for bone, v in ARMS.items():
                    if bone in k["bones"]:
                        k["bones"][bone]["abductionDeg"] = v
                for s in ("l", "r"):
                    for part, ang in NEW.items():
                        o = k["bones"].get(f"{part}-{s}")
                        if o is not None and "angleDeg" in o and abs(o["angleDeg"] - OLD[part]) < 15:
                            o["angleDeg"] = round(ang + (o["angleDeg"] - OLD[part]), 4)


def cyclist(a):
    pivots(a, {"thigh-l": 0.095, "thigh-r": -0.095})
    for p in a["poses"]:
        if p["id"] == "dismounted-astride":
            p["bones"]["thigh-l"]["abductionDeg"] = 9
            p["bones"]["thigh-r"]["abductionDeg"] = -9


edit("jogger", jogger)
edit("cyclist", cyclist)
