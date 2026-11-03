import sys
from lib import Ctx
from human import human, limb2, arc, merge
from texts import texts

LAT = dict(hip=0.095, knee=0.15, ankle=0.165, shoulder=0.19, elbow=0.23, wrist=0.28)
CFG = dict(
    arm="arm", neck=True,
    lat=LAT, armR=0.05,
    headDx=0.0, headHalfD=0.1, headHalfW=0.077, faceDx=0.058, skullRy=0.112, skullRx=0.077, headTopDy=0.002, hairFront=False,
    plan=dict(footX=(0.241, 0.373), toeX=(0.43, 0.45), toe=(0.045, 0.035), hip=(0.13, 0.17), trunk=(0.125, 0.125), shoulder=(0.11, 0.2)),
    front=dict(thigh=[(0, 0.072), (0.45, 0.066), (1, 0.052)], torso=[(-0.36, 0.17), (-0.1, 0.16), (0.3, 0.148), (0.62, 0.17), (0.9, 0.19)], shoeCap=0.034, toeCap=0.065, pelvisW=0.17),
    pelvisYb=0.92, pelvisYt=1.119,
    faceY=dict(eye=0.0, brow=0.02, nose=-0.04, mouth=-0.07),
    R=dict(thigh="trousers", shank="trousers", foot="shoes", toe="shoes", hip="trousers", torso="jacket", upper="jacket", fore="jacket", hand="gloves", neck="face-skin", face="face-skin", hair="hair"),
)
EXTRA = {
    500: dict(
        merged=("Tyres, frame tubes, fork and handlebar are thickened to the printable stroke width (0.1 m of model width) and drawn as one frame shape per view; the plan of the bicycle is one stadium per wheel, one frame stadium, the handlebar and the pedals.", "Reifen, Rahmenrohre, Gabel und Lenker sind auf die druckbare Strichbreite (0,1 m Modellbreite) verdickt und je Ansicht als eine Rahmenform gezeichnet; der Grundriss des Fahrrads besteht aus einem Stadion je Rad, einem Rahmenstadion, dem Lenker und den Pedalen."),
    ),
    200: dict(
        retained=("The bicycle keeps both wheels, main tube, stays, fork, handlebar with grips, saddle, cranks and pedals as separate shapes; the pedals and cranks follow the pedal cycle in plan and front.", "Das Fahrrad behält beide Räder, Hauptrohr, Streben, Gabel, Lenker mit Griffen, Sattel, Kurbeln und Pedale als eigene Formen; Pedale und Kurbeln folgen dem Tretzyklus in Grundriss und Vorderansicht."),
    ),
    100: dict(
        retained=("Fenders, headlamp, chain, chainring, trouser clips, helmet with its strap and the pony tail are kept; the front wheel covers the rear wheel in front.", "Schutzbleche, Scheinwerfer, Kette, Kettenblatt, Hosenklammern, Helm mit Kinnriemen und der Zopf bleiben erhalten; das Vorderrad verdeckt vorn das Hinterrad."),
    ),
    50: dict(
        retained=("Spoke hubs, axle ends, jacket zip, reflective band, collar, trouser clips, helmet vents and strap, eyes, nose and mouth are drawn; the plan shows the saddle outline, chain, chainring and both swept handlebar sides with grips.", "Nabenachsen, Jackenreißverschluss, Reflexstreifen, Kragen, Hosenklammern, Helmschlitze und Kinnriemen, Augen, Nase und Mund sind gezeichnet; der Grundriss zeigt den Sattelumriss, Kette, Kettenblatt und beide geschwungenen Lenkerseiten mit Griffen."),
    ),
}


def bike(c, L):
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    tw = 0.1 if L >= 500 else (0.06 if L >= 200 else 0.048)
    r = tw / 2
    zt = 1.4
    # ---- plan
    for nm, bone, x0 in (("rear", "rear-wheel", -0.34), ("front", "front-wheel", 0.71)):
        c.add("plan", {"kind": "ellipse", "cx": 0.0, "cy": 0.0, "rx": 0.34, "ry": 0.34}, f"tyre-{nm}", bone, zt, region="tyres", solid={"kind": "extruded", "z0": -r, "z1": r})
    fr = 0.05 if L >= 500 else (0.016 if L >= 100 else 0.014)
    c.cap("plan", "main-tube", "frame", (0.27, 0), (0.97, 0), fr, fr, follow=False, z=1.8, region="bike-metal")
    if L >= 500:
        c.cap("plan", "fork-bars", "frame", (0.975, 0), (1.06, 0), 0.05, 0.05, follow=False, z=1.85, region="bike-metal")
    else:
        for s, sg in (("l", 1), ("r", -1)):
            sr = 0.011 if L >= 100 else 0.009
            c.cap("plan", f"chainstay-{s}", "frame", (0.0, sg * 0.065), (0.44, sg * 0.035), sr, sr, follow=False, z=1.8, region="bike-metal")
            c.cap("plan", f"seatstay-{s}", "frame", (0.0, sg * 0.065), (0.27, sg * 0.02), sr, sr, follow=False, z=1.8, region="bike-metal")
            c.cap("plan", f"fork-{s}", "frame", (0.98, sg * 0.045), (1.05, sg * 0.05), sr, sr, follow=False, z=1.8, region="bike-metal")
            c.cap("plan", f"crank-{s}", f"crank-{s}", (0.45, sg * 0.088), (0.45, sg * 0.088), 0.026 if L >= 200 else 0.014, 0.026 if L >= 200 else 0.014, z=2.3 if s == "r" else 0.9, region="bike-metal")
            c.rect("plan", f"pedal-{s}", f"pedal-{s}", 0.398, sg * 0.095 - (0.0 if sg > 0 else 0.095), 0.104, 0.095, z=2.35 if s == "r" else 0.95, region="bike-metal")
        c.ell("plan", "head-tube", "frame", (0.972, 0), 0.02, 0.02, z=1.85, region="bike-metal")
        c.cap("plan", "stem", "frame", (0.93, 0), (0.94, 0), 0.012, 0.012, follow=False, z=1.9, region="bike-metal")
    # handlebar: swept bars with grips
    bw = 0.05 if L >= 500 else 0.014
    gr = 0.05 if L >= 500 else 0.0175
    for s, sg in (("l", 1), ("r", -1)):
        c.cap("plan", f"handlebar-{s}", "frame", (0.925, sg * 0.03), (0.8, sg * (0.325 - gr)), bw, bw, follow=False, z=1.95, region="bike-metal")
        c.cap("plan", f"grip-{s}", "frame", (0.8, sg * (0.325 - gr)), (0.7 + gr, sg * (0.325 - gr)), gr, gr, follow=False, z=1.96, region="bike-metal")
    # saddle outline
    c.poly("plan", "saddle", "frame", [(0.13, 0.05), (0.15, 0.075), (0.2, 0.078), (0.28, 0.06), (0.39, 0.02), (0.39, -0.02), (0.28, -0.06), (0.2, -0.078), (0.15, -0.075), (0.13, -0.05)], z=1.9, region="saddle")
    if not L >= 500 and L <= 100:
        c.cap("plan", "fender-rear", "frame", (-0.292, 0), (0.337, 0), 0.033, 0.033, follow=False, z=1.7, region="bike-metal", role="detail", filled=False)
        c.cap("plan", "fender-front", "frame", (0.742, 0), (1.37, 0), 0.033, 0.033, follow=False, z=1.7, region="bike-metal", role="detail", filled=False)
    if mid_ or fine:
        c.cap("plan", "chainring", "crank-r", (0.369, -0.05), (0.531, -0.05), 0.005, 0.005, follow=False, z=2.2, region="bike-metal")
        c.cap("plan", "chain", "frame", (0.0, -0.05), (0.45, -0.05), 0.004, 0.004, follow=False, z=1.7, region="bike-metal", role="detail")
    if fine:
        c.ell("plan", "headlamp", "frame", (1.01, 0), 0.02, 0.02, z=1.95, region="bike-metal", role="detail")
        for nm, bone, hx in (("rear", "rear-wheel", 0.0), ("front", "front-wheel", 1.05)):
            c.cap("plan", f"axle-{nm}", bone, (hx, -0.07), (hx, 0.07), 0.006, 0.006, follow=False, z=1.6, region="bike-metal")
    # ---- front: wheels, fork, bars, cranks and pedals
    wr = 0.05 if L >= 500 else tw / 2
    for nm, bone, cy in (("rear", "rear-wheel", 0.34), ("front", "front-wheel", 0.34)):
        c.add("front", {"kind": "ellipse", "cx": 0.0, "cy": 0.0, "rx": 0.34, "ry": 0.34}, f"tyre-{nm}", bone, zt + (0.05 if nm == "front" else 0.0), region="tyres", solid={"kind": "extruded", "z0": -wr, "z1": wr})
    if L >= 500:
        c.rect("front", "fork-bars", "frame", -0.05, 0.35, 0.1, 0.55, follow=False, z=1.85, region="bike-metal")
    else:
        for s, sg in (("l", 1), ("r", -1)):
            c.rect("front", f"fork-{s}", "frame", sg * 0.05 - 0.008, 0.352, 0.016, 0.396, follow=False, z=1.8, region="bike-metal")
            limb2(c, f"crank-{s}", f"crank-{s}", sg * 0.088, sg * 0.088, [(0, 0.026 if L >= 200 else 0.0125), (1, 0.026 if L >= 200 else 0.0125)], cap0=0.026, cap1=0.026, z=2.3 if s == "r" else 0.9, region="bike-metal")
            c.extrude_like("front", f"pedal-{s}", f"pedal-{s}", f"pedal-{s}", (0.095 if sg > 0 else -0.19), (0.19 if sg > 0 else -0.095), z=2.35 if s == "r" else 0.95, region="bike-metal")
        c.rect("front", "head-tube", "frame", -0.0125, 0.738, 0.025, 0.159, follow=False, z=1.85, region="bike-metal")
        c.rect("front", "stem", "frame", -0.012, 0.861, 0.024, 0.153, follow=False, z=1.9, region="bike-metal")
        c.poly("front", "handlebar", "frame", [(-0.325, 1.05), (-0.31, 1.075), (0.0, 1.02), (0.31, 1.075), (0.325, 1.05), (0.31, 1.03), (0.0, 1.0), (-0.31, 1.03)], z=1.95, region="bike-metal")
        c.ell("front", "grip-l", "frame", (0.285, 1.052), 0.04, 0.0175, z=1.96, region="bike-metal")
        c.ell("front", "grip-r", "frame", (-0.285, 1.052), 0.04, 0.0175, z=1.96, region="bike-metal")
        c.poly("front", "saddle", "frame", [(-0.075, 0.905), (0.075, 0.905), (0.07, 0.955), (-0.07, 0.955)], z=1.9, region="saddle")
        c.cap("front", "fender-front", "frame", (0.0, 0.49), (0.0, 0.69), 0.035, 0.035, follow=False, z=1.7, region="bike-metal", role="detail" if mid_ or fine else "body", filled=False if (mid_ or fine) else True)
    if L >= 500:
        c.rect("front", "bars-saddle", "frame", -0.325, 0.99, 0.65, 0.1, follow=False, z=1.95, region="bike-metal")
        c.poly("front", "saddle", "frame", [(-0.075, 0.905), (0.075, 0.905), (0.07, 0.955), (-0.07, 0.955)], z=1.9, region="saddle")
    if mid_ or fine:
        c.ell("front", "headlamp", "frame", (0.0, 0.91), 0.02, 0.02, z=1.97, region="bike-metal", role="detail")
        c.poly("front", "chainring", "crank-r", [(-0.053, 0.219), (-0.053, 0.381)], closed=False, follow=False, z=2.2, region="bike-metal", role="detail")
    if fine:
        for nm, bone, hy in (("rear", "rear-wheel", 0.34), ("front", "front-wheel", 0.34)):
            c.ell("front", f"hub-{nm}", bone, (0.0, hy), 0.025, 0.025, z=1.62 + (0.05 if nm == "front" else 0.0), region="bike-metal", role="detail")
    c.poly("front", "ground", "frame", [(-0.45, 0.0), (0.45, 0.0)], closed=False, z=0.0, role="ground", filled=False)


def details(c, L):
    O, T = c.O, c.T
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    hc = T["head"][1] + CFG["headTopDy"] - CFG["skullRy"]
    hx = (O["head"][0] + T["head"][0]) / 2
    # head: skull skin under the helmet, helmet dome, pony tail
    for sh in c.plan:
        if sh["id"] == "head":
            c.retag("plan", "head", "face-skin")
    c.ell("plan", "helmet", "head", (hx + 0.0, 0), 0.1225, 0.092, z=6.5, region="helmet")
    if L < 500:
        c.cap("plan", "hair-tail", "head", (hx - 0.18, 0), (hx - 0.14, 0), 0.02, 0.02, follow=False, z=5.4, region="hair")
        c.poly("front", "helmet", "head", arc(0, 1.665, 0.101, 0.145, -8, 188, 16), z=6.6, region="helmet")
        c.poly("front", "hair-nape", "head", [(-0.045, 1.52), (0.045, 1.52), (0.05, 1.56), (-0.05, 1.56)], z=5.0, region="hair")
    else:
        c.poly("front", "helmet-head", "head", arc(0, 1.665, 0.101, 0.145, -8, 188, 16), z=6.6, regions=["helmet", "hair"])
    if mid_ or fine:
        for s, sg in (("l", 1), ("r", -1)):
            c.poly("front", f"helmet-strap-{s}", "head", [(sg * 0.055, 1.646), (sg * 0.045, 1.56)], closed=False, z=6.7, region="helmet", role="detail")
    if mid_ or fine:
        c.poly("front", "collar", "neck", [(-0.075, 1.45), (-0.06, 1.55), (0.06, 1.55), (0.075, 1.45)], z=3.9, region="jacket")
        c.poly("front", "jacket-zip", "torso", [(0.0, 0.955), (0.0, 1.473)], closed=False, follow=True, z=4.1, region="jacket", role="detail")
    if fine:
        c.rect("front", "jacket-reflective", "torso", -0.16, 1.192, 0.32, 0.094, follow=True, z=4.1, region="jacket", role="detail", filled=False)
        for i, dx in enumerate((-0.03, 0.0, 0.03)):
            c.poly("front", f"helmet-vent-{i}", "head", [(dx, 1.72), (dx, 1.775)], closed=False, z=6.7, region="helmet", role="detail")
        for s, sg in (("l", 1), ("r", -1)):
            c.poly("front", f"trouser-clip-{s}", f"shank-{s}", [(sg * 0.16 - 0.045, 0.25), (sg * 0.16 + 0.045, 0.25)], closed=False, follow=True, z=1.1, region="trousers", role="detail")
            c.poly("plan", f"shoulder-seam-{s}", "neck", [(O["neck"][0] + 0.05, sg * 0.1), (O["neck"][0] - 0.005, sg * 0.2)], closed=False, z=4.2, region="jacket", role="detail")
    bike(c, L)


def fix_solids(c, L):
    """QA4: the cranks (1:200) and the hands (1:500) keep a fore-aft thickness as extruded solids, so they do not collapse to a line in the front view when the bone turns horizontal; thin thighs keep the spread of the astride pose within the lateral limit."""
    for s_, sg in (("l", 1), ("r", -1)):
        if L == 200:
            c.drop("front", [f"crank-{s_}"])
            c.extrude_poly("front", f"crank-{s_}", f"crank-{s_}", [(-0.025, -0.025), (0.195, -0.025), (0.195, 0.025), (-0.025, 0.025)], sg * 0.088 - 0.04, sg * 0.088 + 0.04, z=2.3 if s_ == "r" else 0.9, region="bike-metal")
        if L == 500:
            c.drop("front", [f"hand-{s_}"])
            c.extrude_poly("front", f"hand-{s_}", f"hand-{s_}", [(-0.01, -0.06), (0.19, -0.06), (0.19, 0.06), (-0.01, 0.06)], sg * 0.28 - 0.06, sg * 0.28 + 0.06, z=4.6, region="gloves")


def build(scales=None):
    c = Ctx("cyclist")
    for L in (scales or c.scales):
        c.start(L)
        human(c, L, merge(CFG, dict(front=dict(thigh=[(0, 0.058), (0.45, 0.056), (1, 0.053)])) if L >= 500 else {}))
        details(c, L)
        fix_solids(c, L)
        en, de = texts(L, EXTRA)
        c.commit(L, en, de)
    c.save()


if __name__ == "__main__":
    build([int(x) for x in sys.argv[1:]] or None)
