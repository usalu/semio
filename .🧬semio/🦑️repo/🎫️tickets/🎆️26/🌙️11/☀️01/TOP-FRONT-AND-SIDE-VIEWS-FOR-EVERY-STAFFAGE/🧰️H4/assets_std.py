from lib import *
from std import *


def adult_front(c, out):
    A, k, st, lv, R, rig = c["A"], c["k"], c["st"], c["lv"], c["A"]["reg"], c["rig"]
    hemY = c["waistY"] - A["hem"]
    hw = A["hem_hw"] * k
    nb = c["nbY"]
    if lv >= 1:
        left = [(0.13 * k, nb - 0.02 * k), (0.10 * k, nb + 0.03 * k), (0.085 * k, nb + 0.07 * k), (0.04 * k, nb + 0.088 * k), (0, nb + 0.09 * k)]
        out.append(shape(poly(left + mir(left[::-1])[1:], True, True), "hood", "torso", "body", 3.9, st.get(R["top"], "torso", hatch=False, tone=0.4)))
    if lv == 2:
        out.append(shape(poly([(-hw, hemY + 0.04 * k), (hw, hemY + 0.04 * k)], False, True), "hem-rib", "torso", "detail", 4.1, {"tone": 0.1, "region": R["top"]}, filled=False))
    if lv >= 3:
        out.append(shape(poly([(-hw, hemY), (hw, hemY), (hw, hemY + 0.04 * k), (-hw, hemY + 0.04 * k)], True, True), "hem-rib", "torso", "body", 4.1, st.get(R["top"], "torso", hatch=False, tone=0.35)))
        for i, x in enumerate((-0.04, 0.04)):
            out.append(shape(poly([(x * k, nb - 0.03 * k), (x * 1.1 * k, nb - 0.17 * k)], False, True), f"drawstring-{i}", "torso", "detail", 4.15, {"tone": 0.1, "region": R["top"]}, filled=False))
    if lv >= 2:
        out.append(shape(poly([(-0.045 * k, nb), (0, nb - 0.04 * k), (0.045 * k, nb)], False, True), "collar-rib", "torso", "detail", 4.1, {"tone": 0.1, "region": R["top"]}, filled=False))
        yb, yt = hemY + 0.05 * k, hemY + 0.17 * k
        out.append(shape(poly([(-0.115 * k, yb), (-0.075 * k, yt), (0.075 * k, yt), (0.115 * k, yb)], False, True), "kangaroo-pocket", "torso", "detail", 4.1, {"tone": 0.45 if lv >= 3 else 0.1, "region": R["top"]}, filled=False))
    for sd, sg in SIDES:
        xc = c["xcl"] if sg > 0 else c["xc"]
        fore, shank, thigh, foot = f"forearm-{sd}", f"shank-{sd}", f"thigh-{sd}", f"foot-{sd}"
        if lv == 2:
            out.append(shape(poly([(sg * (xc - 0.036 * k), rig.at(fore, 0.8)), (sg * (xc + 0.036 * k), rig.at(fore, 0.8))], False, True), f"cuff-{sd}", fore, "detail", 4.53, {"tone": 0.1, "region": R["sleeve"]}, filled=False))
        if lv >= 3:
            y0, y1 = rig.at(fore, 0.8), rig.at(fore, 1.0)
            out.append(shape(poly([(sg * (xc - 0.034 * k), y0), (sg * (xc + 0.034 * k), y0), (sg * (xc + 0.030 * k), y1), (sg * (xc - 0.030 * k), y1)], True, True), f"cuff-rib-{sd}", fore, "body", 4.53, st.get(R["sleeve"], fore, hatch=False, tone=0.35)))
        if lv >= 2:
            xl = c["xl"]
            out.append(shape(poly([(sg * (xl - 0.04 * k), rig.at(shank, 0.9)), (sg * (xl + 0.04 * k), rig.at(shank, 0.9))], False, True), f"jeans-hem-{sd}", shank, "detail", 1.03, {"tone": 0.1, "region": R["legs"]}, filled=False))
            g, aY = c["g"], c["ankleY"]
            out.append(shape(poly([(sg * (xl - 0.05 * k), g + 0.013 * k), (sg * (xl + 0.05 * k), g + 0.013 * k)], False, True), f"sole-{sd}", foot, "detail", 1.15, {"tone": 0.1, "region": R["shoes"]}, filled=False))
        if lv >= 3:
            xl = c["xl"]
            for i, dy in enumerate((0.03, 0.0, -0.03)):
                yy = c["ankleY"] + dy * k
                out.append(shape(poly([(sg * (xl - 0.02 * k), yy), (sg * (xl + 0.02 * k), yy)], False, True), f"lace-{sd}-{i}", foot, "texture", 1.2, {"tone": 0.1, "region": R["shoes"]}, filled=False))
            for i, s in enumerate((0.9, 0.97)):
                out.append(shape(poly([(sg * (xl - 0.025 * k), rig.at(thigh, s)), (sg * (xl + 0.025 * k), rig.at(thigh, s) - 0.008 * k)], False, True), f"jeans-knee-{sd}-{i}", thigh, "texture", 1.03, {"tone": 0.1, "region": R["legs"]}, filled=False))


def adult_plan(c, out):
    k, st, lv, R = c["k"], c["st"], c["lv"], c["A"]["reg"]
    if lv >= 1:
        hx = c["rig"].ox("head")
        out.append(shape(ell(hx - 0.075 * k, 0, 0.065 * k, 0.085 * k), "hood", "neck", "body", 3.95, st.get(R["top"], "torso", hatch=False, tone=0.4)))


ADULT = dict(
    id="standing-adult", W=0.2,
    names=dict(arm="arm", forearm="forearm", hand="hand", neck="neck"),
    reg=dict(skin="face-skin", hair="hair", hand="hand-skin", top="hoodie", sleeve="hoodie", legs="jeans", shoes="sneakers"),
    hem=0.17, hem_hw=0.158, waist_hw=0.15, torso_id="hoodie",
    front_extras=adult_front, plan_extras=adult_plan,
)

def phone_front(c, out):
    k, st, lv, R, rig = c["k"], c["st"], c["lv"], c["A"]["reg"], c["rig"]
    adult_front(c, out)
    nb = c["nbY"]
    if lv >= 1:
        for sd, sg in SIDES:
            out.append(shape(ell(sg * 0.078 * k, nb + 0.0 * k, 0.026 * k, 0.034 * k), f"headphone-cup-{sd}", "neck", "body", 4.4, st.get("headphones", "neck")))
    if lv >= 2:
        left = [(0.078 * k, nb + 0.03 * k), (0.07 * k, nb + 0.075 * k), (0.035 * k, nb + 0.095 * k), (0, nb + 0.1 * k)]
        out.append(shape(poly(left + mir(left[::-1])[1:], False), "headphone-band", "neck", "detail", 4.3, {"tone": 0.3, "region": "headphones"}, filled=False))
    ox, oy = rig.o("phone")
    px = -c["xc"]
    out.append(shape(rect(px - (0.037 if lv else 0.05), oy - 0.068, (0.074 if lv else 0.1), 0.136), "phone", "phone", "body", 4.8, st.get("phone", "phone")))
    if lv >= 3:
        out.append(shape(rect(px - 0.031, oy - 0.058, 0.062, 0.116), "phone-screen", "phone", "detail", 4.85, {"tone": 0.1, "region": "phone"}, filled=False))


def phone_plan(c, out):
    adult_plan(c, out)
    if True:
        hx = c["rig"].ox("head")
        pt = [e for e in c["rep"]["geometry"]["elevation"] if e["id"] in ("ponytail", "sil-ponytail")]
        prof = {k_: v for k_, v in pt[0].items() if k_ in ("kind", "d", "points", "closed", "cx", "cy", "rx", "ry", "rotationDeg")} if pt else ell(-0.048, 0.147, 0.145, 0.05)
        out.append(shape(prof, "ponytail", "head", "body", 5.9, c["st"].get("hair", "head"), solid={"kind": "extruded", "z0": -0.04, "z1": 0.04}))
    rig, k, st = c["rig"], c["k"], c["st"]
    ox, oy = rig.o("phone")
    ang = math.radians(rig.w["phone"][2])
    out.append(shape(cap(ox - 0.074 * math.cos(ang), -c["xc"], ox + 0.074 * math.cos(ang), -c["xc"], (0.04 if c["lv"] == 0 else 0.022), (0.04 if c["lv"] == 0 else 0.022), False), "phone", "phone", "body", 3.2, st.get("phone", "phone")))


PHONE = dict(ADULT, id="standing-phone-user", front_extras=phone_front, plan_extras=phone_plan, hair_front=True, hand_extra=-0.03)

def commuter_front(c, out):
    k, st, lv, R, rig = c["k"], c["st"], c["lv"], c["A"]["reg"], c["rig"]
    nb, hemY = c["nbY"], c["waistY"] - c["A"]["hem"]
    hw = c["A"]["hem_hw"] * k
    if lv >= 1:
        sc_ = [(0.085 * k, nb - 0.07 * k), (0.095 * k, nb), (0.07 * k, nb + 0.06 * k), (0, nb + 0.065 * k)]
        out.append(shape(poly(sc_ + mir(sc_[::-1])[1:], True, True), "scarf", "torso", "body", 4.5, st.get("scarf", "torso")))
    if lv >= 2:
        out.append(shape(poly([(-0.07 * k, nb - 0.02 * k), (0, nb - 0.2 * k), (0.07 * k, nb - 0.02 * k)], False, True), "coat-front-edge", "torso", "detail", 4.1, {"tone": 0.1, "region": R["top"]}, filled=False))
        out.append(shape(poly([(-hw, hemY + 0.02 * k), (hw, hemY + 0.02 * k)], False, True), "coat-hem", "torso", "detail", 4.1, {"tone": 0.1, "region": R["top"]}, filled=False))
        for sd, sg in SIDES:
            out.append(shape(poly([(sg * 0.11 * k, c["waistY"] - 0.05 * k), (sg * 0.15 * k, c["waistY"] - 0.07 * k)], False, True), f"coat-pocket-{sd}", "torso", "detail", 4.1, {"tone": 0.2, "region": R["top"]}, filled=False))
    if lv >= 3:
        for i, d in enumerate((0.06, 0.16, 0.26)):
            out.append(shape(ell(0.012 * k, nb - (0.1 + d) * k, 0.008 * k, 0.008 * k), f"coat-button-{i + 1}", "torso", "detail", 4.2, {"tone": 0.1, "region": R["top"]}))
    b = "bag"
    xb0, xb1 = c["xcl"] + 0.02, c["Wl"] + 0.11
    sol = bag_solid(xb0, xb1)
    if lv >= 1:
        out.append(shape(BAG_PROFILE, "bag-body" if lv else "sil-bag", b, "body", 5.5, st.get("bag-leather", b), solid=sol))
        out.append(shape(poly([(c["xcl"] + 0.005, rig.at(b, 0.0)), (c["xcl"] + 0.03, rig.at(b, 0.72))], False, True), "bag-strap", b, "detail", 5.6, {"tone": 0.2, "region": "bag-leather"}, filled=False))
    else:
        out.append(shape(BAG_PROFILE, "sil-bag", b, "body", 5.5, st.get("bag-leather", b), solid=sol))
    if lv >= 3:
        out.append(shape(poly([(xb0, rig.at(b, 0.86)), (xb1, rig.at(b, 0.86))], False), "bag-flap", b, "detail", 5.55, {"tone": 0.34, "region": "bag-leather"}, filled=False))


BAG_PROFILE = rect(0.435, -0.145, 0.205, 0.29, 0.02)


def bag_solid(z0, z1):
    return {"kind": "extruded", "z0": r4(z0), "z1": r4(z1)}


def commuter_plan(c, out):
    rig, k, st = c["rig"], c["k"], c["st"]
    xo = rig.ox("bag")
    out.append(shape(BAG_PROFILE, "bag", "bag", "body", 2.5, st.get("bag-leather", "bag"), solid=bag_solid(c["xcl"] + 0.02, c["Wl"] + 0.11)))
    if c["lv"] >= 3:
        out.append(shape(poly([(xo - 0.1, c["xcl"] + 0.062), (xo - 0.04, c["xcl"] + 0.062)], False), "bag-flap", "bag", "detail", 2.6, {"tone": 0.5}, filled=False))
    if c["lv"] >= 2:
        hx = rig.ox("head")
        out.append(shape(ell(hx - 0.01, 0, 0.07 * k, 0.09 * k), "scarf-plan", "neck", "body", 4.6, st.get("scarf", "neck")))


COMMUTER = dict(
    id="walking-commuter", W=0.2,
    names=dict(arm="arm", forearm="forearm", hand="hand", neck="neck"),
    reg=dict(skin="face-skin", hair="hair", hand="hand-skin", top="coat", sleeve="coat", legs="trousers", shoes="shoes"),
    hem=0.39, hem_hw=0.19, waist_hw=0.16, chest_hw=0.175, torso_id="coat", neckwear_region="scarf",
    front_extras=commuter_front, plan_extras=commuter_plan,
)


def tour_front(c, out):
    k, st, lv, R, rig = c["k"], c["st"], c["lv"], c["A"]["reg"], c["rig"]
    nb, hemY = c["nbY"], c["waistY"] - c["A"]["hem"]
    if lv >= 1:
        out.append(shape(poly([(-0.088 * k, c["top"] - 0.075 * k), (0, c["top"] - 0.06 * k), (0.088 * k, c["top"] - 0.075 * k), (0.092 * k, c["chin"] + 0.12 * k), (-0.092 * k, c["chin"] + 0.12 * k)]), "cap", "head", "body", 5.7, st.get("hair", "head")))
    if lv >= 2:
        out.append(shape(poly([(-0.092 * k, c["top"] - 0.07 * k), (0.092 * k, c["top"] - 0.07 * k)], False), "cap-seam", "head", "detail", 5.75, {"tone": 0.6, "region": "hair"}, filled=False))
        out.append(shape(poly([(-0.02 * k, hemY + 0.01), (-0.02 * k, nb - 0.02 * k)], False, True), "jacket-zip", "torso", "detail", 3.3, {"tone": 0.2, "region": R["top"]}, filled=False))
        out.append(shape(poly([(-0.17 * k, hemY + 0.01), (0.17 * k, hemY + 0.01)], False, True), "jacket-hem", "torso", "detail", 3.3, {"tone": 0.2, "region": R["top"]}, filled=False))
    if lv >= 3:
        out.append(shape(rect(0.03 * k, nb - 0.17 * k, 0.085 * k, 0.07 * k, 0, True), "badge", "torso", "body", 3.55, st.get(R["top"], "torso", tone=0.95)))
        out.append(shape(poly([(-0.03 * k, nb - 0.015 * k), (0.045 * k, nb - 0.17 * k)], False, True), "lanyard", "torso", "detail", 3.5, {"tone": 0.9, "region": R["top"]}, filled=False))
    px = -c["xc"]
    ptw = 0.006 if c["lv"] else 0.03
    out.append(shape(poly([(px - ptw, rig.at("pointer", 0.0)), (px + ptw, rig.at("pointer", 0.0)), (px + ptw, rig.at("pointer", 1.0)), (px - ptw, rig.at("pointer", 1.0))], True, True), "pointer", "pointer", "body", 6.4, st.get("pointer-metal", "pointer")))
    fx = c["xcl"]
    pw = 0.012 if c["lv"] else 0.032
    out.append(shape(poly([(fx - pw, rig.at("flag-pole", 0.0)), (fx + pw, rig.at("flag-pole", 0.0)), (fx + pw, rig.at("flag-pole", 0.97)), (fx - pw, rig.at("flag-pole", 0.97))], True, True), "pole", "flag-pole", "body", 6.3, st.get("pole-metal", "flag-pole")))
    hy = rig.oy("hand-l")
    if c["lv"]:
        out.append(shape(rect(fx - 0.012, hy - 0.2774, 0.024, 0.3089), "pole-grip", "hand-l", "body", 6.3, st.get("pole-metal", "flag-pole")))
    out.append(shape(ell(1.05, 0, 0.016, 0.016), "pole-finial", "flag-pole", "body", 6.35, st.get("pole-metal", "flag-pole", tone=0.8), solid={"kind": "extruded", "z0": r4(fx - 0.016), "z1": r4(fx + 0.016)}))
    out.append(shape(rect(0, 0, 0.46, 0.29), "flag", "flag", "body", 6.2, st.get("flag-cloth", "flag"), solid={"kind": "extruded", "z0": r4(fx - 0.008 - (0.017 if lv == 0 else 0)), "z1": r4(fx + 0.008 + (0.017 if lv == 0 else 0))}))


def tour_plan(c, out):
    rig, k, st = c["rig"], c["k"], c["st"]
    hx = rig.ox("head")
    px = rig.ox("pointer")
    out.append(shape(cap(px, -c["xc"], rig.tx("pointer"), -c["xc"], (0.008 if c["lv"] else 0.03), (0.008 if c["lv"] else 0.03), True), "pointer", "pointer", "body", 3.4, st.get("pointer-metal", "pointer")))
    fx = c["xcl"]
    out.append(shape(cap(rig.ox("flag-pole"), fx, rig.ox("flag-pole"), fx, (0.012 if c["lv"] else 0.032), (0.012 if c["lv"] else 0.032), True), "pole", "flag-pole", "body", 3.3, st.get("pole-metal", "flag-pole")))
    out.append(shape(rect(0, 0, 0.46, 0.29), "flag", "flag", "body", 3.2, st.get("flag-cloth", "flag"), solid={"kind": "extruded", "z0": r4(fx - 0.008 - (0.017 if c["lv"] == 0 else 0)), "z1": r4(fx + 0.008 + (0.017 if c["lv"] == 0 else 0))}))
    out.append(shape(ell(1.05, 0, 0.016, 0.016), "pole-finial", "flag-pole", "body", 3.25, st.get("pole-metal", "flag-pole", tone=0.8), solid={"kind": "extruded", "z0": r4(fx - 0.016), "z1": r4(fx + 0.016)}))
    if c["lv"] >= 1:
        out.append(shape(ell(hx + 0.01, 0, 0.1 * k, 0.09 * k), "cap", "head", "body", 6.05, st.get("hair", "head")))


TOUR = dict(
    id="tour-guide", W=0.25,
    names=dict(arm="upper-arm", forearm="forearm", hand="hand", neck=None),
    reg=dict(skin="head-skin", hair="hair", hand="hand-skin", top="torso-clothing", sleeve="arm-clothing", legs="leg-clothing", shoes="shoes", pelvis="leg-clothing"),
    hem=0.119, hem_hw=0.19, waist_hw=0.18, chest_hw=0.21, pelvis_shape=True,
    front_extras=tour_front, plan_extras=tour_plan, neckwear=False,
)


def worker_front(c, out):
    k, st, lv, R, rig = c["k"], c["st"], c["lv"], c["A"]["reg"], c["rig"]
    w, wy = c["waistY"], c["nbY"]
    left = [(0, w + 0.002), (0.15 * k, w + 0.002), (0.165 * k, c["shY"] - 0.1 * k), (0.185 * k, c["shY"] - 0.02 * k), (0.09 * k, wy - 0.01 * k), (0, wy - 0.02 * k)]
    out.append(shape(poly(left + mir(left[::-1])[1:-1], True, True), "vest", "torso", "body", 4.05, st.get("vest", "torso")))
    if lv >= 2:
        for i, s in enumerate((0.2, 0.62)):
            yb = w + s * (wy - w)
            out.append(shape(poly([(-0.15 * k, yb), (0.15 * k, yb), (0.152 * k, yb + 0.045 * k), (-0.152 * k, yb + 0.045 * k)], True, True), f"vest-band-{'low' if i == 0 else 'high'}", "torso", "detail", 4.1, st.get("vest", "torso", hatch=False, tone=0.55)))
    if lv >= 1:
        hy = c["top"]
        out.append(shape(poly([(-0.095 * k, c["chin"] + 0.12 * k), (-0.092 * k, hy - 0.06 * k), (-0.06 * k, hy + 0.005), (0, hy + 0.012), (0.06 * k, hy + 0.005), (0.092 * k, hy - 0.06 * k), (0.095 * k, c["chin"] + 0.12 * k)]), "helmet", "head", "body", 5.7, st.get("helmet", "head")))
    if lv >= 3:
        out.append(shape(poly([(0, hy + 0.012), (0, c["chin"] + 0.14 * k)], False), "helmet-ridge", "head", "detail", 5.8, {"tone": 0.6, "region": "helmet"}, filled=False))
        out.append(shape(rect(-c["xl"] - 0.035, rig.at("shank-r", 0.0) - 0.07, 0.07, 0.09, 0, True), "knee-pad-r", "shank-r", "detail", 2.1, {"tone": 0.18, "region": R["legs"]}))
    sol = {"kind": "extruded", "z0": -0.2, "z1": 0.2}
    out.append(shape(rect(-0.2, -0.15, 0.4, 0.3), "box", "box-r", "body", 5.8, st.get("box-surface", "box-r"), solid=sol))
    if lv >= 2:
        out.append(shape(rect(-0.2, 0.075, 0.4, 0.04), "box-tape", "box-r", "detail", 5.82, st.get("box-surface", "box-r", tone=0.8), solid=sol))


def worker_plan(c, out):
    rig, k, st = c["rig"], c["k"], c["st"]
    out.append(shape(rect(-0.2, -0.15, 0.4, 0.3), "box", "box-r", "body", 5.8, st.get("box-surface", "box-r"), solid={"kind": "extruded", "z0": -0.2, "z1": 0.2}))
    if c["lv"] >= 1:
        hx = rig.ox("head")
        out.append(shape(ell(hx, 0, 0.1 * k, 0.092 * k), "helmet", "head", "body", 6.05, st.get("helmet", "head")))
    if c["lv"] >= 2:
        out.append(shape(rect(-0.2, 0.075, 0.4, 0.04), "box-tape", "box-r", "detail", 5.82, {"tone": 0.8}, filled=False, solid={"kind": "extruded", "z0": -0.02, "z1": 0.02}))


WORKER = dict(
    id="worker-carrying-box", W=0.25,
    names=dict(arm="upper-arm", forearm="forearm", hand="hand", neck=None),
    reg=dict(skin="head-skin", hair="hair", hand="gloves", top="torso-clothing", sleeve="arm-clothing", legs="leg-clothing", shoes="shoes", pelvis="leg-clothing"),
    hem=0.061, hem_hw=0.17, waist_hw=0.165, chest_hw=0.215, pelvis_shape=True,
    front_extras=worker_front, plan_extras=worker_plan, neckwear=False,
)

CONFIGS = {"standing-adult": ADULT, "standing-phone-user": PHONE, "walking-commuter": COMMUTER, "tour-guide": TOUR, "worker-carrying-box": WORKER}
