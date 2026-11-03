import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
r = d["asset"]["representations"]["500"]
for shapes in r["geometry"].values():
    for s in shapes:
        if s["id"].startswith("hose-"):
            if s["kind"] == "capsule": s["r0"] = s["r1"] = 0.03
            if "solid" in s: s["solid"]["z0"], s["solid"]["z1"] = -0.03, 0.03
t = r["decisions"]["thinElements"]
t["en"] = "Hose, wand and grip are drawn 0.06 m (0.12 mm) wide, up to 1.25 times their real diameter of 0.048 to 0.06 m, so that they stay above the 0.1 mm minimum stroke of the print profiles; they are below the 0.2 mm feature width of the office laser profile, which is why the band is a simplified drawing. Their length and joints are true to scale."
t["de"] = "Schlauch, Rohr und Griff sind 0,06 m (0,12 mm) breit gezeichnet, bis zum 1,25-Fachen ihres wirklichen Durchmessers von 0,048 bis 0,06 m, damit sie über der Mindeststrichstärke von 0,1 mm der Druckprofile bleiben; sie liegen unter der Merkmalsbreite von 0,2 mm des Büro-Laser-Profils, deshalb ist das Band eine vereinfachte Zeichnung. Länge und Gelenke stimmen maßstäblich."
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
