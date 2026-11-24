"""☕️ Writes the repair ops of `used-coffee-table`: the table is turned over from its stored place (upside down on the depot table) to its place of use, and its front and plan were flat shapes that became lines half way (pose `turning-key`: front 0.914 m high against 1.214 m in the side elevation, the foot contacts 0.21 to 0.25 m off the frame). Top, shelf and frame become solids in the front and the plan of every scale: the top and the shelf are their plan outlines (with the chipped corner) swept over their thickness, at 1:50 and 1:100 the four legs and four aprons are their plan sections swept over their length (1:100 thereby draws legs and aprons apart, as its body has them, where 1:200 keeps the frame as one portal), at 1:200 and 1:500 the frame is its front portal swept over the depth of the table. The rest pose draws as before; traces of use stay flat. The script reads the flat shapes, so it writes the ops only while the catalog document still has them."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write, GEOMETRY
a = asset("used-coffee-table")
if "solid" in shape(a, 50, "front", "top"):
    sys.exit("used-coffee-table: already solid, ops kept")
ORIGIN = {"table": 0.3, "shelf": 0.26, "frame": 0.2475}
FRAME_BASE = 0.777
r6 = lambda v: round(v, 6)


def corners(s):
    """🔲️ The corner points of a rect or the points of a poly."""
    return [[s["x"], s["y"]], [s["x"] + s["w"], s["y"]], [s["x"] + s["w"], s["y"] + s["h"]], [s["x"], s["y"] + s["h"]]] if s["kind"] == "rect" else s["points"]


def span(s):
    """↕️ The range of a side elevation rect or poly across its bone."""
    ys = [p[1] for p in corners(s)]
    return min(ys), max(ys)


def slab(scale, sid, bone):
    """🪵️ A board of the table bone or the shelf bone: its plan outline swept over the thickness the side elevation draws."""
    low, high = span(shape(a, scale, "elevation", sid))
    plan = shape(a, scale, "plan", sid)
    return {"kind": "poly", "points": [[r6(ORIGIN[bone] - x), z] for x, z in corners(plan)], "closed": True, "solid": {"kind": "extruded", "sweep": "v", "z0": low, "z1": high}}


def post(sid, length):
    """🦵️ A leg or apron of the frame at 1:50: its plan section swept along the legs."""
    plan = shape(a, 50, "plan", sid)
    return {"kind": "poly", "points": [[r6(ORIGIN["frame"] - x), z] for x, z in corners(plan)], "closed": True, "solid": {"kind": "extruded", "sweep": "u", "z0": 0.0, "z1": length}}


def portal(scale):
    """🚪️ The frame of a coarse band: its front portal swept over the depth the side elevation draws."""
    low, high = span(shape(a, scale, "elevation", "frame"))
    return {"kind": "poly", "points": [[r6(y - FRAME_BASE), z] for z, y in corners(shape(a, scale, "front", "frame"))], "closed": True, "solid": {"kind": "extruded", "sweep": "v", "z0": low, "z1": high}}


PARTS = [("apron-side-f", 0.07), ("apron-side-r", 0.07), ("apron-end-l", 0.07), ("apron-end-r", 0.07), ("leg-fl", 0.425), ("leg-fr", 0.425), ("leg-rl", 0.425), ("leg-rr", 0.425)]
ops = []
for scale in (50, 100, 200, 500):
    k = str(scale)
    for sid, bone in (("top", "table"), ("shelf", "shelf")):
        solid = slab(scale, sid, bone)
        for view in ("front", "plan"):
            ops.append({"op": "shape", "scale": k, "view": view, "id": sid, "shape": like(shape(a, scale, view, sid), drop=GEOMETRY, **solid)})
    if scale in (50, 100):
        style = {"front": {"apron": shape(a, scale, "front", "apron-side" if scale == 50 else "frame"), "leg": shape(a, scale, "front", "leg-l" if scale == 50 else "frame")}, "plan": {"apron": shape(a, scale, "plan", "apron-side-f" if scale == 50 else "frame"), "leg": shape(a, scale, "plan", "leg-fl" if scale == 50 else "frame")}}
        for view in ("front", "plan"):
            for old in (("apron-side", "leg-l", "leg-r") if view == "front" else ()) if scale == 50 else ("frame",):
                ops.append({"op": "shape-remove", "scale": k, "view": view, "id": old})
            for sid, length in PARTS:
                kind = "apron" if sid.startswith("apron") else "leg"
                piece = like(style[view][kind], drop=GEOMETRY, id=sid, **post(sid, length))
                piece["z"] = style[view][kind]["z"] + (0.5 if kind == "leg" and scale == 100 else 0)
                ops.append({"op": "shape-add" if not (scale == 50 and view == "plan") else "shape", "scale": k, "view": view, **({"id": sid} if scale == 50 and view == "plan" else {"after": "shelf"}), "shape": piece})
        ops.append({"op": "note", "scale": k, "decision": "retained", "en": "Front and plan: top, shelf, the four legs and the four aprons are solids that turn with the table.", "de": "Vorderansicht und Draufsicht: Platte, Ablage, die vier Beine und die vier Zargen sind Körper, die sich mit dem Tisch drehen."})
    else:
        solid = portal(scale)
        for view in ("front", "plan"):
            ops.append({"op": "shape", "scale": k, "view": view, "id": "frame", "shape": like(shape(a, scale, view, "frame"), drop=GEOMETRY, **solid)})
write("used-coffee-table", ops)
