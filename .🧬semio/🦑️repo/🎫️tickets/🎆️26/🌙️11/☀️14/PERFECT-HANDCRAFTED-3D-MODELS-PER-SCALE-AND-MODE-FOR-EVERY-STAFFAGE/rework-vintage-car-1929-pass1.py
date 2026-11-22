"""🚗 Second-round pass 1 on vehicles/vintage-car-1929: leaf-spring travel in the rig, wing crowns at 7.5 degree stations (1:50), sidemount spare and headlamps at 1:500.

Usage: python3 rework-vintage-car-1929-pass1.py            writes the candidate document under 🗑️generated/r3-vintage-car-1929/
       python3 rework-vintage-car-1929-pass1.py --apply    writes the volumes source and the catalog root members
"""
import copy
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/vintage-car-1929/🔣️.json"
SOURCE = os.path.join(T, "volumes/vehicles/vintage-car-1929.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-vintage-car-1929/🔣️.json")

SPRING_TRAVEL_M = 0.08
DIVE_DEG = -0.125
DIVE_FRONT_M = 0.0035
DIVE_REAR_M = -0.0035
WHEEL_RADIUS_M = 0.47
ARCH_CENTRES = ((1.7, 0.37), (-1.5, 0.37))


def by_id(items, ident):
    """🔎 The entry of a list with this id."""
    return next(item for item in items if item["id"] == ident)


def put(items, entry, after=None):
    """📌 Replace the entry with the same id or insert it after the named one."""
    for index, item in enumerate(items):
        if item["id"] == entry["id"]:
            items[index] = entry
            return
    if after is None:
        items.append(entry)
        return
    items.insert(next(i for i, item in enumerate(items) if item["id"] == after) + 1, entry)


def halfway(a, b):
    """🌗 The section halfway between two loft stations, on the wheel arch circle where both lie on it."""
    qa, qb = a["rotation"], b["rotation"]
    sign = 1 if sum(x * y for x, y in zip(qa, qb)) >= 0 else -1
    q = [x + sign * y for x, y in zip(qa, qb)]
    norm = math.sqrt(sum(x * x for x in q))
    at = [(x + y) / 2 for x, y in zip(a["at"], b["at"])]
    for cx, cy in ARCH_CENTRES:
        ra = math.hypot(a["at"][0] - cx, a["at"][1] - cy)
        rb = math.hypot(b["at"][0] - cx, b["at"][1] - cy)
        if abs(ra - WHEEL_RADIUS_M) < 0.004 and abs(rb - WHEEL_RADIUS_M) < 0.004:
            reach = math.hypot(at[0] - cx, at[1] - cy)
            at[0] = cx + (at[0] - cx) * (ra + rb) / 2 / reach
            at[1] = cy + (at[1] - cy) * (ra + rb) / 2 / reach
    return {
        "at": [round(x, 6) for x in at],
        "rx": round((a["rx"] + b["rx"]) / 2, 6),
        "ry": round((a["ry"] + b["ry"]) / 2, 6),
        "exponent": a["exponent"],
        "rotation": [round(x / norm, 6) for x in q],
    }


def refine_wings(solids):
    """🪽 Halve the station spacing of every wing loft (15 to 7.5 degrees of arch, sagitta 4 mm to 1 mm)."""
    names = []
    for solid in solids:
        if solid.get("kind") == "loft" and "fender" in solid["id"] and 12 <= len(solid["sections"]) <= 16:
            sections = solid["sections"]
            refined = []
            for a, b in zip(sections, sections[1:]):
                refined += [a, halfway(a, b)]
            solid["sections"] = refined + [sections[-1]]
            names.append(solid["id"])
    return names


def edit(asset):
    """✍️ All hand-stated changes of this pass on one asset document."""
    for ident in ("axle-f", "axle-r"):
        by_id(asset["rig"]["bones"], ident)["joint"] = {"type": "prismatic", "axisDeg": 90, "minM": -SPRING_TRAVEL_M, "maxM": SPRING_TRAVEL_M}
    braking = by_id(asset["poses"], "braking")
    braking["bones"] = {"axle-f": {"dy": DIVE_FRONT_M}, "axle-r": {"dy": DIVE_REAR_M}}
    braking["root"] = {"x": 0, "y": 0, "angleDeg": DIVE_DEG}
    asset["structure"]["assumptions"][0] = {
        "en": "Frame, body, fenders and spare wheel are one rigid point mass; four semi-elliptic leaf springs hand it to the two rigid axles and these to four tyre patches (static front share about 44 %). The rig lets each axle travel 80 mm up and down on its springs; the springs themselves are drawn at their static camber.",
        "de": "Rahmen, Aufbau, Kotflügel und Reserverad sind eine starre Punktmasse; vier Halbelliptik-Blattfedern geben sie an die zwei Starrachsen und diese an vier Reifenaufstandsflächen ab (statischer Vorderachsanteil etwa 44 %). Das Skelett lässt jede Achse 80 mm ein- und ausfedern; die Federn selbst sind in ihrer statischen Sprengung gezeichnet.",
    }

    volumes = asset["volumes"]
    fine = volumes["50"]
    wings = [name for name in (refine_wings(fine["body"]) + refine_wings(fine["thermal"]) + refine_wings(fine["structure"]))]
    print("[DEBUG] wings refined:", wings)
    fine["decisions"].append({
        "en": "The four wings are lofted through a crowned section every 7.5 degrees of the wheel arch, so the crown is a curve at this scale (1 mm sagitta) and not a row of flats. The axles ride on the leaf springs with 80 mm of travel each way in the rig.",
        "de": "Die vier Kotflügel sind alle 7,5 Grad des Radbogens durch einen bombierten Querschnitt geführt, sodass die Wölbung in diesem Maßstab eine Kurve ist (1 mm Stich) und keine Reihe von Flächen. Die Achsen federn im Skelett je 80 mm an den Blattfedern ein und aus.",
    })

    coarse = volumes["500"]
    put(coarse["body"], {
        "id": "spare-tyre", "kind": "revolve", "bone": "chassis", "material": "rubber", "role": "body", "tone": 0.08,
        "profile": [[-0.05, 0.245], [-0.065, 0.3], [-0.04, 0.37], [0.04, 0.37], [0.065, 0.3], [0.05, 0.245], [-0.05, 0.245]],
        "axis": [0, 0, 1], "origin": [0.92, 0.82, -0.615],
    }, after="wing-line-r")
    put(coarse["body"], {
        "id": "spare-cover", "kind": "revolve", "bone": "chassis", "material": "paint", "role": "body", "tone": 0.62,
        "profile": [[-0.0375, 0], [-0.0375, 0.256], [0.0375, 0.256], [0.0375, 0]],
        "axis": [0, 0, -1], "origin": [0.92, 0.82, -0.615],
    }, after="spare-tyre")
    for side, z in (("l", 0.43), ("r", -0.43)):
        put(coarse["body"], {
            "id": "headlamp-" + side, "kind": "revolve", "bone": "chassis", "material": "nickel-plate", "role": "body", "tone": 0.74,
            "profile": [[0, 0], [0.04, 0.09], [0.165, 0.118], [0.184, 0]],
            "axis": [1, 0, 0], "origin": [1.775, 0.98, z],
        }, after="spare-cover")
    put(coarse["body"], {
        "id": "lamp-bar", "kind": "capsule", "bone": "chassis", "material": "nickel-plate", "role": "detail", "tone": 0.74, "slender": True,
        "p0": [1.83, 0.862, -0.6], "p1": [1.83, 0.862, 0.6], "r0": 0.014, "r1": 0.014,
    }, after="headlamp-l")
    put(coarse["thermal"], {"id": "th-spare-tyre", "of": "spare-tyre", "region": "tyres"}, after="th-wing-line-r")
    coarse["decisions"].append({
        "en": "The sidemount spare stands in the right front wing as a tyre with a 75 mm cover disc, and the two 0.24 m headlamps sit on their tie bar between the wings: both are several features large and mark the car from the side and the front, so they stay at 1:500.",
        "de": "Das seitliche Reserverad steht als Reifen mit 75 mm starker Abdeckscheibe im rechten Vorderkotflügel, und die zwei 0,24 m großen Scheinwerfer sitzen auf ihrer Querstange zwischen den Kotflügeln: beide sind mehrere Merkmalsgrößen groß und kennzeichnen den Wagen von der Seite und von vorn, also bleiben sie im Maßstab 1:500.",
    })

    rail = by_id(asset["representations"]["200"]["geometry"]["elevation"], "frame")
    rail["d"] = "M 2.08,0.53 L 1.7,0.52 L 0.5,0.52 L -0.8,0.52 L -1.5,0.63 L -1.97,0.59 L -1.97,0.515 L -1.5,0.54 L -0.8,0.41 L 0.5,0.41 L 1.7,0.43 L 2.08,0.455 Z"

    drawing = asset["representations"]["500"]
    geometry = drawing["geometry"]
    put(geometry["elevation"], {"id": "headlamp-bowl", "bone": "chassis", "role": "body", "z": 3.2, "filled": True, "tone": 0.74, "region": "fenders", "material": "nickel-plate", "kind": "path", "d": "M 1.775,0.98 L 1.815,1.07 L 1.94,1.098 L 1.959,0.98 L 1.94,0.862 L 1.815,0.89 Z"}, after="wing-line")
    put(geometry["elevation"], {"id": "spare-tyre", "bone": "chassis", "role": "body", "z": 3.6, "filled": True, "tone": 0.08, "region": "tyres", "material": "rubber", "kind": "ellipse", "cx": 0.92, "cy": 0.82, "rx": 0.37, "ry": 0.37}, after="headlamp-bowl")
    put(geometry["elevation"], {"id": "spare-cover", "bone": "chassis", "role": "body", "z": 3.7, "filled": True, "tone": 0.62, "region": "body-panels", "material": "paint", "kind": "ellipse", "cx": 0.92, "cy": 0.82, "rx": 0.256, "ry": 0.256}, after="spare-tyre")
    for side, z in (("l", 0.43), ("r", -0.43)):
        put(geometry["front"], {"id": "headlamp-bowl-" + side, "bone": "chassis", "role": "body", "z": 3.2, "filled": True, "tone": 0.74, "region": "fenders", "material": "nickel-plate", "kind": "ellipse", "cx": z, "cy": 0.98, "rx": 0.118, "ry": 0.118})
        low, high = sorted((round(z - 0.118, 3), round(z + 0.118, 3)))
        put(geometry["plan"], {"id": "headlamp-bowl-" + side, "bone": "chassis", "role": "body", "z": 3.2, "filled": True, "tone": 0.74, "region": "fenders", "material": "nickel-plate", "kind": "path", "d": "M 1.775,%s L 1.815,%s L 1.94,%s L 1.959,%s L 1.94,%s L 1.815,%s Z" % (z, round(z + 0.09, 3), high, z, low, round(z - 0.09, 3))})
    decisions = drawing["decisions"]
    decisions["retained"] = {
        "en": "The long bonnet, upright cabin, the wing line, the rear wheel arch, the sidemount spare wheel as a tyre ring with its cover disc in the side elevation, the two headlamps, the two axle beams, one bar per bumper and the two wheels with a single bar for rotation.",
        "de": "Die lange Haube, die aufrechte Kabine, die Kotflügellinie, der hintere Radausschnitt, das seitliche Reserverad als Reifenring mit Abdeckscheibe in der Seitenansicht, die zwei Scheinwerfer, die zwei Achsbalken, ein Balken je Stoßstange und die zwei Räder mit einem einzigen Balken für die Drehung.",
    }
    decisions["omitted"] = {
        "en": "Lamp bar and stalks, fender lamps, louvres, door gaps, windows beyond the band (below 0.1 m = 0.2 mm); the spare wheel in front view and plan, where it stands 0.06 m proud of the cabin side over the wing and lies inside the outline.",
        "de": "Lampenstange und Lampenstützen, Kotflügellampen, Luftschlitze, Türfugen, Fenster jenseits des Bandes (unter 0,1 m = 0,2 mm); das Reserverad in Vorderansicht und Draufsicht, wo es 0,06 m über die Kabinenseite hinaus über dem Kotflügel steht und innerhalb der Kontur liegt.",
    }
    return asset


def write(path, value):
    """💾 One JSON document in the catalog's layout."""
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, ensure_ascii=False, indent=1) + "\n")


def splice(document):
    """🧵 Replace only the changed root members in the catalog text, each in the layout it already has; the volumes go in through volumes-insert.py."""
    tools = {}
    exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("\ndef main")[0], tools)
    text = open(CATALOG, encoding="utf-8").read()
    current = json.loads(text)["asset"]
    for member in ("rig", "poses", "structure", "representations"):
        if current[member] == document["asset"][member]:
            continue
        start, end = tools["member_range"](text, "  ", member)
        value = document["asset"][member]
        body = json.dumps(value, ensure_ascii=False, indent=1).replace("\n", "\n  ") if member == "rig" else tools["dump"](value, 1)
        text = text[:start] + "  " + json.dumps(member) + ": " + body + text[end:]
        print("[DEBUG] spliced", member)
    check = json.loads(text)["asset"]
    assert all(check[k] == document["asset"][k] for k in check if k != "volumes") and list(check) == list(current)
    with open(CATALOG, "w", encoding="utf-8") as handle:
        handle.write(text)


def main():
    """🏁 Candidate by default, source and catalog with --apply."""
    document = json.load(open(CATALOG, encoding="utf-8"))
    source = json.load(open(SOURCE, encoding="utf-8"))
    document["asset"]["volumes"] = copy.deepcopy(source["volumes"])
    edit(document["asset"])
    if "--apply" in sys.argv:
        write(SOURCE, {"volumes": document["asset"]["volumes"]})
        splice(document)
        print("[DEBUG] applied to source and catalog")
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        write(CANDIDATE, document)
        print("[DEBUG] candidate", CANDIDATE)


main()
