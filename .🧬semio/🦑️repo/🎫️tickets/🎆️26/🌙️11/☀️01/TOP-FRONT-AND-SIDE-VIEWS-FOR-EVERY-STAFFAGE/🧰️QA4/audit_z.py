import json, sys
H = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
ASSETS = ["child-on-kick-scooter", "courier-with-hand-truck", "cyclist", "elderly-with-cane", "jogger", "kneeling-gardener", "leaning-person"]


def regions(s):
    return set(s.get("regions") or ([s["region"]] if "region" in s else []))


def main(fix=False):
    bad = 0
    for a in ASSETS:
        d = json.load(open(H + a + "/🔣️.json", encoding="utf-8"))["asset"]
        bones = {b["id"]: b for b in d["rig"]["bones"]}
        for sc, rep in d["representations"].items():
            if rep["status"] != "authored":
                continue
            for view in ("elevation", "front", "plan"):
                shapes = rep["geometry"][view]
                for u in shapes:
                    unfilled = (not u.get("filled", True)) or (u["kind"] == "poly" and not u.get("closed", True))
                    if not unfilled or u.get("role") == "ground":
                        continue
                    ur = regions(u)
                    cut = []
                    for f in shapes:
                        if f is u or not f.get("filled", True) or f["role"] == "ground":
                            continue
                        if f["kind"] == "poly" and not f.get("closed", True):
                            continue
                        near = f["bone"] == u["bone"] or bones[f["bone"]]["parent"] == u["bone"] or bones[u["bone"]]["parent"] == f["bone"]
                        if near and (regions(f) & ur) and f["z"] > u["z"]:
                            cut.append(f["id"])
                    if cut:
                        bad += 1
                        print(f"{a} 1:{sc} {view} {u['id']} z{u['z']} region {sorted(ur)} cut by {cut[:4]}")
    print("violations", bad)


main()
