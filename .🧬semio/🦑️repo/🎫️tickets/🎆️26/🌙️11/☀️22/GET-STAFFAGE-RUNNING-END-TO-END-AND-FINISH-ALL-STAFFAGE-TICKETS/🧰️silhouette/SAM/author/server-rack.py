"""🗄️ Repair ops of `server-rack`.
- Depth: the front door handle and the hinge knuckles stand 15 to 23 mm in front of the 1.000 m cabinet (body 1.015, 1.017, 1.024 m at 1:50, 1:100, 1:200; 1.000 m at 1:500), so 1.00 m missed the body of 1:100 by 16.5 mm. Declared 1.015 m.
- Door frame at 1:50 and 1:100: the folded return of the perforated front door and of the two rear doors is a frame 1.5 mm wide and 25 mm deep in the volume; the drawn `door-frame`, `rear-door-a-frame` and `rear-door-b-frame` had its hole as large as its outline (no area), so plan and front showed no door depth. In the side elevation the outline is the outer edge of the return and the hole its inner edge, as the solid `door-return`; in plan and front, where the door is seen edge-on, the swept outline alone is drawn (the top and the side of the return).
- Front at 1:50 and 1:100: the side panel ended 2 mm under the roof skirt (1.965 against 1.967 m), an open seam across the whole depth; it now meets the roof.
"""
from oplib import write

def ring(y0, y1, inset=0.0):
    """⬛️ One closed ring of a door outline in the plane of the door (x up 0 … 1.867 m, y across)."""
    a, b = (y0 + inset, y1 - inset) if y0 < y1 else (y0 - inset, y1 + inset)
    points = [(inset, a), (inset, b), (1.867 - inset, b), (1.867 - inset, a)]
    return "M " + " L ".join(f"{x:g} {y:g}" for x, y in (points if inset == 0 else points[::-1])) + " Z"


DOORS = {"door-frame": (0.27, -0.27), "rear-door-a-frame": (-0.0035, -0.2725), "rear-door-b-frame": (0.2725, 0.0035)}
ops = [{"op": "dims", "set": {"depthM": 1.015}}]
for scale in ("50", "100"):
    for ident, (y0, y1) in DOORS.items():
        ops.append({"op": "shape-set", "scale": scale, "view": "elevation", "id": ident, "set": {"d": ring(y0, y1) + " " + ring(y0, y1, 0.0015)}})
        for view in ("plan", "front"):
            ops.append({"op": "shape-set", "scale": scale, "view": view, "id": ident, "set": {"d": ring(y0, y1)}})
    ops.append({"op": "shape-set", "scale": scale, "view": "front", "id": "side-panel", "set": {"h": 1.897}})
write("server-rack", ops)
