"""👟️ parent-with-stroller, 1:500: the four shoe shapes of the front view and of the plan were flat outlines of the walking default pose bound to the foot bones, so in every other pose they kept the pitched foot of that pose (front: the right shoe 0.28 m tall and 0.10 m under the ground). They become what the finer scales draw: the side profile of the shoe placed as a solid swept laterally over the width of the shoe, which follows the foot in every pose. Run once: `python3 fix-parent-with-stroller.py`."""
import tv2_doc

SPAN = {"r": (-0.138, -0.054), "l": (0.054, 0.138)}

doc = tv2_doc.Doc(tv2_doc.catalog_file("parent-with-stroller"))
for view in ("front", "plan"):
    for side in ("r", "l"):
        for part in ("shoe", "shoe-toe"):
            path = doc.shape(500, view, f"{part}-{side}")
            old = doc.get(*path)
            profile = doc.get(*doc.shape(500, "elevation", f"{part}-{side}"))
            new = {**old, "points": profile["points"], "solid": {"kind": "extruded", "sweep": "w", "z0": SPAN[side][0], "z1": SPAN[side][1]}}
            doc.set(path, new)
doc.save()
print("[DEBUG] parent-with-stroller: 8 shoe shapes of 1:500 front and plan are solids now")
