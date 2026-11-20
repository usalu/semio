"""🌱 Round 3 wheelchair ramp: the root values outside `volumes` (masses, section, support anchors, areas, notes and the 2D drawings of every scale as true views of the handcrafted body).

Usage: python3 r3-wheelchair-ramp-roots.py            writes the candidate document to 🗑️generated/r3-wheelchair-ramp/🔣️.json
       python3 r3-wheelchair-ramp-roots.py --apply    re-reads the catalog file and replaces only the changed members in place
"""
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-wheelchair-ramp-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
r5 = K.r5
CATALOG = os.path.join(T, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🧱️components", "wheelchair-ramp", "🔣️.json")
MEMBERS = ["provenance", "dimensions", "rig", "anchors", "representations", "thermal"]
AREAS = {"ramp-side": 9.15, "kerb": 3.77}
R = K.POST_R
KERB_SIDE = [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0.7], [10.2, 0.7], [6.6, 0.4], [5.1, 0.4]]
SUPPORTS = [(3.3, 3.6), (5.85, 1.5), (8.4, 3.6), (10.95, 1.5)]
NOTES = K.text(
    "Gradient 1:12 and 1.5 m landings follow ADA 405; DIN 18040-1 demands at most 6 %, so this ramp is an ADA-style ramp. Construction: four pours with movement joints, each a 150 mm reinforced slab on two 200 mm side walls over compacted fill, the toe of the first run as slab on grade; the mass lumps walls, slab and retained fill at 25 kN/m3 over the gross volume. Wheel guards 100 x 100 mm run along both runs and both raised landings (ADA 405.9, DIN 18040-1). Each handrail is a tube 42.4 x 2.6 with its top at 0.87 m, on six posts 48.3 x 3.2 at 1.7 m centres, 100 mm clear of every joint and kerb end, on base plates 120 x 70 x 10 with two M10 anchors on the kerb top; the 0.30 m extensions end in a bend down.",
    "Gefälle 1:12 und 1,5-m-Podeste folgen ADA 405; DIN 18040-1 fordert höchstens 6 %, diese Rampe ist daher eine ADA-Rampe. Aufbau: vier Betonierabschnitte mit Bewegungsfugen, jeder eine bewehrte Platte von 150 mm auf zwei Seitenwänden von 200 mm über verdichteter Hinterfüllung, der Fuß des ersten Laufs als Platte auf dem Boden; die Masse fasst Wände, Platte und Hinterfüllung mit 25 kN/m3 über das Bruttovolumen zusammen. Radabweiser 100 x 100 mm laufen an beiden Läufen und beiden erhöhten Podesten entlang (ADA 405.9, DIN 18040-1). Jeder Handlauf ist ein Rohr 42,4 x 2,6 mit Oberkante auf 0,87 m, auf sechs Pfosten 48,3 x 3,2 im Abstand von 1,7 m, 100 mm frei von jeder Fuge und jedem Bordende, auf Fußplatten 120 x 70 x 10 mit zwei Ankern M10 auf der Bordoberseite; die Verlängerungen von 0,30 m enden in einem Bogen nach unten.",
)


def arc(cx, cy, radius, a0, a1, step=15):
    """🌙 Points of an arc, ends included."""
    n = max(1, round(abs(a1 - a0) / step))
    return [[r5(cx + radius * math.cos(math.radians(a0 + (a1 - a0) * i / n))), r5(cy + radius * math.sin(math.radians(a0 + (a1 - a0) * i / n)))] for i in range(n + 1)]


def rail_outline():
    """🖊️ Side view of the handrail tube with both bends, handrail frame."""
    r, b = K.RAIL_R, K.BEND
    c0 = (r + b, K.RAIL_H - b)
    c1 = (9.3 - r - b, 1.45 - b)
    top = [[x, r5(y + r)] for x, y in K.RAIL_KINKS]
    low = [[x, r5(y - r)] for x, y in reversed(K.RAIL_KINKS)]
    return (
        [[0, 0.75], [0, c0[1]]] + arc(c0[0], c0[1], b + r, 180, 90)[1:] + top + arc(c1[0], c1[1], b + r, 90, 0) + [[9.3, 1.35], [r5(9.3 - 2 * r), 1.35]]
        + arc(c1[0], c1[1], b - r, 0, 90) + low + arc(c0[0], c0[1], b - r, 90, 180) + [[r5(2 * r), 0.75]]
    )


def rail_axis():
    """➰ Centre line of the handrail tube, handrail frame."""
    r, b = K.RAIL_R, K.BEND
    return [[r, 0.75]] + arc(r + b, K.RAIL_H - b, b, 180, 90) + K.RAIL_KINKS + arc(9.3 - r - b, 1.45 - b, b, 90, 0) + [[r5(9.3 - r), 1.35]]


def feet(lift):
    """🦶 Handrail-frame foot of every post, with its world position."""
    return [(k, x, x - 1.2, K.h(x) + lift) for k, x in enumerate(K.POSTS)]


def slanted(cx, cy, along0, along1, up0, up1):
    """📐 A rectangle lying on the 1:12 slope: along the slope and normal to it from a point."""
    pts = []
    for a, n in ((along0, up0), (along1, up0), (along1, up1), (along0, up1)):
        pts.append([r5(cx + a * K.C - n * K.S), r5(cy + a * K.S + n * K.C)])
    return pts


def detail(sid, z, tone, geometry, role="detail", filled=True, pen=None):
    """🧷 A steel detail shape of the handrail bone."""
    out = {"id": sid, "bone": "handrail", "role": role, "z": z, "filled": filled, "tone": tone, "region": "handrail-steel", "material": "steel"}
    if pen:
        out["penMm"] = pen
    out.update(geometry)
    return out


def regeo(shape, geometry):
    """♻️ The same shape with a new geometry."""
    keep = {k: v for k, v in shape.items() if k in ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")}
    keep.update(geometry)
    return keep


def poly(points, closed=True):
    """🔷 Poly geometry."""
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in points], "closed": closed}


def rect(x, y, w, hgt):
    """▭ Rect geometry."""
    return {"kind": "rect", "x": r5(x), "y": r5(y), "w": r5(w), "h": r5(hgt)}


def disc(cx, cy, radius):
    """⚫ Circle geometry."""
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": radius, "ry": radius}


def elevation50(shapes):
    """📜 1:50 side elevation."""
    lift = K.KERB / K.C
    out = []
    for s in shapes:
        sid = s["id"]
        if sid.startswith("rail-return") or sid.startswith("post-") or sid.startswith("plate-"):
            continue
        if sid == "kerb":
            s = regeo(s, poly(KERB_SIDE + [[r5(1.5 - K.KERB * K.S), r5(K.KERB * K.C)]]))
        elif sid == "handrail":
            s = regeo(s, poly(rail_outline()))
        elif sid == "rail-axis":
            s = regeo(s, poly(rail_axis(), False))
        out.append(s)
        if sid == "handrail":
            for k, _, x, y in feet(lift):
                out.append(detail("post-%d" % k, 3, 0.35, rect(x - R, y, 2 * R, K.POST_TOP)))
                out.append(detail("stem-%d" % k, 3, 0.35, rect(x - K.STEM_R, y + K.POST_TOP, 2 * K.STEM_R, 0.062)))
                out.append(detail("plate-%d" % k, 4, 0.3, poly(slanted(x, y, -0.06, 0.06, 0, 0.01))))
                for name, a in (("low", -0.04), ("high", 0.04)):
                    out.append(detail("nut-%s-%d" % (name, k), 4.5, 0.25, poly(slanted(x, y, a - 0.0085, a + 0.0085, 0.01, 0.01 + K.NUT_H))))
    return out


def front50(shapes):
    """🚪 1:50 front elevation, seen from the top landing."""
    lift = K.KERB / K.C
    out = []
    for s in shapes:
        sid = s["id"]
        if sid.startswith("post-") or sid.startswith("plate-"):
            continue
        if sid in ("kerb-a", "kerb-b"):
            a, b = (0.6, 0.7) if sid == "kerb-a" else (-0.7, -0.6)
            s = regeo(s, poly([[a, 0.6], [b, 0.6], [b, 0.69], [b - 0.01, 0.7], [a + 0.01, 0.7], [a, 0.69]]))
            s["z"] = 10.5
        elif sid in ("handrail-a", "handrail-b"):
            sign = 1 if sid.endswith("a") else -1
            s = regeo(s, rect(sign * K.RAIL_Z - K.RAIL_R, 0.75, 2 * K.RAIL_R, 1.4712 - 0.75))
        elif sid in ("rail-axis-a", "rail-axis-b"):
            sign = 1 if sid.endswith("a") else -1
            s = regeo(s, poly([[sign * K.RAIL_Z, 0.75], [sign * K.RAIL_Z, 1.45]], False))
        out.append(s)
        if sid in ("handrail-a", "handrail-b"):
            side = sid[-1]
            mid = K.RAIL_Z if side == "a" else -K.RAIL_Z
            for k, _, _, y in feet(lift):
                out.append(detail("post-%s-%d" % (side, k), 3 + k / 100, 0.35, rect(mid - R, y, 2 * R, K.POST_TOP)))
                out.append(detail("stem-%s-%d" % (side, k), 3 + k / 100, 0.35, rect(mid - K.STEM_R, y + K.POST_TOP, 2 * K.STEM_R, 0.062)))
                out.append(detail("plate-%s-%d" % (side, k), 4 + k / 100, 0.35, rect(mid - 0.035, y - 0.06 * K.S, 0.07, 0.12 * K.S + 0.01)))
    return out


def plan(shapes, scale):
    """🗺️ Plan of a scale: kerbs to the end of the top landing, round posts at their real places, rails between their true ends."""
    out = []
    for s in shapes:
        sid = s["id"]
        if sid.startswith("post-") or sid.startswith("plate-"):
            continue
        if sid in ("kerb-a", "kerb-b"):
            s = dict(s, w=10.2)
        elif sid in ("handrail-a", "handrail-b") and s["kind"] == "capsule":
            s = dict(s, x0=r5(1.2 + K.RAIL_R), x1=r5(10.5 - K.RAIL_R))
        out.append(s)
        if sid in ("handrail-a", "handrail-b"):
            side = sid[-1]
            mid = K.RAIL_Z if side == "a" else -K.RAIL_Z
            for k, x in enumerate(K.POSTS):
                if scale == 50:
                    out.append(detail("plate-%s-%d" % (side, k), 3.5, 0.35, rect(x - 0.06, mid - 0.035, 0.12, 0.07)))
                    for name, a in (("low", -0.04), ("high", 0.04)):
                        out.append(detail("nut-%s-%s-%d" % (name, side, k), 3.8, 0.25, disc(x + a, mid, 0.0085)))
                out.append(detail("post-%s-%d" % (side, k), 4 if scale < 200 else 3, 0.35, disc(x, mid, R)))
    return out


def elevation_coarse(shapes, scale):
    """📏 Side elevation of 1:100, 1:200 and 1:500."""
    out = []
    for s in shapes:
        sid = s["id"]
        if sid.startswith("post-"):
            continue
        if sid == "kerb":
            s = regeo(s, poly(KERB_SIDE + [[1.5, 0.1]]))
        out.append(s)
        if sid == "handrail":
            for k, _, x, y in feet(K.KERB if scale < 500 else 0):
                if scale == 100:
                    out.append(detail("post-%d" % k, 3, 0.3, rect(x - R, y, 2 * R, 0.729)))
                elif scale == 200:
                    out.append(detail("post-%d" % k, 3, 0.3, poly([[x, y], [x, y + 0.75]], False), filled=False, pen=0.18))
                else:
                    out.append(detail("post-%d" % k, 2, 0.3, poly([[x, y], [x, y + 0.85]], False), role="body", filled=False, pen=0.13))
    return out


def front100(shapes):
    """🚪 1:100 front elevation."""
    out = []
    for s in shapes:
        if s["id"] in ("posts-a", "posts-b"):
            mid = K.RAIL_Z if s["id"].endswith("a") else -K.RAIL_Z
            s = regeo(s, rect(mid - R, 0.7, 2 * R, 1.4288 - 0.7))
        out.append(s)
    return out


def apply(asset):
    """✍️ Writes every root fix into the asset."""
    asset["provenance"]["notes"] = NOTES
    body_kg, body_x, body_y = K.body_mass()
    rail_kg, rail_x, rail_y, _ = K.handrail_mass()
    body, rail = asset["rig"]["bones"][0], asset["rig"]["bones"][1]
    body["mass"].update({"kg": round(body_kg, 2), "comX": r5(body_x), "comY": r5(body_y)})
    body["section"]["areaM2"] = 0.21
    rail["mass"].update({"kg": round(rail_kg, 2), "comX": r5(rail_x), "comY": round(rail_y, 3)})
    asset["dimensions"]["massKg"] = round(round(body_kg, 2) + round(rail_kg, 2), 2)
    for anchor, (x, length) in zip(asset["anchors"][:4], SUPPORTS):
        anchor.update({"x": x, "footprintLengthM": length})
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    reps = asset["representations"]
    if any(s["id"] == "stem-0" for s in reps["50"]["geometry"]["elevation"]):
        return
    g = reps["50"]["geometry"]
    g["elevation"], g["front"], g["plan"] = elevation50(g["elevation"]), front50(g["front"]), plan(g["plan"], 50)
    g = reps["100"]["geometry"]
    g["elevation"], g["front"], g["plan"] = elevation_coarse(g["elevation"], 100), front100(g["front"]), plan(g["plan"], 100)
    g = reps["200"]["geometry"]
    g["elevation"], g["plan"] = elevation_coarse(g["elevation"], 200), plan(g["plan"], 200)
    g = reps["500"]["geometry"]
    g["elevation"] = elevation_coarse(g["elevation"], 500)


def spans(text, index):
    """🌳 Parses the JSON value at index into (start, end, node): node is a dict or list of child spans, or None for a scalar."""
    while text[index] in " \n\t\r":
        index += 1
    start = index
    if text[index] == "{":
        node, index = {}, index + 1
        while True:
            while text[index] in " \n\t\r,":
                index += 1
            if text[index] == "}":
                return start, index + 1, node
            key, end = json.JSONDecoder().raw_decode(text, index)
            index = text.index(":", end) + 1
            child = spans(text, index)
            node[key], index = child, child[1]
    if text[index] == "[":
        node, index = [], index + 1
        while True:
            while text[index] in " \n\t\r,":
                index += 1
            if text[index] == "]":
                return start, index + 1, node
            child = spans(text, index)
            node.append(child)
            index = child[1]
    _, end = json.JSONDecoder().raw_decode(text, index)
    return start, end, None


def edits(text, span, old, new, out):
    """✂️ Collects the smallest replacements that turn old into new, each written in the plain layout at its own indentation."""
    start, end, node = span
    if old == new:
        return
    if isinstance(old, dict) and isinstance(new, dict) and list(old) == list(new):
        for key in old:
            edits(text, node[key], old[key], new[key], out)
        return
    if isinstance(old, list) and isinstance(new, list) and len(old) == len(new):
        for child, a, b in zip(node, old, new):
            edits(text, child, a, b, out)
        return
    line = text.rfind("\n", 0, start) + 1
    pad = len(text[line:start]) - len(text[line:start].lstrip(" "))
    lines = json.dumps(new, indent=2, ensure_ascii=False).split("\n")
    out.append((start, end, "\n".join(l if k == 0 else " " * pad + l for k, l in enumerate(lines))))


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = json.loads(source)
    apply(document["asset"])
    if "--apply" in sys.argv:
        out = []
        edits(source, spans(source, 0), before, document, out)
        for start, end, piece in sorted(out, reverse=True):
            source = source[:start] + piece + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
        if check != document:
            raise SystemExit("written text does not parse to the intended document")
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog written:", len(out), "replacements in", ", ".join(MEMBERS))
    else:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "components", "wheelchair-ramp.json"), encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-wheelchair-ramp")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written; body kg", round(K.body_mass()[0], 2), "handrail kg", round(K.handrail_mass()[0], 2))
