"""🌱 Round 3 stepladder roots: summary, provenance, dimensions, rig, anchors, thermal areas and structural assumptions of the real five-step platform stepladder (values from `r3-stepladder-kit.py`)."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("stepladder_kit", os.path.join(T, "r3-stepladder-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

REFERENCES = [
    {"id": "hailo-l60-8160-507", "citation": "Hailo L60 StandardLine 8160-507, aluminium safety stepladder with platform, 5 steps: platform height 1.06 m, ladder length 1.82 m, open height 1.68 m, width 0.48 m, steps 80 mm deep, rated load 150 kg, 4.6 kg (retailer data sheets).", "url": "https://www.topleiter.de/stehleiter/Hailo-L60-Standardline-Stehleiter-5-Stufen-8160-507.html"},
]

NOTES = {
    "en": "Side elevation of the open ladder, climbing side ahead (+x). Front frame: two 60 x 22 x 1.5 mm box stiles at 67.5 degrees (EN 131-1: 60 to 70 degrees for the climbing leg), 1.82 m long, converging from 0.43 m at the feet to 0.32 m at the top, with four 80 x 25 mm treads at a pitch of 0.212 m, the platform at 1.06 m and a grab bar 0.56 m above it (top of the stiles 0.62 m above the platform, EN 131-1 asks for 0.6 m). Rear frame: two 45 x 20 x 1.2 mm rails at 74 degrees on hinge bolts through the stiles at 1.33 m, with the rest tube that carries and locks the rear rim of the platform and two braces; a two-link pressed-steel spreader at 0.53 m on each side. Footprint 1.02 x 0.47 m, open height 1.68 m: the proportions of the five-step class (Hailo L60 8160-507). The mass of 7.1 kg is the sum of the parts from these sections with the densities of EN 1999-1-1 and ISO 10456 (walls 1.5 and 1.2 mm, steel spreaders and axle), heavier than the 4.6 kg of the thin-walled household model. The person is a load (100 kg), not a second asset. The folded state is NOT authored: in a folded pose the nearly parallel rails, treads and spreader links crowd into gaps below the print profile at every scale; the hinge is therefore a fixed joint here.",
    "de": "Seitenansicht der aufgeklappten Leiter, Aufstiegsseite vorn (+x). Vorderrahmen: zwei Kastenholme 60 x 22 x 1,5 mm unter 67,5 Grad (EN 131-1: 60 bis 70 Grad für den Steigschenkel), 1,82 m lang, von 0,43 m an den Füßen auf 0,32 m oben zusammenlaufend, mit vier Stufen 80 x 25 mm im Abstand von 0,212 m, der Plattform auf 1,06 m und einem Haltebügel 0,56 m darüber (Holmende 0,62 m über der Plattform, EN 131-1 verlangt 0,6 m). Stützrahmen: zwei Hinterholme 45 x 20 x 1,2 mm unter 74 Grad an Gelenkbolzen durch die Holme auf 1,33 m, mit dem Auflagerohr, das den hinteren Plattformrand trägt und sperrt, und zwei Traversen; je Seite eine zweigliedrige Stahlspreize auf 0,53 m. Standfläche 1,02 x 0,47 m, Höhe aufgeklappt 1,68 m: die Proportionen der Fünf-Stufen-Klasse (Hailo L60 8160-507). Die Masse von 7,1 kg ist die Summe der Teile aus diesen Querschnitten mit den Dichten nach EN 1999-1-1 und ISO 10456 (Wände 1,5 und 1,2 mm, Stahlspreizen und Achse), schwerer als die 4,6 kg des dünnwandigen Haushaltsmodells. Die Person ist eine Last (100 kg), kein zweites Asset. Der zusammengeklappte Zustand ist NICHT verfasst: in einer zusammengeklappten Pose drängen sich die nahezu parallelen Holme, Stufen und Spreizenglieder in Abstände unter dem Druckprofil bei jedem Maßstab; das Gelenk ist daher hier ein festes Gelenk.",
}

SUMMARY = {
    "en": "A five-step aluminium stepladder with platform, 1.68 m high with the platform at 1.06 m and 1.02 m across the feet, open and locked: empty, under a person standing on it and under the EN 131 rated test load, with the standing load and lateral thrust as loads.",
    "de": "Eine fünfstufige Aluminium-Stehleiter mit Plattform, 1,68 m hoch mit der Plattform auf 1,06 m und 1,02 m über den Füßen, aufgeklappt und gesperrt: leer, unter einer stehenden Person und unter der EN-131-Nennprüflast, mit Standlast und Seitenschub als Lasten.",
}

BONE_TITLES = {
    k.FRONT: {"en": "Front frame: stiles with treads, platform and grab bar", "de": "Vorderrahmen: Holme mit Stufen, Plattform und Haltebügel"},
    k.REAR: {"en": "Rear support frame from the hinge to the feet (locked open)", "de": "Stützrahmen vom Gelenk zu den Füßen (gesperrt)"},
}

SECTION_TITLES = {
    k.FRONT: {"en": "Two box stiles 60 x 22 x 1.5 mm", "de": "Zwei Kastenholme 60 x 22 x 1,5 mm"},
    k.REAR: {"en": "Two box rails 45 x 20 x 1.2 mm", "de": "Zwei Kastenholme 45 x 20 x 1,2 mm"},
}


def assumptions():
    return [
        {"en": "A rigid body on elastic contacts: the four shoes, the front pair at x = %.3f m and z = +-%.3f m, the rear pair at x = %.3f m and z = +-%.3f m." % (k.XF, k.fz(0.0), k.XR, k.rz(0.0)),
         "de": ("Starrer Körper auf elastischen Kontakten: die vier Schuhe, das vordere Paar bei x = %.3f m und z = +-%.3f m, das hintere Paar bei x = %.3f m und z = +-%.3f m." % (k.XF, k.fz(0.0), k.XR, k.rz(0.0))).replace(".", ",").replace(",,", ".").rstrip(",") + "."},
        {"en": "Rated load 150 kg (EN 131-2); the standing load of 100 kg and the 80 N lateral thrust are authored estimates.", "de": "Nennlast 150 kg (EN 131-2); die Standlast von 100 kg und der Seitenschub von 80 N sind Schätzwerte."},
    ]


def masses():
    return k.masses()


def apply(asset, areas=None):
    m = k.masses()
    sec = k.sections()
    asset["summary"] = SUMMARY
    prov = asset["provenance"]
    known = {r["id"] for r in prov["references"]}
    prov["references"] = prov["references"] + [r for r in REFERENCES if r["id"] not in known]
    prov["notes"] = NOTES
    asset["dimensions"] = {"heightM": k.H_TOP, "widthM": round(k.XF + 0.045 - (k.XR - 0.040), 3), "depthM": round(2 * (k.fz(0.0) + k.SHOE_F[1]), 3), "massKg": round(m[k.FRONT][0] + m[k.REAR][0], 1)}
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    hinge = k.local(k.FRONT, (k.HX, k.H_HINGE))
    place = {k.FRONT: (k.XF, 0.0, 180.0 - k.A_FRONT, k.FRONT_LEN), k.REAR: (hinge[0], hinge[1], k.A_FRONT + k.A_REAR, k.REAR_LEN)}
    for bid, (x, y, angle, length) in place.items():
        b = bones[bid]
        b["title"] = BONE_TITLES[bid]
        b["x"], b["y"], b["angleDeg"], b["lengthM"] = round(x + 0.0, 6), round(y + 0.0, 6), round(angle, 4), round(length, 6)
        b["mass"] = {"kg": m[bid][0], "comX": m[bid][1], "comY": m[bid][2], "distribution": "point", "source": "authored-estimate", "reference": "en-1999-1-1"}
        b["section"] = {"areaM2": sec[bid]["areaM2"], "secondMomentM4": sec[bid]["secondMomentM4"], "extremeFibreM": sec[bid]["extremeFibreM"], "material": "alu-mill", "title": SECTION_TITLES[bid]}
    stand = k.local(k.FRONT, ((k.DECK_X0 + k.DECK_X1) / 2, k.H_PLATFORM))
    for a in asset["anchors"]:
        sg = 1 if a["id"].endswith("left") else -1
        if a["id"].startswith("foot-front"):
            a.update({"x": 0.0, "y": 0.0, "z": round(sg * k.fz(0.0), 4), "footprintLengthM": 0.088, "footprintWidthM": 0.038})
        elif a["id"].startswith("foot-rear"):
            a.update({"x": round(k.REAR_LEN, 6), "y": 0.0, "z": round(sg * k.rz(0.0), 4), "footprintLengthM": 0.076, "footprintWidthM": 0.032})
        elif a["id"] == "standing-point":
            a.update({"x": round(stand[0], 6), "y": round(stand[1], 6)})
    th = asset["thermal"]
    for r in th["regions"]:
        if areas and r["id"] in areas:
            r["areaM2"] = areas[r["id"]]
        if r["id"] == "front-rails":
            r["title"] = {"en": "Front stiles, caps and grab bar", "de": "Vorderholme, Kappen und Haltebügel"}
        if r["id"] == "rear-frame":
            r["title"] = {"en": "Rear rails, rest tube and braces", "de": "Hinterholme, Auflagerohr und Traversen"}
        if r["id"] == "spreaders":
            r["layers"][0]["title"] = {"en": "Pressed zinc-plated steel channel 2 mm", "de": "Gepresstes verzinktes Stahlprofil 2 mm"}
        if r["id"] == "feet":
            r["title"] = {"en": "Rubber shoes", "de": "Gummischuhe"}
    asset["structure"]["assumptions"] = assumptions()
    return asset
