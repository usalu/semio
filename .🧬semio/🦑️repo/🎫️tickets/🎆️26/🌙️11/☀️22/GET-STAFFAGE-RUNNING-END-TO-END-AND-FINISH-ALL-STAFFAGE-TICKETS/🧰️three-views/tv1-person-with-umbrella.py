"""☂️ Person with umbrella, three-views sweep (pose umbrella-furled, clip close-umbrella): the front and plan drawings of the right forearm, cuff and hand were flat outlines stretched so that they looked right only with the forearm raised 126 degrees (the default pose); with the arm lowered the plan forearm was 59 cm long and reached 74 mm behind the coat. In front and plan sleeve and cuff become solids revolved about the forearm from the side profile of the same scale, at the lateral position and with the lateral width the old drawings had (as the arm of `person-with-white-cane` is drawn), and the hand, which is no body of revolution, a prism of its side profile over the width the old drawings had. `python3 tv1-person-with-umbrella.py [target json]`."""
import sys

import tv1_edit as e

doc = e.Doc(e.catalog_file("person-with-umbrella"))

SLEEVE = {
    50: {"forearm-r": (-0.0067, 1.07), "shirt-cuff-r": (-0.0175, 0.93)},
    100: {"forearm-r": (-0.0041, 1.01), "shirt-cuff-r": (-0.0176, 0.92)},
    200: {"forearm-r": (-0.0043, 1.01), "shirt-cuff-r": (-0.0176, 0.95)},
    500: {"forearm-r": (-0.0037, 1.02), "shirt-cuff-r": (-0.0172, 0.99)},
}
HAND = {50: (-0.244, -0.163), 100: (-0.244, -0.166), 200: (-0.244, -0.1577), 500: (-0.2625, -0.1598)}

for scale, parts in SLEEVE.items():
    for shape_id, (axis_z, lateral) in parts.items():
        _, side = doc.shape(scale, "elevation", shape_id, "forearm-r")
        for view in ("front", "plan"):
            path, shape = doc.shape(scale, view, shape_id, "forearm-r")
            doc.set(path, {**shape, "points": side["points"], "solid": {"kind": "revolved", "axisBone": "forearm-r", "axis": "u", "axisX": 0, "axisZ": axis_z, "lateralScale": lateral}})

for scale, (z0, z1) in HAND.items():
    _, side = doc.shape(scale, "elevation", "hand-r", "hand-r")
    for view in ("front", "plan"):
        path, shape = doc.shape(scale, view, "hand-r", "hand-r")
        doc.set(path, {**shape, "points": side["points"], "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}})

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
