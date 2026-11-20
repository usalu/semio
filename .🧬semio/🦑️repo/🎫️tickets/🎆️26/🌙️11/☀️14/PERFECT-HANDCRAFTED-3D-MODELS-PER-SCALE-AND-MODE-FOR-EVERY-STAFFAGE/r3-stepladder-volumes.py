"""🧊 Round 3 stepladder volumes: body, thermal and structure of every scale, each list stated part by part from the hand-stated geometry of `r3-stepladder-kit.py` (1:50 full construction, 1:100 merged fittings, 1:200 and 1:500 true-size slender rails with treads at the feature size)."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("stepladder_kit", os.path.join(T, "r3-stepladder-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

AL, ST, RB = "alu-mill", "steel-zinc", "rubber-black"
F, R = k.FRONT, k.REAR
Z = [0.0, 0.0, 1.0]
REGION = {"stile": "front-rails", "grab": "front-rails", "tread": "treads", "platform": "platform", "rail": "rear-frame", "rest": "rear-frame", "brace": "rear-frame", "hinge": "rear-frame", "spreader": "spreaders", "shoe": "feet"}


def T2(en, de):
    return {"en": en, "de": de}


def region_of(sid):
    for prefix, region in REGION.items():
        if sid.startswith(prefix):
            return region
    raise KeyError(sid)


def thermal_of(body):
    """🌡️ Every region is the skin of its own body solids (one layer each, so no nested shells); sub-feature rivet heads carry no area."""
    return [{"id": "th-" + s["id"], "of": s["id"], "region": region_of(s["id"])} for s in body if not s.get("texture")]


def member(s, bone, part="member", **kw):
    s.update({"member": bone, "part": part})
    s.update(kw)
    return s


def ref(sid, bone, part="member"):
    return {"id": "st-" + sid, "of": sid, "member": bone, "part": part}


def stile(sid, sg, y0, y1, material=AL, tone=0.55, w=None, d=None, r=0.003, hole=None, **kw):
    p0, p1 = k.stile_points(sg, y0, y1)
    w, d = w or k.STILE[1], d or k.STILE[0]
    holes = [k.rrect(d - 2 * hole, w - 2 * hole, max(r - hole, 0.0008))] if hole else None
    return k.bar(sid, F, material, "body", tone, k.rrect(d, w, r), p0, p1, k.FN3, holes, **kw)


def rail(sid, sg, y0, y1, tone=0.5, r=0.003, hole=None, **kw):
    p0, p1 = k.rail_points(sg, y0, y1)
    w, d = k.RAIL[1], k.RAIL[0]
    holes = [k.rrect(d - 2 * hole, w - 2 * hole, max(r - hole, 0.0008))] if hole else None
    return k.bar(sid, R, AL, "body", tone, k.rrect(d, w, r), p0, p1, k.RN3, holes, **kw)


def cross_bar(sid, bone, y, x, half, size, along, tone=0.5, r=0.002, hole=None, material=AL):
    """➖ A rectangular cross member along z at the world point (x, y), its section turned to the world direction `along`."""
    holes = [k.rrect(size[0] - 2 * hole, size[1] - 2 * hole, 0.0008)] if hole else None
    return k.bar(sid, bone, material, "body", tone, k.rrect(size[0], size[1], r), [x, y, -half], [x, y, half], along, holes)


def link(sid, bone, x0, x1, zc, tone=0.35, channel=True, **kw):
    """🔗 One pressed-steel spreader link (U channel 20 x 8 x 2, its web against the frame) between two world x positions at the spreader height."""
    h, w, t = k.LINK
    if channel:
        prof = [[-h / 2, -w / 2], [h / 2, -w / 2], [h / 2, w / 2], [h / 2 - t, w / 2], [h / 2 - t, -w / 2 + t], [-h / 2 + t, -w / 2 + t], [-h / 2 + t, w / 2], [-h / 2, w / 2]]
    else:
        prof = k.rrect(h, w, 0.002)
    lo, hi = min(x0, x1), max(x0, x1)
    return k.bar(sid, bone, ST, "body", tone, prof, [lo - 0.012, k.SPREADER_Y, zc], [hi + 0.012, k.SPREADER_Y, zc], [0.0, 1.0, 0.0], **kw)


def rivet(sid, bone, x, y, z0, z1, r, head, tone=0.3, material=ST, **kw):
    """📌 A pin along z from z0 to z1 with a domed head of radius `head` at each end."""
    lo, hi = min(z0, z1), max(z0, z1)
    n = hi - lo
    prof = [[0.0, 0.0], [0.0, head * 0.7], [0.003, head], [0.008, head], [0.008, r], [n - 0.008, r], [n - 0.008, head], [n - 0.003, head], [n, head * 0.7], [n, 0.0]]
    return k.turned(sid, bone, material, "detail", tone, prof, [x, y, lo], Z, **kw)


def shoe(sid, bone, sg, front, tone=0.12, height=None, half=None, sole=None):
    h, w, _ = k.SHOE_F if front else k.SHOE_R
    zc = k.fz(0.0) if front else k.rz(0.0)
    half = half or w
    return k.slab(sid, bone, RB, "body", tone, k.shoe_profile(front, height, sole=sole), sg * zc - half, sg * zc + half)


def pad(anchor, bone, front, sg, thick):
    """🟩 Contact pad of one foot anchor: the sole patch of the shoe on the ground (B3)."""
    xc = k.XF if front else k.XR
    zc = sg * (k.fz(0.0) if front else k.rz(0.0))
    length, width = (0.088, 0.038) if front else (0.076, 0.032)
    if thick > 0.02:
        zc = sg * k.coarse_shoe_z(front, 0.075 if thick > 0.05 else 0.045)
    s = k.block("support-" + anchor, bone, RB, "ground", 0.15, [length, thick, width], [xc, thick / 2, zc], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
    return member(s, bone, "support", anchor=anchor)


def supports(thick):
    out = []
    for side, sg in k.SIDES:
        name = "left" if sg > 0 else "right"
        out.append(pad("foot-front-" + name, F, True, sg, thick))
        out.append(pad("foot-rear-" + name, R, False, sg, thick))
    return out


def mass_solids(com, radii_f, radii_r):
    """🟣 The bone masses as compact solids on the stiles and rails at the height of each centre of mass (half per side)."""
    out = []
    yf, yr = com[F][4], com[R][4]
    for side, sg in k.SIDES:
        out.append(member(k.ball("mass-front-" + side, F, AL, "detail", 0.2, radii_f, [k.fx(yf), yf, sg * k.fz(yf)]), F, "mass"))
        out.append(member(k.ball("mass-rear-" + side, R, AL, "detail", 0.2, radii_r, [k.rx(yr), yr, sg * k.rz(yr)]), R, "mass"))
    return out


def body50():
    out = []
    for side, sg in k.SIDES:
        out.append(stile("stile-" + side, sg, k.STILE_Y0, k.STILE_Y1))
        d = k.stile_dir(sg)
        top = k.stile_points(sg, 0.0, k.STILE_Y1)[1]
        out.append(k.block("stile-cap-" + side, F, AL, "detail", 0.35, [k.CAP + 0.008, k.STILE[0] + 0.004, k.STILE[1] + 0.004], k.add(top, d, k.CAP / 2 - 0.004), d, k.FN3, 0.004))
    half = k.fz(k.BAR_Y) - k.STILE[1] / 2 + 0.003
    out.append(k.rod("grab-bar", F, AL, "body", 0.5, [k.fx(k.BAR_Y), k.BAR_Y, -half], [k.fx(k.BAR_Y), k.BAR_Y, half], k.BAR_R))
    for i, y in enumerate(k.TREAD_Y):
        h = k.tread_half(y)
        out.append(k.slab("tread-%d" % (i + 1), F, AL, "body", 0.6, k.tread_profile(y), -h, h))
    for side, sg in k.SIDES:
        d = k.stile_dir(sg)
        step = [c * k.PITCH / d[1] for c in d]
        for j, off in enumerate((0.018, -0.016)):
            y = k.TREAD_Y[0] - k.TREAD[1] / 2
            zo = sg * (k.fz(y) + k.STILE[1] / 2)
            out.append(k.turned("stile-rivet-%s-%d" % (side, j + 1), F, AL, "texture", 0.3, [[0.0, 0.0], [0.0, 0.005], [0.002, 0.005], [0.003, 0.003], [0.003, 0.0]], [k.fx(y) + off, y, zo - (0.0005 if sg > 0 else -0.0005)], [0.0, 0.0, float(sg)], texture=True, repeat={"count": k.TREADS, "translation": k.r6(k.ldir(F, step))}))
    out.append(k.slab("platform-deck", F, AL, "body", 0.6, k.deck_profile(), -k.PLATFORM_HALF, k.PLATFORM_HALF))
    for side, sg in k.SIDES:
        z0, z1 = sorted((sg * (k.PLATFORM_HALF - k.CHEEK), sg * k.PLATFORM_HALF))
        out.append(k.slab("platform-cheek-" + side, F, AL, "body", 0.5, k.cheek_profile(), z0, z1))
    za = k.fz(k.AXLE_Y) - k.STILE[1] / 2 + 0.003
    zs = k.fz(k.AXLE_Y) - k.STILE[1] / 2
    axle = [[-za, 0.0], [-za, k.AXLE_R], [-zs, k.AXLE_R], [-zs, 0.009], [-k.PLATFORM_HALF, 0.009], [-k.PLATFORM_HALF, k.AXLE_R], [k.PLATFORM_HALF, k.AXLE_R], [k.PLATFORM_HALF, 0.009], [zs, 0.009], [zs, k.AXLE_R], [za, k.AXLE_R], [za, 0.0]]
    out.append(k.turned("platform-axle", F, ST, "detail", 0.3, axle, [k.AXLE_X, k.AXLE_Y, 0.0], Z))
    for side, sg in k.SIDES:
        out.append(link("spreader-front-" + side, F, k.KNUCKLE_X, k.SPREADER_X[0], sg * k.LINK_ZF))
        zi = sg * (k.LINK_ZF - k.LINK[1] / 2 - 0.004)
        out.append(rivet("spreader-pivot-front-" + side, F, k.SPREADER_X[0], k.SPREADER_Y, zi, sg * (k.fz(k.SPREADER_Y) - k.STILE[1] / 2 + 0.008), 0.004, k.RIVET_R))
        out.append(rivet("spreader-knuckle-" + side, F, k.KNUCKLE_X, k.SPREADER_Y, zi, sg * (k.LINK_ZR + k.LINK[1] / 2 + 0.004), 0.004, k.RIVET_R + 0.001))
        out.append(shoe("shoe-front-" + side, F, sg, True))
    for side, sg in k.SIDES:
        out.append(rail("rail-" + side, sg, 0.028, k.H_HINGE))
        zc = sg * k.ZR_TOP
        out.append(k.rod("rail-cap-" + side, R, AL, "detail", 0.35, [k.HX, k.H_HINGE, zc - k.RAIL[1] / 2 - 0.001], [k.HX, k.H_HINGE, zc + k.RAIL[1] / 2 + 0.001], k.RAIL[0] / 2))
    half = k.rz(k.REST_Y) - k.RAIL[1] / 2 + 0.003
    out.append(k.rod("rest-tube", R, AL, "body", 0.5, [k.REST_X, k.REST_Y, -half], [k.REST_X, k.REST_Y, half], k.REST_R))
    for i, y in enumerate(k.BRACE_Y):
        out.append(cross_bar("brace-%d" % (i + 1), R, y, k.rx(y), k.rz(y) - k.RAIL[1] / 2 + 0.003, k.BRACE, [k.R_DIR[0], k.R_DIR[1], 0.0]))
    for side, sg in k.SIDES:
        zi = sg * (k.ZR_TOP - k.RAIL[1] / 2 - 0.008)
        zo = sg * (k.fz(k.H_HINGE) + k.STILE[1] / 2 + 0.008)
        out.append(rivet("hinge-bolt-" + side, R, k.HX, k.H_HINGE, zi, zo, 0.004, 0.009))
        out.append(link("spreader-rear-" + side, R, k.SPREADER_X[1], k.KNUCKLE_X, sg * k.LINK_ZR))
        out.append(rivet("spreader-pivot-rear-" + side, R, k.SPREADER_X[1], k.SPREADER_Y, sg * (k.rz(k.SPREADER_Y) + k.RAIL[1] / 2 - 0.008), sg * (k.LINK_ZR + k.LINK[1] / 2 + 0.004), 0.004, k.RIVET_R))
        out.append(shoe("shoe-rear-" + side, R, sg, False))
    return out


def structure50(com):
    out = []
    for side, sg in k.SIDES:
        out.append(member(stile("member-stile-" + side, sg, k.STILE_Y0, k.STILE_Y1, hole=k.STILE[2]), F))
        out.append(member(rail("member-rail-" + side, sg, 0.028, k.H_HINGE, hole=k.RAIL[2]), R))
    for i, y in enumerate(k.TREAD_Y):
        h = k.tread_half(y)
        out.append(member(k.slab("member-tread-%d" % (i + 1), F, AL, "body", 0.6, k.tread_box(y, 0.0), -h, h, [k.tread_box(y, 0.002)]), F))
    for side, sg in k.SIDES:
        d = k.stile_dir(sg)
        step = [c * k.PITCH / d[1] for c in d]
        y = k.TREAD_Y[0] - k.TREAD[1] / 2
        joint = k.ball("joint-tread-" + side, F, AL, "detail", 0.25, [0.012, 0.012, 0.012], [k.fx(y), y, sg * (k.fz(y) - k.STILE[1] / 2)], repeat={"count": k.TREADS, "translation": k.r6(k.ldir(F, step))})
        out.append(member(joint, F, "joint"))
    half = k.fz(k.BAR_Y) - k.STILE[1] / 2 + 0.003
    out.append(member(k.tube("member-grab-bar", F, AL, "body", 0.5, [k.fx(k.BAR_Y), k.BAR_Y, -half], [k.fx(k.BAR_Y), k.BAR_Y, half], k.BAR_R, 0.0015), F))
    out.append(ref("platform-deck", F))
    out.append(ref("platform-axle", F))
    half = k.rz(k.REST_Y) - k.RAIL[1] / 2 + 0.003
    out.append(member(k.tube("member-rest-tube", R, AL, "body", 0.5, [k.REST_X, k.REST_Y, -half], [k.REST_X, k.REST_Y, half], k.REST_R, 0.0015), R))
    for i, y in enumerate(k.BRACE_Y):
        out.append(member(cross_bar("member-brace-%d" % (i + 1), R, y, k.rx(y), k.rz(y) - k.RAIL[1] / 2 + 0.003, k.BRACE, [k.R_DIR[0], k.R_DIR[1], 0.0], hole=0.0012), R))
    for side, sg in k.SIDES:
        out.append(ref("spreader-front-" + side, F))
        out.append(ref("spreader-rear-" + side, R))
        out.append(ref("spreader-pivot-front-" + side, F, "joint"))
        out.append(ref("spreader-knuckle-" + side, F, "joint"))
        out.append(ref("spreader-pivot-rear-" + side, R, "joint"))
        out.append(ref("hinge-bolt-" + side, R, "joint"))
        out.append(member(k.ball("joint-axle-" + side, F, ST, "detail", 0.25, [0.011, 0.011, 0.011], [k.AXLE_X, k.AXLE_Y, sg * (k.fz(k.AXLE_Y) - k.STILE[1] / 2)]), F, "joint"))
        out.append(member(k.ball("joint-rest-" + side, R, AL, "detail", 0.25, [0.013, 0.013, 0.013], [k.REST_X, k.REST_Y, sg * (k.rz(k.REST_Y) - k.RAIL[1] / 2)]), R, "joint"))
    out += supports(0.012)
    out += mass_solids(com, [0.035, 0.024, 0.016], [0.03, 0.02, 0.015])
    return out


def body100():
    out = []
    for side, sg in k.SIDES:
        out.append(stile("stile-" + side, sg, 0.03, k.H_TOP - 0.03 * k.COS_F, r=0.004))
        out.append(rail("rail-" + side, sg, 0.03, k.H_HINGE, r=0.004))
    half = k.fz(k.BAR_Y) - k.STILE[1] / 2 + 0.003
    out.append(k.rod("grab-bar", F, AL, "body", 0.5, [k.fx(k.BAR_Y), k.BAR_Y, -half], [k.fx(k.BAR_Y), k.BAR_Y, half], k.BAR_R))
    for i, y in enumerate(k.TREAD_Y):
        h = k.tread_half(y)
        out.append(k.slab("tread-%d" % (i + 1), F, AL, "body", 0.6, k.tread_box(y, 0.0), -h, h))
    yt, yb = k.H_PLATFORM, k.H_PLATFORM - k.DECK[2]
    out.append(k.slab("platform", F, AL, "body", 0.6, [[k.DECK_X0, yb], [k.DECK_X1 - 0.006, yb], [k.DECK_X1, yb + 0.006], [k.DECK_X1, yt], [k.DECK_X0, yt]], -k.PLATFORM_HALF, k.PLATFORM_HALF))
    half = k.rz(k.REST_Y) - k.RAIL[1] / 2 + 0.003
    out.append(k.rod("rest-tube", R, AL, "body", 0.5, [k.REST_X, k.REST_Y - 0.004, -half], [k.REST_X, k.REST_Y - 0.004, half], k.REST_R))
    for i, y in enumerate(k.BRACE_Y):
        out.append(cross_bar("brace-%d" % (i + 1), R, y, k.rx(y), k.rz(y) - k.RAIL[1] / 2 + 0.003, k.BRACE, [k.R_DIR[0], k.R_DIR[1], 0.0], r=0.003))
    for side, sg in k.SIDES:
        zi = sg * (k.ZR_TOP - k.RAIL[1] / 2 - 0.004)
        zo = sg * (k.fz(k.H_HINGE) + k.STILE[1] / 2 + 0.006)
        out.append(k.rod("hinge-" + side, R, ST, "detail", 0.3, [k.HX, k.H_HINGE, zi], [k.HX, k.H_HINGE, zo], 0.009))
        out.append(link("spreader-" + side, F, k.SPREADER_X[1], k.SPREADER_X[0], sg * (k.LINK_ZF + k.LINK_ZR) / 2, channel=False, slender=True))
        out.append(shoe("shoe-front-" + side, F, sg, True, sole=0.015))
        out.append(shoe("shoe-rear-" + side, R, sg, False, sole=0.015))
    return out


def structure100(com):
    out = []
    for side, sg in k.SIDES:
        out.append(member(stile("member-stile-" + side, sg, 0.03, k.H_TOP - 0.03 * k.COS_F, r=0.004, hole=0.003), F))
        out.append(member(rail("member-rail-" + side, sg, 0.03, k.H_HINGE, r=0.004, hole=0.003), R))
        out.append(ref("spreader-" + side, F))
        out.append(member(k.ball("joint-hinge-" + side, R, ST, "detail", 0.25, [0.016, 0.016, 0.016], [k.HX, k.H_HINGE, sg * (k.ZR_TOP + k.RAIL[1] / 2)]), R, "joint"))
        out.append(member(k.ball("joint-rest-" + side, R, AL, "detail", 0.25, [0.015, 0.015, 0.015], [k.REST_X, k.REST_Y - 0.004, sg * (k.rz(k.REST_Y) - k.RAIL[1] / 2)]), R, "joint"))
    for i, y in enumerate(k.TREAD_Y):
        h = k.tread_half(y)
        out.append(member(k.slab("member-tread-%d" % (i + 1), F, AL, "body", 0.6, k.tread_box(y, 0.0), -h, h, [k.tread_box(y, 0.004)]), F))
    out.append(ref("grab-bar", F))
    out.append(ref("platform", F))
    out.append(ref("rest-tube", R))
    out.append(ref("brace-1", R))
    out.append(ref("brace-2", R))
    out += supports(0.015)
    out += mass_solids(com, [0.04, 0.026, 0.018], [0.035, 0.022, 0.017])
    return out


def coarse_body(feature, bar, treads_thick, shoes, brace):
    """🪜 1:200 and 1:500: stiles, rails and spreaders are slender parts at their true size (C24), the other parts are thickened to the smallest feature with their tops at the true heights."""
    out = []
    for side, sg in k.SIDES:
        out.append(stile("stile-" + side, sg, 0.012, k.H_TOP - 0.0115, r=0.004, slender=True))
        out.append(rail("rail-" + side, sg, 0.01, k.H_HINGE - 0.012, r=0.004, slender=True))
    half = k.fz(k.BAR_Y) - k.STILE[1] / 2 + 0.003
    yc = k.H_TOP - bar[1] / 2 - 0.02
    out.append(cross_bar("grab-bar", F, yc, k.fx(yc), half, bar, [1.0, 0.0, 0.0], r=0.006))
    for i, y in enumerate(k.TREAD_Y):
        h = k.tread_half(y, treads_thick)
        xc = k.fx(y - k.TREAD[1] / 2)
        x1 = xc + k.TREAD_FRONT
        out.append(k.slab("tread-%d" % (i + 1), F, AL, "body", 0.6, [[x1 - k.TREAD[0], y - treads_thick], [x1, y - treads_thick], [x1, y], [x1 - k.TREAD[0], y]], -h, h))
    yt = k.H_PLATFORM
    out.append(k.slab("platform", F, AL, "body", 0.6, [[k.DECK_X0, yt - feature], [k.DECK_X1, yt - feature], [k.DECK_X1, yt], [k.DECK_X0, yt]], -k.PLATFORM_HALF, k.PLATFORM_HALF))
    if brace:
        y = k.BRACE_Y[0]
        out.append(cross_bar("brace", R, y, k.rx(y), k.rz(y) - k.RAIL[1] / 2 + 0.003, (feature, feature), [k.R_DIR[0], k.R_DIR[1], 0.0], r=0.004))
    length, height, width = shoes
    for side, sg in k.SIDES:
        out.append(link("spreader-" + side, F, k.SPREADER_X[1], k.SPREADER_X[0], sg * (k.LINK_ZF + k.LINK_ZR) / 2, channel=False, slender=True))
        for name, bone, xc, zc in (("front", F, k.XF, k.coarse_shoe_z(True, width)), ("rear", R, k.XR, k.coarse_shoe_z(False, width))):
            out.append(k.slab("shoe-%s-%s" % (name, side), bone, RB, "body", 0.12, [[xc - length / 2, 0.0], [xc + length / 2, 0.0], [xc + length / 2, height], [xc - length / 2, height]], sg * zc - width / 2, sg * zc + width / 2))
    return out


def coarse_structure(com, joint_r, thick, mass_f, mass_r, brace):
    out = []
    for side, sg in k.SIDES:
        out.append(ref("stile-" + side, F))
        out.append(ref("rail-" + side, R))
        out.append(ref("spreader-" + side, F))
        out.append(member(k.ball("joint-hinge-" + side, R, ST, "detail", 0.25, [joint_r] * 3, [k.HX, k.H_HINGE, sg * (k.ZR_TOP + k.RAIL[1] / 2)]), R, "joint"))
    for i in range(k.TREADS):
        out.append(ref("tread-%d" % (i + 1), F))
    out.append(ref("platform", F))
    if brace:
        out.append(ref("brace", R))
    out += supports(thick)
    out += mass_solids(com, mass_f, mass_r)
    return out


DECISIONS = {
    "50": [
        T2("Body: the real five-step platform stepladder built part by part: two 60 x 22 mm box stiles at 67.5 degrees that converge upwards, with moulded end caps and a 28 mm grab bar 0.56 m above the platform; four extruded 80 x 25 mm treads with rounded nose and anti-slip grooves, each fixed by two rivets per end (rivet heads as texture); the ribbed platform with rims and folded side cheeks on its steel axle between the stiles, its rear rim hooked behind the rest tube of the rear frame; two 45 x 20 mm rear rails on the hinge bolts through the stiles, with round hinge heads, the 22 mm rest tube and two 30 x 15 mm braces; on each side a two-link pressed-steel spreader (U channel 20 x 8 mm) with pivot rivets and the knuckle rivet; four rubber shoes with flat soles round the cut ends.",
           "Körper: die echte fünfstufige Plattform-Stehleiter Teil für Teil gebaut: zwei Kastenholme 60 x 22 mm unter 67,5 Grad, die nach oben zusammenlaufen, mit Endkappen und einem 28-mm-Haltebügel 0,56 m über der Plattform; vier stranggepresste Stufen 80 x 25 mm mit runder Nase und Rutschrillen, je Ende mit zwei Nieten befestigt (Nietköpfe als Textur); die geriffelte Plattform mit Rändern und gefalzten Seitenwangen auf ihrer Stahlachse zwischen den Holmen, der hintere Rand hinter dem Auflagerohr des Stützrahmens eingehängt; zwei Hinterholme 45 x 20 mm an den Gelenkbolzen durch die Holme, mit runden Gelenkköpfen, dem 22-mm-Auflagerohr und zwei Traversen 30 x 15 mm; je Seite eine zweigliedrige Stahlspreize (U-Profil 20 x 8 mm) mit Drehnieten und Knieniet; vier Gummischuhe mit flacher Sohle um die Schnittenden."),
        T2("Thermal: every region has one layer, so each region is the skin of its own body solids by reference: front rails with caps and grab bar, treads, platform with cheeks and axle, rear frame with rest tube, braces and hinge bolts, spreader links with their rivets, the four shoes; the sub-feature rivet heads of the treads carry no area.",
           "Thermik: jede Region hat eine Schicht, daher ist jede Region die Haut ihrer eigenen Körper als Verweis: Vorderholme mit Kappen und Haltebügel, Stufen, Plattform mit Wangen und Achse, Stützrahmen mit Auflagerohr, Traversen und Gelenkbolzen, Spreizenglieder mit ihren Nieten, die vier Schuhe; die Nietköpfe der Stufen unter der Darstellungsgrenze tragen keine Fläche."),
        T2("Structure: the load path of the ladder: hollow stiles (wall 1.5 mm) and hollow rear rails (wall 1.2 mm), the treads as hollow box members with a joint at each riveted end, the hollow grab bar, the platform deck on its axle (joints at the stiles) and on the hollow rest tube (joints at the rails), the hollow braces, the spreader links as ties with their pivots and knuckles, the hinge bolts as joints, a contact pad under each shoe and the two bone masses as half masses on the stiles and rails at the height of each centre of mass; caps, cheeks and shoes are absent.",
           "Tragwerk: der Lastpfad der Leiter: hohle Holme (Wand 1,5 mm) und hohle Hinterholme (Wand 1,2 mm), die Stufen als hohle Kastenstäbe mit einem Knoten an jedem genieteten Ende, der hohle Haltebügel, das Plattformdeck auf seiner Achse (Knoten an den Holmen) und auf dem hohlen Auflagerohr (Knoten an den Hinterholmen), die hohlen Traversen, die Spreizenglieder als Zugbänder mit Drehpunkten und Knien, die Gelenkbolzen als Knoten, je Schuh eine Kontaktplatte und die zwei Segmentmassen als Halbmassen auf Holmen und Hinterholmen in Höhe des jeweiligen Schwerpunkts; Kappen, Wangen und Schuhe fehlen."),
    ],
    "100": [
        T2("Body: authored anew for 1:100: stiles and rear rails as plain bars to their ends (caps merged), the grab bar, four plain tread boxes without grooves, the platform as one slab (rims, cheeks and axle merged), rest tube, two braces, one hinge boss per side (bolt, nut and washer merged), each spreader as one true-size slender strap from stile to rail (links and rivets merged) and four shoes with a thicker sole.",
           "Körper: für 1:100 neu verfasst: Holme und Hinterholme als glatte Stäbe bis zum Ende (Kappen verschmolzen), der Haltebügel, vier glatte Stufenkästen ohne Rillen, die Plattform als eine Platte (Ränder, Wangen und Achse verschmolzen), Auflagerohr, zwei Traversen, je Seite ein Gelenkzapfen (Bolzen, Mutter und Scheibe verschmolzen), jede Spreize als ein schlankes Band in wahrer Größe vom Holm zum Hinterholm (Glieder und Nieten verschmolzen) und vier Schuhe mit dickerer Sohle."),
        T2("Thermal: each region is the skin of its body solids by reference; the hinge bosses count to the rear frame.", "Thermik: jede Region ist die Haut ihrer Körper als Verweis; die Gelenkzapfen zählen zum Stützrahmen."),
        T2("Structure: hollow stiles and rails (wall drawn 3 mm), hollow tread boxes, grab bar, platform, rest tube and braces as the members they are, the spreader straps as ties, joints at the hinges and at the platform rest, a contact pad under each shoe and the half masses on stiles and rails.",
           "Tragwerk: hohle Holme und Hinterholme (Wand 3 mm gezeichnet), hohle Stufenkästen, Haltebügel, Plattform, Auflagerohr und Traversen als die Stäbe, die sie sind, die Spreizbänder als Zugbänder, Knoten an den Gelenken und am Plattformauflager, je Schuh eine Kontaktplatte und die Halbmassen auf Holmen und Hinterholmen."),
    ],
    "200": [
        T2("Body: authored anew for 1:200: stiles, rear rails and spreader straps are slender parts at their true size from the floor to their ends (drawn as strokes); the grab bar is a 30 mm bar, the treads are 80 x 30 mm slabs with their tops at the true heights, the platform a 30 mm slab, one 30 mm brace low on the rear frame and four 90 x 45 x 45 mm shoe blocks; hinge, rest tube, upper brace and rivets are omitted.",
           "Körper: für 1:200 neu verfasst: Holme, Hinterholme und Spreizbänder sind schlanke Teile in wahrer Größe vom Boden bis zu ihren Enden (als Striche gezeichnet); der Haltebügel ist ein 30-mm-Stab, die Stufen sind Platten 80 x 30 mm mit der Oberkante in wahrer Höhe, die Plattform eine 30-mm-Platte, eine 30-mm-Traverse unten am Stützrahmen und vier Schuhblöcke 90 x 45 x 45 mm; Gelenk, Auflagerohr, obere Traverse und Nieten entfallen."),
        T2("Thermal: each region is the skin of its body solids by reference; only the sum of the areas is compared at this scale.", "Thermik: jede Region ist die Haut ihrer Körper als Verweis; in diesem Maßstab wird nur die Flächensumme verglichen."),
        T2("Structure: the slender stiles, rails and straps are the members themselves, with the treads, the platform and the brace; a joint at each hinge, a contact pad under each shoe and the half masses on stiles and rails.",
           "Tragwerk: die schlanken Holme, Hinterholme und Bänder sind selbst die Stäbe, dazu Stufen, Plattform und Traverse; ein Knoten je Gelenk, je Schuh eine Kontaktplatte und die Halbmassen auf Holmen und Hinterholmen."),
    ],
    "500": [
        T2("Body: authored anew for 1:500: stiles, rear rails and spreader straps stay slender parts at their true size; the treads are 80 x 75 mm bars, the platform a 75 mm slab, the grab bar a 75 mm bar and the shoes four 100 x 75 x 75 mm blocks, each with its top or sole at the true height; braces, hinge and rest tube are omitted.",
           "Körper: für 1:500 neu verfasst: Holme, Hinterholme und Spreizbänder bleiben schlanke Teile in wahrer Größe; die Stufen sind Stäbe 80 x 75 mm, die Plattform eine 75-mm-Platte, der Haltebügel ein 75-mm-Stab und die Schuhe vier Blöcke 100 x 75 x 75 mm, jeweils mit Oberkante oder Sohle in wahrer Höhe; Traversen, Gelenk und Auflagerohr entfallen."),
        T2("Thermal: each region is the skin of its body solids by reference; only the sum of the areas is compared at this scale.", "Thermik: jede Region ist die Haut ihrer Körper als Verweis; in diesem Maßstab wird nur die Flächensumme verglichen."),
        T2("Structure: the slender stiles, rails and straps are the members themselves, with the tread bars and the platform; a joint at each hinge, a contact pad under each shoe and the half masses on stiles and rails.",
           "Tragwerk: die schlanken Holme, Hinterholme und Bänder sind selbst die Stäbe, dazu Stufenstäbe und Plattform; ein Knoten je Gelenk, je Schuh eine Kontaktplatte und die Halbmassen auf Holmen und Hinterholmen."),
    ],
}


def volume(scale, com):
    if scale == "50":
        body, structure = body50(), structure50(com)
    elif scale == "100":
        body, structure = body100(), structure100(com)
    elif scale == "200":
        body = coarse_body(0.030, (0.030, 0.030), 0.030, (0.090, 0.045, 0.045), True)
        structure = coarse_structure(com, 0.02, 0.03, [0.045, 0.03, 0.02], [0.04, 0.025, 0.019], True)
    else:
        body = coarse_body(0.075, (0.075, 0.075), 0.075, (0.100, 0.075, 0.075), False)
        structure = coarse_structure(com, 0.0375, 0.075, [0.05, 0.0375, 0.0375], [0.05, 0.0375, 0.0375], False)
    return {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal_of(body), "structure": structure, "decisions": DECISIONS[scale]}
