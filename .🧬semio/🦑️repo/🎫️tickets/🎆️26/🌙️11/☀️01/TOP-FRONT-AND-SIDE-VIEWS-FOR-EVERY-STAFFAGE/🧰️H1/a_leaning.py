import sys
from lib import Ctx
from human import human, limb2, DEFAULTS, arc
from texts import texts

LAT = dict(hip=0.09, knee=0.092, ankle=0.095, shoulder=0.175, elbow=0.18, wrist=0.185)
CFG = dict(
    arm="upper-arm", neck=False, neckT=0.28,
    lat=LAT, armR=0.05,
    headDx=-0.006, headHalfD=0.098, headHalfW=0.077, faceDx=0.055, skullRy=0.108, skullRx=0.075, headTopDy=0.034, hairFront=False,
    plan=dict(footX=(-0.024, 0.081), toeX=(0.13, 0.17), toe=(0.046, 0.04), hip=(0.118, 0.17), trunk=(0.124, 0.124), shoulder=(0.11, 0.2)),
    front=dict(torso=[(-0.305, 0.165), (0.0, 0.165), (0.3, 0.15), (0.62, 0.172), (0.9, 0.19)], shoeCap=0.03, toeCap=0.055, pelvisW=0.17),
    pelvisYb=-0.113, pelvisYt=0.098,
    R=dict(thigh="leg-clothing", shank="leg-clothing", foot="shoes", toe="shoes", hip="leg-clothing", torso="torso-clothing", upper="arm-clothing", fore="arm-clothing", hand="hand-skin", neck="head-skin", face="head-skin", hair="hair"),
)
EXTRA = {
    500: dict(
        omitted=("The wall is a single ground line: the courses are not drawn, and the phone is part of the hand.", "Die Wand ist eine einzelne Bodenlinie: die Mauerschichten sind nicht gezeichnet, und das Telefon geht in der Hand auf."),
    ),
    200: dict(
        retained=("The wall appears as one section line in plan and one backdrop outline in front; the beanie is one shape with the hair region.", "Die Wand erscheint als eine Schnittlinie im Grundriss und als ein Hintergrundumriss in der Vorderansicht; die Mütze ist eine Form mit der Haarregion."),
    ),
    100: dict(
        retained=("The wall is a masonry section in plan with its courses and a backdrop with course lines in front; the hood, beanie rim and the phone in the right hand are kept.", "Die Wand ist im Grundriss ein Mauerwerksschnitt mit Schichten und in der Vorderansicht ein Hintergrund mit Schichtlinien; Kapuze, Mützenrand und das Telefon in der rechten Hand bleiben erhalten."),
    ),
    50: dict(
        retained=("The kangaroo pocket, hood strings, hem, cuffs and trouser hems are drawn in front; the plan adds the phone with its glazed screen, the beanie rim and the wall courses.", "Känguru-Tasche, Kapuzenkordeln, Saum, Bündchen und Hosensäume sind vorn gezeichnet; der Grundriss ergänzt das Telefon mit Glasdisplay, den Mützenrand und die Mauerschichten."),
    ),
}


def details(c, L):
    O, T = c.O, c.T
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    hc = T["head"][1] + CFG["headTopDy"] - CFG["skullRy"]
    hx = (O["head"][0] + T["head"][0]) / 2 + CFG["headDx"]
    rx, ry = CFG["skullRx"], CFG["skullRy"]
    if L < 500:
        beanie = arc(0, hc, rx + 0.005, ry + 0.005, -8, 188, 14) + [(-rx - 0.003, hc + 0.03), (rx + 0.003, hc + 0.03)]
        c.poly("front", "beanie", "head", beanie, z=6.2, region="hair")
        if mid_ or fine:
            c.rect("front", "beanie-rim", "head", -rx - 0.007, hc + 0.03 - 0.034, 2 * rx + 0.014, 0.034, r=0.01, z=6.25, region="hair", role="detail")
        if L <= 200:
            c.poly("front", "hood", "head", [(-0.125, 0.545), (-0.1, 0.52), (-0.05, 0.505), (0.0, 0.5), (0.05, 0.505), (0.1, 0.52), (0.125, 0.545), (0.105, 0.63), (0.0, 0.64), (-0.105, 0.63)], z=3.9, region="torso-clothing")
            c.ell("plan", "hood", "head", (-0.038, 0), 0.105, 0.09, z=3.98, region="torso-clothing")
    # phone in the right hand (hand-r), wall
    if L <= 100:
        c.rect("front", "phone", "hand-r", -LAT["wrist"] - 0.032, -0.199, 0.064, 0.125, r=0.008, follow=True, z=4.7, region="phone-surface")
        c.rect("plan", "phone", "hand-r", -0.045, -LAT["wrist"] - 0.032, 0.032, 0.064, r=0.006, z=3.2, region="phone-surface")
    if fine:
        c.rect("plan", "phone-screen", "hand-r", -0.0185, -LAT["wrist"] - 0.025, 0.0055, 0.05, z=3.3, region="phone-surface", role="glazing")
    # wall: section in plan (ground, unfilled), backdrop outline in front
    c.rect("plan", "wall-surface", "wall", -0.12, -0.6, 0.12, 1.2, z=0.1, region="wall-face", role="ground", filled=False)
    c.rect("front", "wall-surface", "wall", -0.6, 0.0, 1.2, 1.9, z=0.1, region="wall-face", role="ground", filled=False)
    if mid_ or fine:
        for i in range(1, 4):
            y = -0.6 + i * 0.3
            c.poly("plan", f"wall-course-{i}", "wall", [(-0.12, y), (0.0, y)], closed=False, z=0.2, region="wall-face", role="ground")
            c.poly("front", f"wall-course-{i}", "wall", [(-0.6, 0.3 * i), (0.6, 0.3 * i)], closed=False, z=0.2, region="wall-face", role="ground")
    if fine:
        for s, sg in (("l", 1), ("r", -1)):
            c.poly("front", f"hood-string-{s}", "torso", [(sg * 0.03, 0.547), (sg * 0.03, 0.40)], closed=False, follow=True, z=4.2, region="torso-clothing", role="detail")
            c.poly("front", f"trouser-hem-{s}", f"shank-{s}", [(sg * 0.095 - 0.05, -0.796), (sg * 0.095 + 0.05, -0.796)], closed=False, follow=True, z=1.1, region="leg-clothing", role="detail")
            c.poly("front", f"cuff-{s}", f"forearm-{s}", [(sg * 0.185 - 0.04, -0.024), (sg * 0.185 + 0.04, -0.024)], closed=False, follow=True, z=4.6, region="arm-clothing", role="detail")
            c.poly("front", f"jeans-seam-{s}", f"thigh-{s}", [(sg * (0.09 + 0.075), -0.059), (sg * (0.095 + 0.07), -0.391)], closed=False, follow=True, z=3.1, region="leg-clothing", role="detail")
        c.poly("front", "hoodie-pocket", "torso", [(-0.1, 0.176), (-0.12, 0.078), (0.12, 0.078), (0.1, 0.176)], closed=False, follow=True, z=4.1, region="torso-clothing", role="detail")
        c.poly("front", "hoodie-hem", "torso", [(-0.165, -0.039), (0.165, -0.039)], closed=False, follow=True, z=4.1, region="torso-clothing", role="detail")


def build(scales=None):
    c = Ctx("leaning-person")
    for L in (scales or c.scales):
        c.start(L)
        human(c, L, CFG)
        details(c, L)
        en, de = texts(L, EXTRA)
        c.commit(L, en, de)
    c.save()


if __name__ == "__main__":
    build([int(x) for x in sys.argv[1:]] or None)
