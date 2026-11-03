"""H1 scratch: handcrafted front and plan outline of a rigged human, parametrised per asset and per scale.

`human(c, L, over)` adds shapes to `c.front` and `c.plan`. Every number comes from the asset's overrides (defaults here are only the
adult baseline); the level of detail follows the scale denominator L: 500 composed silhouette, 200 main masses, 100 articulated,
50 full detail. Coordinates are rest-pose model metres (root at the origin), lateral: left is +.
"""
import math

DEFAULTS = dict(
    arm="arm", neck=True, neckT=0.3,
    lat=dict(hip=0.095, knee=0.095, ankle=0.095, shoulder=0.18, elbow=0.18, wrist=0.18),
    armR=0.05,
    headDx=0.0, headHalfD=0.095, headHalfW=0.077, faceDx=0.057, skullRy=0.112, skullRx=0.077, headTopDy=0.0,
    plan=dict(thigh=(0.078, 0.06), shank=(0.058, 0.043), foot=(0.04, 0.046), toe=(0.046, 0.04), upper=(0.052, 0.046), fore=(0.045, 0.034),
              hand=(0.03, 0.022), hip=(0.12, 0.17), trunk=(0.12, 0.12), shoulder=(0.11, 0.2), heel=0.02),
    front=dict(
        thigh=[(0, 0.088), (0.45, 0.078), (1, 0.058)], shank=[(0, 0.056), (0.3, 0.058), (0.75, 0.042), (1, 0.038)],
        foot=[(0, 0.04), (0.6, 0.047), (1, 0.047)], toe=[(0, 0.047), (1, 0.041)],
        torso=[(0, 0.165), (0.3, 0.148), (0.62, 0.17), (0.9, 0.19)], neck=[(0, 0.052), (1, 0.047)],
        upper=[(0, 0.052), (0.5, 0.048), (1, 0.042)], fore=[(0, 0.042), (0.35, 0.044), (1, 0.032)], hand=[(0, 0.032), (0.5, 0.04), (1, 0.022)],
        pelvisW=0.17, shoeCap=0.035, toeCap=0.04, handCap=0.03),
    R=dict(thigh="trousers", shank="trousers", foot="shoes", toe="shoes", hip="trousers", torso="jacket", upper="jacket", fore="jacket", hand="gloves", neck="face-skin", face="face-skin", hair="hair"),
    zf=dict(thigh=1.0, shank=1.05, knee=1.08, foot=1.2, toe=1.22, pelvis=3.0, torso=4.0, neck=3.8, upper=4.5, fore=4.55, hand=4.6, head=6.0, hair=6.2),
    zp=dict(thigh=1.05, shank=1.1, foot=1.2, toe=1.2, upper=3.0, fore=3.05, hand=3.1, waist=3.9, trunk=3.95, shoulders=4.0, neck=5.0, head=6.0, face=6.1),
)


def merge(base, over):
    out = dict(base)
    for k, v in over.items():
        out[k] = merge(base[k], v) if isinstance(v, dict) and isinstance(base.get(k), dict) else v
    return out


def mid(a, b, t=0.5):
    return (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)


def arc(cx, cy, rx, ry, a0, a1, n=10):
    return [(cx + rx * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + ry * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def limb2(c, id, bone, x0, x1, prof, **kw):
    """Front limb whose axis runs from lateral x0 (bone origin) to x1 (bone tip): a sheared outline (still bound followTip)."""
    (ox, oy), (tx, ty) = c.O[bone], c.T[bone]
    t0, t1 = kw.pop("t0", 0.0), kw.pop("t1", 1.0)
    cap0, cap1 = kw.pop("cap0", 0.0), kw.pop("cap1", 0.0)
    ws = kw.pop("wscale", 1.0)

    def yy(t):
        return oy + t * (ty - oy)

    def xx(t):
        return x0 + (x1 - x0) * t

    def w_at(t):
        if t <= prof[0][0]:
            return prof[0][1]
        for (ta, wa), (tb, wb) in zip(prof, prof[1:]):
            if ta <= t <= tb:
                return wa + (wb - wa) * (t - ta) / (tb - ta) if tb > ta else wa
        return prof[-1][1]

    ts = [t0] + [t for t, _ in prof if t0 < t < t1] + [t1]
    sgn = 1 if ty <= oy else -1
    left = [(xx(t) - ws * w_at(t), yy(t)) for t in ts]
    right = [(xx(t) + ws * w_at(t), yy(t)) for t in reversed(ts)]
    n = 6
    bot = [(xx(t1) - ws * w_at(t1) * math.cos(math.pi * i / n), yy(t1) - sgn * cap1 * math.sin(math.pi * i / n)) for i in range(1, n)] if cap1 > 0 else []
    top = [(xx(t0) + ws * w_at(t0) * math.cos(math.pi * i / n), yy(t0) + sgn * cap0 * math.sin(math.pi * i / n)) for i in range(1, n)] if cap0 > 0 else []
    c.poly("front", id, bone, left + bot + right + top, follow=True, **kw)


def human(c, L, over=None):
    cfg = merge(DEFAULTS, over or {})
    if L >= 500:
        cfg["plan"] = {k: (tuple(max(x, 0.053) for x in v) if isinstance(v, tuple) and k not in ("footX", "toeX", "hip", "trunk", "shoulder") else v) for k, v in cfg["plan"].items()}
        cfg["front"] = {k: ([(t, max(w, 0.053)) for t, w in v] if isinstance(v, list) and k not in ("torso",) else v) for k, v in cfg["front"].items()}
    if L >= 500:
        cfg["zp"] = dict(cfg["zp"], fore=4.05, hand=4.06)
    P, F, R, Z, ZP, LAT = cfg["plan"], cfg["front"], cfg["R"], cfg["zf"], cfg["zp"], cfg["lat"]
    O, T = c.O, c.T
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    A = cfg["arm"]
    sb = cfg.get("shoulderBone") or ("neck" if cfg["neck"] else f"{A}-l")
    bottom = min(min(O["toe-l"][1], T["toe-l"][1]), min(O["toe-r"][1], T["toe-r"][1]))
    for s, sg in (("l", 1), ("r", -1)):
        th, sh, ft, to = f"thigh-{s}", f"shank-{s}", f"foot-{s}", f"toe-{s}"
        ua, fa, ha = f"{A}-{s}", f"forearm-{s}", f"hand-{s}"
        hipY, kneeY, ankY = sg * LAT["hip"], sg * LAT["knee"], sg * LAT["ankle"]
        shY, elY, wrY = sg * LAT["shoulder"], sg * LAT["elbow"], sg * LAT["wrist"]
        # ------------------------------------------------------------------ plan: legs
        c.cap("plan", f"thigh-{s}", th, (O[th][0], hipY), (T[th][0], kneeY), *P["thigh"], z=ZP["thigh"], region=R["thigh"])
        c.cap("plan", f"shank-{s}", sh, (O[sh][0], kneeY), (T[sh][0], ankY), *P["shank"], z=ZP["shank"], region=R["shank"])
        fx = P.get("footX", (O[ft][0] - P["heel"], T[ft][0]))
        tx_ = P.get("toeX", (O[to][0], T[to][0]))
        c.cap("plan", f"foot-{s}", ft, (fx[0], ankY), (fx[1], ankY), *P["foot"], z=ZP["foot"], region=R["foot"])
        if True:
            c.cap("plan", f"toe-{s}", to, (tx_[0], ankY), (tx_[1], ankY), *P["toe"], z=ZP["toe"], region=R["toe"])
        # ------------------------------------------------------------------ plan: arms
        c.cap("plan", f"upper-arm-{s}", ua, (O[ua][0], shY), (T[ua][0], elY), *P["upper"], z=ZP["upper"], region=R["upper"])
        c.cap("plan", f"forearm-{s}", fa, (O[fa][0], elY), (T[fa][0], wrY), *P["fore"], z=ZP["fore"], region=R["fore"])
        c.cap("plan", f"hand-{s}", ha, (O[ha][0], wrY), (T[ha][0], wrY), *P["hand"], z=ZP["hand"], region=R["hand"])
        # ------------------------------------------------------------------ front: legs
        limb2(c, f"thigh-{s}", th, hipY, kneeY, F["thigh"], cap0=0.04, z=Z["thigh"], region=R["thigh"])
        limb2(c, f"shank-{s}", sh, kneeY, ankY, F["shank"], z=Z["shank"], region=R["shank"])
        if L <= 200:
            c.ell("front", f"knee-{s}", sh, (kneeY, O[sh][1]), F["shank"][0][1] * 0.95, 0.05, z=Z["knee"], region=R["shank"])
        drop = max(0.0, T[ft][1] - bottom) if abs(T[ft][1] - O[ft][1]) < 0.02 else 0.0
        limb2(c, f"foot-{s}", ft, ankY, ankY, F["foot"], cap0=F["shoeCap"], cap1=drop, z=Z["foot"], region=R["foot"])
        if L <= 200:
            limb2(c, f"toe-{s}", to, ankY, ankY, F["toe"], cap0=F["toeCap"], z=Z["toe"], region=R["toe"])
        # ------------------------------------------------------------------ front: arms
        limb2(c, f"upper-arm-{s}", ua, shY, elY, F["upper"], cap0=0.045, cap1=0.04, z=Z["upper"], region=R["upper"])
        limb2(c, f"forearm-{s}", fa, elY, wrY, F["fore"], cap1=0.06 if L >= 500 else 0.03, z=Z["fore"], region=R["fore"])
        limb2(c, f"hand-{s}", ha, wrY, wrY, F["hand"], cap1=0.09 if L >= 500 else F["handCap"], z=Z["hand"], region=R["hand"])
    # ---------------------------------------------------------------------- plan: trunk and head
    tb = "torso"
    if L < 500:
        c.ell("plan", "waist", "pelvis", (O["pelvis"][0], 0), P["hip"][0], P["hip"][1], z=ZP["waist"], region=R["hip"])
    c.cap("plan", "trunk", tb, (O[tb][0], 0), (T[tb][0], 0), P["trunk"][0], P["trunk"][1], z=ZP["trunk"], region=R["torso"])
    shoulder_regions = [R["torso"], R["hip"]] if L >= 500 else None
    sh_kw = dict(regions=sorted(set(shoulder_regions))) if shoulder_regions else dict(region=R["torso"])
    c.ell("plan", "shoulders", sb, (O[sb][0] + cfg.get("shoulderDx", 0.0), 0), P["shoulder"][0], LAT["shoulder"] + cfg["armR"] - 0.005, z=ZP["shoulders"], **sh_kw)
    hx = mid(O["head"], T["head"])[0] + cfg["headDx"]
    if L >= 500:
        c.ell("plan", "head", "head", (hx, 0), cfg["headHalfD"], cfg["headHalfW"], z=ZP["head"], regions=sorted({R["hair"], R["face"]}))
    else:
        c.ell("plan", "head", "head", (hx, 0), cfg["headHalfD"], cfg["headHalfW"], z=ZP["head"], region=R["hair"])
        c.ell("plan", "face", "head", (hx + cfg["faceDx"], 0), 0.033, 0.053, z=ZP["face"], region=R["face"])
    if mid_ or fine:
        for s, sg in (("l", 1), ("r", -1)):
            c.ell("plan", f"ear-{s}", "head", (hx - 0.01, sg * (cfg["headHalfW"] + 0.0)), 0.012, 0.007, z=6.2, region=R["face"], role="detail")
    if fine:
        c.cap("plan", "nose", "head", (hx + cfg["headHalfD"] - 0.002, 0), (hx + cfg["headHalfD"] + 0.014, 0), 0.011, 0.008, follow=False, z=6.3, region=R["face"], role="detail")
    if cfg["neck"] and L <= 100:
        c.ell("plan", "neck", "neck", (O["neck"][0] + 0.005, 0), 0.065, 0.075, z=ZP["neck"], region=R["neck"])
    # ---------------------------------------------------------------------- front: trunk and head
    if L < 500:
        pass
    c.poly("front", "pelvis", "pelvis", _pelvis(c, cfg), z=Z["pelvis"], region=R["hip"])
    tw = F["torso"]
    (ox, oy), (tx, ty) = O[tb], T[tb]
    pts = [(-w, oy + t * (ty - oy)) for t, w in tw] + [(-0.07, ty + 0.02), (0.07, ty + 0.02)] + [(w, oy + t * (ty - oy)) for t, w in reversed(tw)]
    c.poly("front", "torso", tb, pts, follow=True, z=Z["torso"], region=R["torso"])
    top = T["head"][1] + cfg["headTopDy"]
    hc = top - cfg["skullRy"]
    if L >= 500:
        c.ell("front", "head", "head", (0, hc), cfg["skullRx"], cfg["skullRy"], z=Z["head"], regions=sorted({R["hair"], R["face"]}))
    else:
        if cfg["neck"]:
            if L <= 200:
                limb2(c, "neck", "neck", 0.0, 0.0, F["neck"], cap1=0.0, z=Z["neck"], region=R["neck"])
        elif L <= 200:
            limb2(c, "neck", "head", 0.0, 0.0, [(0, F["neck"][0][1]), (1, F["neck"][-1][1])], t1=cfg["neckT"], z=Z["neck"], region=R["neck"])
        c.ell("front", "head", "head", (0, hc), cfg["skullRx"], cfg["skullRy"], z=Z["head"], region=R["face"])
        rx, ry = cfg["skullRx"], cfg["skullRy"]
        if cfg.get("hairFront", True):
            hl = cfg.get("hairline", 0.05)
            pts = arc(0, hc, rx + 0.004, ry + 0.004, -8, 188, 14) + [(-rx * 0.96, hc + 0.015), (-rx * 0.7, hc + hl), (0, hc + hl - 0.012), (rx * 0.7, hc + hl), (rx * 0.96, hc + 0.015)]
            c.poly("front", "hair", "head", pts, z=Z["hair"], region=R["hair"])
        if L <= 100:
            for s, sg in (("l", 1), ("r", -1)):
                c.ell("front", f"ear-{s}", "head", (sg * (rx + 0.006), hc - 0.004), (0.012 if L == 100 else 0.009), 0.024, z=Z["head"] - 0.1, region=R["face"], role="detail" if False else "body")
        if fine and cfg.get("face", True):
            f = cfg.get("faceY", dict(eye=-0.004, brow=0.016, nose=-0.04, mouth=-0.072))
            for s, sg in (("l", 1), ("r", -1)):
                c.ell("front", f"eye-{s}", "head", (sg * 0.03, hc + f["eye"]), 0.009, 0.006, z=6.3, region=R["face"], role="detail")
                c.poly("front", f"brow-{s}", "head", [(sg * 0.05, hc + f["brow"] - 0.003), (sg * 0.03, hc + f["brow"] + 0.003), (sg * 0.012, hc + f["brow"] - 0.001)], closed=False, z=6.3, region=R["face"], role="detail")
            c.poly("front", "nose", "head", [(0, hc + f["eye"] - 0.005), (-0.008, hc + f["nose"] + 0.004), (0.0, hc + f["nose"]), (0.008, hc + f["nose"] + 0.004)], closed=False, z=6.3, region=R["face"], role="detail")
            c.poly("front", "mouth", "head", [(-0.017, hc + f["mouth"]), (0.0, hc + f["mouth"] - 0.003), (0.017, hc + f["mouth"])], closed=False, z=6.3, region=R["face"], role="detail")
    return cfg


def _pelvis(c, cfg):
    F = cfg["front"]
    O = c.O
    yt = cfg.get("pelvisYt", O["pelvis"][1] + 0.09)
    yb = cfg.get("pelvisYb", O["thigh-l"][1] - 0.07)
    w = F["pelvisW"]
    return [(-w, yb), (w, yb), (w + 0.005, (yb + yt) / 2), (w - 0.01, yt), (-w + 0.01, yt), (-w - 0.005, (yb + yt) / 2)]
