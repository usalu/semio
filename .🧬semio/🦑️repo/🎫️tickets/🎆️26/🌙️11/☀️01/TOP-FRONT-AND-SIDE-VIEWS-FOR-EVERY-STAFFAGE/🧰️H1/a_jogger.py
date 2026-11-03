import sys
from lib import Ctx
from human import human, merge, DEFAULTS
from texts import texts

CFG = dict(
    arm="upper-arm", neck=False, neckT=0.3,
    lat=dict(hip=0.09, knee=0.09, ankle=0.09, shoulder=0.175, elbow=0.175, wrist=0.175), armR=0.052,
    headDx=-0.006, headHalfD=0.101, headHalfW=0.078, faceDx=0.055, skullRy=0.108, skullRx=0.077, headTopDy=0.018, hairline=0.055,
    plan=dict(footX=(-0.032, 0.09), toeX=(0.14, 0.19), toe=(0.046, 0.038), hip=(0.124, 0.17), trunk=(0.128, 0.128), shoulder=(0.11, 0.2)),
    front=dict(torso=[(-0.15, 0.165), (0.0, 0.165), (0.3, 0.15), (0.62, 0.172), (0.9, 0.19)], toeCap=0.068, pelvisW=0.17),
    pelvisYb=-0.12, pelvisYt=0.108,
    R=dict(thigh="leg-skin", shank="leg-skin", foot="shoes", toe="shoes", hip="leg-clothing", torso="torso-clothing", upper="arm-skin", fore="arm-skin", hand="hand-skin", neck="head-skin", face="head-skin", hair="hair"),
)

EXTRA = {
    500: dict(
        merged=("The sleeve joins the arm shape, the shorts join the thigh shape and the sock the shank shape; head-band and hair share the head shape.", "Der Ärmel geht in der Armform, die Shorts gehen in der Oberschenkelform und die Socke geht in der Unterschenkelform auf; Stirnband und Haar teilen sich die Kopfform."),
    ),
    200: dict(
        retained=("The sleeve and the shorts are separate shapes over the bare arm and thigh; the shirt hem reaches below the waist.", "Ärmel und Shorts sind eigene Formen über dem nackten Arm und Oberschenkel; der Shirtsaum reicht unter die Taille."),
    ),
    100: dict(
        retained=("The sleeve, shorts, sock top, head-band and the arm-band are kept as layers; the legs keep their calf swell.", "Ärmel, Shorts, Sockenrand, Stirnband und Armband bleiben als Schichten erhalten; die Beine behalten ihre Wadenschwellung."),
    ),
    50: dict(
        retained=("Shirt collar, hem and stripe, shorts hem and side stripe, sock top, shoe laces, head-band, eyes, brows, nose and mouth are drawn; the plan adds shoulder seams, head-band ring, nose and ears.", "Shirtkragen, Saum und Streifen, Shortssaum und Seitenstreifen, Sockenrand, Schnürung, Stirnband, Augen, Brauen, Nase und Mund sind gezeichnet; der Grundriss ergänzt Schulternähte, Stirnbandring, Nase und Ohren."),
    ),
}


def details(c, L):
    O, T = c.O, c.T
    F, LAT = CFG["front"], CFG["lat"]
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    for s, sg in (("l", 1), ("r", -1)):
        c.retag("plan", f"thigh-{s}", "leg-clothing")
        c.retag("plan", f"upper-arm-{s}", "arm-clothing")
    hip = LAT["hip"]
    if L >= 500:
        for s, sg in (("l", 1), ("r", -1)):
            for v in ("front",):
                for sh in c.front:
                    if sh["id"] == f"thigh-{s}":
                        sh["regions"] = ["leg-skin", "leg-clothing"]; sh.pop("region")
                    if sh["id"] == f"upper-arm-{s}":
                        sh["regions"] = ["arm-skin", "arm-clothing"]; sh.pop("region")
        return
    d = dict(wscale=1.13)
    for s, sg in (("l", 1), ("r", -1)):
        th, ua = f"thigh-{s}", f"upper-arm-{s}"
        from human import limb2
        limb2(c, f"shorts-{s}", th, sg * hip, sg * hip, F.get("thighProf", DEFAULTS["front"]["thigh"]), t0=0, t1=0.47, wscale=1.13, cap0=0.045, z=1.04, region="leg-clothing")
        limb2(c, f"sleeve-{s}", ua, sg * LAT["shoulder"], sg * LAT["elbow"], DEFAULTS["front"]["upper"], t0=0, t1=0.45, wscale=1.12, cap0=0.05, z=4.52, region="arm-clothing")
    if mid_ or fine:
        limb2(c, "armband", "upper-arm-l", LAT["shoulder"], LAT["elbow"], DEFAULTS["front"]["upper"], t0=0.55, t1=0.8, wscale=1.1, z=4.53, region="arm-clothing")
        hc = T["head"][1] + CFG["headTopDy"] - CFG["skullRy"]
        c.rect("front", "headband", "head", -CFG["skullRx"] - 0.004, hc + 0.095 - 0.038, 2 * CFG["skullRx"] + 0.008, 0.038, r=0.01, z=6.25, region="hair", role="detail")
    if fine:
        for s, sg in (("l", 1), ("r", -1)):
            th, sh, ft = f"thigh-{s}", f"shank-{s}", f"foot-{s}"
            c.poly("front", f"shorts-hem-{s}", th, [(sg * hip - 0.1, -0.207), (sg * hip + 0.1, -0.207)], closed=False, follow=True, z=3.1, region="leg-clothing", role="detail")
            c.poly("front", f"shorts-stripe-{s}", th, [(sg * (hip + 0.078), 0.0), (sg * (hip + 0.062), -0.207)], closed=False, follow=True, z=3.1, region="leg-clothing", role="detail")
            c.poly("front", f"sock-top-{s}", sh, [(sg * hip - 0.048, -0.786), (sg * hip + 0.048, -0.786)], closed=False, follow=True, z=1.1, region="leg-skin", role="detail")
            for i, y in enumerate((-0.858, -0.868)):
                c.poly("front", f"lace-{s}-{i}", ft, [(sg * hip - 0.022, y), (sg * hip + 0.022, y)], closed=False, z=1.3, region="shoes", role="detail")
        c.poly("front", "shirt-collar", "torso", [(-0.07, 0.579), (0.0, 0.54), (0.07, 0.579)], closed=False, follow=True, z=4.1, region="torso-clothing", role="detail")
        c.poly("front", "shirt-hem", "torso", [(-0.16, 0.046), (0.16, 0.046)], closed=False, follow=True, z=4.1, region="torso-clothing", role="detail")
        c.poly("front", "shirt-stripe", "torso", [(-0.172, 0.414), (0.172, 0.414)], closed=False, follow=True, z=4.1, region="torso-clothing", role="texture")
        hx = (O["head"][0] + T["head"][0]) / 2 + CFG["headDx"]
        c.ell("plan", "headband", "head", (hx, 0), 0.094, 0.072, z=6.15, region="hair", role="detail", filled=False)
        for s, sg in (("l", 1), ("r", -1)):
            c.poly("plan", f"shoulder-seam-{s}", "upper-arm-l", [(0.045, sg * 0.07), (-0.005, sg * 0.16)], closed=False, z=4.2, region="torso-clothing", role="detail")
            c.poly("plan", f"shoe-lace-{s}", f"foot-{s}", [(0.03, sg * 0.09 - 0.012), (0.085, sg * 0.09 - 0.012)], closed=False, z=1.4, region="shoes", role="detail")


def fix_pitching(c, L):
    """QA4: feet as extruded shoe profiles in plan, head follows its bone tip in front and plan (hamstring fold, held quad stretch)."""
    import math
    O, T = c.O, c.T
    hx = (O["head"][0] + T["head"][0]) / 2 + CFG["headDx"]
    SOLE = {50: ([0.026, 0.037, 0.043, 0.040, 0.034], [0.040, 0.033, 0.022]), 100: ([0.026, 0.037, 0.043, 0.040, 0.034], [0.040, 0.033, 0.022]), 200: ([0.036, 0.043], [0.03])}
    for s_, sg in (("l", 1), ("r", -1)):
        c.drop("plan", [f"foot-{s_}", f"toe-{s_}"])
        zc = sg * CFG["lat"]["ankle"]
        if L >= 500:
            w = 0.053
            c.extrude_like("plan", f"foot-{s_}", f"foot-{s_}", f"shoe-{s_}", zc - w, zc + w, z=1.2, region="shoes")
            c.extrude_like("plan", f"toe-{s_}", f"toe-{s_}", f"toe-{s_}", zc - w, zc + w, z=1.2, region="shoes")
        else:
            c.slices("plan", f"foot-{s_}", f"foot-{s_}", c.bbox_local(c.elev(f"shoe-{s_}")), SOLE[L][0], zc, z=1.2, region="shoes")
            c.slices("plan", f"toe-{s_}", f"toe-{s_}", c.bbox_local(c.elev(f"shoe-toe-{s_}")), SOLE[L][1], zc, z=1.2, region="shoes")
    fx = hx + 0.085
    for sh in c.front:
        if sh["bone"] != "head" or sh["id"] not in ("eye-l", "eye-r", "brow-l", "brow-r", "nose", "mouth", "ear-l", "ear-r"):
            continue
        if sh["kind"] == "ellipse":
            xs, ys = (sh["cx"] - sh["rx"], sh["cx"] + sh["rx"]), (sh["cy"] - sh["ry"], sh["cy"] + sh["ry"])
        else:
            px, py = [q[0] for q in sh["points"]], [q[1] for q in sh["points"]]
            xs, ys = (min(px), max(px)), (min(py) - 0.003, max(py) + 0.003)
        fw = hx - 0.01 if sh["id"].startswith("ear") else fx
        corners = [c.L("head", fw + dx, y) for dx, y in ((-0.006, ys[0]), (0.006, ys[0]), (0.006, ys[1]), (-0.006, ys[1]))]
        for k in ("cx", "cy", "rx", "ry", "rotationDeg", "followTip"):
            sh.pop(k, None)
        sh.update({"kind": "poly", "points": [[round(q[0], 5), round(q[1], 5)] for q in corners], "closed": True, "solid": {"kind": "extruded", "z0": round(xs[0], 5), "z1": round(xs[1], 5)}})
    if L >= 500:
        reach = c.bones["forearm-l"]["lengthM"] + c.bones["hand-l"]["lengthM"]
        for s_, sg in (("l", 1), ("r", -1)):
            c.drop("front", [f"forearm-{s_}", f"hand-{s_}"])
            zc = sg * CFG["lat"]["wrist"]
            c.extrude_poly("front", f"forearm-{s_}", f"forearm-{s_}", [(0.0, -0.058), (reach, -0.058), (reach, 0.058), (0.0, 0.058)], zc - 0.058, zc + 0.058, z=4.55, regions=["arm-skin", "hand-skin"])
    c.drop("front", ["torso"])
    if L >= 500:
        c.extrude_poly("front", "torso", "torso", [(-0.07, 0.098), (0.1, 0.104), (0.3, 0.124), (0.36, 0.124), (0.398, 0.118), (0.445, 0.1), (0.4757, 0.06), (0.4757, -0.05), (0.431, -0.105), (0.398, -0.124), (0.36, -0.133), (0.276, -0.133), (0.154, -0.121), (0.0, -0.112), (-0.07, -0.105)], -0.17, 0.17, z=4.0, region="torso-clothing")
    else:
        c.extrude_like("front", "torso", "torso", "torso", -0.17, 0.17, z=4.0, region="torso-clothing")
    for sh in c.front:
        if sh["bone"] != "head" or sh["id"] not in ("head", "hair", "headband"):
            continue
        if sh["kind"] == "ellipse":
            n = 28
            sh["points"] = [[round(sh["cx"] + sh["rx"] * math.cos(2 * math.pi * i / n), 5), round(sh["cy"] + sh["ry"] * math.sin(2 * math.pi * i / n), 5)] for i in range(n)]
            for k in ("cx", "cy", "rx", "ry", "rotationDeg"):
                sh.pop(k, None)
        elif sh["kind"] == "rect":
            x, y, w_, h_ = sh["x"], sh["y"], sh["w"], sh["h"]
            sh["points"] = [[x, y], [x + w_, y], [x + w_, y + h_], [x, y + h_]]
            for k in ("x", "y", "w", "h", "r"):
                sh.pop(k, None)
        if sh["kind"] in ("ellipse", "rect"):
            sh["kind"] = "poly"
            sh["closed"] = True
        if sh["kind"] == "poly":
            sh["followTip"] = True
            if L == 200 and sh["id"] in ("head", "hair"):
                cy = sum(q[1] for q in sh["points"]) / len(sh["points"])
                sh["points"] = [[q[0], round(q[1] + (0.02 if q[1] > cy else 0.0), 5)] for q in sh["points"]]
    for sh in c.plan:
        if sh["bone"] == "head" and sh["id"] in ("head", "headband") and sh["kind"] == "ellipse":
            r = sh["ry"]
            for k in ("cx", "cy", "rx", "ry", "rotationDeg"):
                sh.pop(k, None)
            sh.update({"kind": "capsule", "x0": round(hx, 5), "y0": 0.0, "x1": round(hx - 0.03, 5), "y1": 0.0, "r0": r, "r1": r, "followTip": True})


def build(scales=None):
    c = Ctx("jogger")
    for L in (scales or c.scales):
        c.start(L)
        human(c, L, CFG)
        details(c, L)
        fix_pitching(c, L)
        en, de = texts(L, EXTRA)
        c.commit(L, en, de)
    c.save()
    return c


if __name__ == "__main__":
    build([int(x) for x in sys.argv[1:]] or None)
