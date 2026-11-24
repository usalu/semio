"""🦾️ Repair ops of `excavator` of the second round.

- Volume 1:100 and 1:200: the bodies ended at the stick tail plates (6.770 m ahead of the slew axis) while the 1:50 body ends at the bosses of the stick cylinder rod on those plates (6.799 m), so the lengths disagreed (9.627, 9.596, 9.594 m against the declared 9.63 m, more than a smallest feature at 1:100 and 1:200). Both scales get the two bosses (220 mm discs, far above their smallest feature) and the pin through them as the 1:50 body has them, and the drawings get the two bosses and the pin in the three views, as extrusions over the lateral range of their solids, over the tail plates at 1:100 and under them at 1:200 (the order in which the print profile finds no visible part below its 0.2 mm).
"""
import copy
import oplib

asset = oplib.load("excavator")
ops = []
for scale in ("100", "200"):
    for ident in ("stick-rod-boss-right", "stick-rod-boss-left", "stick-rod-pin"):
        ops.append({"op": "solid-add", "scale": scale, "after": "stick-tail-left", "solid": copy.deepcopy(oplib.solid(asset, 50, ident))})
    for view in ("elevation", "front", "plan"):
        shapes = asset["representations"][scale]["geometry"][view]
        eye = oplib.shape(asset, 50, view, "stick-rod-eye")
        anchor = next((s for s in shapes if s["id"] == ("stick-tail-left" if scale == "100" else "stick-tail-right")), None) or next(s for s in shapes if s["id"] == "stick-body")
        for ident, r, lo, hi in (("stick-rod-boss-right", 0.11, -0.255, -0.185), ("stick-rod-boss-left", 0.11, 0.025, 0.095), ("stick-rod-pin", 0.045, -0.275, 0.115)):
            part = {"id": ident, "bone": "stick", "role": "detail", "z": round(anchor["z"] + (0.25 if scale == "100" else -0.25), 3), "filled": True, "tone": eye["tone"], "kind": "ellipse", "cx": -0.794, "cy": 0.457, "rx": r, "ry": r, "material": eye["material"], "solid": {"kind": "extruded", "sweep": "w", "z0": lo, "z1": hi}}
            ops.append({"op": "shape-add", "scale": scale, "view": view, "after": anchor["id"], "shape": part})
oplib.write("excavator", ops)
