"""🛹️ The skateboard of `humans/skateboarder` at 1:100, 1:200 and 1:500, modelled for the smallest feature of each scale (15 / 30 / 75 mm)
instead of the 1:50 parts list: every number is hand-chosen against the 8.0 x 31.9 in street board of the reference table
(deck top 5.75 mm above the board bone, underside of the wheels 85 mm below it, axles at +-205 mm, wheels 54 x 32 mm at +-85.5 mm).
`apply(asset)` is called by `rework-02-skateboarder-rig.py`.
"""
import copy
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib02", os.path.join(T, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
MM = 0.001
FEATURE = {100: 15, 200: 30, 500: 75}
TOP = [(-400, 47, 40), (-385, 42, 72), (-355, 31, 94), (-300, 14, 101.5), (-250, 5.75, 101.5), (0, 5.75, 101.5), (250, 5.75, 101.5), (300, 14, 101.5), (355, 32, 94), (388, 44, 72), (405, 50, 40)]
BOARD_OLD = ("deck-", "grip-", "baseplate-", "hanger-", "bushing-", "kingpin-", "pivot-", "axle-")


def deck(scale, wood):
    """🪵️ Deck as one slab of the scale's thickness under the true top line: flat between the kicks, nose and tail raised and rounded in plan.
    Rows in mm: (station along the board, top of the deck above the board bone, half breadth)."""
    half = FEATURE[scale] / 2
    keys = [(u * MM, (top - half) * MM, 0, half * MM, breadth * MM, 5) for u, top, breadth in TOP]
    return lib.loft("deck", "board", wood["material"], wood["tone"], keys, sub=2, round0=6 * MM, round1=6 * MM)


def board(wood):
    """🛹️ 1:500: deck, trucks and wheels as one mass that stands on the ground between the axles and lifts to the kicks."""
    rows = [(-400, 47, 40, 75), (-385, 42, 72, 75), (-355, 31, 94, 75), (-300, 14, 101.5, 75), (-262, 5.75, 101.5, 75), (-240, 5.75, 101.5, 90.75), (0, 5.75, 101.5, 90.75), (240, 5.75, 101.5, 90.75), (262, 5.75, 101.5, 75), (300, 14, 101.5, 75), (355, 32, 94, 75), (388, 44, 72, 75), (405, 50, 40, 75)]
    keys = [(u * MM, (top - depth / 2) * MM, 0, depth / 2 * MM, breadth * MM, 5) for u, top, breadth, depth in rows]
    return lib.loft("board", "board", wood["material"], wood["tone"], keys, sub=1, round0=6 * MM, round1=6 * MM)


def trucks(scale, metal, hangers):
    """🔩️ 1:100: baseplate, bushing stack on the inclined kingpin, hanger and axle bar per truck; 1:200: one block per truck between the wheels."""
    out = []
    for end, sign in (("f", 1), ("r", -1)):
        if scale == 100:
            out.append(lib.box(f"baseplate-{end}", "board", metal["material"], metal["tone"], [sign * 0.211, -0.01675, 0], [0.068, 0.015, 0.056], 0.004))
            out.append(lib.capsule(f"truck-{end}", "board", metal["material"], metal["tone"], [sign * 0.189, -0.022, 0], [sign * 0.202, -0.064, 0], 0.014, 0.012))
            out.append(hangers[end])
            out.append(lib.capsule(f"axle-{end}", "board", metal["material"], metal["tone"], [sign * 0.205, -0.058, -0.1015], [sign * 0.205, -0.058, 0.1015], 0.0075))
        else:
            out.append(lib.box(f"truck-{end}", "board", metal["material"], metal["tone"], [sign * 0.213, -0.04, 0], [0.064, 0.036, 0.139], 0.006))
    return out


def scale_of(volume, scale):
    body = volume["body"]
    by = {s["id"]: s for s in body}
    wood, metal = by["deck-l"], by["baseplate-f"]
    hangers = {end: copy.deepcopy(by[f"hanger-{end}"]) for end in "fr"}
    gone = {s["id"] for s in body if s["bone"] == "board" and s["id"].startswith(BOARD_OLD)}
    new = [board(wood)] if scale == 500 else [deck(scale, wood)] + trucks(scale, metal, hangers)
    if scale == 500:
        gone |= {s["id"] for s in body if s["bone"] in ("wheel-f", "wheel-r")}
    first = next(i for i, s in enumerate(body) if s["id"] in gone)
    volume["body"] = [s for s in body[:first] if s["id"] not in gone] + new + [s for s in body[first:] if s["id"] not in gone]
    volume["thermal"] = [s for s in volume["thermal"] if s.get("of") not in gone] + [lib.of(f"th-{s['id']}", s["id"], "deck" if s["id"] in ("deck", "board") else "trucks") for s in new]
    if scale == 500:
        metal_region, tyre = by["baseplate-f"], by["wheel-f-l"]
        for end, sign in (("f", 1), ("r", -1)):
            volume["thermal"].append(lib.box(f"th-truck-{end}", "board", metal_region["material"], metal_region["tone"], [sign * 0.213, -0.0475, 0], [0.075, 0.075, 0.075], 0, region="trucks"))
            volume["thermal"].append(lib.box(f"th-wheels-{end}", "board", tyre["material"], tyre["tone"], [sign * 0.205, -0.0475, 0], [0.075, 0.075, 0.203], 0, region="wheels"))
    structure = [s for s in volume["structure"] if s.get("of") not in gone]
    radius = FEATURE[scale] / 2 * MM
    for solid in structure:
        if solid["id"] == "mass-board":
            solid["radii"] = [radius, radius, radius]
        if solid["id"].startswith("joint-wheel-") and scale == 100:
            solid["r0"] = solid["r1"] = 0.0075
    if scale != 100:
        structure = [s for s in structure if not s["id"].startswith(("joint-wheel-", "occipital-condyle-"))]
    volume["structure"] = structure + [{"id": f"st-{s['id']}", "of": s["id"], "member": "board", "part": "member"} for s in new]


def apply(asset):
    """🔨️ Rewrites the board of the three coarse scales of the asset's volumes."""
    for scale in (100, 200, 500):
        scale_of(asset["volumes"][str(scale)], scale)
