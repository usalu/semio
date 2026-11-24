"""🛴️ Child on kick scooter, three-views sweep (kick poses, the wheels roll). The front and plan wheels were flat outlines of the wheel seen edge-on, so a rolling wheel turned them like a plate: the front tyre lifted up to 55 mm off the ground and the plan tyre ended up to 59 mm short of the side elevation. A wheel is a disc: in front and plan tyre and hub become the circle of the side elevation extruded over the width the old drawing had (30 mm tyre, 26 mm hub, the 72 mm printable strip of 1:500), which reads the same at every angle of roll. `python3 tv1-child-on-kick-scooter.py [target json]`."""
import sys

import tv1_edit as e

doc = e.Doc(e.catalog_file("child-on-kick-scooter"))

DISCS = {
    50: {"front": {"tyre": (0.0725, -0.015, 0.015), "hub": (0.014, -0.013, 0.013)}, "plan": {"tyre": (0.0725, -0.015, 0.015)}},
    100: {"front": {"tyre": (0.0725, -0.015, 0.015)}, "plan": {"tyre": (0.0725, -0.015, 0.015)}},
    200: {"front": {"tyre": (0.0725, -0.015, 0.015)}, "plan": {"tyre": (0.0725, -0.015, 0.015)}},
    500: {"front": {"wheel": (0.0725, -0.037, 0.035)}},
}

for scale, views in DISCS.items():
    for view, parts in views.items():
        for part, (radius, z0, z1) in parts.items():
            for end in ("rear", "front"):
                path, shape = doc.shape(scale, view, f"{part}-{end}", f"wheel-{end}")
                kept = {k: v for k, v in shape.items() if k not in ("kind", "points", "closed")}
                doc.set(path, {**kept, "kind": "ellipse", "cx": 0, "cy": 0, "rx": radius, "ry": radius, "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}})

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
