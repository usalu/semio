"""NW-H1: shared handcrafted drawing parts of the human figure (head profile, hand, shoe) used by the four asset scripts. Every asset/scale composes its own shapes
from these parts with its own numbers; this module only holds the common outlines of the face profile and the foot geometry."""
import math
from nwlib import *

# head profile, normalised: x forward, y up, unit = head height (chin to vertex), origin = head-bone origin (neck top, ~ at the jaw angle)
PROFILE = [(0.02, 0.90), (0.20, 0.84), (0.31, 0.72), (0.345, 0.58), (0.35, 0.49), (0.40, 0.40), (0.47, 0.325), (0.41, 0.275), (0.405, 0.235), (0.39, 0.20),
           (0.40, 0.165), (0.385, 0.13), (0.36, 0.06), (0.30, -0.045), (0.20, -0.10), (0.06, -0.10), (-0.05, -0.02), (-0.20, 0.10), (-0.30, 0.30), (-0.36, 0.52), (-0.33, 0.72), (-0.22, 0.86)]
# coarse profile for small scales (forehead, nose, chin kept; lips merged)
PROFILE_COARSE = [(0.02, 0.90), (0.26, 0.80), (0.345, 0.58), (0.36, 0.45), (0.47, 0.325), (0.40, 0.27), (0.39, 0.12), (0.30, -0.045), (0.12, -0.10), (-0.05, -0.02), (-0.30, 0.30), (-0.36, 0.55), (-0.25, 0.84)]


def head_pts(h, prof, k=1.0, dx=0.0, dy=0.0):
    return [h.head_pt(p[0] * k + dx, p[1] * k + dy) + tuple(p[2:]) for p in prof]


def shoe_outline(h, kind, heel_h=0.02, toe_k=1.0, ankle_open=0.0):
    """shoe silhouette in rest-world coordinates (x forward from ankle x0, sole on y=0). kind 'lace' (ankle shoe), 'sneaker', 'boot'.
    returns list of points (x,y[,'c'])"""
    a = h.rest["foot-l"]
    ax = a.x
    hb = h.heel_back
    L = h.Lball + h.Ltoe * 1.45
    ah = h.ah
    top = ah + 0.018
    back = ax - hb - 0.012
    if kind == "sneaker":
        return [(back, 0.0, 'c'), (ax + h.Lball * 0.3, 0.0, 'c'), (ax + h.Lball, 0.0, 'c'), (ax + L - 0.012, 0.0, 'c'), (ax + L + 0.006, 0.014), (ax + L - 0.004, 0.034), (ax + h.Lball + 0.03, 0.050),
                (ax + 0.07, ah + 0.004), (ax + 0.035, top + 0.016), (ax - 0.032, top + 0.016), (ax - hb + 0.002, ah * 0.9), (back, 0.040)]
    if kind == "boot":
        return [(back, 0.0, 'c'), (ax + L - 0.015, 0.0, 'c'), (ax + L + 0.012, 0.024), (ax + L - 0.008, 0.062), (ax + h.Lball + 0.04, 0.085), (ax + 0.08, ah + 0.02), (ax + 0.05, top + 0.05),
                (ax - 0.055, top + 0.05), (back + 0.004, ah * 0.7)]
    return [(back, 0.0, 'c'), (ax + L - 0.014, 0.0, 'c'), (ax + L + 0.006, 0.016), (ax + L - 0.006, 0.040), (ax + h.Lball + 0.025, 0.056), (ax + 0.07, ah + 0.002),
            (ax + 0.04, top + 0.012), (ax - 0.04, top + 0.012), (ax - hb + 0.004, ah * 0.85), (back, 0.045)]


def hand_tube(h, REST, side):
    b = "hand-" + side
    p0 = REST[b].ap(0, 0)
    p1 = REST[b].ap(h.Lha, 0)
    st = [(-0.1, .026, .026), (0.12, .03, .032), (0.4, .036, .04), (0.62, .036, .042), (0.84, .03, .038), (1.0, .018, .026)]
    return tube(p0, p1, st, 0.0, 0.5)


def dress(sh, h, rig, REST, o, level):
    """Compose the clothed human of outfit `o` at drawing level 50/100/200/500 (rest-world coordinates -> bones).
    o: dict(reg={skin,hair,hat,hands,upper,lower,shoes}, tone={...}, torso, hip, skirt|None, sleeve_up, sleeve_fore, thigh, shank, hat_pts, hat_c, hair_pts, shoe, hatch, glasses, nub, buttons, far_hatch, near_only)"""
    A = sh.add
    R = o["reg"]
    T_ = o["tone"]
    HD = h.hd

    def seg(bone):
        return REST[bone].ap(0, 0), REST[bone].ap(rig.by[bone]["lengthM"], 0)

    def down(bone, st, c0=0.0, c1=0.0):
        p0, p1 = seg(bone)
        return tube(p0, p1, [(t, front, back) for t, back, front in st], c0, c1)

    def up(bone, st, c0=0.0, c1=0.0):
        p0, p1 = seg(bone)
        return tube(p0, p1, [(t, back, front) for t, back, front in st], c0, c1)

    def line(bone, pts):
        return sh.poly(bone, pts, False)

    def hp(u, v):
        return h.head_pt(u, v)

    hx = o.get("hatch")
    fh = o.get("far_hatch")
    if level == 500:
        # one near arm only; legs as overlapping filled masses; coat mass includes collar
        for s in ("l", "r"):
            far = s == "l"
            zf = 1.0 if far else 6.0
            tn = T_["lower_far"] if far else T_["lower"]
            A("thigh-" + s, "thigh-" + s, sh.path("thigh-" + s, down("thigh-" + s, o["thigh500"])), "body", zf + 0.1, True, tn, region=R["lower"])
            A("shank-" + s, "shank-" + s, sh.path("shank-" + s, down("shank-" + s, o["shank500"])), "body", zf + 0.2, True, tn, region=R["lower"])
            isb = s == o.get("boot_side")
            so = shoe_outline(h, o["boot_shoe"] if isb else o["shoe"])
            A("shoe-" + s, "foot-" + s, sh.path("foot-" + s, so), "body", zf + 0.3, True, T_["boot"] if isb else T_["shoes"], region=R["boot"] if isb else R["shoes"])
            if isb and o.get("boot_shaft500"):
                A("boot-shaft-" + s, "shank-" + s, sh.path("shank-" + s, down("shank-" + s, o["boot_shaft500"])), "body", zf + 0.25, True, T_["boot"], region=R["boot"])
            if o.get("skirt"):
                A("skirt-" + s, "thigh-" + s, sh.path("thigh-" + s, down("thigh-" + s, o["skirt500"])), "body", zf + 0.5, True, T_.get("skirt_far", T_["upper_far"]) if far else T_.get("skirt", T_["upper"]), region=R.get("skirt", R["upper"]))
        A("torso", "torso", sh.path("torso", up("torso", o["torso500"], 0.0, 0.3)), "body", 5.0, True, T_["upper"], region=R["upper"])
        A("hip", "pelvis", sh.path("pelvis", up("pelvis", o["hip500"])), "body", 5.1, True, T_["upper"], region=R["upper"])
        A("arm", "arm-r", line("arm-r", [REST["arm-r"].ap(0, 0), REST["arm-r"].ap(rig.by["arm-r"]["lengthM"], 0)]), "body", 7.0, False, 0.5, region=R["upper"], pen=o["pen_arm500"])
        A("forearm", "forearm-r", line("forearm-r", [REST["forearm-r"].ap(0, 0), REST["forearm-r"].ap(rig.by["forearm-r"]["lengthM"], 0)]), "body", 7.1, False, 0.5, region=R["upper"], pen=o["pen_arm500"])
        A("hand", "hand-r", line("hand-r", [REST["hand-r"].ap(0, 0), REST["hand-r"].ap(h.Lha * 0.8, 0)]), "body", 7.2, False, 0.5, region=R["hands"], pen=o["pen_hand500"])
        A("head", "head", sh.path("head", head_pts(h, PROFILE_COARSE)), "body", 6.0, True, T_["skin"], regions=[R["skin"], R["hair"]])
        if o.get("hat_c") is not None: A("hat", "head", sh.path("head", head_pts(h, o["hat_c"])), "body", 6.5, True, T_["hat"], region=R["hat"])
        return
    for s in ("l", "r"):
        far = s == "l"
        zf = 1.0 if far else 6.0
        tl = T_["lower_far"] if far else T_["lower"]
        tu = T_["upper_far"] if far else T_["upper"]
        hu = fh if far else hx
        if level in (50, 100):
            A("thigh-g-" + s, "thigh-" + s, sh.path("thigh-" + s, down("thigh-" + s, o["thigh"])), "body", zf + 0.1, True, tl, region=R["lower"])
            A("shank-g-" + s, "shank-" + s, sh.path("shank-" + s, down("shank-" + s, o["shank"])), "body", zf + 0.2, True, tl, region=R["lower"])
        else:
            A("thigh-g-" + s, "thigh-" + s, sh.path("thigh-" + s, down("thigh-" + s, o["thigh"])), "body", zf + 0.1, True, tl, region=R["lower"])
            A("shank-g-" + s, "shank-" + s, sh.path("shank-" + s, down("shank-" + s, o["shank"])), "body", zf + 0.2, True, tl, region=R["lower"])
        if level == 50 and o.get("cuff"):
            A("cuff-" + s, "shank-" + s, line("shank-" + s, [REST["shank-" + s].ap(o["cuff"], 0.052), REST["shank-" + s].ap(o["cuff"], -0.047)]), "detail", zf + 0.25, False, 0.4, region=R["lower"])
        if o.get("skirt"):
            A("skirt-" + s, "thigh-" + s, sh.path("thigh-" + s, down("thigh-" + s, o["skirt"])), "body", zf + 0.5, True, (T_.get("skirt_far", tu) if far else T_.get("skirt", tu)), region=R.get("skirt", R["upper"]), hatch=o.get("skirt_hatch") if level <= 100 else None)
        isb = s == o.get("boot_side")
        so = shoe_outline(h, o["boot_shoe"] if isb else o["shoe"])
        sreg = R["boot"] if isb else R["shoes"]
        stone = T_["boot"] if isb else T_["shoes"]
        if isb and o.get("boot_shaft"):
            A("boot-shaft-" + s, "shank-" + s, sh.path("shank-" + s, down("shank-" + s, o["boot_shaft"])), "body", zf + 0.25, True, stone, region=sreg)
            if level == 50:
                for i, t_ in enumerate((0.55, 0.68, 0.81)):
                    A("boot-strap-%d" % i, "shank-" + s, line("shank-" + s, [REST["shank-" + s].ap(h.Lsh * t_, 0.062), REST["shank-" + s].ap(h.Lsh * t_, -0.062)]), "detail", zf + 0.27, False, 0.6, region=sreg)
        ball = REST["foot-" + s].x + h.Lball
        if level in (50, 100):
            A("shoe-rear-" + s, "foot-" + s, sh.path("foot-" + s, clip_x(so, ball, 'lt')), "body", zf + 0.3, True, stone, region=sreg)
            A("shoe-front-" + s, "toe-" + s, sh.path("toe-" + s, clip_x(so, ball, 'gt')), "body", zf + 0.32, True, stone, region=sreg)
        else:
            A("shoe-" + s, "foot-" + s, sh.path("foot-" + s, so), "body", zf + 0.3, True, stone, region=sreg)
        ax = REST["foot-" + s].x
        if level == 50:
            A("sole-" + s, "foot-" + s, line("foot-" + s, [(ax - 0.058, 0.012), (ax + 0.10, 0.012)]), "detail", zf + 0.4, False, 0.4, region=sreg)
            for i, lx in enumerate((0.02, 0.05, 0.08)):
                A("lace-%s%d" % (s, i), "foot-" + s, line("foot-" + s, [(ax + lx, h.ah + 0.010), (ax + lx + 0.016, h.ah - 0.014)]), "detail", zf + 0.4, False, 0.5, region=sreg)
        A("sleeve-up-" + s, "arm-" + s, sh.path("arm-" + s, down("arm-" + s, o["sleeve_up"], 0.6, 0.0)), "body", zf + 1.5, True, tu, region=R["upper"], hatch=hu if level <= 100 else None)
        A("sleeve-fore-" + s, "forearm-" + s, sh.path("forearm-" + s, down("forearm-" + s, o["sleeve_fore"])), "body", zf + 1.6, True, tu, region=R["upper"], hatch=hu if level <= 100 else None)
        if level == 50:
            A("cuff-s-" + s, "forearm-" + s, line("forearm-" + s, [REST["forearm-" + s].ap(0.20, 0.04), REST["forearm-" + s].ap(0.20, -0.04)]), "detail", zf + 1.7, False, 0.5, region=R["upper"])
        th = 0.82 if not far else 0.7
        A("hand-" + s, "hand-" + s, sh.path("hand-" + s, hand_tube(h, REST, s)), "body", zf + 4.0, True, th, region=R["hands"])
        if level in (50, 100):
            A("thumb-" + s, "hand-" + s, sh.capsule("hand-" + s, REST["hand-" + s].ap(0.02, 0.025), REST["hand-" + s].ap(0.10, 0.035), 0.012, 0.010), "body", zf + 4.1, True, th, region=R["hands"])
        if level == 50:
            for i, t in enumerate((0.45, 0.65, 0.82)):
                A("finger-%s%d" % (s, i), "hand-" + s, line("hand-" + s, [REST["hand-" + s].ap(h.Lha * t, 0.044 - 0.004 * i), REST["hand-" + s].ap(h.Lha * (t + 0.10), 0.038 - 0.004 * i)]), "detail", zf + 4.2, False, 0.5, region=R["hands"])
    A("torso", "torso", sh.path("torso", up("torso", o["torso"], 0.0, 0.4)), "body", 5.0, True, T_["upper"], region=R["upper"], hatch=hx if level <= 100 else None)
    A("hip", "pelvis", sh.path("pelvis", up("pelvis", o["hip"])), "body", 5.1, True, T_["upper"], region=R["upper"], hatch=hx if level <= 100 else None)
    if level in (50, 100):
        A("collar", "neck", sh.path("neck", up("neck", o["collar"])), "body", 5.4, True, T_["collar"], region=R["upper"], hatch=hx if level == 50 else None)
        A("neck", "neck", sh.path("neck", up("neck", [(-0.3, .042, .040), (0.5, .040, .038), (1.2, .040, .040)])), "body", 4.8, True, T_["skin"], region=R["skin"])
    else:
        A("neck", "neck", sh.path("neck", up("neck", [(-0.4, .060, .062), (0.5, .045, .045), (1.2, .040, .040)])), "body", 4.8, True, T_["upper"], regions=[R["upper"], R["skin"]])
    if level == 50:
        A("front", "torso", line("torso", [REST["torso"].ap(0.372, -0.040), REST["torso"].ap(0.02, -0.108)]), "detail", 5.2, False, 0.5, region=R["upper"]) if o.get("buttons") else None
        if o.get("buttons"):
            for i, t in enumerate((0.30, 0.52, 0.74)):
                c = REST["torso"].ap(h.Lt * t, -0.108 if t < 0.4 else -0.118)
                A("button-%d" % i, "torso", sh.ellipse("torso", c, 0.009, 0.009), "detail", 5.3, True, 0.15, region=R["upper"])
    prof = PROFILE if level in (50, 100) else PROFILE_COARSE
    if level in (50, 100):
        A("head", "head", sh.path("head", head_pts(h, prof)), "body", 6.0, True, T_["skin"], region=R["skin"])
        A("hair", "head", sh.path("head", head_pts(h, o["hair_pts"])), "body", 6.3, True, T_["hair"], region=R["hair"])
        if o.get("hat_c") is not None: A("hat", "head", sh.path("head", head_pts(h, o["hat_pts"] if level == 50 else o["hat_c"])), "body", 6.5, True, T_["hat"], region=R["hat"], hatch=o.get("hat_hatch"))
        A("ear", "head", sh.ellipse("head", hp(-0.04, 0.38), 0.05 * HD, 0.11 * HD), "detail", 6.2, True, 0.78, region=R["skin"])
        if level == 50:
            if o.get("nub"):
                A("hat-nub", "head", sh.ellipse("head", hp(0.02, 1.07), 0.025, 0.020), "detail", 6.6, True, 0.2, region=R["hat"])
            if o.get("glasses"):
                A("glasses", "head", sh.path("head", circle_pts(hp(0.275, 0.47), 0.085 * HD, 10, 0.07 * HD), True), "detail", 6.4, False, 0.5, region=R["skin"])
                A("glasses-arm", "head", line("head", [hp(0.19, 0.50), hp(-0.03, 0.50)]), "detail", 6.4, False, 0.5, region=R["skin"])
            A("mouth", "head", line("head", [hp(0.385, 0.165), hp(0.33, 0.16)]), "detail", 6.4, False, 0.5, region=R["skin"])
    else:
        A("head", "head", sh.path("head", head_pts(h, prof)), "body", 6.0, True, T_["skin"], regions=[R["skin"], R["hair"]])
        if o.get("hat_c") is not None: A("hat", "head", sh.path("head", head_pts(h, o["hat_c"])), "body", 6.5, True, T_["hat"], region=R["hat"])
