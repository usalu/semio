"""📦️ worker-carrying-box, all four scales: the shoes (rear foot and toe box of both feet) of the plan and of the front view were flat outlines of the standing figure that the foot bone only carries along as a card. In the swing phase of the walk the rear foot is pitched 49 degrees toe-down behind the body (poses `walk-midstance-l/-r`, `walk-toe-off-l/-r`, clip `carry-walk` at 0.22 s and 0.66 s): the side elevation shows the heel back to x −0.465 m, the plan card, foreshortened, only to −0.39 … −0.41 m (tolerance 0.055 m), and the front view showed a pitched shoe 0.12 m high where the side elevation shows 0.26 m.

Each of the shoe shapes of the two views becomes what the stroller parent and the finer humans draw: the side profile of that part of its scale placed as a solid swept laterally over the width of the shoe (the lateral span of the old outline, in rest coordinates of the asset), so plan and front view follow the foot through the whole stride. Run once: `python3 fix-worker-carrying-box.py`."""
import sys

import tv2_doc

VIEWS = tuple(sys.argv[1:]) or ("plan", "front")
BONES = ("foot-l", "foot-r", "toe-l", "toe-r")

doc = tv2_doc.Doc(tv2_doc.catalog_file("worker-carrying-box"))
count = 0
for scale in (50, 100, 200, 500):
    base = ("asset", "representations", str(scale), "geometry")
    side_ids = {shape["id"] for shape in doc.get(*base, "elevation") if shape["bone"] in BONES}
    for view in VIEWS:
        lateral_axis = 1 if view == "plan" else 0
        for shape_id in [shape["id"] for shape in doc.get(*base, view) if shape["bone"] in BONES and shape["role"] == "body" and shape["filled"]]:
            if shape_id not in side_ids:
                raise SystemExit(f"1:{scale} {view} {shape_id}: no side profile of that id")
            path = doc.shape(scale, view, shape_id)
            old = doc.get(*path)
            side = doc.get(*doc.shape(scale, "elevation", shape_id))
            lateral = [p[lateral_axis] for p in old["points"]]
            doc.set(path, {**old, "points": side["points"], "solid": {"kind": "extruded", "sweep": "w", "z0": round(min(lateral), 4), "z1": round(max(lateral), 4)}})
            count += 1
doc.save()
print(f"[DEBUG] worker-carrying-box: {count} shoe shapes of {', '.join(VIEWS)} are swept side profiles now")
