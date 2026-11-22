"""🪑 Re-stances the bar stool frame by hand: four legs on the diagonals, a foot square of 370 mm, the footrest ring outside the legs at 290 mm, in the volumes source, the 2D drawings, the symbol, the anchors and the texts.

Usage: python3 rework-bar-stool-frame.py [--catalog]
Without a flag it writes the source `volumes/furniture/bar-stool.json` and the candidate `🗑️generated/r3-bar-stool/🔣️.json`; with `--catalog` it writes the catalog file too (re-read immediately before).
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/furniture/bar-stool.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/bar-stool/🔣️.json"
CANDIDATE = os.path.join(T, "🗑️generated/r3-bar-stool/🔣️.json")

TUBE = 0.0125
FOOT_RADIUS = 0.262
FOOT = round(FOOT_RADIUS / math.sqrt(2), 5)
RING_HEIGHT = 0.29
RING_RADIUS = 0.2365
RING_OUTER = RING_RADIUS + TUBE
RING_INNER = RING_RADIUS - TUBE
OLD_RING_RADIUS = 0.2
LEG = {
    "50": {"top": 0.6705, "topRadius": 0.144, "foot": 0.014},
    "100": {"top": 0.6705, "topRadius": 0.144, "foot": 0.0125},
    "200": {"top": 0.676, "topRadius": 0.146, "foot": 0.0125},
    "500": {"top": 0.676, "topRadius": 0.146, "foot": 0.0125},
}
FEET = {"foot-fl": (-FOOT, FOOT), "foot-fr": (-FOOT, -FOOT), "foot-rl": (FOOT, FOOT), "foot-rr": (FOOT, -FOOT)}
FOOT_TITLES = {
    "foot-fl": {"en": "Front left foot", "de": "Fuß vorn links"},
    "foot-fr": {"en": "Front right foot", "de": "Fuß vorn rechts"},
    "foot-rl": {"en": "Rear left foot", "de": "Fuß hinten links"},
    "foot-rr": {"en": "Rear right foot", "de": "Fuß hinten rechts"},
}
RENAMES = {"foot-xp": "foot-fl", "foot-zn": "foot-fr", "foot-zp": "foot-rl", "foot-xn": "foot-rr"}


def top(scale):
    return round(LEG[scale]["topRadius"] / math.sqrt(2), 5)


def leg_radius_at(scale, height):
    leg = LEG[scale]
    return leg["topRadius"] + (FOOT_RADIUS - leg["topRadius"]) * (leg["top"] - height) / (leg["top"] - leg["foot"])


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def rename(value):
    if isinstance(value, dict):
        return {key: rename(item) for key, item in value.items()}
    if isinstance(value, list):
        return [rename(item) for item in value]
    if isinstance(value, str):
        if value in RENAMES:
            return RENAMES[value]
        if value.startswith("support-") and value[8:] in RENAMES:
            return "support-" + RENAMES[value[8:]]
    return value


def replace_text(value, pairs):
    if isinstance(value, dict):
        return {key: replace_text(item, pairs) for key, item in value.items()}
    if isinstance(value, list):
        return [replace_text(item, pairs) for item in value]
    if isinstance(value, str):
        for old, new in pairs:
            value = value.replace(old, new)
    return value


def volumes(source):
    for scale, lists in source.items():
        t, weld_ring = top(scale), round((leg_radius_at(scale, RING_HEIGHT) + RING_RADIUS) / 2 / math.sqrt(2), 5)
        for name in ("body", "thermal", "structure"):
            for solid in lists[name]:
                identifier = solid["id"]
                if identifier == "leg":
                    solid["p0"] = [LEG[scale]["top"], -t, -t]
                    solid["p1"] = [LEG[scale]["foot"], -FOOT, -FOOT]
                elif identifier in ("glide", "m-foot"):
                    solid["origin"] = [0.0, -FOOT, -FOOT]
                elif identifier == "ring" and solid["origin"][0] != RING_HEIGHT:
                    solid["profile"] = [[a, round(r + RING_RADIUS - OLD_RING_RADIUS, 5)] for a, r in solid["profile"]]
                    solid["origin"] = [RING_HEIGHT, 0.0, 0.0]
                elif identifier == "j-weld-top":
                    solid["transform"]["translation"] = [0.678, -t, -t]
                elif identifier == "j-weld-ring":
                    solid["transform"]["translation"] = [RING_HEIGHT, -weld_ring, -weld_ring]
                elif identifier.startswith("support-foot-"):
                    y, z = FEET[identifier[8:]]
                    solid["origin"] = [0.0, y, z]
        lists["structure"] = [solid for solid in lists["structure"] if solid["id"] not in MASSES] + masses(scale)
    return source


MASSES = ("mass-frame", "mass-pan", "mass-leg", "mass-ring")
MARKER = {
    "50": {"leg": [0.02, 0.012, 0.012], "ring": [0.012, 0.012, 0.02], "pan": ([0.008, 0.02, 0.02], 0.676)},
    "100": {"leg": [0.025, 0.0125, 0.0125], "ring": [0.0125, 0.0125, 0.025], "pan": ([0.0085, 0.025, 0.025], 0.6745)},
    "200": {"leg": [0.035, 0.015, 0.015], "ring": [0.015, 0.015, 0.035], "pan": ([0.016, 0.035, 0.035], 0.682)},
    "500": {"leg": [0.0375, 0.0375, 0.0375], "ring": [0.0375, 0.0375, 0.0375], "pan": None},
}


def masses(scale):
    marker, leg = MARKER[scale], LEG[scale]
    middle = round((leg["top"] + leg["foot"]) / 2, 5)
    offset = round(leg_radius_at(scale, middle) / math.sqrt(2), 5)
    quarter = {"count": 4, "rotationAxis": [1.0, 0.0, 0.0], "rotationRad": 1.570796, "origin": [0.0, 0.0, 0.0]}
    base = {"kind": "ellipsoid", "bone": "frame", "material": "chrome-steel", "role": "detail", "tone": 0.4}
    if scale == "500":
        node = round((leg_radius_at(scale, RING_HEIGHT) + RING_RADIUS) / 2 / math.sqrt(2), 5)
        return [{"id": "mass-frame", **base, "radii": marker["leg"], "transform": {"translation": [RING_HEIGHT, -node, -node]}, "repeat": dict(quarter), "member": "frame", "part": "mass"}]
    result = []
    if marker["pan"]:
        result.append({"id": "mass-pan", **base, "radii": marker["pan"][0], "transform": {"translation": [marker["pan"][1], 0.0, 0.0]}, "member": "frame", "part": "mass"})
    result.append({"id": "mass-leg", **base, "radii": marker["leg"], "transform": {"translation": [middle, -offset, -offset]}, "repeat": dict(quarter), "member": "frame", "part": "mass"})
    result.append({"id": "mass-ring", **base, "radii": marker["ring"], "transform": {"translation": [RING_HEIGHT, -RING_RADIUS, 0.0]}, "repeat": dict(quarter), "member": "frame", "part": "mass"})
    return result


def ferrule(y, front):
    half = 0.015
    side = [[0.0, y - half + 0.0015], [0.0015, y - half], [0.019, y - half], [0.022, y - half + 0.003], [0.022, y + half - 0.003], [0.019, y + half], [0.0015, y + half], [0.0, y + half - 0.0015]]
    return [[round(b, 5), a] for a, b in side] if front else [[a, round(b, 5)] for a, b in side]


def drawings(asset):
    for scale, representation in asset["representations"].items():
        geometry, leg, t = representation["geometry"], LEG[scale], top(scale)
        for view in ("elevation", "front"):
            shapes = [shape for shape in geometry[view] if shape["id"] not in ("leg-m", "ferrule-m")]
            for shape in shapes:
                sign = 1 if shape["id"].endswith("-l") else -1
                if view == "front":
                    sign = 1 if shape["id"].endswith("-l") else -1
                if shape["id"] in ("leg-l", "leg-r"):
                    ends = [[leg["top"], sign * t], [leg["foot"], sign * FOOT]]
                    if view == "front":
                        ends = [[b, a] for a, b in ends]
                    if shape["kind"] == "capsule":
                        shape["x0"], shape["y0"], shape["x1"], shape["y1"] = ends[0][0], ends[0][1], ends[1][0], ends[1][1]
                    else:
                        shape["points"] = ends
                elif shape["id"] in ("ferrule-l", "ferrule-r"):
                    shape["points"] = ferrule(sign * FOOT, view == "front")
                elif shape["id"] == "ring":
                    if view == "elevation":
                        shape["x0"], shape["y0"], shape["x1"], shape["y1"] = RING_HEIGHT, RING_RADIUS, RING_HEIGHT, -RING_RADIUS
                    else:
                        shape["x0"], shape["y0"], shape["x1"], shape["y1"] = -RING_RADIUS, RING_HEIGHT, RING_RADIUS, RING_HEIGHT
            geometry[view] = shapes
        plan, pen = [], {"50": 0.25, "100": 0.25, "200": 0.13}.get(scale)
        annulus = "M {o} 0 A {o} {o} 0 1 0 -{o} 0 A {o} {o} 0 1 0 {o} 0 Z M {i} 0 A {i} {i} 0 1 0 -{i} 0 A {i} {i} 0 1 0 {i} 0 Z".format(o=RING_OUTER, i=RING_INNER)
        corners = {"leg-fl": (1, 1), "leg-fr": (1, -1), "leg-rl": (-1, 1), "leg-rr": (-1, -1)}
        rest = [shape for shape in geometry["plan"] if shape["id"] != "ring" and not shape["id"].startswith(("leg-", "ferrule-"))]
        if pen:
            for identifier, (ahead, left) in corners.items():
                plan.append({"id": identifier, "bone": "frame", "role": "body", "z": 2, "filled": True, "tone": 0.7, "kind": "capsule", "region": "legs", "material": "chrome-steel", "penMm": pen, "x0": ahead * t, "y0": left * t, "x1": ahead * FOOT, "y1": left * FOOT, "r0": TUBE, "r1": TUBE})
            if scale == "50":
                for identifier, (ahead, left) in corners.items():
                    plan.append({"id": identifier.replace("leg", "ferrule"), "bone": "frame", "role": "body", "z": 2.1, "filled": True, "tone": 0.15, "kind": "ellipse", "region": "legs", "material": "rubber", "penMm": 0.25, "cx": ahead * FOOT, "cy": left * FOOT, "rx": 0.015, "ry": 0.015})
            ring = {"id": "ring", "bone": "frame", "role": "body", "z": 2.3, "filled": True, "tone": 0.7, "kind": "path", "region": "footrest-ring", "material": "chrome-steel", "d": annulus}
            if scale != "200":
                ring["penMm"] = 0.25
            plan.append(ring)
        else:
            for identifier, (ahead, left) in corners.items():
                plan.append({"id": identifier, "bone": "frame", "role": "body", "z": 2, "filled": False, "tone": 0.3, "kind": "poly", "region": "legs", "material": "chrome-steel", "points": [[ahead * t, left * t], [ahead * FOOT, left * FOOT]], "closed": False})
            plan.append({"id": "ring", "bone": "frame", "role": "body", "z": 2.3, "filled": True, "tone": 0.3, "kind": "path", "region": "footrest-ring", "material": "chrome-steel", "d": annulus})
        geometry["plan"] = plan + rest
    symbol = asset["symbols"][0]["geometry"]
    for view in ("elevation", "front"):
        for shape in symbol[view]:
            if shape["id"].startswith("leg-"):
                sign = -1 if shape["x0"] < 0 else 1
                shape["x0"], shape["x1"] = sign * 0.102, sign * round(FOOT, 3)
            elif shape["id"] == "foot-ring":
                shape["points"] = [[-RING_OUTER, RING_HEIGHT], [RING_OUTER, RING_HEIGHT]]
    for shape in symbol["plan"]:
        if shape["id"] == "plan-ring":
            shape["rx"] = shape["ry"] = RING_OUTER


TEXTS = [
    ("0.425 x 0.87 m (seat top 0.752 m, footrest 0.27 m, foot circle 0.395 m)", "0.498 x 0.87 m (seat top 0.752 m, footrest ring 0.498 m over all at 0.29 m, four feet on the diagonals at the corners of a 0.37 m square)"),
    ("0,425 x 0,87 m (Sitzoberkante 0,752 m, Fußring 0,27 m, Fußkreis 0,395 m)", "0,498 x 0,87 m (Sitzoberkante 0,752 m, Fußring 0,498 m über alles auf 0,29 m, vier Füße auf den Diagonalen an den Ecken eines Quadrats von 0,37 m)"),
    ("Stool sign 0.85 x 1.74 mm", "Stool sign 1.0 x 1.74 mm"),
    ("Hockerzeichen 0,85 x 1,74 mm", "Hockerzeichen 1,0 x 1,74 mm"),
    ("three legs as body-pen strokes", "the leg pairs of the diagonal frame as two body-pen strokes"),
    ("drei Beine als Körperstriche", "die Beinpaare des Diagonalgestells als zwei Körperstriche"),
    ("and the ring disc, the leg feet hidden under the ring.", "the four legs running out on the diagonals under it and the ring of 0.50 m as a true annulus over them."),
    ("und die Ringscheibe, die Beinenden unter dem Ring verborgen.", "die vier Beine, die auf den Diagonalen darunter herauslaufen, und darüber der Ring von 0,50 m als echter Kreisring."),
    ("Stool 2.1 x 4.4 mm: cushion disc on the mount band, three legs and the ring as true 25 mm tubes", "Stool 2.5 x 4.4 mm: cushion disc on the mount band, the two leg pairs of the diagonal frame and the ring as true 25 mm tubes"),
    ("Hocker 2,1 x 4,4 mm: Polsterscheibe auf dem Aufnahmeband, drei Beine und der Ring als echte 25-mm-Rohre", "Hocker 2,5 x 4,4 mm: Polsterscheibe auf dem Aufnahmeband, die zwei Beinpaare des Diagonalgestells und der Ring als echte 25-mm-Rohre"),
    ("Stool 4.3 x 8.7 mm", "Stool 5.0 x 8.7 mm"),
    ("Hocker 4,3 x 8,7 mm", "Hocker 5,0 x 8,7 mm"),
    ("four tube legs to the floor", "four tube legs on the diagonals to the floor"),
    ("vier Rohrbeine bis zum Boden", "vier Rohrbeine auf den Diagonalen bis zum Boden"),
    ("Stool 8.5 x 17.4 mm", "Stool 10.0 x 17.4 mm"),
    ("Hocker 8,5 x 17,4 mm", "Hocker 10,0 x 17,4 mm"),
    ("four splayed 25 mm tubes in rubber glides, the footrest ring welded outside the legs at 0.27 m", "four 25 mm tubes splayed on the diagonals in rubber glides (each elevation shows a near and a far leg as one), the footrest ring welded outside the legs at 0.29 m"),
    ("vier gespreizte 25-mm-Rohre in Gummigleitern, der außen an die Beine geschweißte Fußring auf 0,27 m", "vier auf den Diagonalen gespreizte 25-mm-Rohre in Gummigleitern (jede Ansicht zeigt ein vorderes und ein hinteres Bein als eines), der außen an die Beine geschweißte Fußring auf 0,29 m"),
    ("Plan: ring, leg feet and glides outside the cushion disc", "Plan: the legs running out under the cushion disc on the diagonals, the ring over them and the glides outside the ring"),
    ("Grundriss: Ring, Beinenden und Gleiter außerhalb der Polsterscheibe", "Grundriss: die Beine laufen auf den Diagonalen unter der Polsterscheibe heraus, darüber der Ring, die Gleiter außerhalb des Rings"),
    ("four splayed legs with a foot ring, leg footprint 0.42 m", "four legs splayed on the diagonals with a foot ring of 0.50 m, feet on a 0.37 m square"),
    ("vier gespreizte Beine mit Fußring, Standfläche 0,42 m", "vier auf den Diagonalen gespreizte Beine mit Fußring von 0,50 m, Füße auf einem Quadrat von 0,37 m"),
    ("leg footprint radius 0.21 m", "footrest ring radius 0.25 m"),
    ("Standflächenradius 0,21 m", "Fußringradius 0,25 m"),
    ("one under the rubber glide of each leg (x or z = +-0.1975 m)", "one under the rubber glide of each leg (on the diagonals, x and z = +-0.185 m, so the overturning line of two adjacent feet lies 0.185 m from the axis, outside the 0.18 m seat radius)"),
    ("je einem unter dem Gummigleiter jedes Beins (x oder z = +-0,1975 m)", "je einem unter dem Gummigleiter jedes Beins (auf den Diagonalen, x und z = +-0,185 m, die Kippkante zweier benachbarter Füße liegt damit 0,185 m von der Achse, außerhalb des Sitzradius von 0,18 m)"),
    ("a footrest ring welded to their outside at 0.27 m", "a footrest ring welded to their outside at 0.29 m"),
    ("ein außen auf 0,27 m angeschweißter Fußring", "ein außen auf 0,29 m angeschweißter Fußring"),
    ("splayed 4.7 degrees from the 0.144 m pan radius to the 0.1975 m foot circle", "standing on the diagonals and splayed 10.2 degrees (7.2 in each elevation) from the 0.144 m pan radius to the 0.262 m foot radius, a 0.37 m foot square"),
    ("um 4,7 Grad gespreizt vom Pfannenradius 0,144 m zum Fußkreis 0,1975 m", "auf den Diagonalen stehend und um 10,2 Grad gespreizt (7,2 in jeder Ansicht) vom Pfannenradius 0,144 m zum Fußradius 0,262 m, ein Fußquadrat von 0,37 m"),
    ("(25 mm tube bent to 0.40 m) welded to the outside of the legs at 0.27 m", "(25 mm tube bent to 0.473 m on its centre line) welded to the outside of the legs at 0.29 m, 462 mm under the seat top"),
    ("(25-mm-Rohr, auf 0,40 m gebogen) außen an die Beine auf 0,27 m geschweißt", "(25-mm-Rohr, auf 0,473 m Achsmaß gebogen) außen an die Beine auf 0,29 m geschweißt, 462 mm unter der Sitzoberkante"),
    ("four true 25 mm legs as slender tubes", "four true 25 mm legs on the diagonals as slender tubes"),
    ("vier echte 25-mm-Beine als schlanke Rohre", "vier echte 25-mm-Beine auf den Diagonalen als schlanke Rohre"),
    ("legs as true 25 mm tubes running into the floor", "legs on the diagonals as true 25 mm tubes running into the floor"),
    ("Beine als echte 25-mm-Rohre bis zum Boden", "Beine auf den Diagonalen als echte 25-mm-Rohre bis zum Boden"),
    ("legs and footrest ring stay true 25 mm tubes", "the four diagonal legs and the footrest ring stay true 25 mm tubes"),
    ("Beine und Fußring bleiben echte 25-mm-Rohre", "die vier Diagonalbeine und der Fußring bleiben echte 25-mm-Rohre"),
]


def root(asset):
    asset["dimensions"]["widthM"] = asset["dimensions"]["depthM"] = round(2 * RING_OUTER, 3)
    for bone in asset["rig"]["bones"]:
        if bone["id"] == "seat":
            bone["joint"] = {"type": "revolute", "continuousTwist": True}
    anchors = []
    for anchor in asset["anchors"]:
        if anchor["id"] in FEET:
            anchor["y"], anchor["z"] = FEET[anchor["id"]]
            anchor["title"] = FOOT_TITLES[anchor["id"]]
        elif anchor["id"] == "ring-front":
            anchor["x"], anchor["y"] = RING_HEIGHT + TUBE, -RING_RADIUS
        anchors.append(anchor)
    order = ["foot-fl", "foot-fr", "foot-rl", "foot-rr"]
    asset["anchors"] = sorted(anchors, key=lambda anchor: order.index(anchor["id"]) if anchor["id"] in order else len(order))
    for region in asset["thermal"]["regions"]:
        if region["id"] == "footrest-ring":
            region["areaM2"] = 0.117
    drawings(asset)


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = rename(json.loads(text))
    asset = document["asset"]
    source = rename(json.load(open(SOURCE, encoding="utf-8")))
    source["volumes"] = replace_text(volumes(source["volumes"]), TEXTS)
    root(asset)
    asset = replace_text(asset, TEXTS)
    asset["volumes"] = source["volumes"]
    document["asset"] = asset
    open(SOURCE, "w", encoding="utf-8").write(dump(source, 0) + "\n")
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w", encoding="utf-8").write(dump(document, 0) + "\n")
    print("source and candidate written; catalog text reproduced by dump:", dump(json.loads(text), 0) + "\n" == text)
    if "--catalog" in sys.argv:
        if open(CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("catalog changed while editing; retry")
        open(CATALOG, "w", encoding="utf-8").write(dump(document, 0) + "\n")
        print("catalog written")


main()
