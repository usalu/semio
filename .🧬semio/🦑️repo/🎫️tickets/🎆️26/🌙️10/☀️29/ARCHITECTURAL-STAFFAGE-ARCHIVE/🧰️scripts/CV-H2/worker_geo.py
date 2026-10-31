"""worker-carrying-box: authored geometry per scale (default pose carrying-box-low)"""
from common import *
M = lambda pts: [(p[0]/100, p[1]/100) + tuple(p[2:]) for p in pts]
def WP(pts, closed=True): return D(cr(M(pts), closed))
def WL(pts): return "M " + " L ".join(f"{x/100:.4f} {y/100:.4f}" for x, y in pts)

THIGH = [(0, .105, .115), (.10, .11, .115), (.25, .085, .095), (.38, .066, .068), (.435, .062, .062)]
SHANK = [(0, .065, .066), (.12, .058, .075), (.25, .05, .055), (.36, .045, .048), (.425, .043, .045)]
TORSO = [(-.06, .125, .12), (0, .125, .12), (.1, .125, .125), (.2, .128, .14), (.3, .12, .148), (.38, .10, .13), (.43, .07, .10), (.465, .03, .05)]
UARM = [(0, .052, .056), (.15, .047, .05), (.31, .042, .044)]
FORE = [(0, .042, .045), (.14, .038, .04), (.27, .034, .034)]
HEAD = [(-15.4, 168.5), (-14, 174), (-9, 178), (-3.3, 178.2), (2, 177), (4.4, 173.5), (4.9, 169.6), (7.7, 165.6, 'c'), (5.6, 164.2), (5.2, 162.6), (4.5, 161.2), (3.8, 158.7), (-0.5, 157.8), (1.0, 153), (0.8, 150.4, 'c'), (-9.6, 150.4, 'c'), (-9.2, 158), (-12.5, 161)]
HELM = [(-16.8, 167.2, 'c'), (-17.6, 176.2), (-11.4, 182.2), (-3.2, 182.4), (3.8, 182.5), (7.2, 178.3), (7.7, 173.9), (13.2, 173.3, 'c'), (13.2, 171.5, 'c'), (7.9, 170.9), (5.8, 169.4), (-16.5, 166.4, 'c')]
BOOT = [(13.2, 0.6, 'c'), (-6.3, 0.0, 'c'), (-6.7, 8.1), (-6.4, 14), (-5.3, 20.7, 'c'), (5.3, 20.7, 'c'), (5.5, 12.6), (7.3, 10.9), (10.2, 8.9), (13.2, 7.0, 'c')]
TOE = [(12.5, 7.0, 'c'), (17.2, 4.7), (22.4, 2.8), (23.1, 1.0), (21.4, 0.0, 'c'), (12.5, 0.0, 'c')]
HEADFRONT = None


def build(c):
    reps = {}
    sh = c.add
    def legs(S, regs_leg, regs_shoe=None):
        for s_, z in (("r", 2), ("l", 1)):
            far = s_ == "l"; hat = "far-limb" if (far and S == 100) else None
            sh(f"shank-{s_}", f"shank-{s_}", "body", z-0.03, d=tube(0.4248, SHANK if S != 500 else SHANK[:-2] + [(.36, .05, .05), (.425, .05, .05)], front=1), tone=0.3, region="leg-clothing", hatch=hat)
            sh(f"thigh-{s_}", f"thigh-{s_}", "body", z, d=tube(0.4349, THIGH, front=1, cap1=0.03), tone=0.3, region="leg-clothing", hatch=hat)
            sh(f"boot-{s_}", f"foot-{s_}", "body", z+0.2, world=WP(BOOT), tone=0.08, region="shoes")
            sh(f"boot-toe-{s_}", f"toe-{s_}", "body", z+0.2, world=WP(TOE), tone=0.08, region="shoes")
            if S <= 100:
                sh(f"hem-{s_}", f"shank-{s_}", "detail", z+0.1, pts=[(0.3, -0.047), (0.3, 0.043)], filled=False, region="leg-clothing")
                sh(f"sole-{s_}", f"foot-{s_}", "detail", z+0.25, world=WL([(-6.4, 3), (13, 3)]), filled=False, region="shoes")
    def box(S):
        sh("box", "box-r", "body", 5.8, ell=None, d=D(cr([(-0.2, -0.15, 'c'), (0.2, -0.15, 'c'), (0.2, 0.15, 'c'), (-0.2, 0.15, 'c')])), tone=0.66, region="box-surface")
        if S <= 200: sh("box-tape", "box-r", "detail", 5.82, pts=[(-0.2, 0.095), (0.2, 0.095)], filled=False, region="box-surface")
        if S <= 100: sh("box-handle", "box-r", "detail", 5.82, d=D(cr([(-0.075, -0.015, 'c'), (0.075, -0.015, 'c'), (0.075, 0.03, 'c'), (-0.075, 0.03, 'c')])), filled=False, region="box-surface")
    # ------------------------------------------------------------------------------------------- 1:500
    c.shapes = []
    sh("head-helmet", "head", "body", 5.5, world=WP([(-16.8, 167.2, 'c'), (-17.6, 176.2), (-11.4, 182.2), (-3.2, 182.4), (3.8, 182.5), (7.2, 178.3), (7.7, 170), (7.7, 165.6, 'c'), (4.5, 161.2), (3.8, 158.7), (0.8, 150.4, 'c'), (-9.6, 150.4, 'c'), (-9.2, 158), (-14, 161)]), tone=0.5, regions=["head-skin", "hair", "helmet"])
    sh("torso-mass", "torso", "body", 4, d=tube(0.4652, TORSO, front=-1, cap1=0.01), tone=0.5, regions=["torso-clothing", "vest"])
    sh("hips", "pelvis", "body", 3, world=WP([(-11.7, 104.2, 'c'), (-14.5, 90), (-11.7, 84.5, 'c'), (9.8, 84.5, 'c'), (12.5, 90), (12.3, 104.2, 'c')]), tone=0.3, region="leg-clothing")
    sh("arm-r", "upper-arm-r", "body", 6, pts=[(0.0, 0.0), (0.3135, 0.0)], filled=False, region="arm-clothing")
    sh("forearm-hand-r", "forearm-r", "body", 6.1, pts=[(0.0, 0.0), (0.2731, 0.0), (0.4, 0.0)], filled=False, regions=["arm-clothing", "gloves"])
    legs(500, None); box(500)
    reps["500"] = list(c.shapes)
    # ------------------------------------------------------------------------------------------- 1:200
    c.shapes = []
    sh("head", "head", "body", 5, world=WP(HEAD), tone=0.8, regions=["head-skin", "hair"])
    sh("helmet", "head", "body", 5.6, world=WP(HELM), tone=0.5, region="helmet")
    sh("torso", "torso", "body", 3.2, d=tube(0.4652, TORSO, front=-1, cap1=0.01), tone=0.5, region="torso-clothing")
    sh("vest", "torso", "body", 3.4, d=tube(0.4652, [(-.07, .13, .125), (0, .13, .125), (.1, .13, .13), (.2, .133, .145), (.3, .125, .152), (.37, .105, .135)], front=-1), tone=0.62, region="vest")
    sh("vest-band-low", "torso", "detail", 3.5, pts=[(0.09, -0.135), (0.09, 0.13)], filled=False, region="vest")
    sh("vest-band-high", "torso", "detail", 3.5, pts=[(0.3, -0.125), (0.3, 0.15)], filled=False, region="vest")
    sh("hips", "pelvis", "body", 3.05, world=WP([(-11.7, 104.2, 'c'), (-14.5, 90), (-11.7, 84.5, 'c'), (9.8, 84.5, 'c'), (12.5, 90), (12.3, 104.2, 'c')]), tone=0.3, region="leg-clothing")
    for s_, z in (("r", 6), ("l", 0.4)):
        sh(f"upper-arm-{s_}", f"upper-arm-{s_}", "body", z, d=tube(0.3135, UARM, front=1, cap1=0.035), tone=0.5, region="arm-clothing")
        sh(f"forearm-hand-{s_}", f"forearm-{s_}", "body", z+0.02, d=tube(0.2731, FORE + [(.31, .03, .03), (.35, .02, .022), (.38, .008, .01)], front=1, cap0=0.035, cap1=0.01), tone=0.55, regions=["arm-clothing", "gloves"])
    legs(200, None); box(200)
    reps["200"] = list(c.shapes)
    # ------------------------------------------------------------------------------------------- 1:100
    c.shapes = []
    sh("head", "head", "body", 5, world=WP(HEAD), tone=0.8, region="head-skin")
    sh("hair", "head", "body", 5.4, world=WP([(-9.2, 158, 'c'), (-12.5, 161), (-15.4, 168.5), (-14.5, 172), (-10.2, 165.6), (-8.5, 159.5)]), tone=0.9, region="hair")
    sh("helmet", "head", "body", 5.6, world=WP(HELM), tone=0.5, region="helmet")
    sh("helmet-ridge", "head", "detail", 5.7, world=WL([(-6, 181.5), (1.5, 179), (7.4, 173.6)]), filled=False, region="helmet")
    sh("ear", "head", "detail", 5.3, ell=(0.1598, 0.0212, 0.024, 0.0127), tone=0.7, region="head-skin")
    sh("torso", "torso", "body", 3.2, d=tube(0.4652, TORSO, front=-1, cap1=0.01), tone=0.5, region="torso-clothing")
    sh("vest", "torso", "body", 3.4, d=tube(0.4652, [(-.07, .13, .125), (0, .13, .125), (.1, .13, .13), (.2, .133, .145), (.3, .125, .152), (.37, .105, .135)], front=-1), tone=0.62, region="vest")
    sh("vest-band-low", "torso", "detail", 3.5, pts=[(0.07, -0.131), (0.11, -0.131), (0.11, 0.126), (0.07, 0.126)], closed=True, tone=0.9, region="vest")
    sh("vest-band-high", "torso", "detail", 3.5, pts=[(0.28, -0.126), (0.32, -0.126), (0.32, 0.152), (0.28, 0.152)], closed=True, tone=0.9, region="vest")
    sh("vest-strap", "torso", "detail", 3.55, world=WL([(-16, 147), (0, 143), (10, 105)]), filled=False, region="vest")
    sh("hips", "pelvis", "body", 3.05, world=WP([(-11.7, 104.2, 'c'), (-14.5, 90), (-11.7, 84.5, 'c'), (9.8, 84.5, 'c'), (12.5, 90), (12.3, 104.2, 'c')]), tone=0.3, region="leg-clothing")
    for s_, z in (("r", 6), ("l", 0.4)):
        hat = "far-limb" if s_ == "l" else None
        sh(f"upper-arm-{s_}", f"upper-arm-{s_}", "body", z, d=tube(0.3135, UARM, front=1, cap1=0.035), tone=0.5, region="arm-clothing", hatch=hat)
        sh(f"forearm-{s_}", f"forearm-{s_}", "body", z+0.02, d=tube(0.2731, FORE, front=1, cap0=0.035, cap1=0.01), tone=0.5, region="arm-clothing", hatch=hat)
        sh(f"cuff-{s_}", f"forearm-{s_}", "detail", z+0.03, pts=[(0.255, -0.033), (0.255, 0.036)], filled=False, region="arm-clothing")
        sh(f"glove-{s_}", f"hand-{s_}", "body", z+0.2, d=D(cr([(-0.014, 0.03), (0.03, 0.034), (0.08, 0.03), (0.125, 0.022), (0.15, 0.006), (0.146, -0.014), (0.12, -0.026), (0.07, -0.031), (0.02, -0.029), (-0.014, -0.025)])), tone=0.3, region="gloves")
        sh(f"glove-thumb-{s_}", f"hand-{s_}", "body", z+0.22, d=D(cr([(-0.004, 0.022), (0.03, 0.05), (0.065, 0.066), (0.098, 0.07), (0.1, 0.057), (0.075, 0.04), (0.035, 0.023)])), tone=0.3, region="gloves")
    legs(100, None); box(100)
    reps["100"] = list(c.shapes)
    return reps


if __name__ == "__main__":
    from sheet import sheet
    c = Ctx('worker-carrying-box'); reps = build(c); w = pose_world(c.a)
    pens = {"500": {"body": .13, "detail": None, "texture": None, "glazing": None, "ground": .13}, "200": {"body": .18, "detail": .13, "texture": None, "glazing": None, "ground": .18}, "100": {"body": .25, "detail": .15, "texture": .1, "glazing": None, "ground": .25}}
    b = (-0.3, -0.02, 0.75, 1.9)
    items = [(reps[k], w, int(k), pens[k], b, "line") for k in ("500", "200", "100")]
    items.append((c.a['geometry']['elevation'], w, 50, {"body": .35, "detail": .25, "texture": .1, "glazing": .1, "ground": .35}, b, "line"))
    print(sheet("wrk-sheet", items, 900))
