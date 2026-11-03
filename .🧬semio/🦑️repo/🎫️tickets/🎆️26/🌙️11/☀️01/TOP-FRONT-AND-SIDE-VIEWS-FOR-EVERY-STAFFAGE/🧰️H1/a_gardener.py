import sys
from lib import Ctx
from human import human, limb2, DEFAULTS, arc
from texts import texts

LAT = dict(hip=0.09, knee=0.09, ankle=0.09, shoulder=0.19, elbow=0.19, wrist=0.19)
CFG = dict(
    arm="upper-arm", neck=False, neckT=0.18,
    lat=LAT, armR=0.05,
    headDx=-0.004, headHalfD=0.1, headHalfW=0.09, faceDx=0.05, skullRy=0.105, skullRx=0.072, headTopDy=0.001, hairFront=False,
    plan=dict(footX=(-0.022, 0.17), toeX=(0.15, 0.175), toe=(0.046, 0.04), hip=(0.12, 0.17), trunk=(0.12, 0.12), shoulder=(0.11, 0.2)),
    front=dict(torso=[(-0.16, 0.165), (0.0, 0.165), (0.3, 0.15), (0.62, 0.17), (0.9, 0.19)], shoeCap=0.047, toeCap=0.055, pelvisW=0.165),
    pelvisYb=0.43, pelvisYt=0.594,
    faceY=dict(eye=0.019, brow=0.04, nose=-0.012, mouth=-0.05),
    R=dict(thigh="trousers", shank="trousers", foot="clogs", toe="clogs", hip="trousers", torso="shirt-torso", upper="shirt-sleeves", fore="forearms", hand="gloves", neck="face", face="face", hair="hat"),
)
EXTRA = {
    100: dict(
        retained=("The pad and the trug stand beside the gardener as separate forms: the pad as a rounded strip, the trug as a round bucket with the weed heap; the wide-brimmed hat keeps crown and brim.", "Polster und Trog stehen als eigene Formen neben der Person: das Polster als abgerundeter Streifen, der Trog als runder Eimer mit dem Unkrauthaufen; der breitkrempige Hut behält Krone und Krempe."),
        omitted=("Weeds are one mound without single stalks.", "Das Unkraut ist ein Haufen ohne einzelne Halme."),
    ),
    50: dict(
        retained=("The knee patches, hat band, eyes, nose, mouth, the trowel with its rib, the rim of the trug and the weed stalks are drawn; the plan shows the brim, crown and band of the hat as three rings.", "Knieflicken, Hutband, Augen, Nase, Mund, die Kelle mit ihrer Mittelrippe, der Rand des Trogs und die Unkrauthalme sind gezeichnet; der Grundriss zeigt Krempe, Krone und Band des Hutes als drei Ringe."),
        omitted=("The gardener's second leg is drawn in front and plan even where the side view hides it behind the first.", "Das zweite Bein der Person ist in Vorderansicht und Grundriss auch dort gezeichnet, wo die Seitenansicht es hinter dem ersten verbirgt."),
    ),
}


def details(c, L):
    O, T = c.O, c.T
    fine = L <= 50
    hc = T["head"][1] + CFG["headTopDy"] - CFG["skullRy"]
    # forearm regions: skin sleeves and gloves share the shape
    for s in ("l", "r"):
        for view in ("front", "plan"):
            for sh in (c.front if view == "front" else c.plan):
                if sh["id"] == f"forearm-{s}":
                    sh.pop("region", None)
                    sh["regions"] = ["forearms", "gloves"]
    c.drop("plan", ["face"])
    # pad and trug
    c.rect("plan", "pad", "pad", -0.56, -0.2, 0.76, 0.4, r=0.04, z=0.5, region="pad")
    c.rect("front", "pad", "pad", -0.2, 0.0, 0.4, 0.04, r=0.015, z=0.5, region="pad")
    c.ell("plan", "trug", "trug", (1.02, 0.0), 0.255, 0.25, z=0.5, region="trug")
    c.poly("front", "trug", "trug", [(-0.21, 0.0), (0.21, 0.0), (0.255, 0.21), (-0.255, 0.21)], z=7.0, region="trug")
    c.poly("front", "trug-weeds", "trug", [(-0.2, 0.205)] + arc(0, 0.205, 0.2, 0.107, 180, 0, 12) + [(0.2, 0.205)], z=7.05, region="trug")
    c.ell("plan", "trug-weeds", "trug", (1.02, 0.0), 0.2, 0.19, z=0.55, region="trug", role="detail", filled=True)
    c.poly("front", "trug-rim", "trug", [(-0.255, 0.21), (0.255, 0.21)], closed=False, z=7.06, region="trug", role="detail")
    c.ell("plan", "trug-rim", "trug", (1.02, 0.0), 0.225, 0.22, z=0.56, region="trug", role="detail", filled=False)
    # trowel in the right hand
    c.cap("plan", "trowel-blade", "trowel", (0.0, -LAT["wrist"]), (0.0, -LAT["wrist"]), 0.033, 0.033, z=3.7, region="trowel")
    limb2(c, "trowel-blade", "trowel", -LAT["wrist"], -LAT["wrist"], [(0.0, 0.016), (0.4, 0.034), (0.8, 0.03), (1.0, 0.004)], t0=0.3, t1=1.0, z=4.8, region="trowel")
    # hat
    c.ell("plan", "hat-brim", "head", (0.0, 0.0), 0.235, 0.235, z=5.9, region="hat")
    c.poly("front", "hat-crown", "head", [(-0.104, 1.233)] + arc(0, 1.233, 0.104, 0.107, 180, 0, 14) + [(0.104, 1.233)], z=6.4, region="hat")
    bx0, bx1, by0, by1 = c.bbox_local(c.elev("hat-brim"))
    c.add("front", {"kind": "poly", "points": [[round(bx0, 4), 0.0], [round(bx0, 4), round(by1, 4)], [round(bx1, 4), round(by1, 4)], [round(bx1, 4), 0.0]], "closed": True}, "hat-brim", "head", 6.3, region="hat",
          solid={"kind": "revolved", "axisBone": "head", "axisX": round(O["head"][0], 4), "mirror": True})
    if L <= 100:
        for s, sg in (("l", 1), ("r", -1)):
            c.ell("front", f"knee-patch-{s}", f"shank-{s}", (sg * LAT["knee"], O[f"shank-{s}"][1] + 0.044), 0.04, 0.04, z=1.09, region="trousers", role="detail", filled=False)
    if fine:
        c.poly("front", "hat-band", "head", [(-0.104, 1.262), (0.104, 1.262)], closed=False, z=6.5, region="hat", role="detail")
        c.ell("plan", "hat-band", "head", (0.0, 0.0), 0.104, 0.1, z=6.1, region="hat", role="detail", filled=False)
        c.ell("plan", "hat-crown-ring", "head", (0.0, 0.0), 0.13, 0.13, z=6.05, region="hat", role="detail", filled=False)
        c.poly("front", "trowel-rib", "trowel", [(-LAT["wrist"], 0.3), (-LAT["wrist"], 0.1)], closed=False, follow=True, z=4.85, region="trowel", role="detail")
        for i, dx in enumerate((-0.07, -0.02, 0.04, 0.09)):
            c.poly("front", f"weed-stalk-{i}", "trug", [(dx, 0.21), (dx * 1.3, 0.3)], closed=False, z=7.07, region="trug", role="detail")
        for s, sg in (("l", 1), ("r", -1)):
            c.poly("front", f"clog-strap-{s}", f"foot-{s}", [(sg * LAT["ankle"] - 0.04, -0.262), (sg * LAT["ankle"] + 0.04, -0.262)], closed=False, z=1.3, region="clogs", role="detail")


def build(scales=None):
    c = Ctx("kneeling-gardener")
    for L in (scales or c.scales):
        c.start(L)
        human(c, L, CFG)
        details(c, L)
        en, de = texts(L, EXTRA)
        c.commit(L, en, de)
    c.save()


if __name__ == "__main__":
    build([int(x) for x in sys.argv[1:]] or None)
