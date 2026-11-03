import json

C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"


def edit(cat, slug, fn):
    p = C + cat + "/" + slug + "/🔣️.json"
    d = json.load(open(p, encoding="utf-8-sig"))
    fn(d["asset"])
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def fox(a):
    for s in a["representations"]["200"]["geometry"]["front"]:
        if s["id"] in ("gaskin-n", "gaskin-f"):
            s["points"] = [[round(x * 0.96, 5), y] for x, y in s["points"]]


def thermostat(a):
    r = a["representations"]["100"]
    for s in r["geometry"]["front"]:
        if s["id"] == "housing":
            s["x"], s["w"] = -0.0065, 0.02
        if s["id"] == "dial":
            s["solid"]["z0"], s["solid"]["z1"] = -0.0265, -0.0065
    for s in r["geometry"]["plan"]:
        if s["id"] == "housing":
            s["y"], s["h"] = -0.0065, 0.02
        if s["id"] == "dial":
            s["solid"]["z0"], s["solid"]["z1"] = -0.0265, -0.0065
    d = r["decisions"]
    d["omitted"]["en"] = d["omitted"]["en"].replace("the slab is 0.022 m thick instead of 0.015 m", "the slab is 0.020 m thick instead of 0.015 m and the dial stands 0.020 m proud instead of 0.012 m, so that both reach the 0.2 mm feature width and the depth stays within 0.04 m")
    d["omitted"]["de"] = d["omitted"]["de"].replace("die Platte ist 0,022 m statt 0,015 m dick", "die Platte ist 0,020 m statt 0,015 m dick und der Knopf steht 0,020 m statt 0,012 m vor, damit beide die Merkmalsbreite von 0,2 mm erreichen und die Tiefe innerhalb von 0,04 m bleibt")
    if "0.020 m proud" not in d["omitted"]["en"]:
        raise SystemExit("thermostat text not replaced")


def shower(a):
    r = a["representations"]["200"]
    base = {"id": "bracket-top", "bone": "rail", "role": "body", "z": 3.1, "filled": True, "tone": 0.8, "kind": "rect", "region": "rail", "material": "chrome"}
    r["geometry"]["elevation"].append({**base, "x": 0.0, "y": 0.775, "w": 0.05, "h": 0.035})
    r["geometry"]["front"].append({**base, "x": -0.025, "y": 0.775, "w": 0.05, "h": 0.035})
    r["geometry"]["plan"].append({**base, "x": 0.0, "y": -0.025, "w": 0.05, "h": 0.05})


def vac(a):
    r = a["representations"]["500"]
    for v, shapes in r["geometry"].items():
        for s in shapes:
            if s["id"].startswith(("hose-", "wand-", "grip-")):
                rr = 0.04 if s["id"].startswith("hose-") else 0.035
                if s["kind"] == "capsule":
                    s["r0"] = s["r1"] = rr
                if "solid" in s:
                    s["solid"]["z0"], s["solid"]["z1"] = -rr, rr
    t = r["decisions"]["thinElements"]
    t["en"] = "Hose is drawn 0.08 m (0.16 mm) and wand and grip 0.07 m (0.14 mm) wide, about 1.5 times their real diameter of 0.048 to 0.06 m, so that they stay above the 0.1 mm minimum stroke of the print profiles; they are below the 0.2 mm feature width of the office laser profile, which is why the band is a simplified drawing. Their length and joints are true to scale."
    t["de"] = "Schlauch ist 0,08 m (0,16 mm), Rohr und Griff 0,07 m (0,14 mm) breit gezeichnet, etwa das Anderthalbfache ihres wirklichen Durchmessers von 0,048 bis 0,06 m, damit sie über der Mindeststrichstärke von 0,1 mm der Druckprofile bleiben; sie liegen unter der Merkmalsbreite von 0,2 mm des Büro-Laser-Profils, deshalb ist das Band eine vereinfachte Zeichnung. Länge und Gelenke stimmen maßstäblich."


edit("🐕️animals", "red-fox", fox)
edit("🔌️appliances", "room-thermostat", thermostat)
edit("🔌️appliances", "shower-head", shower)
edit("🔌️appliances", "vacuum-cleaner", vac)
print("ok")
