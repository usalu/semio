"""🖍 The handdrawn elevation, front and plan of the used dog waste station per scale, to the same numbers as the solids of `author_scales`.

Level 1 is 1:50 (every part and the traces of use), 2 is 1:100, 3 is 1:200 and 4 is 1:500 (body masses only).
"""
from author_geometry import shape, g_rect, g_poly, g_path, g_ellipse, el_rect, el_poly, el_path, el_ellipse, swept, rounded_rect, rect_pts, circle, loc
from author_scales import FOOT, CHIP, POST_H, POST_TOP, CAP_TOP, SIGN, HOUSE, WALL, SLOT, ROLL_C, ROLL_R, TAB, LID_HINGE, BIN, OPEN, HOOD_HINGE, FLAP_HINGE, BAG, chip_profile

SOLID_PEN = {1: None, 2: 0.25, 3: 0.25, 4: 0.25}


def stroke(sc, view, near, sid, bone, role, pts, closed=False, region=None, moving_z=None, el=False):
    """〰 An unfilled stroke: world points of the view, or of the side profile when `el` or `moving_z` is set."""
    g = el_poly(bone, pts, closed) if (el or moving_z) else g_poly(pts, closed)
    sc.draw(view, near, shape(sid, bone, role, False, 0.2, g, region=region, solid=swept(*moving_z) if moving_z else None))


def footing(sc, lv):
    """🧱 Footing in the three views: cut body below ground, the knocked-off corner and moss on the shaded rear."""
    x0, x1, y0, y1, z0, z1 = FOOT
    hatch = "concrete-stipple" if lv <= 2 else None
    kw = dict(region="footing-concrete", material="concrete-weathered", hatch=hatch)
    sc.draw("elevation", -z0, shape("footing", "footing", "body", True, 0.55, el_rect("footing", x0, x1, y0, y1), **kw))
    sc.draw("front", x1, shape("footing", "footing", "body", True, 0.55, g_rect(z0, y0, z1 - z0, y1 - y0), **kw))
    pts = chip_profile() if lv <= 3 else rect_pts(x0, x1, z0, z1)
    sc.draw("plan", y1, shape("footing", "footing", "body", True, 0.55, g_poly(pts), **kw))
    if lv <= 2:
        stroke(sc, "elevation", -z0 + 0.001, "footing-chip", "footing", "detail", [(x1 - CHIP, -0.03), (x1 - CHIP, y1)], el=True)
        stroke(sc, "front", x1 + 0.001, "footing-chip", "footing", "detail", [(z0 + CHIP, -0.03), (z0 + CHIP, y1)])
        stroke(sc, "elevation", -z0 + 0.001, "footing-head-joint", "footing", "detail", [(x0, -0.03), (x1, -0.03)], el=True)
    if lv == 1:
        for k, (a, b) in enumerate(((-0.142, -0.118), (-0.112, -0.094), (-0.088, -0.074))):
            stroke(sc, "elevation", -z0 + 0.002, f"footing-moss-{k}", "footing", "texture", [(a, 0.006), ((a + b) / 2, 0.012), (b, 0.007)], el=True)
        for k, (a, b) in enumerate(((-0.13, -0.06), (0.04, 0.11))):
            stroke(sc, "plan", y1 + 0.002, f"footing-moss-{k}", "footing", "texture", [(-0.135, a), (-0.128, (a + b) / 2), (-0.136, b)])


def left_bag(sc, lv):
    """🛍 The knotted bag on the footing head."""
    cx, cy, cz, rx, ry, rz = BAG
    if lv == 4:
        ry = 0.038
        cy = 0.02 + ry - 0.002
    kw = dict(region="left-bag", material="filled-bag")
    sc.draw("elevation", -(cz - rz), shape("left-bag", "left-bag", "body", True, 0.12, el_ellipse("left-bag", cx, cy, rx, ry), **kw))
    sc.draw("front", cx + rx, shape("left-bag", "left-bag", "body", True, 0.12, g_ellipse(cz, cy, rz, ry), **kw))
    sc.draw("plan", cy + ry, shape("left-bag", "left-bag", "body", True, 0.12, g_ellipse(cx, cz, rx, rz), **kw))
    if lv == 1:
        for k, dx in ((0, -0.016), (1, 0.016)):
            ey = cy + ry + 0.006
            sc.draw("elevation", -(cz - 0.008) + 0.001, shape(f"left-bag-ear-{k}", "left-bag", "detail", True, 0.12, el_ellipse("left-bag", cx + dx, ey, 0.008, 0.012), **kw))
            sc.draw("plan", ey + 0.012, shape(f"left-bag-ear-{k}", "left-bag", "detail", True, 0.12, g_ellipse(cx + dx, cz, 0.008, 0.008), **kw))
        sc.draw("front", cx + 0.008 + 0.001, shape("left-bag-ear-0", "left-bag", "detail", True, 0.12, g_ellipse(cz, cy + ry + 0.006, 0.008, 0.012), **kw))


def post(sc, lv):
    """🪧 Post with its corroded foot, the flaking paint edge and the cap."""
    if lv == 4:
        sc.drawbox("post", "post", "body", 0.38, (-POST_H, POST_H, 0.02, CAP_TOP, -POST_H, POST_H), regions=["post-paint", "post-foot-corroded"], material="steel-powder-green")
        return
    top = POST_TOP if lv == 1 else CAP_TOP
    sc.drawbox("post-foot", "post", "body", 0.3, (-POST_H, POST_H, 0.02, 0.15, -POST_H, POST_H), region="post-foot-corroded", material="steel-corroded", views="ef")
    sc.drawbox("post", "post", "body", 0.38, (-POST_H, POST_H, 0.15, top, -POST_H, POST_H), region="post-paint", material="steel-powder-green", views="ef")
    if lv == 1:
        sc.drawbox("post-cap", "post", "detail", 0.32, (-0.032, 0.032, POST_TOP, CAP_TOP, -0.032, 0.032), region="post-paint", material="steel-powder-green")
    else:
        sc.drawbox("post-top", "post", "body", 0.38, (-POST_H, POST_H, top - 0.01, top, -POST_H, POST_H), region="post-paint", material="steel-powder-green", views="p")
    if lv <= 2:
        edge = [(-0.024, 0.148), (-0.012, 0.156), (0.0, 0.146), (0.01, 0.158), (0.024, 0.151)]
        stroke(sc, "elevation", 0.0301, "paint-edge", "post", "detail", edge, el=True)
        stroke(sc, "front", 0.0301, "paint-edge", "post", "detail", edge)
    if lv == 1:
        for k, (yy, w) in enumerate(((0.045, 0.012), (0.09, 0.016), (0.118, 0.01))):
            ring = [(-w, yy - 0.008), (w, yy - 0.006), (w * 0.8, yy + 0.007), (-w * 0.9, yy + 0.008)]
            stroke(sc, "elevation", 0.0302, f"rust-{k}", "post", "texture", [(x - 0.004, y) for x, y in ring], closed=True, el=True)
            stroke(sc, "front", 0.0302, f"rust-{k}", "post", "texture", [(x + 0.003, y) for x, y in ring], closed=True)


def sign(sc, lv, x0, x1):
    """🐕 The sign: edge on in the side elevation and plan, the faded face with pictogram, text and sticker residue in the front."""
    _, _, y0, y1, z0, z1 = SIGN
    pen = SOLID_PEN[lv]
    kw = dict(region="sign-plate", material="aluminium-sign")
    sc.draw("elevation", -z0, shape("sign-plate", "sign", "body", True, 0.82, el_rect("sign", x0, x1, y0, y1), pen=pen, **kw))
    sc.draw("front", x1, shape("sign-plate", "sign", "body", True, 0.82, g_poly(rounded_rect(z0, z1, y0, y1, 0.015, 3)), **kw))
    sc.draw("plan", y1, shape("sign-plate", "sign", "body", True, 0.82, g_rect(x0, z0, x1 - x0, z1 - z0), pen=pen, **kw))
    if lv <= 3:
        sc.draw("front", x1 + 0.001, shape("sign-border", "sign", "detail", False, 0.5, g_poly(rounded_rect(z0 + 0.014, z1 - 0.014, y0 + 0.014, y1 - 0.014, 0.008, 3)), region="sign-plate"))
    if lv <= 2:
        dog = [(-0.105, 1.775), (-0.06, 1.775), (-0.045, 1.80), (-0.03, 1.80), (-0.02, 1.83), (-0.032, 1.845), (-0.048, 1.84), (-0.05, 1.82), (-0.09, 1.81), (-0.11, 1.825), (-0.115, 1.80)]
        stroke(sc, "front", x1 + 0.002, "sign-dog", "sign", "texture" if lv == 1 else "detail", dog, closed=True)
        bag = [(0.04, 1.775), (0.09, 1.775), (0.095, 1.83), (0.075, 1.83), (0.072, 1.845), (0.058, 1.845), (0.055, 1.83), (0.035, 1.83)]
        stroke(sc, "front", x1 + 0.002, "sign-bag", "sign", "texture" if lv == 1 else "detail", bag, closed=True)
        stroke(sc, "front", x1 + 0.002, "sign-text-0", "sign", "texture" if lv == 1 else "detail", [(-0.11, 1.735), (0.11, 1.735)])
    if lv == 1:
        stroke(sc, "front", x1 + 0.002, "sign-text-1", "sign", "texture", [(-0.09, 1.71), (0.06, 1.71)])
        stroke(sc, "front", x1 + 0.002, "sign-arrow", "sign", "texture", [(-0.01, 1.80), (0.025, 1.80), (0.015, 1.81), (0.025, 1.80), (0.015, 1.79)])
        stroke(sc, "front", x1 + 0.003, "sign-sticker-residue", "sign", "texture", [(0.075, 1.852), (0.118, 1.855), (0.116, 1.885), (0.072, 1.881)], closed=True)
        stroke(sc, "front", x1 + 0.003, "sign-fade", "sign", "texture", [(-0.12, 1.88), (-0.06, 1.865), (0.0, 1.875)])


def clamps(sc, lv, bone, ys, lug):
    """🗜 Band clamps around the post: proud of it in the side elevation and the plan."""
    for k, yc in enumerate(ys):
        b = (-0.032, lug, yc - 0.0125, yc + 0.0125, -0.032, 0.032)
        sc.drawbox(f"{bone}-clamp-{k}", bone, "detail", 0.45, b, region="brackets-galvanised", material="steel-galvanised", views="efp", lift=0.0001)


def dispenser(sc, lv):
    """📦 Dispenser housing, lid, lock, roll and bag tab; at 1:500 one box with its lid."""
    x0, x1, y0, y1, z0, z1 = HOUSE
    pen = SOLID_PEN[lv]
    if lv == 4:
        b = (x0, 0.1685, y0, 1.3515, -0.1235, 0.1235)
        sc.drawbox("dispenser", "dispenser", "body", 0.36, b, regions=["dispenser-housing", "dispenser-lid", "brackets-galvanised"], material="steel-powder-green")
        sc.drawbox("bag-roll", "bag-roll", "body", 0.2, (ROLL_C[0] - ROLL_R, ROLL_C[0] + ROLL_R, ROLL_C[1] - ROLL_R, ROLL_C[1] + ROLL_R, -0.105, 0.105), region="bag-roll", material="bag-film-roll", views="fp", lift=-0.2)
        sc.draw("elevation", 0.105 - 0.2, shape("bag-roll", "bag-roll", "body", True, 0.2, el_ellipse("bag-roll", ROLL_C[0], ROLL_C[1], ROLL_R, ROLL_R), region="bag-roll", material="bag-film-roll"))
        return
    sc.drawbox("dispenser-mount", "dispenser", "detail", 0.45, (0.030, 0.035, 1.06, 1.34, -0.055, 0.055), region="brackets-galvanised", material="steel-galvanised", pen=pen)
    if lv <= 2:
        clamps(sc, lv, "dispenser", (1.09, 1.31), 0.032)
    kw = dict(region="dispenser-housing", material="steel-powder-green")
    sc.drawbox("dispenser-housing", "dispenser", "body", 0.36, HOUSE, views="ef", **kw)
    sc.draw("plan", y0 + WALL, shape("dispenser-inside", "dispenser", "detail", True, 0.15, g_rect(x0 + WALL, z0 + WALL, x1 - x0 - 2 * WALL, z1 - z0 - 2 * WALL), pen=pen, **kw))
    sc.draw("plan", y0 + WALL + 0.0001, shape("dispenser-slot", "dispenser", "detail", True, 0.05, g_rect(SLOT[0], SLOT[2], SLOT[1] - SLOT[0], SLOT[3] - SLOT[2]), pen=pen, **kw))
    sc.draw("plan", y1, shape("dispenser-walls", "dispenser", "body", True, 0.36, g_path([rect_pts(x0, x1, z0, z1), rect_pts(x0 + WALL, x1 - WALL, z0 + WALL, z1 - WALL)]), **kw))
    if lv == 1:
        sc.drawbox("dispenser-lock", "dispenser", "detail", 0.25, (x1, x1 + 0.008, 1.282, 1.298, -0.008, 0.008), views="ep", **kw)
        sc.draw("front", x1 + 0.008, shape("dispenser-lock", "dispenser", "detail", True, 0.25, g_ellipse(0.0, 1.29, 0.008, 0.008), **kw))
        stroke(sc, "front", x1 + 0.001, "dispenser-sticker-0", "dispenser", "texture", [(-0.095, 1.16), (-0.03, 1.162), (-0.031, 1.20), (-0.094, 1.197)], closed=True)
        stroke(sc, "front", x1 + 0.001, "dispenser-sticker-1", "dispenser", "texture", [(0.03, 1.105), (0.085, 1.102), (0.087, 1.122), (0.032, 1.126)], closed=True)
        stroke(sc, "front", x1 + 0.001, "dispenser-bag-sign", "dispenser", "texture", [(0.02, 1.22), (0.06, 1.22), (0.062, 1.26), (0.046, 1.26), (0.045, 1.27), (0.035, 1.27), (0.034, 1.26), (0.018, 1.26)], closed=True)
        stroke(sc, "front", x1 + 0.001, "dispenser-scratch-0", "dispenser", "texture", [(-0.10, 1.08), (-0.04, 1.095)])
        stroke(sc, "front", x1 + 0.001, "dispenser-scratch-1", "dispenser", "texture", [(-0.08, 1.07), (0.0, 1.082)])
        stroke(sc, "elevation", 0.1201, "dispenser-tag", "dispenser", "texture", [(0.05, 1.11), (0.07, 1.15), (0.085, 1.105), (0.10, 1.145), (0.12, 1.115), (0.145, 1.14)], el=True)
    lx0, lx1, lz = 0.035, 0.1685, 0.1235
    lkw = dict(region="dispenser-lid", material="steel-powder-green")
    lid_box = (lx0, lx1, 1.335 if lv <= 2 else 1.35, 1.3515, -lz, lz)
    sc.drawbox("lid", "dispenser-lid", "body", 0.42, lid_box, pen=pen if lv == 3 else None, **lkw)
    if lv == 1:
        hx, hy = LID_HINGE
        hinge = el_ellipse("dispenser-lid", hx, hy, 0.004, 0.004)
        sc.draw("elevation", 0.1001, shape("lid-hinge", "dispenser-lid", "detail", True, 0.3, hinge, **lkw))
        sc.draw("front", hx + 0.004, shape("lid-hinge", "dispenser-lid", "detail", True, 0.3, hinge, solid=swept(-0.10, 0.10), **lkw))
        sc.draw("plan", hy + 0.004, shape("lid-hinge", "dispenser-lid", "detail", True, 0.3, hinge, solid=swept(-0.10, 0.10), **lkw))
    rkw = dict(region="bag-roll", material="bag-film-roll")
    rc = ROLL_C
    sc.draw("elevation", 0.105, shape("bag-roll", "bag-roll", "body", True, 0.2, el_ellipse("bag-roll", rc[0], rc[1], ROLL_R, ROLL_R), **rkw))
    sc.draw("front", rc[0] + ROLL_R, shape("bag-roll", "bag-roll", "body", True, 0.2, g_rect(-0.105, rc[1] - ROLL_R, 0.21, 2 * ROLL_R), **rkw))
    sc.draw("plan", rc[1] + ROLL_R, shape("bag-roll", "bag-roll", "body", True, 0.2, g_rect(rc[0] - ROLL_R, -0.105, 2 * ROLL_R, 0.21), **rkw))
    tpen = 0.25 if lv >= 2 else None
    sc.drawbox("bag-tab", "bag-tab", "detail" if lv <= 2 else "body", 0.25, TAB, pen=tpen, **rkw)


def waste_bin(sc, lv):
    """🗑 Bin body with the throw-in opening and the flap behind it, the hood with lock and hinge; at 1:500 one box with its hood."""
    x0, x1, y0, y1, z0, z1 = BIN
    pen = SOLID_PEN[lv]
    if lv == 4:
        b = (x0, 0.2685, y0, 0.8615, -0.1585, 0.1585)
        sc.drawbox("bin", "bin", "body", 0.6, b, regions=["bin-body", "bin-hood"], material="steel-galvanised")
        return
    sc.drawbox("bin-mount", "bin", "detail", 0.45, (0.030, 0.035, 0.45, 0.82, -0.06, 0.06), region="brackets-galvanised", material="steel-galvanised", pen=pen)
    if lv <= 2:
        clamps(sc, lv, "bin", (0.50, 0.77), 0.032)
    kw = dict(region="bin-body", material="steel-galvanised")
    sc.draw("elevation", -z0, shape("bin-side", "bin", "body", True, 0.6, el_rect("bin", x0, x1, y0, y1), **kw))
    sc.draw("front", x0 + WALL, shape("bin-inside", "bin", "detail", True, 0.15, g_rect(z0 + WALL, y0 + WALL, z1 - z0 - 2 * WALL, y1 - y0 - WALL), pen=pen, **kw))
    ox0, ox1, oy0, oy1 = OPEN
    sc.draw("front", x1, shape("bin-front", "bin", "body", True, 0.6, g_path([rect_pts(z0, z1, y0, y1), rect_pts(ox0, ox1, oy0, oy1)]), **kw))
    sc.draw("plan", y0 + WALL, shape("bin-inside", "bin", "detail", True, 0.15, g_rect(x0 + WALL, z0 + WALL, x1 - x0 - 2 * WALL, z1 - z0 - 2 * WALL), pen=pen, **kw))
    sc.draw("plan", y1, shape("bin-walls", "bin", "body", True, 0.6, g_path([rect_pts(x0, x1, z0, z1), rect_pts(x0 + WALL, x1 - WALL, z0 + WALL, z1 - WALL)]), **kw))
    fx, fy = FLAP_HINGE
    flap = el_rect("throw-flap", 0.252, 0.2535, 0.695, 0.811)
    sc.draw("elevation", 0.115 - 0.05, shape("flap-plate", "throw-flap", "body", True, 0.5, flap, pen=pen, **kw))
    sc.draw("front", 0.2535, shape("flap-plate", "throw-flap", "body", True, 0.5, flap, solid=swept(-0.115, 0.115), **kw))
    sc.draw("plan", 0.805, shape("flap-plate", "throw-flap", "body", True, 0.5, flap, solid=swept(-0.115, 0.115), pen=pen, **kw))
    rod = el_ellipse("throw-flap", fx, fy, 0.005, 0.005)
    sc.draw("elevation", 0.115 - 0.049, shape("flap-rod", "throw-flap", "detail", True, 0.4, rod, **kw))
    if lv == 1:
        tag = [(-0.13, 0.52), (-0.10, 0.58), (-0.08, 0.5), (-0.05, 0.57), (-0.02, 0.51), (0.01, 0.56), (0.03, 0.52), (0.06, 0.55)]
        stroke(sc, "front", x1 + 0.001, "bin-graffiti-0", "bin", "texture", tag)
        stroke(sc, "front", x1 + 0.001, "bin-graffiti-1", "bin", "texture", [(-0.12, 0.49), (-0.02, 0.475), (0.07, 0.49)])
        stroke(sc, "front", x1 + 0.001, "bin-dent", "bin", "texture", [(0.08, 0.42), (0.105, 0.435), (0.13, 0.428), (0.14, 0.415)])
        stroke(sc, "front", x1 + 0.001, "bin-sticker", "bin", "texture", [(0.05, 0.62), (0.12, 0.62), (0.12, 0.665), (0.05, 0.665)], closed=True)
        for k, zz in enumerate((-0.13, -0.08, 0.09, 0.135)):
            stroke(sc, "front", x1 + 0.001, f"bin-streak-{k}", "bin", "texture", [(zz, 0.82), (zz + 0.003, 0.78 - 0.02 * (k % 2))])
        stroke(sc, "elevation", -z0 + 0.001, "bin-dent", "bin", "texture", [(0.22, 0.418), (0.235, 0.44), (0.252, 0.43)], el=True)
        stroke(sc, "elevation", -z0 + 0.001, "bin-streak", "bin", "texture", [(0.10, 0.825), (0.102, 0.77)], el=True)
        stroke(sc, "elevation", -z0 + 0.001, "bin-gum", "bin", "texture", circle(0.19, 0.62, 0.006, 8), closed=True, el=True)
    hx0, hx1, hz = 0.035, 0.2685, 0.1585
    hkw = dict(region="bin-hood", material="steel-galvanised")
    hood_box = (hx0, hx1, 0.835 if lv <= 2 else 0.86, 0.8615, -hz, hz)
    sc.drawbox("hood", "bin-hood", "body", 0.66, hood_box, pen=pen if lv == 3 else None, **hkw)
    if lv == 1:
        hx, hy = HOOD_HINGE
        hinge = el_ellipse("bin-hood", hx, hy, 0.005, 0.005)
        sc.draw("elevation", 0.1201, shape("hood-hinge", "bin-hood", "detail", True, 0.3, hinge, **hkw))
        sc.draw("front", hx + 0.005, shape("hood-hinge", "bin-hood", "detail", True, 0.3, hinge, solid=swept(-0.12, 0.12), **hkw))
        sc.draw("plan", hy + 0.005, shape("hood-hinge", "bin-hood", "detail", True, 0.3, hinge, solid=swept(-0.12, 0.12), **hkw))
        lock = el_rect("bin-hood", hx1, hx1 + 0.010, 0.8385, 0.8565)
        sc.draw("elevation", 0.0091, shape("hood-lock", "bin-hood", "detail", True, 0.25, lock, **hkw))
        sc.draw("front", hx1 + 0.010, shape("hood-lock", "bin-hood", "detail", True, 0.25, lock, solid=swept(-0.009, 0.009), **hkw))
        sc.draw("plan", 0.8565, shape("hood-lock", "bin-hood", "detail", True, 0.25, lock, solid=swept(-0.009, 0.009), **hkw))
