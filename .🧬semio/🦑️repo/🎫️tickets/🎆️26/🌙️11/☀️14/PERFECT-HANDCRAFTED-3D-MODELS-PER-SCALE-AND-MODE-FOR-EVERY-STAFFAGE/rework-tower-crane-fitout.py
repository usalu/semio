"""🏗️ Tower crane, second round, continuation: the fit-out a real hammerhead crane has and the model lacked, with hand-set numbers.

Adds at every scale it is visible at (3D body, thermal, structure, 2D in three views): 50 mm pendant tie bars instead of 16 mm,
the counter-jib deck with handrails, handrails round the slewing platform, mast section joints every 4 m and rest platforms every 8 m,
the hoist rope from the winch over the head sheave down to a heel sheave and along the jib to the tip anchor, and a portal brace
in the two front panels of the tower head the rope passes. Idempotent. Usage: python3 rework-tower-crane-fitout.py [--apply]
"""
import copy
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"
SOURCE = os.path.join(HERE, "volumes/machinery/tower-crane.json")
CANDIDATE = os.path.join(HERE, "🗑️generated/rework-tower-crane/candidate.json")
SCALES = ("50", "100", "200", "500")
STEEL, ROPE = "steel-s355", "wire-rope"
RAIL_R, ROPE_R, TIE_R = 0.02, 0.007, 0.025
DECK_TOP = -0.7
ROPE_LEVEL_JIB = -1.062
HEEL = (0.611, 3.538)
DOWN_FROM, DOWN_TO = (0.411, 3.538, 0.0), (0.25, 12.0, 0.0)
UP_FROM, UP_TO = (-3.8655, 6.96, 0.0), (-0.204, 12.144, 0.0)
PORTAL = {
    "tower-head-diagonal-5": ((0.97297, 2.6, 0.75243), (0.97297, 2.6, -0.75243), (0.74595, 4.0, 0.0)),
    "tower-head-diagonal-19": ((0.39312, 7.0, 0.37553), (0.39312, 7.0, -0.37553), (0.26732, 8.6, 0.0)),
}
NEW_PREFIXES = ("counter-deck", "counter-rail-", "slew-rail-", "mast-joint-", "mast-rest-", "hoist-rope-", "hoist-heel-", "winch-fairlead", "jib-rope-anchor")


def rnd(value):
    return round(value, 5)


def rod(sid, bone, a, b, radius, material=STEEL, role="detail", tone=0.35, repeat=None):
    length = math.dist(a, b)
    solid = {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[0, radius], [rnd(length), radius]], "axis": [rnd((b[i] - a[i]) / length) for i in range(3)], "origin": [rnd(v) for v in a], "slender": True}
    if repeat:
        solid["repeat"] = {"count": repeat[0], "translation": list(repeat[1])}
    return solid


def box(sid, bone, size, at, tone=0.4, sheet=False, repeat=None, material=STEEL):
    solid = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": "body", "tone": tone, "size": list(size), "transform": {"translation": list(at)}}
    if sheet:
        solid["sheet"] = True
    if repeat:
        solid["repeat"] = {"count": repeat[0], "translation": list(repeat[1])}
    return solid


def rails(scale):
    """🪜 Handrails as rods: (id, bone, from, to, repeat). Posts and top rail down to 1:200, the knee rail down to 1:100, nothing at 1:500."""
    if scale != "50":
        return []
    out = []
    knee = scale == "50"
    top, mid = DECK_TOP - 1.1, DECK_TOP - 0.55
    for side, z in (("l", 0.62), ("r", -0.62)):
        out.append((f"counter-rail-post-{side}-tail", "counter-jib", (7.3, DECK_TOP, z), (7.3, top, z), (6, (1.83, 0, 0))))
        out.append((f"counter-rail-post-{side}-head", "counter-jib", (0.9, DECK_TOP, z), (0.9, top, z), (2, (1.9, 0, 0))))
        for name, y in (("top", top),) + ((("mid", mid),) if knee else ()):
            out.append((f"counter-rail-{name}-{side}-tail", "counter-jib", (7.3, y, z), (16.45, y, z), None))
            out.append((f"counter-rail-{name}-{side}-head", "counter-jib", (0.9, y, z), (2.8, y, z), None))
    for name, y in (("top", top),) + ((("mid", mid),) if knee else ()):
        out.append((f"counter-rail-{name}-end", "counter-jib", (16.45, y, -0.62), (16.45, y, 0.62), None))
    out.append(("slew-rail-post-l", "slewing", (-2.3, 1.2, 1.15), (-2.3, 2.3, 1.15), (5, (1.4, 0, 0))))
    out.append(("slew-rail-post-r", "slewing", (-2.3, 1.2, -1.15), (-2.3, 2.3, -1.15), (3, (1.4, 0, 0))))
    out.append(("slew-rail-post-r-cab", "slewing", (1.4, 1.2, -1.15), (1.4, 2.3, -1.15), None))
    for name, y in (("top", 2.3),) + ((("mid", 1.75),) if knee else ()):
        out.append((f"slew-rail-{name}-l", "slewing", (-2.3, y, 1.15), (3.3, y, 1.15), None))
        out.append((f"slew-rail-{name}-r", "slewing", (-2.3, y, -1.15), (1.4, y, -1.15), None))
        out.append((f"slew-rail-{name}-back", "slewing", (-2.3, y, -1.15), (-2.3, y, 1.15), None))
        out.append((f"slew-rail-{name}-front", "slewing", (3.3, y, -0.85), (3.3, y, 1.15), None))
    return out


def ropes():
    return [
        ("hoist-rope-up", "slewing", UP_FROM, UP_TO),
        ("hoist-rope-down", "slewing", DOWN_FROM, DOWN_TO),
        ("hoist-rope-jib", "jib", (HEEL[0], ROPE_LEVEL_JIB, 0.0), (54.9, ROPE_LEVEL_JIB, 0.0)),
    ]


def plates(scale):
    """🧱 Box and plate parts: (id, bone, size, centre, sheet, repeat, structural)."""
    coarse = scale in ("200", "500")
    out = [] if scale == "500" else [("counter-deck", "counter-jib", (15.6, 0.03, 1.24), (8.65, -0.685, 0), coarse, None, False)]
    for sx, sv in ((1, 1), (1, -1), (-1, 1), (-1, -1)):
        tag = ("p" if sv > 0 else "n") + "x" + ("p" if sx > 0 else "n") + "z"
        out.append((f"mast-joint-{tag}", "mast", (0.36, 0.22, 0.22), (4, -0.92 * sv, 0.92 * sx), False, (9, (4, 0, 0)), True))
    if scale != "500":
        out.append(("mast-rest-platform", "mast", (0.03, 1.26, 1.8), (8.06, 0.27, 0), coarse, (4, (8, 0, 0)), False))
    out.append(("jib-rope-anchor", "jib", (0.08, 0.36, 0.08), (54.9, -0.92, 0), False, None, True))
    if scale != "500":
        out.append(("winch-fairlead", "winch", (0.3, 0.06, 0.2), (1.13, 1.83, 0), False, None, False))
    return out


def heel(scale):
    """🎡 Heel sheave below the jib foot: sheave, pin and the two hanger plates from the head tie at 4.0 m (one block at 1:500)."""
    if scale == "500":
        return []
    hanger = [[0.546, 3.45], [0.676, 3.45], [0.8, 4.03], [0.69, 4.03]]
    thick = 0.012 if scale in ("50",) else 0.03
    inner = 0.045 if scale == "50" else 0.03
    out = [{"id": "hoist-heel-sheave", "kind": "revolve", "bone": "slewing", "material": STEEL, "role": "detail", "tone": 0.3, "profile": [[0, 0.2], [0.06, 0.2]], "axis": [0, 0, 1], "origin": [HEEL[0], HEEL[1], -0.03]}]
    for side, sign in (("l", 1), ("r", -1)):
        lo, hi = sorted((sign * inner, sign * (inner + thick)))
        out.append({"id": f"hoist-heel-hanger-{side}", "kind": "extrude", "bone": "slewing", "material": STEEL, "role": "body", "tone": 0.4, "profile": hanger, "plane": "xy", "from": rnd(lo), "to": rnd(hi)})
    if scale == "50":
        out.append({"id": "hoist-heel-pin", "kind": "revolve", "bone": "slewing", "material": STEEL, "role": "detail", "tone": 0.3, "profile": [[0, 0.025], [0.114, 0.025]], "axis": [0, 0, 1], "origin": [HEEL[0], HEEL[1], -0.057]})
    return out


def is_new(sid):
    return sid.startswith(NEW_PREFIXES) or sid.endswith("-portal-b")


def strip(entries):
    return [e for e in entries if not is_new(e["id"].removeprefix("th-").removeprefix("member-"))]


def edit_volumes(volumes):
    for scale in SCALES:
        volume = volumes[scale]
        body = strip(volume["body"])
        ids = {s["id"] for s in body}
        added, structural = [], []
        for solid in body:
            if solid["id"].startswith("pendant-"):
                solid["profile"] = [[0, TIE_R], [solid["profile"][1][0], TIE_R]]
                solid["material"] = STEEL
            if solid["id"] in PORTAL:
                a, b, m = PORTAL[solid["id"]]
                radius = solid["profile"][1][1]
                new = rod(solid["id"], "slewing", a, m, radius, role=solid["role"], tone=solid["tone"])
                solid.clear()
                solid.update(new)
                added.append(rod(new["id"] + "-portal-b", "slewing", b, m, radius, role=new["role"], tone=new["tone"]))
                structural.append(new["id"] + "-portal-b")
        for sid, bone, a, b, repeat in rails(scale):
            added.append(rod(sid, bone, a, b, RAIL_R, repeat=repeat))
        for sid, bone, a, b in ropes():
            added.append(rod(sid, bone, a, b, ROPE_R, material=ROPE, tone=0.2))
            structural.append(sid)
        for sid, bone, size, at, sheet, repeat, load in plates(scale):
            added.append(box(sid, bone, size, at, sheet=sheet, repeat=repeat))
            if load:
                structural.append(sid)
        for solid in heel(scale):
            added.append(solid)
            structural.append(solid["id"])
        assert not ids & {s["id"] for s in added}
        volume["body"] = body + added
        thermal = strip(volume["thermal"])
        structure = strip(volume["structure"])
        for solid in added:
            region = "rope" if solid["material"] == ROPE else "winch-house" if solid["bone"] == "winch" else "lattice"
            thermal.append({"id": "th-" + solid["id"], "of": solid["id"], "region": region})
        for sid in structural:
            bone = next(s["bone"] for s in added if s["id"] == sid)
            structure.append({"id": "member-" + sid, "of": sid, "member": bone, "part": "member"})
        volume["thermal"], volume["structure"] = thermal, structure


def pen(diameter, scale):
    return max(0.13, round(diameter * 1000 / int(scale), 3))


def member(sid, bone, a, b, diameter, scale, region="lattice", material=STEEL, depth=3.2):
    return {"kind": "member", "from": dict(zip("xyz", map(rnd, a))), "to": dict(zip("xyz", map(rnd, b))), "section": {"kind": "circle", "d": diameter}, "id": sid, "bone": bone, "role": "detail", "z": depth, "filled": False, "tone": 0.3, "region": region, "material": material, "penMm": pen(diameter, scale)}


def swept(sid, bone, x, y, w, h, z0, z1, region="lattice", material=STEEL, role="body"):
    return {"kind": "rect", "x": rnd(x), "y": rnd(y), "w": w, "h": h, "id": sid, "bone": bone, "role": role, "z": 3.2, "filled": True, "tone": 0.4, "region": region, "material": material, "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}


def rail_shape(sid, bone, a, b):
    """🪜 A handrail tube as the swept rectangle of its side view (no `member` shape: a handrail is no member of the load path)."""
    r = RAIL_R
    if abs(b[0] - a[0]) > 1e-9:
        shape = swept(sid, bone, min(a[0], b[0]), a[1] - r, rnd(abs(b[0] - a[0])), 2 * r, rnd(a[2] - r), rnd(a[2] + r), role="detail")
    elif abs(b[1] - a[1]) > 1e-9:
        shape = swept(sid, bone, a[0] - r, min(a[1], b[1]), 2 * r, rnd(abs(b[1] - a[1])), rnd(a[2] - r), rnd(a[2] + r), role="detail")
    else:
        shape = swept(sid, bone, a[0] - r, a[1] - r, 2 * r, 2 * r, rnd(min(a[2], b[2])), rnd(max(a[2], b[2])), role="detail")
    shape["tone"] = 0.3
    return shape


def shapes_of(scale):
    out = []
    for sid, bone, a, b, repeat in rails(scale):
        count, step = repeat or (1, (0, 0, 0))
        for k in range(count):
            shift = [step[i] * k for i in range(3)]
            out.append(rail_shape(sid + (f"-{k + 1}" if repeat else ""), bone, [a[i] + shift[i] for i in range(3)], [b[i] + shift[i] for i in range(3)]))
    for sid, bone, a, b in ropes():
        out.append(member(sid, bone, a, b, 2 * ROPE_R, scale, region="rope", material=ROPE))
    for sid, bone, size, at, _sheet, repeat, _load in plates(scale):
        count, step = repeat or (1, (0, 0, 0))
        for k in range(count):
            cx, cy, cz = (at[i] + step[i] * k for i in range(3))
            region = "winch-house" if bone == "winch" else "lattice"
            out.append(swept(sid + (f"-{k + 1}" if repeat else ""), bone, cx - size[0] / 2, cy - size[1] / 2, size[0], size[1], rnd(cz - size[2] / 2), rnd(cz + size[2] / 2), region=region))
    if scale != "500":
        out.append({"id": "hoist-heel-sheave", "bone": "slewing", "role": "detail", "z": 3.3, "filled": True, "tone": 0.3, "kind": "ellipse", "cx": HEEL[0], "cy": HEEL[1], "rx": 0.2, "ry": 0.2, "region": "lattice", "material": STEEL, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.03, "z1": 0.03}})
    if scale != "500":
        out.append({"kind": "poly", "points": [[0.546, 3.45], [0.676, 3.45], [0.8, 4.03], [0.69, 4.03]], "closed": True, "id": "hoist-heel-hanger", "bone": "slewing", "role": "body", "z": 3.25, "filled": True, "tone": 0.4, "region": "lattice", "material": STEEL, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.06, "z1": 0.06}})
    return out


def edit_views(asset):
    for scale in SCALES:
        geometry = asset["representations"][scale]["geometry"]
        new = shapes_of(scale)
        for view in ("elevation", "front", "plan"):
            shapes = [s for s in geometry[view] if not is_new(s["id"]) and not (scale == "200" and s["id"] == "winch-skids")]
            for shape in shapes:
                if scale == "200" and shape["id"] == "winch-house" and shape.get("kind") == "rect" and shape["y"] == 0:
                    shape["y"], shape["h"] = -0.055, rnd(shape["h"] + 0.055)
            portal = []
            for shape in shapes:
                if shape["id"].startswith("pendant-"):
                    shape["material"] = STEEL
                    shape["section"] = {"kind": "circle", "d": 2 * TIE_R}
                    shape["filled"] = scale in ("50", "100")
                    shape["penMm"] = 0.13 if shape["filled"] else pen(2 * TIE_R, scale)
                for sid, (a, b, m) in PORTAL.items():
                    if shape["id"] in (sid, sid + "-1", sid + "-stroke"):
                        diameter = 0.06
                        first = member(shape["id"], "slewing", a, m, diameter, scale, depth=shape.get("z", 1.8))
                        first["role"] = shape.get("role", "body")
                        shape.clear()
                        shape.update(first)
                        second = copy.deepcopy(first)
                        second["id"] = sid + "-portal-b"
                        second["from"] = dict(zip("xyz", b))
                        portal.append(second)
                size = shape["section"].get("d", shape["section"].get("h", 0)) if shape["kind"] == "member" else 0
                if (view == "plan" or (view == "elevation" and scale != "50") or (view == "front" and scale == "50")) and shape["bone"] in ("mast", "jib", "counter-jib") and size >= 0.12:
                    shape["filled"] = scale != "500"
                    shape["penMm"] = 0.13 if scale != "500" else pen(size, scale)
            geometry[view] = shapes + portal + copy.deepcopy(new)


TEXT = {
    "50": ("Continuation: the three pendants are 50 mm tie bars (16 mm could not carry the jib); the counter-jib has a 30 mm deck between its top chords and 1.1 m handrails (40 mm tube, knee rail, posts every 1.83 m) that stop at the winch house; the slewing platform has the same handrail on three sides and up to the cab; the mast is built of 4 m sections with a 360 mm joint sleeve on every chord and has a rest platform beside the ladder every 8 m; the hoist rope runs from the winch through a roof fairlead over the head sheave, down inside the tower head to a heel sheave hung from the head tie at 4.0 m, and along the jib at the level of the trolley sheaves to an anchor under the tip; the two front panels of the head the rope passes carry a portal brace (two bars to the middle of the tie) instead of one diagonal.", "Fortsetzung: Die drei Abspannungen sind 50-mm-Zugstangen (16 mm könnten den Ausleger nicht tragen); der Gegenausleger hat einen 30-mm-Belag zwischen den Obergurten und 1,1 m hohe Geländer (40-mm-Rohr, Knieleiste, Pfosten alle 1,83 m), die am Windenhaus enden; die Drehbühne hat dasselbe Geländer auf drei Seiten und bis zur Kabine; der Mast besteht aus 4-m-Schüssen mit einer 360-mm-Stoßmuffe an jedem Gurt und hat alle 8 m ein Ruhepodest neben der Leiter; das Hubseil läuft von der Winde durch eine Dachführung über die Kopfrolle, im Turmkopf hinab zu einer Fußrolle, die am Kopfriegel auf 4,0 m hängt, und auf Höhe der Katzrollen den Ausleger entlang zu einer Verankerung unter der Spitze; die zwei vorderen Felder des Kopfes, durch die das Seil läuft, haben einen Portalverband (zwei Stäbe zur Riegelmitte) statt einer Diagonale."),
    "100": ("Continuation: 50 mm pendant tie bars, counter-jib deck, mast joint sleeves and rest platforms, the hoist rope run over the head sheave and along the jib and the portal brace of the head are modelled as at 1:50; there are no handrails (the product measures a 40 mm tube at under 0.2 mm at 1:150, the end of this band); the heel sheave hangs between two 30 mm plates without a separate pin.", "Fortsetzung: 50-mm-Zugstangen, Belag des Gegenauslegers, Stoßmuffen und Ruhepodeste des Mastes, der Hubseillauf über die Kopfrolle und den Ausleger entlang und der Portalverband des Kopfes sind wie in 1:50 modelliert; Geländer gibt es keine (das Produkt misst ein 40-mm-Rohr bei 1:150, dem Ende dieses Bereichs, mit unter 0,2 mm); die Fußrolle hängt zwischen zwei 30-mm-Laschen ohne eigenen Bolzen."),
    "200": ("Continuation: 50 mm pendant tie bars; no handrails (a 40 mm tube prints at 0.2 mm and below in this band); deck and rest platforms are declared sheets of their true 30 mm; joint sleeves, hoist rope run, heel sheave between 30 mm plates and portal brace as at 1:100.", "Fortsetzung: 50-mm-Zugstangen; keine Geländer (ein 40-mm-Rohr druckt in diesem Bereich mit 0,2 mm und weniger); Belag und Ruhepodeste sind deklarierte Bleche mit ihren wahren 30 mm; Stoßmuffen, Hubseillauf, Fußrolle zwischen 30-mm-Laschen und Portalverband wie in 1:100."),
    "500": ("Continuation: 50 mm pendant tie bars; no handrails (0.08 mm on paper); no deck, rest platforms or heel sheave (30 mm plates and a 60 mm disc do not print at 1:1000), the joint sleeves as 220 mm blocks, the hoist rope run at its true 14 mm as a slender line. The lattice bars keep their true sections as slender members at this scale on purpose: a crane drawn at 1:500 is its line pattern, and thicker typified bars would close the panels.", "Fortsetzung: 50-mm-Zugstangen; keine Geländer (0,08 mm auf dem Papier); weder Belag noch Ruhepodeste noch Fußrolle (30-mm-Bleche und eine 60-mm-Scheibe drucken in 1:1000 nicht), die Stoßmuffen als 220-mm-Klötze, der Hubseillauf mit seinen wahren 14 mm als schlanke Linie. Die Gitterstäbe behalten in diesem Maßstab bewusst ihre wahren Querschnitte als schlanke Stäbe: Ein Kran in 1:500 ist sein Linienbild, dickere typisierte Stäbe würden die Felder schließen."),
}


def edit_texts(asset, volumes):
    for scale in SCALES:
        en, de = TEXT[scale]
        for target in (volumes[scale]["decisions"][0], asset["representations"][scale].get("decisions")):
            if not isinstance(target, dict) or "en" not in target:
                continue
            for language, text in (("en", en), ("de", de)):
                marker = " Continuation: " if language == "en" else " Fortsetzung: "
                target[language] = target[language].split(marker)[0].replace("three 16 mm pendant ropes", "three 50 mm pendant tie bars").replace("drei 16-mm-Abspannseile", "drei 50-mm-Abspannstangen") + " " + text
    for region in asset["thermal"]["regions"]:
        if region["id"] == "lattice":
            region["areaM2"] = 513
        if region["id"] == "pendants":
            region["areaM2"] = 13
            region["material"] = STEEL
        if region["id"] == "rope":
            region["areaM2"] = 7.1
    for assumption in asset["structure"]["assumptions"]:
        assumption["en"] = assumption["en"].replace("The jib is a cantilever from the slewing unit; the pendant ties are not members.", "The analytic bone model treats the jib as a cantilever from the slewing unit; the structure solids show the real load path with the three 50 mm pendant tie bars as members.")
        assumption["de"] = assumption["de"].replace("Der Ausleger ist ein Kragarm am Drehwerk; die Abspannungen sind keine Stäbe.", "Das analytische Stabmodell behandelt den Ausleger als Kragarm am Drehwerk; die Tragwerkskörper zeigen den wirklichen Lastpfad mit den drei 50-mm-Zugstangen als Stäben.")


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    source = json.load(open(SOURCE, encoding="utf-8"))
    edit_volumes(source["volumes"])
    edit_views(document["asset"])
    edit_texts(document["asset"], source["volumes"])
    document["asset"]["volumes"] = source["volumes"]
    for scale in SCALES:
        print(scale, {name: len(source["volumes"][scale][name]) for name in ("body", "thermal", "structure")}, {view: len(document["asset"]["representations"][scale]["geometry"][view]) for view in ("elevation", "front", "plan")})
    if "--apply" in sys.argv:
        json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        json.dump(document, open(CATALOG, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print("applied")
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print("candidate", CANDIDATE)


if __name__ == "__main__":
    main()
