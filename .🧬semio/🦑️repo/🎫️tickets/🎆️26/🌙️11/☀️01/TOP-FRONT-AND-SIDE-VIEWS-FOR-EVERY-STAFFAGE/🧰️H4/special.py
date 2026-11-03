"""Wheelchair user and sleeping person: the two figures whose front and plan are not a standing body."""
from lib import *
from std import *
import std
from assets_std import CONFIGS


def dome(hw, y0, y1):
    left = [(hw * math.cos(math.radians(a)), y0 + (y1 - y0) * math.sin(math.radians(a))) for a in range(0, 91, 15)]
    return left + mir(left[::-1])[1:]


# ----------------------------------------------------------------------------- wheelchair
def wheel_front(A, asset, rep, sc):
    c = ctx_of(A, asset, rep, sc)
    k, st, lv, R, rig = c["k"], c["st"], c["lv"], A["reg"], c["rig"]
    c["g"] = 0.072
    fy = rig.oy("frame")
    out = []
    legs_front(c, out)
    out = [s for s in out if "thigh" not in s["id"]]
    for s in out:
        s["z"] = round(s["z"] + 3.3, 3)
    for sd, sg in SIDES:
        thigh = f"thigh-{sd}"
        xl = 0.088 * k if lv else 0.066 * k
        hwt = 0.083 * k if lv else 0.095 * k
        y0, y1 = rig.oy(thigh) + 0.09 * k, rig.ty(thigh) - 0.085 * k
        pl = [(sg * (xl + hwt), y0 - 0.02), (sg * (xl + hwt * 1.02), y1 + 0.03), (sg * (xl + hwt * 0.8), y1), (sg * (xl - hwt * 0.8), y1), (sg * (xl - hwt * 1.02), y1 + 0.03), (sg * (xl - hwt), y0 - 0.02), (sg * (xl - hwt * 0.7), y0), (sg * (xl + hwt * 0.7), y0)]
        out.append(shape(poly(chaikin(pl, 1), True, True), f"thigh-{sd}" if lv else f"sil-thigh-{sd}", thigh, "body", 4.3, st.get(R["legs"], thigh)))
    hip = rig.oy("pelvis")
    pl = [(0.17 * k, hip - 0.05 * k), (0.16 * k, rig.oy("torso") + 0.02), (-0.16 * k, rig.oy("torso") + 0.02), (-0.17 * k, hip - 0.05 * k)]
    out.append(shape(poly(pl), "pelvis", "pelvis", "body", 3.05, st.get(R["legs"], "pelvis")))
    torso_front(c, out)
    neck_front(c, out)
    arms_front(c, out)
    head_front(c, out)
    if lv >= 1:
        left = [(0.12 * k, c["nbY"] - 0.02), (0.095 * k, c["nbY"] + 0.04), (0.06 * k, c["nbY"] + 0.07), (0, c["nbY"] + 0.075)]
        out.append(shape(poly(left + mir(left[::-1])[1:], True, True), "hood", "neck", "body", 4.5, st.get(R["top"], "neck", hatch=False, tone=0.32)))
    # chair
    tyw = 0.1 if lv == 0 else 0.045
    for sd, sg in SIDES:
        cx = sg * (0.30 if lv == 0 else 0.33)
        out.append(shape(rect(cx - tyw / 2, fy - 0.305, tyw, 0.61), f"rear-tyre-{sd}" if lv else f"sil-wheel-{sd}", "rear-wheel", "body", 7, st.get("tyres", "rear-wheel")))
        if lv >= 2:
            out.append(shape(rect(sg * 0.30 - 0.006, fy - 0.28, 0.012, 0.56), f"hand-rim-{sd}", "rear-wheel", "detail", 7.05, st.get("frame-metal", "rear-wheel", tone=0.45)))
        if lv >= 3:
            out.append(shape(rect(sg * 0.315 - 0.03, fy - 0.034, 0.06, 0.068), f"rear-hub-{sd}", "rear-wheel", "body", 7.2, st.get("frame-metal", "rear-wheel", tone=0.55)))
        if lv >= 1:
            out.append(shape(rect(sg * 0.215 - 0.0175, fy - 0.2372 - 0.068, 0.035, 0.136), f"castor-tyre-{sd}", "castor", "body", 7, st.get("tyres", "castor")))
        if lv >= 1:
            out.append(shape(rect(sg * 0.205 - 0.013, fy - 0.207, 0.026, 0.41), f"frame-hanger-{sd}", "frame", "body", 6.7, st.get("frame-metal", "frame")))
        if lv >= 2:
            out.append(shape(rect(sg * 0.205 - 0.013, fy + 0.15, 0.026, 0.487), f"frame-back-tube-{sd}", "frame", "body", 6.8, st.get("frame-metal", "frame")))
            out.append(shape(rect(sg * 0.205 - 0.02, fy + 0.591, 0.04, 0.048), f"frame-push-handle-{sd}", "frame", "body", 6.8, st.get("frame-metal", "frame", tone=0.15)))
        if lv >= 3:
            out.append(shape(rect(sg * 0.185 - 0.01, fy + 0.17, 0.02, 0.145), f"brake-lever-{sd}", "frame", "detail", 6.9, st.get("frame-metal", "frame", tone=0.2)))
    out.append(shape(rect(-0.21, fy + 0.165, 0.42, 0.072), "seat-cushion" if lv else "sil-seat", "frame", "body", 2.4, st.get("cushion", "frame")))
    if lv >= 1:
        out.append(shape(rect(-0.2, fy + 0.225, 0.4, 0.33), "back-sling", "frame", "body", 2.3, st.get("cushion", "frame")))
    out.append(shape(rect(-0.18, fy - 0.23, 0.36, 0.025 if lv else 0.04), "footplate" if lv else "sil-footplate", "frame", "body", 6.7, st.get("frame-metal", "frame")))
    if lv >= 2:
        out.append(shape(rect(-0.205, fy - 0.02, 0.41, 0.025), "frame-lower-rail", "frame", "detail", 6.7, st.get("frame-metal", "frame")))
    return out


def wheel_plan(A, asset, rep, sc):
    c = ctx_of(A, asset, rep, sc)
    out = std.plan_std(A, asset, rep, sc)
    st, lv, rig = c["st"], c["lv"], c["rig"]
    fx = rig.ox("frame")
    tyw = 0.1 if lv == 0 else 0.045
    for sd, sg in SIDES:
        cy = sg * (0.30 if lv == 0 else 0.33)
        out.append(shape(rect(rig.ox("rear-wheel") - 0.305, cy - tyw / 2, 0.61, tyw), f"rear-tyre-{sd}" if lv else f"sil-wheel-{sd}", "rear-wheel", "body", 0.9, st.get("tyres", "rear-wheel")))
        if lv >= 2:
            out.append(shape(rect(rig.ox("rear-wheel") - 0.28, sg * 0.30 - 0.006, 0.56, 0.012), f"hand-rim-{sd}", "rear-wheel", "detail", 0.95, st.get("frame-metal", "rear-wheel", tone=0.45)))
        if lv >= 1:
            out.append(shape(rect(rig.ox("castor") - 0.068, sg * 0.215 - 0.0175, 0.136, 0.035), f"castor-tyre-{sd}", "castor", "body", 0.9, st.get("tyres", "castor")))
        if lv >= 2:
            out.append(shape(rect(fx - 0.086, sg * 0.205 - 0.0125, 0.518, 0.025), f"frame-seat-rail-{sd}", "frame", "body", 0.8, st.get("frame-metal", "frame")))
        if lv >= 2:
            out.append(shape(rect(fx - 0.214, sg * 0.205 - 0.02, 0.116, 0.04), f"frame-push-handle-{sd}", "frame", "body", 0.8, st.get("frame-metal", "frame", tone=0.15)))
    out.append(shape(rect(fx - 0.07, -0.21, 0.49, 0.42), "seat-cushion" if lv else "sil-seat", "frame", "body", 0.6, st.get("cushion", "frame")))
    out.append(shape(rect(fx + 0.46, -0.17, 0.32, 0.34), "footplate" if lv else "sil-footplate", "frame", "body", 0.7, st.get("frame-metal", "frame")))
    return out


WHEEL = dict(
    id="wheelchair-user", W=0.2, k=1.03,
    names=dict(arm="arm", forearm="forearm", hand="hand", neck="neck"),
    reg=dict(skin="face-skin", hair="beanie", hand="gloves", top="hoodie", sleeve="hoodie", legs="jeans", shoes="shoes"),
    hem=0.136, hem_hw=0.158, waist_hw=0.15, torso_id="hoodie", neckwear=False, wheel=True,
)


# ----------------------------------------------------------------------------- sleeping person
def sleep_front(A, asset, rep, sc):
    c = ctx_of_sleep(asset, rep, sc)
    st, lv, rig = c["st"], c["lv"], c["rig"]
    out = []
    add = out.append
    top, bot = "top", "bottom"
    add(shape(rect(-0.2, 0.448, 0.4, 0.136), "pillow", "bed-surface", "body", 1, st.get("pillow", "bed-surface")))
    hy = rig.oy("head")
    if lv == 0:
        add(shape(ell(0, hy + 0.0, 0.08, 0.105), "sil-head", "head", "body", 2, st.merged(["face-skin", "hair"])))
    else:
        add(shape(ell(0, hy, 0.075, 0.103), "head", "head", "body", 2, st.get("face-skin", "head")))
        add(shape(poly([(-0.078, hy + 0.02), (-0.05, hy + 0.09), (0, hy + 0.108), (0.05, hy + 0.09), (0.078, hy + 0.02), (0.06, hy + 0.05), (0, hy + 0.062), (-0.06, hy + 0.05)]), "hair", "head", "body", 2.2, st.get("hair", "head")))
        add(shape(poly([(0.045, rig.oy("neck") - 0.04), (0.04, rig.ty("neck") + 0.01), (-0.04, rig.ty("neck") + 0.01), (-0.045, rig.oy("neck") - 0.04)]), "neck", "neck", "body", 3.8, st.get("face-skin", "neck")))
    add(shape(poly(dome(0.19, 0.46, 0.758)), "duvet-chest" if lv else "sil-chest", "torso", "body", 4, st.get(top, "torso")))
    if lv >= 1:
        add(shape(poly(dome(0.185, 0.46, 0.748)), "duvet-hips", "pelvis", "body", 4.1, st.get(top, "pelvis")))
    add(shape(poly(dome(0.17, 0.46, 0.723), True, True), "duvet-thighs" if lv else "sil-thighs", "thigh-r", "body", 4.2, st.get(bot, "thigh-r")))
    add(shape(poly(dome(0.15, 0.46, 0.66), True, True), "duvet-shins" if lv else "sil-shins", "shank-r", "body", 4.3, st.get(bot, "shank-r")))
    add(shape(poly([(-0.2, 0.33), (0.2, 0.33), (0.2, 0.52), (0.15, 0.565), (0, 0.575), (-0.15, 0.565), (-0.2, 0.52)]), "duvet-skirt-legs", "bed-surface", "body", 6.05, st.get(bot, "bed-surface")))
    if lv >= 2:
        add(shape(poly([(-0.2, 0.52), (0.2, 0.52)], False), "duvet-hem", "bed-surface", "detail", 6.1, {"tone": 0.3, "region": bot}, filled=False))
        add(shape(poly([(-0.12, 0.7), (0.12, 0.7)], False), "duvet-fold-chest", "torso", "detail", 4.5, {"tone": 0.3, "region": top}, filled=False))
    if lv >= 1:
        for sd, sg in SIDES:
            add(shape(ell(sg * 0.15, rig.oy(f"arm-{sd}"), 0.05, 0.05), f"sleeve-{sd}", f"arm-{sd}", "body", 6.5, st.get(top, "arm-r")))
            add(shape(ell(sg * 0.15, rig.oy(f"hand-{sd}"), 0.03, 0.035), f"hand-{sd}", f"hand-{sd}", "body", 6.7, st.get("hand-skin", "hand-r")))
    for sd, sg in SIDES:
        foot = f"foot-{sd}"
        y0, y1 = rig.oy(foot), rig.ty(foot)
        pl = [(sg * 0.125, y0 - 0.03), (sg * 0.133, y0 + 0.5 * (y1 - y0)), (sg * 0.138, y1), (sg * 0.125, y1 + 0.055), (sg * 0.095, y1 + 0.062), (sg * 0.062, y1 + 0.05), (sg * 0.05, y1), (sg * 0.055, y0 + 0.5 * (y1 - y0)), (sg * 0.068, y0 - 0.03)]
        add(shape(poly(chaikin(pl, 1), True), f"foot-{sd}" if lv else f"sil-foot-{sd}", foot, "body", 6.8, st.get("feet-skin", foot)))
    return out


def ctx_of_sleep(asset, rep, sc):
    return dict(asset=asset, rep=rep, sc=sc, lv=LV[sc], st=Sty(rep), rig=Rig(asset))


def sleep_plan(A, asset, rep, sc):
    c = ctx_of_sleep(asset, rep, sc)
    st, lv, rig = c["st"], c["lv"], c["rig"]
    out = []
    add = out.append
    top, bot = "top", "bottom"
    add(shape(rect(0.07, -0.2, 0.638, 0.4), "pillow", "bed-surface", "body", 1, st.get("pillow", "bed-surface")))
    hx = (rig.ox("head") + rig.tx("head")) / 2
    add(shape(ell(hx + 0.01, 0, 0.11, 0.085), "head" if lv else "sil-head", "head", "body", 2, st.get("hair", "head") if lv else st.merged(["face-skin", "hair"])))
    if lv >= 1:
        add(shape(ell(hx + 0.035, 0, 0.065, 0.058), "face", "head", "body", 2.1, st.get("face-skin", "head")))
    add(shape(cap(rig.ox("torso"), 0, rig.tx("torso"), 0, 0.19, 0.19, True), "duvet-chest" if lv else "sil-chest", "torso", "body", 4, st.get(top, "torso")))
    if lv >= 1:
        add(shape(cap(rig.ox("pelvis"), 0, rig.tx("pelvis"), 0, 0.185, 0.185, True), "duvet-hips", "pelvis", "body", 4.1, st.get(top, "pelvis")))
    add(shape(cap(rig.ox("thigh-r"), 0, rig.tx("thigh-r"), 0, 0.17, 0.16, True), "duvet-thighs" if lv else "sil-thighs", "thigh-r", "body", 4.2, st.get(bot, "thigh-r")))
    add(shape(cap(rig.ox("shank-r"), 0, rig.tx("shank-r"), 0, 0.15, 0.13, True), "duvet-shins" if lv else "sil-shins", "shank-r", "body", 4.3, st.get(bot, "shank-r")))
    if lv >= 2:
        add(shape(poly([(rig.ox("torso") - 0.2, -0.17), (rig.ox("torso") - 0.2, 0.17)], False), "duvet-fold-chest", "torso", "detail", 4.5, {"tone": 0.3, "region": top}, filled=False))
    for sd, sg in SIDES:
        if lv >= 1:
            add(shape(cap(rig.ox(f"arm-{sd}"), sg * 0.15, rig.tx(f"arm-{sd}"), sg * 0.15, 0.05, 0.045, True), f"sleeve-{sd}", f"arm-{sd}", "body", 6.5, st.get(top, "arm-r")))
            add(shape(cap(rig.ox(f"forearm-{sd}"), sg * 0.15, rig.tx(f"forearm-{sd}"), sg * 0.15, 0.042, 0.035, True), f"forearm-{sd}", f"forearm-{sd}", "body", 6.6, st.get("hand-skin", "hand-r")))
            add(shape(cap(rig.ox(f"hand-{sd}"), sg * 0.15, rig.tx(f"hand-{sd}"), sg * 0.15, 0.032, 0.025, True), f"hand-{sd}", f"hand-{sd}", "body", 6.7, st.get("hand-skin", "hand-r")))
        add(shape(cap(rig.ox(f"foot-{sd}"), sg * 0.09, rig.tx(f"toe-{sd}") - 0.03, sg * 0.09, 0.045, 0.04, True), f"foot-{sd}" if lv else f"sil-foot-{sd}", f"foot-{sd}", "body", 6.8, st.get("feet-skin", f"foot-{sd}")))
    add(shape(rect(0.5, -0.2, 0.56, 0.4), "duvet-skirt-top", "bed-surface", "body", 6, st.get(top, "bed-surface")))
    add(shape(rect(1.04, -0.2, 0.68, 0.4), "duvet-skirt-legs", "bed-surface", "body", 6.05, st.get(bot, "bed-surface")))
    return out


SLEEP = dict(id="sleeping-person")
