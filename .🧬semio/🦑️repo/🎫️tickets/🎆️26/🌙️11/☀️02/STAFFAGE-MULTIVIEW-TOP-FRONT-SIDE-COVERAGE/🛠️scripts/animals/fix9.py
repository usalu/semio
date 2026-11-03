import json
C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/"
def edit(slug, rules):
    p = C + slug + "/🔣️.json"
    d = json.load(open(p, encoding="utf-8"))
    n = 0
    for sc, r in d["asset"]["representations"].items():
        for s in r.get("geometry", {}).get("elevation", []):
            z = s.get("solid")
            if z and z["kind"] == "extruded" and (z["z0"], z["z1"]) == (-0.03, 0.03) and (sc, s["id"]) in rules:
                z["z0"], z["z1"] = rules[(sc, s["id"])]; n += 1
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    print(slug, n)
edit("pigeon", {("50", "wing-bar"): (-0.075, -0.045), ("50", "primary-1"): (-0.075, -0.05), ("50", "primary-2"): (-0.075, -0.05), ("50", "primary-3"): (-0.075, -0.05), ("100", "near-shoulder"): (-0.072, -0.032), ("100", "far-shoulder"): (0.032, 0.072)})
edit("mallard-duck", {("50", "speculum"): (-0.078, -0.04), ("50", "primary-lines"): (-0.078, -0.04), ("100", "wing-line"): (-0.078, -0.04), ("200", "wing"): (-0.078, -0.04)})
