"""🚛️ Hand repairs of `concrete-mixer-truck` of the third round (after the ops of SAM and R2-M), authored against the catalog file as it stood before this round (`SAM_SOURCE`), each named by what the body has:
front 1:50 and 1:100: the weld seam of the belly is a pen stroke of 12.5 and 18 mm that lay on the outline of the barrel, half of it outside the body, where it ended the slit beside the right mirror head higher in the drawing than in the body; at 1:50 the seam is a mark of role `texture` like its two seam solids, at 1:100 (no texture pen, no seam solid) its ellipse is 7 mm smaller so the stroke lies inside the outline; the barrel shell (a ring of 30 flats in the kernel, an edge at the sides, 6 mm inside the round drawn outline) is turned a quarter tessellation step about its own axis, which changes nothing of the round body and brings its widest line within 1.6 mm of the drawn outline at the sides and at the top;
front 1:50 and plan 1:50: the tyres as their bodies project across the axle (rounded shoulders, three grooves) in place of rectangles;
side elevation 1:50: drive housing with the step to the 0.36 m drum flange, the rear mud flap stroke where its solid hangs (x -3.29 m, drawn at -3.31 m), the lift barrel with its 22 mm neck and its eye, the lift hanger as its two 28 mm plates with the 44 mm clearance between them (it was one block of 100 mm), the grille overlaps the cab by 4 mm;
plan 1:50: the two door handles (30 mm proud of the cab side, hidden under the square tyre corner so far), the two hinge blocks and the hinge pin of the folding chute, the intake elbow, the platform stay and the left hopper stay overlap their neighbours by 3 to 5 mm (seams of one raster cell)."""
import copy, math
from oplib import load, shape, solid, write

ID = "concrete-mixer-truck"
a = load(ID)
ops = []
QUARTER = {"rotation": [round(math.sin(math.radians(1.5)), 6), 0.0, 0.0, round(math.cos(math.radians(1.5)), 6)]}
for scale in ("50", "100"):
    ops.append({"op": "solid-set", "scale": scale, "id": "drum-shell", "set": {"transform": QUARTER}})
ops.append({"op": "shape-set", "scale": "50", "view": "front", "id": "drum-seam", "set": {"role": "texture"}})
seam = shape(a, 100, "front", "drum-seam")
ops.append({"op": "shape-set", "scale": "100", "view": "front", "id": "drum-seam", "set": {"rx": round(seam["rx"] - 0.007, 4), "ry": round(seam["ry"] - 0.007, 4)}})


def tyre(view, shape_id, solid_id, centre):
    """🛞️ A tyre seen across its axle as its revolved body projects: full width up to the widest ring, the shoulders and the three grooves beyond it. Front elevation: `centre` is the lateral centre, the axle is 0.52 m up; plan: `centre` is (x of the axle, lateral centre)."""
    s = copy.deepcopy(shape(a, 50, view, shape_id))
    prof = solid(a, 50, solid_id)["profile"][2:22]
    for k in ("x", "y", "w", "h"):
        s.pop(k)
    s["kind"] = "poly"
    s["closed"] = True
    if view == "front":
        s["points"] = [[round(centre + p, 4), round(0.52 + r, 4)] for p, r in prof] + [[round(centre + p, 4), round(0.52 - r, 4)] for p, r in reversed(prof)]
    else:
        s["points"] = [[round(centre[0] + r, 4), round(centre[1] + p, 4)] for p, r in prof] + [[round(centre[0] - r, 4), round(centre[1] + p, 4)] for p, r in reversed(prof)]
    return {"op": "shape", "scale": "50", "view": view, "id": shape_id, "shape": s}


for sid, vid, c in (("tyre-3-left", "tyre-3-l-outer", 1.095), ("tyre-3-left-inner", "tyre-3-l-inner", 0.745), ("tyre-3-right", "tyre-3-r-outer", -1.095), ("tyre-3-right-inner", "tyre-3-r-inner", -0.745), ("tyre-1-left", "tyre-1-l", 1.02), ("tyre-1-right", "tyre-1-r", -1.02)):
    ops.append(tyre("front", sid, vid, c))
AXLE_X = {"1": 3.025, "2": 1.23, "3": -1.275, "4": -2.675}
for n in "1234":
    for side, sign in (("left", 1), ("right", -1)):
        twins = (("", 1.02),) if n in "12" else (("", 1.095), ("-inner", 0.745))
        for suffix, lateral in twins:
            vid = f"tyre-{n}-{side[0]}" if n in "12" else f"tyre-{n}-{side[0]}-{'inner' if suffix else 'outer'}"
            ops.append(tyre("plan", f"tyre-{n}-{side}{suffix}", vid, (AXLE_X[n], sign * lateral)))

housing = [[-0.46, -0.25], [-0.42, -0.33], [-0.12, -0.33], [-0.1, -0.39], [-0.05, -0.39], [-0.05, -0.36], [0.02, -0.36], [0.02, 0.36], [-0.05, 0.36], [-0.05, 0.39], [-0.1, 0.39], [-0.12, 0.33], [-0.42, 0.33], [-0.46, 0.25]]
ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": "drive-housing", "set": {"points": housing}})
gearbox = copy.deepcopy(shape(a, 50, "elevation", "drive-gearbox"))
gearbox["points"] = [[-0.46, -0.25], [-0.42, -0.33], [-0.12, -0.33], [-0.1, -0.39], [-0.05, -0.39], [-0.05, 0.39], [-0.1, 0.39], [-0.12, 0.33], [-0.42, 0.33], [-0.46, 0.25]]
ops.append({"op": "shape", "scale": "50", "view": "elevation", "id": "drive-gearbox", "shape": gearbox})
ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": "mud-flap", "set": {"points": [[-3.29, 0.65], [-3.29, 0.23]]}})
barrel = [[-0.03, 0], [-0.03, 0.022], [0.03, 0.022], [0.03, 0.035], [0.285, 0.035], [0.285, 0.041], [0.31, 0.041], [0.31, 0]]
for view in ("elevation", "front", "plan"):
    if any(s["id"] == "chute-lift-barrel" for s in a["representations"]["50"]["geometry"][view]):
        ops.append({"op": "shape-set", "scale": "50", "view": view, "id": "chute-lift-barrel", "set": {"points": barrel}})
like = shape(a, 50, "elevation", "chute-lift-barrel")
eye = {"id": "chute-lift-barrel-eye", "bone": "chute-lift", "role": "detail", "z": like["z"] + 0.001, "filled": True, "tone": 0.35, "kind": "poly", "points": [[round(0.03 * math.cos(math.radians(15 * k)), 5), round(0.03 * math.sin(math.radians(15 * k)), 5)] for k in range(24)], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.02, "z1": 0.02}, "region": "chute", "material": "paint-steel"}
ops.append({"op": "shape-add", "scale": "50", "view": "elevation", "after": "chute-lift-barrel", "shape": eye})

main = shape(a, 50, "plan", "chute-main")


def block(ident, y0, y1, z0, z1, u0, u1, tone, material):
    """🔩️ A box on the chute, drawn in plan as the prism the trough is drawn as (section in the plane across the chute, swept along it)."""
    return {"op": "shape-add", "scale": "50", "view": "plan", "after": "chute-main", "shape": {"id": ident, "bone": "chute", "role": "detail", "z": main["z"] + 0.01, "filled": True, "tone": tone, "kind": "poly", "points": [[y0, z0], [y1, z0], [y1, z1], [y0, z1]], "closed": True, "solid": {"kind": "extruded", "sweep": "u", "z0": u0, "z1": u1}, "region": "chute", "material": material}}


ops.append(block("chute-hinge-1", -0.035, 0.015, -0.277, -0.247, 1.195, 1.265, 0.35, "galvanised-steel"))
ops.append(block("chute-hinge-2", -0.035, 0.015, 0.247, 0.277, 1.195, 1.265, 0.35, "galvanised-steel"))
ops.append(block("chute-ext-hinge-pin", -0.012, 0.012, -0.28, 0.28, 1.238, 1.262, 0.3, "cast-iron"))

for view in ("elevation", "front", "plan"):
    hangers = [x for x in a["representations"]["50"]["geometry"][view] if x["id"] == "chute-lift-hanger"]
    for h in hangers:
        ops.append({"op": "shape-set", "scale": "50", "view": view, "id": "chute-lift-hanger", "set": {"solid": {**h["solid"], "z0": 0.022, "z1": 0.05}}})
        ops.append({"op": "shape-add", "scale": "50", "view": view, "after": "chute-lift-hanger", "shape": {**copy.deepcopy(h), "id": "chute-lift-hanger-r", "solid": {**h["solid"], "z0": -0.05, "z1": -0.022}}})
grille = [[4.476 if x < 4.485 else x, y] for x, y in shape(a, 50, "elevation", "front-grille")["points"]]
ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": "front-grille", "set": {"points": grille}})
for side, sign in (("l", 1), ("r", -1)):
    y0, y1 = sorted((sign * 1.135, sign * 1.165))
    ops.append({"op": "shape-add", "scale": "50", "view": "plan", "after": "cab", "shape": {"id": f"door-handle-{side}", "bone": "chassis", "role": "detail", "z": 2.05, "filled": True, "tone": 0.2, "kind": "rect", "x": 3.44, "y": y0, "w": 0.16, "h": round(y1 - y0, 4), "region": "cab", "material": "paint-steel"}})
elbow = [[2.296 if abs(x - 2.3007) < 1e-9 else x, y] for x, y in shape(a, 50, "plan", "air-intake-elbow")["points"]]
ops.append({"op": "shape-set", "scale": "50", "view": "plan", "id": "air-intake-elbow", "set": {"points": elbow}})
stay = [[x, 0.576 if abs(y - 0.5806) < 1e-9 else y] for x, y in shape(a, 50, "plan", "platform-stay")["points"]]
ops.append({"op": "shape-set", "scale": "50", "view": "plan", "id": "platform-stay", "set": {"points": stay}})
pts = [[x, 0.503 if abs(y - 0.4994) < 1e-9 else y] for x, y in shape(a, 50, "plan", "hopper-stay-l")["points"]]
ops.append({"op": "shape-set", "scale": "50", "view": "plan", "id": "hopper-stay-l", "set": {"points": pts}})
write(ID, ops)
