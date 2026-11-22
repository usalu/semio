"""📜 The asset document of the used backrest park bench: texts, materials, rig, anchors, poses, clips, thermal and structural models around the geometry of `author-bench.py`.

`document(g)` takes the namespace of `author-bench.py` (geometry, parts, drawings, volumes, masses) and returns the whole `AssetDocument`.

@see ./author-bench.py — the entry point and the geometry
"""
import math


def document(g):
    """📄 The complete asset document."""
    T, r6 = g["T"], g["r6"]
    M, BONES, BONE = g["M"], g["BONES"], g["BONE"]
    FRAME_X, FOOT_S, Y0 = g["FRAME_X"], g["FOOT_S"], g["Y_TOP_FOOTING"]
    areas = region_areas(g)
    total = g["TOTAL_KG"]
    z_com = g["Z_COM"]

    materials = [
        {"id": "hardwood-greyed", "title": T("Greyed tropical hardwood (Bangkirai type, weathered)", "Vergrautes Tropenhartholz (Bangkirai-Typ, bewittert)"), "emissivity": 0.92, "solarAbsorptivity": 0.58, "densityKgM3": 900, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 17000000000, "strengthPa": 60000000, "source": "authored-estimate", "reference": "en-338-2016"},
        {"id": "robinia-fresh", "title": T("Fresh robinia (replacement slat, unweathered)", "Frische Robinie (Ersatzlatte, unbewittert)"), "emissivity": 0.90, "solarAbsorptivity": 0.52, "densityKgM3": 740, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 13000000000, "strengthPa": 40000000, "source": "authored-estimate", "reference": "wagenfuehr-holzatlas-2007"},
        {"id": "cast-iron-painted", "title": T("Grey cast iron EN-GJL-200, weathered fir-green paint", "Grauguss EN-GJL-200, verwitterter tannengrüner Anstrich"), "emissivity": 0.91, "solarAbsorptivity": 0.88, "densityKgM3": 7200, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 100000000000, "strengthPa": 200000000, "source": "literature", "reference": "en-1561-2011"},
        {"id": "cast-iron-rusted", "title": T("Grey cast iron, paint lost, rusted", "Grauguss, Anstrich verloren, verrostet"), "emissivity": 0.82, "solarAbsorptivity": 0.80, "densityKgM3": 7200, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 100000000000, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "concrete-c20-25", "title": T("Concrete C20/25, weathered", "Beton C20/25, bewittert"), "emissivity": 0.92, "solarAbsorptivity": 0.68, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 30000000000, "strengthPa": 20000000, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "steel-galvanised", "title": T("Hot-dip galvanised steel, weathered", "Feuerverzinkter Stahl, bewittert"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "spray-paint", "title": T("Aerosol spray paint (graffiti)", "Sprühlack (Graffiti)"), "emissivity": 0.90, "solarAbsorptivity": 0.90, "source": "illustrative"},
        {"id": "sticker-residue", "title": T("Torn paper sticker and adhesive residue", "Abgerissener Papieraufkleber mit Kleberresten"), "emissivity": 0.90, "solarAbsorptivity": 0.30, "source": "illustrative"},
        {"id": "moss-algae", "title": T("Moss and green algae film", "Moos- und Grünalgenbelag"), "emissivity": 0.95, "solarAbsorptivity": 0.85, "source": "illustrative"},
        {"id": "chewing-gum", "title": T("Trodden chewing gum", "Festgetretener Kaugummi"), "emissivity": 0.90, "solarAbsorptivity": 0.80, "source": "illustrative"},
    ]

    rig_bones = []
    for b in BONES:
        rb = {k: v for k, v in b.items() if k not in ("o", "a")}
        rig_bones.append(rb)
    rig = {"kind": "structural", "root": "ground", "bones": rig_bones}

    def rel_z(bone, z):
        return r6(z - BONE[bone]["o"][2])

    lift_y = g["rail_top"](-z_com) - 0.020
    anchors = []
    for side, bone in (("r", "end-r"), ("l", "end-l")):
        name = T("right", "rechts") if side == "r" else T("left", "links")
        for foot in ("front", "rear"):
            anchors.append({"id": f"foot-{foot}-{side}", "bone": bone, "x": 0, "y": 0, "z": rel_z(bone, -FOOT_S[foot]), "kind": "support", "support": "pin",
                            "title": T(f"{'Front' if foot == 'front' else 'Rear'} foot, {name['en']}, dowelled to the point footing", f"{'Vorderer' if foot == 'front' else 'Hinterer'} Fuß {name['de']}, auf das Punktfundament gedübelt"),
                            "footprintLengthM": 0.085, "footprintWidthM": 0.06, "frictionCoefficient": 0.6})
        anchors.append({"id": f"lift-{side}", "bone": bone, "x": r6(lift_y - Y0), "y": 0, "z": rel_z(bone, z_com), "kind": "support", "support": "pin",
                        "title": T(f"Lifting point on the seat rail, {name['en']} (round sling of the depot's loader crane)", f"Anschlagpunkt an der Sitzzarge {name['de']} (Rundschlinge des Ladekrans des Bauhofs)")})
    seat_z = BONE["seat"]["o"][2]
    back_z = BONE["back-lower"]["o"][2]
    s_sit = 0.06
    y_sit = g["rail_top"](s_sit) + 0.045 - 0.425
    s_lean = g["post_centre"](0.55) + 0.045 / 2 / g["C15"] + 0.040 * g["C15"]
    for name, x, t in (("a", 0.40, T("first", "erste")), ("b", 1.40, T("second", "zweite")), ("c", 0.90, T("third", "dritte"))):
        anchors.append({"id": f"sit-{name}", "bone": "seat", "x": x, "y": r6(y_sit), "z": r6(-s_sit - seat_z), "kind": "load", "title": T(f"Seat place of the {t['en']} sitter", f"Sitzplatz der {t['de']} sitzenden Person")})

    def sitter_loads(names):
        out = []
        for name, x in (("a", 0.40), ("b", 1.40), ("c", 0.90)):
            if name not in names:
                continue
            out.append({"id": f"sitter-{name}", "kind": "occupant", "title": T(f"Seated adult {name.upper()}: 80 % of 80 kg on the seat", f"Sitzende erwachsene Person {name.upper()}: 80 % von 80 kg auf dem Sitz"), "bone": "seat", "x": x, "y": r6(y_sit), "z": r6(-s_sit - seat_z), "fxN": 0, "fyN": -628, "source": "authored-estimate", "reference": "din-33402-2-2020"})
            out.append({"id": f"lean-{name}", "kind": "occupant", "title": T(f"Back of sitter {name.upper()} leaning on the lower back slat", f"Rücken der Person {name.upper()} an der unteren Rückenlatte"), "bone": "back-lower", "x": x, "y": r6(0.55 - g["BACK_Y"]["back-lower"]), "z": r6(-s_lean - back_z), "fxN": 0, "fyN": 0, "fzN": 100, "source": "authored-estimate", "reference": "en-581-2-2015"})
        return out

    def binding(seat_exp, seat_sun, seat_contact, back_exp, back_sun, back_contact, frame_sun, feet_exp, feet_sun, foot_contact, footing_exp, footing_sun):
        b = {
            "seat-slats-greyed": {"exposedFraction": seat_exp, "sunlitFraction": seat_sun},
            "seat-slat-replaced": {"exposedFraction": seat_exp, "sunlitFraction": seat_sun},
            "back-slats-greyed": {"exposedFraction": back_exp, "sunlitFraction": back_sun},
            "cast-frames": {"exposedFraction": 1.0, "sunlitFraction": frame_sun},
            "feet-rusted": {"exposedFraction": feet_exp, "sunlitFraction": feet_sun},
            "footings": {"exposedFraction": footing_exp, "sunlitFraction": footing_sun, "contact": {"fraction": round(1 - footing_exp, 3), "tempC": 16}},
        }
        if seat_contact:
            for k in ("seat-slats-greyed", "seat-slat-replaced"):
                b[k]["contact"] = {"fraction": seat_contact, "tempC": 31}
        if back_contact:
            b["back-slats-greyed"]["contact"] = {"fraction": back_contact, "tempC": 30}
        if foot_contact:
            b["feet-rusted"]["contact"] = {"fraction": foot_contact, "tempC": 24}
        return {"regions": b, "emitters": {}}

    feet = ["foot-front-r", "foot-rear-r", "foot-front-l", "foot-rear-l"]
    service = binding(1.0, 0.9, 0, 1.0, 0.55, 0, 0.45, 0.7, 0.35, 0.3, 0.12, 0.08)
    poses = [
        {"id": "in-service", "title": T("In service", "Im Einsatz"), "description": T("The bench as it stands in the Volkspark after about twenty years: cast frames dowelled to their two point footings, whose top edges stand 25 mm proud of the eroded water-bound gravel; empty, in the afternoon sun.", "Die Bank, wie sie nach etwa zwanzig Jahren im Volkspark steht: Gussseitenteile auf ihre zwei Punktfundamente gedübelt, deren Oberkanten 25 mm über der ausgetretenen wassergebundenen Decke stehen; leer, in der Nachmittagssonne."), "bones": {}, "ground": "authored", "contacts": feet, "thermal": service, "loads": []},
        {"id": "maintenance", "title": T("Lifted out by the depot", "Vom Bauhof ausgehoben"), "description": T("The depot has taken the nuts off the four anchor dowels and lifts the whole bench 300 mm on two round slings choked around the seat rails, hooked to the loader crane above the centre of mass; the footings with their dowels stay in the ground.", "Der Bauhof hat die Muttern der vier Ankerdübel gelöst und hebt die ganze Bank an zwei um die Sitzzargen geschnürten Rundschlingen am Ladekran über dem Schwerpunkt 300 mm an; die Fundamente mit ihren Dübeln bleiben im Boden."), "bones": {"bench": {"dy": 0.30}}, "ground": "authored", "contacts": ["lift-r", "lift-l"], "thermal": binding(1.0, 0.9, 0, 1.0, 0.55, 0, 0.45, 1.0, 0.35, 0, 0.14, 0.12), "loads": []},
        {"id": "in-use", "title": T("Three sitters", "Drei Sitzende"), "description": T("Three adults sit side by side, each with 80 % of 80 kg on the seat and leaning on the lower back slat; their bodies cover and shade part of the slats.", "Drei Erwachsene sitzen nebeneinander, jede Person mit 80 % von 80 kg auf dem Sitz und an die untere Rückenlatte gelehnt; ihre Körper bedecken und beschatten einen Teil der Latten."), "bones": {}, "ground": "authored", "contacts": feet, "thermal": binding(0.62, 0.35, 0.38, 0.8, 0.25, 0.2, 0.4, 0.7, 0.2, 0.3, 0.12, 0.05), "loads": sitter_loads("abc")},
        {"id": "in-use-one", "key": True, "title": T("One sitter (key)", "Eine sitzende Person (Schlüssel)"), "description": T("Key of the sitting-down clip: the first sitter on the right place.", "Schlüssel des Hinsetz-Clips: die erste Person auf dem rechten Platz."), "bones": {}, "ground": "authored", "contacts": feet, "thermal": binding(0.87, 0.72, 0.13, 0.93, 0.45, 0.07, 0.43, 0.7, 0.3, 0.3, 0.12, 0.07), "loads": sitter_loads("a")},
        {"id": "in-use-two", "key": True, "title": T("Two sitters (key)", "Zwei sitzende Personen (Schlüssel)"), "description": T("Key of the sitting-down clip: the second sitter on the left place, the middle still free.", "Schlüssel des Hinsetz-Clips: die zweite Person auf dem linken Platz, die Mitte noch frei."), "bones": {}, "ground": "authored", "contacts": feet, "thermal": binding(0.75, 0.53, 0.25, 0.87, 0.35, 0.13, 0.42, 0.7, 0.25, 0.3, 0.12, 0.06), "loads": sitter_loads("ab")},
    ]

    clips = [
        {"id": "sitting-down", "title": T("Three people sit down", "Drei Personen setzen sich"), "description": T("One after the other three adults sit down: first on the right, then on the left, last in the middle; the bench stays on its four dowelled feet.", "Nacheinander setzen sich drei Erwachsene: zuerst rechts, dann links, zuletzt in die Mitte; die Bank bleibt auf ihren vier gedübelten Füßen."),
         "durationS": 10, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 2.5, "pose": "in-use-one", "ease": "smooth"}, {"t": 5.0, "pose": "in-use-two", "ease": "smooth"}, {"t": 7.5, "pose": "in-use", "ease": "smooth"}, {"t": 10, "pose": "in-use", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 10} for a in feet],
         "phases": [{"id": "first-sitter", "fromS": 0, "toS": 2.5, "title": T("First sitter", "Erste Person")}, {"id": "second-sitter", "fromS": 2.5, "toS": 5.0, "title": T("Second sitter", "Zweite Person")}, {"id": "third-sitter", "fromS": 5.0, "toS": 7.5, "title": T("Third sitter", "Dritte Person")}, {"id": "seated", "fromS": 7.5, "toS": 10, "title": T("All seated", "Alle sitzen")}],
         "events": [{"t": 2.5, "kind": "load-on", "title": T("First sitter is down", "Erste Person sitzt"), "target": "sitter-a"}, {"t": 5.0, "kind": "load-on", "title": T("Second sitter is down", "Zweite Person sitzt"), "target": "sitter-b"}, {"t": 7.5, "kind": "load-on", "title": T("Third sitter is down", "Dritte Person sitzt"), "target": "sitter-c"}],
         "tracks": []},
        {"id": "maintenance", "title": T("Lifting the bench out", "Bank ausheben"), "description": T("The depot crew unscrews the four dowel nuts, chokes two round slings around the seat rails, takes the load on the loader crane and lifts the bench 300 mm off its footings; the footings and dowels stay behind.", "Der Bautrupp schraubt die vier Dübelmuttern ab, schnürt zwei Rundschlingen um die Sitzzargen, nimmt die Last am Ladekran auf und hebt die Bank 300 mm von ihren Fundamenten; Fundamente und Dübel bleiben zurück."),
         "durationS": 14, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 3.5, "pose": "in-service", "ease": "smooth"}, {"t": 10, "pose": "maintenance", "ease": "smooth"}, {"t": 14, "pose": "maintenance", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 3.5} for a in feet] + [{"anchor": a, "fromS": 2.5, "toS": 14} for a in ("lift-r", "lift-l")],
         "phases": [{"id": "unbolting", "fromS": 0, "toS": 2.5, "title": T("Dowel nuts off", "Dübelmuttern lösen")}, {"id": "slinging", "fromS": 2.5, "toS": 3.5, "title": T("Slings taut", "Schlingen gespannt")}, {"id": "lifting", "fromS": 3.5, "toS": 10, "title": T("Lifting", "Heben")}, {"id": "holding", "fromS": 10, "toS": 14, "title": T("Held clear of the footings", "Über den Fundamenten gehalten")}],
         "events": [{"t": 2.5, "kind": "contact-on", "title": T("Slings take the load", "Schlingen nehmen die Last auf"), "target": "lift-r"}, {"t": 3.5, "kind": "contact-off", "title": T("Feet leave the footings", "Füße lösen sich von den Fundamenten"), "target": "foot-front-r"}, {"t": 3.5, "kind": "motion", "title": T("Crane lifts", "Kran hebt")}],
         "tracks": []},
        {"id": "summer-day", "title": T("Sun and tree shade over a summer afternoon", "Sonne und Baumschatten an einem Sommernachmittag"), "description": T("Illustrative scenario: the shade of the lime tree moves over the empty bench and back; the dark painted frames heat and cool fastest, the timber slowly, the footings hardly.", "Beispielszenario: Der Schatten der Linde wandert über die leere Bank und zurück; die dunkel lackierten Gestelle erwärmen und kühlen am schnellsten, das Holz langsam, die Fundamente kaum."),
         "durationS": 60, "fps": 10, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 30, "pose": "in-service", "ease": "smooth"}, {"t": 60, "pose": "in-service", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 60} for a in feet],
         "phases": [{"id": "shade", "fromS": 0, "toS": 30, "title": T("Tree shade", "Baumschatten")}, {"id": "sun", "fromS": 30, "toS": 60, "title": T("Full sun", "Volle Sonne")}],
         "events": [{"t": 30, "kind": "thermal", "title": T("The shade has passed", "Der Schatten ist weitergezogen")}],
         "tracks": [{"target": f"region:{r}:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": lo}, {"t": 30, "v": hi}, {"t": 60, "v": lo}]} for r, lo, hi in (("seat-slats-greyed", 0.15, 0.9), ("seat-slat-replaced", 0.15, 0.9), ("back-slats-greyed", 0.1, 0.55), ("cast-frames", 0.1, 0.45), ("feet-rusted", 0.05, 0.35))]},
    ]

    def region(rid, title, kind, material, area, char, geom, sky, layer, hc, extra=None):
        r = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": round(area, 3), "characteristicLengthM": char, "geometry": geom, "skyViewFactor": sky, "layers": [layer], "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "iso-10456-2007"}
        if extra:
            r.update(extra)
        return r

    def layer(lid, title, kind, material, t, k, rho, cp):
        alpha = k / (rho * cp)
        hc = round(rho * cp * min(t, math.sqrt(alpha * 3600)))
        return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / k, 5), "heatCapacityJM2K": hc, "source": "literature", "reference": "iso-10456-2007"}, hc

    lw, hw = layer("hardwood-slat-45", T("Greyed hardwood slat 45 mm", "Vergraute Laubholzlatte 45 mm"), "timber", "hardwood-greyed", 0.045, 0.18, 900, 1600)
    lr, hr = layer("robinia-slat-45", T("Fresh robinia slat 45 mm", "Frische Robinienlatte 45 mm"), "timber", "robinia-fresh", 0.045, 0.16, 740, 1600)
    lb, hb = layer("hardwood-slat-40", T("Greyed hardwood slat 40 mm", "Vergraute Laubholzlatte 40 mm"), "timber", "hardwood-greyed", 0.040, 0.18, 900, 1600)
    lf, hf = layer("cast-plate-half", T("Half of the 22 mm cast plate (heated from both faces)", "Hälfte der 22-mm-Gussplatte (von beiden Seiten erwärmt)"), "metal", "cast-iron-painted", 0.011, 50, 7200, 460)
    lp, hpd = layer("cast-pad-half", T("Half of the 20 mm cast foot pad", "Hälfte der 20-mm-Gussfußplatte"), "metal", "cast-iron-rusted", 0.010, 50, 7200, 460)
    lc, hc = layer("concrete-footing", T("Concrete footing 220 mm", "Betonfundament 220 mm"), "masonry", "concrete-c20-25", 0.22, 2.0, 2400, 1000)
    regions = [
        region("seat-slats-greyed", T("Greyed seat slats (two rear)", "Vergraute Sitzlatten (zwei hintere)"), "surface", "hardwood-greyed", areas["seat-slats-greyed"], 0.13, "horizontal-plate-up", 0.6, lw, hw),
        region("seat-slat-replaced", T("Replaced front seat slat (fresh robinia)", "Ersetzte vordere Sitzlatte (frische Robinie)"), "surface", "robinia-fresh", areas["seat-slat-replaced"], 0.13, "horizontal-plate-up", 0.6, lr, hr),
        region("back-slats-greyed", T("Greyed back slats, moss on the shaded top edge", "Vergraute Rückenlatten, Moos auf der beschatteten Oberkante"), "surface", "hardwood-greyed", areas["back-slats-greyed"], 0.13, "vertical-plate", 0.5, lb, hb),
        region("cast-frames", T("Painted cast end frames with armrests", "Lackierte Gussseitenteile mit Armlehnen"), "surface", "cast-iron-painted", areas["cast-frames"], 0.045, "vertical-plate", 0.5, lf, hf, {"ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "literature", "reference": "en-1561-2011"}),
        region("feet-rusted", T("Rusted foot pads", "Verrostete Fußplatten"), "surface", "cast-iron-rusted", areas["feet-rusted"], 0.06, "horizontal-plate-up", 0.4, lp, hpd),
        region("footings", T("Concrete point footings", "Beton-Punktfundamente"), "surface", "concrete-c20-25", areas["footings"], 0.24, "vertical-plate", 0.1, lc, hc),
    ]
    thermal = {
        "environment": {"airTempC": 28, "meanRadiantTempC": 30, "skyTempC": 14, "groundTempC": 36, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325},
        "referenceEnvironment": "summer-hot-sun",
        "operating": [], "regions": regions, "emitters": [], "passengers": [],
        "assumptions": [
            T("Passive surfaces only: absorbed sun, long-wave exchange with sky and gravel, convection and conduction with heat capacity; no emitters and no prescribed temperatures. The bench has no electrical element and draws 0 W (stated as passive on the frames).", "Nur passive Oberflächen: absorbierte Sonne, Langwellenaustausch mit Himmel und Kies, Konvektion und Leitung mit Wärmekapazität; keine Emitter und keine vorgegebenen Temperaturen. Die Bank hat kein elektrisches Element und nimmt 0 W auf (an den Gestellen als passiv angegeben)."),
            T("Weathering changes the optics: the dark fir-green paint, chalked after twenty years, absorbs about 0.88 of the sun, the bare rusted feet 0.80, the silver-grey hardwood 0.58 and the fresh yellow robinia slat 0.52 (authored estimates after the absorptance ranges of Incropera table A.12 and the ASHRAE Fundamentals); the frames are the hot part in summer sun.", "Die Bewitterung verändert die Optik: Der dunkle tannengrüne, nach zwanzig Jahren kreidende Anstrich absorbiert etwa 0,88 der Sonne, die blanken verrosteten Füße 0,80, das silbergraue Hartholz 0,58 und die frische gelbe Robinienlatte 0,52 (Schätzwerte nach den Absorptionsbereichen von Incropera Tabelle A.12 und den ASHRAE Fundamentals); in der Sommersonne sind die Gestelle der heiße Teil."),
            T("Areal heat capacity is rho*cp*min(thickness, sqrt(alpha*3600 s)): the share of the section that takes part within an hour (hardwood 21 mm, robinia 23 mm, cast plate and pad half of their thickness because both faces exchange, concrete 55 mm).", "Die flächenbezogene Wärmekapazität ist rho*cp*min(Dicke, sqrt(alpha*3600 s)): der Teil des Querschnitts, der binnen einer Stunde teilnimmt (Hartholz 21 mm, Robinie 23 mm, Gussplatte und Fußplatte je zur Hälfte, weil beide Seiten austauschen, Beton 55 mm)."),
            T("The footings are buried: only their 25 mm proud top edges and tops are in air (exposed share 0.12); the rest touches the soil at 16 degC (summer soil temperature at 0.5 m).", "Die Fundamente liegen im Boden: Nur ihre 25 mm überstehenden Oberkanten und Oberseiten sind an der Luft (Anteil 0,12); der Rest berührt das Erdreich bei 16 degC (Bodentemperatur im Sommer in 0,5 m Tiefe)."),
            T("Sitters cover the seat and back with clothed bodies at 31 and 30 degC (ISO 7730 clothing surfaces) and shade them; covered shares from three adults of 0.13 m2 seat and 0.05 m2 back contact each.", "Sitzende bedecken Sitz und Lehne mit bekleideten Körpern bei 31 und 30 degC (Bekleidungsoberflächen nach ISO 7730) und beschatten sie; bedeckte Anteile aus drei Erwachsenen mit je 0,13 m2 Sitz- und 0,05 m2 Rückenkontakt."),
            T("Graffiti, sticker residue, moss and chewing gum are drawn and modelled as surface marks only; their thermal effect on the slat regions is neglected.", "Graffiti, Aufkleberreste, Moos und Kaugummi sind nur als Oberflächenspuren gezeichnet und modelliert; ihre thermische Wirkung auf die Lattenregionen wird vernachlässigt."),
        ],
    }

    structure = {
        "gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [],
        "assumptions": [
            T(f"The bench ({total:.1f} kg: two cast end frames of {M['frame']['kg']:.1f} kg with armrests and foot pads, three seat slats and two back slats with their carriage bolts) is one rigid body. In service it stands on its four feet, each dowelled with one M12 anchor rod into the point footing (pins); lifted out it hangs from two round slings choked around the seat rails at the lateral position of the centre of mass (pins at the lifting points). No support is permanent: every pose and clip names its supports.", f"Die Bank ({total:.1f} kg: zwei Gussseitenteile zu je {M['frame']['kg']:.1f} kg mit Armlehnen und Fußplatten, drei Sitz- und zwei Rückenlatten mit ihren Schlossschrauben) ist ein starrer Körper. Im Einsatz steht sie auf ihren vier Füßen, jeder mit einer M12-Ankerstange in das Punktfundament gedübelt (Gelenke); ausgehoben hängt sie an zwei um die Sitzzargen geschnürten Rundschlingen in der Querlage des Schwerpunkts (Gelenke an den Anschlagpunkten). Kein Auflager ist dauerhaft: Jede Pose und jeder Clip nennt ihre Auflager."),
            T("The two point footings (each 0.68 x 0.22 x 0.425 m of C20/25, about 150 kg) belong to the ground: they are drawn and modelled with the bench because their tops are visible and they stay behind when it is lifted, but their weight goes straight into the soil and is neither part of the lifted rigid body nor of dimensions.massKg.", "Die beiden Punktfundamente (je 0,68 x 0,22 x 0,425 m C20/25, etwa 150 kg) gehören zum Boden: Sie sind mit der Bank gezeichnet und modelliert, weil ihre Oberseiten sichtbar sind und sie beim Ausheben zurückbleiben, ihr Gewicht geht aber direkt ins Erdreich und gehört weder zum gehobenen starren Körper noch zu dimensions.massKg."),
            T("In use each of three adults (80 kg after DIN 33402-2) puts 80 % of the body weight (628 N) on the seat and leans on the lower back slat with 100 N; the static seat and back loads of EN 581-2 and EN 1728 (1300 N and 560 N on one place) are test loads, not service loads, and are not applied.", "Im Gebrauch setzen drei Erwachsene (80 kg nach DIN 33402-2) je 80 % des Körpergewichts (628 N) auf den Sitz und lehnen sich mit 100 N an die untere Rückenlatte; die statischen Sitz- und Rückenprüflasten von EN 581-2 und EN 1728 (1300 N und 560 N an einem Platz) sind Prüf-, keine Gebrauchslasten und werden nicht angesetzt."),
            T("Members: slats as hardwood D60 and robinia D40 after EN 338 spanning 1.56 m between the frames; the end frames as EN-GJL-200 grey cast iron after EN 1561 (legs 45 x 22 mm); the footings as plain C20/25 after EN 206. Cracks, checks and the replaced slat do not change the rigid-body reactions.", "Glieder: Latten als Laubholz D60 und Robinie D40 nach EN 338 mit 1,56 m Spannweite zwischen den Gestellen; die Seitenteile als Grauguss EN-GJL-200 nach EN 1561 (Beine 45 x 22 mm); die Fundamente als unbewehrter C20/25 nach EN 206. Risse, Trockenrisse und die ersetzte Latte ändern die Starrkörperreaktionen nicht."),
        ],
        "clinical": False,
    }

    reps, vols = {}, {}
    for scale in g["SCALES"]:
        views = g["drawings"](scale)
        reps[str(scale)] = representation(g, scale, views)
        body, therm, struct = g["volumes"](scale)
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": therm, "structure": struct, "decisions": volume_decisions(T, scale)}

    x0, x1, y0, y1, z0, z1 = extents(g)
    asset = {
        "id": "used-backrest-park-bench",
        "category": "furniture",
        "title": T("Used Backrest Park Bench", "Gebrauchte Parkbank mit Lehne"),
        "summary": T(
            f"A classic 1.80 m municipal park bench with backrest after twenty years in a Volkspark: two one-piece grey cast-iron end frames with armrests, painted fir green and rusting at the feet, carry three seat slats and two back slats of tropical hardwood weathered silver-grey and checked; the front seat slat has been replaced by the depot in fresh yellow robinia. The frames are dowelled to two concrete point footings whose tops stand 25 mm proud of the eroded gravel. Graffiti and a torn sticker on the backrest, moss on the shaded top edge, chewing gum on the seat and a heel hollow in front of it tell its use. It is lifted out on two round slings and carries three sitters. Side elevation: the bench front along its length.",
            f"Eine klassische 1,80 m lange städtische Parkbank mit Lehne nach zwanzig Jahren im Volkspark: Zwei einteilige Grauguss-Seitenteile mit Armlehnen, tannengrün lackiert und an den Füßen rostend, tragen drei Sitz- und zwei Rückenlatten aus Tropenhartholz, silbergrau verwittert und rissig; die vordere Sitzlatte hat der Bauhof durch frische gelbe Robinie ersetzt. Die Gestelle sind auf zwei Beton-Punktfundamente gedübelt, deren Oberseiten 25 mm aus dem ausgetretenen Kies ragen. Graffiti und ein abgerissener Aufkleber an der Lehne, Moos auf der beschatteten Oberkante, Kaugummi auf dem Sitz und eine Fersenmulde davor erzählen von ihrem Gebrauch. Sie wird an zwei Rundschlingen ausgehoben und trägt drei Sitzende. Seitenansicht: die Bankvorderseite entlang ihrer Länge."),
        "tags": ["used", "park", "public-space", "seating", "bench", "street-furniture", "cast-iron", "hardwood", "timber", "robinia", "concrete", "point-footing", "weathered"],
        "provenance": provenance(T),
        "dimensions": {"heightM": round(y1 - y0, 3), "widthM": round(x1 - x0, 3), "depthM": round(z1 - z0, 3), "massKg": round(total, 1)},
        "materials": materials,
        "rig": rig,
        "volumes": vols,
        "anchors": anchors,
        "representations": reps,
        "symbols": [],
        "poses": poses,
        "clips": clips,
        "thermal": thermal,
        "structure": structure,
        "presentation": {"defaultPose": "in-service", "defaultClip": "sitting-down", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def extents(g):
    """📦 World extents of the 1:50 body (x, y, z)."""
    xs, ys, zs = [], [], []
    for p in g["parts"](50):
        x0, x1, y0, y1, s0, s1 = g["world_box"](p)
        xs += [x0, x1]
        ys += [y0, y1]
        zs += [-s0, -s1]
    return min(xs), max(xs), min(ys), max(ys), min(zs), max(zs)


def region_areas(g):
    """🌡️ Outer area of every thermal region at 1:50: prisms 2A + P L, boxes their six faces, less the faces the armrest buries in the plate."""
    out = {}
    for p in g["parts"](50):
        if not p.get("thermal", True):
            continue
        if p["kind"] == "xprism":
            a, per, _ = g["section_props"](p["prof"], p["holes"])
            area = 2 * a + per * (p["x1"] - p["x0"])
        else:
            x0, x1, y0, y1, s0, s1 = g["world_box"](p)
            dx, dy, ds = x1 - x0, y1 - y0, s1 - s0
            area = 2 * (dx * dy + dy * ds + dx * ds)
        out[p["region"]] = out.get(p["region"], 0.0) + area
    out["cast-frames"] -= 2 * (2 * 0.046 * 0.040 + 0.040 * 0.022)
    return out


def representation(g, scale, views):
    """🖼️ One authored representation with pens, hatches, ranges and decisions."""
    T = g["T"]
    pens = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    ranges = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    hatches = []
    if scale == 50:
        hatches = [
            {"id": "grain", "title": T("Weathered hardwood grain", "Verwitterte Hartholzmaserung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13},
            {"id": "grain-fresh", "title": T("Fresh robinia grain", "Frische Robinienmaserung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.9, "penMm": 0.13},
            {"id": "concrete", "title": T("Concrete", "Beton"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13},
            {"id": "moss", "title": T("Moss and algae", "Moos und Algen"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.4, "penMm": 0.13},
        ]
    elif scale == 100:
        hatches = [{"id": "concrete", "title": T("Concrete", "Beton"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13}]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": ranges[0], "maxDenominator": ranges[1]}, "pens": pens, "hatches": hatches, "geometry": views, "decisions": rep_decisions(T, scale)}


def rep_decisions(T, scale):
    """📝 The design decisions of one representation."""
    if scale == 50:
        return {
            "silhouette": T("Bench 36 x 24.7 mm with its footings. Side elevation (the bench front): the replaced front seat slat light in front of the two greyed rear slats, the two greyed back slats with the armrest gap between them, the cast frames edge-on with the arm posts standing in front of the seat, the rusted foot pads and dowel ends, the two point footings below the ground line. Front view (left end): the one-piece cast frame with its arm post, raked rail and post, kicked-back rear leg, stretcher and opening, the armrest with its knuckle, the eased slat ends, the footing with its proud top edge and the heel hollow in the gravel in front. Plan: five slats with their drainage gaps, armrests, frame plates, feet and dowels, the footings around the feet and the worn hollow.", "Bank 36 x 24,7 mm mit ihren Fundamenten. Seitenansicht (Bankvorderseite): die ersetzte vordere Sitzlatte hell vor den zwei vergrauten hinteren Latten, die zwei vergrauten Rückenlatten mit der Armlehnenlücke dazwischen, die Gussgestelle von der Kante mit den Armstützen vor dem Sitz, die verrosteten Fußplatten und Dübelenden, die zwei Punktfundamente unter der Geländelinie. Vorderansicht (linkes Ende): das einteilige Gussgestell mit Armstütze, geneigter Zarge und Pfosten, ausgestelltem Hinterbein, Strebe und Öffnung, die Armlehne mit Knauf, die gebrochenen Lattenenden, das Fundament mit überstehender Oberkante und die Fersenmulde im Kies davor. Grundriss: fünf Latten mit Wasserfugen, Armlehnen, Gestellplatten, Füße und Dübel, die Fundamente um die Füße und die ausgetretene Mulde."),
            "retained": T("Every slat with its 15 mm gap and 4 mm eased edges, the replaced slat in its own tone and grain, the casting with its stretcher and opening, the armrest knuckle, foot pads, dowel ends, carriage-bolt heads, checks, graffiti, sticker residue, moss, chewing gum, rust and paint flakes.", "Jede Latte mit ihrer 15-mm-Fuge und 4-mm-Kantenbruch, die ersetzte Latte in eigenem Ton und eigener Maserung, der Guss mit Strebe und Öffnung, der Armlehnenknauf, Fußplatten, Dübelenden, Schlossschraubenköpfe, Risse, Graffiti, Aufkleberreste, Moos, Kaugummi, Rost und Lackabplatzungen."),
            "merged": T("The two end frames coincide in the front view (the near one is drawn); front and rear foot coincide in the side elevation.", "Die beiden Seitenteile fallen in der Vorderansicht zusammen (das nahe ist gezeichnet); vorderer und hinterer Fuß fallen in der Seitenansicht zusammen."),
            "omitted": T("Nuts and washers of the dowels (taken off for maintenance, shown as the dowel end), the hidden lower ends of the dowels in the concrete, the bolt nuts under the rails.", "Muttern und Scheiben der Dübel (zur Pflege abgenommen, als Dübelende gezeigt), die verborgenen unteren Dübelenden im Beton, die Schraubenmuttern unter den Zargen."),
            "thinElements": T("The 15 mm slat gaps are 0.3 mm at 1:50 and close by ink beyond 1:60; the 22 mm frame plates and 12 mm dowels keep their true width with their pens.", "Die 15-mm-Lattenfugen sind bei 1:50 0,3 mm und schließen sich jenseits 1:60 durch die Tinte; die 22-mm-Gestellplatten und 12-mm-Dübel behalten ihre wahre Breite mit ihren Stiften."),
            "hatching": T("Weathered grain every 0.6 mm and finer fresh grain every 0.9 mm on the slats in elevation and plan, concrete dots on the footings, moss stipple on the top edge of the upper back slat.", "Verwitterte Maserung alle 0,6 mm und feinere frische Maserung alle 0,9 mm auf den Latten in Ansicht und Grundriss, Betonpunkte auf den Fundamenten, Moospunktierung auf der Oberkante der oberen Rückenlatte."),
            "styles": T("Greyed slats mid grey (0.52 to 0.60), the replaced slat light (0.86), cast frames dark (0.30, armrests 0.26), rusted feet darkest (0.18), concrete light (0.78), the heel hollow as a pale ground patch.", "Vergraute Latten mittelgrau (0,52 bis 0,60), die ersetzte Latte hell (0,86), Gussgestelle dunkel (0,30, Armlehnen 0,26), verrostete Füße am dunkelsten (0,18), Beton hell (0,78), die Fersenmulde als helle Geländefläche."),
            "overlays": T("Every shape carries one region: greyed seat slats, replaced slat, greyed back slats, cast frames, rusted feet, footings; the traces carry the region they sit on.", "Jede Form trägt eine Region: vergraute Sitzlatten, ersetzte Latte, vergraute Rückenlatten, Gussgestelle, verrostete Füße, Fundamente; die Spuren tragen die Region, auf der sie liegen."),
        }
    if scale == 100:
        return {
            "silhouette": T("Bench 18 x 12.4 mm: the same construction with square slat edges, the casting with its stretcher, the armrest with a three-segment knuckle, the footings below ground.", "Bank 18 x 12,4 mm: dieselbe Konstruktion mit scharfkantigen Latten, der Guss mit Strebe, die Armlehne mit dreiteiligem Knauf, die Fundamente unter Gelände."),
            "retained": T("Five slats with their gaps, the replaced slat in its own tone, frames, armrests, foot pads, footings, graffiti, moss, rust at the feet and the heel hollow.", "Fünf Latten mit ihren Fugen, die ersetzte Latte im eigenen Ton, Gestelle, Armlehnen, Fußplatten, Fundamente, Graffiti, Moos, Rost an den Füßen und die Fersenmulde."),
            "merged": T("Eased edges into square ones; the near end frame stands for both in the front view.", "Gebrochene Kanten zu scharfen; das nahe Seitenteil steht in der Vorderansicht für beide."),
            "omitted": T("Bolt heads, dowel ends, checks, sticker residue, chewing gum, paint flakes and grain hatching (all below 0.2 mm of paper).", "Schraubenköpfe, Dübelenden, Risse, Aufkleberreste, Kaugummi, Lackabplatzungen und Maserung (alle unter 0,2 mm auf Papier)."),
            "thinElements": T("The 15 mm gaps are 0.15 mm of paper and close by ink beyond 1:100; frame plates 0.22 mm.", "Die 15-mm-Fugen sind 0,15 mm auf Papier und schließen sich jenseits 1:100 durch die Tinte; Gestellplatten 0,22 mm."),
            "hatching": T("Concrete dots every 0.8 mm on the footings only.", "Betonpunkte alle 0,8 mm nur auf den Fundamenten."),
            "styles": T("As at 1:50: greyed slats mid grey, replaced slat light, frames dark, feet darkest, concrete light.", "Wie bei 1:50: vergraute Latten mittelgrau, ersetzte Latte hell, Gestelle dunkel, Füße am dunkelsten, Beton hell."),
            "overlays": T("One region per shape.", "Eine Region je Form."),
        }
    if scale == 200:
        return {
            "silhouette": T("Bench 9 x 6.2 mm: the replaced front slat and one greyed rear seat board, two back slats, the cast frames as 30 mm plates with the stretcher and opening, square armrests, 30 mm foot pads and the footings.", "Bank 9 x 6,2 mm: die ersetzte vordere Latte und ein vergrautes hinteres Sitzbrett, zwei Rückenlatten, die Gussgestelle als 30-mm-Platten mit Strebe und Öffnung, eckige Armlehnen, 30-mm-Fußplatten und die Fundamente."),
            "retained": T("Seat height, back rake, the armrest gap between the back slats, the replaced slat as a light band, the footings below ground.", "Sitzhöhe, Lehnenneigung, die Armlehnenlücke zwischen den Rückenlatten, die ersetzte Latte als helles Band, die Fundamente unter Gelände."),
            "merged": T("The two greyed rear seat slats and their 15 mm gap into one board; the front slat closes onto the arm post; the frame plate thickened from 22 to 30 mm.", "Die zwei vergrauten hinteren Sitzlatten mit ihrer 15-mm-Fuge zu einem Brett; die vordere Latte schließt an die Armstütze an; die Gestellplatte von 22 auf 30 mm verdickt."),
            "omitted": T("All surface traces, the armrest knuckle, hatching.", "Alle Oberflächenspuren, der Armlehnenknauf, Schraffuren."),
            "thinElements": T("Every part at least 30 mm (0.15 mm of paper); the 90 mm armrest gap stays open.", "Jedes Teil mindestens 30 mm (0,15 mm auf Papier); die 90-mm-Armlehnenlücke bleibt offen."),
            "hatching": T("None.", "Keine."),
            "styles": T("Tones only: replaced slat light, greyed timber mid grey, cast iron dark, concrete light.", "Nur Töne: ersetzte Latte hell, vergrautes Holz mittelgrau, Gusseisen dunkel, Beton hell."),
            "overlays": T("One region per shape.", "Eine Region je Form."),
        }
    return {
        "silhouette": T("Bench 3.6 x 2.5 mm: a light front band and a grey rear seat slab, two back slabs, each end frame as one 75 mm casting with its armrest, the foot blocks and the footings.", "Bank 3,6 x 2,5 mm: ein helles vorderes Band und eine graue hintere Sitzplatte, zwei Rückenplatten, jedes Seitenteil als ein 75-mm-Guss mit seiner Armlehne, die Fußklötze und die Fundamente."),
        "retained": T("Overall size, seat height, back rake, the open leg space, the footings below ground, the replaced slat as a lighter band.", "Gesamtgröße, Sitzhöhe, Lehnenneigung, der offene Beinraum, die Fundamente unter Gelände, die ersetzte Latte als helleres Band."),
        "merged": T("Slats into 75 mm thick slabs, legs, rail, posts and armrest into 75 mm bands of one casting outline, the foot pads into 75 mm blocks.", "Latten zu 75 mm dicken Platten, Beine, Zarge, Pfosten und Armlehne zu 75-mm-Bändern eines Gussumrisses, die Fußplatten zu 75-mm-Klötzen."),
        "omitted": T("The stretcher (35 mm), slat gaps, every trace.", "Die Strebe (35 mm), Lattenfugen, jede Spur."),
        "thinElements": T("Every member at least 75 mm (0.15 mm of paper).", "Jedes Glied mindestens 75 mm (0,15 mm auf Papier)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Front band 0.86, timber 0.55, cast iron 0.30, feet 0.18, concrete 0.78.", "Vorderes Band 0,86, Holz 0,55, Gusseisen 0,30, Füße 0,18, Beton 0,78."),
        "overlays": T("One region per shape.", "Eine Region je Form."),
    }


def volume_decisions(T, scale):
    """🧊 What each scale's 3D lists keep, merge and leave out."""
    if scale == 50:
        return [
            T("Body for 7.5 mm: each end frame as one 22 mm grey cast-iron plate extruded from its true outline (front leg, arm post, raked rail, raked post, kicked-back rear leg, stretcher and the opening between them), a 50 mm armrest with its knuckle embedded 12 mm in the post, 85 x 20 x 60 mm rusted foot pads, the M12 anchor dowels outboard of the legs on the 0.68 x 0.22 x 0.425 m point footings; three seat slats 130 x 45 mm on the 5 degree rail (the front one fresh robinia) and two back slats 130 x 40 mm on the 15 degree posts, all with 4 mm eased edges; carriage-bolt heads, graffiti, sticker residue, moss and chewing gum as texture solids on the faces.", "Körper für 7,5 mm: jedes Seitenteil als eine 22-mm-Graugussplatte, aus ihrem wahren Umriss extrudiert (Vorderbein, Armstütze, geneigte Zarge, geneigter Pfosten, ausgestelltes Hinterbein, Strebe und die Öffnung dazwischen), eine 50-mm-Armlehne mit Knauf, 12 mm in den Pfosten eingebunden, verrostete Fußplatten 85 x 20 x 60 mm, die M12-Ankerdübel außen neben den Beinen auf den Punktfundamenten 0,68 x 0,22 x 0,425 m; drei Sitzlatten 130 x 45 mm auf der 5-Grad-Zarge (die vordere frische Robinie) und zwei Rückenlatten 130 x 40 mm an den 15-Grad-Pfosten, alle mit 4-mm-Kantenbruch; Schlossschraubenköpfe, Graffiti, Aufkleberreste, Moos und Kaugummi als Texturkörper auf den Flächen."),
            T("Thermal: every region is the surface of its body solids (two greyed seat slats, the replaced slat, two back slats, frames with armrests, the four pads, the two footings); the dowels and texture marks carry no region. Structure: frames, armrests, pads, slats and footings as members, one M10 carriage bolt per slat and frame and the M12 dowels as joints, masses at the centres of the frames, seat and back.", "Thermik: Jede Region ist die Oberfläche ihrer Körper (zwei vergraute Sitzlatten, die ersetzte Latte, zwei Rückenlatten, Gestelle mit Armlehnen, die vier Fußplatten, die zwei Fundamente); Dübel und Texturspuren tragen keine Region. Tragwerk: Gestelle, Armlehnen, Fußplatten, Latten und Fundamente als Glieder, je Latte und Gestell eine M10-Schlossschraube und die M12-Dübel als Gelenke, Massen in den Schwerpunkten von Gestellen, Sitz und Lehne."),
        ]
    if scale == 100:
        return [
            T("Body for 15 mm, authored anew: square-edged slats with their 15 mm gaps, the cast plates at 22 mm with the same outline and opening, armrests with a three-segment knuckle, the foot pads; the dowels (12 mm) and bolt heads are below the feature and left out; graffiti and moss stay as texture solids.", "Körper für 15 mm, neu gebaut: scharfkantige Latten mit ihren 15-mm-Fugen, die Gussplatten mit 22 mm und gleichem Umriss samt Öffnung, Armlehnen mit dreiteiligem Knauf, die Fußplatten; Dübel (12 mm) und Schraubenköpfe liegen unter dem Merkmal und entfallen; Graffiti und Moos bleiben als Texturkörper."),
            T("Thermal: one surface per region from the body solids. Structure: the same members; bolts and dowels as 15 mm joints; masses at the centres.", "Thermik: eine Oberfläche je Region aus den Körpern. Tragwerk: dieselben Glieder; Schrauben und Dübel als 15-mm-Gelenke; Massen in den Schwerpunkten."),
        ]
    if scale == 200:
        return [
            T("Body for 30 mm: the replaced front slat closes onto the arm post and the two greyed rear slats are one 285 mm board (their 15 mm gap is below the feature); two back slats; cast plates thickened to 30 mm, square armrests, 30 mm foot pads; no textures.", "Körper für 30 mm: die ersetzte vordere Latte schließt an die Armstütze an, die zwei vergrauten hinteren Latten sind ein 285-mm-Brett (ihre 15-mm-Fuge liegt unter dem Merkmal); zwei Rückenlatten; Gussplatten auf 30 mm verdickt, eckige Armlehnen, 30-mm-Fußplatten; keine Texturen."),
            T("Thermal: one surface per region. Structure: members as in the body, bolts and dowels as 30 mm joints, masses at the centres.", "Thermik: eine Oberfläche je Region. Tragwerk: Glieder wie im Körper, Schrauben und Dübel als 30-mm-Gelenke, Massen in den Schwerpunkten."),
        ]
    return [
        T("Body for 75 mm: seat and back slats as 75 mm thick slabs (the replaced front one kept apart for its region), each end frame one 75 mm casting with 75 mm bands and the armrest merged into its outline, the stretcher left out, 75 mm foot blocks, the footings.", "Körper für 75 mm: Sitz- und Rückenlatten als 75 mm dicke Platten (die ersetzte vordere wegen ihrer Region getrennt), jedes Seitenteil ein 75-mm-Guss mit 75-mm-Bändern und in den Umriss verschmolzener Armlehne, die Strebe entfällt, 75-mm-Fußklötze, die Fundamente."),
        T("Thermal: one surface per region, compared only in total at this scale. Structure: members as in the body, bolts and dowels as 75 mm joints, masses at the centres.", "Thermik: eine Oberfläche je Region, bei diesem Maßstab nur in der Summe verglichen. Tragwerk: Glieder wie im Körper, Schrauben und Dübel als 75-mm-Gelenke, Massen in den Schwerpunkten."),
    ]


def provenance(T):
    """📚 Sources of the bench."""
    return {
        "author": "Semio staffage content (used park objects wave, row 01)",
        "method": "handcrafted",
        "created": "2026-11-22",
        "references": [
            {"id": "din-18040-3-2014", "citation": "DIN 18040-3:2014-12 Barrierefreies Bauen - Planungsgrundlagen - Teil 3: Öffentlicher Verkehrs- und Freiraum (seats in public space 0.46-0.48 m high with backrest and armrests)."},
            {"id": "en-581-1-2017", "citation": "EN 581-1:2017 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 1: General safety requirements."},
            {"id": "en-581-2-2015", "citation": "EN 581-2:2015 Outdoor furniture - Part 2: Mechanical safety requirements and test methods for seating (static seat and back load tests, stability)."},
            {"id": "en-1728-2012", "citation": "EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability (seat static load 1300 N, back static load 560 N for public use)."},
            {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (body mass of German adults 18-65 years, median about 77 kg; 80 kg used)."},
            {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - densities, self-weight, imposed loads."},
            {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (hardwood D60: E0,mean 17 GPa, fm,k 60 MPa; D40: E0,mean 13 GPa, fm,k 40 MPa)."},
            {"id": "en-1561-2011", "citation": "EN 1561:2011 Founding - Grey cast irons (EN-GJL-200: tensile strength 200 MPa, E 88-113 GPa, density 7.2 kg/dm3, conductivity about 50 W/(m K))."},
            {"id": "en-206-2013", "citation": "EN 206:2013+A2:2021 Concrete - Specification, performance, production and conformity (C20/25 for unreinforced footings)."},
            {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity and specific heat of timber, cast iron, steel, concrete)."},
            {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley, table A.12 (emissivity and solar absorptance of painted, oxidised and galvanised metals, wood, concrete)."},
            {"id": "ashrae-fundamentals-2021", "citation": "ASHRAE Handbook - Fundamentals (2021), chapters 14 and 15 (solar absorptance of building surfaces, dark paints 0.85-0.95, weathered timber and concrete 0.5-0.75)."},
            {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment (outer clothing surface temperatures of seated persons around 28-32 degC)."},
            {"id": "wagenfuehr-holzatlas-2007", "citation": "Wagenführ, R. (2007). Holzatlas, 6th ed., Hanser (air-dry densities: Bangkirai/Yellow Balau 850-980 kg/m3, robinia 730-800 kg/m3; natural durability class 1-2 of both)."},
            {"id": "fll-wegedecken-2007", "citation": "FLL (2007). Fachbericht für Planung, Bau und Instandhaltung von Wassergebundenen Wegen (wearing course 30-50 mm of crushed 0/8 mm on a base course; erosion and hollows at seats and entries)."},
            {"id": "park-bench-catalogues", "citation": "Municipal street-furniture catalogues of classic cast-iron frame park benches (types, no brands): 1.80 m long, 0.65-0.75 m deep, 0.80-0.85 m high, seat 0.44-0.47 m, five hardwood slats of 120-145 x 40-50 mm fixed by M8-M10 carriage bolts, one-piece cast end frames of 18-25 kg with armrests, total 80-100 kg; installation dowelled on point footings of about 0.6 x 0.25 x 0.4-0.5 m or cast in, anchor rods M12."},
        ],
        "notes": T("Side elevation along the bench length (the sitters face the viewer). Construction after classic cast-iron frame benches of municipal catalogues and the seating dimensions of DIN 18040-3: seat 0.463 m high at the front edge, 0.42 m deep, raked 5 degrees; back slats to 0.836 m on posts raked 15 degrees (100 degrees between seat and back); armrests 0.665 m high (0.20 m above the seat front); two one-piece EN-GJL-200 end frames 0.12 m in from the slat ends; three 130 x 45 mm seat slats and two 130 x 40 mm back slats of Bangkirai-type hardwood, the front seat slat replaced in robinia; four M12 dowels into two C20/25 point footings. Masses from the modelled 1:50 volumes and the densities of the materials. Every value is an authored estimate or a literature value; none is measured.", "Seitenansicht entlang der Banklänge (die Sitzenden blicken zum Betrachter). Konstruktion nach klassischen Gussgestellbänken städtischer Kataloge und den Sitzmaßen der DIN 18040-3: Sitz an der Vorderkante 0,463 m hoch, 0,42 m tief, 5 Grad geneigt; Rückenlatten bis 0,836 m an 15 Grad geneigten Pfosten (100 Grad zwischen Sitz und Lehne); Armlehnen 0,665 m hoch (0,20 m über der Sitzvorderkante); zwei einteilige Seitenteile EN-GJL-200 0,12 m von den Lattenenden; drei Sitzlatten 130 x 45 mm und zwei Rückenlatten 130 x 40 mm aus Hartholz vom Bangkirai-Typ, die vordere Sitzlatte durch Robinie ersetzt; vier M12-Dübel in zwei Punktfundamente C20/25. Massen aus den modellierten Volumen 1:50 und den Dichten der Werkstoffe. Jeder Wert ist eine Schätzung oder ein Literaturwert; keiner ist gemessen."),
    }
