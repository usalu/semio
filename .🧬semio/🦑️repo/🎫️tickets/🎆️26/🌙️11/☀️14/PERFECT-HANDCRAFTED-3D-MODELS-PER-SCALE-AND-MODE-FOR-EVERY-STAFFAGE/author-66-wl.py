from author_48_wllib import *

RF = Ctx("rear-frame", "paint-steel", None, "body", 0.45)
FF = Ctx("front-frame", "paint-steel", None, "body", 0.45)
LA = Ctx("lift-arm", "paint-steel", "lift-arm", "body", 0.4)
BK = Ctx("bucket", "paint-steel", "bucket", "body", 0.3)
WR = Ctx("wheel-r", "rubber", "tyres", "body", 0.12)
WF = Ctx("wheel-f", "rubber", "tyres", "body", 0.12)
HY = Ctx("lift-cyl", "chrome", "hydraulics", "detail", 0.6)
IRON = "cast-iron"

# ---- polygons (bone-local, taken from the 2D drawings)
HOOD = [(-2.6, 0.62), (-2.6, 1.9), (-3.0, 2.4), (-1.1, 2.4), (-0.9, 2.2), (-0.9, 0.62)]
CWT = [(-3.5, 0.62), (-3.5, 2.05), (-3.35, 2.3), (-3.0, 2.4), (-2.6, 1.9), (-2.6, 0.62)]
FFR = [(-0.9, 0.62), (2.35, 0.62), (2.35, 1.2), (1.5, 1.45), (1.5, 2.2), (1.0, 2.2), (1.0, 1.45), (-0.9, 1.45)]
CAB = [(-0.9, 1.45), (-0.9, 3.28), (-0.8, 3.4), (0.7, 3.4), (0.82, 3.32), (0.98, 2.2), (1.0, 1.45)]
ARM = [(-0.25, 0.32), (1, 0.48), (2, 0.38), (2.85, 0.25), (2.85, -0.25), (2, -0.22), (1, -0.28), (-0.25, -0.32)]
BUCKET = [(-0.45, -0.6), (1.5, -0.8), (1.62, -0.62), (1.05, 0.05), (0.15, 0.6), (-0.35, 0.55), (-0.55, 0)]
CWN = [(-3.5, 0.62), (-3.5, 1.0), (-3.2, 1.0), (-3.2, 1.8), (-3.5, 1.8), (-3.5, 2.05), (-3.35, 2.3), (-3.0, 2.4), (-2.6, 1.9), (-2.6, 0.62)]
WINA = [(-0.6, 2.1), (0.0, 2.1), (0.0, 3.2), (-0.6, 3.2)]
WINB = [(0.2, 2.1), (0.78, 2.1), (0.66, 3.2), (0.2, 3.2)]


def tunnel(sid, ctx, poly, w0, w1, wall):
    return ext(sid, ctx, ccw(poly), "xy", w0, w1, holes=[inset(poly, wall)])


def mir(s, a, b):
    s["repeat"] = {"count": 2, "translation": [0, 0, -(a + b)]}
    return s


def sline(p0, p1, th, w0, w1, sid, ctx, outward=1):
    (x0, y0), (x1, y1) = p0, p1
    l = math.hypot(x1 - x0, y1 - y0)
    nx, ny = -(y1 - y0) / l * outward, (x1 - x0) / l * outward
    poly = [(x0, y0), (x1, y1), (x1 + nx * th, y1 + ny * th), (x0 + nx * th, y0 + ny * th)]
    return ext(sid, ctx, ccw(poly), "xy", w0, w1)



D = 0.0001
ALL6 = ["+u", "-u", "+v", "-v", "+w", "-w"]
RIMM = "cast-iron"


def skt(sid, ctx, poly, w0, w1, layer):
    return ext(sid, ctx(layer=layer), ccw(inset(poly, -D)), "xy", w0, w1, holes=[inset(poly, D)])


def skp(sid, ctx, poly, a, b, layer, holes=None, rep=False):
    s_ = ext(sid, ctx(layer=layer), ccw(poly), "xy", a, b, holes=holes)
    return mir(s_, a, b) if rep else s_


def skbeam(prefix, ctx, poly, w0, w1, plate, layer, mirror=True):
    out = [skt(prefix + "-skin", ctx, poly, w0 + plate - 0.001, w1 - plate + 0.001, layer)]
    if mirror:
        mir(out[0], w0 + plate - 0.001, w1 - plate + 0.001)
    out += [skp(prefix + "-skin-o", ctx, poly, w1 - 0.0005, w1 + D, layer, rep=mirror), skp(prefix + "-skin-i", ctx, poly, w0 - D, w0 + 0.0005, layer, rep=mirror)]
    return out


def ref(sid, of, region, layer, **kw):
    return tag(th(sid, of, region, layer), **kw)


def emitter(sid, ctx, profile, plane, a, b, name):
    e = ext(sid, ctx, profile, plane, a, b)
    e["tags"] = [name]
    e["sectionFaces"] = ALL6
    return e


def lugpoly(phase_deg):
    a = math.radians(phase_deg)
    ca, sa = math.cos(a), math.sin(a)
    return [(x * ca - y * sa, x * sa + y * ca) for x, y in ((0.80, -0.07), (0.875, -0.07), (0.875, 0.07), (0.80, 0.07))]


def wheel(bone, ctx, sc):
    c, k = ctx, bone[-1]
    r_body = 0.845 if sc == 50 else 0.875
    out = [mir(rev("tyre-" + k, c, [[-0.3, 0.0], [-0.3, min(0.74, r_body - 0.03)], [-0.26, r_body], [0.26, r_body], [0.3, min(0.74, r_body - 0.03)], [0.3, 0.0]], "w", origin=(0, 0, 1.1)), 1.1, 1.1)]
    if sc <= 200:
        out.append(mir(rev("rim-" + k, c(material=RIMM, role="detail", tone=0.6), [[-0.11, 0.0], [-0.11, 0.5], [0.303, 0.5], [0.303, 0.0]], "w", origin=(0, 0, 1.1)), 1.1, 1.1))
    if sc <= 100:
        out.append(mir(rev("hub-" + k, c(material=RIMM, role="detail", tone=0.3), [[0.0, 0.0], [0.0, 0.14], [0.016 if sc == 100 else 0.01, 0.14], [0.016 if sc == 100 else 0.01, 0.0]], "w", origin=(0, 0, 1.403)), 1.403, 1.403))
    if sc == 50:
        for side, z in (("l", 1.1), ("r", -1.1)):
            sg = 1 if z > 0 else -1
            for row, (lo, hi, ph) in (("a", (0.01, 0.25, 0.0)), ("b", (-0.25, -0.01, 7.5))):
                out.append(ext("lugs-%s-%s-%s" % (k, side, row), c(role="detail", tone=0.05), ccw(lugpoly(ph)), "xy", z + lo, z + hi, repeat={"count": 24, "rotationRad": 0.2618, "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}))
            out.append(rev("bolt-%s-%s" % (k, side), c(material=RIMM, role="detail", tone=0.0), [[0.0, 0.0], [0.0, 0.045], [0.02, 0.045], [0.02, 0.0]], "w", origin=(0.3, 0, 1.413 if sg > 0 else -1.433), repeat={"count": 6, "rotationRad": 1.0472, "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}))
    return out


PLATES = {50: dict(hood=0.008, ff=0.012, arm=0.02, cab=0.008, bk=0.025), 100: dict(hood=0.015, ff=0.015, arm=0.02, cab=0.015, bk=0.025), 200: dict(hood=0.03, ff=0.03, arm=0.03, cab=0.03, bk=0.03), 500: dict(hood=0.075, ff=0.075, arm=0.075, cab=0.075, bk=0.075)}


def build(sc):
    P = PLATES[sc]
    ARMW, FFW = 0.02, 0.012
    body = []
    # rear frame
    body += [tunnel("hood-sheet", RF(region="engine-bay", tone=0.5), HOOD, -(1.0 - P["hood"] + 0.0), 1.0 - P["hood"], 0.002)]
    body += [mir(ext("hood-panel", RF(region="engine-bay", tone=0.5), ccw(HOOD), "xy", 1.0 - P["hood"] - 0.0, 1.0), 1.0 - P["hood"], 1.0)]
    if sc == 50:
        body += [mir(tag(cap("hood-louvres-%d" % i, RF(role="detail", tone=0.3, region="engine-bay"), (-2.3, y, 1.006), (-1.2, y, 1.006), 0.006), slender=True), 1.006, 1.006) for i, y in enumerate((2.1, 1.85))]
    CW = Ctx("rear-frame", IRON, "counterweight", "body", 0.35)
    if sc <= 100:
        body += [ext("counterweight", CW, ccw(CWN), "xy", -0.5, 0.5), mir(ext("counterweight-side", CW, ccw(CWT), "xy", 0.499, 1.1), 0.499, 1.1)]
        if sc == 50:
            body += [box("grille-bar-%d" % i, CW(role="detail", tone=0.2), (-3.46, y - 0.012, -0.501), (-3.4, y + 0.012, 0.501)) for i, y in enumerate((1.1, 1.26, 1.42, 1.58, 1.74)) if y < 1.78]
    else:
        body += [ext("counterweight", CW, ccw(CWT), "xy", -1.1, 1.1)]
    if sc <= 100:
        body += beam("chassis", RF(tone=0.35), [(-3.2, 0.62), (-0.8, 0.62), (-0.8, 1.05), (-3.2, 1.05)], 0.4, 0.55, FFW, plate=max(FFW, P["ff"]))
    if sc == 50:
        body += [cyl("axle-r", RF(tone=0.3), "w", (-1.7, 0.875, 0), 2.1, 0.09)]
    body += [ext("engine-block", Ctx("rear-frame", IRON, None, "detail", 0.3), [[-2.3, 0.85], [-1.4, 0.85], [-1.4, 1.85], [-2.3, 1.85]], "xy", -0.42, 0.42),
             cyl("muffler", Ctx("rear-frame", "exhaust-steel", None, "detail", 0.25), "u", (-1.55, 2.0, -0.9), 0.6, 0.16)]
    if sc <= 100:
        body += [ext("radiator", Ctx("rear-frame", "paint-steel", None, "detail", 0.4), [[-2.55, 0.95], [-2.4, 0.95], [-2.4, 2.0], [-2.55, 2.0]], "xy", -0.8, 0.8),
                 cyl("exhaust-riser", Ctx("rear-frame", "exhaust-steel", None, "detail", 0.25), "v", (-1.23, 2.18, -0.9), 0.36, 0.075)]
    body += [rev("exhaust-stack", Ctx("rear-frame", "exhaust-steel", "exhaust-stack", "body", 0.25), [[2.3, 0.09], [3.15, 0.09], [3.15, 0.0885], [2.3, 0.0885], [2.3, 0.09]], "v", origin=(-1.23, 0, -0.9))]
    rail = RF(role="detail", tone=0.3)
    if sc <= 100:
        body += [tag(cap("hr-post-%d" % i, rail, (x, 2.4, z), (x, 2.62, z), 0.0125 if sc == 50 else 0.0055), slender=(sc == 100)) for i, (x, z) in enumerate([(-2.9, 0.95), (-1.4, 0.95), (-2.9, -0.95), (-1.4, -0.95)])]
        body += [tag(cap("hr-side-l", rail, (-2.9, 2.62, 0.95), (-1.4, 2.62, 0.95), 0.0167), slender=True), tag(cap("hr-side-r", rail, (-2.9, 2.62, -0.95), (-1.4, 2.62, -0.95), 0.0167), slender=True), tag(cap("hr-end", rail, (-2.9, 2.62, -0.95), (-2.9, 2.62, 0.95), 0.0167), slender=True)]
    # front frame
    body += beam("ff-box", FF(region="front-frame"), [(-0.9, 0.62), (2.35, 0.62), (2.35, 1.2), (-0.9, 1.2)], -0.9, 0.9, FFW, mirror=False, plate=max(FFW, P["ff"]))
    body += beam("ff-rail", FF(region="front-frame"), FFR, 0.62, 0.88, FFW, plate=max(FFW, P["ff"]) if P["ff"] < 0.12 else 0.12)
    if sc == 50:
        body += [cyl("axle-f", FF(tone=0.3), "w", (1.7, 0.875, 0), 2.1, 0.09), cyl("articulation-pin", FF(tone=0.3), "v", (-0.75, 1.035, 0), 0.83, 0.07), cyl("lift-pivot", FF(region="front-frame", tone=0.3), "w", (1.1, 2.0, 0), 2.5, 0.07)]
    if sc <= 100:
        body += [box("access-step", FF(region="front-frame", role="detail", tone=0.15), (-0.9, 0.9, -1.0), (-0.5, 1.1, -0.8)),
                 mir(box("head-lamp", Ctx("front-frame", "glass", None, "detail", 0.9), (2.3, 0.95, 0.5), (2.35, 1.15, 0.7)), 0.5, 0.7)]
    cw = P["cab"]
    body += [tunnel("cab-sheet", FF(region="cab", tone=0.55), CAB, -(0.8 - cw), 0.8 - cw, 0.0015)]
    if sc >= 500:
        body += [mir(ext("cab-panel", FF(region="cab", tone=0.55), ccw(CAB), "xy", 0.8 - cw, 0.8, holes=[ccw([(-0.6, 2.1), (0.78, 2.1), (0.66, 3.2), (-0.6, 3.2)])]), 0.8 - cw, 0.8)]
        GL = Ctx("front-frame", "glass", "cab-glazing", "glazing", 0.8)
        body += [mir(ext("cab-glass", GL, ccw([(-0.6, 2.1), (0.78, 2.1), (0.66, 3.2), (-0.6, 3.2)]), "xy", 0.794, 0.8), 0.794, 0.8), sline((0.82, 3.32), (0.98, 2.2), 0.006, -0.65, 0.65, "cab-windscreen", GL)]
    else:
        body += [mir(ext("cab-panel", FF(region="cab", tone=0.55), ccw(CAB), "xy", 0.8 - cw, 0.8, holes=[ccw(WINA), ccw(WINB)]), 0.8 - cw, 0.8)]
        GL = Ctx("front-frame", "glass", "cab-glazing", "glazing", 0.8)
        body += [mir(ext("cab-glass-a", GL, ccw(WINA), "xy", 0.794, 0.8), 0.794, 0.8), mir(ext("cab-glass-b", GL, ccw(WINB), "xy", 0.794, 0.8), 0.794, 0.8), sline((0.82, 3.32), (0.98, 2.2), 0.006, -0.65, 0.65, "cab-windscreen", GL)]
    if sc == 50:
        body += [ext("mirror", FF(region="cab", role="detail", tone=0.2), ccw([(0.85, 3.25), (1.1, 3.25), (1.1, 3.0), (0.95, 3.0)]), "xy", -0.95, -0.8), mir(box("door-handle", FF(region="cab", role="detail", tone=0.2), (-0.2, 2.0, 0.8), (0.2, 2.05, 0.83)), 0.8, 0.83)]
    # lift arm and cylinders
    body += beam("arm", LA, ARM, 0.9, 1.2, ARMW, plate=max(ARMW, P["arm"]))
    if sc == 50:
        body += [cyl("torque-tube", LA(tone=0.35), "w", (1.0, 0.0, 0), 1.8, 0.14), cyl("bucket-pin", LA(tone=0.3), "w", (2.7, 0.0, 0), 2.55, 0.06)]
    body += [mir(cyl("lift-cyl", HY, "u", (0.55, 0, 0.6), 1.1, 0.12), 0.6, 0.6), mir(cyl("lift-rod", Ctx("lift-rod", "chrome", "hydraulics", "detail", 0.7), "u", (0.2282, 0, 0.6), 0.4564, 0.05), 0.6, 0.6),
             cyl("tilt-cyl", Ctx("tilt-cyl", "chrome", "hydraulics", "detail", 0.6), "u", (0.45, 0, 0), 0.9, 0.1), cyl("tilt-rod", Ctx("tilt-rod", "chrome", "hydraulics", "detail", 0.7), "u", (0.134, 0, 0), 0.2679, 0.045)]
    # bucket
    bt = P["bk"]
    PATH = [(1.5, -0.8), (-0.45, -0.6), (-0.55, 0.0), (-0.35, 0.55), (0.15, 0.6)]
    body += [mir(ext("bucket-cheek", BK, ccw(BUCKET), "xy", 1.475 - bt, 1.475), 1.475 - bt, 1.475), ext("bucket-plate", BK, ccw(strip(PATH, bt, side=-1)), "xy", -1.4755 + bt, 1.4755 - bt)]
    ribpath = [(1.35, -0.75), (-0.43, -0.55), (-0.5, 0.0), (-0.32, 0.52)]
    if sc == 50:
        body += [mir(ext("bucket-rib-a", BK, ccw(strip(ribpath, 0.05, side=-1)), "xy", 0.285, 0.305), 0.285, 0.305), mir(ext("bucket-rib-b", BK, ccw(strip(ribpath, 0.05, side=-1)), "xy", 0.875, 0.895), 0.875, 0.895)]
    elif sc == 100:
        body += [mir(ext("bucket-rib-a", BK, ccw(strip(ribpath, 0.05, side=-1)), "xy", 0.728, 0.748), 0.728, 0.748), ext("bucket-rib-b", BK, ccw(strip(ribpath, 0.05, side=-1)), "xy", -0.01, 0.01)]
    if sc >= 50:
        body += [ext("bucket-edge", WS, ccw([(1.3, -0.77), (1.62, -0.62), (1.5, -0.8)]), "xy", -1.475, 1.475)]
    if sc == 50:
        body += [mir(ext("bucket-ear", BK, ccw([(-0.52, -0.2), (0.05, -0.2), (0.2, -0.05), (0.2, 0.1), (0.05, 0.25), (-0.52, 0.3)]), "xy", 1.2, 1.225), 1.2, 1.225),
                 mir(ext("tilt-lug", BK(tone=0.35), ccw([(-0.52, 0.40), (-0.36, 0.45), (-0.30, 0.60), (-0.52, 0.62)]), "xy", 0.05, 0.07), 0.05, 0.07), cyl("tilt-pin", BK(tone=0.3), "w", (-0.45, 0.5, 0), 0.2, 0.03)]
    body += wheel("wheel-r", WR, sc) + wheel("wheel-f", WF, sc)
    return body


def env(sid, ctx, poly, w0, w1, layer, mirror=False):
    s_ = ext(sid, ctx(layer=layer), ccw(poly), "xy", w0, w1)
    return mir(s_, w0, w1) if mirror else s_


def build_thermal(sc, body):
    P = PLATES[sc]
    ids = {b["id"] for b in body}
    t = []
    t += [env("th-hood", RF(region="engine-bay"), HOOD, -1.0, 1.0, "hood")]
    t += [ref("th-hood-louvres-%d" % i, "hood-louvres-%d" % i, "engine-bay", "hood") for i in (0, 1) if "hood-louvres-%d" % i in ids]
    t += [ref("th-engine-block", "engine-block", "engine-bay", "hood"), ref("th-counterweight", "counterweight", "counterweight", "iron")] + ([ref("th-counterweight-side", "counterweight-side", "counterweight", "iron")] if "counterweight-side" in ids else [])
    t += [rev("th-exhaust-stack", Ctx("rear-frame", "exhaust-steel", "exhaust-stack", "body", 0.25)(layer="pipe"), [[2.3, 0.0], [2.3, 0.09], [3.15, 0.09], [3.15, 0.0]], "v", origin=(-1.23, 0, -0.9))]
    th_ = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[sc]
    t += [emitter("th-exhaust-stack-jet", Ctx("rear-frame", "exhaust-steel", "exhaust-stack", "detail", 0.25)(layer="pipe"), ngon(0.065, 24, -0.9, -1.23), "zx", 3.15, 3.15 + th_, "exhaust-stack-jet"),
          emitter("th-radiator-air", Ctx("rear-frame", "paint-steel", "engine-bay", "detail", 0.4)(layer="hood"), ngon(0.4, 32, 1.6, 0.0), "yz", -3.5 - th_ / 2, -3.5 + th_ / 2, "radiator-air")]
    t += [env("th-cab", FF(region="cab"), CAB, -0.8, 0.8, "sheet")]
    if sc >= 500:
        t += [ref("th-cab-glass", "cab-glass", "cab-glazing", "pane"), ref("th-cab-windscreen", "cab-windscreen", "cab-glazing", "pane")]
    else:
        t += [ref("th-cab-glass-a", "cab-glass-a", "cab-glazing", "pane"), ref("th-cab-glass-b", "cab-glass-b", "cab-glazing", "pane"), ref("th-cab-windscreen", "cab-windscreen", "cab-glazing", "pane")]
    t += [env("th-ff-box", FF(region="front-frame"), [(-0.9, 0.62), (2.35, 0.62), (2.35, 1.2), (-0.9, 1.2)], -0.9, 0.9, "plate"), env("th-ff-rail", FF(region="front-frame"), FFR, 0.62, 0.88, "plate", mirror=True)]
    for k in ("access-step", "lift-pivot"):
        if k in ids:
            t.append(ref("th-" + k, k, "front-frame", "plate"))
    t += [env("th-arm", LA, ARM, 0.9, 1.2, "plate", mirror=True)]
    t += [ref("th-" + k, k, "bucket", "plate") for k in ("bucket-cheek", "bucket-plate", "bucket-rib-a", "bucket-rib-b", "bucket-edge", "bucket-ear", "tilt-lug") if k in ids]
    t += [ref("th-" + k, k, "hydraulics", "tube") for k in ("lift-cyl", "lift-rod", "tilt-cyl", "tilt-rod")]
    for b_ in "rf":
        t += [ref("th-tyre-" + b_, "tyre-" + b_, "tyres", "rubber")]
        for sd in ("l", "r"):
            for row in ("a", "b"):
                if "lugs-%s-%s-%s" % (b_, sd, row) in ids:
                    t.append(ref("th-lugs-%s-%s-%s" % (b_, sd, row), "lugs-%s-%s-%s" % (b_, sd, row), "tyres", "rubber"))
            if "bolt-%s-%s" % (b_, sd) in ids:
                t.append(ref("th-bolt-%s-%s" % (b_, sd), "bolt-%s-%s" % (b_, sd), "tyres", "rubber"))
        for k in ("rim", "hub"):
            if "%s-%s" % (k, b_) in ids:
                t.append(ref("th-%s-%s" % (k, b_), "%s-%s" % (k, b_), "tyres", "rubber"))
    return t


def build_structure(sc, body):
    ids = {b["id"] for b in body}
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[sc]
    SM = Ctx("rear-frame", "paint-steel", None, "detail", 0.15)
    st = []
    pairs = [("rear-chassis", "chassis-tube", "rear-frame"), ("hood", "hood-sheet", "rear-frame"), ("counterweight", "counterweight", "rear-frame"), ("front-box", "ff-box-tube", "front-frame"), ("front-rail", "ff-rail-tube", "front-frame"), ("cab", "cab-sheet", "front-frame"),
             ("lift-arm", "arm-tube", "lift-arm"), ("torque-tube", "torque-tube", "lift-arm"), ("bucket", "bucket-plate", "bucket"), ("bucket-cheek", "bucket-cheek", "bucket"), ("lift-cyl", "lift-cyl", "lift-cyl"), ("lift-rod", "lift-rod", "lift-rod"),
             ("tilt-cyl", "tilt-cyl", "tilt-cyl"), ("tilt-rod", "tilt-rod", "tilt-rod"), ("wheel-r", "tyre-r", "wheel-r"), ("wheel-f", "tyre-f", "wheel-f")]
    if sc == 500:
        pairs = [("hood", "hood-sheet", "rear-frame"), ("front-box", "ff-box-tube", "front-frame"), ("lift-arm", "arm-tube", "lift-arm"), ("bucket", "bucket-plate", "bucket"), ("lift-cyl", "lift-cyl", "lift-cyl"), ("lift-rod", "lift-rod", "lift-rod"), ("tilt-cyl", "tilt-cyl", "tilt-cyl"), ("tilt-rod", "tilt-rod", "tilt-rod"), ("wheel-r", "tyre-r", "wheel-r"), ("wheel-f", "tyre-f", "wheel-f")]
    st += [stm("member-" + n, of, bone, "member") for n, of, bone in pairs if of in ids]
    if sc <= 100:
        st += [stm("joint-" + n, of, bone, "joint") for n, of, bone in (("articulation", "articulation-pin", "front-frame"), ("lift-pivot", "lift-pivot", "front-frame"), ("bucket-pin", "bucket-pin", "lift-arm"), ("tilt-pin", "tilt-pin", "bucket"), ("axle-r", "axle-r", "rear-frame"), ("axle-f", "axle-f", "front-frame")) if of in ids]
    m = max(f, 0.0)
    ry = max(0.2, f / 2 + 0.005)
    st += [ell("mass-rear", SM(member="rear-frame", part="mass"), (-2.0, 1.4, 0), (0.7, max(0.5, ry), 0.6)), ell("mass-front", SM(member="front-frame", part="mass", bone="front-frame"), (0.4, 0.9, 0), (0.6, max(0.2, ry), 0.7)),
           mir(ell("mass-wheel-r", SM(member="wheel-r", part="mass", bone="wheel-r"), (0, 0, 1.1), (0.4, 0.4, max(0.2, ry))), 1.1, 1.1), mir(ell("mass-wheel-f", SM(member="wheel-f", part="mass", bone="wheel-f"), (0, 0, 1.1), (0.4, 0.4, max(0.2, ry))), 1.1, 1.1)]
    for k, bone, (x, z) in (("contact-r-l", "rear-frame", (-1.7, 1.1)), ("contact-r-r", "rear-frame", (-1.7, -1.1)), ("contact-f-l", "front-frame", (1.7, 1.1)), ("contact-f-r", "front-frame", (1.7, -1.1))):
        st.append(tag(box("support-" + k, Ctx(bone, "rubber", None, "detail", 0.1)(member=bone, part="support"), (x - 0.4, 0.0, z - 0.3), (x + 0.4, f, z + 0.3)), tags=[k]))
    return st


DEC = {
    50: [
        text("1:50 is the detailed model, built from the drawn profiles of the three views. Rear frame: hood as a 2 mm steel shell (closed tunnel with 8 mm end walls, which is the print feature) with two louvre slats on the side, cast-iron counterweight, a hidden engine block, radiator, muffler with riser, the exhaust stack as a 1.5 mm pipe, a box-beam chassis, the rear axle and a handrail of five slender 33 mm tubes on four posts. Front frame: a closed 12 mm box beam and the two tower rails, articulation pin, lift pivot, step and lamps; cab as a 1.5 mm shell with two window openings per side, 6 mm panes and a raked windscreen, mirror and door handles. Lift arms as 20 mm box beams with torque tube and bucket pin, the lift and tilt cylinders with chrome rods, the bucket as two 25 mm cheeks, a 25 mm scoop plate, ribs, wear edge, ears and tilt lug. Four tyres as revolves with a 30 mm tread depth: 24 staggered chevron lugs per side half so that the side view shows the full tread circle, six wheel bolts, rim and hub. Everything stands at its true size in the carry pose.",
             "1:50 ist das Detailmodell aus den gezeichneten Profilen der drei Ansichten. Hinterwagen: Haube als 2-mm-Stahlschale (geschlossener Tunnel mit 8-mm-Stirnwänden, dem Druckmerkmal) mit zwei Lüftungslamellen an der Seite, Gegengewicht aus Gusseisen, ein verdeckter Motorblock, Kühler, Schalldämpfer mit Steigrohr, das Auspuffrohr als 1,5-mm-Rohr, ein Kastenträger-Fahrgestell, die Hinterachse und ein Handlauf aus fünf schlanken 33-mm-Rohren auf vier Pfosten. Vorderwagen: ein geschlossener 12-mm-Kastenträger und die beiden Turmholme, Knickbolzen, Hubarmlager, Trittstufe und Scheinwerfer; Kabine als 1,5-mm-Schale mit je zwei Fensteröffnungen pro Seite, 6-mm-Scheiben und geneigter Frontscheibe, Spiegel und Türgriffe. Hubarme als 20-mm-Kastenträger mit Torsionsrohr und Schaufelbolzen, Hub- und Kippzylinder mit Chromstangen, die Schaufel aus zwei 25-mm-Wangen, einem 25-mm-Schaufelblech, Rippen, Schneide, Ohren und Kipplasche. Vier Reifen als Rotationskörper mit 30 mm Profiltiefe: je Seitenhälfte 24 versetzte Winkelstollen, sodass die Seitenansicht den vollen Profilkreis zeigt, sechs Radbolzen, Felge und Nabe. Alles steht in der Transportstellung in Originalgröße."),
        text("Thermal: every region has solids at 1:50 and each region has a single layer. Hood, cab, front frame and lift arm are closed envelope solids of their own drawn profile (the outer surface of the steel plate; a thin hollow shell would also count its cavity, which is no surface towards the environment), counterweight, cylinders, bucket parts, panes, tyres with lugs, bolts, rim and hub reference the body solids that carry them, the exhaust stack is a solid revolve, the engine block sits buried in the hood. The stack jet and the radiator air outlet are emitter discs of 7.5 mm. The outer areas are the total outer areas of C29; the pose carries the exposure.",
             "Thermik: Jede Region hat bei 1:50 Körper und jede Region eine einzige Schicht. Haube, Kabine, Vorderwagen und Hubarm sind geschlossene Hüllkörper ihres gezeichneten Profils (die Außenfläche des Stahlblechs; eine dünne hohle Schale würde auch ihren Hohlraum zählen, der keine Fläche zur Umgebung ist), Gegengewicht, Zylinder, Schaufelteile, Scheiben, Reifen mit Stollen, Bolzen, Felge und Nabe verweisen auf die tragenden Körper, das Auspuffrohr ist ein voller Rotationskörper, der Motorblock steckt verdeckt in der Haube. Der Auspuffstrahl und der Kühlerluftaustritt sind Emitterscheiben von 7,5 mm. Die Außenflächen sind die Gesamtflächen nach C29; die Pose trägt die Exposition."),
        text("Structure: the members are the load-bearing hollow solids of the body themselves, referenced one to one (chassis box, hood shell, counterweight, front box and tower rails, cab frame, lift arm, torque tube, bucket scoop and cheek, both cylinders and rods, four wheels), the joints are the articulation pin, lift pivot, bucket and tilt pins and both axles, the masses are ellipsoids on the rear frame, the front frame and the wheels, and four 0.8 x 0.6 m support pads under the tyres stand on the ground plane with their anchors.",
             "Tragwerk: Die Glieder sind die tragenden Hohlkörper des Körpers selbst, eins zu eins referenziert (Fahrgestellkasten, Haubenschale, Gegengewicht, Vorderkasten und Turmholme, Kabinenrahmen, Hubarm, Torsionsrohr, Schaufelblech und Wange, beide Zylinder samt Stangen, vier Räder), die Gelenke sind Knickbolzen, Hubarmlager, Schaufel- und Kippbolzen und beide Achsen, die Massen sind Ellipsoide auf Hinter- und Vorderwagen und den Rädern, und vier Stützflächen von 0,8 x 0,6 m stehen mit ihren Ankern unter den Reifen auf der Bodenebene."),
    ],
    100: [
        text("1:100 is modelled from its own drawings: the hood as a 2 mm shell with 15 mm end walls, counterweight, engine block, radiator, muffler with riser, a plain exhaust pipe, a chassis box beam and the handrail as slender 33 mm tubes; front box beam and tower rails, step and lamp; the cab as a 1.5 mm shell with 15 mm walls, two panes per side and the windscreen; lift arms as 20 mm box beams, four cylinder parts, the bucket as two 25 mm cheeks, a scoop plate, two ribs and the wear edge; four tyres as plain revolves (the tread lugs merge into the circle as drawn) with rim and hub. What 1:50 builds from small parts is merged or dropped here: lugs, bolts, axles, pins, mirror, door handles, louvres and the bucket ears.",
             "1:100 ist aus den eigenen Zeichnungen modelliert: Haube als 2-mm-Schale mit 15-mm-Stirnwänden, Gegengewicht, Motorblock, Kühler, Schalldämpfer mit Steigrohr, ein einfaches Auspuffrohr, ein Fahrgestell-Kastenträger und der Handlauf aus schlanken 33-mm-Rohren; Vorderkastenträger und Turmholme, Trittstufe und Scheinwerfer; die Kabine als 1,5-mm-Schale mit 15-mm-Wänden, zwei Scheiben pro Seite und Frontscheibe; Hubarme als 20-mm-Kastenträger, vier Zylinderteile, die Schaufel aus zwei 25-mm-Wangen, einem Schaufelblech, zwei Rippen und der Schneide; vier Reifen als einfache Rotationskörper (die Stollen gehen wie gezeichnet im Kreis auf) mit Felge und Nabe. Was 1:50 aus kleinen Teilen baut, ist hier zusammengefasst oder entfällt: Stollen, Bolzen, Achsen, Zapfen, Spiegel, Türgriffe, Lüftungsschlitze und Schaufelohren."),
        text("Thermal: the same closed envelope solids as at 1:50 (hood, cab, front frame, lift arm), exhaust stack as a solid revolve and references to the body solids for counterweight, panes, cylinders, bucket, tyres, rim, hub and step; emitter discs of 15 mm at the stack and the radiator. The outer areas agree with 1:50 within 8 % except the tyres, which have no lugs here (30.6 against 46.2 m2).",
             "Thermik: Dieselben geschlossenen Hüllkörper wie bei 1:50 (Haube, Kabine, Vorderwagen, Hubarm), das Auspuffrohr als voller Rotationskörper und Verweise auf die Körper für Gegengewicht, Scheiben, Zylinder, Schaufel, Reifen, Felge, Nabe und Stufe; Emitterscheiben von 15 mm am Rohr und am Kühler. Die Außenflächen stimmen mit 1:50 auf 8 % überein, außer den Reifen, die hier keine Stollen haben (30,6 gegen 46,2 m2)."),
        text("Structure: members referencing the hollow beams, hood, cab, lift arm, bucket, the four cylinder parts and the wheels, mass ellipsoids on both frames and the wheels and four contact pads with anchors; the joints are merged into the members at this scale.",
             "Tragwerk: Glieder, die die hohlen Träger, Haube, Kabine, Hubarm, Schaufel, die vier Zylinderteile und die Räder referenzieren, Massenellipsoide auf beiden Rahmen und den Rädern sowie vier Aufstandsflächen mit Ankern; die Gelenke gehen bei diesem Maßstab in den Gliedern auf."),
    ],
    200: [
        text("1:200 is modelled anew from its drawings as merged masses: hood and counterweight as two separate masses, a hidden engine block, front box beam with its tower rails, the cab as a 30 mm shell with two panes and the windscreen, the cranked lift arm as one beam, four cylinder parts at their real widths, the bucket as two cheeks and a scoop plate, and each tyre as one revolve with a rim disc. The handrail, step, lamp, radiator, riser, bolts, lugs, pins, axles and mirror are merged into the masses around them; the 2 mm and 1.5 mm sheets are thinner than the 30 mm feature but their hollow bodies are larger than it, so they stay hollow shells with end walls of 30 mm.",
             "1:200 ist neu aus den eigenen Zeichnungen als zusammengefasste Massen modelliert: Haube und Gegengewicht als zwei getrennte Massen, ein verdeckter Motorblock, Vorderkastenträger mit Turmholmen, die Kabine als 30-mm-Schale mit zwei Scheiben und Frontscheibe, der gekröpfte Hubarm als ein Träger, vier Zylinderteile in Realbreite, die Schaufel aus zwei Wangen und einem Schaufelblech und jeder Reifen als ein Rotationskörper mit Felgenscheibe. Handlauf, Stufe, Leuchte, Kühler, Steigrohr, Bolzen, Stollen, Zapfen, Achsen und Spiegel gehen in den umgebenden Massen auf; die 2-mm- und 1,5-mm-Bleche sind dünner als das 30-mm-Merkmal, ihre Hohlkörper sind aber größer, sodass sie hohle Schalen mit 30-mm-Stirnwänden bleiben."),
        text("Thermal: closed envelope solids for hood, cab, front frame and lift arm, references for counterweight, panes, cylinders, bucket and tyres, a solid exhaust stack and 30 mm emitter discs. Each region has a solid; only the total area is compared at this scale.",
             "Thermik: Geschlossene Hüllkörper für Haube, Kabine, Vorderwagen und Hubarm, Verweise für Gegengewicht, Scheiben, Zylinder, Schaufel und Reifen, ein voller Auspuffrohr-Körper und 30-mm-Emitterscheiben. Jede Region hat einen Körper; bei diesem Maßstab wird nur die Gesamtfläche verglichen."),
        text("Structure: hood, counterweight, box beam and rails, cab, lift arm and bucket, the cylinders and rods and the wheels as members referencing the body solids, mass ellipsoids and four contact pads with anchors; no joints.",
             "Tragwerk: Haube, Gegengewicht, Kastenträger und Holmen, Kabine, Hubarm und Schaufel, die Zylinder samt Stangen und die Räder als Glieder, die die Körper referenzieren, Massenellipsoide und vier Aufstandsflächen mit Ankern; keine Gelenke."),
    ],
    500: [
        text("1:500 is modelled anew from its drawings: one rear mass (hood shell with 75 mm end walls and the counterweight), one front body (box beam and tower rails), the cab with a single glass band and the windscreen, the lift arm as one cranked beam with its four cylinder parts, the bucket as one scoop (two cheeks and a plate) and the two tyres as plain discs; the 75 mm feature merges rims, hubs, handrail, step, lamp, radiator, mirror and the arm joints. The stack stays a true 0.18 m pipe because the 2D draws it. The 2D omits the cylinders; they stay in the model at their true widths because the hydraulics region and the load path need them, so the tilt cylinder between the arms shows as 3D-only in the plan.",
             "1:500 ist neu aus den eigenen Zeichnungen modelliert: eine Hinterwagenmasse (Haubenschale mit 75-mm-Stirnwänden und das Gegengewicht), ein Vorderwagenkörper (Kastenträger und Turmholme), die Kabine mit einem einzigen Glasband und der Frontscheibe, der Hubarm als ein gekröpfter Träger mit seinen vier Zylinderteilen, die Schaufel als eine Schaufel (zwei Wangen und ein Blech) und die beiden Reifen als einfache Scheiben; das 75-mm-Merkmal fasst Felgen, Naben, Handlauf, Stufe, Leuchte, Kühler, Spiegel und die Armgelenke zusammen. Das Rohr bleibt ein echtes 0,18-m-Rohr, weil es die 2D zeichnet. Die 2D lässt die Zylinder weg; im Modell bleiben sie in Realbreite, weil die Hydraulikregion und der Kraftfluss sie brauchen, sodass der Kippzylinder zwischen den Armen in der Draufsicht nur im 3D erscheint."),
        text("Thermal: closed envelope solids for hood, cab, front frame and lift arm, references for counterweight, glass, cylinders, bucket and tyres, a solid stack and 75 mm emitter discs; only the total area is compared and it equals the 1:200 total within 2 %.",
             "Thermik: Geschlossene Hüllkörper für Haube, Kabine, Vorderwagen und Hubarm, Verweise für Gegengewicht, Glas, Zylinder, Schaufel und Reifen, ein voller Rohrkörper und 75-mm-Emitterscheiben; nur die Gesamtfläche wird verglichen und sie gleicht der von 1:200 auf 2 %."),
        text("Structure: hood, box beam, lift arm, bucket, four cylinder parts and two wheels as members, four mass ellipsoids and four contact pads with anchors.",
             "Tragwerk: Haube, Kastenträger, Hubarm, Schaufel, vier Zylinderteile und zwei Räder als Glieder, vier Massenellipsoide und vier Aufstandsflächen mit Ankern."),
    ],
}
SCALES = {}
for _sc in (50, 100, 200, 500):
    _b = build(_sc)
    SCALES[_sc] = dict(body=_b, thermal=build_thermal(_sc, _b), structure=build_structure(_sc, _b), decisions=DEC[_sc])
import os
if __name__ == "__main__":
    only = os.environ.get("ONLY")
    write("wheel-loader", {k: v for k, v in SCALES.items() if not only or str(k) in only.split(",")}, category="machinery")
