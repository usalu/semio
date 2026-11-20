"""✂️ Shared dimensions, linkage kinematics and part helpers of the round 3 scissor lift (Genie GS-1932 class): one hand-stated part gives its solid and its true orthographic 2D shapes."""
import math

ARM = 1.46
PITCH = 0.11
THETA0 = math.degrees(math.asin(PITCH / ARM))
THETA_MID = 30.0
PIN_TOP_RAISED = 5.79 - 0.10
BASE_Y = 0.46
THETA_MAX = math.degrees(math.asin((PIN_TOP_RAISED - BASE_Y) / (4 * ARM)))
XR = -0.7275
XF = XR + ARM * math.cos(math.radians(THETA0))
TOP_Y = BASE_Y + 4 * PITCH
CYL_S1 = ARM / 2
CYL_S2 = 0.18
AXLE_X = 0.66
AXLE_Y = 0.1525
AXLE_Z = 0.35
PLATFORM = (XR, TOP_Y)
EXT = (0.815, TOP_Y)
STAGES = 4


def r5(value):
    rounded = round(value, 5)
    return int(rounded) if rounded == int(rounded) else rounded


def cylinder(theta_deg):
    """🛢️ World direction (degrees) and length of the lift cylinder at an arm angle: barrel pin on the centre pivot tube of b1, rod eye on the lower cross tube of b2."""
    t = math.radians(theta_deg)
    dx = (CYL_S1 - CYL_S2) * math.cos(t)
    dy = (ARM + CYL_S2 - CYL_S1) * math.sin(t)
    return math.degrees(math.atan2(dy, dx)), math.hypot(dx, dy)


CYL_PHI0, CYL_LEN0 = cylinder(THETA0)


def offsets(theta_deg, ext=None):
    """📐 Pose offsets of every moving bone for an arm angle (closed linkage solved exactly)."""
    d = theta_deg - THETA0
    phi, length = cylinder(theta_deg)
    bones = {"a1": {"angleDeg": r5(d)}, "slide": {"dx": r5(ARM * (math.cos(math.radians(theta_deg)) - math.cos(math.radians(THETA0))))}, "b1": {"angleDeg": r5(-d)}}
    for k in range(2, STAGES + 1):
        bones[f"a{k}"] = {"angleDeg": r5(2 * d)}
        bones[f"b{k}"] = {"angleDeg": r5(-2 * d)}
    bones["platform"] = {"angleDeg": r5(d)}
    bones["lift-cylinder"] = {"angleDeg": r5(phi - CYL_PHI0 + d)}
    bones["lift-rod"] = {"dx": r5(length - CYL_LEN0)}
    if ext is not None:
        bones["deck-ext"] = {"dx": r5(ext)}
    return bones


def rest_frames():
    """🧭 World origin and angle of every bone at rest (stowed)."""
    frames = {"chassis": (0.0, 0.0, 0.0), "wheel-f": (AXLE_X, AXLE_Y, 0.0), "wheel-r": (-AXLE_X, AXLE_Y, 0.0), "slide": (XF, BASE_Y, 0.0), "platform": (XR, TOP_Y, 0.0), "deck-ext": (EXT[0], EXT[1], 0.0)}
    c, s = math.cos(math.radians(THETA0)), math.sin(math.radians(THETA0))
    for k in range(1, STAGES + 1):
        y = BASE_Y + (k - 1) * PITCH
        frames[f"a{k}"] = (XR, y, THETA0)
        frames[f"b{k}"] = (XF, y, 180.0 - THETA0)
    frames["lift-cylinder"] = (XF - CYL_S1 * c, BASE_Y + CYL_S1 * s, CYL_PHI0)
    frames["lift-rod"] = (frames["lift-cylinder"][0] + CYL_LEN0 * math.cos(math.radians(CYL_PHI0)), frames["lift-cylinder"][1] + CYL_LEN0 * math.sin(math.radians(CYL_PHI0)), CYL_PHI0)
    return frames


FRAMES = rest_frames()


def world(bone, u, v):
    ox, oy, angle = FRAMES[bone]
    c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
    return ox + u * c - v * s, oy + u * s + v * c


def ngon(cx, cy, r, n=16):
    return [[r5(cx + r * math.cos(2 * math.pi * i / n)), r5(cy + r * math.sin(2 * math.pi * i / n))] for i in range(n)]


class Scale:
    """🧱 Part collector of one authored scale: body solids with their regions, the 2D shapes of the three viewpoints, and the own solids of the thermal and structure lists."""

    def __init__(self, denominator):
        self.denominator = denominator
        self.body = []
        self.regions = {}
        self.prims = []
        self.thermal_own = []
        self.structure = []
        self.origin = (0.0, 0.0)

    def at(self, origin):
        self.origin = origin
        return self

    def _base(self, id, bone, material, tone, role):
        return {"id": id, "bone": bone, "material": material, "role": role, "tone": tone}

    def _keep(self, solid, region, target, extra):
        if target == "body":
            self.body.append(solid)
            if region:
                self.regions[solid["id"]] = region
        elif target == "thermal":
            self.thermal_own.append({**solid, "region": region, **extra})
        else:
            self.structure.append({**solid, **extra})

    def _draw(self, id, bone, profile, z0, z1, material, tone, role, region, views, mirror, circle=None):
        if not views:
            return
        self.prims.append({"id": id, "bone": bone, "profile": profile, "z0": z0, "z1": z1, "material": material, "tone": tone, "role": role, "region": region, "views": views, "mirror": mirror, "circle": circle})

    def bar(self, id, bone, x, y, z, material, tone, region=None, role="body", radius=0, views="efp", mirror=False, target="body", **extra):
        """📦 Axis-aligned bar: x, y, z are (min, max) pairs in the current origin frame; mirror repeats it at -z."""
        ox, oy = self.origin
        x0, x1, y0, y1 = x[0] - ox, x[1] - ox, y[0] - oy, y[1] - oy
        solid = {**self._base(id, bone, material, tone, role), "kind": "box", "size": [r5(x1 - x0), r5(y1 - y0), r5(z[1] - z[0])]}
        if radius:
            solid["cornerRadius"] = radius
        solid["transform"] = {"translation": [r5((x0 + x1) / 2), r5((y0 + y1) / 2), r5((z[0] + z[1]) / 2)]}
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-(z[0] + z[1]))]}
        self._keep(solid, region, target, extra)
        if target == "body":
            self._draw(id, bone, [[r5(x0), r5(y0)], [r5(x1), r5(y0)], [r5(x1), r5(y1)], [r5(x0), r5(y1)]], z[0], z[1], material, tone, role, region, views, mirror)
        return solid

    def prism(self, id, bone, profile, z, material, tone, region=None, role="body", holes=None, views="efp", mirror=False, target="body", **extra):
        """🔷 Profile in the sagittal plane of the current origin frame, extruded across z = (min, max)."""
        ox, oy = self.origin
        local = [[r5(px - ox), r5(py - oy)] for px, py in profile]
        solid = {**self._base(id, bone, material, tone, role), "kind": "extrude", "profile": local}
        if holes:
            solid["holes"] = [[[r5(px - ox), r5(py - oy)] for px, py in hole] for hole in holes]
        solid.update({"plane": "xy", "from": r5(z[0]), "to": r5(z[1])})
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-(z[0] + z[1]))]}
        self._keep(solid, region, target, extra)
        if target == "body":
            self._draw(id, bone, local, z[0], z[1], material, tone, role, region, views, mirror)
        return solid

    def tube(self, id, bone, x, yc, zc, size, wall, material, tone, mirror=False, **extra):
        """🟦 Hollow rectangular section along the bone axis for the structure list: size = (height, width), real wall."""
        ox, oy = self.origin
        h, w = size
        outer = [[r5(yc - oy - h / 2), r5(zc - w / 2)], [r5(yc - oy + h / 2), r5(zc - w / 2)], [r5(yc - oy + h / 2), r5(zc + w / 2)], [r5(yc - oy - h / 2), r5(zc + w / 2)]]
        inner = [[r5(yc - oy - h / 2 + wall), r5(zc - w / 2 + wall)], [r5(yc - oy + h / 2 - wall), r5(zc - w / 2 + wall)], [r5(yc - oy + h / 2 - wall), r5(zc + w / 2 - wall)], [r5(yc - oy - h / 2 + wall), r5(zc + w / 2 - wall)]]
        solid = {**self._base(id, bone, material, tone, "body"), "kind": "extrude", "profile": outer, "holes": [inner], "plane": "yz", "from": r5(x[0] - ox), "to": r5(x[1] - ox)}
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-2 * zc)]}
        self.structure.append({**solid, **extra})
        return solid

    def roll(self, id, bone, cx, cy, r, z, material, tone, region=None, role="body", bore=0, views="efp", mirror=False, target="body", **extra):
        """🥁 Cylinder about the lateral axis (pin, cross tube, roller): centre (cx, cy), z = (min, max); bore makes it a pipe."""
        ox, oy = self.origin
        cx, cy = cx - ox, cy - oy
        length = r5(z[1] - z[0])
        profile = [[0, r5(bore)], [0, r5(r)], [length, r5(r)], [length, r5(bore)], [0, r5(bore)]] if bore else [[0, r5(r)], [length, r5(r)]]
        solid = {**self._base(id, bone, material, tone, role), "kind": "revolve", "profile": profile, "axis": [0, 0, 1], "origin": [r5(cx), r5(cy), r5(z[0])]}
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-(z[0] + z[1]))]}
        self._keep(solid, region, target, extra)
        if target == "body":
            self._draw(id, bone, ngon(cx, cy, r), z[0], z[1], material, tone, role, region, views, mirror, circle=(r5(cx), r5(cy), r5(r)))
        return solid

    def rod(self, id, bone, x, cy, cz, r, material, tone, region=None, role="body", bore=0, views="efp", mirror=False, target="body", **extra):
        """🧪 Cylinder along the bone axis (barrel, piston rod): x = (min, max), centre line at (cy, cz)."""
        ox, oy = self.origin
        x0, x1, cy = x[0] - ox, x[1] - ox, cy - oy
        length = r5(x1 - x0)
        profile = [[0, r5(bore)], [0, r5(r)], [length, r5(r)], [length, r5(bore)], [0, r5(bore)]] if bore else [[0, r5(r)], [length, r5(r)]]
        solid = {**self._base(id, bone, material, tone, role), "kind": "revolve", "profile": profile, "axis": [1, 0, 0], "origin": [r5(x0), r5(cy), r5(cz)]}
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-2 * cz)]}
        self._keep(solid, region, target, extra)
        if target == "body":
            self._draw(id, bone, [[r5(x0), r5(cy - r)], [r5(x1), r5(cy - r)], [r5(x1), r5(cy + r)], [r5(x0), r5(cy + r)]], cz - r, cz + r, material, tone, role, region, views, mirror)
        return solid

    def post(self, id, bone, cx, y, cz, r, material, tone, region=None, role="body", views="efp", mirror=False, target="body", **extra):
        """📍 Upright cylinder: centre (cx, cz), y = (min, max)."""
        ox, oy = self.origin
        cx, y0, y1 = cx - ox, y[0] - oy, y[1] - oy
        solid = {**self._base(id, bone, material, tone, role), "kind": "revolve", "profile": [[0, r5(r)], [r5(y1 - y0), r5(r)]], "axis": [0, 1, 0], "origin": [r5(cx), r5(y0), r5(cz)]}
        if mirror:
            solid["repeat"] = {"count": 2, "translation": [0, 0, r5(-2 * cz)]}
        self._keep(solid, region, target, extra)
        if target == "body":
            self._draw(id, bone, [[r5(cx - r), r5(y0)], [r5(cx + r), r5(y0)], [r5(cx + r), r5(y1)], [r5(cx - r), r5(y1)]], cz - r, cz + r, material, tone, role, region, views, mirror)
        return solid

    def wheel(self, id, bone, side, profile, z, material, tone, region, role="body", views="efp", draw_r=None, closed=True):
        """🛞 Body of revolution about the axle of a wheel bone on one side (side = +1 left, -1 right); profile = (outward axial offset from the wheel centre plane, radius)."""
        points = [[r5(a), r5(r)] for a, r in profile]
        if closed:
            points.append(points[0])
        solid = {**self._base(id, bone, material, tone, role), "kind": "revolve", "profile": points, "axis": [0, 0, side], "origin": [0, 0, r5(side * AXLE_Z)]}
        self.body.append(solid)
        if region:
            self.regions[id] = region
        radius = draw_r or max(r for _, r in profile)
        z0, z1 = sorted((side * z[0], side * z[1]))
        self._draw(id, bone, ngon(0, 0, radius, 24), z0, z1, material, tone, role, region, views, False, circle=(0, 0, r5(radius)))
        return solid

    def own(self, solid, target, **extra):
        (self.thermal_own if target == "thermal" else self.structure).append({**solid, **extra})

    def ref(self, id, of, member, part):
        self.structure.append({"id": id, "of": of, "member": member, "part": part})

    def shapes(self):
        """✏️ The three drawings: every part once per viewpoint, far to near, as its profile on its bone; front and plan carry the lateral span as an extruded solid."""
        def extent(prim, z0, z1):
            xs, ys = zip(*[world(prim["bone"], u, v) for u, v in prim["profile"]])
            return min(xs), max(xs), min(ys), max(ys), z0, z1

        def common(prim, id, order):
            shape = {"id": id, "bone": prim["bone"], "role": prim["role"], "z": order, "filled": True, "tone": prim["tone"]}
            if prim["region"]:
                shape["region"] = prim["region"]
            shape["material"] = prim["material"]
            return shape

        views = {"elevation": [], "front": [], "plan": []}
        for prim in self.prims:
            copies = [("", prim["z0"], prim["z1"])]
            if prim["mirror"]:
                copies = [("-l", prim["z0"], prim["z1"]), ("-r", -prim["z1"], -prim["z0"])]
            if "e" in prim["views"]:
                suffix, z0, z1 = copies[-1]
                box = extent(prim, z0, z1)
                views["elevation"].append((-box[4], box[5], prim, prim["id"], None))
            for suffix, z0, z1 in copies:
                box = extent(prim, z0, z1)
                if "f" in prim["views"]:
                    views["front"].append((box[1], box[0], prim, prim["id"] + suffix, (z0, z1)))
                if "p" in prim["views"]:
                    views["plan"].append((box[3], box[2], prim, prim["id"] + suffix, (z0, z1)))
        result = {}
        for view, entries in views.items():
            ordered = sorted(enumerate(entries), key=lambda item: (round(item[1][0], 4), round(item[1][1], 4), item[0]))
            shapes = []
            for rank, (_, (_, _, prim, id, span)) in enumerate(ordered):
                shape = common(prim, id, r5(2 + rank * 0.01))
                if span is None and prim["circle"]:
                    cx, cy, radius = prim["circle"]
                    shape.update({"kind": "ellipse", "cx": cx, "cy": cy, "rx": radius, "ry": radius})
                else:
                    shape.update({"kind": "poly", "points": prim["profile"], "closed": True})
                if span is not None:
                    shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": r5(span[0]), "z1": r5(span[1])}
                shapes.append(shape)
            result[view] = shapes
        return result

    def thermal(self):
        refs = [{"id": "th-" + solid["id"], "of": solid["id"], "region": self.regions[solid["id"]]} for solid in self.body if solid["id"] in self.regions]
        return refs + self.thermal_own

    def volume(self, decisions):
        return {"status": "authored", "denominator": self.denominator, "body": self.body, "thermal": self.thermal(), "structure": self.structure, "decisions": decisions}
