import sys
from lib import Ctx
from human import human, limb2, arc
from texts import texts

LAT = dict(hip=0.095, knee=0.1, ankle=0.105, shoulder=0.16, elbow=0.19, wrist=0.21)
CFG = dict(
    arm="arm", neck=True,
    lat=LAT, armR=0.075,
    headDx=-0.012, headHalfD=0.105, headHalfW=0.08, faceDx=0.058, skullRy=0.113, skullRx=0.078, headTopDy=0.01, hairFront=False,
    plan=dict(footX=(-0.065, 0.14), toeX=(0.19, 0.21), toe=(0.047, 0.04), foot=(0.042, 0.048), thigh=(0.085, 0.065), shank=(0.062, 0.05), upper=(0.075, 0.065), fore=(0.062, 0.05), hand=(0.045, 0.032),
              hip=(0.125, 0.18), trunk=(0.125, 0.125), shoulder=(0.115, 0.23)),
    front=dict(
        torso=[(-0.4, 0.185), (-0.1, 0.18), (0.3, 0.165), (0.62, 0.18), (0.9, 0.2)],
        upper=[(0, 0.075), (0.5, 0.072), (1, 0.062)], fore=[(0, 0.062), (0.4, 0.062), (1, 0.05)], hand=[(0, 0.045), (0.5, 0.05), (1, 0.032)],
        thigh=[(0, 0.092), (0.45, 0.082), (1, 0.062)], shank=[(0, 0.06), (0.3, 0.062), (0.75, 0.048), (1, 0.045)], foot=[(0, 0.05), (0.6, 0.052), (1, 0.052)],
        shoeCap=0.1, toeCap=0.062, pelvisW=0.175),
    pelvisYb=0.88, pelvisYt=1.05,
    faceY=dict(eye=0.008, brow=0.03, nose=-0.025, mouth=-0.066),
    R=dict(thigh="trousers", shank="trousers", foot="boots", toe="boots", hip="trousers", torso="jacket", upper="jacket", fore="jacket", hand="gloves", neck="face-skin", face="face-skin", hair="cap"),
)
RAIL_Y = 0.21
EXTRA = {
    500: dict(
        merged=("The hand truck is two rails, the wheel pair and the parcel stack as three shapes; the three parcels are one stadium in plan.", "Die Sackkarre besteht aus zwei Holmen, dem Radpaar und dem Paketstapel als drei Formen; die drei Pakete sind im Grundriss ein Stadion."),
    ),
    200: dict(
        retained=("Rails, wheels, toe plate and the three parcels are separate shapes; the parcels keep their own footprints in plan and their own faces in front.", "Holme, Räder, Schaufel und die drei Pakete sind eigene Formen; die Pakete behalten im Grundriss ihre eigenen Grundflächen und vorn ihre eigenen Stirnflächen."),
    ),
    100: dict(
        retained=("Cross braces, axle stay, hubs, parcel tape and the reflective bands of the jacket are kept; the cap has crown, brim and button.", "Querstreben, Achsstütze, Naben, Paketband und die Reflexstreifen der Jacke bleiben erhalten; die Mütze hat Krone, Schirm und Knopf."),
    ),
    50: dict(
        retained=("Jacket zip, chest pocket, collar, cuff straps, cargo pockets, knee patches, trouser hems, boot laces, eyes, nose and mouth are drawn in front; the plan adds the shoulder seams and the cap button.", "Jackenreißverschluss, Brusttasche, Kragen, Ärmelriemen, Cargotaschen, Knieflicken, Hosensäume, Stiefelschnürung, Augen, Nase und Mund sind vorn gezeichnet; der Grundriss ergänzt die Schulternähte und den Mützenknopf."),
    ),
}


def details(c, L):
    O, T = c.O, c.T
    fine, mid_, coarse = L <= 50, L <= 100, L <= 200
    hc = T["head"][1] + CFG["headTopDy"] - CFG["skullRy"]
    hx = (O["head"][0] + T["head"][0]) / 2 + CFG["headDx"]
    rx = CFG["skullRx"]
    if L < 500:
        crown = [(-rx - 0.012, hc + 0.05)] + arc(0, hc + 0.05, rx + 0.012, 0.08, 180, 0, 14) + [(rx + 0.012, hc + 0.05)]
        c.poly("front", "cap", "head", crown, z=6.6, region="cap")
        c.ell("front", "cap-brim", "head", (0.0, hc + 0.05), rx + 0.016, 0.016, z=6.65, region="cap")
        c.cap("plan", "cap-brim", "head", (hx + 0.085, 0), (hx + 0.1, 0), 0.062, 0.05, follow=False, z=6.45, region="cap")
    rail_end = 0.15
    rr = 0.018 if L <= 200 else 0.05
    for s, sg in (("l", 1), ("r", -1)):
        c.cap("plan", f"rail-{s}", "hand-truck", (0.66, sg * RAIL_Y), (rail_end, sg * RAIL_Y), rr, rr, z=6.5, region="hand-truck")
        c.cap("plan", f"tyre-{s}", "truck-lift", (0.555, sg * 0.275), (0.765, sg * 0.275), 0.025 if L <= 200 else 0.05, 0.025 if L <= 200 else 0.05, follow=False, z=6.6, region="tyres")
        limb2(c, f"rail-{s}", "hand-truck", sg * RAIL_Y, sg * RAIL_Y, [(0, 0.016 if L <= 200 else 0.05), (1, 0.016 if L <= 200 else 0.05)], t0=-0.076, t1=0.9, cap1=0.016, z=6.5, region="hand-truck")
        c.cap("front", f"tyre-{s}", "truck-lift", (sg * 0.275, 0.025 if L <= 200 else 0.05), (sg * 0.275, 0.235 if L <= 200 else 0.21), 0.025 if L <= 200 else 0.05, 0.025 if L <= 200 else 0.05, follow=False, z=6.6, region="tyres")
        if L <= 100:
            c.rect("front", f"hub-{s}", "truck-lift", sg * 0.275 - 0.03, 0.1, 0.06, 0.06, z=6.65, region="tyres", role="detail")
    if L <= 200:
        c.rect("front", "handle", "hand-truck", -RAIL_Y - 0.02, 1.035, 2 * RAIL_Y + 0.04, 0.04, r=0.015, follow=True, z=6.5, region="hand-truck")
        c.rect("front", "plate", "hand-truck", -0.19, 0.16, 0.38, 0.04, z=6.9, follow=False, region="hand-truck")
    else:
        c.rect("front", "handle", "hand-truck", -RAIL_Y - 0.05, 1.01, 2 * RAIL_Y + 0.1, 0.1, follow=True, z=6.5, region="hand-truck")
    if L <= 100:
        c.rect("front", "axle-stay", "hand-truck", -0.28, 0.1, 0.56, 0.03, z=6.55, follow=False, region="hand-truck", role="detail")
        for i, y in enumerate((0.45, 0.8)):
            c.poly("front", f"brace-{i}", "hand-truck", [(-RAIL_Y, y), (RAIL_Y, y)], closed=False, follow=True, z=6.55, region="hand-truck", role="detail")
    boxes = (("a", 0.23, (0.193, 0.553), (0.506, 1.027)), ("b", 0.2, (0.553, 0.83), (0.394, 0.840)), ("c", 0.18, (0.83, 1.1), (0.303, 0.653)))
    if L >= 500:
        stack = [(0.06, -0.42), (0.98, -0.42), (0.98, -0.02), (0.06, -0.02)]
        c.extrude_poly("front", "parcel-stack", "parcels", stack, -0.23, 0.23, z=7.0, region="parcels")
        c.extrude_poly("plan", "parcel-stack", "parcels", stack, -0.23, 0.23, z=7.0, region="parcels")
    else:
        for k, (nm, hw, (y0, y1), (x0, x1)) in enumerate(boxes):
            c.extrude_like("front", f"parcel-{nm}", "parcels", f"parcel-{nm}", -hw, hw, z=7 + 0.05 * k, region="parcels")
            zz = 7.0 + 0.1 * k
            c.extrude_like("plan", f"parcel-{nm}", "parcels", f"parcel-{nm}", -hw, hw, z=zz, region="parcels")
            if mid_ or fine:
                t = c.elev(f"tape-{nm}")["points"]
                (lx0, ly), (lx1, _) = t
                c.extrude_poly("front", f"tape-{nm}", "parcels", [(lx0, ly - 0.006), (lx1, ly - 0.006), (lx1, ly + 0.006), (lx0, ly + 0.006)], -0.006, 0.006, z=7.1 + 0.05 * k, region="parcels", role="detail", tone=0.2)
                xm = 0.5 * (x0 + x1)
                corners = [c.L("parcels", xm + dx, y) for dx, y in ((-0.006, y0), (0.006, y0), (0.006, y1), (-0.006, y1))]
                c.extrude_poly("plan", f"tape-{nm}", "parcels", corners, -hw, hw, z=zz + 0.05, region="parcels", role="detail", tone=0.2)
    if fine:
        c.cap("plan", "cap-button", "head", (hx + 0.01, 0), (hx + 0.012, 0), 0.011, 0.011, follow=False, z=6.5, region="cap", role="detail")
        for s, sg in (("l", 1), ("r", -1)):
            c.poly("plan", f"shoulder-seam-{s}", "neck", [(O["neck"][0] + 0.05, sg * 0.1), (O["neck"][0] - 0.005, sg * 0.2)], closed=False, z=4.2, region="jacket", role="detail")
            c.poly("front", f"cargo-pocket-{s}", f"thigh-{s}", [(sg * 0.115, 0.817), (sg * 0.185, 0.817), (sg * 0.185, 0.697), (sg * 0.115, 0.697)], closed=False, follow=True, z=3.1, region="trousers", role="detail")
            c.poly("front", f"trouser-hem-{s}", f"shank-{s}", [(sg * 0.105 - 0.05, 0.114), (sg * 0.105 + 0.05, 0.114)], closed=False, follow=True, z=1.1, region="trousers", role="detail")
            c.ell("front", f"knee-patch-{s}", f"shank-{s}", (sg * 0.1, O[f"shank-{s}"][1] + 0.018), 0.05, 0.034, z=1.09, region="trousers", role="detail", filled=False)
            limb2(c, f"cuff-strap-{s}", f"forearm-{s}", sg * 0.2, sg * 0.21, [(0, 0.062), (1, 0.05)], t0=0.85, t1=1.0, wscale=1.08, z=4.58, region="jacket")
            for i, dx in enumerate((-0.03, 0.0, 0.03)):
                c.poly("front", f"boot-lace-{s}-{i}", f"foot-{s}", [(sg * 0.105 + dx, 0.127), (sg * 0.105 + dx, 0.07)], closed=False, z=1.3, region="boots", role="detail")
        c.poly("front", "jacket-zip", "torso", [(0.0, 0.89), (0.0, 1.49)], closed=False, follow=True, z=4.1, region="jacket", role="detail")
        c.poly("front", "chest-pocket", "torso", [(0.06, 1.42), (0.15, 1.42), (0.15, 1.34), (0.06, 1.34)], closed=False, follow=True, z=4.1, region="jacket", role="detail")
        c.poly("front", "collar-line", "torso", [(-0.085, 1.537), (0.0, 1.5), (0.085, 1.537)], closed=False, follow=True, z=4.1, region="jacket", role="detail")
        c.ell("front", "cap-button", "head", (0.0, hc + 0.05 + 0.08), 0.006, 0.006, z=6.7, region="cap", role="detail")
    if mid_ or fine:
        for i, (y0, y1) in enumerate(((1.086, 1.114), (1.136, 1.164))):
            c.rect("front", f"reflective-band-{i}", "torso", -0.17, y0, 0.34, y1 - y0, follow=True, z=4.12, region="jacket")


def build(scales=None):
    c = Ctx("courier-with-hand-truck")
    for L in (scales or c.scales):
        c.start(L)
        human(c, L, CFG)
        details(c, L)
        en, de = texts(L, EXTRA)
        c.commit(L, en, de)
    c.save()


if __name__ == "__main__":
    build([int(x) for x in sys.argv[1:]] or None)
