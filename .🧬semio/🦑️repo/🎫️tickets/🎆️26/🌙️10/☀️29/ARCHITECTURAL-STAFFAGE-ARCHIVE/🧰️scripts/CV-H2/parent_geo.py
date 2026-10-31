"""parent-with-stroller: authored geometry per scale (default pose pushing-stroller)"""
from common import *
M = lambda pts: [(p[0]/100, p[1]/100) + tuple(p[2:]) for p in pts]
def WP(pts, closed=True): return D(cr(M(pts), closed))
def WL(pts): return "M " + " L ".join(f"{x/100:.4f} {y/100:.4f}" for x, y in pts)
RING = lambda ro, ri: kappa_circle(0, 0, ro) + " " + kappa_circle(0, 0, ri, -ri)

THIGH = [(0, .10, .105), (.1, .10, .105), (.25, .08, .085), (.38, .062, .062), (.4276, .058, .058)]
SHANK = [(0, .06, .06), (.12, .052, .07), (.25, .047, .052), (.36, .042, .044), (.4176, .04, .041)]
TORSO = [(-.14, .125, .125), (-.05, .12, .125), (0, .12, .125), (.1, .12, .12), (.2, .125, .115), (.3, .13, .12), (.38, .125, .115), (.43, .10, .10), (.457, .05, .06)]
UARM = [(0, .05, .052), (.15, .047, .048), (.3, .04, .042)]
FORE = [(0, .04, .042), (.14, .036, .038), (.27, .032, .032)]
HEAD = [(-3.2, 164.9), (-2, 170), (2.5, 173.5), (7.8, 173.4), (12.7, 172.5), (14.9, 169.7), (15.3, 164.7), (17.9, 160.9, 'c'), (15.6, 159.5), (15.2, 157.3), (14, 154.8), (9.7, 153.2), (11.4, 146, 'c'), (2, 146, 'c'), (2.5, 153.5), (-0.1, 156), (-3.4, 159)]
HAIR = [(15.1, 166.7), (15.5, 170.7), (11.4, 174.2), (6.9, 174), (0.3, 173.8), (-4.2, 170.1), (-3.9, 164.5), (-6.3, 157.8), (-8.3, 152, 'c'), (-5.3, 151.2), (-1.9, 155.5), (1.3, 160.9), (5.6, 168.5), (10.3, 168.9), (14.6, 168.5)]
SCARF = [(12.2, 146, 'c'), (13.2, 150), (12.2, 153), (9.3, 154), (1.5, 153.5), (-1.4, 151), (-0.5, 147), (0.5, 146, 'c')]
SHOE = [(10, 0, 'c'), (-8.9, 0, 'c'), (-9.5, 3.5), (-9.1, 7.5), (-8.1, 10.9), (0.6, 10.9), (2.7, 9.4), (6, 7.7), (10, 5.6, 'c')]
TOE = [(9.5, 5.6, 'c'), (12, 4.6), (16, 3), (18.8, 2.2), (18.8, 1), (17, 0, 'c'), (9.5, 0, 'c')]
FRAME = [("push-bar", [(53, 99), (72, 42)], .24), ("chassis", [(72, 42), (129, 38)], .26), ("rear-strut", [(74, 42), (78, 15.9)], .22), ("front-fork", [(127, 38), (130, 8)], .2), ("footrest", [(110, 38), (124, 33)], .16)]
SEAT = [(69, 42, 'c'), (67, 86, 'c'), (73, 88, 'c'), (76, 70), (80, 52), (114, 50, 'c'), (116, 44, 'c'), (80, 42, 'c')]
HOOD = [(68, 88, 'c'), (70, 103), (80, 110), (90, 112), (100, 106), (106, 100), (112, 95), (115, 88, 'c'), (114, 82, 'c'), (108, 82, 'c'), (106, 88), (98, 94), (88, 95), (78, 95), (71, 92)]


def build(c):
    reps = {}
    sh = c.add
    shoe_loc = {}
    def shoes(S):
        w = c.w
        if not shoe_loc:
            shoe_loc["foot"] = to_local(w["foot-l"], WP(SHOE)); shoe_loc["toe"] = to_local(w["toe-l"], WP(TOE))
        for s_, z in (("l", 2.2), ("r", 1.2)):
            sh(f"shoe-{s_}", f"foot-{s_}", "body", z, d=shoe_loc["foot"], tone=0.2, region="shoes")
            sh(f"shoe-toe-{s_}", f"toe-{s_}", "body", z, d=shoe_loc["toe"], tone=0.2, region="shoes")
    def legs(S):
        for s_, z in (("l", 2), ("r", 1)):
            far = s_ == "r"; hat = "far-limb" if (far and S == 100) else None
            sh(f"shank-{s_}", f"shank-{s_}", "body", z-0.03, d=tube(0.4176, SHANK if S != 500 else SHANK[:-2] + [(.36, .05, .05), (.4176, .05, .05)], front=1), tone=0.35, region="leg-clothing", hatch=hat)
            sh(f"thigh-{s_}", f"thigh-{s_}", "body", z, d=tube(0.4276, THIGH, front=1, cap1=0.03), tone=0.35, region="leg-clothing", hatch=hat)
            if S <= 100: sh(f"hem-{s_}", f"shank-{s_}", "detail", z+0.1, pts=[(0.37, -0.042), (0.37, 0.04)], filled=False, region="leg-clothing")
        shoes(S)
    def stroller(S):
        if S == 500:
            sh("frame", "stroller", "body", 4.4, world=WL([(50, 100), (53, 99), (72, 42), (129, 38), (130, 8)]), filled=False, region="stroller-frame")
            sh("rear-strut", "stroller", "body", 4.4, world=WL([(74, 42), (78, 15.9)]), filled=False, region="stroller-frame")
            sh("tyre-rear", "wheel-rear", "body", 4, d=kappa_circle(0, 0, 0.1592), filled=False, region="tyres")
            sh("tyre-front", "wheel-front", "body", 4, d=kappa_circle(0, 0, 0.0796), tone=0.1, region="tyres")
            sh("canopy", "stroller-hood", "body", 5.2, world=WP([(68, 88, 'c'), (70, 103), (80, 110), (90, 112), (100, 106), (106, 100), (112, 95), (115, 88, 'c'), (114, 82, 'c'), (104, 80), (97, 84), (90, 86), (78, 86)]), tone=0.55, region="stroller-fabric")
            return
        pens_ = {200: dict(bar=.13, strut=.13, fork=.13, foot=.13, handle=.18), 100: dict(bar=.24, strut=.22, fork=.2, foot=.16, handle=.34)}[S]
        for nm, pts, pen100 in FRAME:
            pen = pen100 if S == 100 else .13
            sh(nm, "stroller", "body", 4.4, world=WL(pts), filled=False, region="stroller-frame", pen=pen)
        sh("handle-grip", "stroller", "body", 6, world=WL([(45, 100), (55, 100)]), filled=False, region="stroller-frame", pen=pens_["handle"])
        sh("seat-fabric", "stroller", "body", 4.2, world=WP(SEAT), tone=0.5, region="stroller-fabric", hatch=("fabric" if S == 100 else None))
        sh("basket", "stroller", "body", 4.1, world=WP([(80, 30, 'c'), (118, 30, 'c'), (114, 20, 'c'), (84, 20, 'c')]), tone=0.4, region="stroller-fabric")
        for w_, bone, ro, ri in (("rear", "wheel-rear", 0.1592, 0.1192), ("front", "wheel-front", 0.0796, 0.0496)):
            if S == 200:
                sh(f"tyre-{w_}", bone, "body", 4, d=kappa_circle(0, 0, ro), filled=False, region="tyres", pen=(0.18 if w_ == "rear" else 0.13))
                if w_ == "rear": sh("spoke-rear", bone, "texture", 4.06, d="M -0.14 0 L 0.14 0", filled=False, region="tyres")
            else:
                sh(f"tyre-{w_}", bone, "body", 4, d=RING(ro, ri), tone=0.1, region="tyres")
                sh(f"hub-{w_}", bone, "body", 4.05, d=kappa_circle(0, 0, 0.02 if w_ == "rear" else 0.012), tone=0.3, region="tyres")
                if w_ == "rear": sh("spokes-rear", bone, "texture", 4.06, d=" ".join(f"M {-0.12*math.cos(math.radians(a)):.4f} {-0.12*math.sin(math.radians(a)):.4f} L {0.12*math.cos(math.radians(a)):.4f} {0.12*math.sin(math.radians(a)):.4f}" for a in (0, 45, 90, 135)), filled=False, region="tyres")
    def child(S):
        if S == 500:
            sh("child-mass", "child", "body", 4.6, world=WP([(75, 54, 'c'), (74, 84), (84, 88), (90, 76), (98, 64), (112, 62), (116, 54, 'c'), (112, 50, 'c'), (85, 50, 'c')]), tone=0.6, regions=["child-clothing", "child-skin"])
            return
        sh("child-body", "child", "body", 4.6, world=stadium(0.82, 0.58, 0.80, 0.80, 0.075, 0.065), tone=0.6, region="child-clothing")
        sh("child-legs", "child", "body", 4.7, world=WP([(82, 54, 'c'), (112, 56), (116, 56), (117, 50), (114, 46), (112, 42, 'c'), (108, 43, 'c'), (110, 49.5), (82, 46.5, 'c')]), tone=0.6, region="child-clothing")
        sh("child-head", "child", "body", 4.8, world=kappa_circle(0.81, 0.88, 0.075, 0.08), tone=0.88, region="child-skin")
        sh("child-hat", "child", "body", 4.9, world=WP([(73.5, 88, 'c'), (74, 96), (81, 99), (88, 97), (89.5, 89.5, 'c'), (88, 90), (81, 93.5), (75, 90.5)]), tone=0.3, region="child-clothing")
        sh("child-arm", "child", "body", 4.85, world=WL([(82, 74), (99, 62)]), filled=False, region="child-clothing", pen=(0.4 if S == 100 else 0.3))
        sh("hood", "stroller-hood", "body", 5.2, world=WP(HOOD), tone=0.45, region="stroller-fabric", hatch=("fabric" if S == 100 else None))
        if S == 100: sh("hood-rib", "stroller-hood", "detail", 5.25, world=WL([(69, 91), (80, 106), (96, 108), (108, 97)]), filled=False, region="stroller-fabric")
    # --------------------------------------------------------------------------------------------- 1:500
    c.shapes = []
    sh("head-hair", "head", "body", 5, world=WP(HEAD[:8] + [(15.6, 159.5), (14, 154.8), (11.4, 146, 'c'), (2, 146, 'c'), (-3.4, 159), (-8.3, 152, 'c'), (-5.3, 151.2), (-3.9, 164.5)]), tone=0.6, regions=["head-skin", "hair"])
    sh("torso-mass", "torso", "body", 4, d=tube(0.4574, TORSO, front=-1, cap1=0.01), tone=0.45, regions=["torso-clothing"])
    sh("hips", "pelvis", "body", 3.05, world=WP([(-10.7, 100.7, 'c'), (-13.2, 90), (-10.7, 81.3, 'c'), (9, 81.3, 'c'), (11.9, 92), (11.3, 100.7, 'c')]), tone=0.3, region="leg-clothing")
    sh("arm-r", "upper-arm-r", "body", 6, pts=[(0.0, 0.0), (0.3082, 0.0)], filled=False, region="arm-clothing")
    sh("forearm-hand-r", "forearm-r", "body", 6.1, pts=[(0.0, 0.0), (0.2685, 0.0), (0.38, 0.0)], filled=False, regions=["arm-clothing", "hand-skin"])
    legs(500); stroller(500); child(500)
    reps["500"] = list(c.shapes)
    # --------------------------------------------------------------------------------------------- 1:200
    c.shapes = []
    sh("head", "head", "body", 5, world=WP(HEAD), tone=0.8, region="head-skin")
    sh("hair", "head", "body", 5.4, world=WP(HAIR), tone=0.9, region="hair")
    sh("scarf", "head", "body", 4.8, world=WP(SCARF), tone=0.4, region="torso-clothing")
    sh("jacket", "torso", "body", 4, d=tube(0.4574, TORSO, front=-1, cap1=0.01), tone=0.45, region="torso-clothing")
    sh("jacket-zip", "torso", "detail", 4.1, pts=[(0.43, -0.1), (0.2, -0.123), (-0.1, -0.12)], filled=False, region="torso-clothing")
    sh("hips", "pelvis", "body", 3.05, world=WP([(-10.7, 100.7, 'c'), (-13.2, 90), (-10.7, 81.3, 'c'), (9, 81.3, 'c'), (11.9, 92), (11.3, 100.7, 'c')]), tone=0.3, region="leg-clothing")
    for s_, z in (("r", 6), ("l", 0.4)):
        sh(f"upper-arm-{s_}", f"upper-arm-{s_}", "body", z, d=tube(0.3082, UARM, front=1, cap1=0.035), tone=0.45, region="arm-clothing")
        sh(f"forearm-hand-{s_}", f"forearm-{s_}", "body", z+0.02, d=tube(0.2685, FORE + [(.3, .028, .03), (.34, .018, .02), (.37, .006, .008)], front=1, cap1=0.01), tone=0.55, regions=["arm-clothing", "hand-skin"])
    legs(200); stroller(200); child(200)
    reps["200"] = list(c.shapes)
    # --------------------------------------------------------------------------------------------- 1:100
    c.shapes = []
    sh("head", "head", "body", 5, world=WP(HEAD), tone=0.8, region="head-skin")
    sh("hair", "head", "body", 5.4, world=WP(HAIR), tone=0.9, region="hair")
    sh("ear", "head", "detail", 5.5, ell=(0.1571, 0.0195, 0.024, 0.012), tone=0.7, region="head-skin")
    sh("scarf", "head", "body", 4.8, world=WP(SCARF), tone=0.4, region="torso-clothing")
    sh("collar", "torso", "body", 4.5, world=WP([(12.3, 139.2, 'c'), (15.1, 144.8), (12.7, 149.2), (6.9, 150), (1.1, 150.8), (-3.3, 147.4), (-4.1, 141.5, 'c')]), tone=0.4, region="torso-clothing")
    sh("jacket", "torso", "body", 4, d=tube(0.4574, TORSO, front=-1, cap1=0.01), tone=0.45, region="torso-clothing", hatch="jacket-shell")
    sh("jacket-zip", "torso", "detail", 4.1, pts=[(0.43, -0.1), (0.2, -0.123), (-0.1, -0.12)], filled=False, region="torso-clothing")
    sh("jacket-hem", "torso", "detail", 4.1, pts=[(-0.139, -0.122), (-0.139, 0.12)], filled=False, region="torso-clothing")
    sh("hips", "pelvis", "body", 3.05, world=WP([(-10.7, 100.7, 'c'), (-13.2, 90), (-10.7, 81.3, 'c'), (9, 81.3, 'c'), (11.9, 92), (11.3, 100.7, 'c')]), tone=0.3, region="leg-clothing")
    for s_, z in (("r", 6), ("l", 0.4)):
        hat = "far-limb" if s_ == "l" else None
        sh(f"upper-arm-{s_}", f"upper-arm-{s_}", "body", z, d=tube(0.3082, UARM, front=1, cap1=0.035), tone=0.45, region="arm-clothing", hatch=hat or "jacket-shell")
        sh(f"forearm-{s_}", f"forearm-{s_}", "body", z+0.02, d=tube(0.2685, FORE, front=1, cap1=0.01), tone=0.45, region="arm-clothing", hatch=hat or "jacket-shell")
        sh(f"cuff-{s_}", f"forearm-{s_}", "detail", z+0.03, pts=[(0.243, -0.03), (0.243, 0.034)], filled=False, region="arm-clothing")
        sh(f"hand-{s_}", f"hand-{s_}", "body", z+0.2, d=D(cr([(-0.014, 0.028), (0.028, 0.032), (0.075, 0.029), (0.118, 0.021), (0.14, 0.006), (0.136, -0.013), (0.112, -0.025), (0.066, -0.03), (0.02, -0.028), (-0.014, -0.024)])), tone=0.8, region="hand-skin")
        sh(f"thumb-{s_}", f"hand-{s_}", "body", z+0.22, d=D(cr([(-0.004, 0.021), (0.028, 0.048), (0.06, 0.063), (0.092, 0.068), (0.094, 0.056), (0.07, 0.04), (0.033, 0.022)])), tone=0.8, region="hand-skin")
    legs(100); stroller(100); child(100)
    reps["100"] = list(c.shapes)
    return reps


if __name__ == "__main__":
    from sheet import sheet
    c = Ctx('parent-with-stroller'); reps = build(c); w = pose_world(c.a)
    pens = {"500": {"body": .13, "detail": None, "texture": None, "glazing": None, "ground": .13}, "200": {"body": .18, "detail": .13, "texture": .1, "glazing": None, "ground": .18}, "100": {"body": .25, "detail": .15, "texture": .1, "glazing": None, "ground": .25}}
    b = (-0.5, -0.02, 1.45, 1.85)
    items = [(reps[k], w, int(k), pens[k], b, "line") for k in ("500", "200", "100")]
    items.append((c.a['geometry']['elevation'], w, 50, {"body": .35, "detail": .25, "texture": .1, "glazing": .1, "ground": .35}, b, "line"))
    print(sheet("par-sheet", items, 700))
