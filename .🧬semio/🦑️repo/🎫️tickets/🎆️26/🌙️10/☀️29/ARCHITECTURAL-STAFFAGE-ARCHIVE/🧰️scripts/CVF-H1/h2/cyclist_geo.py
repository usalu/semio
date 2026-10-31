"""cyclist: authored geometry per scale"""
from common import *
M = lambda pts: [(p[0]/100, p[1]/100) + tuple(p[2:]) for p in pts]
def WP(pts, closed=True): return D(cr(M(pts), closed))
def WL(pts): return "M " + " L ".join(f"{x/100:.4f} {y/100:.4f}" for x, y in pts)
RING = lambda ro, ri: kappa_circle(0, 0, ro) + " " + kappa_circle(0, 0, ri, -ri)
HUB = lambda r: kappa_circle(0, 0, r)

THIGH = [(0, .088, .090), (.10, .095, .095), (.25, .082, .080), (.38, .064, .060), (.42, .060, .058)]
SHANK = [(0, .060, .060), (.12, .050, .072), (.28, .045, .055), (.40, .040, .042)]
TORSO = [(-.11, .126, .118), (0, .122, .112), (.12, .120, .104), (.20, .124, .106), (.27, .140, .114), (.36, .120, .112), (.43, .065, .085), (.449, 0, .03)]
SLEEVE = [(0, .058, .062), (.15, .054, .056), (.31, .047, .048)]
FORE = [(0, .045, .048), (.12, .042, .046), (.24, .038, .038)]


def rider(c, sh, scale):
    """shared rider construction; detail grows with the scale"""
    S = scale
    # --- head -----------------------------------------------------------------------------------------------
    if S == 500:
        sh("head-helmet", "head", "body", 5.5, world=WP([(42.9, 163), (41, 172), (46, 178.7), (55.8, 178.8), (63, 177), (65, 172), (63.5, 167.4), (62, 162), (63, 159.5), (60.4, 157.3), (59.8, 155.5), (58.4, 152.8), (53.5, 153.2), (47, 157), (44, 160)]), tone=0.5, regions=["face-skin", "helmet", "hair"])
    else:
        face = [(45.4, 159.8), (42.8, 164), (44, 169.5), (47.8, 173), (53, 174), (58, 172.8), (60.5, 169.5), (61, 166), (60.9, 164.6), (62, 161.5, 'c'), (63, 159.4), (61.6, 158.2), (60.4, 157.2), (59.8, 156), (58.6, 153.2), (53.2, 153.4), (46.5, 157.6)]
        sh("head", "head", "body", 5, world=WP(face), tone=0.8, region="face-skin")
        sh("hair-tail", "head", "body", 5.4, world=WP([(43.6, 164.9), (38, 162.5), (34.4, 154.5), (35.8, 147.5), (39.5, 150), (41.5, 157.4), (44.1, 161.4)]), tone=0.9, region="hair")
        sh("helmet", "head", "body", 5.5, world=WP([(42.9, 163, 'c'), (41, 172), (46, 178.8), (55.8, 179), (62.5, 177.6), (65.2, 172.6), (63.5, 167.4, 'c'), (59, 169.2), (50, 168.4)]), tone=0.3, region="helmet")
        if S <= 100:
            sh("helmet-vent", "head", "detail", 5.6, world=WL([(48.5, 174.9), (58.3, 175.5)]), filled=False, region="helmet")
        sh("neck", "neck", "body", 3.8, d=tube(0.0966, [(0, .052, .055), (.05, .046, .05), (.097, .04, .05)], front=-1, cap1=0.01), tone=0.8, region="face-skin")
        sh("collar", "neck", "body", 4.5, d=tube(0.0966, [(-.02, .09, .075), (.02, .062, .072), (.075, .064, .062)], front=-1, cap1=0.01), tone=0.4, region="jacket")
    # --- torso and pelvis ---------------------------------------------------------------------------------------
    if S == 500:
        sh("torso-mass", "torso", "body", 4, d=tube(0.449, TORSO, front=-1, cap1=0.01), tone=0.4, regions=["jacket", "face-skin"])
        sh("seat-trousers", "pelvis", "body", 3, world=WP([(16.3, 111.9), (13, 103), (13.8, 94.7, 'c'), (33.8, 91.2, 'c'), (38.5, 108)]), tone=0.3, regions=["trousers"])
    else:
        sh("jacket", "torso", "body", 4, d=tube(0.449, TORSO, front=-1, cap1=0.01), tone=0.4, region="jacket", hatch=("softshell" if S == 100 else None))
        sh("seat-trousers", "pelvis", "body", 3, world=WP([(16.3, 111.9), (13, 103), (13.8, 94.7, 'c'), (33.8, 91.2, 'c'), (38.5, 108)]), tone=0.3, region="trousers")
        if S <= 200: sh("jacket-reflective", "torso", "detail", 4.1, pts=[(0.16, -0.125), (0.16, 0.112)], filled=False, region="jacket")
        if S <= 100: sh("jacket-zip", "torso", "detail", 4.1, pts=[(0.43, -0.06), (0.16, -0.106), (-0.10, -0.123)], filled=False, region="jacket")
    # --- arms ------------------------------------------------------------------------------------------------
    for s_, a in (("r", 6), ("l", 0.5)):
        if S == 500 and s_ == "l": continue
        far = (s_ == "l")
        if S == 500:
            if far: continue
            sh("arm-r", "arm-r", "body", 6, pts=[(0.0, 0.0), (0.319, 0.0)], filled=False, regions=["jacket"])
            sh("forearm-hand-r", "forearm-r", "body", 6.1, pts=[(0.0, 0.0), (0.251, 0.0), (0.36, 0.0)], filled=False, regions=["jacket", "gloves"])
            continue
        hat = "far-limb" if far and S == 100 else None
        sh(f"sleeve-{s_}", f"arm-{s_}", "body", a, d=tube(0.319, SLEEVE, front=1, cap0=0.03, cap1=0.035), tone=0.4, region="jacket", hatch=hat)
        if S == 200:
            sh(f"forearm-hand-{s_}", f"forearm-{s_}", "body", a+0.02, d=tube(0.251, FORE + [(.28, .034, .034), (.33, .022, .024), (.36, .01, .012)], front=1, cap0=0.035, cap1=0.01), tone=0.5, regions=["jacket", "gloves"])
        else:
            sh(f"forearm-{s_}", f"forearm-{s_}", "body", a+0.02, d=tube(0.251, FORE, front=1, cap0=0.035, cap1=0.01), tone=0.4, region="jacket", hatch=hat)
            sh(f"glove-{s_}", f"hand-{s_}", "body", a+0.2, d=D(cr([(-0.014, 0.032), (0.03, 0.036), (0.08, 0.032), (0.125, 0.024), (0.15, 0.008), (0.146, -0.012), (0.12, -0.026), (0.07, -0.032), (0.02, -0.03), (-0.014, -0.026)])), tone=0.25, region="gloves")
            sh(f"glove-thumb-{s_}", f"hand-{s_}", "body", a+0.22, d=D(cr([(-0.004, 0.022), (0.03, 0.05), (0.065, 0.068), (0.098, 0.074), (0.1, 0.06), (0.075, 0.042), (0.035, 0.024)])), tone=0.25, region="gloves")
    # --- legs ------------------------------------------------------------------------------------------------
    for s_, z in (("r", 4), ("l", 1)):
        far = (s_ == "l"); hat = "far-limb" if far and S == 100 else None
        sh(f"shank-{s_}", f"shank-{s_}", "body", z-0.03, d=tube(0.4106, SHANK if S != 500 else SHANK[:-2] + [(.36, .05, .05), (.41, .05, .05)], front=1), tone=0.3, region="trousers", hatch=hat)
        sh(f"thigh-{s_}", f"thigh-{s_}", "body", z, d=tube(0.4202, THIGH, front=1, cap1=0.03), tone=0.3, region="trousers", hatch=hat)
        if S <= 100: sh(f"trouser-clip-{s_}", f"shank-{s_}", "detail", z+0.01, pts=[(0.33, -0.046), (0.33, 0.044)], filled=False, region="trousers")
        if S == 500: continue
        # shoes: sneaker, world authored for the near foot (foot bone follows the pedal, so local shape reused for the far foot)
    return


def shoes(c, sh, scale):
    for s_, z in (("r", 4.1), ("l", 1.1)):
        # local to foot bone: x from ankle toward the ball, y up; authored once in the near foot's default world pose
        fw = c.w[f"foot-{s_}"]; tw = c.w[f"toe-{s_}"]
        if s_ == "r":
            rear = [(34.1, 27.5), (33.1, 24.5), (33.8, 21.2, 'c'), (53.6, 16.5, 'c'), (55.1, 22.8), (50.6, 25.3), (44.6, 29.0), (42, 30.2), (37, 30.2), (35, 29.6)]
            toe = [(53.4, 16.5, 'c'), (57.7, 15.5), (60.6, 16.2), (60.6, 17.8), (57.7, 21.4), (54.9, 22.9, 'c')]
            rear_l = to_local(c.w["foot-r"], WP(rear)); toe_l = to_local(c.w["toe-r"], WP(toe))
            sh_cache.update(rear=rear_l, toe=toe_l)
        sh(f"shoe-{s_}", f"foot-{s_}", "body", z, d=sh_cache["rear"], tone=0.1, region="shoes")
        sh(f"shoe-toe-{s_}", f"toe-{s_}", "body", z, d=sh_cache["toe"], tone=0.1, region="shoes")
        if scale <= 100: sh(f"sole-{s_}", f"foot-{s_}", "detail", z+0.05, pts=[(-0.02, -0.07), (0.17, 0.0)], filled=False, region="shoes")
sh_cache = {}


def build(c):
    reps = {}
    def sh(*a, **k): return c.add(*a, **k)
    # ============================================================ 1:500
    c.shapes = []
    for w_, bone in (("rear", "rear-wheel"), ("front", "front-wheel")):
        sh(f"tyre-{w_}", bone, "body", 1.4, d=kappa_circle(0, 0, 0.34), filled=False, region="tyres")
    sh("main-triangle", "frame", "body", 1.8, world=WL([(45, 30), (27.5, 87.4), (95.5, 87.5), (99, 76), (45, 30)]), filled=False, region="bike-metal")
    sh("rear-triangle", "frame", "body", 1.8, world=WL([(0, 34), (45, 30), (27.5, 87.4), (0, 34)]), filled=False, region="bike-metal")
    sh("fork-bars", "frame", "body", 1.85, world=WL([(105, 34), (99, 76), (95.5, 87.5), (91.5, 100), (74, 105)]), filled=False, region="bike-metal")
    sh("saddle", "frame", "body", 1.9, world=WL([(13, 92.5), (39, 91.5)]), filled=False, region="saddle")
    rider(c, sh, 500); shoes(c, sh, 500)
    reps["500"] = list(c.shapes)
    # ============================================================ 1:200
    c.shapes = []
    for w_, bone in (("rear", "rear-wheel"), ("front", "front-wheel")):
        sh(f"tyre-{w_}", bone, "body", 1.4, d=kappa_circle(0, 0, 0.34), filled=False, region="tyres", pen=0.18)
        sh(f"hub-{w_}", bone, "body", 1.6, d=HUB(0.02), tone=0.2, region="bike-metal")
        sh(f"spokes-{w_}", bone, "texture", 1.5, d="M -0.32 0 L 0.32 0 M 0 -0.32 L 0 0.32 M -0.226 -0.226 L 0.226 0.226 M -0.226 0.226 L 0.226 -0.226", filled=False, region="bike-metal")
    for nm, pts, pen in (("seat-tube", [(45, 30), (27.5, 87.4)], 0.18), ("top-tube", [(27.5, 87.4), (95.5, 86)], 0.18), ("down-tube", [(45, 30), (99, 76)], 0.18), ("head-tube", [(99, 76), (95.5, 87.5)], 0.18),
                         ("chainstay", [(45, 30), (0, 34)], 0.13), ("seatstay", [(27.5, 87.4), (0, 34)], 0.13), ("fork", [(99, 76), (105, 34)], 0.13), ("stem-bars", [(95.5, 87.5), (91.5, 100), (74, 105)], 0.13), ("seatpost", [(27.5, 87.4), (25.5, 92.5)], 0.13)):
        sh(nm, "frame", "body", 1.8, world=WL(pts), filled=False, region="bike-metal", pen=pen)
    sh("saddle", "frame", "body", 1.9, world=WL([(13, 92.5), (39, 91.5)]), filled=False, region="saddle", pen=0.18)
    sh("chainring", "crank-r", "body", 2.2, d=kappa_circle(0, 0, 0.081), filled=False, region="bike-metal", pen=0.13)
    for s_, z in (("r", 2.3), ("l", 0.9)):
        sh(f"crank-{s_}", f"crank-{s_}", "body", z, pts=[(0, 0), (0.17, 0)], filled=False, region="bike-metal", pen=0.13)
        sh(f"pedal-{s_}", f"pedal-{s_}", "body", z+0.05, pts=[(-0.052, 0), (0.052, 0)], filled=False, region="bike-metal", pen=0.13)
    rider(c, sh, 200); shoes(c, sh, 200)
    reps["200"] = list(c.shapes)
    # ============================================================ 1:100
    c.shapes = []
    for w_, bone in (("rear", "rear-wheel"), ("front", "front-wheel")):
        sh(f"tyre-{w_}", bone, "body", 1.4, d=RING(0.34, 0.30), tone=0.1, region="tyres")
        sh(f"hub-{w_}", bone, "body", 1.6, d=HUB(0.025), tone=0.2, region="bike-metal")
        sh(f"spokes-{w_}", bone, "texture", 1.5, d=" ".join(f"M {-0.30*math.cos(math.radians(a)):.4f} {-0.30*math.sin(math.radians(a)):.4f} L {0.30*math.cos(math.radians(a)):.4f} {0.30*math.sin(math.radians(a)):.4f}" for a in range(0, 180, 30)), filled=False, region="bike-metal")
    def arc(cx, cy, r, a0, a1, n=14):
        return [(cx + r*math.cos(math.radians(a0+(a1-a0)*i/n)), cy + r*math.sin(math.radians(a0+(a1-a0)*i/n))) for i in range(n+1)]
    sh("fender-rear", "frame", "body", 1.7, d=D(cr(arc(0, 0.34, 0.385, 28, 150), closed=False)), filled=False, region="bike-metal", pen=0.15)
    sh("fender-front", "frame", "body", 1.7, d=D(cr(arc(1.05, 0.34, 0.385, 30, 152), closed=False)), filled=False, region="bike-metal", pen=0.15)
    for nm, pts, pen in (("seat-tube", [(45, 30), (27.5, 87.4)], 0.34), ("top-tube", [(27.5, 87.4), (95.5, 86)], 0.32), ("down-tube", [(45, 30), (99, 76)], 0.36), ("head-tube", [(99, 76), (95.5, 87.5)], 0.44),
                         ("chainstay", [(45, 30), (0, 34)], 0.22), ("seatstay", [(27.5, 87.4), (0, 34)], 0.2), ("fork", [(99, 76), (105, 34)], 0.24), ("stem", [(95.5, 87.5), (91.5, 100)], 0.28), ("seatpost", [(27.5, 87.4), (25.5, 92.5)], 0.22)):
        sh(nm, "frame", "body", 1.8, world=WL(pts), filled=False, region="bike-metal", pen=pen)
    sh("handlebar", "frame", "body", 1.95, d=D(cr(M([(91.5, 100), (88, 105), (82, 107.5), (74, 106)]), closed=False)), filled=False, region="bike-metal", pen=0.22)
    sh("grip", "frame", "body", 1.96, world=WL([(80, 106.5), (70, 105.2)]), filled=False, region="bike-metal", pen=0.3)
    sh("saddle", "frame", "body", 1.9, world=WP([(13, 92.5), (14, 95), (20, 95.5), (27, 94.5), (32, 94), (37, 93.5), (39, 91.5, 'c'), (30, 90.5), (20, 90.5), (15, 90.5)]), tone=0.2, region="saddle")
    sh("headlamp", "frame", "body", 1.95, world=WP([(99, 93, 'c'), (103, 93, 'c'), (103, 89, 'c'), (99, 89, 'c')]), tone=0.5, region="bike-metal")
    sh("chainring", "crank-r", "body", 2.2, d=kappa_circle(0, 0, 0.081), filled=False, region="bike-metal", pen=0.1)
    sh("chain-top", "frame", "detail", 1.7, world=WL([(45, 38.1), (0, 38.1)]), filled=False, region="bike-metal", pen=0.1)
    sh("chain-bottom", "frame", "detail", 1.7, world=WL([(45, 21.9), (0, 29.9)]), filled=False, region="bike-metal", pen=0.1)
    for s_, z in (("r", 2.3), ("l", 0.9)):
        sh(f"crank-{s_}", f"crank-{s_}", "body", z, pts=[(0, 0), (0.17, 0)], filled=False, region="bike-metal", pen=0.28)
        sh(f"pedal-{s_}", f"pedal-{s_}", "body", z+0.05, pts=[(-0.052, 0), (0.052, 0)], filled=False, region="bike-metal", pen=0.16)
    rider(c, sh, 100); shoes(c, sh, 100)
    reps["100"] = list(c.shapes)
    return reps


if __name__ == "__main__":
    from sheet import sheet
    c = Ctx('cyclist'); reps = build(c); w = pose_world(c.a)
    pens = {"500": {"body": .13, "detail": None, "texture": None, "glazing": None, "ground": .13}, "200": {"body": .18, "detail": .13, "texture": .1, "glazing": None, "ground": .18}, "100": {"body": .25, "detail": .15, "texture": .1, "glazing": None, "ground": .25}}
    items = [(reps[k], w, int(k), pens[k], (-0.45, -0.02, 1.5, 1.85), "line") for k in ("500", "200", "100")]
    items.append(([s for s in c.a['geometry']['elevation']], w, 50, {"body": .35, "detail": .25, "texture": .1, "glazing": .1, "ground": .35}, (-0.45, -0.02, 1.5, 1.85), "line"))
    print(sheet("cyc-sheet", items, 700))
