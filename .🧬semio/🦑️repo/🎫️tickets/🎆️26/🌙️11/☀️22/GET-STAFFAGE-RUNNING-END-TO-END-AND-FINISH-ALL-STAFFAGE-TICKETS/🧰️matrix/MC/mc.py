"""🧱️ Helpers of the ops writers of batch MC on top of `lib.py`: the rest frames of the bones in the rig plane, the bones a pose, a clip or the root turns, and the writers of solids for front and plan shapes on turning bones (`solidify` for shapes with a side elevation twin, `own_sweep` for an outline swept along the bone axis that points at the viewer, `sliver` for a stroke, `skin` for a face mark). Every writer reads only what its own ops do not change (the side elevation and the flat outline, or the solid already in place), so writing and applying again gives the same file."""
import json, math
from lib import *


def rest(a):
    """🦴️ The rest frame of every bone in the rig plane: origin (x, y) and angle in degrees (the lateral placement is not part of it)."""
    out, bones = {}, {b["id"]: b for b in a["rig"]["bones"]}

    def frame(bid):
        if bid in out:
            return out[bid]
        b = bones[bid]
        if b.get("parent") is None:
            px, py, pa = 0.0, 0.0, 0.0
        else:
            px, py, pa = frame(b["parent"])
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        out[bid] = (px + c * b.get("x", 0) - s * b.get("y", 0), py + s * b.get("x", 0) + c * b.get("y", 0), pa + b.get("angleDeg", 0))
        return out[bid]

    for bid in bones:
        frame(bid)
    return out


def to_world(f, p):
    """🌍️ A bone-local point of the rig plane in rest-pose metres."""
    c, s = math.cos(math.radians(f[2])), math.sin(math.radians(f[2]))
    return (f[0] + c * p[0] - s * p[1], f[1] + s * p[0] + c * p[1])


def to_local(f, p):
    """🏠️ A rest-pose point of the rig plane in the frame of a bone."""
    c, s = math.cos(math.radians(f[2])), math.sin(math.radians(f[2]))
    dx, dy = p[0] - f[0], p[1] - f[1]
    return (c * dx + s * dy, -s * dx + c * dy)


def outline(s):
    """✏️ The points of the authored outline of a rect, poly, ellipse, capsule or straight-line path shape (curves as 24 points)."""
    k = s["kind"]
    if k == "poly":
        return [tuple(p) for p in s["points"]]
    if k == "rect":
        return [(s["x"], s["y"]), (s["x"] + s["w"], s["y"]), (s["x"] + s["w"], s["y"] + s["h"]), (s["x"], s["y"] + s["h"])]
    if k == "ellipse":
        return [(s["cx"] + s["rx"] * math.cos(t * math.pi / 12), s["cy"] + s["ry"] * math.sin(t * math.pi / 12)) for t in range(24)]
    if k == "capsule":
        r = max(s["r0"], s["r1"])
        return [(s["x0"] - r, s["y0"] - r), (s["x0"] + r, s["y0"] + r), (s["x1"] - r, s["y1"] - r), (s["x1"] + r, s["y1"] + r), (s["x0"] - r, s["y0"] + r), (s["x0"] + r, s["y0"] - r), (s["x1"] - r, s["y1"] + r), (s["x1"] + r, s["y1"] - r)]
    try:
        return [tuple(p) for run in segments(s["d"]) for p in run]
    except ValueError:
        return curved(s["d"])


def curved(d):
    """➰️ The end points of the commands of a path with curves or arcs (absolute commands), enough for its box when the curves do not bulge beyond their ends."""
    import re
    out, tokens = [], re.findall(r"[A-Za-z]|-?[0-9.]+(?:e-?[0-9]+)?", d)
    i, cmd = 0, None
    take = {"M": 2, "L": 2, "C": 6, "S": 4, "Q": 4, "T": 2, "A": 7, "H": 1, "V": 1}
    while i < len(tokens):
        if tokens[i].isalpha():
            cmd = tokens[i]
            i += 1
            if cmd in "Zz":
                continue
        if cmd not in take:
            raise ValueError(f"path command {cmd}")
        nums = [float(x) for x in tokens[i : i + take[cmd]]]
        i += take[cmd]
        if cmd == "H":
            out.append((nums[0], out[-1][1]))
        elif cmd == "V":
            out.append((out[-1][0], nums[0]))
        else:
            out.append((nums[-2], nums[-1]))
    return out


def is_closed(s):
    """⭕️ Whether a shape has a closed outline."""
    return s["kind"] in ("rect", "ellipse", "capsule") or (s["kind"] == "poly" and s.get("closed")) or (s["kind"] == "path" and "Z" in s["d"].upper())


def is_box(s):
    """⬜️ Whether the outline of a shape is its own box (a plain rectangle)."""
    if s["kind"] == "rect":
        return s.get("r", 0) == 0
    if s["kind"] != "poly" or not s.get("closed"):
        return False
    b = box(s)
    return all((abs(p[0] - b[0]) < 1e-9 or abs(p[0] - b[1]) < 1e-9) and (abs(p[1] - b[2]) < 1e-9 or abs(p[1] - b[3]) < 1e-9) for p in s["points"])


def fill_of(s):
    """📐️ The share of its own box that the outline of a closed shape fills (1 for a rectangle)."""
    pts = outline(s)
    if s["kind"] == "capsule":
        return 0.9
    if s["kind"] == "path" and any(c in s["d"] for c in "CSQTAcsqta"):
        return 0
    b = box(s)
    area = abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2
    full = (b[1] - b[0]) * (b[3] - b[2])
    return area / full if full > 0 else 0


def other_side(name):
    """🪞️ The ids a part of the other side may carry."""
    out = []
    for x, y in (("-left", "-right"), ("-right", "-left"), ("-l-", "-r-"), ("-r-", "-l-"), ("left-", "right-"), ("right-", "left-")):
        if x in name:
            out.append(name.replace(x, y))
    for x, y in (("-l", "-r"), ("-r", "-l")):
        if name.endswith(x):
            out.append(name[: -len(x)] + y)
    return out


def subtree(a, roots):
    """🌳️ The given bones and all bones below them."""
    out, bones = set(roots), a["rig"]["bones"]
    grew = True
    while grew:
        grew = False
        for b in bones:
            if b.get("parent") in out and b["id"] not in out:
                out.add(b["id"])
                grew = True
    return out


def views(a, scale):
    """🗺️ The three shape lists of a band."""
    return a["representations"][str(scale)]["geometry"]


def find(a, scale, view, sid):
    """🔎️ A drawn shape by id or None."""
    return next((s for s in views(a, scale)[view] if s["id"] == sid), None)


def solidify(a, bones, twins=None, force=(), skip=(), half=HAIR, quiet=False, loose=0.85, only=None):
    """🧊️ The ops that make every closed front and plan shape of the given bones, in every band, the side elevation outline of its twin (same id unless `twins` names another) swept laterally over the span the shape has: only where the flat shape is a rectangle in its view (its outline fills at least `loose` of its box; a rounded corner is given up for the true profile) or is named in `force`, and where the twin shows the same height (front) or forward range (plan) at rest within 3 mm or the shape is named in `force`. A shape without a twin of its id takes the twin of the other side (`-left` for `-right`, `-l` for `-r` and back) when the side elevation draws that one. `only` limits the bands. What it leaves is printed for a manual op. Shapes that are solids already are kept."""
    twins, frames, ops, left = twins or {}, rest(a), [], []
    for scale in scales(a):
        if only is not None and str(scale) not in only:
            continue
        g = views(a, scale)
        for view in ("front", "plan"):
            for s in g[view]:
                if s["bone"] not in bones or s["id"] in skip:
                    continue
                if "solid" in s:
                    ops.append(op(scale, view, s["id"], s))
                    continue
                name = twins.get(s["id"]) or next((n for n in [s["id"]] + other_side(s["id"]) if any(t["id"] == n and t["bone"] == s["bone"] for t in g["elevation"])), s["id"])
                twin = next((t for t in g["elevation"] if t["id"] == name and t["bone"] == s["bone"]), None)
                why = None
                if twin is None:
                    why = "no twin"
                elif not is_closed(s) or not is_closed(twin):
                    why = "open"
                elif fill_of(s) < loose and s["id"] not in force:
                    why = "not a rectangle (fills %.2f of its box)" % fill_of(s)
                else:
                    pts = [to_world(frames[s["bone"]], p) for p in outline(twin)]
                    b = box(s)
                    own = (b[2], b[3]) if view == "front" else (b[0], b[1])
                    got = (min(p[1] for p in pts), max(p[1] for p in pts)) if view == "front" else (min(p[0] for p in pts), max(p[0] for p in pts))
                    if max(abs(own[0] - got[0]), abs(own[1] - got[1])) > 0.003 and s["id"] not in force:
                        why = "rest range %.3f..%.3f against twin %.3f..%.3f" % (own[0], own[1], got[0], got[1])
                if why is not None:
                    left.append("1:%s %s %s [%s] %s %s: %s" % (scale, view, s["id"], s["bone"], s["kind"], "fill" if s.get("filled") else "line", why))
                    continue
                ops.append(prism(a, scale, view, s["id"], name, half))
    if not quiet:
        for line in left:
            print("   left flat:", line)
    return ops


def sliver(target, pts, z0, z1, t=0.001, sweep="w"):
    """〰️ A stroke as a solid: the target shape (paint and bone kept, unfilled) becomes a closed sliver of thickness 2 t about a bone-local polyline `pts`, swept from z0 to z1: with `sweep` "w" the polyline lies in the rig plane (u, v) and is swept laterally (an edge or a band seen from the side), with "u" it lies in the (v, w) plane and with "v" in the (u, w) plane (a mark drawn on a face, swept thinly along the normal of the face). A single point gives a square of side 2 t."""
    if len(pts) == 1:
        (x, y), ring = pts[0], None
        ring = [[x - t, y - t], [x + t, y - t], [x + t, y + t], [x - t, y + t]]
    else:
        left, right = [], []
        for i, (x, y) in enumerate(pts):
            (ax, ay), (bx, by) = pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)]
            n = math.hypot(bx - ax, by - ay)
            nx, ny = -(by - ay) / n, (bx - ax) / n
            left.append([round(x + t * nx, 6), round(y + t * ny, 6)])
            right.append([round(x - t * nx, 6), round(y - t * ny, 6)])
        ring = left + right[::-1]
    return like(target, drop=GEOMETRY, kind="poly", points=[[round(x, 6), round(y, 6)] for x, y in ring], closed=True, solid=extruded(sweep, z0, z1))


def own_sweep(a, scale, view, sid, lo, hi):
    """🧱️ The op that makes a front or plan shape its own outline swept along the bone axis that points at the viewer (forward in the front, up in the plan) from `lo` to `hi`, both rest-pose positions on that world axis: for a part whose outline in the view is not a rectangle and whose bone rests along a world axis (0, 90, 180 or 270 degrees). The outline at rest stays as drawn; a shape that is a solid already is kept."""
    s = shape(a, scale, view, sid)
    if "solid" in s:
        return op(scale, view, sid, s)
    ox, oy, angle = rest(a)[s["bone"]]
    q = round(angle / 90) % 4
    if abs(angle - round(angle / 90) * 90) > 1e-6:
        raise ValueError(f"{sid}: bone {s['bone']} rests at {angle} degrees")
    ux, uy = [(1, 0), (0, 1), (-1, 0), (0, -1)][q]
    vx, vy = -uy, ux
    pts = outline(s)
    if view == "front":
        along_u = uy != 0
        coord = (lambda y: (y - oy) * uy) if along_u else (lambda y: (y - oy) * vy)
        profile = [[round(coord(y), 6), round(x, 6)] for x, y in pts]
        sign = vx if along_u else ux
        origin = ox
    else:
        along_u = ux != 0
        coord = (lambda x: (x - ox) * ux) if along_u else (lambda x: (x - ox) * vx)
        profile = [[round(coord(x), 6), round(y, 6)] for x, y in pts]
        sign = vy if along_u else uy
        origin = oy
    z = sorted(((lo - origin) * sign, (hi - origin) * sign))
    return op(scale, view, sid, like(s, drop=GEOMETRY, kind="poly", points=profile, closed=True, solid=extruded("v" if along_u else "u", z[0], z[1])))


def add(scale, view, after, new):
    """➕️ The op that adds a shape after another one (or replaces the shape of that id)."""
    return {"op": "shape-add", "scale": str(scale), "view": view, "after": after, "shape": new}


def put(path, value):
    """🧷️ The op that sets a value at a JSON path of the asset (steps are keys, indices or `{ "id": … }` matches)."""
    return {"op": "set", "path": path, "value": value}


def split_anchor(a, lumped, parts):
    """✂️ The anchor list with a lumped anchor replaced by its parts (D45); when the lumped anchor is gone the parts are refreshed in place, so a second run gives the same list."""
    out, done = [], False
    ids = {p["id"] for p in parts}
    for n in a["anchors"]:
        if n["id"] == lumped or n["id"] in ids:
            if not done:
                out.extend(parts)
                done = True
        else:
            out.append(n)
    if not done:
        raise ValueError(f"neither {lumped} nor its parts are anchors")
    return out


def split_loads(a, lumped_anchor, sides):
    """⚖️ The ops that replace, in every pose, a load on a lumped anchor by one load per part with an equal share of its force: `sides` maps the suffix of the new load id to `(anchor id, { en, de } suffix of the title)`. A pose whose loads are split already is rewritten as it is."""
    ops = []
    for pose in a["poses"]:
        loads, n = [], len(sides)
        for load in pose.get("loads", []):
            if load.get("anchor") != lumped_anchor:
                loads.append(load)
                continue
            for suffix, (anchor, title) in sides.items():
                part = dict(load, id=f"{load['id']}-{suffix}", anchor=anchor, title={k: f"{v}{title[k]}" for k, v in load["title"].items()})
                for k in ("fxN", "fyN", "fzN", "momentNm"):
                    if k in part:
                        part[k] = round(part[k] / n, 5)
                loads.append(part)
        if "loads" in pose:
            ops.append(put(["poses", {"id": pose["id"]}, "loads"], loads))
    return ops


def stand_feet(a, lumped, stem, x, z, length, width):
    """🦶️ The ops that split the lumped contact of a two-leg stand into one contact per foot at its lateral place (D45): the anchors, the contacts of poses and clips, the supports (op `contact-split`), and the clip events that name the anchor, one event per foot."""
    old = next((n for n in a["anchors"] if n["id"] == lumped), None) or next(n for n in a["anchors"] if n["id"] == f"{stem}-left")
    sides = (("left", z, "Left stand foot", "Linker Ständerfuß"), ("right", -z, "Right stand foot", "Rechter Ständerfuß"))
    parts = [dict({k: v for k, v in old.items() if k != "z"}, id=f"{stem}-{side}", x=x, z=zz, title={"en": en, "de": de}, footprintLengthM=length, footprintWidthM=width) for side, zz, en, de in sides]
    ops = [{"op": "contact-split", "anchor": lumped, "parts": parts}]
    for clip in a.get("clips", []):
        events, changed = [], False
        for e in clip.get("events", []):
            if e.get("target") != lumped:
                events.append(e)
                continue
            changed = True
            for side, zz, en, de in sides:
                events.append(dict(e, target=f"{stem}-{side}", title={"en": f"{en} touches down" if e["kind"] == "contact-on" else f"{en} lifts off", "de": f"{de} setzt auf" if e["kind"] == "contact-on" else f"{de} hebt ab"}))
        if changed or any(e.get("target", "").startswith(f"{stem}-") for e in clip.get("events", [])):
            ops.append(put(["clips", {"id": clip["id"]}, "events"], events))
    return ops


def rebind(a, ids, bone, only=("front", "plan")):
    """🔗️ The ops that bind the named front and plan shapes, in every band that draws them, to another bone: the front and plan picture of a body of revolution about the lateral axis (a tyre, a brake drum, a hub) does not change when the wheel spins, so it belongs to the axle bone that carries the wheel and does not spin; flat on the spinning wheel bone it turned over with the wheel. Shapes that are solids are left on their bone."""
    ops = []
    for scale in scales(a):
        for view in only:
            for s in views(a, scale)[view]:
                if s["id"] in ids and "solid" not in s:
                    ops.append({"op": "shape-set", "scale": str(scale), "view": view, "id": s["id"], "set": {"bone": bone}})
    return ops


def member(target, a_end, b_end, d):
    """🦿️ A round bar as a `member` shape between two bone-local points (x along the bone, y its normal, z lateral from the bone origin): paint, bone and order of the target shape, the same round bar in every view and pose."""
    return like(target, drop=GEOMETRY, kind="member", **{"from": {"x": round(a_end[0], 6), "y": round(a_end[1], 6), "z": round(a_end[2], 6)}, "to": {"x": round(b_end[0], 6), "y": round(b_end[1], 6), "z": round(b_end[2], 6)}}, section={"kind": "circle", "d": d})


def world_box(a, scale, sid, view="elevation"):
    """📦️ The rest-pose box [x0, x1, y0, y1] of a side elevation shape in asset metres."""
    s = shape(a, scale, view, sid)
    pts = [to_world(rest(a)[s["bone"]], p) for p in outline(s)]
    return [min(p[0] for p in pts), max(p[0] for p in pts), min(p[1] for p in pts), max(p[1] for p in pts)]


def lateral_rest(a, bone):
    """↔️ The lateral rest position of a bone origin: the sum of the `z` of the bone and of its ancestors."""
    bones, z = {b["id"]: b for b in a["rig"]["bones"]}, 0.0
    while bone is not None:
        z += bones[bone].get("z", 0)
        bone = bones[bone].get("parent")
    return z


def copy_from(a, scale, view, sid, **changes):
    """🧬️ A copy of a shape of another band with given members changed."""
    return like(shape(a, scale, view, sid), **changes)


def clip(points, axis, keep_below, limit):
    """✂️ A closed polygon cut by a line of constant x (`axis` 0) or y (`axis` 1): the part at or below `limit` when `keep_below`, else the part at or above it (Sutherland-Hodgman)."""
    inside = (lambda p: p[axis] <= limit + 1e-12) if keep_below else (lambda p: p[axis] >= limit - 1e-12)
    out = []
    for i, p in enumerate(points):
        q = points[(i + 1) % len(points)]
        if inside(p):
            out.append([round(p[0], 6), round(p[1], 6)])
        if inside(p) != inside(q):
            t = (limit - p[axis]) / (q[axis] - p[axis])
            out.append([round(p[0] + t * (q[0] - p[0]), 6), round(p[1] + t * (q[1] - p[1]), 6)])
    return out


def shift_path(d, dx):
    """➡️ A path of absolute `M`, `L`, `A` and `Z` commands moved along x."""
    import re
    tokens, out, i, cmd = re.findall(r"[A-Za-z]|-?[0-9.]+(?:e-?[0-9]+)?", d), [], 0, None
    take = {"M": 2, "L": 2, "A": 7}
    while i < len(tokens):
        if tokens[i].isalpha():
            cmd = tokens[i]
            out.append(cmd)
            i += 1
            if cmd in "Zz":
                continue
        if cmd not in take:
            raise ValueError(f"path command {cmd}")
        nums = [float(x) for x in tokens[i : i + take[cmd]]]
        i += take[cmd]
        nums[-2] = round(nums[-2] + dx, 6)
        out.extend(repr(n) if n != int(n) else str(int(n)) for n in nums)
    return " ".join(out)
