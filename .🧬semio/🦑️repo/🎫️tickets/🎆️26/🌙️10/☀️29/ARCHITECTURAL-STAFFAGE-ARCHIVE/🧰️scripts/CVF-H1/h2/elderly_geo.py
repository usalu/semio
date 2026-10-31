"""elderly-with-cane: authored geometry per scale (world cm points of the default pose 'stooped-standing' or bone-local width tables)."""
from common import *

M = lambda pts: [(p[0]/100, p[1]/100) + tuple(p[2:]) for p in pts]
def WP(pts, closed=True): return D(cr(M(pts), closed))
def WL(pts): return "M " + " L ".join(f"{x/100:.4f} {y/100:.4f}" for x, y in pts)


def build(c):
    reps = {}
    sh = lambda *a, **k: c.add(*a, **k)

    # ---------------------------------------------------------------- 1:500
    c.shapes = []
    ALL = ["face-skin", "hair", "cap", "hand-skin", "jacket", "trousers", "shoes", "cane-wood"]
    sh("head-cap", "head", "body", 5, world=WP([(10.5, 150), (12.5, 158.5), (17.5, 162.5), (24, 162), (30.5, 154.3, 'c'), (27, 152, 'c'), (27.6, 147.5), (25.2, 144), (23.2, 141.2), (17, 140.6), (11, 146)]), tone=0.55, regions=["face-skin", "hair", "cap"])
    sh("torso-mass", "torso", "body", 4, world=WP([(11.8, 139.5), (3, 137), (-3, 129), (-4.5, 116), (-9, 100), (-13.5, 83.5, 'c'), (9.5, 77, 'c'), (15.5, 98), (21.5, 114), (23, 125), (19.5, 134.5)]), tone=0.5, regions=["jacket"])
    sh("hips", "pelvis", "body", 3, world=WP([(-9.5, 100), (-13, 83.5), (-11.8, 80, 'c'), (9.5, 76, 'c'), (13.4, 90)]), tone=0.35, regions=["trousers"])
    sh("arm-r", "arm-r", "body", 6, pts=[(0.0, 0.0), (0.307, 0.0)], filled=False, regions=["jacket"])
    sh("forearm-hand-r", "forearm-r", "body", 6.1, pts=[(0.0, 0.0), (0.242, 0.0), (0.34, 0.0)], filled=False, regions=["jacket", "hand-skin"])
    for s_, z in (("r", 3), ("l", 1)):
        sh(f"thigh-{s_}", f"thigh-{s_}", "body", z, d=tube(0.4046, [(0.0, 0.10, 0.095), (0.1, 0.098, 0.095), (0.25, 0.08, 0.078), (0.405, 0.058, 0.056)], front=1, cap1=0.02), tone=0.35, regions=["trousers"])
        sh(f"shank-{s_}", f"shank-{s_}", "body", z-0.02, d=tube(0.3953, [(0.0, 0.058, 0.058), (0.12, 0.05, 0.064), (0.3, 0.048, 0.05), (0.37, 0.05, 0.05)], front=1), tone=0.35, regions=["trousers"])
        sh(f"shoe-{s_}", f"foot-{s_}", "body", z+0.1, world=WP([(-4.7, 0.4, 'c'), (-4.8, 5), (-4, 8.8), (0.5, 10), (6, 9.5), (15.8, 5.2), (15.8, 0, 'c')]), tone=0.1, regions=["shoes"])
        sh(f"shoe-toe-{s_}", f"toe-{s_}", "body", z+0.1, world=WP([(15, 0, 'c'), (20.5, 0, 'c'), (22.4, 1.2), (22.2, 2.6), (20, 3.8), (15, 4.6, 'c')]), tone=0.1, regions=["shoes"])
    sh("cane", "cane", "body", 6.3, pts=[(0.0, 0.0), (0.86, 0.0)], filled=False, region="cane-wood", pen=0.13)
    reps["500"] = dict(shapes=list(c.shapes))

    # ---------------------------------------------------------------- 1:200
    c.shapes = []
    sh("head", "head", "body", 5, world=WP([(10.7, 149), (11.5, 156), (15, 160.5), (21, 161.4), (25.8, 156.7), (26.4, 152.5), (27.6, 147.4), (25.3, 145.1), (24.8, 143.4), (22.6, 140.6), (17.6, 141.3), (12.5, 145.8)]), tone=0.8, region="face-skin")
    sh("hair", "head", "body", 5.4, world=WP([(9.7, 148.1, 'c'), (8, 154.6), (10, 159), (15, 160.8), (19.2, 159, 'c'), (15.5, 157.5), (12, 154), (11, 150)]), tone=0.9, region="hair")
    sh("cap", "head", "body", 5.6, world=WP([(12.2, 159.8), (13, 163), (19.4, 163.9), (24, 161), (30.4, 153.6, 'c'), (28, 153.2, 'c'), (24, 154.9), (17, 157.2), (12.4, 158.6)]), tone=0.4, region="cap")
    sh("collar", "neck", "body", 4.5, d=tube(0.093, [(-0.015, 0.075, 0.07), (0.02, 0.06, 0.07), (0.075, 0.065, 0.06)], front=-1, cap1=0.01), tone=0.4, region="jacket")
    sh("jacket", "torso", "body", 4, d=tube(0.4325, [(-0.16, 0.146, 0.091), (-0.02, 0.150, 0.088), (0.10, 0.148, 0.088), (0.19, 0.152, 0.098), (0.26, 0.156, 0.112), (0.33, 0.14, 0.118), (0.40, 0.10, 0.105)], front=-1, cap1=0.03), tone=0.4, region="jacket")
    sh("jacket-front", "torso", "detail", 4.1, world=WL([(19.6, 134.3), (17.4, 117.3), (9.3, 79)]), filled=False, region="jacket")
    sh("trousers-seat", "pelvis", "body", 3.05, world=WP([(-7.6, 99.3), (-11, 90), (-11.8, 83.1, 'c'), (7, 77.7, 'c'), (11.5, 83.2), (13.2, 93.3)]), tone=0.3, region="trousers")
    for s_, z, a in (("r", 3, 6), ("l", 1, 0.5)):
        sh(f"sleeve-{s_}", f"arm-{s_}", "body", a, d=tube(0.307, [(0.0, 0.052, 0.058), (0.15, 0.05, 0.052), (0.3, 0.043, 0.046)], front=1, cap0=0.03, cap1=0.035), tone=0.4, region="jacket")
        sh(f"forearm-hand-{s_}", f"forearm-{s_}", "body", a+0.02, d=tube(0.242, [(0.0, 0.044, 0.046), (0.12, 0.040, 0.040), (0.233, 0.036, 0.036), (0.28, 0.03, 0.03), (0.33, 0.02, 0.022), (0.36, 0.008, 0.012)], front=1, cap0=0.035, cap1=0.01), tone=0.55, regions=["jacket", "hand-skin"])
        sh(f"thigh-{s_}", f"thigh-{s_}", "body", z, d=tube(0.4046, [(0.0, 0.10, 0.095), (0.1, 0.098, 0.095), (0.25, 0.08, 0.078), (0.405, 0.058, 0.056)], front=1, cap1=0.03), tone=0.3, region="trousers")
        sh(f"shank-{s_}", f"shank-{s_}", "body", z-0.02, d=tube(0.3953, [(0.0, 0.058, 0.058), (0.12, 0.048, 0.064), (0.3, 0.043, 0.048), (0.37, 0.041, 0.042)], front=1), tone=0.3, region="trousers")
        sh(f"shoe-{s_}", f"foot-{s_}", "body", z+0.1, world=WP([(-4.7, 0.4, 'c'), (-4.8, 5), (-4, 8.8), (0.5, 10), (6, 9.5), (15.8, 5.2), (15.8, 0, 'c')]), tone=0.08, region="shoes")
        sh(f"shoe-toe-{s_}", f"toe-{s_}", "body", z+0.1, world=WP([(15, 0, 'c'), (20.5, 0, 'c'), (22.4, 1.2), (22.2, 2.6), (20, 3.8), (15, 4.6, 'c')]), tone=0.08, region="shoes")
    sh("cane", "cane", "body", 6.3, pts=[(0.0, 0.0), (0.86, 0.0)], filled=False, region="cane-wood", pen=0.18)
    sh("cane-crook", "cane", "body", 6.3, world=WP([(26.1, 90.3), (26.2, 95), (28, 98.5), (31, 99.2), (32.8, 97), (31.8, 94.4)], closed=False), filled=False, region="cane-wood", pen=0.18)
    reps["200"] = dict(shapes=list(c.shapes))

    # ---------------------------------------------------------------- 1:100
    c.shapes = []
    sh("head", "head", "body", 5, world=WP([(10.7, 148.2), (9.5, 154), (12.5, 159.5), (18, 161.6), (23.8, 160.3), (26.3, 156.5), (26.4, 152.2), (26.9, 148.2), (27.7, 146.6, 'c'), (26, 145.4), (25, 144.3), (24.4, 143, 'c'), (22.6, 140.5), (18.8, 140.3), (16.5, 142.5), (11.8, 146.1)]), tone=0.8, region="face-skin")
    sh("neck", "neck", "body", 3.8, d=tube(0.093, [(0.0, 0.052, 0.055), (0.05, 0.045, 0.05), (0.095, 0.04, 0.05)], front=-1, cap1=0.01), tone=0.8, region="face-skin")
    sh("hair", "head", "body", 5.4, world=WP([(9.7, 148.1, 'c'), (8, 154.6), (9.8, 158.8), (14.8, 160.8), (19.2, 159, 'c'), (16, 157.9), (12.7, 156.3), (10.7, 151.1)]), tone=0.92, region="hair")
    sh("cap", "head", "body", 5.6, world=WP([(12.2, 159.8), (12.8, 163.2), (19.4, 164), (24, 161), (30.3, 153.9, 'c'), (28, 153, 'c'), (24, 154.6), (17.5, 157.1), (12.4, 158.8)]), tone=0.35, region="cap")
    sh("cap-seam", "head", "detail", 5.65, world=WL([(13.3, 160.2), (19, 161.3), (24.8, 158.9)]), filled=False, region="cap")
    sh("ear", "head", "detail", 5.2, ell=(0.0428, 0.0241, 0.012, 0.02), filled=True, tone=0.75, region="face-skin")
    sh("collar", "neck", "body", 4.5, d=tube(0.093, [(-0.02, 0.09, 0.07), (0.02, 0.06, 0.07), (0.075, 0.062, 0.06)], front=-1, cap1=0.01), tone=0.35, region="jacket")
    sh("jacket", "torso", "body", 4, d=tube(0.4325, [(-0.16, 0.146, 0.091), (-0.02, 0.150, 0.088), (0.10, 0.148, 0.088), (0.19, 0.152, 0.098), (0.26, 0.156, 0.112), (0.33, 0.14, 0.118), (0.40, 0.10, 0.105)], front=-1, cap1=0.03), tone=0.38, region="jacket", hatch="tweed-weave")
    sh("jacket-front", "torso", "detail", 4.1, world=WL([(19.6, 134.3), (17.4, 117.3), (9.3, 79)]), filled=False, region="jacket")
    sh("jacket-pocket", "torso", "detail", 4.1, world=WL([(0.7, 91.1), (10.5, 88.3), (9.3, 83.8)]), filled=False, region="jacket")
    sh("trousers-seat", "pelvis", "body", 3.05, world=WP([(-7.6, 99.3), (-11, 90), (-11.8, 83.1, 'c'), (7, 77.7, 'c'), (11.5, 83.2), (13.2, 93.3)]), tone=0.3, region="trousers")
    for s_, z, a in (("r", 3, 6), ("l", 1, 0.5)):
        h = "far-limb" if s_ == "l" else None
        sh(f"sleeve-{s_}", f"arm-{s_}", "body", a, d=tube(0.307, [(0.0, 0.052, 0.058), (0.15, 0.05, 0.052), (0.3, 0.043, 0.046)], front=1, cap0=0.03, cap1=0.035), tone=0.38, region="jacket", hatch=h or "tweed-weave")
        sh(f"forearm-{s_}", f"forearm-{s_}", "body", a+0.02, d=tube(0.242, [(0.0, 0.044, 0.046), (0.12, 0.040, 0.040), (0.233, 0.036, 0.036)], front=1, cap0=0.035, cap1=0.01), tone=0.38, region="jacket", hatch=h or "tweed-weave")
        sh(f"hand-{s_}", f"hand-{s_}", "body", a+0.2, d=D(cr([(-0.012, 0.03), (0.03, 0.034), (0.075, 0.03), (0.115, 0.022), (0.134, 0.006), (0.128, -0.014), (0.105, -0.026), (0.06, -0.032), (0.02, -0.03), (-0.014, -0.025)])), tone=0.8, region="hand-skin")
        sh(f"thumb-{s_}", f"hand-{s_}", "body", a+0.22, d=D(cr([(-0.004, 0.022), (0.03, 0.05), (0.06, 0.066), (0.09, 0.07), (0.092, 0.058), (0.07, 0.04), (0.03, 0.024)])), tone=0.8, region="hand-skin")
        sh(f"thigh-{s_}", f"thigh-{s_}", "body", z, d=tube(0.4046, [(0.0, 0.10, 0.095), (0.1, 0.098, 0.095), (0.25, 0.08, 0.078), (0.38, 0.062, 0.058), (0.405, 0.056, 0.054)], front=1, cap1=0.03), tone=0.3, region="trousers", hatch=h)
        sh(f"shank-{s_}", f"shank-{s_}", "body", z-0.02, d=tube(0.3953, [(0.0, 0.058, 0.058), (0.12, 0.048, 0.064), (0.3, 0.043, 0.048), (0.37, 0.041, 0.042)], front=1), tone=0.3, region="trousers", hatch=h)
        sh(f"shoe-{s_}", f"foot-{s_}", "body", z+0.1, world=WP([(-4.7, 0.4, 'c'), (-4.8, 5), (-4, 8.8), (0.5, 10.2), (6, 9.7), (10, 8), (15.8, 5.2), (15.8, 0, 'c')]), tone=0.08, region="shoes")
        sh(f"shoe-toe-{s_}", f"toe-{s_}", "body", z+0.1, world=WP([(15, 0, 'c'), (20.5, 0, 'c'), (22.4, 1.2), (22.2, 2.6), (20, 3.8), (15, 4.6, 'c')]), tone=0.08, region="shoes")
        sh(f"sole-{s_}", f"foot-{s_}", "detail", z+0.15, world=WL([(-4.6, 1.8), (15.8, 1.8)]), filled=False, region="shoes")
    sh("cane", "cane", "body", 6.3, pts=[(0.0, 0.0), (0.86, 0.0)], filled=False, region="cane-wood", pen=0.25)
    sh("cane-crook", "cane", "body", 6.3, world=WP([(26.1, 90.3), (26.2, 95), (28, 98.5), (31, 99.2), (32.8, 97), (31.8, 94.4)], closed=False), filled=False, region="cane-wood", pen=0.25)
    reps["100"] = dict(shapes=list(c.shapes))
    return reps


if __name__ == "__main__":
    c = Ctx('elderly-with-cane'); reps = build(c)
    pensets = {"500": {"body": .13, "detail": None, "texture": None, "glazing": None, "ground": .13},
               "200": {"body": .18, "detail": .13, "texture": None, "glazing": None, "ground": .18},
               "100": {"body": .25, "detail": .15, "texture": .1, "glazing": None, "ground": .25}}
    for k, r in reps.items():
        dn = int(k); b = (-0.3, -0.03, 0.7, 1.75)
        for nm, dpi, mode in (("print", 600, "line"), ("zoom", int(800*25.4/(1750/dn)), "line"), ("solid", 600, "solid")):
            p = preview(c, r["shapes"], [], pensets[k], dn, f"eld-{k}-{nm}", dpi, bounds=b if nm == "zoom" else (-0.15, -0.02, 0.55, 1.7), mode=mode)
        print(k, p)

    from sheet import sheet
    items = []
    w = pose_world(c.a)
    for k in ("500", "200", "100"):
        items.append((reps[k]["shapes"], w, int(k), pensets[k], (-0.1, -0.02, 0.5, 1.7), "line"))
    v1 = [s for s in c.a['geometry']['elevation']]
    items.append((v1, w, 50, {"body": .35, "detail": .25, "texture": .13, "glazing": .1, "ground": .35}, (-0.1, -0.02, 0.5, 1.7), "line"))
    print(sheet("eld-sheet", items, 1000))
