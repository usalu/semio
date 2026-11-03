"""🛢️ pa: OI-21 - a real 200 L cylinder: jacket 0.60 m (60 mm foam round a 0.48 m tank), 1.48 m high (quarters 0.33 m), 80 kg dry; the 200 kg of water are a separate filling - run once."""
from jio import load, save

OLD_Q, NEW_Q = 0.36, 0.33
OLD_TOP, NEW_TOP = 1.6, 1.48
HALF_OLD, HALF_NEW = 0.27, 0.30
SX = HALF_NEW / HALF_OLD
DIAL_SHIFT = -0.015


def stretch(y):
    return round(0.06 + (y - 0.06) * NEW_Q / OLD_Q, 5)


def swap(holder, pairs):
    for lang, swaps in pairs.items():
        for old, new in swaps:
            if old in holder[lang]:
                holder[lang] = holder[lang].replace(old, new)


path, data = load("hot-water-cylinder")
a = data["asset"]
a["dimensions"].update({"heightM": NEW_TOP, "widthM": 0.66, "depthM": 0.6, "massKg": 80})

masses = {"base": (5, 0.03), "zone-4": (16, 0.165), "zone-3": (16, 0.165), "zone-2": (16, 0.165), "zone-1": (16, 0.165), "cap": (6, 0.04), "connections": (5, 0.74)}
for bone in a["rig"]["bones"]:
    if bone["id"] in masses:
        bone["mass"]["kg"], bone["mass"]["comY"] = masses[bone["id"]]
    if bone["id"] in ("zone-3", "zone-2", "zone-1", "cap"):
        bone["y"] = NEW_Q
    if bone["id"].startswith("zone-"):
        bone["lengthM"] = NEW_Q
    if bone["id"] == "connections":
        bone["lengthM"] = NEW_TOP
    if bone["id"] == "needle":
        bone["y"] = round(NEW_Q / 2, 5)
for anchor in a["anchors"]:
    if anchor["id"] == "foot-left":
        anchor["x"] = -0.215
    if anchor["id"] == "foot-right":
        anchor["x"] = 0.215


def fix_shape(shape):
    sid, bone = shape["id"], shape["bone"]
    if shape["kind"] == "rect" and bone.startswith("zone-") or sid in ("lower", "upper"):
        shape["x"], shape["w"] = -HALF_NEW, 2 * HALF_NEW
        shape["h"] = round(shape["h"] * NEW_Q / OLD_Q, 5)
        return
    if sid in ("foot-l", "foot-r"):
        shape["x"] = -0.25 if sid == "foot-l" else 0.18
    elif sid == "feet":
        shape["x"], shape["w"] = -0.25, 0.5
    elif sid == "base-ring":
        shape["x"], shape["w"] = -0.28, 0.56
    elif sid in ("cap", "cap-seam"):
        shape["points"] = [[round(px * SX, 5), py] for px, py in shape["points"]]
    elif bone == "connections":
        if sid == "hot-out":
            shape["y"] = round(NEW_TOP - (OLD_TOP - shape["y"]), 5)
            shape["x"] = round(shape["x"] * SX, 5)
        else:
            shape["y"] = stretch(shape["y"])
            if sid.startswith("nut-"):
                shape["x"] = -0.33 if shape["x"] < 0 else 0.318
            elif sid in ("side-l",):
                shape["x"] = -0.33
            elif sid in ("side-r",):
                shape["x"] = 0.29
            else:
                shape["x"] = -0.33 if shape["x"] < 0 else 0.3
    elif bone in ("zone-2", "zone-3") and sid != "needle":
        if sid == "zip":
            shape["points"] = [[px, 0.3 if py > 0.3 else py] for px, py in shape["points"]]
        else:
            if "points" in shape:
                shape["points"] = [[px, round(py + DIAL_SHIFT, 5)] for px, py in shape["points"]]


for rep in a["representations"].values():
    for shape in rep.get("geometry", {}).get("elevation", []):
        fix_shape(shape)

for region in a["thermal"]["regions"]:
    if region["id"].startswith("zone-"):
        region["areaM2"] = 0.62
        region["characteristicLengthM"] = NEW_Q
        layer = region["layers"][0]
        layer["thicknessM"], layer["resistanceM2KW"] = 0.06, 1.7
        layer["title"] = {"en": "Polyurethane foam jacket 60 mm with sheet skin", "de": "Polyurethan-Schaummantel 60 mm mit Blechhaut"}
    if region["id"] == "top-cap":
        region["areaM2"] = 0.31

swap(a["summary"], {
    "en": [("of 0.6 m diameter and 1.6 m height in front elevation", "of 0.6 m diameter and 1.48 m height in front elevation")],
    "de": [("von 0,6 m Durchmesser und 1,6 m Höhe in der Vorderansicht", "von 0,6 m Durchmesser und 1,48 m Höhe in der Vorderansicht")],
})
swap(a["provenance"]["notes"], {
    "en": [
        ("of about 200 litres (product class of EN 12897): outer diameter of the foam jacket 0.54 m, 0.6 m with the side stubs, height 1.6 m with the dome, standing on feet 0.06 m high; the mass of 275 kg is the tank filled with water (authored estimate: 200 kg water, 75 kg steel, coil, jacket and fittings).",
         "of 200 litres (product class of EN 12897, as built by the usual makers at 0.58 to 0.60 m and 1.3 to 1.5 m): outer diameter of the foam jacket 0.60 m (a steel tank of 0.48 m with 60 mm of foam), 0.66 m with the side stubs, height 1.48 m with the dome, standing on feet 0.06 m high; the tank body of about 1.1 m holds the 200 litres. The mass of 80 kg is the empty cylinder like the dry mass of the other appliances (authored estimate: 62 kg steel tank and coil, foam jacket, cover and fittings); the 200 kg of water are the filling and are not part of massKg (filled 280 kg, 2.7 kN on the floor)."),
        ("76 W (energy label class B", "76 W (the 60 mm of foam, R 1.7, lets about 51 W through the jacket and the feet, seams and stub bridges add the rest of the 61 W of the quarters; energy label class B"),
    ],
    "de": [
        ("von etwa 200 Litern (Produktklasse nach EN 12897): Außendurchmesser des Schaummantels 0,54 m, mit den seitlichen Stutzen 0,6 m, Höhe 1,6 m mit der Wölbung, auf Füßen von 0,06 m Höhe; die Masse von 275 kg ist der mit Wasser gefüllte Speicher (Schätzwert: 200 kg Wasser, 75 kg Stahl, Rohrwendel, Mantel und Armaturen).",
         "von 200 Litern (Produktklasse nach EN 12897, bei den üblichen Herstellern 0,58 bis 0,60 m und 1,3 bis 1,5 m): Außendurchmesser des Schaummantels 0,60 m (ein Stahlbehälter von 0,48 m mit 60 mm Schaum), mit den seitlichen Stutzen 0,66 m, Höhe 1,48 m mit der Wölbung, auf Füßen von 0,06 m Höhe; der Behälterkörper von etwa 1,1 m fasst die 200 Liter. Die Masse von 80 kg ist der leere Speicher wie die Trockenmasse der übrigen Geräte (Schätzwert: 62 kg Stahlbehälter und Rohrwendel, Schaummantel, Abdeckung und Armaturen); die 200 kg Wasser sind die Füllung und gehören nicht zu massKg (gefüllt 280 kg, 2,7 kN auf dem Boden)."),
        ("76 W beträgt (Grenze", "76 W beträgt (die 60 mm Schaum, R 1,7, lassen etwa 51 W durch den Mantel und Füße, Fugen und Stutzenbrücken liefern den Rest der 61 W der Viertel; Grenze"),
    ],
})
for item in a["structure"]["assumptions"]:
    swap(item, {
        "en": [("The filled 275 kg tank stands on two feet in the plane of the view (a third foot lies behind them); the load of 2.7 kN is carried by the floor.", "The empty 80 kg cylinder stands on two feet in the plane of the view (a third foot lies behind them); filled with 200 kg of water (280 kg, 2.7 kN) the floor carries it.")],
        "de": [("Der gefüllte Speicher von 275 kg steht auf zwei Füßen in der Bildebene (ein dritter liegt dahinter); die Last von 2,7 kN trägt der Boden.", "Der leere Speicher von 80 kg steht auf zwei Füßen in der Bildebene (ein dritter liegt dahinter); mit 200 kg Wasser gefüllt (280 kg, 2,7 kN) trägt ihn der Boden.")],
    })
for key, pairs in {
    "50": {
        "en": [("A 0.6 m by 1.6 m front elevation, 12 mm by 32 mm on paper", "A 0.66 m by 1.48 m front elevation, 13 mm by 30 mm on paper")],
        "de": [("Eine Vorderansicht von 0,6 m mal 1,6 m, auf Papier 12 mm mal 32 mm", "Eine Vorderansicht von 0,66 m mal 1,48 m, auf Papier 13 mm mal 30 mm")],
    },
    "100": {
        "en": [("A 6 mm by 16 mm front elevation", "A 7 mm by 15 mm front elevation")],
        "de": [("Eine Vorderansicht von 6 mm mal 16 mm", "Eine Vorderansicht von 7 mm mal 15 mm")],
    },
    "200": {
        "en": [("A 3 mm by 8 mm mark", "A 3 mm by 7 mm mark")],
        "de": [("Ein Zeichen von 3 mm mal 8 mm", "Ein Zeichen von 3 mm mal 7 mm")],
    },
}.items():
    swap(a["representations"][key]["decisions"]["silhouette"], pairs)
swap(a["representations"]["500"]["reason"], {
    "en": [("the 0.6 m by 1.6 m cylinder is 1.2 mm by 3.2 mm", "the 0.6 m by 1.48 m cylinder is 1.2 mm by 3.0 mm")],
    "de": [("der 0,6 m mal 1,6 m große Zylinder 1,2 mm mal 3,2 mm groß", "der 0,6 m mal 1,48 m große Zylinder 1,2 mm mal 3,0 mm groß")],
})
save(path, data)
print("ok")
