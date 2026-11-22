"""🪜 Spiral stair roots for the 1:500 rework: the 2D views draw the same eighteen posts as the model (one per tread, two per landing rail) and the texts say so. Without `--apply` writes the candidate document for `--file`."""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("roots18", os.path.join(T, "rework-18-spiral-stair-roots.py"))
R = importlib.util.module_from_spec(spec)
spec.loader.exec_module(R)
OUT = os.path.join(T, "🗑️generated", "r3-spiral-stair", "🔣️.json")
MEMBERS = ("representations", "symbols")
NEAR_Z = {"elevation": 2.5, "front": 6.5}
FAR_Z = 0.5
LANDING_POST_Z = 1.5
PANEL_TONE = 0.72
CORNER = 0.54801
UNDER_Z = 2.1
SWAPS = (
    ("the posts, the handrail and the 52 balusters as 0.15 mm lines (the 75 mm feature; the balusters read as the infill band)",
     "the posts, the handrail and eighteen infill posts as 0.15 mm lines (the 75 mm feature): one per tread at the newel's place on the tread, 0.30 m apart, and two under each landing rail, because the real 52 balusters at 0.10 m centres would close into a black band at 0.2 mm spacing"),
    ("die Pfosten, der Handlauf und die 52 Stäbe als 0,15-mm-Linien (das Merkmal 75 mm; die Stäbe lesen sich als Füllband)",
     "die Pfosten, der Handlauf und achtzehn Füllpfosten als 0,15-mm-Linien (das Merkmal 75 mm): einer je Stufe an der Stelle des Antrittspfostens auf der Stufe, 0,30 m auseinander, und zwei unter jedem Podesthandlauf, weil die wahren 52 Stäbe im Abstand von 0,10 m bei 0,2 mm Abstand zu einem schwarzen Band zuliefen"),
    ("the posts, the handrail and eighteen infill posts as 0.15 mm lines (the 75 mm feature): one per tread at the newel's place on the tread, 0.30 m apart, and two under each landing rail, because the real 52 balusters at 0.10 m centres would close into a black band at 0.2 mm spacing",
     "the posts and the handrail as 0.15 mm lines (the 75 mm feature) and the infill as light guard panels, one per tread under the helical rail and one under each landing rail: the real 52 balusters at 0.10 m centres leave 26 mm between rods of the feature and would close into a black band at 0.2 mm spacing, so they merge into the surface they form, as the treads and the column merge into the stair mass; panels on the far side lie behind the stair mass, panels on the near side in front of it"),
    ("die Pfosten, der Handlauf und achtzehn Füllpfosten als 0,15-mm-Linien (das Merkmal 75 mm): einer je Stufe an der Stelle des Antrittspfostens auf der Stufe, 0,30 m auseinander, und zwei unter jedem Podesthandlauf, weil die wahren 52 Stäbe im Abstand von 0,10 m bei 0,2 mm Abstand zu einem schwarzen Band zuliefen",
     "die Pfosten und der Handlauf als 0,15-mm-Linien (das Merkmal 75 mm) und die Füllung als helle Geländerfelder, eines je Stufe unter dem Wendelhandlauf und eines unter jedem Podesthandlauf: die wahren 52 Stäbe im Abstand von 0,10 m lassen zwischen Stäben im Merkmal 26 mm und liefen bei 0,2 mm Abstand zu einem schwarzen Band zu, sie verschmelzen deshalb zu der Fläche, die sie bilden, wie Stufen und Säule zur Treppenmasse verschmelzen; Felder der abgewandten Seite liegen hinter der Treppenmasse, Felder der zugewandten Seite davor"),
    ("Sleeve joints, anchors, feet and flanges; two of three balusters on every tread and four of six and three of five under the landing rails.", "Sleeve joints, anchors, feet and flanges; the single balusters."),
    ("Hülsenfugen, Anker, Füße und Flansche; zwei von drei Stäben auf jeder Stufe sowie vier von sechs und drei von fünf unter den Podesthandläufen.", "Hülsenfugen, Anker, Füße und Flansche; die einzelnen Stäbe."),
    ("Mid-grey mass, black post, rail and baluster lines.", "Mid-grey mass, light guard panels, black post and rail lines."),
    ("Mittelgraue Masse, schwarze Pfosten-, Handlauf- und Stablinien.", "Mittelgraue Masse, helle Geländerfelder, schwarze Pfosten- und Handlauflinien."),
    ("Posts, rail and balusters are 0.15 mm lines", "Posts and rail are 0.15 mm lines"),
    ("Pfosten, Handlauf und Stäbe sind 0,15-mm-Linien", "Pfosten und Handlauf sind 0,15-mm-Linien"),
    ("the balusters and posts that carry the rail.", "the guard under the rail and the posts that carry it."),
    ("die Stäbe und Pfosten, die den Handlauf tragen.", "das Geländerfeld unter dem Handlauf und die Pfosten, die ihn tragen."),
    ("Total height 3.89 m (rail)", "Total height 3.74 m (column post)"),
    ("Gesamthöhe 3,89 m (Handlauf)", "Gesamthöhe 3,74 m (Säulenpfosten)"),
    ("and the 3.89 m height are true", "and the 3.74 m height are true"),
    ("und die Höhe 3,89 m sind echt", "und die Höhe 3,74 m sind echt"),
    ("Sleeve joints, anchors, feet and flanges.", "Sleeve joints, anchors, feet and flanges; two of three balusters on every tread and four of six and three of five under the landing rails."),
    ("Hülsenfugen, Anker, Füße und Flansche.", "Hülsenfugen, Anker, Füße und Flansche; zwei von drei Stäben auf jeder Stufe sowie vier von sechs und drei von fünf unter den Podesthandläufen."),
)


def swap(value):
    if isinstance(value, str):
        for old, new in SWAPS:
            value = value.replace(old, new)
        return value
    if isinstance(value, dict):
        return {key: swap(item) for key, item in value.items()}
    if isinstance(value, list):
        return [swap(item) for item in value]
    return value


def hull(points):
    points = sorted(set(points))
    def half(seq):
        out = []
        for q in seq:
            while len(out) >= 2 and (out[-1][0] - out[-2][0]) * (q[1] - out[-2][1]) - (out[-1][1] - out[-2][1]) * (q[0] - out[-2][0]) <= 0:
                out.pop()
            out.append(q)
        return out[:-1]
    return [[round(x, 5), round(y, 5)] for x, y in half(points) + half(points[::-1])]


def panel_corners(solid):
    """📐 The eight world corners of one copy-0 panel solid of the 1:500 body (extrude on yz turned about y, or on zx)."""
    lo, hi = solid["from"], solid["to"]
    if solid["plane"] == "zx":
        return [(x, y, z) for z, x in solid["profile"] for y in (lo, hi)]
    half_turn = 2 * math.asin(-solid["transform"]["rotation"][1])
    lift = solid["transform"].get("translation", [0, 0, 0])[1]
    return [turned((x, y + lift, z), half_turn) for y, z in solid["profile"] for x in (lo, hi)]


def turned(point, angle):
    x, y, z = point
    return (x * math.cos(angle) - z * math.sin(angle), y, x * math.sin(angle) + z * math.cos(angle))


def panels(body):
    """🧱 Every guard panel of the 1:500 body as (id, world corners, mid point)."""
    out = []
    by = {solid["id"]: solid for solid in body}
    out.append(("infill-1", panel_corners(by["infill-start"])))
    step = -by["infill"]["repeat"]["rotationRad"]
    rise = by["infill"]["repeat"]["translation"][1]
    for k in range(by["infill"]["repeat"]["count"]):
        out.append(("infill-%d" % (k + 2), [turned((x, y + k * rise, z), k * step) for x, y, z in panel_corners(by["infill"])]))
    out.append(("infill-outer", panel_corners(by["infill-outer"])))
    out.append(("infill-inner", panel_corners(by["infill-inner"])))
    return out


def banded(shapes, view, body):
    kept = [shape for shape in shapes if not shape["id"].startswith("baluster-") and not shape["id"].startswith("infill-")]
    at = next(i for i, shape in enumerate(kept) if shape["id"] == "newel")
    drawn = []
    for name, corners in panels(body):
        mid = [sum(c[i] for c in corners) / len(corners) for i in range(3)]
        near = mid[2] < 0 if view == "elevation" else mid[0] > 0
        points = hull([(c[0], c[1]) if view == "elevation" else (c[2], c[1]) for c in corners])
        drawn.append({"id": name, "bone": "balustrade", "role": "body", "z": LANDING_POST_Z if (name, view) == ("infill-inner", "elevation") else NEAR_Z[view] if near else FAR_Z, "filled": True, "tone": PANEL_TONE, "region": "balustrade-steel", "material": "steel", "kind": "poly", "points": points, "closed": True})
    return kept[:at] + drawn + kept[at:]


def under_landing(plan):
    by = {shape["id"]: shape for shape in plan}
    first, second = by["handrail-1"], by["handrail-2"]
    corner = next(i for i, point in enumerate(second["points"]) if abs(point[0] - CORNER) < 1e-4 and abs(point[1] - CORNER) < 1e-4)
    first["points"] = first["points"] + second["points"][1:corner + 1]
    second["points"] = second["points"][corner:]
    first["z"] = UNDER_Z


def roots(asset):
    for scale in ("50", "100", "200", "500"):
        under_landing(asset["representations"][scale]["geometry"]["plan"])
    geometry = asset["representations"]["500"]["geometry"]
    body = json.load(open(R.SRC))["volumes"]["500"]["body"]
    for view in ("elevation", "front"):
        geometry[view] = banded(geometry[view], view, body)
    asset["representations"]["500"]["decisions"] = swap(asset["representations"]["500"]["decisions"])
    asset["symbols"] = swap(asset["symbols"])


def main():
    with open(R.CAT, encoding="utf-8") as f:
        source = f.read()
    doc, before = json.loads(source), json.loads(source)
    roots(doc["asset"])
    if "--apply" not in sys.argv:
        cand = copy.deepcopy(doc)
        cand["asset"]["volumes"] = json.load(open(R.SRC))["volumes"]
        os.makedirs(os.path.dirname(OUT), exist_ok=True)
        with open(OUT, "w") as f:
            json.dump(cand, f, indent=2, ensure_ascii=False)
            f.write("\n")
        print("[DEBUG] candidate", OUT)
        return
    for key in MEMBERS:
        start, end = R.member(source, key)
        if R.dumped(before["asset"][key]) != source[start:end]:
            raise SystemExit("member %s is not in the plain layout, not written" % key)
        source = source[:start] + R.dumped(doc["asset"][key]) + source[end:]
    check = json.loads(source)
    for key in check["asset"]:
        assert check["asset"][key] == doc["asset"][key], key
    with open(R.CAT, "w", encoding="utf-8") as f:
        f.write(source)
    print("[DEBUG] applied", R.CAT)


main()
