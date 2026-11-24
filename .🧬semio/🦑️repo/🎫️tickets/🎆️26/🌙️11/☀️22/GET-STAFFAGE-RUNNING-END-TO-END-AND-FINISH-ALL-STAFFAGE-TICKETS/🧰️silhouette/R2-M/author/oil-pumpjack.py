"""🛢️ Hand repairs of `oil-pumpjack` of the second round (the traced parts follow in oil-pumpjack.sh): drawn extrusions take the lateral range of their solids.

- 1:500 front and plan: each counterweight was drawn 0.43 to 0.75 m from the axis (over the crank arm); its solid is 0.43 to 0.59 m. The crank shaft of the body, which crosses the 10 mm clearance between gearbox and counterweight, is drawn (it was missing, so the clearance read as one open slit).
- 1:200 front: the crank hub was one cylinder through the gearbox; the body has two hubs (0.48 to 0.78 m each side) and the crank shaft between them, which crosses the 10 mm clearance between gearbox and counterweight.
- 1:50 and 1:100 front: the same two hubs, and the two bearing caps of the gearbox between gearbox and hub (the one cylinder stood 20 mm proud of the caps in the clearance beside the gearbox).
- 1:50 and 1:100 front: the feet of the front legs in the web of the skid beams reach 3 mm into the beam flange (a seam of one raster cell).
- 1:50 side elevation: the two halves of the cap of the polished rod beside the bridle rope become one piece under it (seams of one raster cell).
- 1:50 front: the polished rod runs 5 mm into the horsehead face (it ended 0.9 mm under it: a seam the body does not have).
- 1:500 front: the motor shaft reaches the motor (it ended 60 mm before it).
"""
import copy
import oplib

asset = oplib.load("oil-pumpjack")
ops = []
for view in ("front", "plan"):
    for side, sign in (("l", 1), ("r", -1)):
        lo, hi = sorted((sign * 0.43, sign * 0.59))
        ops.append({"op": "shape-set", "scale": "500", "view": view, "id": f"counterweight-{side}", "set": {"solid": {"kind": "extruded", "sweep": "w", "z0": lo, "z1": hi}}})
hub = copy.deepcopy(oplib.shape(asset, 200, "front", "crank-hub"))
hub["solid"] = {"kind": "extruded", "sweep": "w", "z0": 0.48, "z1": 0.78}
ops.append({"op": "shape", "scale": "200", "view": "front", "id": "crank-hub", "shape": hub})
other = copy.deepcopy(hub)
other["id"] = "crank-hub-r"
other["solid"] = {"kind": "extruded", "sweep": "w", "z0": -0.78, "z1": -0.48}
ops.append({"op": "shape-add", "scale": "200", "view": "front", "after": "crank-hub", "shape": other})
shaft = copy.deepcopy(hub)
shaft.update({"id": "crank-shaft", "rx": 0.08, "ry": 0.08, "z": 5.6, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.76, "z1": 0.76}})
ops.append({"op": "shape-add", "scale": "200", "view": "front", "after": "crank-hub-r", "shape": shaft})
for scale in ("50", "100"):
    hub = copy.deepcopy(oplib.shape(asset, scale, "front", "crank-hub"))
    hub["solid"] = {"kind": "extruded", "sweep": "w", "z0": 0.48, "z1": 0.78}
    ops.append({"op": "shape", "scale": scale, "view": "front", "id": "crank-hub", "shape": hub})
    other = copy.deepcopy(hub)
    other["id"] = "crank-hub-r"
    other["solid"] = {"kind": "extruded", "sweep": "w", "z0": -0.78, "z1": -0.48}
    ops.append({"op": "shape-add", "scale": scale, "view": "front", "after": "crank-hub", "shape": other})
    for side, lo, hi in (("l", 0.42, 0.48), ("r", -0.48, -0.42)):
        cap = {"id": f"bearing-cap-{side}", "bone": "gearbox", "role": "body", "z": 5.6, "filled": True, "tone": 0.55, "kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.24, "ry": 0.24, "region": "gearbox", "material": "cast-iron", "solid": {"kind": "extruded", "sweep": "w", "z0": lo, "z1": hi}}
        ops.append({"op": "shape-add", "scale": scale, "view": "front", "after": "crank-hub-r", "shape": cap})
    for ident in ("leg-fl", "leg-fr"):
        foot = copy.deepcopy(oplib.shape(asset, scale, "front", ident))
        foot["points"] = [[x, 0.5275 if y > 0.52 else y] for x, y in foot["points"]]
        ops.append({"op": "shape", "scale": scale, "view": "front", "id": ident, "shape": foot})
cap = copy.deepcopy(oplib.shape(asset, 50, "elevation", "polished-rod-part"))
cap["points"] = [[0.1844, 0.0246], [0.1844, -0.0239], [0.2086, -0.0239], [0.2086, 0.0246]]
ops.append({"op": "shape", "scale": "50", "view": "elevation", "id": "polished-rod-part", "shape": cap})
ops.append({"op": "shape-remove", "scale": "50", "view": "elevation", "id": "polished-rod-part-2"})
shaft500 = copy.deepcopy(shaft)
shaft500["role"] = "body"
ops.append({"op": "shape-add", "scale": "500", "view": "front", "after": "crank-arm-r", "shape": shaft500})
rod = copy.deepcopy(oplib.shape(asset, 50, "front", "polished-rod"))
rod["to"]["x"] = 0.19
ops.append({"op": "shape", "scale": "50", "view": "front", "id": "polished-rod", "shape": rod})
ops.append({"op": "shape-set", "scale": "500", "view": "front", "id": "motor-shaft", "set": {"w": 0.43}})
oplib.write("oil-pumpjack", ops)
