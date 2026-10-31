"""NW-H1 person-with-rollator: the four individually drawn representations (rest-world coordinates, converted per bone)."""
from rollator_phys import *
from figure import *
from docbuild import D

HD = h.hd


def line(sh, bone, pts):
    return sh.poly(bone, pts, False)


def W2(bone, lx, ly):
    return REST[bone].ap(lx, ly)
W = lambda bone, lx, ly: REST[bone].ap(lx, ly)


def seg(bone):
    return REST[bone].ap(0, 0), REST[bone].ap(rig.by[bone]["lengthM"], 0)


def down(bone, st, c0=0.0, c1=0.0):
    p0, p1 = seg(bone)
    return tube(p0, p1, [(t, front, back) for t, back, front in st], c0, c1)


def up(bone, st, c0=0.0, c1=0.0):
    p0, p1 = seg(bone)
    return tube(p0, p1, [(t, back, front) for t, back, front in st], c0, c1)


def frm(x, y):
    return (X_R0 + x, y)


def fpts(pts):
    return [(X_R0 + p[0], p[1]) + tuple(p[2:]) for p in pts]


def hp(u, v):
    return h.head_pt(u, v)


HAIR = [(-0.34, 0.60), (-0.37, 0.42), (-0.31, 0.22), (-0.19, 0.20), (-0.17, 0.30), (-0.22, 0.45), (-0.24, 0.62)]
BERET = [(-0.37, 0.60), (-0.41, 0.76), (-0.27, 0.97), (0.04, 1.04), (0.30, 0.97), (0.42, 0.84), (0.41, 0.69), (0.23, 0.655), (0.0, 0.64), (-0.2, 0.62)]
BERET_C = [(-0.38, 0.62), (-0.40, 0.80), (-0.20, 1.0), (0.10, 1.04), (0.40, 0.90), (0.40, 0.68), (0.1, 0.64)]

LOD50 = [{"id": "loden-twill", "title": tx("Loden twill", "Loden-Köper"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1},
         {"id": "basket-mesh", "title": tx("Basket mesh", "Korbgitter"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.2, "penMm": 0.1},
         {"id": "far-side", "title": tx("Far side", "Abgewandte Seite"), "pattern": "lines", "angleDeg": 60, "spacingMm": 0.6, "penMm": 0.1}]





def hand_shape(sh, side, z, scale):
    b = "hand-" + side
    p0 = REST[b].ap(0, 0)
    L = h.Lha
    p1 = REST[b].ap(L, 0)
    st = [(-0.1, .026, .026), (0.12, .03, .032), (0.4, .036, .04), (0.62, .036, .042), (0.84, .03, .038), (1.0, .018, .026)]
    return tube(p0, p1, st, 0.0, 0.5)


OUT = dict(
    reg=dict(skin="face-skin", hair="hair", hat="beret", hands="hands", upper="coat", lower="trousers", shoes="shoes"),
    tone=dict(skin=0.85, hair=0.93, hat=0.30, upper=0.38, upper_far=0.30, lower=0.33, lower_far=0.28, shoes=0.22, collar=0.33),
    torso=[(-0.12, .118, .108), (0.0, .108, .112), (0.25, .098, .106), (0.5, .104, .128), (0.78, .112, .122), (0.93, .098, .096), (1.0, .066, .064)],
    hip=[(-1.0, .116, .112), (0.0, .114, .110), (1.0, .110, .108), (1.3, .108, .106)],
    skirt=[(-0.1, .112, .105), (0.2, .117, .104), (0.5, .122, .11), (0.78, .13, .116)],
    sleeve_up=[(-0.10, .052, .05), (0.3, .050, .049), (0.7, .046, .046), (1.08, .042, .043)],
    sleeve_fore=[(-0.12, .044, .044), (0.3, .042, .043), (0.7, .040, .040), (0.93, .040, .040), (1.0, .040, .040)],
    thigh=[(0.60, .062, .056), (0.85, .056, .054), (1.06, .054, .05)],
    shank=[(-0.06, .056, .052), (0.15, .060, .054), (0.40, .058, .048), (0.75, .048, .040), (0.92, .044, .040), (0.97, .047, .043)],
    collar=[(-0.6, .066, .068), (0.1, .064, .070), (0.55, .056, .062), (0.7, .050, .056)],
    hat_pts=BERET, hat_c=BERET_C, hair_pts=HAIR, shoe="lace", hatch="loden-twill", far_hatch="far-side", glasses=True, nub=True, buttons=True, cuff=0.352,
    thigh500=[(0.0, .075, .07), (0.5, .065, .06), (1.05, .055, .05)], shank500=[(-0.05, .058, .054), (0.4, .055, .048), (1.0, .046, .04)],
    skirt500=[(-0.1, .112, .105), (0.5, .12, .11), (0.78, .13, .116)], torso500=[(-0.12, .118, .108), (0.5, .104, .128), (0.93, .098, .096), (1.0, .066, .064)], hip500=[(-1.0, .116, .112), (1.3, .108, .106)],
    pen_arm500=0.16, pen_hand500=0.14)


def build50(sh):
    dress(sh, h, rig, REST, OUT, 50)
    rollator50(sh)


def tube_caps(sh, id_, p0, p1, r, region, z, tone=0.7, bone="frame", role="body", pen=None):
    return sh.add(id_, bone, sh.capsule(bone, p0, p1, r, r), role, z, True, tone, region=region, pen=pen)


def rollator50(sh):
    A = sh.add
    f = frm
    tube_caps(sh, "rear-upright", f(0.02, 0.12), f(0.0, 0.84), 0.0125, "frame", 9.0, 0.78, pen=0.15)
    tube_caps(sh, "front-leg", f(0.52, 0.12), f(0.47, 0.50), 0.0125, "frame", 9.0, 0.78, pen=0.15)
    tube_caps(sh, "seat-rail", f(0.02, 0.49), f(0.47, 0.49), 0.0125, "frame", 9.0, 0.78, pen=0.15)
    tube_caps(sh, "lower-rail", f(0.015, 0.26), f(0.50, 0.24), 0.0125, "frame", 9.0, 0.78, pen=0.15)
    tube_caps(sh, "front-brace", f(0.47, 0.50), f(0.30, 0.30), 0.0100, "frame", 9.0, 0.78, pen=0.15)
    tube_caps(sh, "grip", f(0.0, 0.85), f(-0.14, 0.85), 0.017, "grips", 9.6, 0.15, pen=0.15)
    tube_caps(sh, "brake-lever", f(-0.02, 0.832), f(0.07, 0.805), 0.007, "frame", 9.5, 0.6, pen=0.1)
    A("brake-cable", "frame", line(sh, "frame", [f(0.06, 0.805), f(0.045, 0.45), f(0.03, 0.14)]), "detail", 9.4, False, 0.5, region="frame")
    A("seat", "frame", sh.path("frame", fpts([(0.04, 0.50, 'c'), (0.47, 0.50, 'c'), (0.475, 0.523), (0.455, 0.540, 'c'), (0.07, 0.540, 'c'), (0.035, 0.523)])), "body", 3.0, True, 0.35, region="seat-basket")
    A("basket", "frame", sh.path("frame", fpts([(0.10, 0.475, 'c'), (0.44, 0.475, 'c'), (0.42, 0.275), (0.12, 0.275)])), "body", 2.9, True, 0.6, region="seat-basket", hatch="basket-mesh")
    A("backrest", "frame", sh.path("frame", fpts([(-0.012, 0.58, 'c'), (0.036, 0.58, 'c'), (0.036, 0.73, 'c'), (-0.012, 0.73, 'c')])), "body", 8.9, True, 0.3, region="seat-basket")
    A("bag", "frame", sh.path("frame", fpts([(0.15, 0.40, 'c'), (0.40, 0.40, 'c'), (0.415, 0.55), (0.28, 0.585), (0.14, 0.55)])), "body", 3.5, True, 0.78, region="bag")
    A("bag-handle", "frame", sh.path("frame", fpts([(0.20, 0.57), (0.255, 0.68), (0.32, 0.57)]), False), "detail", 3.6, False, 0.5, region="bag")
    for nm, bone, cx in (("rear", "wheel-rear", 0.0), ("front", "wheel-front", WB)):
        c = frm(cx, R_W)
        A("tyre-" + nm, bone, sh.ellipse(bone, c, 0.100, 0.100), "body", 9.2, True, 0.12, region="tyres")
        A("rim-" + nm, bone, sh.ellipse(bone, c, 0.066, 0.066), "body", 9.3, True, 0.8, region="frame")
        A("hub-" + nm, bone, sh.ellipse(bone, c, 0.016, 0.016), "detail", 9.4, True, 0.3, region="frame")
        for k in range(5):
            a = math.radians(90 + 72 * k)
            A("spoke-%s%d" % (nm, k), bone, line(sh, bone, [(c[0] + 0.016 * math.cos(a), c[1] + 0.016 * math.sin(a)), (c[0] + 0.062 * math.cos(a), c[1] + 0.062 * math.sin(a))]), "detail", 9.45, False, 0.4, region="frame")
    A("caster-fork", "frame", sh.path("frame", fpts([(0.505, 0.30, 'c'), (0.535, 0.30, 'c'), (0.54, 0.12), (0.50, 0.12)])), "body", 9.1, True, 0.7, region="frame")


def ground_line(sh, x0=-0.5, x1=1.5, bone="pelvis"):
    return None


def stroke(sh, id_, pts, region, z, pen, bone="frame", tone=0.5, role="body"):
    return sh.add(id_, bone, sh.poly(bone, [frm(*p) for p in pts], False), role, z, False, tone, region=region, pen=pen)


def wheels(sh, level):
    A = sh.add
    for nm, bone, cx in (("rear", "wheel-rear", 0.0), ("front", "wheel-front", WB)):
        c = frm(cx, R_W)
        A("tyre-" + nm, bone, sh.ellipse(bone, c, 0.100, 0.100), "body", 9.2, True, 0.12, region="tyres")
        if level >= 100 and level <= 100:
            A("rim-" + nm, bone, sh.ellipse(bone, c, 0.062, 0.062), "body", 9.3, True, 0.8, region="frame")
            A("hub-" + nm, bone, sh.ellipse(bone, c, 0.02, 0.02), "detail", 9.4, True, 0.3, region="frame")
            A("spoke-" + nm, bone, sh.poly(bone, [c, (c[0] + 0.062, c[1])], False), "detail", 9.45, False, 0.4, region="frame")


def rollator100(sh):
    A = sh.add
    f = frm
    tube_caps(sh, "rear-upright", f(0.02, 0.12), f(0.0, 0.84), 0.0125, "frame", 9.0, 0.78, pen=0.1)
    tube_caps(sh, "front-leg", f(0.52, 0.12), f(0.47, 0.50), 0.0125, "frame", 9.0, 0.78, pen=0.1)
    tube_caps(sh, "seat-rail", f(0.02, 0.49), f(0.47, 0.49), 0.0125, "frame", 9.0, 0.78, pen=0.1)
    tube_caps(sh, "lower-rail", f(0.015, 0.26), f(0.50, 0.24), 0.0125, "frame", 9.0, 0.78, pen=0.1)
    tube_caps(sh, "grip", f(0.0, 0.85), f(-0.14, 0.85), 0.017, "grips", 9.6, 0.15, pen=0.1)
    stroke(sh, "brake-lever", [(-0.02, 0.832), (0.07, 0.805)], "frame", 9.5, 0.15, role="detail")
    A("seat", "frame", sh.path("frame", fpts([(0.04, 0.50, 'c'), (0.47, 0.50, 'c'), (0.475, 0.523), (0.455, 0.540, 'c'), (0.07, 0.540, 'c'), (0.035, 0.523)])), "body", 3.0, True, 0.35, region="seat-basket")
    A("basket", "frame", sh.path("frame", fpts([(0.10, 0.50, 'c'), (0.44, 0.50, 'c'), (0.42, 0.275), (0.12, 0.275)])), "body", 2.9, True, 0.62, region="seat-basket", hatch="basket-mesh")
    A("backrest", "frame", sh.path("frame", fpts([(-0.012, 0.58, 'c'), (0.036, 0.58, 'c'), (0.036, 0.73, 'c'), (-0.012, 0.73, 'c')])), "body", 8.9, True, 0.3, region="seat-basket")
    A("bag", "frame", sh.path("frame", fpts([(0.15, 0.40, 'c'), (0.40, 0.40, 'c'), (0.415, 0.55), (0.28, 0.585), (0.14, 0.55)])), "body", 3.5, True, 0.78, region="bag")
    wheels(sh, 100)
    A("caster-fork", "frame", sh.path("frame", fpts([(0.505, 0.30, 'c'), (0.535, 0.30, 'c'), (0.54, 0.12), (0.50, 0.12)])), "body", 9.1, True, 0.7, region="frame")


def rollator200(sh):
    A = sh.add
    stroke(sh, "rear-upright", [(0.02, 0.12), (0.0, 0.84)], "frame", 9.0, 0.13)
    stroke(sh, "front-leg", [(0.52, 0.12), (0.47, 0.50)], "frame", 9.0, 0.13)
    stroke(sh, "lower-rail", [(0.015, 0.26), (0.50, 0.24)], "frame", 9.0, 0.13)
    stroke(sh, "grip", [(0.0, 0.85), (-0.14, 0.85)], "grips", 9.6, 0.17)
    stroke(sh, "seat", [(0.04, 0.52), (0.47, 0.52)], "seat-basket", 9.1, 0.2)
    A("basket", "frame", sh.path("frame", fpts([(0.10, 0.52, 'c'), (0.44, 0.52, 'c'), (0.42, 0.27, 'c'), (0.12, 0.27, 'c')])), "body", 2.9, True, 0.62, region="seat-basket")
    A("bag", "frame", sh.path("frame", fpts([(0.15, 0.45, 'c'), (0.40, 0.45, 'c'), (0.41, 0.58), (0.28, 0.60), (0.14, 0.58)])), "body", 3.5, True, 0.8, region="bag")
    wheels(sh, 200)


def rollator500(sh):
    A = sh.add
    stroke(sh, "upright", [(0.02, R_W), (0.0, 0.85)], "frame", 9.0, 0.10)
    stroke(sh, "front-leg", [(0.52, R_W), (0.47, 0.50)], "frame", 9.0, 0.10)
    stroke(sh, "seat", [(0.0, 0.50), (0.49, 0.50)], "seat-basket", 9.1, 0.12)
    stroke(sh, "grip", [(0.0, 0.85), (-0.14, 0.85)], "grips", 9.6, 0.13)
    A("basket", "frame", sh.path("frame", fpts([(0.08, 0.50, 'c'), (0.44, 0.50, 'c'), (0.40, 0.27, 'c'), (0.12, 0.27, 'c')])), "body", 2.9, True, 0.62, regions=["seat-basket", "bag"])
    wheels(sh, 500)


def build100(sh):
    dress(sh, h, rig, REST, OUT100, 100)
    rollator100(sh)


def build200(sh):
    dress(sh, h, rig, REST, OUT200, 200)
    rollator200(sh)


def build500(sh):
    dress(sh, h, rig, REST, OUT500, 500)
    rollator500(sh)


import copy
OUT100 = copy.deepcopy(OUT)
OUT100.update(hair_pts=HAIR, hat_c=BERET_C, buttons=False, glasses=False, nub=False, cuff=None)
OUT200 = copy.deepcopy(OUT)
OUT200.update(thigh=[(0.45, .066, .060), (0.85, .056, .054), (1.06, .054, .05)])
OUT500 = copy.deepcopy(OUT)


def reps():
    out = {}
    for k, fn, rng, pens, hat in (("500", build500, (350, 1000), {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.13}, [])
                                  , ("200", build200, (150, 350), {"body": 0.13, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.18}, [])
                                  , ("100", build100, (75, 150), {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.25}, [dict(LOD50[0], spacingMm=0.6), dict(LOD50[1], spacingMm=0.8), dict(LOD50[2], spacingMm=0.5)])
                                  , ("50", build50, (20, 75), {"body": 0.25, "detail": 0.15, "texture": 0.1, "glazing": None, "ground": 0.25}, LOD50)):
        sh = Shapes(rig)
        fn(sh)
        out[k] = dict(range=rng, pens=pens, hatches=hat, shapes=sh.items, decisions=DECS[k])
    return out


DEC50 = D(
    ("Full articulation: head profile with glasses, ear and beret, loden coat with collar, front line, buttons and sleeve cuffs, mitten hands with thumb, trousers per segment, laced shoes in two parts at the ball joint, and the rollator as real tube members, spoked wheels, grip, brake lever with cable, seat, basket, backrest strap and shopping bag.",
     "Volle Gliederung: Kopfprofil mit Brille, Ohr und Baskenmütze, Lodenmantel mit Kragen, Frontlinie, Knöpfen und Ärmelaufschlägen, Fäustlingshände mit Daumen, Hose je Segment, Schnürschuhe zweiteilig am Ballengelenk sowie der Rollator mit echten Rohrprofilen, Speichenrädern, Griff, Bremshebel mit Zug, Sitz, Korb, Rückenband und Einkaufstasche."),
    ("Both thumbs, cuff and sole lines, lace marks, brake cable, wheel spokes, bag handle.", "Beide Daumen, Aufschlag- und Sohlenlinien, Schnürungsmarken, Bremszug, Radspeichen, Taschenhenkel."),
    ("Hair fringe and beret share the head bone; the far-side rollator frame is hidden behind the near side and not drawn.", "Haarrand und Baskenmütze liegen auf dem Kopfknochen; der abgewandte Rollatorrahmen verschwindet hinter der nahen Seite und wird nicht gezeichnet."),
    ("Eyes, fingers separations, coat stitching, tyre tread, frame welds, wheel bearings.", "Augen, Fingertrennungen, Mantelnähte, Reifenprofil, Rahmenschweißnähte, Radlager."),
    ("Frame tubes are 25 mm real and drawn as 0.5 mm capsules: no thickening. Brake lever 14 mm.", "Rahmenrohre sind real 25 mm und als 0,5-mm-Kapseln gezeichnet: keine Verstärkung. Bremshebel 14 mm."),
    ("Loden twill 0.8 mm lines at 45 degrees on the coat, basket mesh 1.2 mm cross hatch, far-side limbs 0.6 mm lines.", "Loden-Köper 0,8-mm-Linien unter 45 Grad auf dem Mantel, Korbgitter 1,2-mm-Kreuzschraffur, abgewandte Glieder 0,6-mm-Linien."),
    ("Body pen 0.25 mm, detail 0.15 mm, texture 0.10 mm; dark tyres, light rims, mid-tone coat.", "Körperstift 0,25 mm, Detail 0,15 mm, Textur 0,10 mm; dunkle Reifen, helle Felgen, mittlerer Mantelton."),
    ("Every thermal region is drawn separately: face and neck, hair, beret, hands, coat, trousers, shoes, frame, seat and basket, grips, tyres and bag.", "Jede thermische Region ist einzeln gezeichnet: Gesicht und Hals, Haar, Baskenmütze, Hände, Mantel, Hose, Schuhe, Rahmen, Sitz und Korb, Griffe, Reifen und Tasche."))


DECS = {"50": DEC50}
DECS["100"] = D(
    ("Articulated clothing outlines: beret, hair, coarse face profile with nose and chin, coat in torso, hip, skirt and sleeve pieces, mitten hands with thumb, trousers per segment, shoes in two parts at the ball joint, and the rollator with 25 mm tube capsules, wheels with rim and hub, seat, basket and bag.",
     "Gegliederte Kleidungskonturen: Baskenmütze, Haar, grobes Gesichtsprofil mit Nase und Kinn, Mantel in Rumpf-, Hüft-, Schoß- und Ärmelteilen, Fäustlingshände mit Daumen, Hose je Segment, Schuhe zweiteilig am Ballengelenk sowie der Rollator mit 25-mm-Rohrkapseln, Rädern mit Felge und Nabe, Sitz, Korb und Tasche."),
    ("Ear, both thumbs, brake lever, backrest strap, seat pad.", "Ohr, beide Daumen, Bremshebel, Rückenband, Sitzpolster."),
    ("Neck skin joins the collar piece of the coat; the caster fork is one piece with the front tube end.", "Der Hals geht in das Kragenstück des Mantels über; die Lenkgabel bildet mit dem Rohrende ein Stück."),
    ("Glasses, eye, mouth, buttons, cuff and lace lines, spokes, brake cable, bag handle, far rollator side.", "Brille, Auge, Mund, Knöpfe, Aufschlag- und Schnürlinien, Speichen, Bremszug, Taschenhenkel, abgewandte Rollatorseite."),
    ("Tubes are 25 mm real and drawn as 0.25 mm capsules: exact, no thickening; the 14 mm brake lever is a 0.13 mm line, thickened to the detail pen.", "Rohre sind real 25 mm und als 0,25-mm-Kapseln gezeichnet: exakt, keine Verstärkung; der 14-mm-Bremshebel ist eine 0,13-mm-Linie, auf den Detailstift verstärkt."),
    ("Coarse twill 0.6 mm lines on the coat, basket 0.8 mm cross hatch, far limbs 0.5 mm lines.", "Grober Köper aus 0,6-mm-Linien auf dem Mantel, Korb 0,8-mm-Kreuzschraffur, abgewandte Glieder 0,5-mm-Linien."),
    ("Body pen 0.25 mm, detail 0.13 mm, texture 0.10 mm.", "Körperstift 0,25 mm, Detail 0,13 mm, Textur 0,10 mm."),
    ("All twelve regions are drawn; face and neck skin are separate from hair and beret.", "Alle zwölf Regionen sind gezeichnet; Gesichts- und Halshaut sind von Haar und Baskenmütze getrennt."))
DECS["200"] = D(
    ("Main masses: head with beret and hair as one mass, neck-and-coat collar piece, coat torso and hip, one skirt piece and one trouser piece per leg, one sleeve piece per upper arm and forearm, small mitten hands, one shoe mass per foot; the rollator as 0.13 mm tube lines, two wheel discs, a seat line, basket and bag masses.",
     "Hauptmassen: Kopf mit Baskenmütze und Haar als eine Masse, Hals mit Mantelkragen, Manteltorso und Hüfte, ein Schoßteil und ein Hosenteil je Bein, ein Ärmelstück je Ober- und Unterarm, kleine Fäustlingshände, eine Schuhmasse je Fuß; der Rollator als 0,13-mm-Rohrlinien, zwei Radscheiben, eine Sitzlinie, Korb- und Taschenmasse."),
    ("Nose and chin of the face profile, one internal edge per sleeve through the elbow overlap.", "Nase und Kinn des Gesichtsprofils, eine innere Kante je Ärmel durch die Ellbogenüberlappung."),
    ("Hair and beret, neck and collar, shoe parts, rim and tyre, backrest and seat pad.", "Haar und Baskenmütze, Hals und Kragen, Schuhteile, Felge und Reifen, Rückenband und Sitzpolster."),
    ("Ear, thumbs, glasses, buttons, laces, spokes, hubs, brake lever and cable, caster fork.", "Ohr, Daumen, Brille, Knöpfe, Schnürung, Speichen, Naben, Bremshebel und Zug, Lenkgabel."),
    ("The 25 mm tubes (0.125 mm) are drawn with the 0.13 mm pen: within 5 % of true width; grip and seat lines are thickened to 0.17 and 0.2 mm (34 and 40 mm) to stay printable.", "Die 25-mm-Rohre (0,125 mm) sind mit dem 0,13-mm-Stift gezeichnet: innerhalb von 5 % der wahren Breite; Griff- und Sitzlinie sind auf 0,17 und 0,2 mm (34 und 40 mm) verstärkt, um druckbar zu bleiben."),
    ("No hatching; tone only (dark coat, darker tyres).", "Keine Schraffur; nur Ton (dunkler Mantel, dunklere Reifen)."),
    ("Body pen 0.13 mm, detail 0.10 mm, ground 0.18 mm; solid tones.", "Körperstift 0,13 mm, Detail 0,10 mm, Boden 0,18 mm; Flächentöne."),
    ("Merged shapes list their regions: head (face, hair) and neck (coat, face skin); frame, grips, seat and bag keep their own strokes and masses.", "Zusammengelegte Formen nennen ihre Regionen: Kopf (Gesicht, Haar) und Hals (Mantel, Gesichtshaut); Rahmen, Griffe, Sitz und Tasche behalten eigene Linien und Massen."))
DECS["500"] = D(
    ("One composed figure of five filled masses (head, coat torso, hip and two skirt pieces), overlapping leg masses and a single near arm drawn as a stroke; the rollator as three tube strokes, a seat stroke, one basket mass and two wheel discs. Height and length are the real 1.6 m and 1.1 m.",
     "Eine zusammengesetzte Figur aus gefüllten Massen (Kopf, Manteltorso, Hüfte und zwei Schoßteile), überlappenden Beinmassen und einem einzelnen nahen Arm als Linie; der Rollator als drei Rohrlinien, eine Sitzlinie, eine Korbmasse und zwei Radscheiben. Höhe und Länge sind die realen 1,6 m und 1,1 m."),
    ("Nose of the face profile, the stooped posture and the hand on the grip.", "Nase des Gesichtsprofils, die gebeugte Haltung und die Hand am Griff."),
    ("Face, hair and neck are one head mass; beret is its cap; basket and shopping bag are one mass; thigh, shank and shoe of one leg are bound per bone but overlap.", "Gesicht, Haar und Hals sind eine Kopfmasse; die Baskenmütze ist ihre Kappe; Korb und Einkaufstasche sind eine Masse; Ober-, Unterschenkel und Schuh eines Beins sind je Knochen gebunden, überlappen aber."),
    ("Far arm (hidden behind the near arm), thumbs, laces, buttons, spokes, hubs, brake lever and cable, backrest, rims, far rollator side.", "Abgewandter Arm (verdeckt vom nahen Arm), Daumen, Schnürung, Knöpfe, Speichen, Naben, Bremshebel und Zug, Rückenband, Felgen, abgewandte Rollatorseite."),
    ("Tubes (25 mm real) are drawn at the 0.10 mm minimum stroke (50 mm): twice the true width, kept because dropping the aid would remove the identity; the arm is a 0.16 mm stroke (80 mm), equal to the sleeve width.", "Rohre (real 25 mm) sind mit dem Mindeststrich 0,10 mm (50 mm) gezeichnet: doppelte wahre Breite, beibehalten, weil ohne das Hilfsmittel die Identität verloren ginge; der Arm ist ein 0,16-mm-Strich (80 mm), gleich der Ärmelbreite."),
    ("No hatching at 1:500.", "Keine Schraffur bei 1:500."),
    ("Body pen 0.13 mm; tones: dark coat, trousers and tyres; light head and basket.", "Körperstift 0,13 mm; Töne: dunkler Mantel, Hose und Reifen; heller Kopf und Korb."),
    ("Merged shapes list all regions they stand for: the head mass lists face-skin and hair, the basket mass seat-basket and bag; hands, grips, tyres and frame have own strokes or discs.", "Zusammengelegte Formen nennen alle Regionen, für die sie stehen: die Kopfmasse Gesichtshaut und Haar, die Korbmasse Sitz-Korb und Tasche; Hände, Griffe, Reifen und Rahmen haben eigene Linien oder Scheiben."))
