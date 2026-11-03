import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
r = d["asset"]["representations"]["500"]
for shapes in r["geometry"].values():
    for s in shapes:
        if s["id"].startswith(("wand-", "grip-")):
            if s["kind"] == "capsule": s["r0"] = s["r1"] = 0.03
            if "solid" in s: s["solid"]["z0"], s["solid"]["z1"] = -0.03, 0.03
t = r["decisions"]["thinElements"]
t["en"] = t["en"].replace("wand and grip 0.07 m (0.14 mm)", "wand and grip 0.06 m (0.12 mm)").replace("about 1.5 times", "up to 1.7 times")
t["de"] = t["de"].replace("Rohr und Griff 0,07 m (0,14 mm)", "Rohr und Griff 0,06 m (0,12 mm)").replace("etwa das Anderthalbfache", "bis zum 1,7-Fachen")
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
