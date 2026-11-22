"""✏️ The 2D drawings of the used marching drums at every authored scale: filled shapes from the outlines of the parts, the turned sticks and the strainer lever as `solid` shapes, strokes for thin parts at 1:500 and the hand-drawn traces of use.

@see author_parts.py — the parts the outlines come from
"""
import math

from author_geometry import Ring, Tube, circle_pts, nearness, offset_path, r6, view_of
from author_parts import BAR_R, FRAMES, XS, YB
from author_scales import bass_body, snare_body

VIEWS = ("elevation", "front", "plan")


def coarse_draw(scale, part):
    """🗺 The drawing decomposition of a merged turned body at 1:200 and 1:500: hoop bands, shell and head faces as separate regions (merged into `regions` where a band is below the print feature)."""
    if part.id == "bass-body":
        zh = 0.158 if scale == 200 else 0.141
        ze = 0.200 if scale == 200 else 0.216
        rh = 0.3245 if scale == 200 else 0.342
        hoops = ["bass-hoops"] if scale == 200 else ["bass-hoops", "bass-hardware"]
        out = [(Ring("z", (0.0, YB, 0.0), -zh, zh, 0.297, 0.305), ["bass-shell"], 0.62, ("elevation", "front", "plan"))]
        for s in (-1, 1):
            out.append((Ring("z", (0.0, YB, 0.0), (zh if s > 0 else -ze), (ze if s > 0 else -zh), 0.3065 if scale == 200 else 0.0, rh), hoops, 0.5, ("front", "plan")))
        if scale == 200:
            out.append((Ring("z", (0.0, YB, 0.0), -0.200, -0.158, 0.3065, 0.3245), ["bass-hoops"], 0.5, ("elevation",)))
            out.append((Ring("z", (0.0, YB, 0.0), -0.1785, -0.178, 0.0, 0.3065), ["bass-heads"], 0.82, ("elevation",)))
        else:
            out.append((Ring("z", (0.0, YB, 0.0), -ze, ze, 0.0, rh), ["bass-heads", "bass-hoops", "bass-hardware"], 0.7, ("elevation",)))
        return out
    if part.id == "snare-body" and scale == 500:
        whole = Ring("y", (XS, 0.0, 0.0), 0.0, 0.329, 0.0, 0.1905)
        return [(whole, ["snare-wrap", "snare-hardware", "snare-heads"], 0.8, ("elevation", "front")), (whole, ["snare-heads", "snare-hardware", "snare-wrap"], 0.85, ("plan",))]
    if part.id == "snare-body":
        hw = ["snare-hardware"]
        out = [(Ring("y", (XS, 0.0, 0.0), 0.030, 0.299, 0.172, 0.178), ["snare-wrap"], 0.78, ("elevation", "front"))]
        out.append((Ring("y", (XS, 0.0, 0.0), 0.317, 0.31725, 0.0, 0.1785), ["snare-heads"], 0.9, ("elevation",)))
        for a0, a1 in ((0.0, 0.030), (0.299, 0.329)):
            out.append((Ring("y", (XS, 0.0, 0.0), a0, a1, 0.1795, 0.1905), hw, 0.85, ("elevation", "front")))
        out.append((Ring("y", (XS, 0.0, 0.0), 0.305, 0.329, 0.1795, 0.1905), hw, 0.85, ("plan",)))
        out.append((Ring("y", (XS, 0.0, 0.0), 0.317, 0.31725, 0.0, 0.1795), ["snare-heads"], 0.9, ("plan",)))
        return out
    if scale == 500 and "-bar-" in part.id:
        tube = part.prims[0]
        wide = Tube(tube.path, tube.z, 0.033)
        return [(wide, ["carrier-frames", "carrier-pads"], part.tone, part.views)]
    return None


def stick_profile(scale):
    """🥢 The half outline of a stick in its bone frame (u from the fulcrum, v radial), butt to tip, for the revolved `solid` shapes."""
    if scale in (50,):
        pts = [(-0.13, 0.0), (-0.13, 0.0055), (-0.128, 0.0072), (-0.1225, 0.0075), (0.18, 0.0072), (0.262, 0.004), (0.2655, 0.0042), (0.2705, 0.0045), (0.2745, 0.0035), (0.276, 0.0)]
    else:
        pts = [(-0.13, 0.0), (-0.13, 0.006), (-0.1225, 0.0075), (0.2715, 0.0045), (0.276, 0.0)]
    return [[r6(a), r6(b)] for a, b in pts]


class Sheet:
    """📄 The shapes of one view of one scale, with distinct paint orders so every shape is its own layer."""

    def __init__(self):
        self.shapes = []
        self.count = 0

    def add(self, shape, near, bias=0.0):
        self.count += 1
        shape["z"] = round(5.0 + near + bias + self.count * 1e-5, 5)
        self.shapes.append(shape)


def geometry_of(item, view, frame):
    """🔺 The shape geometry of one outline item in paper coordinates (elevation bone-local)."""
    if isinstance(item, tuple) and item[0] == "capsule":
        x0, y0, x1, y1, r0, r1 = item[1]
        if view == "elevation":
            (x0, y0), (x1, y1) = frame.elev(x0, y0), frame.elev(x1, y1)
        return {"kind": "capsule", "x0": r6(x0), "y0": r6(y0), "x1": r6(x1), "y1": r6(y1), "r0": r6(r0), "r1": r6(r1)}
    if isinstance(item, tuple) and item[0] == "ellipse":
        cx, cy, rx, ry = item[1]
        g = {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry)}
        if view == "elevation":
            c = frame.elev(cx, cy)
            g["cx"], g["cy"] = c[0], c[1]
            if abs(rx - ry) > 1e-9 and abs(frame.deg) > 1e-9:
                g["rotationDeg"] = r6(-frame.deg)
        return g
    return None


def ring_points(ring, view, frame):
    """📐 The points of one ring, converted to the bone frame in the elevation."""
    return [frame.elev(p[0], p[1]) if view == "elevation" else [r6(p[0]), r6(p[1])] for p in ring]


def path_d(rings):
    """〰 An SVG path of closed polygon rings (even-odd: a ring inside another is a hole)."""
    return " ".join("M " + " L ".join(f"{p[0]:.6f} {p[1]:.6f}" for p in ring) + " Z" for ring in rings)


def outline_shapes(sid, bone, prim, view, frame, base):
    """✏️ The filled shapes of one primitive in one view: polygons, annuli as two-ring paths, capsules and ellipses."""
    items = prim.outline(view)
    out = []
    if items and isinstance(items[0], list) and len(items) == 2 and isinstance(items[1], list):
        out.append(dict(base, id=sid, bone=bone, kind="path", d=path_d([ring_points(r, view, frame) for r in items])))
        return out
    for k, item in enumerate(items):
        g = geometry_of(item, view, frame)
        if g is None:
            g = {"kind": "poly", "points": ring_points(item, view, frame), "closed": True}
        out.append(dict(base, id=sid if len(items) == 1 else f"{sid}-{k}", bone=bone, **g))
    return out


def copies(part, prim):
    """🔁 The world copies of a repeated primitive (a lug, claw or ear around a drum axis), as lightweight primitives."""
    if part.repeat is None:
        return [prim]
    axis, c, n, step = part.repeat
    out = []
    for k in range(n):
        out.append(rotated(prim, axis, c, step * k))
    return out


def rotated(prim, axis, c, deg):
    """🔄 A primitive turned by deg about the world axis through c (box or upright cylinder)."""
    from author_geometry import Box, q_axis, q_mul, q_rot

    ax = (0, 1, 0) if axis == "y" else (0, 0, 1)
    sign = -1 if axis == "y" else 1
    q = q_axis(ax, sign * deg)
    if isinstance(prim, Box):
        d = (prim.c[0] - c[0], prim.c[1] - c[1], prim.c[2] - c[2])
        e = q_rot(q, d)
        return Box((c[0] + e[0], c[1] + e[1], c[2] + e[2]), prim.size, q_mul(q, prim.q))
    d = (prim.c[0] - c[0], prim.c[1] - c[1], prim.c[2] - c[2])
    e = q_rot(q, d)
    return Ring(prim.axis, (c[0] + e[0], c[1] + e[1], c[2] + e[2]), prim.a0, prim.a1, prim.r_in, prim.r_out)


def regions_field(regions):
    """🏷 The region binding of a shape: one region or a merged list."""
    return {"region": regions[0]} if len(regions) == 1 else {"regions": list(regions)}


def draw_scale(scale, parts):
    """🖼 The three views of one scale: every drawn part as filled shapes, the sticks and the strainer lever as `solid` shapes, thin parts as strokes at 1:500, plus the traces of use."""
    sheets = {v: Sheet() for v in VIEWS}
    for part in parts:
        if not part.draw:
            continue
        frame = FRAMES[part.bone]
        if part.bone.startswith("stick-"):
            draw_stick(scale, part, sheets)
            continue
        if part.id == "strainer-lever":
            draw_lever(part, sheets)
            continue
        if scale == 500 and part.id.endswith("belly-pad"):
            continue
        specs = coarse_draw(scale, part)
        if specs is None:
            specs = [(prim, [part.region], part.tone, part.views) for prim in part.prims]
        for i, (prim, regions, tone, views) in enumerate(specs):
            for view in views:
                stroke = scale == 500 and thin_at_500(part, view)
                for k, cp in enumerate(copies(part, prim)):
                    sid = part.id if len(specs) == 1 else f"{part.id}-{i}"
                    if part.repeat is not None:
                        sid = f"{sid}-{k}"
                    base = {"role": part.role, "filled": not stroke, "tone": tone, "material": part.material}
                    base.update(regions_field(["carrier-frames", "carrier-pads"] if stroke else regions))
                    if skipped(scale, view, part, cp):
                        continue
                    near = drum_nearness(part, cp, view)
                    if stroke:
                        for shape in stroke_shapes(sid, part, cp, view, frame, base):
                            sheets[view].add(shape, near)
                    else:
                        for shape in outline_shapes(sid, part.bone, cp, view, frame, base):
                            sheets[view].add(shape, near)
    traces(scale, sheets)
    return {v: sheets[v].shapes for v in VIEWS}


HARDWARE = ("snare-lugs", "snare-ears", "snare-strainer-body", "snare-butt", "snare-mount", "bass-lugs", "bass-rods", "bass-claws", "bass-handles", "bass-cleat", "bass-mount", "bass-tension")


def side_of(part, prim, view):
    """🧭 Where a part sits on its drum seen from a view: its radial direction from the drum axis along the view direction (1 nearest to the viewer, -1 behind the drum)."""
    c = prim.centroid()
    if part.id.startswith("snare"):
        d = (c[0] - XS, c[2])
        n = math.hypot(*d) or 1.0
        return {"elevation": -d[1] / n, "front": d[0] / n, "plan": None}[view]
    d = (c[0], c[1] - YB)
    n = math.hypot(*d) or 1.0
    return {"elevation": None, "front": d[0] / n, "plan": d[1] / n}[view]


def drum_nearness(part, prim, view):
    """🔭 The paint depth of a shape: hardware on a drum is painted just over the drum body when it sits on the near half and just under it on the far half; everything else by its nearest point."""
    if not part.id.startswith(HARDWARE):
        return nearness(prim.points3(), view)
    side = side_of(part, prim, view)
    if side is None:
        return nearness(prim.points3(), view)
    if part.id.startswith("snare"):
        body = {"elevation": (0.178, 0.1905), "front": (XS + 0.178, XS + 0.1905)}[view]
    else:
        body = {"front": (0.305, 0.3245), "plan": (YB + 0.305, YB + 0.3245)}[view]
    return body[1] + 0.001 + 0.0001 * side if side > 0 else body[0] - 0.001 + 0.0001 * side


def skipped(scale, view, part, prim):
    """🙈 Whether a hardware copy is left out of a view because the drum hides it but for a sliver below the print feature at the end of the band: tube lugs under the hoop in plan, lugs and the butt edge-on at the silhouette, bass lugs behind the claws, mounts under the shell edge."""
    pid = part.id
    if pid == "snare-lugs" and view == "plan":
        return True
    if scale == 100:
        side = side_of(part, prim, view) if pid.startswith(HARDWARE) else None
        if pid in ("snare-lugs",) and side is not None and abs(side) < 0.9:
            return True
        if pid == "snare-butt" and view in ("front", "plan"):
            return True
        if pid.startswith("bass-lugs") and (view == "elevation" or (side is not None and abs(side) < 0.9)):
            return True
        if pid == "bass-mount" and view == "plan":
            return True
    return False


def thin_at_500(part, view):
    """🪡 Whether a part is drawn as a stroke at 1:500 (a bar, a plate seen edge-on): its filled outline would print under 0.2 mm at 1:1000."""
    if part.id.endswith("-plate"):
        return view != "front"
    return False


def stroke_shapes(sid, part, prim, view, frame, base):
    """➖ The centre-line stroke of a thin part at 1:500."""
    if isinstance(prim, Tube):
        if view == "elevation":
            pts = [frame.elev(x, y) for x, y in prim.path]
        elif view == "front":
            ys = [p[1] for p in prim.path]
            pts = [[r6(prim.z), r6(min(ys))], [r6(prim.z), r6(max(ys))]]
        else:
            xs = [p[0] for p in prim.path]
            pts = [[r6(min(xs)), r6(prim.z)], [r6(max(xs)), r6(prim.z)]]
    else:
        c, s = prim.c, prim.size
        if view == "elevation":
            pts = [frame.elev(c[0], c[1] - s[1] / 2), frame.elev(c[0], c[1] + s[1] / 2)]
        else:
            pts = [[r6(c[0]), r6(c[2] - s[2] / 2)], [r6(c[0]), r6(c[2] + s[2] / 2)]]
    return [dict(base, id=sid, bone=part.bone, kind="poly", points=pts, closed=False)]


def draw_stick(scale, part, sheets):
    """🥢 A stick in every view as one revolved `solid` shape about its own axis (it swings in the playing clips), a stroke along its axis at 1:200 and 1:500 where its section is below the print feature."""
    bone = part.bone
    f = FRAMES[bone]
    near_pts = [(f.o[0] - 0.13, f.o[1] + 0.0075, f.o[2] - 0.0075), (f.o[0] + 0.276, f.o[1] + 0.0075, f.o[2] + 0.0075)]
    base = {"role": "body", "tone": part.tone, "region": "sticks", "material": "hickory"}
    for view in VIEWS:
        near = nearness(near_pts, view)
        if scale == 500:
            if view == "front":
                continue
            pts = [[-0.13, 0.0], [0.276, 0.0]] if view == "elevation" else [[r6(f.o[0] - 0.13), r6(f.o[2])], [r6(f.o[0] + 0.276), r6(f.o[2])]]
            sheets[view].add(dict(base, id=part.id, bone=bone, filled=False, kind="poly", points=pts, closed=False), near)
            continue
        shape = dict(base, id=part.id, bone=bone, filled=True, kind="poly", points=stick_profile(scale), closed=True, solid={"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": 0, "mirror": True})
        sheets[view].add(shape, near)


def draw_lever(part, sheets):
    """🎚 The throw-off lever of the strainer as an extruded `solid` across its 8 mm thickness, so it turns right in all three views."""
    prof = [[0.0, -0.005], [0.05, -0.005], [0.05, 0.005], [0.0, 0.005]]
    base = {"role": "detail", "tone": part.tone, "region": "snare-hardware", "material": "chrome-steel"}
    for view in VIEWS:
        near = {"elevation": 0.212, "front": XS + 0.2, "plan": 0.33}[view]
        sheets[view].add(dict(base, id=part.id, bone=part.bone, filled=True, kind="poly", points=prof, closed=True, solid={"kind": "extruded", "sweep": "w", "z0": -0.212, "z1": -0.204}), near, 0.05)


def trace(sheets, view, sid, bone, pts, region, near, closed=False, role="texture", tone=0.3):
    """🩹 One hand-drawn trace of use: an open or closed unfilled stroke just above the surface it marks (`near` is the nearness of that surface)."""
    sheets[view].add({"id": sid, "bone": bone, "role": role, "filled": False, "tone": tone, "region": region, "kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}, near, 0.0002)


def traces(scale, sheets):
    """🩹 The traces of use the scale can print: worn chrome at the rimshot zone, the worn coating of the batter head, the faded and lifted wrap seam, the stained calf head with its beater spot, the glued crack of the bass hoop, the taped sling and the old inventory sticker."""
    fs = FRAMES["snare"]
    fb = FRAMES["bass"]
    if scale == 50:
        e = lambda x, y: fs.elev(x, y)
        trace(sheets, "elevation", "wrap-seam", "snare", [e(XS + 0.062, 0.03), e(XS + 0.062, 0.30)], "snare-wrap", 0.178)
        trace(sheets, "elevation", "wrap-lifted", "snare", [e(XS + 0.062, 0.21), e(XS + 0.085, 0.215), e(XS + 0.092, 0.26), e(XS + 0.062, 0.275)], "snare-wrap", 0.178)
        trace(sheets, "elevation", "wrap-faded", "snare", [e(XS - 0.15, 0.06), e(XS - 0.05, 0.05), e(XS + 0.02, 0.09), e(XS - 0.02, 0.16), e(XS - 0.13, 0.15)], "snare-wrap", 0.178, closed=True)
        trace(sheets, "plan", "batter-worn", "snare", circle_pts(XS - 0.01, 0.0, 0.055, 16), "snare-heads", 0.31725, closed=True)
        trace(sheets, "plan", "batter-worn-core", "snare", circle_pts(XS - 0.005, -0.008, 0.022, 12), "snare-heads", 0.31725, closed=True)
        trace(sheets, "plan", "rim-wear", "snare", circle_pts(XS, 0.0, 0.184, 10, 150.0, 210.0, closed=False), "snare-hardware", 0.329, tone=0.5)
        trace(sheets, "elevation", "calf-beater-spot", "bass", [fb.elev(*p) for p in circle_pts(0.02, YB + 0.03, 0.075, 16)], "bass-heads", 0.1785, closed=True)
        trace(sheets, "elevation", "calf-stain", "bass", [fb.elev(x, y) for x, y in [(-0.16, YB - 0.12), (-0.09, YB - 0.17), (-0.03, YB - 0.14), (-0.07, YB - 0.08), (-0.14, YB - 0.07)]], "bass-heads", 0.1785, closed=True)
        trace(sheets, "elevation", "hoop-crack", "bass", [fb.elev(x, y) for x, y in [(-0.1650, YB + 0.2858), (-0.1560, YB + 0.2721), (-0.1640, YB + 0.2690)]], "bass-hoops", 0.200, role="detail")
        trace(sheets, "front", "inventory-sticker", "bass", [(-0.06, YB - 0.05), (0.03, YB - 0.05), (0.03, YB + 0.01), (-0.06, YB + 0.01)], "bass-shell", 0.305, closed=True)
        for side, z in (("r", -0.095), ("l", 0.095)):
            for k, dx in enumerate((-0.012, 0.012)):
                trace(sheets, "plan", f"sling-tape-{side}-{k}", "snare-carrier", [(XS - 0.36 + dx, z - 0.022), (XS - 0.36 + dx, z + 0.022)], "carrier-pads", 0.812, role="detail")
    elif scale == 100:
        trace(sheets, "plan", "batter-worn", "snare", circle_pts(XS - 0.01, 0.0, 0.055, 12), "snare-heads", 0.31725, closed=True, role="detail")
        trace(sheets, "elevation", "calf-beater-spot", "bass", [fb.elev(*p) for p in circle_pts(0.02, YB + 0.03, 0.075, 12)], "bass-heads", 0.1785, closed=True, role="detail")
