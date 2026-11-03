import sys
from lib import Ctx
from human import human, limb2, arc
from texts import texts

LAT = dict(hip=0.06, knee=0.065, ankle=0.065, shoulder=0.125, elbow=0.15, wrist=0.19)
CFG = dict(
    arm="arm", neck=True,
    lat=LAT, armR=0.042,
    headDx=0.01, headHalfD=0.08, headHalfW=0.068, faceDx=0.045, skullRy=0.092, skullRx=0.062, headTopDy=0.0, hairFront=False,
    plan=dict(footX=(-0.046, 0.068), toeX=(0.12, 0.145), thigh=(0.062, 0.048), shank=(0.045, 0.034), foot=(0.032, 0.037), toe=(0.037, 0.03),
              upper=(0.042, 0.037), fore=(0.037, 0.028), hand=(0.026, 0.019), hip=(0.098, 0.135), trunk=(0.1, 0.1), shoulder=(0.09, 0.165), heel=0.02),
    front=dict(
        thigh=[(0, 0.07), (0.45, 0.062), (1, 0.046)], shank=[(0, 0.045), (0.3, 0.046), (0.75, 0.034), (1, 0.03)],
        foot=[(0, 0.032), (0.6, 0.037), (1, 0.037)], toe=[(0, 0.037), (1, 0.032)],
        torso=[(-0.13, 0.135), (0, 0.135), (0.3, 0.12), (0.62, 0.135), (0.9, 0.15)], neck=[(0, 0.04), (1, 0.036)],
        upper=[(0, 0.042), (0.5, 0.039), (1, 0.034)], fore=[(0, 0.034), (0.35, 0.036), (1, 0.026)], hand=[(0, 0.026), (0.5, 0.032), (1, 0.018)],
        pelvisW=0.135, shoeCap=0.041, toeCap=0.057, handCap=0.025),
    pelvisYb=0.594, pelvisYt=0.759,
    faceY=dict(eye=0.0, brow=0.018, nose=-0.03, mouth=-0.055),
    R=dict(thigh="leggings", shank="leggings", foot="sneakers", toe="sneakers", hip="jacket", torso="jacket", upper="jacket", fore="jacket", hand="hands", neck="face-skin", face="face-skin", hair="hair"),
)
EXTRA = {
    500: dict(
        merged=("Deck, wheels and rear fender are one stadium in plan; the handlebar is thickened to the printable stroke width (0.1 m of model width), the stem and the fork are merged into the handlebar and the wheel shapes.", "Brett, Räder und Hinterradschutzblech sind im Grundriss ein Stadion; der Lenker ist auf die druckbare Strichbreite (0,1 m Modellbreite) verdickt, Lenkerstange und Gabel gehen in Lenker und Radformen auf."),
    ),
    200: dict(
        retained=("The scooter keeps deck, both wheels, stem with the handlebar and its grips, fork and fender as separate shapes; the backpack is its own form.", "Der Roller behält Trittbrett, beide Räder, Lenkstange mit Lenker und Griffen, Gabel und Schutzblech als eigene Formen; der Rucksack ist eine eigene Form."),
    ),
    100: dict(
        retained=("Head tube, clamp, struts, the backpack straps and pocket, the ponytail behind the helmet and the jacket buttons are kept.", "Steuerrohr, Klemme, Streben, Rucksackriemen und -tasche, der Zopf hinter dem Helm und die Jackenknöpfe bleiben erhalten."),
    ),
    50: dict(
        retained=("Wheel hubs, brake pad, sternum strap, button line, helmet strap, eyes, nose and mouth and the sneaker soles are drawn; the plan shows the strut pair, the rear fender and the two fork blades.", "Radnaben, Bremsklotz, Brustgurt, Knopfleiste, Helmriemen, Augen, Nase und Mund sowie die Sneaker-Sohlen sind gezeichnet; der Grundriss zeigt das Strebenpaar, das Hinterradschutzblech und die beiden Gabelholme."),
    ),
}


def scooter(c, L):
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    deck_w = 0.065
    if L >= 500:
        c.cap("plan", "scooter-base", "frame", (-0.157, 0.0), (0.638, 0.0), 0.065, 0.065, follow=False, z=0.5, regions=["deck", "wheels"])
        c.cap("plan", "handlebar", "frame", (0.45, -0.175), (0.45, 0.175), 0.05, 0.05, follow=False, z=2.8, regions=["grips", "stem"])
        c.cap("front", "wheel-front", "wheel-front", (0.0, 0.05), (0.0, 0.095), 0.05, 0.05, follow=False, z=9.2, regions=["wheels", "stem"])
        c.rect("front", "bar-stem", "frame", -0.225, 0.78, 0.45, 0.1, follow=False, z=4.58, regions=["grips", "stem"])
        c.rect("front", "stem-column", "frame", -0.05, 0.1, 0.1, 0.7, follow=False, z=9.1, region="stem")
        c.rect("front", "deck", "frame", -deck_w, 0.08, 2 * deck_w, 0.066, follow=False, z=9.0, region="deck")
        return
    ww = 0.015
    c.rect("plan", "deck", "frame", -0.1, -deck_w, 0.59, 2 * deck_w, r=0.02, z=0.5, region="deck")
    c.rect("plan", "fender", "frame", -0.22, -0.04, 0.22, 0.08, r=0.03, z=0.5, region="deck")
    c.cap("plan", "tyre-rear", "wheel-rear", (-0.207, 0.0), (-0.092, 0.0), ww, ww, follow=False, z=0.4, region="wheels")
    c.cap("plan", "tyre-front", "wheel-front", (0.5715, 0.0), (0.6885, 0.0), ww, ww, follow=False, z=0.4, region="wheels")
    c.cap("plan", "stem", "frame", (0.44, 0.0), (0.54, 0.0), 0.014, 0.014, follow=False, z=0.6, region="stem")
    c.ell("plan", "head-tube", "frame", (0.525, 0.0), 0.028, 0.026, z=0.6, region="stem")
    for s, sg in (("l", 1), ("r", -1)):
        c.cap("plan", f"fork-{s}", "frame", (0.53, sg * 0.026), (0.64, sg * 0.026), 0.012, 0.012, follow=False, z=0.6, region="stem")
    c.cap("plan", "handlebar", "frame", (0.45, -0.19), (0.45, 0.19), 0.0175, 0.0175, follow=False, z=2.8, region="stem")
    for s, sg in (("l", 1), ("r", -1)):
        c.cap("plan", f"grip-{s}", "frame", (0.45, sg * 0.15), (0.45, sg * 0.1925), 0.0175, 0.0175, follow=False, z=2.9, region="grips")
    # front view
    c.cap("front", "tyre-rear", "wheel-rear", (0.0, 0.0145), (0.0, 0.1305), ww, ww, follow=False, z=9.2, region="wheels")
    c.cap("front", "tyre-front", "wheel-front", (0.0, 0.0145), (0.0, 0.1305), ww, ww, follow=False, z=9.25, region="wheels")
    c.rect("front", "deck", "frame", -deck_w, 0.08, 2 * deck_w, 0.066, follow=False, z=9.0, region="deck")
    c.rect("front", "stem", "frame", -0.014, 0.157, 0.028, 0.676, follow=False, z=9.1, region="stem")
    for s, sg in (("l", 1), ("r", -1)):
        c.rect("front", f"fork-{s}", "frame", (0.026 if sg > 0 else -0.048), 0.0725, 0.022, 0.0855, follow=False, z=9.1, region="stem")
    c.rect("front", "handlebar", "frame", -0.2075, 0.823, 0.415, 0.035, r=0.0175, z=4.57, follow=False, region="stem")
    for s, sg in (("l", 1), ("r", -1)):
        c.rect("front", f"grip-{s}", "frame", (0.1 if sg > 0 else -0.225), 0.823, 0.125, 0.035, r=0.0175, z=4.58, follow=False, region="grips")
    c.rect("front", "head-tube", "frame", -0.028, 0.082, 0.056, 0.116, follow=False, z=9.15, region="stem")
    if mid_ or fine:
        c.rect("front", "clamp", "frame", -0.022, 0.773, 0.044, 0.044, follow=False, z=9.4, region="stem", role="detail")
        for s, sg in (("l", 1), ("r", -1)):
            c.cap("plan", f"strut-{s}", "frame", (-0.158, sg * 0.03), (-0.08, sg * 0.03), 0.006, 0.006, follow=False, z=0.55, region="stem", role="detail")
    if fine:
        for nm, bone in (("rear", "wheel-rear"), ("front", "wheel-front")):
            c.ell("front", f"hub-{nm}", bone, (0.0, 0.0725), 0.014, 0.014, z=9.4 + (0.05 if nm == "front" else 0.0), region="wheels", role="detail")
        c.rect("plan", "brake-pad", "frame", -0.13, -0.03, 0.08, 0.06, z=0.6, region="deck", role="detail")


def details(c, L):
    O, T = c.O, c.T
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    hc = T["head"][1] + CFG["headTopDy"] - CFG["skullRy"]
    hx = (O["head"][0] + T["head"][0]) / 2 + CFG["headDx"]
    c.retag("plan", "head", "face-skin")
    c.ell("plan", "helmet", "head", (hx - 0.005, 0), 0.088, 0.078, z=6.5, region="helmet")
    if L >= 500:
        c.poly("front", "helmet-head", "head", arc(0, 1.21, 0.088, 0.113, -8, 188, 14), z=6.6, regions=["helmet", "hair"])
    else:
        c.poly("front", "helmet", "head", arc(0, 1.21, 0.088, 0.113, -8, 188, 14), z=6.6, region="helmet")
        c.cap("plan", "hair-tail", "head", (hx - 0.105, 0), (hx - 0.095, 0), 0.018, 0.018, follow=False, z=5.4, region="hair")
        c.poly("front", "hair-nape", "head", [(-0.035, 1.1), (0.035, 1.1), (0.04, 1.14), (-0.04, 1.14)], z=5.0, region="hair")
    # backpack
    if L >= 500:
        c.rect("plan", "backpack", "neck", -0.202, -0.115, 0.122, 0.23, follow=False, z=3.9, region="backpack")
    else:
        c.rect("plan", "backpack", "neck", -0.202, -0.115, 0.122, 0.23, r=0.03, z=3.9, region="backpack")
        for s, sg in (("l", 1), ("r", -1)):
            c.poly("front", f"backpack-strap-{s}", "torso", [(sg * 0.075, 1.03), (sg * 0.085, 0.78)], closed=False, follow=True, z=5.5, region="backpack", role="detail")
    if mid_ or fine:
        c.rect("plan", "backpack-pocket", "neck", -0.19, -0.07, 0.04, 0.14, z=4.0, region="backpack", role="detail", filled=False)
        c.poly("front", "jacket-front", "torso", [(0.0, 1.108), (0.0, 0.749)], closed=False, follow=True, z=5.2, region="jacket", role="detail")
    if fine:
        for i, y in enumerate((0.827, 0.899, 0.971)):
            c.ell("front", f"button-{i}", "torso", (0.014, y), 0.009, 0.009, z=5.3, region="jacket", role="detail")
        c.poly("front", "sternum-strap", "torso", [(-0.075, 0.95), (0.075, 0.95)], closed=False, follow=True, z=5.6, region="backpack", role="detail")
        for s, sg in (("l", 1), ("r", -1)):
            c.poly("front", f"helmet-strap-{s}", "head", [(sg * 0.05, 1.209), (sg * 0.042, 1.12)], closed=False, z=6.7, region="helmet", role="detail")
            c.poly("plan", f"sole-{s}", f"foot-{s}", [(-0.05, sg * 0.065 + 0.0), (0.09, sg * 0.065)], closed=False, z=1.4, region="sneakers", role="detail")
    scooter(c, L)


def build(scales=None):
    c = Ctx("child-on-kick-scooter")
    for L in (scales or c.scales):
        c.start(L)
        human(c, L, CFG)
        details(c, L)
        en, de = texts(L, EXTRA)
        c.commit(L, en, de)
    c.save()


if __name__ == "__main__":
    build([int(x) for x in sys.argv[1:]] or None)
