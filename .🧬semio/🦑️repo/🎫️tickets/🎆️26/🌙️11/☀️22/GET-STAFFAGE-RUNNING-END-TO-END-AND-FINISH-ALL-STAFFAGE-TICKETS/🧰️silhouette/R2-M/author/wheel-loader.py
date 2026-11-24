"""🚧️ Hand repairs of `wheel-loader` of the second round (the traced parts follow in wheel-loader.sh).

- Volume 1:50 and 1:100: the belly plate of the front frame ended 5 mm before the axle and left the frame open to the ground between axle housing, axle pads and nose skid (triangular voids of 70 mm, slits of 10 mm); it now runs under the axle to the nose skid, along the lower edge of the side plates (unseen in the side and front elevation).
- Drawing 1:50, side elevation and plan: the four work lamps stood 15 mm in front of where their solids sit into the roof edge.
- Drawing 1:50 side elevation: the wiper begins inside the cab front instead of 0.6 mm in front of it (a seam of one raster cell).
- Drawing 1:50 and 1:100 side elevation: the lower hitch pin is 160 mm long as its solid, not 190 mm, and the middle propeller shaft behind it is drawn whole (it had a notch where the long pin stood).
- Drawing 1:50 side elevation: the two rounded elbows of the handrail overlap the rails they join by 3 mm (seams of one raster cell).
- Drawing 1:50 plan: the two slivers between roof edge and outer work lamps go (the lamps stand where their solids are now).
"""
import copy
import oplib

asset = oplib.load("wheel-loader")
ops = []
plate = [[0.16, 0.619], [2.06, 0.5808], [2.06, 0.6308], [0.16, 0.669]]
for scale in ("50", "100"):
    ops.append({"op": "solid-set", "scale": scale, "id": "belly-plate", "set": {"profile": plate}})
for view in ("elevation", "plan"):
    for s in asset["representations"]["50"]["geometry"][view]:
        if s["id"].startswith("work-lamp-") and s.get("kind") == "rect":
            ops.append({"op": "shape-set", "scale": "50", "view": view, "id": s["id"], "set": {"x": 0.345}})
wiper = copy.deepcopy(oplib.shape(asset, 50, "elevation", "cab-wiper-arm"))
wiper["points"] = [[0.236 if x < 0.2410 else x, y] for x, y in wiper["points"]]
ops.append({"op": "shape", "scale": "50", "view": "elevation", "id": "cab-wiper-arm", "shape": wiper})
pin = [[0, 0.59], [0.055, 0.59], [0.055, 0.75], [0, 0.75], [-0.055, 0.75], [-0.055, 0.59]]
for scale in ("50", "100"):
    for ident in ("hitch-pin-lower", "hitch-pin-lower-edge"):
        ops.append({"op": "shape-set", "scale": scale, "view": "elevation", "id": ident, "set": {"points": pin}})
for scale in ("50", "100"):
    shaft = copy.deepcopy(oplib.shape(asset, scale, "elevation", "propshaft-m"))
    shaft["points"] = [p for p in shaft["points"] if not (abs(p[0]) < 0.0575 and p[1] < 0.79)]
    ops.append({"op": "shape", "scale": scale, "view": "elevation", "id": "propshaft-m", "shape": shaft})
for ident in ("work-lamp-l-1-part", "work-lamp-r-1-part"):
    ops.append({"op": "shape-remove", "scale": "50", "view": "plan", "id": ident})
for ident, edge in (("handrail-l-1", -1.4994), ("handrail-l-0", -2.4502)):
    elbow = copy.deepcopy(oplib.shape(asset, 50, "elevation", ident))
    step = -0.0036 if ident == "handrail-l-1" else 0.0032
    elbow["points"] = [[round(edge + step, 4) if abs(x - edge) < 1e-6 else x, 2.797 if abs(y - 2.8003) < 1e-6 else y] for x, y in elbow["points"]]
    ops.append({"op": "shape", "scale": "50", "view": "elevation", "id": ident, "shape": elbow})
oplib.write("wheel-loader", ops)
