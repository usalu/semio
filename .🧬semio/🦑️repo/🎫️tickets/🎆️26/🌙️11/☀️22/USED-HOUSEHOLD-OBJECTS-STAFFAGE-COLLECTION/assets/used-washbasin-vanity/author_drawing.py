"""✏️ Drawing chisel of the asset `used-washbasin-vanity`: the elevation, front and plan shapes derived from the same part lists as the solids, and the hand-drawn traces of use per scale.

@see ./author_geometry.py — the parts and numbers
@see ./author-used-washbasin-vanity.py — the writer of the document
"""
from author_geometry import (BOWL_LAT, BOWL_X, BOWL_Y, HANDLE_Y, ORIGIN, PT, TAP_X, XD1, XF1, XM0, XM1, XS0, XS1, XV1, YD1, YF0, YF1, YM1, ZF, ZM, ZS, bowl_rim, loc, r6)


def sh(id, bone, role, z, filled, tone, geom, region=None, material=None, hatch=None, pen=None):
    """✏️ One 2D shape."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    return s


def rect(a0, b0, a1, b1):
    """⬜️ A rectangle from two corners."""
    return {"kind": "rect", "x": r6(min(a0, a1)), "y": r6(min(b0, b1)), "w": r6(abs(a1 - a0)), "h": r6(abs(b1 - b0))}


def poly(pts, closed=True):
    """🔷️ A polygon or polyline."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}


def path(d):
    """〰️ A path of open strokes."""
    return {"kind": "path", "d": d}


def ellipse(cx, cy, rx, ry):
    """⚪️ An ellipse."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry)}


def capsule(a0, b0, a1, b1, r):
    """💊️ A stadium."""
    return {"kind": "capsule", "x0": r6(a0), "y0": r6(b0), "x1": r6(a1), "y1": r6(b1), "r0": r, "r1": r}


def eloc_pts(bone, pts):
    """📍️ World (x, y) points to the bone-local elevation frame."""
    return [loc(bone, x, y) for x, y in pts]


def copies(p):
    """🧬️ The translations of the copies of a part."""
    if not p["rep"]:
        return [(0.0, 0.0, 0.0)]
    n, t = p["rep"]
    return [(t[0] * k, t[1] * k, t[2] * k) for k in range(n)]


def view_geom(p, view, d):
    """📐️ The geometry of one copy (offset d) of a part in one view: elevation in bone-local (u, v), front in world (z, y), plan in world (x, z)."""
    g = p["geom"]
    bone = p["bone"]
    dx, dy, dz = d
    if g[0] == "box":
        _, x0, x1, y0, y1, z0, z1 = g
        x0, x1, y0, y1, z0, z1 = x0 + dx, x1 + dx, y0 + dy, y1 + dy, z0 + dz, z1 + dz
        if view == "elevation":
            u0, v0 = loc(bone, x0, y0)
            u1, v1 = loc(bone, x1, y1)
            return rect(u0, v0, u1, v1)
        return rect(z0, y0, z1, y1) if view == "front" else rect(x0, z0, x1, z1)
    if g[0] == "exy":
        _, pts, z0, z1, _h = g
        xs, ys = [x for x, _ in pts], [y for _, y in pts]
        if view == "elevation":
            return poly(eloc_pts(bone, pts))
        return rect(z0, min(ys), z1, max(ys)) if view == "front" else rect(min(xs), z0, max(xs), z1)
    if g[0] == "eyz":
        _, pts, x0, x1, _h = g
        ys, zs = [y for y, _ in pts], [z for _, z in pts]
        if view == "elevation":
            u0, v0 = loc(bone, x0, min(ys))
            u1, v1 = loc(bone, x1, max(ys))
            return rect(u0, v0, u1, v1)
        return poly([(z, y) for y, z in pts]) if view == "front" else rect(x0, min(zs), x1, max(zs))
    if g[0] == "ezx":
        _, pts, y0, y1, _h = g
        zs, xs = [z for z, _ in pts], [x for _, x in pts]
        if view == "elevation":
            u0, v0 = loc(bone, min(xs), y0)
            u1, v1 = loc(bone, max(xs), y1)
            return rect(u0, v0, u1, v1)
        return rect(min(zs), y0, max(zs), y1) if view == "front" else poly([(x, z) for z, x in pts])
    if g[0] == "rev":
        _, xc, y0, zc, _prof, _closed, lat, outline = g
        rmax = max(r for _, r in outline)
        if view == "plan":
            return ellipse(xc, zc, rmax * lat, rmax)
        if view == "elevation":
            pts = [(xc + r * lat, y0 + a) for a, r in outline] + [(xc - r * lat, y0 + a) for a, r in reversed(outline)]
            return poly(eloc_pts(bone, pts))
        return poly([(zc + r, y0 + a) for a, r in outline] + [(zc - r, y0 + a) for a, r in reversed(outline)])
    _, p0, p1, r = g
    if view == "elevation":
        a = loc(bone, p0[0], p0[1])
        b = loc(bone, p1[0], p1[1])
        return capsule(a[0], a[1], b[0], b[1], r)
    if view == "front":
        return capsule(p0[2], p0[1], p1[2], p1[1], r)
    return capsule(p0[0], p0[2], p1[0], p1[2], r)


def extent(p, d):
    """📦️ World bounds (x0, x1, y0, y1, z0, z1) of one copy of a part."""
    g = p["geom"]
    dx, dy, dz = d
    if g[0] == "box":
        _, x0, x1, y0, y1, z0, z1 = g
    elif g[0] == "exy":
        _, pts, z0, z1, _h = g
        x0, x1, y0, y1 = min(a for a, _ in pts), max(a for a, _ in pts), min(b for _, b in pts), max(b for _, b in pts)
    elif g[0] == "eyz":
        _, pts, x0, x1, _h = g
        y0, y1, z0, z1 = min(a for a, _ in pts), max(a for a, _ in pts), min(b for _, b in pts), max(b for _, b in pts)
    elif g[0] == "ezx":
        _, pts, y0, y1, _h = g
        z0, z1, x0, x1 = min(a for a, _ in pts), max(a for a, _ in pts), min(b for _, b in pts), max(b for _, b in pts)
    elif g[0] == "rev":
        _, xc, yb, zc, _prof, _closed, lat, outline = g
        rmax = max(r for _, r in outline)
        x0, x1, y0, y1, z0, z1 = xc - rmax * lat, xc + rmax * lat, yb, yb + max(a for a, _ in outline), zc - rmax, zc + rmax
    else:
        _, p0, p1, r = g
        x0, x1, y0, y1, z0, z1 = min(p0[0], p1[0]) - r, max(p0[0], p1[0]) + r, min(p0[1], p1[1]) - r, max(p0[1], p1[1]) + r, min(p0[2], p1[2]) - r, max(p0[2], p1[2]) + r
    return (x0 + dx, x1 + dx, y0 + dy, y1 + dy, z0 + dz, z1 + dz)


def nearness(e, view):
    """👁️ How near a part copy comes to the viewer (larger is nearer): from the right (-z) in elevation, from ahead (+x) in front, from above (+y) in plan."""
    return {"elevation": -e[4], "front": e[1], "plan": e[3]}[view]


def auto_shapes(parts, view, hatches=True):
    """🖨️ One filled body shape per part copy in a view, painted from the farthest to the nearest part (copies that coincide in the side view drawn once, as the nearest of them)."""
    items = []
    for order, p in enumerate(parts):
        if view not in p["views"]:
            continue
        cs = copies(p)
        if view == "elevation":
            groups = {}
            for c in cs:
                groups.setdefault((round(c[0], 6), round(c[1], 6)), []).append(c)
            cs = [min(g, key=lambda c: extent(p, c)[4]) for g in groups.values()]
        for k, c in enumerate(cs):
            sid = p["id"] if len(cs) == 1 else f"{p['id']}-{k + 1}"
            items.append((nearness(extent(p, c), view), order, sid, p, c))
    items.sort(key=lambda t: (round(t[0], 6), t[1]))
    return [sh(sid, p["bone"], p["role"], round(1.0 + 0.1 * i, 2), True, p["tone"], view_geom(p, view, c), region=p["region"], material=p["material"], hatch=p["hatch"] if hatches else None) for i, (_n, _o, sid, p, c) in enumerate(items)]


def details(scale):
    """🩹️ The traces of use and the secondary lines per view, hand-placed in world coordinates (elevation shapes in the frame of their bone)."""
    v, m, b, d = "vanity", "mirror", "basin", "drawer"
    el, fr, pl = [], [], []
    if scale in (50, 100):
        rim = bowl_rim(0.012 if scale == 50 else 0.015)
        pl.append(sh("b-bowl-inner", b, "detail", 40, True, 0.7, ellipse(BOWL_X, 0.0, rim, rim * BOWL_LAT), region="basin", material="vitreous-china"))
        pl.append(sh("b-waste", b, "detail", 41, True, 0.5, ellipse(BOWL_X, 0.0, 0.0225, 0.0225), region="basin", material="chrome-brass"))
        fr.append(sh("mc-door-joint", m, "detail", 42, False, 0.3, path(f"M 0 {r6(PT + 0.01)} L 0 {r6(YM1 - 0.01)}"), region="mirror-doors"))
        fr.append(sh("d-reveal", d, "detail", 43, False, 0.3, path(f"M {r6(-ZF + 0.004)} {r6(YF1 - 0.002)} L {r6(ZF - 0.004)} {r6(YF1 - 0.002)}"), region="drawer"))
        el.append(sh("b-deck-line", b, "detail", 44, False, 0.3, poly(eloc_pts(b, [(XS0 + 0.005, YD1 - 0.012), (XS1 - 0.005, YD1 - 0.012)]), closed=False), region="basin"))
        el.append(sh("mc-door-line", m, "detail", 45, False, 0.3, poly(eloc_pts(m, [(XM1, PT + 0.005), (XM1, YM1 - 0.005)]), closed=False), region="mirror-doors"))
    if scale == 50:
        el += [
            sh("v-swelling", v, "texture", 50, False, 0.3, poly(eloc_pts(v, [(XV1 - 0.15, PT + 0.004), (XV1 - 0.1, PT + 0.007), (XV1 - 0.05, PT + 0.005), (XV1 - 0.004, PT + 0.009)]), closed=False), region="vanity-carcass"),
            sh("v-edge-chip", v, "detail", 51, False, 0.25, poly(eloc_pts(v, [(XV1 - 0.012, PT + 0.002), (XV1 - 0.008, PT + 0.012), (XV1 - 0.002, PT + 0.016)]), closed=False), region="vanity-carcass"),
            sh("mc-sticker", m, "texture", 52, False, 0.3, rect(*loc(m, -0.36, PT + 0.40), *loc(m, -0.31, PT + 0.45)), region="mirror-carcass"),
            sh("mc-cable", m, "texture", 53, False, 0.3, path(" ".join([f"M {r6(0.006)} {r6(0.67)} L {r6(0.006)} {r6(0.42)}", f"M {r6(0.002)} {r6(0.60)} L {r6(0.012)} {r6(0.60)}", f"M {r6(0.002)} {r6(0.48)} L {r6(0.012)} {r6(0.48)}"])), region="mirror-carcass"),
            sh("b-limescale", b, "texture", 54, False, 0.3, poly(eloc_pts(b, [(TAP_X - 0.026, YD1 + 0.002), (TAP_X + 0.026, YD1 + 0.002)]), closed=False), region="tap"),
        ]
        fr += [
            sh("d-old-holes", d, "detail", 60, True, 0.2, {"kind": "path", "d": f"M -0.06 {r6(HANDLE_Y - 0.004)} L -0.068 {r6(HANDLE_Y - 0.004)} L -0.068 {r6(HANDLE_Y + 0.004)} L -0.06 {r6(HANDLE_Y + 0.004)} Z M 0.068 {r6(HANDLE_Y - 0.004)} L 0.06 {r6(HANDLE_Y - 0.004)} L 0.06 {r6(HANDLE_Y + 0.004)} L 0.068 {r6(HANDLE_Y + 0.004)} Z"}, region="drawer", material="mdf-gloss"),
            sh("d-scratches", d, "texture", 61, False, 0.3, path("M -0.2 0.36 L -0.12 0.33 M -0.17 0.30 L -0.06 0.27 M 0.1 0.25 L 0.19 0.23 M 0.14 0.33 L 0.23 0.31"), region="drawer"),
            sh("d-chip", d, "detail", 62, False, 0.25, poly([(-ZF + 0.002, YF0 + 0.02), (-ZF + 0.012, YF0 + 0.012), (-ZF + 0.02, YF0 + 0.002)], closed=False), region="drawer"),
            sh("d-swelling", d, "texture", 63, False, 0.3, path(f"M {r6(-ZF + 0.03)} {r6(YF0 + 0.006)} L {r6(-0.1)} {r6(YF0 + 0.009)} L {r6(0.08)} {r6(YF0 + 0.006)} L {r6(ZF - 0.03)} {r6(YF0 + 0.008)}"), region="drawer"),
            sh("b-chip", b, "detail", 64, False, 0.25, poly([(0.20, YD1 - 0.002), (0.207, YD1 - 0.012), (0.222, YD1 - 0.012), (0.228, YD1 - 0.002)], closed=False), region="basin"),
            sh("mc-desilvering", m, "texture", 65, False, 0.3, path(f"M {r6(ZM - 0.06)} {r6(PT + 0.006)} L {r6(ZM - 0.04)} {r6(PT + 0.02)} L {r6(ZM - 0.006)} {r6(PT + 0.03)} M {r6(-ZM + 0.006)} {r6(PT + 0.035)} L {r6(-ZM + 0.03)} {r6(PT + 0.018)} L {r6(-ZM + 0.07)} {r6(PT + 0.006)}"), region="mirror-doors"),
            sh("mc-limescale-spots", m, "texture", 66, False, 0.3, path(" ".join(f"M {r6(zz - 0.004)} {r6(yy)} L {r6(zz + 0.004)} {r6(yy)}" for zz, yy in ((-0.12, PT + 0.08), (-0.05, PT + 0.11), (0.04, PT + 0.07), (0.11, PT + 0.12), (0.17, PT + 0.09)))), region="mirror-doors"),
            sh("mc-door-sticker", m, "texture", 67, False, 0.3, rect(0.2, PT + 0.6, 0.25, PT + 0.65), region="mirror-doors"),
        ]
        pl += [
            sh("b-limescale-tap", b, "texture", 70, False, 0.3, ellipse(TAP_X, 0.0, 0.036, 0.036), region="basin"),
            sh("b-limescale-waste", b, "texture", 71, False, 0.3, ellipse(BOWL_X, 0.0, 0.032, 0.032 * BOWL_LAT), region="basin"),
            sh("b-soap-mark", b, "texture", 72, False, 0.3, rect(-0.13, 0.17, -0.07, 0.25), region="basin"),
            sh("b-chip", b, "detail", 73, False, 0.25, poly([(XS1 - 0.002, 0.24), (XS1 - 0.012, 0.226), (XS1 - 0.01, 0.21)], closed=False), region="basin"),
        ]
    if scale == 100:
        fr.append(sh("d-chip", d, "detail", 62, False, 0.25, poly([(-ZF + 0.003, YF0 + 0.03), (-ZF + 0.03, YF0 + 0.003)], closed=False), region="drawer"))
    if scale == 200:
        rim = bowl_rim(0.03)
        pl.append(sh("b-bowl-inner", b, "detail", 40, True, 0.7, ellipse(BOWL_X, 0.0, rim, rim * BOWL_LAT), region="basin", material="vitreous-china"))
    return el, fr, pl
