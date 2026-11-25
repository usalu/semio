"""🪑️ Writes the spec of `office-swivel-chair`: the seat swivels about the column, and at the angles between the quarter turns the three views disagreed (measured 2026-11-25: front and plan 29 to 32 mm at all four bands, side elevation and plan 20.4 mm at 1:200 and 1:500, tolerance 20 mm). Cause: the backrest shell is curved in the plan and the seat cushion has rounded corners in the plan, while their solids in the front (and of the cushion in the side elevation) were one plate or one box over the full width, whose corners reach further than the body when it turns. The spec nests them (`nest` of fix.ts), so every drawing at rest stays as it is:
- backrest (shell, pad): the plan draws the shell as a crescent whose rear edge lies 0.15 m behind the bone at the centre and about 0.06 m at its lateral ends. The front plate is slid forward at its level until its rear edge at nine tenths of its half width is where the plan draws it, and it gets under it its parts between the middle strips and between the centre strips of the side elevation (0.17 and 0.105 m), each slid back to where the plan draws the rear edge at that width and cut short at its foot by what it would show below the plate at the fullest recline of the joint;
- seat cushion: it keeps its outline with the span of its front and rear ends and gets under it the part at full width over the depth it has there. Both come from the plan outline of the band: the two rectangles are the cross whose reach along the two diagonals is the reach of the drawn outline (so the views agree at 0, 45 and 90 degrees of swivel); the narrower of the front and the rear end gives the span.
Every number is read from shapes the spec does not change (the strips of the side elevation, the plan outlines) or is the distance still to slide, which is zero once the repair is in place, so a second run writes a spec that changes nothing."""
import math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import asset, r6, write

a = asset("office-swivel-chair")
bones = {bone["id"]: bone for bone in a["rig"]["bones"]}
chain, lean = "backrest", 0.0
while chain is not None:
    lean += math.radians(bones[chain]["angleDeg"])
    chain = bones[chain]["parent"]
ux, vx = math.cos(lean), -math.sin(lean)


def crossings(points, w):
    """✂️ The first coordinates at which a closed outline crosses the line of second coordinate `w`."""
    found = []
    for (u0, w0), (u1, w1) in zip(points, points[1:] + points[:1]):
        if (w0 > w) != (w1 > w):
            found.append(u0 + (u1 - u0) * (w - w0) / (w1 - w0))
    return found


recline = math.radians(bones["backrest"]["joint"]["maxDeg"])
ops = []
for key, representation in a["representations"].items():
    geometry = representation.get("geometry")
    if geometry is None:
        continue
    elevation = {shape["id"]: shape for shape in geometry["elevation"]}
    plan = {shape["id"]: shape for shape in geometry["plan"]}
    for shape in geometry["front"]:
        sheet = plan.get(shape["id"])
        if shape["bone"] != "backrest" or sheet is None or "zTip" not in sheet.get("solid", {}) or f"{shape['id']}-mid-left" not in elevation:
            continue
        solid = sheet["solid"]
        along = lambda u: ux * u + vx * ((solid["z0"] + solid["z1"]) / 2 + ((solid["zTip"][0] + solid["zTip"][1]) / 2 - (solid["z0"] + solid["z1"]) / 2) * u / bones["backrest"]["lengthM"])
        drawn = lambda w: min(along(u) for u in crossings(sheet["points"], w))
        plate = lambda w: ux * max(crossings(shape["points"], w)) + vx * shape["solid"]["z1"]
        bands = [0.9 * max(abs(p[1]) for p in shape["points"]), elevation[f"{shape['id']}-mid-left"]["solid"]["z1"], elevation[shape["id"]]["solid"]["z1"]]
        level = ux * (-math.cos(lean) / math.sin(lean)) + vx
        cover = (drawn(bands[0]) - plate(bands[0])) / level
        middle = (shape["solid"]["z0"] + shape["solid"]["z1"]) / 2
        pieces = []
        for name, band in (("middle", bands[1]), ("centre", bands[2])):
            slide = (drawn(band) - plate(band)) / level - cover
            pieces.append({"id": f"{shape['id']}-{name}", "clip": [-r6(band), r6(band)], "trimLow": round(abs(slide) / math.cos(lean - math.pi / 2) * math.sin(recline) / math.sin(lean + recline) + 0.002, 4), "slide": round(slide, 4)})
        ops.append({"op": "nest", "scale": key, "view": "front", "id": shape["id"], "axis": 1, "mid": r6(middle) if abs(cover) < 0.0002 else round(middle + cover, 4), "pieces": pieces})
    for cushion in ("seat-cushion", "seat-band"):
        plan = next((shape for shape in geometry["plan"] if shape["id"] == cushion), None)
        if plan is None:
            continue
        points = plan["points"]
        xs, ws = [p[0] for p in points], [p[1] for p in points]
        widest = max(abs(w) for w in ws)
        front, rear = max(p[0] + abs(p[1]) for p in points), max(abs(p[1]) - p[0] for p in points)
        at_end = min(front - max(xs), rear + min(xs))
        full = [-(rear - widest), front - widest]
        profile = elevation[cushion]["points"]
        assert abs(min(p[1] for p in profile) + max(xs)) < 0.003 and abs(max(p[1] for p in profile) + min(xs)) < 0.003, (key, cushion)
        ops.append({"op": "nest", "scale": key, "view": "elevation", "id": cushion, "axis": 1, "range": [r6(-at_end), r6(at_end)], "pieces": [
            {"id": f"{cushion}-middle", "clip": [r6(-max(full)), r6(-min(full))], "range": [r6(min(ws)), r6(max(ws))]},
        ]})
        ops.append({"op": "nest", "scale": key, "view": "front", "id": cushion, "axis": 1, "range": [r6(-max(full)), r6(-min(full))], "pieces": [
            {"id": f"{cushion}-middle", "clip": [r6(-at_end), r6(at_end)], "range": [r6(-max(xs)), r6(-min(xs))]},
        ]})
write("office-swivel-chair", {"ops": ops})
