"""Author 43: two-wheeler builders on the author-19 solid kit. Everything is modelled in WORLD coordinates of the default pose and converted to the bone-local frame of the dump (u, v, w = x, y, z of the 2D shapes)."""
import os, math, json, re, subprocess
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())

CATV = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/"


def asset_of(slug):
    return json.load(open(CATV + slug + "/🔣️.json"))["asset"]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def sub3(a, b):
    return [a[i] - b[i] for i in range(3)]


def add3(a, b):
    return [a[i] + b[i] for i in range(3)]


def mul3(a, k):
    return [x * k for x in a]


def norm3(a):
    n = math.sqrt(dot(a, a))
    return [x / n for x in a]


def cross3(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def basis_quat(ex, ey, ez):
    m = [[ex[0], ey[0], ez[0]], [ex[1], ey[1], ez[1]], [ex[2], ey[2], ez[2]]]
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [r6(c) for c in q]


class Pose:
    def __init__(self, slug):
        d = json.load(open(os.path.join(T, "🗑️generated", "author-43", slug + ".dump.json")))
        self.f = d["frames"]
        self.slug = slug

    def loc(self, bone, p):
        f = self.f[bone]
        q = sub3(p, f["o"])
        return [dot(q, f["u"]), dot(q, f["v"]), dot(q, f["w"])]

    def dloc(self, bone, d):
        f = self.f[bone]
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]

    def wld(self, bone, p):
        f = self.f[bone]
        return [f["o"][i] + p[0] * f["u"][i] + p[1] * f["v"][i] + p[2] * f["w"][i] for i in range(3)]


def bend_path(pts, rad, n=6):
    """Polyline with rounded corners: every inner vertex is replaced by an arc-like quadratic fillet of at most `rad` (tangent length) sampled with n points."""
    out = [pts[0]]
    for i in range(1, len(pts) - 1):
        p0, p1, p2 = pts[i - 1], pts[i], pts[i + 1]
        a = sub3(p0, p1)
        b = sub3(p2, p1)
        la, lb = math.sqrt(dot(a, a)), math.sqrt(dot(b, b))
        t = min(rad, la * 0.45, lb * 0.45)
        s = add3(p1, mul3(a, t / la))
        e = add3(p1, mul3(b, t / lb))
        for k in range(n + 1):
            u = k / n
            out.append([(1 - u) ** 2 * s[j] + 2 * u * (1 - u) * p1[j] + u * u * e[j] for j in range(3)])
    out.append(pts[-1])
    res = [out[0]]
    for p in out[1:]:
        if math.sqrt(dot(sub3(p, res[-1]), sub3(p, res[-1]))) > 1e-5:
            res.append(p)
    return res


def resample(pts, step):
    out = [pts[0]]
    for a, b in zip(pts, pts[1:]):
        L = math.sqrt(dot(sub3(b, a), sub3(b, a)))
        n = max(1, int(round(L / step)))
        for k in range(1, n + 1):
            out.append([a[j] + (b[j] - a[j]) * k / n for j in range(3)])
    return out


class Bike(Scale):
    def __init__(self, den, slug):
        super().__init__(den)
        self.P = Pose(slug)
        self.slug = slug

    def cap(self, id, bone, mat, tone, a, b, r, r1=None, role="body", **kw):
        s = capsule(id, bone, mat, tone, self.P.loc(bone, a), self.P.loc(bone, b), r, r1, role=role)
        s.update(kw)
        return s

    def tube(self, id, bone, mat, tone, a, b, ro, ri=None, role="body", lip=None, **kw):
        """Straight tube: revolve about the axis a to b; hollow (wall ro - ri) when ri is given; closed round ends."""
        la, lb = self.P.loc(bone, a), self.P.loc(bone, b)
        d = sub3(lb, la)
        L = math.sqrt(dot(d, d))
        ax = [x / L for x in d]
        if ri:
            prof = [[0, ri], [L, ri], [L, ro], [0, ro], [0, ri]]
        else:
            prof = [[0, 0], [0, ro], [L, ro], [L, 0], [0, 0]]
        s = revolve(id, bone, mat, tone, ax, la, prof, role=role)
        s.update(kw)
        return s

    def sweep(self, id, bone, mat, tone, pts, r, ri=None, rx=None, ry=None, e=2, lat=(0, 0, 1), role="body", caps=True, step=0.02, local=False, **kw):
        """Loft along a world path with constant (or per-point) round or oval sections, oriented explicitly (x = lateral w, y = in-plane normal)."""
        pts = resample(pts, step) if step else pts
        loc = [list(p) if local else self.P.loc(bone, p) for p in pts]
        n = len(loc)
        secs = []
        for i, c in enumerate(loc):
            t = norm3(sub3(loc[min(n - 1, i + 1)], loc[max(0, i - 1)]))
            ex = norm3(sub3(list(lat), mul3(t, dot(list(lat), t))))
            ey = cross3(t, ex)
            q = basis_quat(ex, ey, t)
            a = rx(i / (n - 1)) if callable(rx) else (rx if rx is not None else r)
            b = ry(i / (n - 1)) if callable(ry) else (ry if ry is not None else r)
            secs.append(sec(c, a, b, exponent=e if e != 2 else None, rotation=q))
        s = loft(id, bone, mat, tone, secs, role=role, capStart=caps, capEnd=caps)
        s.update(kw)
        return s


def fillet(pts, Rb, step=0.012):
    """Planar polyline (x, y, z const) with circular bends of radius Rb: returns (samples, parts) where parts are ('line', p0, p1) or ('arc', centre, a0, a1, Rb) with a0 to a1 counter-clockwise (a1 < a0 for clockwise)."""
    segs = []
    n = len(pts)
    cuts = []
    for i in range(1, n - 1):
        p0, p1, p2 = pts[i - 1], pts[i], pts[i + 1]
        a = norm3(sub3(p0, p1))
        b = norm3(sub3(p2, p1))
        phi = math.acos(max(-1, min(1, dot(a, b))))
        t = Rb / math.tan(phi / 2)
        la = math.sqrt(dot(sub3(p0, p1), sub3(p0, p1)))
        lb = math.sqrt(dot(sub3(p2, p1), sub3(p2, p1)))
        rb = Rb
        if t > 0.48 * min(la, lb):
            t = 0.48 * min(la, lb)
            rb = t * math.tan(phi / 2)
        s = add3(p1, mul3(a, t))
        e = add3(p1, mul3(b, t))
        bis = norm3(add3(a, b))
        c = add3(p1, mul3(bis, rb / math.sin(phi / 2)))
        cuts.append((s, e, c, rb))
    parts = []
    cur = pts[0]
    for i, (s, e, c, rb) in enumerate(cuts):
        parts.append(("line", cur, s))
        a0 = math.atan2(s[1] - c[1], s[0] - c[0])
        a1 = math.atan2(e[1] - c[1], e[0] - c[0])
        d = a1 - a0
        while d > math.pi:
            d -= 2 * math.pi
        while d < -math.pi:
            d += 2 * math.pi
        parts.append(("arc", c, a0, a0 + d, rb))
        cur = e
    parts.append(("line", cur, pts[-1]))
    out = [parts[0][1]]
    for p in parts:
        if p[0] == "line":
            out += resample([p[1], p[2]], step)[1:]
        else:
            _, c, a0, a1, rb = p
            k = max(3, int(abs(a1 - a0) * rb / step))
            for j in range(1, k + 1):
                a = a0 + (a1 - a0) * j / k
                out.append([c[0] + rb * math.cos(a), c[1] + rb * math.sin(a), c[2]])
    return out, parts


def offset_poly(poly, d):
    """Inward offset (d > 0) of a simple counter-clockwise or clockwise polygon by intersecting shifted edges; adequate for the convex-ish outlines used here."""
    n = len(poly)
    area = sum(poly[i][0] * poly[(i + 1) % n][1] - poly[(i + 1) % n][0] * poly[i][1] for i in range(n)) / 2
    sgn = 1 if area > 0 else -1
    lines = []
    for i in range(n):
        a, b = poly[i], poly[(i + 1) % n]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy)
        if L < 1e-9:
            continue
        nx, ny = -dy / L * sgn, dx / L * sgn
        lines.append(((a[0] + nx * d, a[1] + ny * d), (dx / L, dy / L)))
    out = []
    m = len(lines)
    for i in range(m):
        (p, u), (q, v) = lines[i - 1], lines[i]
        den = u[0] * v[1] - u[1] * v[0]
        if abs(den) < 1e-9:
            out.append([q[0], q[1]])
            continue
        t = ((q[0] - p[0]) * v[1] - (q[1] - p[1]) * v[0]) / den
        out.append([p[0] + u[0] * t, p[1] + u[1] * t])
    return out


def superpoly(cz, cr, hz, hr, e=3.0, n=28):
    """Closed profile (axial, radial) of a superellipse section: half width hz along the axis, half height hr radially."""
    pts = []
    for k in range(n):
        a = 2 * math.pi * k / n
        c, s = math.cos(a), math.sin(a)
        pts.append([cz + hz * math.copysign(abs(c) ** (2 / e), c), cr + hr * math.copysign(abs(s) ** (2 / e), s)])
    pts.append(pts[0])
    return pts


def frame_angle(P, bone):
    f = P.f[bone]
    return math.atan2(f["u"][1], f["u"][0])


def _bend(self, id, bone, mat, tone, centre, Rb, a0, a1, ro, ri, n=10, z=0.0, role="body"):
    """Hollow (ri given) or solid circular tube bend about the world z axis through `centre`, from world angle a0 to a1 (counter-clockwise when a1 > a0)."""
    out = []
    lo, hi = min(a0, a1), max(a0, a1)
    halves = (("a", 1), ("b", -1)) if ri else (("a", 1), ("b", -1))
    for name, sgn in halves:
        pts = [[sgn * ro * math.sin(math.pi * i / n), Rb + ro * math.cos(math.pi * i / n)] for i in range(n + 1)]
        if ri:
            pts += [[sgn * ri * math.sin(math.pi * (n - i) / n), Rb + ri * math.cos(math.pi * (n - i) / n)] for i in range(n + 1)]
        pts.append(pts[0])
        d = revolve(f"{id}-{name}", bone, mat, tone, "z", [0, 0, 0], pts, sweep=hi - lo, role=role)
        c = self.P.loc(bone, [centre[0], centre[1], z])
        rot = math.degrees(lo - frame_angle(self.P, bone))
        d["transform"] = {"translation": vec(c), "rotation": qaxis([0, 0, 1], rot)}
        out.append(d)
    return out


Bike.bend = _bend


def _tube_path(self, id, bone, mat, tone, pts, Rb, ro, ri=None, role="body", z=0.0):
    """Hollow tube along a planar polyline with circular bends: straight hollow revolves and half-ring bends (all hollow solids of the member)."""
    _, parts = fillet([[p[0], p[1], z] for p in pts], Rb)
    out = []
    for k, p in enumerate(parts):
        if p[0] == "line":
            if math.sqrt(dot(sub3(p[2], p[1]), sub3(p[2], p[1]))) > 1e-4:
                out.append(self.tube(f"{id}-s{k // 2 + 1}", bone, mat, tone, p[1], p[2], ro, ri, role=role))
        else:
            _, c, a0, a1, rb = p
            out += self.bend(f"{id}-b{k // 2 + 1}", bone, mat, tone, c, rb, a0, a1, ro, ri, z=z, role=role)
    return out


Bike.tube_path = _tube_path


def _wheel_rings(self, bone, tag, ids):
    pass


def _fender(self, id, bone, mat, tone, hub, r_edge, H, zw, wall, a0, a1, n=12, role="body"):
    """Mudguard: shallow arch section revolved about the wheel axis through world `hub` from world angle a0 to a1 (degrees)."""
    outer = []
    for k in range(n + 1):
        z = -zw + 2 * zw * k / n
        outer.append([z, r_edge + H * (1 - (z / zw) ** 2)])
    inner = [[z, r - wall] for z, r in reversed(outer)]
    prof = outer + inner + [outer[0]]
    d = revolve(id, bone, mat, tone, "z", [0, 0, 0], prof, sweep=math.radians(a1 - a0), role=role)
    c = self.P.loc(bone, [hub[0], hub[1], 0.0])
    rot = a0 - math.degrees(frame_angle(self.P, bone))
    d["transform"] = {"translation": vec(c), "rotation": qaxis([0, 0, 1], rot)}
    return d


Bike.fender = _fender


def wheel_solids(B, bone, tag, hub_r, tyre_out, tyre_in, rim_in, tyre_hw, rim_hw, hub_hw, flange_z=0.03, flange_r=0.032, spokes=True, spoke_r=0.0015, axle_hw=0.05, axle_r=0.0095, nut_r=0.016, mat_tyre="rubber", mat_rim="alloy", mat_hub="chrome"):
    """Tyre and rim as revolves about the hub axis (z), drum hub with flanges and axle, and 36 tangent-laced slender spokes (four groups of nine)."""
    out = {}
    rc = (tyre_out + tyre_in) / 2
    out["tyre"] = revolve(f"tyre-{tag}", bone, mat_tyre, 0.1, "z", [0, 0, 0], superpoly(0, rc, tyre_hw, (tyre_out - tyre_in) / 2, 3.0, 32))
    rr = (tyre_in + rim_in) / 2
    out["rim"] = revolve(f"rim-{tag}", bone, mat_rim, 0.78, "z", [0, 0, 0], superpoly(0, rr, rim_hw, (tyre_in - rim_in) / 2, 6.0, 28), role="body")
    h = hub_hw
    prof = [[-h, 0], [-h, hub_r * 0.9], [-h + 0.004, hub_r], [h - 0.004, hub_r], [h, hub_r * 0.9], [h, 0], [-h, 0]]
    out["hub"] = revolve(f"hub-{tag}", bone, mat_hub, 0.7, "z", [0, 0, 0], prof)
    ax = [[-axle_hw, 0], [-axle_hw, nut_r], [-axle_hw + 0.012, nut_r], [-axle_hw + 0.012, axle_r], [axle_hw - 0.012, axle_r], [axle_hw - 0.012, nut_r], [axle_hw, nut_r], [axle_hw, 0], [-axle_hw, 0]]
    out["axle"] = revolve(f"axle-{tag}", bone, mat_hub, 0.5, "z", [0, 0, 0], ax)
    fl = []
    for sgn, nm in ((1, "a"), (-1, "b")):
        fp = [[sgn * flange_z - 0.004, 0], [sgn * flange_z - 0.004, flange_r], [sgn * flange_z + 0.004, flange_r], [sgn * flange_z + 0.004, 0], [sgn * flange_z - 0.004, 0]]
        fl.append(revolve(f"flange-{tag}-{nm}", bone, mat_hub, 0.6, "z", [0, 0, 0], fp, role="detail"))
    out["flanges"] = fl
    sp = []
    if spokes:
        R = rim_in + 0.003
        for gi, (zf, lead) in enumerate(((flange_z, 1), (flange_z, -1), (-flange_z, 1), (-flange_z, -1))):
            a_h = math.radians(10 * gi)
            a_r = a_h + lead * math.radians(62)
            p0 = [flange_r * math.cos(a_h), flange_r * math.sin(a_h), zf]
            p1 = [R * math.cos(a_r), R * math.sin(a_r), 0.0]
            s = capsule(f"spoke-{tag}-{gi + 1}", bone, mat_rim, 0.6, p0, p1, spoke_r, spoke_r, role="texture", repeat={"count": 9, "rotationAxis": [0, 0, 1], "rotationRad": r6(2 * math.pi / 9), "origin": [0, 0, 0]})
            s["slender"] = True
            s["texture"] = True
            sp.append(s)
    out["spokes"] = sp
    return out


def write_scale(slug, den, B, category="vehicles"):
    path = os.path.join(T, "volumes", category, slug + ".json")
    cur = json.load(open(path))
    cur["volumes"][str(den)] = B.out()
    json.dump(cur, open(path, "w"), ensure_ascii=False, indent=1)
    return {k: len(v) for k, v in (("body", B.body), ("thermal", B.thermal), ("structure", B.structure))}


def run_check(slug, den, rules=None, extra=()):
    bun = subprocess.check_output("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1", shell=True, text=True).strip()
    cmd = [bun, os.path.join(T, "volumes-check.ts"), "vehicles/" + slug, "--scale", str(den), "--product"] + (["--rules", rules] if rules else []) + list(extra)
    r = subprocess.run(cmd, cwd=T, capture_output=True, text=True)
    return r.stdout + r.stderr


def inset(s, t, id):
    """Nested layer shell: the solid inset by t (first order for lofts and revolves; None when it vanishes)."""
    k = s["kind"]
    d = json.loads(json.dumps(s))
    d["id"] = id
    for key in ("sectionFaces",):
        d.pop(key, None)
    if k == "capsule":
        if min(d["r0"], d["r1"]) - t <= 0.0002:
            return None
        d["r0"] = r6(d["r0"] - t)
        d["r1"] = r6(d["r1"] - t)
    elif k == "ellipsoid":
        d["radii"] = [r6(max(r - t, 0.0003)) for r in d["radii"]]
    elif k == "box":
        d["size"] = [r6(max(x - 2 * t, 0.0006)) for x in d["size"]]
        if d.get("cornerRadius"):
            d["cornerRadius"] = r6(max(d["cornerRadius"] - t, 0))
    elif k == "loft":
        for q in d["sections"]:
            q["rx"] = r6(max(q["rx"] - t, 0.0003))
            q["ry"] = r6(max(q["ry"] - t, 0.0003))
    elif k == "extrude":
        d["profile"] = [[r6(x), r6(y)] for x, y in offset_vertices(d["profile"], t)]
        if d.get("holes"):
            d["holes"] = [[[r6(x), r6(y)] for x, y in offset_vertices(h, -t)] for h in d["holes"]]
        d["from"] = r6(d["from"] + t)
        d["to"] = r6(d["to"] - t)
    elif k == "revolve":
        d["profile"] = [[r6(x), r6(y)] for x, y in offset_vertices(d["profile"], t)]
        d["profile"].append(list(d["profile"][0]))
    return d


def thermal_from(B, reg, layered, layers, skip=()):
    """Thermal list: the outer layer of every region solid is a reference to the body solid; regions with several layers get nested shells inset by the cumulative outer layer thicknesses (layers: region -> [(layer id, thickness)] innermost first)."""
    by = {s["id"]: s for s in B.body}
    for region, ids in reg.items():
        L = layers.get(region, [])
        for i in ids:
            if i in skip:
                continue
            outer = L[-1][0] if len(L) > 1 else None
            B.thof(f"th-{i}", i, region, outer)
            if len(L) > 1:
                cum = 0.0
                for (lid, th) in reversed(L[:-1]):
                    cum += L[-1][1] if lid == L[-2][0] else 0
                t = L[-1][1]
                sh = inset(by[i], t, f"th-{i}-{L[-2][0]}")
                if sh is not None and not by[i].get("slender"):
                    sh["region"] = region
                    sh["layer"] = L[-2][0]
                    B.thermal.append(sh)


def offset_vertices(poly, d):
    """Inward offset by moving every vertex along the mean of its two adjacent inward edge normals (stable for concave outlines at small d)."""
    pts = poly[:-1] if poly[0] == poly[-1] else poly
    n = len(pts)
    area = sum(pts[i][0] * pts[(i + 1) % n][1] - pts[(i + 1) % n][0] * pts[i][1] for i in range(n)) / 2
    sgn = 1 if area > 0 else -1
    out = []
    for i in range(n):
        a, b, c = pts[i - 1], pts[i], pts[(i + 1) % n]
        nx = ny = 0.0
        for p, q in ((a, b), (b, c)):
            dx, dy = q[0] - p[0], q[1] - p[1]
            L = math.hypot(dx, dy) or 1.0
            nx += -dy / L * sgn
            ny += dx / L * sgn
        L = math.hypot(nx, ny) or 1.0
        out.append([b[0] + nx / L * d, b[1] + ny / L * d])
    return out
