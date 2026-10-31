"""Offline approximation of the print-suitability gap rule and a quick SVG/PNG preview of rest-pose geometry (own tools, not the repo CLIs)."""
import math, subprocess
from lib import flat
from common import PENS


def _segdist(p, q, r, s):
    d1x, d1y, d2x, d2y = q[0] - p[0], q[1] - p[1], s[0] - r[0], s[1] - r[1]
    den = d1x * d2y - d1y * d2x
    if abs(den) > 1e-14:
        t = ((r[0] - p[0]) * d2y - (r[1] - p[1]) * d2x) / den
        u = ((r[0] - p[0]) * d1y - (r[1] - p[1]) * d1x) / den
        if 0 <= t <= 1 and 0 <= u <= 1:
            return 0.0
    def pd(x, a, b):
        dx, dy = b[0] - a[0], b[1] - a[1]
        l2 = dx * dx + dy * dy
        t = 0 if l2 == 0 else max(0, min(1, ((x[0] - a[0]) * dx + (x[1] - a[1]) * dy) / l2))
        return math.hypot(x[0] - a[0] - t * dx, x[1] - a[1] - t * dy)
    return min(pd(p, r, s), pd(q, r, s), pd(r, p, q), pd(s, p, q))


def _inside(p, rings):
    ins = False
    for ring in rings:
        j = len(ring) - 1
        for i in range(len(ring)):
            a, b = ring[i], ring[j]
            if (a[1] > p[1]) != (b[1] > p[1]) and p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]:
                ins = not ins
            j = i
    return ins


def gaps(shapes, scale, pens, min_gap=0.15, search=1.0):
    k = 1000.0 / scale
    items = []
    for s in shapes:
        pen = s.pen or pens[s.role]
        if pen is None or s.role == "ground":
            continue
        rings = [[(x * k, -y * k) for x, y in r] for r in flat(s.cmds, 3)]
        closed = any(c[0] == "Z" for c in s.cmds)
        edges = []
        for r in rings:
            for i in range(len(r) - 1):
                edges.append((r[i], r[i + 1]))
        xs = [p[0] for r in rings for p in r]
        ys = [p[1] for r in rings for p in r]
        items.append((s, pen, rings, closed and s.filled or closed, edges, min(xs), min(ys), max(xs), max(ys)))
    out = []
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            a, b = items[i], items[j]
            reach = search + (a[1] + b[1]) / 2
            if a[5] > b[7] + reach or b[5] > a[7] + reach or a[6] > b[8] + reach or b[6] > a[8] + reach:
                continue
            best = 1e9
            for ea in a[4]:
                for eb in b[4]:
                    d = _segdist(ea[0], ea[1], eb[0], eb[1])
                    if d < best:
                        best = d
                    if best <= 1e-4:
                        break
                if best <= 1e-4:
                    break
            if best <= 1e-4 or best > reach:
                continue
            if (a[3] and b[4] and _inside(b[4][0][0], a[2])) or (b[3] and a[4] and _inside(a[4][0][0], b[2])):
                continue
            g = max(0.0, best - (a[1] + b[1]) / 2)
            if g < min_gap:
                out.append((round(g, 3), a[0].id, b[0].id))
    return sorted(out)


def svg(shapes, scale, pens, path, ppm=8.0, box=None):
    k = 1000.0 / scale
    pts = [p for s in shapes for r in flat(s.cmds, 4) for p in r]
    x0, x1 = min(p[0] for p in pts), max(p[0] for p in pts)
    y0, y1 = min(0, min(p[1] for p in pts)), max(p[1] for p in pts)
    if box:
        x0, y0, x1, y1 = box
    m = 0.3
    W, H = (x1 - x0 + 2 * m) * k * ppm, (y1 - y0 + 2 * m) * k * ppm
    f = k * ppm
    out = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d"><rect width="100%%" height="100%%" fill="white"/>' % (W, H, W, H)]
    out.append('<line x1="0" x2="%d" y1="%f" y2="%f" stroke="#555" stroke-width="%f"/>' % (W, (y1 + m) * f, (y1 + m) * f, pens["ground"] * ppm))
    for s in sorted(shapes, key=lambda s: s.z):
        pen = s.pen or pens[s.role]
        if pen is None:
            continue
        d = ""
        for r in flat(s.cmds, 8):
            d += "M" + "L".join("%.2f %.2f" % ((x - x0 + m) * f, (y1 - y + m) * f) for x, y in r)
            if any(c[0] == "Z" for c in s.cmds):
                d += "Z"
        g = int(255 - s.tone * 130) if s.filled else 255
        fill = "rgb(%d,%d,%d)" % (g, g, g) if s.filled else "none"
        out.append('<path d="%s" fill="%s" stroke="#111" stroke-width="%.2f" stroke-linejoin="round"/>' % (d, fill, pen * ppm))
    out.append("</svg>")
    open(path + ".svg", "w").write("".join(out))
    subprocess.run(["rsvg-convert", path + ".svg", "-o", path + ".png"], check=True)


def pose_frames(S, offsets):
    """posed world frame (x, y, ang) of every bone for bone angle offsets (deg)."""
    fr = {}
    for id in S.rig.order:
        b = S.rig.bones[id]
        off = offsets.get(id, 0.0)
        if b.parent is None:
            fr[id] = (b.bx, b.by, b.ang + off)
        else:
            p = S.rig.bones[b.parent]
            lx, ly = p.to_local((b.bx, b.by))
            px, py, pa = fr[b.parent]
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            fr[id] = (px + lx * c - ly * s, py + lx * s + ly * c, pa + (b.ang - p.ang) + off)
    return fr


def posed(S, shapes, offsets):
    from lib import Shape
    fr = pose_frames(S, offsets)
    out = []
    for sh in shapes:
        b = sh.bone
        x, y, a = fr[b.id]
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        def tf(p):
            lx, ly = b.to_local(p)
            return (x + lx * c - ly * s, y + lx * s + ly * c)
        cmds = [(cm[0],) + tuple(tf(p) for p in cm[1:]) for cm in sh.cmds]
        n = Shape(sh.id, sh.bone, sh.role, cmds, sh.filled, sh.tone, sh.z, sh.region, sh.regions, sh.material, sh.hatch, sh.pen)
        out.append(n)
    return out


def pose_gaps(S, shapes, scale):
    from common import lean_deg
    res = {}
    for p in S.poses:
        off = {id: lean_deg(S, id, p.get("v10", 0.0), p.get("droop", 0.0)) for id in S.rig.order[1:]}
        for g in gaps(posed(S, shapes, off), int(scale), PENS[scale]):
            k = (g[1], g[2])
            if k not in res or g[0] < res[k][0]:
                res[k] = (g[0], p["id"])
    return sorted((v[0], k[0], k[1], v[1]) for k, v in res.items())
