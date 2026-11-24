"""🚗️ Hand repairs of `sectional-garage-door` of the third round (after the repair of SC), authored against the catalog file as it stood before this round (`SAM_SOURCE`); the asset has no single generator, `apply.ts sectional-garage-door` is the repair.
Side elevation 1:50: the end stiles of the four panels are drawn (43 mm caps over the panel ends, as the front and the plan draw them): they close the finger-guard groove of every panel joint in the side view and stand 1.5 mm proud of both skins.
Front elevation 1:50: the ceiling feet of the hangers are the 2.5 mm angles the body has (they were blocks of 55 x 50 mm).
Front elevation and plan 1:50: each cable drum is its 32 mm hub, its two 90 mm flanges and its 80 mm grooved barrel (it was one cylinder of 90 mm over the full 80 mm).
Front elevation 1:100: the vertical tracks reach the end stiles (the 7.5 mm clearance between them is 0.075 mm on paper and bridged in the body by the jamb brackets and roller stems every 0.5 to 0.67 m)."""
import copy
from oplib import load, shape, write

ID = "sectional-garage-door"
a = load(ID)
ops = []
panels = a["representations"]["50"]["geometry"]["elevation"]
for n in "1234":
    for side in ("left", "right"):
        stile = copy.deepcopy(shape(a, 50, "front", f"stile-{n}-{side}"))
        ops.append({"op": "shape-add", "scale": "50", "view": "elevation", "after": f"panel-{n}", "shape": stile})
for side, sign in (("left", 1), ("right", -1)):
    foot = copy.deepcopy(shape(a, 50, "front", f"ceiling-foot-{side}"))
    for key in ("x", "y", "w", "h"):
        foot.pop(key)
    foot.update(kind="poly", closed=True, points=[[sign * z, y] for z, y in ((1.19, 2.8475), (1.19, 2.85), (1.245, 2.85), (1.245, 2.8), (1.2425, 2.8), (1.2425, 2.8475))])
    ops.append({"op": "shape", "scale": "50", "view": "front", "id": f"ceiling-foot-{side}", "shape": foot})
    for view in ("front", "plan"):
        drum = shape(a, 50, view, f"drum-{side}")

        def ring(ident, radius, z0, z1, z):
            """🛞️ One step of the drum as a disc swept over its width."""
            lo, hi = sorted((sign * z0, sign * z1))
            return dict(copy.deepcopy(drum), id=ident, rx=radius, ry=radius, z=z, solid={"kind": "extruded", "z0": lo, "z1": hi, "sweep": "w"})

        ops.append({"op": "shape", "scale": "50", "view": view, "id": f"drum-{side}", "shape": ring(f"drum-{side}", 0.08, 1.165, 1.213, drum["z"])})
        ops.append({"op": "shape-add", "scale": "50", "view": view, "after": f"drum-{side}", "shape": ring(f"drum-{side}-hub", 0.032, 1.145, 1.16, drum["z"])})
        ops.append({"op": "shape-add", "scale": "50", "view": view, "after": f"drum-{side}", "shape": ring(f"drum-{side}-flange-in", 0.09, 1.159, 1.166, drum["z"] + 0.01)})
        ops.append({"op": "shape-add", "scale": "50", "view": view, "after": f"drum-{side}", "shape": ring(f"drum-{side}-flange-out", 0.09, 1.212, 1.225, drum["z"] + 0.01)})
    track = shape(a, 100, "front", f"track-{side}")
    ops.append({"op": "shape-set", "scale": "100", "view": "front", "id": f"track-{side}", "set": {"x": 1.2095 if sign > 0 else track["x"], "w": 0.038}})
write(ID, ops)
