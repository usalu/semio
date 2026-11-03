"""Hand-designed front and plan views of the standing human figures (adult, phone user, commuter, tour guide, worker).

Every coordinate is designed here in model metres in the rig rest pose; the per-asset configuration in `assets_std.py` only
chooses the garment regions, the lateral extents and the extras that are specific to one figure."""
import math

from lib import *

SIDES = (("l", 1), ("r", -1))


def ctx_of(A, asset, rep, sc):
    rig = Rig(asset)
    n = A["names"]
    top = rig.ty("head")
    g = rig.ground()
    k = A.get("k", (top - g) / 1.69)
    c = dict(A=A, asset=asset, rig=rig, rep=rep, sc=sc, lv=LV[sc], st=Sty(rep), k=k, g=g, top=top, n=n)
    c["W"] = A["W"]
    c["Wl"] = A.get("Wl", A["W"])
    c["xl"] = 0.088 * k
    c["xc"] = A["W"] - 0.047 * k
    c["xcl"] = c["Wl"] - 0.047 * k
    c["hipY"] = rig.oy("pelvis")
    c["waistY"] = rig.oy("torso")
    c["nbY"] = rig.ty("torso")
    c["shY"] = rig.oy(n["arm"] + "-l")
    c["headY0"] = rig.oy("head")
    c["hh"] = 0.224 * k
    c["chin"] = top - c["hh"]
    c["kneeY"] = rig.oy("shank-l")
    c["ankleY"] = rig.oy("foot-l")
    c["adultfoot"] = abs(rig.ty("foot-l") - rig.oy("foot-l")) > 0.03
    return c


def side_pts(left, sg):
    return left if sg > 0 else mir(left)


def limb_poly(c, bone, xcen, prof, sg, smooth=1, cap_top=None, cap_bot=None):
    rig = c["rig"]
    y0, y1 = rig.oy(bone), rig.ty(bone)
    outer = [(xcen + hw, y0 + s * (y1 - y0)) for s, hw in prof]
    inner = [(xcen - hw, y0 + s * (y1 - y0)) for s, hw in reversed(prof)]
    pl = outer + inner
    if smooth:
        pl = chaikin(pl, smooth)
    return side_pts(pl, sg) if sg > 0 else [(-x, y) for x, y in pl]


def head_front(c, out):
    A, k, st, lv, R = c["A"], c["k"], c["st"], c["lv"], c["A"]["reg"]
    chin, top, hh = c["chin"], c["top"], c["hh"]
    cyh = chin + hh / 2
    hwd = 0.072 * k
    if lv == 0:
        out.append(shape(ell(0, cyh + 0.002, hwd + 0.006 * k, hh / 2 + 0.004), "sil-head", "head", "body", 5, st.merged([R["skin"], R["hair"]])))
        return
    out.append(shape(ell(0, cyh, hwd, hh / 2), "head", "head", "body", 5, st.get(R["skin"], "head")))
    hair = arc(0, cyh + 0.002, hwd + 0.007 * k, hh / 2 + 0.004, 8, 172, 10)
    line = [(-hwd * 0.97, cyh + 0.030 * k), (-0.045 * k, top - 0.052 * k), (0, top - 0.060 * k), (0.045 * k, top - 0.052 * k), (hwd * 0.97, cyh + 0.030 * k)]
    if A.get("hair_front", True):
        out.append(shape(poly(hair + line), "hair", "head", "body", 5.5, st.get(R["hair"], "head")))
    if lv >= 2:
        for sd, sg in SIDES:
            out.append(shape(ell(sg * (hwd + 0.004 * k), chin + 0.46 * hh, 0.012 * k, 0.028 * k), f"ear-{sd}", "head", "detail", 5.2, st.get(R["skin"], "head", tone=0.3), ))
    if lv >= 3:
        ey = chin + 0.58 * hh
        for sd, sg in SIDES:
            out.append(shape(ell(sg * 0.032 * k, ey, 0.009 * k, 0.0052 * k), f"eye-{sd}", "head", "detail", 5.3, st.get(R["skin"], "head", tone=0.9)))
            out.append(shape(poly([(sg * 0.018 * k, ey + 0.022 * k), (sg * 0.034 * k, ey + 0.027 * k), (sg * 0.048 * k, ey + 0.021 * k)], False), f"brow-{sd}", "head", "texture", 5.3, st.get(R["skin"], "head", tone=0.1), filled=False))
        out.append(shape(poly([(0.001, ey - 0.006 * k), (0.008 * k, ey - 0.045 * k), (-0.002, ey - 0.048 * k)], False), "nose", "head", "texture", 5.3, st.get(R["skin"], "head", tone=0.1), filled=False))
        out.append(shape(poly([(-0.022 * k, chin + 0.2 * hh), (0, chin + 0.2 * hh - 0.004 * k), (0.022 * k, chin + 0.2 * hh)], False), "mouth", "head", "texture", 5.3, st.get(R["skin"], "head", tone=0.1), filled=False))
        for i, (x0, x1) in enumerate(((-0.040, -0.052), (0.0, 0.0), (0.040, 0.052))):
            out.append(shape(poly([(x0 * k, top - 0.012 * k), (x1 * k, top - 0.048 * k)], False), f"hair-strand-{i}", "head", "texture", 5.6, st.get(R["hair"], "head", tone=0.1), filled=False))


def neck_front(c, out):
    if c["lv"] == 0:
        return
    k, st, R, n, rig = c["k"], c["st"], c["A"]["reg"], c["n"], c["rig"]
    bone = "neck" if n.get("neck") else "head"
    bot = c["nbY"] - 0.016 * k
    topn = c["chin"] + 0.05 * k
    pl = [(0.047 * k, topn), (0.047 * k, c["chin"] + 0.02 * k), (0.044 * k, bot + 0.04 * k), (0.052 * k, bot), (-0.052 * k, bot), (-0.044 * k, bot + 0.04 * k), (-0.047 * k, c["chin"] + 0.02 * k), (-0.047 * k, topn)]
    out.append(shape(poly(pl, True, follow=bone == "neck"), "neck", bone, "body", 3.8, st.get(R["skin"], bone)))


def torso_front(c, out):
    A, k, st, lv, R, rig = c["A"], c["k"], c["st"], c["lv"], c["A"]["reg"], c["rig"]
    W, Wl = c["W"], c["Wl"]
    hemY = c["waistY"] - A["hem"]
    hem_hw = A.get("hem_hw", 0.158) * k
    waist_hw = A.get("waist_hw", 0.150) * k
    chest_hw = A.get("chest_hw", W - 0.03)
    sh_hw = chest_hw - 0.004
    neck_hw = 0.050 * k
    left = [(0, hemY), (hem_hw, hemY), (hem_hw - 0.002, hemY + 0.02 * k), (waist_hw, c["waistY"]), (chest_hw - 0.006, c["shY"] - 0.20 * k), (chest_hw, c["shY"] - 0.09 * k),
            (sh_hw, c["shY"] - 0.012 * k), (sh_hw - 0.006, c["shY"] + 0.014 * k), (0.12 * k, c["nbY"] - 0.006 * k), (neck_hw, c["nbY"] + 0.004 * k), (0, c["nbY"] - 0.002 * k)]
    full = left + mir(left[::-1])[1:-1]
    sid = "sil-body" if lv == 0 else A.get("torso_id", "torso")
    out.append(shape(poly(full, True, follow=True), sid, "torso", "body", 4, st.get(R["top"], "torso")))


def arms_front(c, out):
    A, k, st, lv, R, rig, n = c["A"], c["k"], c["st"], c["lv"], c["A"]["reg"], c["rig"], c["n"]
    for sd, sg in SIDES:
        xc = c["xcl"] if sg > 0 else c["xc"]
        arm, fore, hand = f"{n['arm']}-{sd}", f"{n['forearm']}-{sd}", f"{n['hand']}-{sd}"
        if lv == 0:
            up = [(-0.10, 0.054 * k), (0, 0.056 * k), (0.5, 0.052 * k), (1.0, 0.048 * k), (1.02, 0.048 * k)]
            sf = 1.0 + (rig.oy(hand) - rig.ty(hand)) / (rig.oy(fore) - rig.ty(fore))
            lo = [(-0.05, 0.058 * k), (0.4, 0.053 * k), (1.0, 0.050 * k), (sf - 0.35, 0.047 * k), (sf - 0.1, 0.040 * k), (sf, 0.024 * k)]
            out.append(shape(poly(limb_poly(c, arm, xc, up, sg), True, True), f"sil-arm-{sd}", arm, "body", 4.5, st.get(R["sleeve"], arm)))
            out.append(shape(poly(limb_poly(c, fore, xc, lo, sg), True, True), f"sil-forearm-{sd}", fore, "body", 4.52, st.merged([R["sleeve"], R["hand"]])))
            continue
        up = [(-0.14, 0.040 * k), (-0.06, 0.047 * k), (0.12, 0.047 * k), (0.55, 0.042 * k), (1.0, 0.037 * k), (1.03, 0.036 * k)]
        out.append(shape(poly(limb_poly(c, arm, xc, up, sg), True, True), f"sleeve-{sd}", arm, "body", 4.5, st.get(R["sleeve"], arm)))
        lo = [(-0.05, 0.038 * k), (0.15, 0.038 * k), (0.55, 0.034 * k), (1.0, 0.030 * k), (1.02, 0.030 * k)]
        out.append(shape(poly(limb_poly(c, fore, xc, lo, sg), True, True), f"forearm-{sd}", fore, "body", 4.52, st.get(R["sleeve"], fore)))
        hd = [(-0.08, 0.027 * k), (0.15, 0.030 * k), (0.5, 0.031 * k), (0.78, 0.027 * k), (0.93, 0.020 * k), (1.0, 0.010 * k)]
        hl = rig.ty(hand) - rig.oy(hand)
        hpl = limb_poly(c, hand, xc, hd, sg, smooth=1)
        out.append(shape(poly(hpl, True, True), f"hand-{sd}", hand, "body", 4.7, st.get(R["hand"], hand)))
        if lv >= 3:
            for i, dx in enumerate((-0.009, 0.0, 0.009)):
                out.append(shape(poly([(sg * (xc + dx * k), rig.at(hand, 0.55)), (sg * (xc + dx * k), rig.at(hand, 0.9))], False, True), f"finger-{sd}-{i}", hand, "texture", 4.75, st.get(R["hand"], hand, tone=0.1), filled=False))


def legs_front(c, out):
    A, k, st, lv, R, rig, g = c["A"], c["k"], c["st"], c["lv"], c["A"]["reg"], c["rig"], c["g"]
    for sd, sg in SIDES:
        thigh, shank, foot, toe = f"thigh-{sd}", f"shank-{sd}", f"foot-{sd}", f"toe-{sd}"
        if lv == 0:
            xl = 0.066 * k
            tp = [(-0.10, 0.074 * k), (0, 0.078 * k), (0.5, 0.068 * k), (1.0, 0.060 * k), (1.03, 0.060 * k)]
            sp = [(-0.05, 0.058 * k), (0.2, 0.056 * k), (0.8, 0.050 * k), (1.0, 0.050 * k)]
            fw = 0.053 * k
            tid, sid, fid = f"sil-thigh-{sd}", f"sil-shank-{sd}", f"sil-shoe-{sd}"
        else:
            xl = c["xl"]
            tp = [(-0.10, 0.074 * k), (0, 0.080 * k), (0.25, 0.074 * k), (0.6, 0.064 * k), (1.0, 0.055 * k), (1.03, 0.054 * k)]
            sp = [(-0.05, 0.054 * k), (0.15, 0.056 * k), (0.45, 0.049 * k), (0.8, 0.040 * k), (1.0, 0.037 * k)]
            fw = 0.050 * k
            tid, sid, fid = f"thigh-{sd}", f"shank-{sd}", f"shoe-{sd}"
        out.append(shape(poly(limb_poly(c, thigh, xl, tp, sg), True, True), tid, thigh, "body", 1.0, st.get(R["legs"], thigh)))
        out.append(shape(poly(limb_poly(c, shank, xl, sp, sg), True, True), sid, shank, "body", 1.02, st.get(R["legs"], shank)))
        aY = c["ankleY"]
        if c["adultfoot"]:
            top = aY + 0.04 * k
            left = [(xl + 0.040 * k, top), (xl + 0.044 * k, aY), (xl + fw - 0.004 * k, g + 0.45 * (aY - g)), (xl + fw, g + 0.022 * k), (xl + fw - 0.006 * k, g), (xl - fw + 0.006 * k, g), (xl - fw, g + 0.022 * k),
                    (xl - fw + 0.004 * k, g + 0.45 * (aY - g)), (xl - 0.044 * k, aY), (xl - 0.040 * k, top)]
            out.append(shape(poly(side_pts(left, sg) if sg > 0 else mir(left), True, True), fid, foot, "body", 1.1, st.get(R["shoes"], foot)))
        else:
            out.append(shoe_solid(c, sd, sg, foot, "shoe" if lv else "sil-shoe"))


def shoe_solid(c, sd, sg, foot, sid):
    k, st, rig, R = c["k"], c["st"], c["rig"], c["A"]["reg"]
    x0 = -0.05 * k
    x1 = rig.tx(f"toe-{sd}") - rig.ox(foot) + 0.012
    y0 = c["g"] - c["ankleY"]
    xl = c["xl"]
    z = sorted((sg * (xl - 0.05 * k), sg * (xl + 0.05 * k)))
    return shape(rect(x0, y0, x1 - x0, 0.045 * k - y0, 0.02), f"{sid}-{sd}", foot, "body", 1.1, st.get(R["shoes"], foot), solid={"kind": "extruded", "z0": r4(z[0]), "z1": r4(z[1])})


def front_std(A, asset, rep, sc):
    c = ctx_of(A, asset, rep, sc)
    out = []
    legs_front(c, out)
    if A.get("pelvis_shape") and c["lv"] > 0:
        hip = c["hipY"]
        pl = [(0.162 * c["k"], hip - 0.05 * c["k"]), (0.16 * c["k"], c["waistY"] + 0.02 * c["k"]), (-0.16 * c["k"], c["waistY"] + 0.02 * c["k"]), (-0.162 * c["k"], hip - 0.05 * c["k"])]
        out.append(shape(poly(pl), "pelvis", "pelvis", "body", 3.05, c["st"].get(c["A"]["reg"]["pelvis"], "pelvis")))
    torso_front(c, out)
    neck_front(c, out)
    arms_front(c, out)
    head_front(c, out)
    if A.get("front_extras"):
        A["front_extras"](c, out)
    return out


def plan_std(A, asset, rep, sc):
    c = ctx_of(A, asset, rep, sc)
    k, st, lv, R, rig, n = c["k"], c["st"], c["lv"], A["reg"], c["rig"], c["n"]
    out = []
    ly = 0.085 * k
    for sd, sg in SIDES:
        thigh, shank, foot, toe = f"thigh-{sd}", f"shank-{sd}", f"foot-{sd}", f"toe-{sd}"
        y = sg * (0.066 * k if lv == 0 else ly)
        zf = 0 if sg > 0 else 0
        out.append(shape(cap(rig.ox(thigh), y, rig.tx(thigh), y, 0.082 * k, 0.064 * k, True), f"thigh-{sd}" if lv else f"sil-thigh-{sd}", thigh, "body", 1.05, st.get(R["legs"], thigh)))
        out.append(shape(cap(rig.ox(shank), y, rig.tx(shank), y, 0.056 * k, 0.043 * k, True), f"shank-{sd}" if lv else f"sil-shank-{sd}", shank, "body", 1.1, st.get(R["legs"], shank)))
        if not c["adultfoot"]:
            out.append(shoe_solid(c, sd, sg, foot, "shoe" if lv else "sil-shoe"))
        elif lv == 0:
            out.append(shape(cap(rig.ox(foot) - 0.02 * k, y, rig.tx(toe) - 0.045 * k + A.get("toe_extra", 0), y, 0.044 * k, 0.040 * k, True), f"sil-shoe-{sd}", foot, "body", 1.2, st.get(R["shoes"], foot)))
        else:
            fx0, fx1 = rig.ox(foot) - 0.02 * k, rig.tx(foot) + (0.0 if c["adultfoot"] else 0.0)
            out.append(shape(cap(fx0, y, fx1, y, 0.038 * k, 0.045 * k, True), f"shoe-heel-{sd}", foot, "body", 1.2, st.get(R["shoes"], foot)))
            out.append(shape(cap(rig.ox(toe), y, rig.tx(toe) - 0.045 * k + A.get("toe_extra", 0), y, 0.045 * k, 0.0396 * k, True), f"shoe-toe-{sd}", toe, "body", 1.2, st.get(R["shoes"], toe)))
        if lv >= 3:
            out.append(shape(poly([(rig.ox(foot) + 0.04 * k, y - sg * 0.022 * k), (rig.ox(foot) + 0.12 * k, y - sg * 0.018 * k)], False), f"shoe-laces-{sd}", foot, "detail", 1.4, {"tone": 0.6, "region": R["shoes"]}, filled=False))
        xc = c["xcl"] if sg > 0 else c["xc"]
        ya = sg * xc
        arm, fore, hand = f"{n['arm']}-{sd}", f"{n['forearm']}-{sd}", f"{n['hand']}-{sd}"
        out.append(shape(cap(rig.ox(arm), ya, rig.tx(arm), ya, 0.054 * k, 0.050 * k, True), f"upper-arm-{sd}" if lv else f"sil-arm-{sd}", arm, "body", 3, st.get(R["sleeve"], arm)))
        out.append(shape(cap(rig.ox(fore), ya, rig.tx(fore), ya, 0.058 * k, 0.048 * k, True), f"forearm-{sd}" if lv else f"sil-forearm-{sd}", fore, "body", 3.05, st.get(R["sleeve"], fore)))
        if lv == 0:
            out.append(shape(cap(rig.ox(hand), ya, rig.tx(hand) + 0.01 * k + A.get("hand_extra", 0), ya, 0.034 * k, 0.026 * k, True), f"sil-hand-{sd}", hand, "body", 3.1, st.get(R["hand"], hand)))
        if lv >= 1:
            out.append(shape(cap(rig.ox(hand), ya, rig.tx(hand) + 0.01 * k + A.get("hand_extra", 0), ya, 0.026 * k, 0.021 * k, True), f"hand-{sd}", hand, "body", 3.1, st.get(R["hand"], hand)))
    # trunk
    W = c["W"]
    out.append(shape(ell(0, 0, 0.105 * k, 0.165 * k) | {"cx": r4(rig.ox("pelvis")), "cy": 0}, "waist", "pelvis", "body", 3.9, st.get(R["top"], "torso")))
    out.append(shape(cap(rig.ox("torso"), 0, rig.tx("torso"), 0, 0.115 * k, 0.12 * k, True), "trunk", "torso", "body", 3.95, st.get(R["top"], "torso")))
    shb = "neck" if n.get("neck") else "head"
    shx = rig.ox(shb)
    sh_ry = min(c["xc"], c["xcl"]) + 0.03 * k
    out.append(shape(ell(shx + 0.0, 0, 0.115 * k, sh_ry), "shoulders", shb, "body", 4, st.get(R["top"], "torso")))
    hx = rig.ox("head")
    if lv >= 3:
        for sd, sg in SIDES:
            out.append(shape(poly([(shx + 0.05 * k, sg * 0.07 * k), (shx - 0.005 * k, sg * (sh_ry - 0.03 * k))], False), f"shoulder-seam-{sd}", shb, "detail", 4.2, {"tone": 0.55, "region": R["top"]}, filled=False))
    if lv >= 1 and A.get("neckwear", True):
        out.append(shape(ell(hx - 0.008 * k, 0, 0.065 * k, 0.088 * k), "neckwear", shb, "body", 5, st.get(A.get("neckwear_region", R["top"]), "torso")))
    if lv >= 1:
        out.append(shape(ell(hx - 0.005 * k, 0, 0.09 * k, 0.074 * k), "head", "head", "body", 6, st.get(R["hair"], "head")))
    else:
        out.append(shape(ell(hx - 0.005 * k, 0, 0.09 * k, 0.074 * k), "head", "head", "body", 6, st.get(R["hair"], "head")))
    if lv >= 2:
        out.append(shape(ell(hx + 0.05 * k, 0, 0.0324 * k, 0.0533 * k), "face", "head", "body", 6.1, st.get(R["skin"], "head")))
    if lv >= 3:
        out.append(shape(cap(hx + 0.085 * k, 0, hx + 0.1 * k, 0, 0.011 * k, 0.008 * k), "nose", "head", "detail", 6.3, st.get(R["skin"], "head")))
        for sd, sg in SIDES:
            out.append(shape(ell(hx - 0.005 * k, sg * 0.077 * k, 0.012 * k, 0.007 * k), f"ear-{sd}", "head", "detail", 6.2, st.get(R["skin"], "head")))
        out.append(shape(poly([(hx - 0.018 * k, -0.033 * k), (hx - 0.049 * k, 0.007 * k), (hx - 0.067 * k, 0.026 * k)], False), "hair-parting", "head", "detail", 6.3, {"tone": 0.5}, filled=False))
    if A.get("plan_extras"):
        A["plan_extras"](c, out)
    return out
