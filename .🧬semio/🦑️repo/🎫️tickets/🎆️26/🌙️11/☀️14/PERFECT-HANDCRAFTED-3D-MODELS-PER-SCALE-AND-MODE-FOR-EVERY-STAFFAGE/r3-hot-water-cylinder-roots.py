"""🌱 Round 3 hot-water cylinder: builds the candidate asset document with every root correction (anchors of the three-foot stand, connection mass, enamelled steel material and tank-wall layer, 2D of the feet, seam, inspection cover and dial at 1:50, texts) from the catalog file, writes it to `🗑️generated/r3-hot-water-cylinder/🔣️.json` with the source volumes merged in, and with `--apply` writes only the changed members into the catalog file in its own formatting (re-read right before, member order kept, `volumes` untouched).

Usage: python3 r3-hot-water-cylinder-roots.py [--apply]
"""
import copy
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/hot-water-cylinder/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-hot-water-cylinder", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "appliances", "hot-water-cylinder.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-hot-water-cylinder-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
PLAN_Z = -1

REFERENCES = [
    {"id": "viessmann-vitocell-100-v", "citation": "Viessmann Vitocell 100-V data sheets (types CVAA and CVAB-A, 160 to 300 l), read 2026-11-20: 200 l at 581 mm diameter, 605 mm over the connections and 1409 mm height (CVAA), 634 to 637 mm by 1349 mm (CVAB-A); enamelled steel tank with internal smooth-tube coil, rigid polyurethane foam insulation, adjustable feet, inspection and cleaning opening."},
    {"id": "din-28011", "citation": "DIN 28011:2012 Dished heads, torispherical (Kloepper type): crown radius equal to the outer diameter, knuckle radius one tenth of it, head depth 0.1935 D - 0.455 s."},
]
ENAMELLED = {"id": "enamelled-steel", "title": {"en": "Enamelled steel tank wall", "de": "Emaillierte Stahl-Behälterwand"}, "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"}
TANK_LAYER = {"id": "tank-wall", "title": {"en": "Enamelled steel tank wall 3 mm", "de": "Emaillierte Stahl-Behälterwand 3 mm"}, "kind": "metal", "material": "enamelled-steel", "thicknessM": 0.003, "resistanceM2KW": 6e-05, "heatCapacityJM2K": 10830, "source": "literature", "reference": "incropera-2018"}
TEXTS_50 = {
    "silhouette": {"en": "A 0.66 m by 1.48 m side elevation, 13 mm by 30 mm on paper: a tall insulated cylinder on three levelling feet (the right pair one behind the other) and a plinth, with a domed cover, four stacked jacket quarters, five side stubs with union nuts and one stub on the cover, the round thermometer in the upper half, the type plate, the inspection cover low down and the vertical lock seam of the skin. Front view: the 0.6 m cylinder on its three feet, the four quarters, the cover with its seam, the lock seam, the thermometer and the inspection cover seen edge-on on the left and the two front stubs with union nuts seen end-on on the axis. Top view: the 0.6 m cover with its seam ring, the hot-water stub, the rear and the front stub with union nuts and the edges of thermometer and inspection cover.", "de": "Eine Seitenansicht von 0,66 m mal 1,48 m, auf Papier 13 mm mal 30 mm: ein hoher gedämmter Zylinder auf drei Stellfüßen (das rechte Paar hintereinander) und einem Sockel, mit gewölbter Abdeckung, vier übereinanderliegenden Mantelvierteln, fünf seitlichen Stutzen mit Überwurfmuttern und einem Stutzen auf der Abdeckung, dem runden Thermometer in der oberen Hälfte, dem Typenschild, dem Revisionsdeckel unten und dem senkrechten Falz der Haut. Vorderansicht: der 0,6 m breite Zylinder auf seinen drei Füßen, die vier Viertel, die Abdeckung mit ihrer Fuge, der Falz, Thermometer und Revisionsdeckel links von der Kante gesehen und die beiden vorderen Stutzen mit Überwurfmuttern in der Stirnansicht auf der Achse. Draufsicht: die 0,6 m große Abdeckung mit ihrem Fugenring, der Warmwasserstutzen, der hintere und der vordere Stutzen mit Überwurfmuttern und die Kanten von Thermometer und Revisionsdeckel."},
    "retained": {"en": "The three levelling feet (pad, screw, socket), the plinth, the four jacket quarters (the thermal quarters), the domed cover with its seam, five side stubs with union nuts, the hot-water stub on the cover, the thermometer with bezel, three ticks and its needle, the type plate, the inspection cover and the lock seam of the skin over the full height.", "de": "Die drei Stellfüße (Teller, Schraube, Muffe), der Sockel, die vier Mantelviertel (die Wärmeviertel), die gewölbte Abdeckung mit ihrer Fuge, fünf seitliche Stutzen mit Überwurfmuttern, der Warmwasserstutzen auf der Abdeckung, das Thermometer mit Ring, drei Marken und seiner Nadel, das Typenschild, der Revisionsdeckel und der Falz der Haut über die ganze Höhe."},
    "merged": {"en": "The steel tank, the coil and the foam jacket are one shell per quarter; the third foot stands behind the right one in the side elevation.", "de": "Stahlbehälter, Rohrwendel und Schaummantel sind eine Schale je Viertel; der dritte Fuß steht in der Seitenansicht hinter dem rechten."},
}


def bone(asset, bid):
    """🦴 The bone with the given id."""
    return next(b for b in asset["rig"]["bones"] if b["id"] == bid)


def foot_poly(cx):
    """🦶 Side outline of one levelling foot: 50 mm pad with chamfer, M16 screw, 36 mm socket into the plinth."""
    half = [(0.025, 0), (0.025, 0.006), (0.019, 0.010), (0.008, 0.010), (0.008, 0.028), (0.018, 0.028), (0.018, 0.042)]
    right = [(cx + a, b) for a, b in half]
    left = [(cx - a, b) for a, b in reversed(half)]
    return [[K.r5(a), K.r5(b)] for a, b in right + left]


def path_of(polys):
    """〰️ One SVG path with one closed subpath per polygon."""
    return " ".join("M " + " L ".join("%s %s" % (K.r5(a), K.r5(b)) for a, b in p) + " Z" for p in polys)


def seam(sid, bone_id, x, y0, y1):
    """🧵 The vertical lock seam of the skin as an open detail stroke."""
    return {"id": sid, "bone": bone_id, "role": "detail", "z": 3.3, "filled": False, "tone": 0.4, "kind": "poly", "points": [[x, K.r5(y0)], [x, K.r5(y1)]], "closed": False}


def bump(sid, bone_id, ztone, cy, prof, flip=1):
    """🔘 Edge-on outline of a part on the -z face (dial bezel, inspection cover) seen in front or plan: prof = (depth below the jacket, half height) pairs from the jacket outwards."""
    up = [(flip * d, cy + h) for d, h in prof]
    down = [(flip * d, cy - h) for d, h in reversed(prof)]
    return {"id": sid, "bone": bone_id, "role": "detail", "z": 3.3, "filled": True, "tone": ztone, "kind": "poly", "points": [[K.r5(a), K.r5(b)] for a, b in up + down], "closed": True}


BEZEL = [(-0.296, 0.05), (-0.304, 0.05), (-0.308, 0.046)]
COVER = [(-0.289, 0.08), (-0.300, 0.08), (-0.306, 0.074), (-0.310, 0.06)]


def geometry50(view, shapes):
    """✏️ 1:50 drawings: three feet, lock seam over the full height, inspection cover, dial bezel edge-on."""
    out = []
    for s in shapes:
        s = copy.deepcopy(s)
        if s["id"] == "zip" or s["id"].startswith("seam-") or s["id"] == "inspection-cover" or (view != "elevation" and s["id"] == "dial-bezel"):
            continue
        if view == "elevation" and s["id"] in ("foot-l", "foot-r"):
            cx = K.FEET[0][0] if s["id"] == "foot-l" else K.FEET[1][0]
            s = {k: v for k, v in s.items() if k not in ("x", "y", "w", "h")}
            s["kind"], s["points"], s["closed"] = "poly", foot_poly(cx), True
        if view == "front" and s["id"] in ("foot-l", "foot-r"):
            s = {k: v for k, v in s.items() if k not in ("x", "y", "w", "h")}
            if s["id"] == "foot-l":
                s["kind"], s["points"], s["closed"] = "poly", foot_poly(0.0), True
            else:
                s["kind"], s["d"] = "path", path_of([foot_poly(K.FEET[1][1]), foot_poly(K.FEET[2][1])])
        out.append(s)
    if view == "elevation":
        idx = next(i for i, s in enumerate(out) if s["id"] == "label-line") + 1
        add = [seam("seam-4", "zone-4", 0.22, 0.012, K.ZONE_H), seam("seam-3", "zone-3", 0.22, 0, K.ZONE_H), seam("seam-2", "zone-2", 0.22, 0, K.ZONE_H), seam("seam-1", "zone-1", 0.22, 0, K.ZONE_H),
               {"id": "inspection-cover", "bone": "zone-4", "role": "detail", "z": 3.3, "filled": True, "tone": 0.8, "kind": "ellipse", "cx": 0, "cy": 0.17, "rx": 0.08, "ry": 0.08}]
        out[idx:idx] = add
    if view == "front":
        x = K.r5(K.jacket_point(0.22))
        add = [seam("seam-4", "zone-4", x, 0.072, 0.39), seam("seam-3", "zone-3", x, 0.39, 0.72), seam("seam-2", "zone-2", x, 0.72, 1.05), seam("seam-1", "zone-1", x, 1.05, 1.38),
               bump("dial-bezel", "zone-2", 0.95, 0.885, BEZEL), bump("inspection-cover", "zone-4", 0.8, 0.23, COVER)]
        out += add
    if view == "plan":
        out += [bump("dial-bezel", "zone-2", 0.95, 0.0, [(d, h) for d, h in BEZEL], 1), bump("inspection-cover", "zone-4", 0.8, 0.0, COVER, 1)]
        for s in out[-2:]:
            s["points"] = [[K.r5(b), K.r5(PLAN_Z * -a)] for a, b in s["points"]]
    return out


def roots(asset):
    """🌱 Every root correction in one place, applied to a copy of the catalog asset."""
    a = copy.deepcopy(asset)
    refs = a["provenance"]["references"]
    for r in REFERENCES:
        if not any(x["id"] == r["id"] for x in refs):
            refs.append(r)
    if not any(m["id"] == "enamelled-steel" for m in a["materials"]):
        a["materials"].append(ENAMELLED)
    for region in a["thermal"]["regions"]:
        if region["id"] in ("zone-1", "zone-2", "zone-3", "zone-4") and not any(l["id"] == "tank-wall" for l in region["layers"]):
            region["layers"].insert(0, copy.deepcopy(TANK_LAYER))
    c = bone(a, "connections")["mass"]
    c["comX"], c["comY"] = -0.02, 0.84
    for anc in a["anchors"]:
        if anc["id"] == "foot-left":
            anc["x"] = K.FEET[0][0]
            anc["title"] = {"en": "Left foot", "de": "Linker Fuß"}
        if anc["id"] == "foot-right":
            anc["x"] = K.FEET[1][0]
            anc["title"] = {"en": "Right pair of feet", "de": "Rechtes Fußpaar"}
            anc["footprintWidthM"] = 0.05
        if anc["id"] == "foot-left":
            anc["footprintWidthM"] = 0.05
        if anc["id"] in ("foot-left", "foot-right"):
            anc["footprintLengthM"] = 0.05
    a["structure"]["assumptions"][0] = {
        "en": "The empty 80 kg cylinder stands on three levelling feet at 120 degrees on a 0.25 m circle: in the side elevation the left foot stands alone at x -0.25 m and the right pair one behind the other at x 0.125 m, so the left foot carries one third and the pair two thirds; filled with 200 kg of water (280 kg, 2.7 kN) the floor carries it.",
        "de": "Der leere Speicher von 80 kg steht auf drei Stellfüßen im Abstand von 120 Grad auf einem Kreis von 0,25 m: in der Seitenansicht steht der linke Fuß allein bei x -0,25 m und das rechte Paar hintereinander bei x 0,125 m, sodass der linke Fuß ein Drittel und das Paar zwei Drittel trägt; mit 200 kg Wasser gefüllt (280 kg, 2,7 kN) trägt ihn der Boden.",
    }
    rep = a["representations"]["50"]
    rep["geometry"] = {v: geometry50(v, s) for v, s in rep["geometry"].items()}
    for k, v in TEXTS_50.items():
        rep["decisions"][k] = v
    th = rep["decisions"].get("thinElements")
    if th:
        th["en"] = th["en"].replace("label lines and the zip", "label lines and the lock seam")
        th["de"] = th["de"].replace("Schildlinien und der Reißverschluss", "Schildlinien und der Falz")
    return a


def apply(want):
    """✍️ Writes the changed members (never `volumes`) into the catalog file, in its two-space style."""
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text, "    ", key)
        body = json.dumps(want[key], indent=2, ensure_ascii=False).replace("\n", "\n    ")
        text = text[:start] + "    " + json.dumps(key) + ": " + body + text[end:]
    got = json.loads(text)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    open(CATALOG, "w", encoding="utf-8").write(text)
    return changed


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    want = roots(doc["asset"])
    cand = copy.deepcopy(doc)
    cand["asset"] = copy.deepcopy(want)
    cand["asset"]["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    with open(CANDIDATE, "w", encoding="utf-8") as f:
        json.dump(cand, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("candidate", CANDIDATE)
    if "--apply" in sys.argv:
        print("written", CATALOG, "members:", ", ".join(apply(want)) or "none")


if __name__ == "__main__":
    main()
