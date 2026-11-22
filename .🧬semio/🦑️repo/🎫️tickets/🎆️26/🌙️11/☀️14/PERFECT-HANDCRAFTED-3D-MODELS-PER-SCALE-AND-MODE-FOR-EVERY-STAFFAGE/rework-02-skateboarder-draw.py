"""✏️ 2D drawings of `humans/skateboarder` that the reworked form invalidates: at 1:500 the board is one mass of deck, trucks and wheels,
drawn by hand over the solid `board` (elevation in the frame of the board bone, front and plan in rest coordinates with the root at the origin).
`apply(asset)` is called by `rework-02-skateboarder-rig.py`.
"""
TOP = [(-0.406, 0.047), (-0.385, 0.042), (-0.355, 0.031), (-0.300, 0.014), (-0.262, 0.00575), (0.262, 0.00575), (0.300, 0.014), (0.355, 0.032), (0.388, 0.044), (0.411, 0.050)]
UNDER = [(0.411, -0.025), (0.388, -0.031), (0.355, -0.043), (0.300, -0.061), (0.262, -0.06925), (0.240, -0.085), (-0.240, -0.085), (-0.262, -0.06925), (-0.300, -0.061), (-0.355, -0.044), (-0.385, -0.033), (-0.406, -0.028)]
FRONT = [(-0.1015, 0.004), (-0.0975, 0), (0.0975, 0), (0.1015, 0.004), (0.1015, 0.088), (0.094, 0.117), (0.072, 0.129), (0.04, 0.135), (-0.04, 0.135), (-0.072, 0.129), (-0.094, 0.117), (-0.1015, 0.088)]
HALF = [(-0.406, 0.04), (-0.385, 0.072), (-0.355, 0.094), (-0.300, 0.1015), (0.300, 0.1015), (0.355, 0.094), (0.388, 0.072), (0.411, 0.04)]


def apply(asset):
    """🔨️ Redraws the board of 1:500 in the three views."""
    geometry = asset["representations"]["500"]["geometry"]
    outline = {"elevation": TOP + UNDER, "front": FRONT, "plan": [(x, -y) for x, y in HALF] + [(x, y) for x, y in reversed(HALF)]}
    for view, points in outline.items():
        shape = next(s for s in geometry[view] if s["id"] == "deck" and s["bone"] == "board")
        shape["points"] = [list(p) for p in points]
        shape.pop("solid", None)
