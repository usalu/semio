"""🪣 The four handcrafted scales of the used dog waste station: body, thermal and structure solids and the three 2D views per scale.

Every number is a world position in metres of the rest pose (x ahead, y up, z left); `author_geometry` turns it into bone-local solids and shapes.
"""
from author_geometry import (box, plan_extrude, front_extrude, ellipsoid, side_extrude, circle, rounded_rect, rect_pts, shape, g_rect, g_poly,
                             g_path, g_ellipse, el_rect, el_poly, el_path, el_ellipse, swept, loc, r6)

FOOT = (-0.15, 0.15, -0.60, 0.02, -0.15, 0.15)
CHIP = 0.035
POST_H = 0.03
POST_TOP = 1.93
CAP_TOP = 1.94
SIGN = (0.034, 0.036, 1.66, 1.91, -0.15, 0.15)
HOUSE = (0.035, 0.165, 1.05, 1.35, -0.12, 0.12)
WALL = 0.0015
SLOT = (0.128, 0.148, -0.07, 0.07)
ROLL_C = (0.10, 1.22)
ROLL_R = 0.0375
CORE_R = 0.0105
TAB = (0.137, 0.138, 1.01, 1.22, -0.06, 0.06)
LID_HINGE = (0.037, 1.355)
BIN = (0.035, 0.265, 0.40, 0.86, -0.155, 0.155)
OPEN = (-0.11, 0.11, 0.70, 0.80)
HOOD_HINGE = (0.037, 0.8655)
FLAP_HINGE = (0.2585, 0.809)
BAG = (0.09, 0.056, 0.08, 0.06, 0.038, 0.045)

MOVING = {"dispenser-lid", "bin-hood", "throw-flap"}


class Scale:
    """🧺 The lists of one authored scale while they are built."""

    def __init__(self, d):
        self.d = d
        self.body, self.thermal, self.structure = [], [], []
        self.views = {"elevation": [], "front": [], "plan": []}

    def add(self, solid):
        """➕ A body solid."""
        self.body.append(solid)
        return solid

    def th(self, tid, region, of=None, solid=None):
        """🌡 A thermal solid: a reference to a body solid or an own solid."""
        if of is not None:
            self.thermal.append({"id": tid, "of": of, "region": region})
        else:
            s = dict(solid)
            s["id"] = tid
            s["region"] = region
            self.thermal.append(s)

    def st(self, sid, member, part, of=None, solid=None, anchor=None, members=None):
        """🏗 A structure solid."""
        if of is not None:
            s = {"id": sid, "of": of}
        else:
            s = dict(solid)
            s["id"] = sid
        s["member"] = member
        if members:
            s["members"] = members
        s["part"] = part
        if anchor:
            s["anchor"] = anchor
        self.structure.append(s)

    def draw(self, view, near, sh):
        """✏️ A 2D shape with its nearness to the viewer at rest (larger paints later)."""
        self.views[view].append((near, len(self.views[view]), sh))

    def drawbox(self, sid, bone, role, tone, b, region=None, regions=None, material=None, views="efp", pen=None, hatch=None, lift=0.0):
        """🖼 The three rect shapes of an axis-aligned world box; parts of turning bones are swept solids in front and plan."""
        x0, x1, y0, y1, z0, z1 = b
        moving = bone in MOVING
        kw = dict(region=region, regions=regions, material=material, pen=pen, hatch=hatch)
        if "e" in views:
            self.draw("elevation", -z0 + lift, shape(sid, bone, role, True, tone, el_rect(bone, x0, x1, y0, y1), **kw))
        if "f" in views:
            g = el_rect(bone, x0, x1, y0, y1) if moving else g_rect(z0, y0, z1 - z0, y1 - y0)
            self.draw("front", x1 + lift, shape(sid, bone, role, True, tone, g, solid=swept(z0, z1) if moving else None, **kw))
        if "p" in views:
            g = el_rect(bone, x0, x1, y0, y1) if moving else g_rect(x0, z0, x1 - x0, z1 - z0)
            self.draw("plan", y1 + lift, shape(sid, bone, role, True, tone, g, solid=swept(z0, z1) if moving else None, **kw))

    def finish(self):
        """🎨 The three views in paint order with their z."""
        out = {}
        for view, items in self.views.items():
            items.sort(key=lambda t: (round(t[0], 7), t[1]))
            shapes = []
            for k, (_, _, sh) in enumerate(items):
                sh = dict(sh)
                sh["z"] = round(1 + 0.1 * k, 1)
                shapes.append(sh)
            out[view] = shapes
        return out


def post_profile(h, wall, ro, ri):
    """🔲 The square hollow section in plan: outer ring with corner radius `ro` and the bore with `ri`."""
    return rounded_rect(-h, h, -h, h, ro, 3), rounded_rect(-h + wall, h - wall, -h + wall, h - wall, ri, 3)


def chip_profile():
    """🪨 The plan of the footing head with its knocked-off front-right corner."""
    x0, x1, _, _, z0, z1 = FOOT
    return [(x0, z0), (x1 - CHIP, z0), (x1, z0 + CHIP), (x1, z1), (x0, z1)]


def clamp_profile(y_lug):
    """🗜 A band clamp around the 60 mm post in plan with its lug towards the part it holds."""
    if y_lug <= 0.032:
        return rect_pts(-0.032, 0.032, -0.032, 0.032), [rect_pts(-0.030, 0.030, -0.030, 0.030)]
    return [(-0.032, -0.032), (0.032, -0.032), (0.032, -0.02), (y_lug, -0.02), (y_lug, 0.02), (0.032, 0.02), (0.032, 0.032), (-0.032, 0.032)], [rect_pts(-0.030, 0.030, -0.030, 0.030)]


def u_profile(x0, x1, z0, z1, t, open_side):
    """🥡 A U of plates in plan of thickness `t`: open at the front (`+x`) or the rear (`-x`)."""
    if open_side == "+x":
        return [(x0, z0), (x1, z0), (x1, z0 + t), (x0 + t, z0 + t), (x0 + t, z1 - t), (x1, z1 - t), (x1, z1), (x0, z1)]
    return [(x1, z1), (x0, z1), (x0, z1 - t), (x1 - t, z1 - t), (x1 - t, z0 + t), (x0, z0 + t), (x0, z0), (x1, z0)]


def footing(sc, split):
    """🧱 The C20/25 point foundation 0.30 x 0.30 x 0.62 m with the post set in, and the filled bag somebody left on its head."""
    x0, x1, y0, y1, z0, z1 = FOOT
    if split:
        sc.add(box("footing-block", "footing", "concrete-weathered", "body", 0.55, x0, x1, y0, -0.03, z0, z1))
        sc.add(plan_extrude("footing-head", "footing", "concrete-weathered", "body", 0.55, chip_profile(), -0.03, y1))
        sc.th("th-footing-block", "footing-concrete", of="footing-block")
        sc.th("th-footing-head", "footing-concrete", of="footing-head")
        sc.st("member-footing", "footing", "member", of="footing-block")
        sc.st("member-footing-head", "footing", "member", of="footing-head")
    else:
        sc.add(box("footing-block", "footing", "concrete-weathered", "body", 0.55, x0, x1, y0, y1, z0, z1))
        sc.th("th-footing-block", "footing-concrete", of="footing-block")
        sc.st("member-footing", "footing", "member", of="footing-block")


def left_bag(sc, ears):
    """🛍 The knotted filled dog-waste bag left on the footing head."""
    cx, cy, cz, rx, ry, rz = BAG
    if sc.d == 500:
        ry = 0.038
        cy = 0.02 + ry - 0.002
    sc.add(ellipsoid("left-bag", "left-bag", "filled-bag", "body", 0.12, cx, cy, cz, rx, ry, rz))
    sc.th("th-left-bag", "left-bag", of="left-bag")
    if ears:
        for k, dx in ((0, -0.016), (1, 0.016)):
            sc.add(ellipsoid(f"left-bag-ear-{k}", "left-bag", "filled-bag", "detail", 0.12, cx + dx, cy + ry + 0.006, cz, 0.008, 0.012, 0.008))
            sc.th(f"th-left-bag-ear-{k}", "left-bag", of=f"left-bag-ear-{k}")


def post(sc, hollow, foot_floor=None):
    """🪧 The 60 x 60 x 3 mm galvanised and powder-coated post, set 0.52 m into the footing."""
    if hollow:
        outer, bore = post_profile(POST_H, 0.003, 0.0045, 0.0015)
        sc.add(plan_extrude("post", "post", "steel-powder-green", "body", 0.38, outer, -0.50, POST_TOP, holes=[bore]))
        sc.add(box("post-cap", "post", "steel-powder-green", "detail", 0.32, -0.032, 0.032, POST_TOP, CAP_TOP, -0.032, 0.032, radius=0.003))
        sc.th("th-post-paint", "post-paint", solid=plan_extrude("x", "post", "steel-powder-green", "body", 0.38, outer, 0.15, POST_TOP, holes=[bore]))
        sc.th("th-post-cap", "post-paint", of="post-cap")
        sc.th("th-post-foot", "post-foot-corroded", solid=plan_extrude("x", "post", "steel-corroded", "body", 0.3, outer, 0.02, 0.15, holes=[bore]))
        sc.st("member-post", "post", "member", of="post")
    else:
        extra = {"slender": True} if sc.d == 500 else {}
        sc.add(box("post", "post", "steel-powder-green", "body", 0.38, -POST_H, POST_H, -0.50, CAP_TOP, -POST_H, POST_H, **extra))
        sc.th("th-post-paint", "post-paint", solid=box("x", "post", "steel-powder-green", "body", 0.38, -POST_H, POST_H, 0.15, CAP_TOP, -POST_H, POST_H, **extra))
        fh = 0.0375 if sc.d == 500 else POST_H
        sc.th("th-post-foot", "post-foot-corroded", solid=box("x", "post", "steel-corroded", "body", 0.3, -fh, fh, 0.02, 0.15, -fh, fh))
        sc.st("member-post", "post", "member", of="post")


def sign(sc, clamps):
    """🐕 The 300 x 250 x 2 mm aluminium sign with its faded print, held by two band clamps (merged into the post face at coarse scales)."""
    x0, x1, y0, y1, z0, z1 = SIGN
    if not clamps:
        x0, x1 = 0.030, 0.032
    sc.add(front_extrude("sign-plate", "sign", "aluminium-sign", "body", 0.82, rounded_rect(z0, z1, y0, y1, 0.015, 3), x0, x1, sheet=True))
    sc.th("th-sign-plate", "sign-plate", of="sign-plate")
    sc.st("member-sign", "sign", "member", of="sign-plate")
    if clamps:
        for k, yc in ((0, 1.70), (1, 1.87)):
            outer, holes = clamp_profile(0.034)
            sc.add(plan_extrude(f"sign-clamp-{k}", "sign", "steel-galvanised", "detail", 0.45, outer, yc - 0.0125, yc + 0.0125, holes=holes))
            sc.th(f"th-sign-clamp-{k}", "brackets-galvanised", of=f"sign-clamp-{k}")
            sc.st(f"joint-sign-clamp-{k}", "sign", "joint", of=f"sign-clamp-{k}")
    return x0, x1


def dispenser(sc, level):
    """📦 The powder-coated roll-bag dispenser 240 x 130 x 300 mm with its top lid, axle, half-empty roll and the hanging bag tab."""
    x0, x1, y0, y1, z0, z1 = HOUSE
    t = WALL
    if level <= 3:
        sc.add(box("dispenser-mount", "dispenser", "steel-galvanised", "detail", 0.45, 0.030, 0.035, 1.06, 1.34, -0.055, 0.055, sheet=True))
        sc.th("th-dispenser-mount", "brackets-galvanised", of="dispenser-mount")
        sc.st("joint-dispenser-mount", "dispenser", "joint", of="dispenser-mount")
    else:
        sc.th("th-dispenser-mount", "brackets-galvanised", solid=box("x", "dispenser", "steel-galvanised", "detail", 0.45, 0.030, 0.035, 1.06, 1.34, -0.055, 0.055, sheet=True))
    if level <= 2:
        for k, yc in ((0, 1.09), (1, 1.31)):
            outer, holes = clamp_profile(0.032)
            sc.add(plan_extrude(f"dispenser-clamp-{k}", "dispenser", "steel-galvanised", "detail", 0.45, outer, yc - 0.0125, yc + 0.0125, holes=holes))
            sc.th(f"th-dispenser-clamp-{k}", "brackets-galvanised", of=f"dispenser-clamp-{k}")
            sc.st(f"joint-dispenser-clamp-{k}", "dispenser", "joint", of=f"dispenser-clamp-{k}")
    if level <= 3:
        sc.add(plan_extrude("dispenser-walls", "dispenser", "steel-powder-green", "body", 0.36, rect_pts(x0, x1, z0, z1), y0, y1, holes=[rect_pts(x0 + t, x1 - t, z0 + t, z1 - t)]))
        hole = [rect_pts(SLOT[0], SLOT[1], SLOT[2], SLOT[3])]
        sc.add(plan_extrude("dispenser-bottom", "dispenser", "steel-powder-green", "detail", 0.3, rect_pts(x0 + t, x1 - t, z0 + t, z1 - t), y0, y0 + t, holes=hole, sheet=True))
        sc.add(side_extrude("dispenser-axle", "dispenser", "steel-galvanised", "detail", 0.4, circle(ROLL_C[0], ROLL_C[1] + CORE_R - 0.004, 0.004, 12), z0 + t, z1 - t, slender=True))
        sc.th("th-dispenser-walls", "dispenser-housing", of="dispenser-walls")
        sc.th("th-dispenser-bottom", "dispenser-housing", of="dispenser-bottom")
        sc.th("th-dispenser-axle", "dispenser-housing", of="dispenser-axle")
        sc.st("member-dispenser", "dispenser", "member", of="dispenser-walls")
        sc.st("member-dispenser-bottom", "dispenser", "member", of="dispenser-bottom")
        if level == 1:
            sc.add(front_extrude("dispenser-lock", "dispenser", "steel-galvanised", "detail", 0.25, circle(0.0, 1.29, 0.008, 16), x1, x1 + 0.008))
            sc.th("th-dispenser-lock", "dispenser-housing", of="dispenser-lock")
        lx0, lx1, lz = 0.035, 0.1685, 0.1235
        sc.add(box("lid-plate", "dispenser-lid", "steel-powder-green", "body", 0.42, lx0, lx1, 1.35, 1.3515, -lz, lz, sheet=True))
        sc.th("th-lid-plate", "dispenser-lid", of="lid-plate")
        if level <= 2:
            sc.add(plan_extrude("lid-skirt", "dispenser-lid", "steel-powder-green", "body", 0.42, u_profile(lx0, lx1, -lz, lz, t, "-x"), 1.335, 1.35))
            sc.th("th-lid-skirt", "dispenser-lid", of="lid-skirt")
        if level == 1:
            sc.add(side_extrude("lid-hinge", "dispenser-lid", "steel-galvanised", "detail", 0.3, circle(LID_HINGE[0], LID_HINGE[1], 0.004, 12), -0.10, 0.10, slender=True))
            sc.th("th-lid-hinge", "dispenser-lid", of="lid-hinge")
            sc.st("joint-lid-hinge", "dispenser", "joint", of="lid-hinge")
        hole = [circle(ROLL_C[0], ROLL_C[1], CORE_R, 16)] if level <= 2 else None
        sc.add(side_extrude("bag-roll", "bag-roll", "bag-film-roll", "body", 0.2, circle(ROLL_C[0], ROLL_C[1], ROLL_R, 24), -0.105, 0.105, holes=hole))
        sc.th("th-bag-roll", "bag-roll", solid=side_extrude("x", "bag-roll", "bag-film-roll", "body", 0.2, circle(ROLL_C[0], ROLL_C[1], ROLL_R, 24), -0.105, 0.105, holes=hole))
        tx0, tx1, ty0, ty1, tz0, tz1 = TAB
        sc.add(box("bag-tab", "bag-tab", "bag-film-roll", "detail", 0.25, tx0, tx1, ty0, ty1, tz0, tz1, sheet=True))
        sc.th("th-bag-tab", "bag-roll", solid=box("x", "bag-tab", "bag-film-roll", "detail", 0.25, tx0, tx1, ty0, ty1, tz0, tz1, sheet=True))
    else:
        lx1, lz = 0.1685, 0.1235
        sc.add(box("dispenser-box", "dispenser", "steel-powder-green", "body", 0.36, x0, lx1, y0, 1.3515, -lz, lz))
        sc.th("th-dispenser-box", "dispenser-housing", of="dispenser-box")
        sc.th("th-lid-plate", "dispenser-lid", solid=box("x", "dispenser-lid", "steel-powder-green", "body", 0.42, 0.035, lx1, 1.3515, 1.353, -lz, lz, sheet=True))
        sc.st("member-dispenser", "dispenser", "member", of="dispenser-box")
        sc.add(side_extrude("bag-roll", "bag-roll", "bag-film-roll", "body", 0.2, circle(ROLL_C[0], ROLL_C[1], ROLL_R, 16), -0.105, 0.105))
        sc.th("th-bag-roll", "bag-roll", solid=side_extrude("x", "bag-roll", "bag-film-roll", "body", 0.2, circle(ROLL_C[0], ROLL_C[1], ROLL_R, 16), -0.105, 0.105))


def waste_bin(sc, level):
    """🗑 The galvanised 30 l dog-waste bin 310 x 230 x 460 mm with its rain hood, throw-in opening and inward flap."""
    x0, x1, y0, y1, z0, z1 = BIN
    t = WALL
    if level <= 3:
        sc.add(box("bin-mount", "bin", "steel-galvanised", "detail", 0.45, 0.030, 0.035, 0.45, 0.82, -0.06, 0.06, sheet=True))
        sc.th("th-bin-mount", "brackets-galvanised", of="bin-mount")
        sc.st("joint-bin-mount", "bin", "joint", of="bin-mount")
    else:
        sc.th("th-bin-mount", "brackets-galvanised", solid=box("x", "bin", "steel-galvanised", "detail", 0.45, 0.030, 0.035, 0.45, 0.82, -0.06, 0.06, sheet=True))
    if level <= 2:
        for k, yc in ((0, 0.50), (1, 0.77)):
            outer, holes = clamp_profile(0.032)
            sc.add(plan_extrude(f"bin-clamp-{k}", "bin", "steel-galvanised", "detail", 0.45, outer, yc - 0.0125, yc + 0.0125, holes=holes))
            sc.th(f"th-bin-clamp-{k}", "brackets-galvanised", of=f"bin-clamp-{k}")
            sc.st(f"joint-bin-clamp-{k}", "bin", "joint", of=f"bin-clamp-{k}")
    if level <= 3:
        sc.add(plan_extrude("bin-walls", "bin", "steel-galvanised", "body", 0.6, u_profile(x0, x1 - t, z0, z1, t, "+x"), y0, y1))
        ox0, ox1, oy0, oy1 = OPEN
        sc.add(front_extrude("bin-front", "bin", "steel-galvanised", "body", 0.6, rect_pts(z0, z1, y0, y1), x1 - t, x1, holes=[rect_pts(ox0, ox1, oy0, oy1)], sheet=True))
        sc.add(plan_extrude("bin-bottom", "bin", "steel-galvanised", "detail", 0.5, rect_pts(x0 + t, x1 - t, z0 + t, z1 - t), y0, y0 + t, sheet=True))
        for sid in ("bin-walls", "bin-front", "bin-bottom"):
            sc.th(f"th-{sid}", "bin-body", of=sid)
        sc.st("member-bin", "bin", "member", of="bin-walls")
        sc.st("member-bin-front", "bin", "member", of="bin-front")
        sc.st("member-bin-bottom", "bin", "member", of="bin-bottom")
        fx, fy = FLAP_HINGE
        sc.add(side_extrude("flap-rod", "throw-flap", "steel-galvanised", "detail", 0.4, circle(fx, fy, 0.005, 12), -0.115, 0.115, slender=True))
        sc.add(box("flap-plate", "throw-flap", "steel-galvanised", "body", 0.5, 0.252, 0.2535, 0.695, 0.811, -0.115, 0.115, sheet=True))
        sc.th("th-flap-rod", "bin-body", of="flap-rod")
        sc.th("th-flap-plate", "bin-body", of="flap-plate")
        sc.st("joint-flap-rod", "bin", "joint", of="flap-rod")
        hx0, hx1, hz = 0.035, 0.2685, 0.1585
        sc.add(box("hood-plate", "bin-hood", "steel-galvanised", "body", 0.66, hx0, hx1, 0.86, 0.8615, -hz, hz, sheet=True))
        sc.th("th-hood-plate", "bin-hood", of="hood-plate")
        if level <= 2:
            sc.add(plan_extrude("hood-skirt", "bin-hood", "steel-galvanised", "body", 0.66, u_profile(hx0, hx1, -hz, hz, t, "-x"), 0.835, 0.86))
            sc.th("th-hood-skirt", "bin-hood", of="hood-skirt")
        if level == 1:
            sc.add(side_extrude("hood-hinge", "bin-hood", "steel-galvanised", "detail", 0.3, circle(HOOD_HINGE[0], HOOD_HINGE[1], 0.005, 12), -0.12, 0.12, slender=True))
            sc.add(front_extrude("hood-lock", "bin-hood", "steel-galvanised", "detail", 0.25, circle(0.0, 0.8475, 0.009, 16), hx1, hx1 + 0.010))
            sc.th("th-hood-hinge", "bin-hood", of="hood-hinge")
            sc.th("th-hood-lock", "bin-hood", of="hood-lock")
            sc.st("joint-hood-hinge", "bin", "joint", of="hood-hinge")
    else:
        hx1, hz = 0.2685, 0.1585
        sc.add(box("bin-box", "bin", "steel-galvanised", "body", 0.6, x0, hx1, y0, 0.8615, -hz, hz))
        sc.th("th-bin-box", "bin-body", of="bin-box")
        sc.th("th-hood-plate", "bin-hood", solid=box("x", "bin-hood", "steel-galvanised", "body", 0.66, 0.035, hx1, 0.8615, 0.863, -hz, hz, sheet=True))
        sc.st("member-bin", "bin", "member", of="bin-box")


def support_and_masses(sc, masses):
    """⚓ The fixed support under the footing and the mass markers at the centres of mass of the heavy parts."""
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[sc.d]
    sc.st("support-footing-base", "footing", "support", solid=box("x", "footing", "concrete-weathered", "detail", 0.2, -0.15, 0.15, -0.60, -0.60 + f, -0.15, 0.15), anchor="footing-base")
    r = 0.0376 if sc.d == 500 else 0.03
    for bone, (cx, cy, cz) in masses.items():
        sc.st(f"mass-{bone}", bone, "mass", solid=ellipsoid("x", bone, "steel-galvanised", "detail", 0.15, cx, cy, cz, r, r, r))
