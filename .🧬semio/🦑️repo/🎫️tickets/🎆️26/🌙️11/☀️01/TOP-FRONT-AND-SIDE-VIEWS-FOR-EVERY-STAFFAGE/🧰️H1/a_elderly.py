import sys
from lib import Ctx
from human import human, limb2, DEFAULTS, arc
from texts import texts

# the cane is held in the right hand: right wrist and cane sit a little outside the elbow
LAT = dict(hip=0.09, knee=0.095, ankle=0.098, shoulder=0.185, elbow=0.19, wrist=0.2)
CFG = dict(
    arm="arm", neck=True,
    lat=LAT, armR=0.05,
    headDx=0.0, headHalfD=0.098, headHalfW=0.076, faceDx=0.055, skullRy=0.1125, skullRx=0.076, headTopDy=0.003, hairline=0.03,
    plan=dict(footX=(-0.012, 0.11), toeX=(0.16, 0.18), toe=(0.046, 0.04), hip=(0.125, 0.165), trunk=(0.13, 0.13), shoulder=(0.11, 0.195)),
    front=dict(torso=[(-0.47, 0.175), (-0.2, 0.17), (0.1, 0.16), (0.5, 0.172), (0.9, 0.19)], shoeCap=0.032, toeCap=0.06, pelvisW=0.165),
    pelvisYb=0.777, pelvisYt=0.993,
    faceY=dict(eye=-0.006, brow=0.015, nose=-0.04, mouth=-0.07),
    R=dict(thigh="trousers", shank="trousers", foot="shoes", toe="shoes", hip="trousers", torso="jacket", upper="jacket", fore="jacket", hand="hand-skin", neck="face-skin", face="face-skin", hair="hair"),
)
CANE_Y = -0.225
EXTRA = {
    500: dict(
        merged=("Cane shaft and crook are one thickened stroke in front; in plan the cane is the crook stadium (the shaft is hidden below it).", "Stockschaft und Krücke sind vorn ein verdickter Strich; im Grundriss ist der Stock das Krückenstadion (der Schaft liegt darunter verborgen)."),
    ),
    200: dict(
        retained=("The cane is drawn as shaft with crook in front and as crook with shaft dot in plan; the flat cap has its own shape over the white hair.", "Der Stock ist vorn als Schaft mit Krücke und im Grundriss als Krücke mit Schaftpunkt gezeichnet; die Schiebermütze hat eine eigene Form über dem weißen Haar."),
    ),
    100: dict(
        retained=("The cane has crook, shaft and ferrule, the cap has its brim, the jacket has a button line and the head shows the white hair at the sides.", "Der Stock hat Krücke, Schaft und Zwinge, die Mütze ihren Schirm, die Jacke eine Knopfleiste und der Kopf zeigt seitlich das weiße Haar."),
    ),
    50: dict(
        retained=("Glasses, forehead wrinkles, cap seam, jacket buttons and pocket, cane grain and ferrule ring are drawn; the plan shows the cap with its brim over the hair ring and the cane crook with the thumb of the right hand.", "Brille, Stirnfalten, Mützennaht, Jackenknöpfe und -tasche, Stockmaserung und Zwingenring sind gezeichnet; der Grundriss zeigt die Mütze mit Schirm über dem Haarkranz und die Stockkrücke mit dem Daumen der rechten Hand."),
    ),
}


def details(c, L):
    O, T = c.O, c.T
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    hc = T["head"][1] + CFG["headTopDy"] - CFG["skullRy"]
    hx = (O["head"][0] + T["head"][0]) / 2
    rx, ry = CFG["skullRx"], CFG["skullRy"]
    # cane (right hand): shaft + crook + ferrule
    if L >= 500:
        cy = -0.19
        c.cap("plan", "cane", "cane", (O["cane"][0], cy), (T["cane"][0] - 0.012, cy), 0.05, 0.05, z=6.3, region="cane-wood")
        limb2(c, "cane", "cane", cy, cy, [(0, 0.05), (1, 0.05)], z=6.3, region="cane-wood", cap0=0.0)
    else:
        w = 0.022 if L > 100 else (0.02 if L == 100 else 0.012)
        c.slices("plan", "cane-crook", "cane", c.bbox_local(c.elev("cane-crook")), [w + 0.002, w + 0.002, 0.85 * (w + 0.002), 0.6 * (w + 0.002)], CANE_Y, along="y", z=6.35, region="cane-wood")
        c.cap("plan", "cane-shaft", "cane", (O["cane"][0], CANE_Y), (T["cane"][0], CANE_Y), w, (0.0125 if fine else w + 0.004) if (mid_ or fine) else w + 0.002, z=6.3, region="cane-wood")
        limb2(c, "cane-shaft", "cane", CANE_Y, CANE_Y, [(0, w), (0.92, w), (1, w + 0.002)], cap0=0.02, z=6.3, region="cane-wood")
        if mid_ or fine:
            limb2(c, "cane-ferrule", "cane", CANE_Y, CANE_Y, [(0.93, w + 0.004), (1, w + 0.002)], t0=0.93, t1=1.0, z=6.4, region="cane-wood")
    # flat cap, hair at the sides
    if L < 500:
        c.poly("front", "cap", "head", [(-rx - 0.012, hc + 0.045)] + arc(0, hc + 0.045, rx + 0.012, 0.075, 180, 0, 14) + [(rx + 0.012, hc + 0.045)], z=6.6, region="cap")
        c.ell("front", "cap-brim", "head", (0.0, hc + 0.045), rx + 0.022, 0.016, z=6.65, region="cap")
        c.ell("plan", "cap", "head", (hx + 0.015, 0), 0.093, 0.073, z=6.4, region="cap")
        c.cap("plan", "cap-brim", "head", (hx + 0.085, 0), (hx + 0.1, 0), 0.05, 0.04, follow=False, z=6.45, region="cap")
    # jacket details
    if mid_ or fine:
        c.poly("front", "jacket-front", "torso", [(0.0, 1.343), (0.0, 0.79)], closed=False, follow=True, z=4.1, region="jacket", role="detail")
        if fine:
            for i, y in enumerate((1.004, 0.92, 1.09)):
                c.ell("front", f"jacket-button-{i}", "torso", (0.012, y), 0.009, 0.009, z=4.2, region="jacket", role="detail")
            c.poly("front", "jacket-pocket-l", "torso", [(0.08, 0.911), (0.15, 0.911), (0.15, 0.838), (0.08, 0.838)], closed=False, follow=True, z=4.1, region="jacket", role="detail")
            c.poly("front", "jacket-pocket-r", "torso", [(-0.08, 0.911), (-0.15, 0.911), (-0.15, 0.838), (-0.08, 0.838)], closed=False, follow=True, z=4.1, region="jacket", role="detail")
            for s, sg in (("l", 1), ("r", -1)):
                c.ell("front", f"glasses-{s}", "head", (sg * 0.03, hc - 0.005), 0.022, 0.016, z=6.35, region="face-skin", role="detail", filled=False)
            c.poly("front", "glasses-bridge", "head", [(-0.01, hc - 0.002), (0.01, hc - 0.002)], closed=False, z=6.35, region="face-skin", role="detail")
            for i, dy in enumerate((0.04, 0.05)):
                c.poly("front", f"wrinkle-{i}", "head", [(-0.035, hc + dy), (0.0, hc + dy + 0.004), (0.035, hc + dy)], closed=False, z=6.3, region="face-skin", role="texture")
            c.poly("front", "cap-seam", "head", [(0.0, hc + 0.05), (0.0, hc + 0.118)], closed=False, z=6.7, region="cap", role="detail")
            c.poly("plan", "cap-seam", "head", [(hx - 0.04, 0), (hx + 0.07, 0)], closed=False, z=6.5, region="cap", role="detail")
            for s, sg in (("l", 1), ("r", -1)):
                c.poly("front", f"trouser-crease-{s}", f"thigh-{s}", [(sg * 0.093, 0.883), (sg * 0.093, 0.455)], closed=False, follow=True, z=3.1, region="trousers", role="detail")


def build(scales=None):
    c = Ctx("elderly-with-cane")
    for L in (scales or c.scales):
        c.start(L)
        human(c, L, CFG)
        details(c, L)
        en, de = texts(L, EXTRA)
        c.commit(L, en, de)
    c.save()


if __name__ == "__main__":
    build([int(x) for x in sys.argv[1:]] or None)
