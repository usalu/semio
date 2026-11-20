"""🚜 Shared reference numbers, rig frames, mast and pallet kinematics and the part collector of the round 3 forklift (Jungheinrich EFG 425, Duplex ZT 3100): one hand-stated part in world rest coordinates gives its solid and its true orthographic 2D shapes."""
import math

FACE = 0.425
REAR = -2.016
WB = 1.72
RF, TF, ZF = 0.29575, 0.225, 0.4875
RR, TR, ZR = 0.2276, 0.18, 0.47
PIVOT = (0.16, 0.30)
MAST_TOP_OUTER = 2.165
MAST_TOP_INNER = 2.225
GUARD_TOP = 2.24
TILT_BASE = (-0.22, 0.76)
TILT_PIN = (0.18, 0.95)
TILT_Z = 0.42
TILT_BARREL = 0.27
TILT_ROD = 0.30
FORK_TOP = 0.06
FORK_T = 0.04
FORK_W = 0.12
FORK_L = 1.15
FORK_Z = 0.18625
PALLET_X = FACE + 0.005
STRINGER_LOW = 0.100
INNER_MAX = 1.475
CARRIAGE_MAX = 1.625
FREE_LIFT = 0.15


def r5(value):
    rounded = round(value, 5)
    return 0 if rounded == 0 else (int(rounded) if rounded == int(rounded) else rounded)


def text(en, de):
    return {"en": en, "de": de}


def rot(x, y, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return x * c - y * s, x * s + y * c


REST = {
    "chassis": (REAR, 0.0, 0.0),
    "battery": (-0.7155, 0.15, 0.0),
    "counterweight": (REAR, 0.2, 0.0),
    "axle-f": (0.0, RF, 0.0),
    "axle-r": (-WB, RR, 0.0),
    "wheel-f": (0.0, RF, 0.0),
    "wheel-r": (-WB, RR, 0.0),
    "tilt-cyl": (TILT_BASE[0], TILT_BASE[1], math.degrees(math.atan2(TILT_PIN[1] - TILT_BASE[1], TILT_PIN[0] - TILT_BASE[0]))),
    "mast-outer": (PIVOT[0], PIVOT[1], 90.0),
    "mast-inner": (0.195, 0.16, 90.0),
    "carriage": (0.355, 0.09, 90.0),
    "forks": (FACE, FORK_TOP, 0.0),
    "pallet-x": (PALLET_X, 0.0, 0.0),
    "pallet-y": (PALLET_X, 0.0, 0.0),
    "pallet": (PALLET_X, 0.0, 0.0),
}
TILT_D0 = math.hypot(TILT_PIN[0] - TILT_BASE[0], TILT_PIN[1] - TILT_BASE[1])
REST["tilt-rod"] = (TILT_BASE[0] + (TILT_D0 - TILT_ROD) * math.cos(math.radians(REST["tilt-cyl"][2])), TILT_BASE[1] + (TILT_D0 - TILT_ROD) * math.sin(math.radians(REST["tilt-cyl"][2])), REST["tilt-cyl"][2])


def to_local(bone, x, y):
    ox, oy, angle = REST[bone]
    return rot(x - ox, y - oy, -angle)


def to_world(bone, u, v):
    ox, oy, angle = REST[bone]
    dx, dy = rot(u, v, angle)
    return ox + dx, oy + dy


def dir_local(bone, dx, dy):
    return rot(dx, dy, -REST[bone][2])


def fork_frame(tilt, inner, carriage):
    """🍴 World origin and angle of the fork frame (heel at the fork face, blade top) for a mast tilt (degrees back), inner stage and carriage travel."""
    fx, fy = FACE, FORK_TOP + inner + carriage
    px, py = rot(fx - PIVOT[0], fy - PIVOT[1], tilt)
    return PIVOT[0] + px, PIVOT[1] + py, tilt


def tilt_offsets(tilt):
    """🛢️ Tilt cylinder angle offset and rod travel that keep the rod eye on the mast bracket at a mast tilt."""
    px, py = rot(TILT_PIN[0] - PIVOT[0], TILT_PIN[1] - PIVOT[1], tilt)
    mx, my = PIVOT[0] + px, PIVOT[1] + py
    angle = math.degrees(math.atan2(my - TILT_BASE[1], mx - TILT_BASE[0]))
    return angle - REST["tilt-cyl"][2], math.hypot(mx - TILT_BASE[0], my - TILT_BASE[1]) - TILT_D0


def lift_split(rise):
    """🏗️ Duplex ZT: the carriage rises first by the free lift, then inner stage and carriage share the rest 1 : 1 (chains 2 : 1)."""
    if rise <= FREE_LIFT:
        return 0.0, rise
    inner = (rise - FREE_LIFT) / 2
    return inner, FREE_LIFT + inner


def offsets(tilt=0.0, rise=0.0, carried=None, root_x=0.0, pallet_world_dx=0.0):
    """📐 Pose offsets: mast tilt (degrees back), fork rise (m), pallet carried on the forks or standing on the ground (moved by pallet_world_dx relative to the truck), wheel roll from the root travel."""
    inner, carriage = lift_split(rise)
    bones = {}
    if tilt:
        bones["mast-outer"] = {"angleDeg": r5(tilt)}
    a, d = tilt_offsets(tilt)
    if abs(a) > 1e-6 or abs(d) > 1e-6:
        bones["tilt-cyl"] = {"angleDeg": r5(a)}
        bones["tilt-rod"] = {"dx": r5(d)}
    if inner:
        bones["mast-inner"] = {"dx": r5(inner)}
    if carriage:
        bones["carriage"] = {"dx": r5(carriage)}
    fx, fy, fa = fork_frame(tilt, inner, carriage)
    if carried is None:
        carried = fy + 0.0 > STRINGER_LOW + 1e-9 or tilt != 0
    if carried:
        ox, oy = rot(0.005, -STRINGER_LOW, fa)
        bones["pallet-x"] = {"dx": r5(fx + ox - PALLET_X)}
        bones["pallet-y"] = {"dy": r5(fy + oy)}
        if fa:
            bones["pallet"] = {"angleDeg": r5(fa)}
    elif pallet_world_dx:
        bones["pallet-x"] = {"dx": r5(pallet_world_dx)}
    if root_x:
        bones["wheel-f"] = {"angleDeg": r5(-math.degrees(root_x / RF))}
        bones["wheel-r"] = {"angleDeg": r5(-math.degrees(root_x / RR))}
    return {key: bones[key] for key in sorted(bones, key=list(REST).index)}


def ngon(cx, cy, r, n=24):
    return [[r5(cx + r * math.cos(2 * math.pi * i / n)), r5(cy + r * math.sin(2 * math.pi * i / n))] for i in range(n)]


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def rounded_rect(x0, x1, y0, y1, r, n=4):
    if r <= 0:
        return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
    return arc(x1 - r, y0 + r, r, -90, 0, n) + arc(x1 - r, y1 - r, r, 0, 90, n) + arc(x0 + r, y1 - r, r, 90, 180, n) + arc(x0 + r, y0 + r, r, 180, 270, n)


def superellipse(cz, cy, rz, ry, e, n=32):
    points = []
    for i in range(n):
        t = 2 * math.pi * i / n
        c, s = math.cos(t), math.sin(t)
        points.append((cz + rz * math.copysign(abs(c) ** (2 / e), c), cy + ry * math.copysign(abs(s) ** (2 / e), s)))
    return points


def hull(points):
    pts = sorted(set((round(x, 5), round(y, 5)) for x, y in points))
    if len(pts) < 3:
        return pts

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def clean(points):
    out = []
    for p in points:
        q = [r5(p[0]), r5(p[1])]
        if not out or q != out[-1]:
            out.append(q)
    if len(out) > 1 and out[0] == out[-1]:
        out.pop()
    return out


class Scale:
    """🧱 Part collector of one authored scale: body solids with their regions, the 2D shapes of the three viewpoints, and the own solids of the thermal and structure lists. Every coordinate handed in is a world rest coordinate (x ahead of the front axle, y up from the ground, z to the left)."""

    def __init__(self, denominator):
        self.denominator = denominator
        self.body = []
        self.regions = {}
        self.prims = []
        self.strokes = []
        self.thermal_own = []
        self.structure = []

    def _base(self, id, bone, material, tone, role):
        return {"id": id, "bone": bone, "material": material, "role": role, "tone": tone}

    def _flags(self, solid, flags):
        for key in ("slender", "texture", "envelope", "sectionFaces", "tags"):
            if flags.get(key):
                solid[key] = flags[key]

    def _keep(self, solid, region, target, extra):
        if target == "body":
            self.body.append(solid)
            if region:
                self.regions[solid["id"]] = region
        elif target == "thermal":
            self.thermal_own.append({**solid, "region": region, **extra})
        else:
            self.structure.append({**solid, **extra})

    def _draw(self, id, bone, local, z0, z1, material, tone, role, region, views, mirror, circle=None, front=None, plan=None, pen=None, filled=True):
        if views:
            self.prims.append({"id": id, "bone": bone, "profile": clean(local), "z0": z0, "z1": z1, "material": material, "tone": tone, "role": role, "region": region, "views": views, "mirror": mirror, "circle": circle, "front": front, "plan": plan, "pen": pen, "filled": filled})

    def ext(self, id, bone, profile, z, material, tone, region=None, role="body", holes=None, views="efp", mirror=False, target="body", front=None, plan=None, pen=None, draw=None, flags=None, **extra):
        """🔷 A side profile (world x, y) extruded across z = (min, max); mirror repeats it at -z."""
        local = [to_local(bone, x, y) for x, y in profile]
        solid = {**self._base(id, bone, material, tone, role), "kind": "extrude", "profile": clean(local)}
        if holes:
            solid["holes"] = [clean([to_local(bone, x, y) for x, y in hole]) for hole in holes]
        solid.update({"plane": "xy", "from": r5(z[0]), "to": r5(z[1])})
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-(z[0] + z[1]))]}
        self._flags(solid, flags or {})
        self._keep(solid, region, target, extra)
        if target == "body":
            outline = [to_local(bone, x, y) for x, y in (draw or profile)]
            self._draw(id, bone, outline, z[0], z[1], material, tone, role, region, views, mirror, front=front, plan=plan, pen=pen)
        return solid

    def box(self, id, bone, x, y, z, material, tone, region=None, role="body", radius=0, views="efp", mirror=False, target="body", front=None, plan=None, pen=None, flags=None, **extra):
        """📦 Axis-aligned box in world rest coordinates (bone at 0 or 90 degrees), x, y, z = (min, max); radius rounds every edge."""
        corners = [to_local(bone, cx, cy) for cx in x for cy in y]
        us, vs = [c[0] for c in corners], [c[1] for c in corners]
        solid = {**self._base(id, bone, material, tone, role), "kind": "box", "size": [r5(max(us) - min(us)), r5(max(vs) - min(vs)), r5(z[1] - z[0])]}
        if radius:
            solid["cornerRadius"] = r5(radius)
        solid["transform"] = {"translation": [r5((max(us) + min(us)) / 2), r5((max(vs) + min(vs)) / 2), r5((z[0] + z[1]) / 2)]}
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-(z[0] + z[1]))]}
        self._flags(solid, flags or {})
        self._keep(solid, region, target, extra)
        if target == "body":
            outline = rounded_rect(x[0], x[1], y[0], y[1], radius, 3) if radius else [(x[0], y[0]), (x[1], y[0]), (x[1], y[1]), (x[0], y[1])]
            self._draw(id, bone, [to_local(bone, px, py) for px, py in outline], z[0], z[1], material, tone, role, region, views, mirror, front=front, plan=plan, pen=pen)
        return solid

    def rev(self, id, bone, origin, axis, profile, material, tone, region=None, role="body", views="efp", mirror=False, target="body", front=None, plan=None, pen=None, sweep=None, flags=None, **extra):
        """🥁 A profile of (axial, radial) revolved about the axis through the world point origin (x, y, z) along the world direction axis (dx, dy, dz)."""
        length = math.sqrt(sum(c * c for c in axis))
        ax = [c / length for c in axis]
        lu, lv = dir_local(bone, ax[0], ax[1])
        ou, ov = to_local(bone, origin[0], origin[1])
        points = clean(profile)
        if len(profile) > 2 and [r5(profile[0][0]), r5(profile[0][1])] == [r5(profile[-1][0]), r5(profile[-1][1])]:
            points.append(points[0])
        solid = {**self._base(id, bone, material, tone, role), "kind": "revolve", "profile": points, "axis": [r5(lu), r5(lv), r5(ax[2])], "origin": [r5(ou), r5(ov), r5(origin[2])]}
        if sweep:
            solid["sweepRad"] = r5(sweep)
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-2 * origin[2])]}
        self._flags(solid, flags or {})
        self._keep(solid, region, target, extra)
        if target == "body":
            radius = max(r for _, r in profile)
            reach = [a for a, _ in profile]
            if abs(ax[2]) > 0.999:
                z0, z1 = sorted((origin[2] + ax[2] * min(reach), origin[2] + ax[2] * max(reach)))
                circle = (r5(ou), r5(ov), r5(radius))
                self._draw(id, bone, ngon(ou, ov, radius), z0, z1, material, tone, role, region, views, mirror, circle=circle, front=front, plan=plan, pen=pen)
            else:
                px, py = -ax[1], ax[0]
                norm = math.hypot(px, py) or 1.0
                px, py = px / norm, py / norm
                side = [(origin[0] + ax[0] * a + px * r, origin[1] + ax[1] * a + py * r) for a, r in profile]
                other = [(origin[0] + ax[0] * a - px * r, origin[1] + ax[1] * a - py * r) for a, r in reversed(profile)]
                outline = hull(side + other) if len(profile) > 2 else side + other
                lateral = math.hypot(ax[0], ax[1])
                zr = radius * lateral + abs(ax[2]) * max(abs(a) for a in reach)
                self._draw(id, bone, [to_local(bone, x, y) for x, y in outline], origin[2] - zr, origin[2] + zr, material, tone, role, region, views, mirror, front=front, plan=plan, pen=pen)
        return solid

    def cyl(self, id, bone, p0, p1, r, material, tone, region=None, role="body", bore=0, **kw):
        """🧪 A cylinder (pipe when bore > 0) from world point p0 to p1."""
        d = [b - a for a, b in zip(p0, p1)]
        length = math.sqrt(sum(c * c for c in d))
        profile = [[0, bore], [0, r], [length, r], [length, bore]] if bore else [[0, r], [length, r]]
        return self.rev(id, bone, p0, d, profile, material, tone, region=region, role=role, **kw)

    def loft(self, id, bone, stations, material, tone, region=None, role="body", views="efp", mirror=False, target="body", caps=True, pen=None, flags=None, **extra):
        """🎢 A loft along world x through stations (x, yc, zc, rz, ry, e): superelliptic sections, rz lateral and ry vertical half extents (bone at 0 degrees)."""
        sections = []
        for x, yc, zc, rz, ry, e in stations:
            u, v = to_local(bone, x, yc)
            sections.append({"at": [r5(u), r5(v), r5(zc)], "rx": r5(rz), "ry": r5(ry), "exponent": r5(e)})
        solid = {**self._base(id, bone, material, tone, role), "kind": "loft", "sections": sections, "capStart": caps, "capEnd": caps}
        if mirror:
            zc = stations[0][2]
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-2 * zc)]}
        self._flags(solid, flags or {})
        self._keep(solid, region, target, extra)
        if target == "body":
            top = [(x, yc + ry) for x, yc, _, _, ry, _ in stations]
            bottom = [(x, yc - ry) for x, yc, _, _, ry, _ in reversed(stations)]
            front = hull([p for x, yc, zc, rz, ry, e in stations for p in superellipse(zc, yc, rz, ry, e)])
            left = [(x, zc + rz) for x, _, zc, rz, _, _ in stations]
            right = [(x, zc - rz) for x, _, zc, rz, _, _ in reversed(stations)]
            z0 = min(zc - rz for _, _, zc, rz, _, _ in stations)
            z1 = max(zc + rz for _, _, zc, rz, _, _ in stations)
            mirrored = None
            if mirror:
                mirrored = True
            self._draw(id, bone, [to_local(bone, x, y) for x, y in top + bottom], z0, z1, material, tone, role, region, views, mirrored, front=front, plan=left + right, pen=pen)
        return solid

    def upright(self, id, bone, plan, y, material, tone, region=None, role="body", views="efp", target="body", pen=None, flags=None, **extra):
        """🏛️ A vertical prism on a bone standing at 90 degrees (mast stages, carriage): plan = closed polygon of world (x, z), y = (bottom, top); the elevation is its fore-aft extent, the plan the true section."""
        ox, oy, angle = REST[bone]
        assert abs(angle - 90) < 1e-9
        solid = {**self._base(id, bone, material, tone, role), "kind": "extrude", "profile": clean([(-(x - ox), z) for x, z in plan]), "plane": "yz", "from": r5(y[0] - oy), "to": r5(y[1] - oy)}
        self._flags(solid, flags or {})
        self._keep(solid, region, target, extra)
        if target == "body":
            xs, zs = [p[0] for p in plan], [p[1] for p in plan]
            outline = [(min(xs), y[0]), (max(xs), y[0]), (max(xs), y[1]), (min(xs), y[1])]
            self._draw(id, bone, [to_local(bone, px, py) for px, py in outline], min(zs), max(zs), material, tone, role, region, views, False, plan=plan, pen=pen)
        return solid

    def lofty(self, id, bone, stations, material, tone, region=None, role="body", views="efp", mirror=False, target="body", caps=True, pen=None, flags=None, **extra):
        """🗼 A loft along world y through stations (y, xc, zc, rx, rz, e): superelliptic horizontal sections, rx fore-aft and rz lateral half extents (bone at 0 degrees)."""
        sections = []
        for y, xc, zc, rx, rz, e in stations:
            u, v = to_local(bone, xc, y)
            sections.append({"at": [r5(u), r5(v), r5(zc)], "rx": r5(rx), "ry": r5(rz), "exponent": r5(e)})
        solid = {**self._base(id, bone, material, tone, role), "kind": "loft", "sections": sections, "capStart": caps, "capEnd": caps}
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-2 * stations[0][2])]}
        self._flags(solid, flags or {})
        self._keep(solid, region, target, extra)
        if target == "body":
            rear = [(xc - rx, y) for y, xc, _, rx, _, _ in stations]
            ahead = [(xc + rx, y) for y, xc, _, rx, _, _ in reversed(stations)]
            left = [(zc + rz, y) for y, _, zc, _, rz, _ in stations]
            right = [(zc - rz, y) for y, _, zc, _, rz, _ in reversed(stations)]
            plan = hull([p for y, xc, zc, rx, rz, e in stations for p in superellipse(xc, zc, rx, rz, e)])
            z0 = min(zc - rz for _, _, zc, _, rz, _ in stations)
            z1 = max(zc + rz for _, _, zc, _, rz, _ in stations)
            self._draw(id, bone, [to_local(bone, x, y) for x, y in rear + ahead], z0, z1, material, tone, role, region, views, mirror or None, front=left + right, plan=plan, pen=pen)
        return solid

    def stroke(self, id, bone, view, points, parent, role="detail", closed=False, filled=False, tone=0.3, material=None, pen=None, region=None):
        """✒️ A 2D-only stroke or small filled detail in one view (world rest coordinates; front: (z, y), plan: (x, z)), drawn right above its parent part."""
        self.strokes.append({"id": id, "bone": bone, "view": view, "points": points, "parent": parent, "role": role, "closed": closed, "filled": filled, "tone": tone, "material": material, "pen": pen, "region": region})

    def own(self, solid, target, **extra):
        (self.thermal_own if target == "thermal" else self.structure).append({**solid, **extra})

    def ref(self, id, of, member, part):
        self.structure.append({"id": id, "of": of, "member": member, "part": part})

    def shapes(self):
        """✏️ The three drawings: every part once per viewpoint, far to near; elevation in bone-local coordinates, front and plan as extruded solids of the elevation profile or as the hand-stated world outline of a part that is not laterally prismatic, strokes right above their parent."""
        def extent(prim, z0, z1):
            xs, ys = zip(*[to_world(prim["bone"], u, v) for u, v in prim["profile"]])
            return min(xs), max(xs), min(ys), max(ys), z0, z1

        def common(prim, id, order):
            shape = {"id": id, "bone": prim["bone"], "role": prim["role"], "z": order, "filled": prim["filled"], "tone": prim["tone"]}
            if prim["region"]:
                shape["region"] = prim["region"]
            if prim["material"]:
                shape["material"] = prim["material"]
            if prim["pen"]:
                shape["penMm"] = prim["pen"]
            return shape

        views = {"elevation": [], "front": [], "plan": []}
        for prim in self.prims:
            copies = [("", prim["z0"], prim["z1"])]
            if prim["mirror"]:
                copies = [("-l", prim["z0"], prim["z1"]), ("-r", -prim["z1"], -prim["z0"])]
            if "e" in prim["views"]:
                _, z0, z1 = copies[-1]
                box = extent(prim, z0, z1)
                views["elevation"].append(((-box[4], box[5]), prim, prim["id"], None, None))
            for index, (suffix, z0, z1) in enumerate(copies):
                box = extent(prim, z0, z1)
                sign = 1 if index == 0 else -1
                if "f" in prim["views"]:
                    override = None if prim["front"] is None else [(sign * x, y) for x, y in prim["front"]]
                    views["front"].append(((box[1], box[0]), prim, prim["id"] + suffix, (z0, z1), override))
                if "p" in prim["views"]:
                    override = None if prim["plan"] is None else [(x, sign * y) for x, y in prim["plan"]]
                    views["plan"].append(((box[3], box[2]), prim, prim["id"] + suffix, (z0, z1), override))
        result = {}
        for view, entries in views.items():
            ordered = sorted(enumerate(entries), key=lambda item: (round(item[1][0][0], 4), round(item[1][0][1], 4), item[0]))
            shapes = []
            for rank, (_, (_, prim, id, span, override)) in enumerate(ordered):
                order = r5(2 + rank * 0.01)
                shape = common(prim, id, order)
                if override is not None:
                    shape.update({"kind": "poly", "points": clean(override), "closed": True})
                elif span is None and prim["circle"]:
                    cx, cy, radius = prim["circle"]
                    shape.update({"kind": "ellipse", "cx": cx, "cy": cy, "rx": radius, "ry": radius})
                else:
                    shape.update({"kind": "poly", "points": prim["profile"], "closed": True})
                    if span is not None:
                        shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": r5(span[0]), "z1": r5(span[1])}
                shapes.append(shape)
                for k, stroke in enumerate(s for s in self.strokes if s["view"] == view and s["parent"] == id):
                    points = [to_local(stroke["bone"], x, y) for x, y in stroke["points"]] if view == "elevation" else stroke["points"]
                    item = {"id": stroke["id"], "bone": stroke["bone"], "role": stroke["role"], "z": r5(order + 0.001 * (k + 1)), "filled": stroke["filled"], "tone": stroke["tone"], "kind": "poly", "points": clean(points), "closed": stroke["closed"]}
                    if stroke["region"]:
                        item["region"] = stroke["region"]
                    if stroke["material"]:
                        item["material"] = stroke["material"]
                    if stroke["pen"]:
                        item["penMm"] = stroke["pen"]
                    shapes.append(item)
            result[view] = shapes
        return result

    def thermal(self):
        refs = [{"id": "th-" + solid["id"], "of": solid["id"], "region": self.regions[solid["id"]]} for solid in self.body if solid["id"] in self.regions]
        return refs + self.thermal_own

    def volume(self, decisions):
        return {"status": "authored", "denominator": self.denominator, "body": self.body, "thermal": self.thermal(), "structure": self.structure, "decisions": decisions}
