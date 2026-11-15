"""Stepladder (author 42): python3 author-42-stepladder.py [50 100 200 500]. Every part is placed from the drawings of its own scale."""
import os, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-42-lib.py")).read())

FF, RF = "front-frame", "rear-frame"
AL, ZN, RB = "alu-mill", "steel-zinc", "rubber-black"
SIDES = (1, -1)
SN = {1: "l", -1: "r"}

FRAIL = {"p0": (-0.36, 0.03, 0.2075), "p1": (-0.12, 1.4952, 0.1325), "w1": 0.06, "w2": 0.045}
RRAIL = {"p0": (0.225, 0.04, 0.1775), "p1": (-0.135, 1.235, 0.1225), "w1": 0.05, "w2": 0.045}


def side(p, s):
    return (p[0], p[1], p[2] * s)


def poly_ext(id, bone, mat, tone, pts, hw, role="body"):
    return ext(id, bone, mat, tone, "xy", pts, -hw, hw, role=role)


TREAD_X = [-0.41, -0.37, -0.33, -0.29]
TREAD_W = [0.3547, 0.3297, 0.3047, 0.2797]
TREAD_Y = [0.238, 0.488, 0.738, 0.988]


def tread_profile(i):
    x0, y0 = TREAD_X[i], TREAD_Y[i]
    return [[x0, y0], [x0 + 0.12, y0], [x0 + 0.12, y0 + 0.03], [x0 + 0.01, y0 + 0.03], [x0 + 0.004, y0 + 0.028], [x0 + 0.0, y0 + 0.021], [x0 - 0.002, y0 + 0.012]]


PLAT = [[-0.23, 1.225], [0.04, 1.225], [0.04, 1.265], [-0.215, 1.265], [-0.223, 1.262], [-0.23, 1.246], [-0.231, 1.236]]
CAP = [[-0.15, 1.44], [-0.09, 1.44], [-0.09, 1.49], [-0.093, 1.494], [-0.099, 1.497], [-0.109, 1.499], [-0.12, 1.5], [-0.131, 1.499], [-0.141, 1.497], [-0.147, 1.494], [-0.15, 1.49]]


def foot(id, bone, cx, cy, cz, rx0, rz0, rx1, rz1, h, off=(0, 0), e=6):
    return loft(id, bone, RB, 0.9, [sec([cx - off[0], cy - off[1], cz], rx0, rz0, e), sec([cx - off[0], cy - off[1] + h, cz], rx1, rz1, e)])


def spreader_end(s, link):
    return {"front": ((-0.261, 0.62, 0.18 * s), (-0.062, 0.55, 0.18 * s)), "rear": ((-0.119, 0.57, 0.15 * s), (0.08, 0.5, 0.15 * s))}[link]



def hollow(w1, w2, t):
    return (w1 - 2 * t, w2 - 2 * t)


# per scale: rails (centre line ends, elevation x plan section), chamfer, treads, platform, spreaders, feet, flags
FR50 = dict(p0=(-0.36, 0.03, 0.2075), p1=(-0.12, 1.4952, 0.1325))
RR50 = dict(p0=(0.225, 0.04, 0.1775), p1=(-0.135, 1.235, 0.1225))
P = {
    50: dict(fr=FR50, frw=(0.06, 0.045), rr=RR50, rrw=(0.05, 0.045), c=0.004, ty=TREAD_Y, tt=0.03, tw=TREAD_W, tm=0.0, plat=(-0.23, 1.225, 0.27, 0.04, 0.165), cap=True, sp=0.012, knuckle=True, hinge=True, braces=True, foot=(0.045, 0.0295, 0.04, 0.025, 0.048), rfoot=(0.045, 0.0275, 0.04, 0.0225, 0.045), nose=True, boxfeet=None),
    100: dict(fr=FR50, frw=(0.06, 0.045), rr=RR50, rrw=(0.05, 0.045), c=0.0, ty=TREAD_Y, tt=0.03, tw=TREAD_W, tm=0.0, plat=(-0.23, 1.225, 0.27, 0.04, 0.165), cap=False, sp=0.015, knuckle=False, hinge=True, braces=True, foot=(0.0475, 0.03, 0.0425, 0.025, 0.05), rfoot=(0.045, 0.0275, 0.04, 0.0225, 0.05), nose=False, boxfeet=None),
    200: dict(fr=dict(p0=(-0.36, 0.01, 0.2), p1=(-0.12, 1.4952, 0.1275)), frw=(0.06, 0.06), rr=dict(p0=(0.225, 0.03, 0.175), p1=(-0.135, 1.235, 0.1175)), rrw=(0.1, 0.06), c=0.0, ty=[0.25, 0.5, 0.75, 1.0], tt=0.036, tw=[0.316, 0.292, 0.266, 0.242], tm=0.0, plat=(-0.225, 1.205, 0.265, 0.06, 0.165), cap=False, sp=0.02, spr=((-0.26, 0.62), (0.087, 0.5), 0.18), knuckle=False, hinge=False, braces=False, foot=None, rfoot=None, nose=False, boxfeet=((0.09, 0.075, 0.075, -0.365, 0.1975), (0.09, 0.06, 0.06, 0.225, 0.18))),
    500: dict(fr=dict(p0=(-0.3465, 0.05, 0.19), p1=(-0.12, 1.4952, 0.13)), frw=(0.134, 0.085), rr=dict(p0=(0.185, 0.05, 0.165), p1=(-0.13, 1.2, 0.12)), rrw=(0.16, 0.085), c=0.0, ty=[0.25, 0.5, 0.75, 1.0], tt=0.0785, tw=[0.235, 0.21, 0.185, 0.16], tm=0.0, plat=(-0.225, 1.2108, 0.265, 0.0785, 0.165), cap=False, sp=None, knuckle=False, hinge=False, braces=False, foot=None, rfoot=None, nose=False, boxfeet=((0.08, 0.075, 0.075, -0.36, 0.1975), (0.08, 0.075, 0.075, 0.23, 0.165))),
}


def build(den):
    q = P[den]
    sc = Scale(den)
    b = sc.b
    for s in SIDES:
        n = SN[s]
        b(tube(f"front-rail-{n}", FF, AL, 0.5, side(q["fr"]["p0"], s), side(q["fr"]["p1"], s), q["frw"][0], q["frw"][1], c=q["c"]))
        b(tube(f"rear-rail-{n}", RF, AL, 0.5, side(q["rr"]["p0"], s), side(q["rr"]["p1"], s), q["rrw"][0], q["rrw"][1], c=q["c"], off=REAR))
    for i in range(4):
        prof = tread_profile(i) if q["nose"] else rect2(TREAD_X[i] - q["tm"], q["ty"][i] - q["tt"] / 2 if den >= 200 else q["ty"][i], 0.12 + 2 * q["tm"], q["tt"])
        b(poly_ext(f"tread-{i}", FF, AL, 0.55, prof, q["tw"][i] / 2))
    px, py, pw, pt, phw = q["plat"]
    b(poly_ext("platform", FF, AL, 0.4, PLAT if q["nose"] else rect2(px - q["tm"], py, pw + 2 * q["tm"], pt), phw))
    if q["cap"]:
        b(poly_ext("top-cap", FF, AL, 0.2, CAP, 0.1))
    for s in SIDES:
        n = SN[s]
        if q["sp"] and den >= 200:
            (x0, y0), (x1, y1), z = q["spr"]
            b(rod(f"spreader-{n}", FF, ZN, 0.6, (x0, y0, z * s), (x1, y1, z * s), q["sp"]))
        elif q["sp"]:
            p0, p1 = spreader_end(s, "front")
            b(rod(f"spreader-front-{n}", FF, ZN, 0.6, p0, p1, q["sp"]))
            p0, p1 = spreader_end(s, "rear")
            b(rod(f"spreader-rear-{n}", RF, ZN, 0.6, p0, p1, q["sp"], off=REAR))
        if q["knuckle"]:
            b(cyl(f"spreader-knuckle-{n}", FF, ZN, 0.7, "z", (-0.09, 0.56, 0.165 * s), 0.062, 0.016))
        if q["hinge"]:
            b(cyl(f"hinge-{n}", RF, AL, 0.7, "z", (0.0, 0.0, 0.15 * s), 0.05, 0.026))
    if q["braces"]:
        b(capsule("rear-brace-low", RF, AL, 0.6, loc((0.162, 0.22, -0.17), REAR), loc((0.162, 0.22, 0.17), REAR), 0.012))
        b(capsule("rear-brace-high", RF, AL, 0.6, loc((-0.025, 0.85, -0.14), REAR), loc((-0.025, 0.85, 0.14), REAR), 0.012))
    for s in SIDES:
        n = SN[s]
        if q["boxfeet"]:
            (w, h, d, x, z), (w2, h2, d2, x2, z2) = q["boxfeet"]
            b(box(f"foot-front-{n}", FF, RB, 0.9, [w + 2 * q["tm"], h, d], [x, h / 2, z * s]))
            b(box(f"foot-rear-{n}", RF, RB, 0.9, [w2 + 2 * q["tm"], h2, d2], [x2 - REAR[0], h2 / 2 - REAR[1], z2 * s]))
        else:
            f = q["foot"]
            b(foot(f"foot-front-{n}", FF, -0.36, 0.0, 0.205 * s, f[0], f[1], f[2], f[3], f[4]))
            f = q["rfoot"]
            b(foot(f"foot-rear-{n}", RF, 0.225, 0.0, 0.1775 * s, f[0], f[1], f[2], f[3], f[4], off=REAR))
    return sc


def rect2(x0, y0, w, h):
    return [[x0, y0], [x0 + w, y0], [x0 + w, y0 + h], [x0, y0 + h]]


REGION = {"front-rail": "front-rails", "top-cap": "front-rails", "tread": "treads", "platform": "platform", "rear-rail": "rear-frame", "rear-brace": "rear-frame", "hinge": "rear-frame", "spreader": "spreaders", "foot": "feet"}


def region_of(id):
    for k, v in REGION.items():
        if id.startswith(k):
            return v


def thermal_refs(sc, den=0):
    for d in sc.body:
        sc.thof("th-" + d["id"], d["id"], region_of(d["id"]))
    if den == 500:
        for s in SIDES:
            sc.th(rod(f"th-spreader-{SN[s]}", FF, ZN, 0.6, (-0.26, 0.62, 0.18 * s), (0.087, 0.5, 0.18 * s), 0.0375, n=8), "spreaders")


def structure(sc, q, den):
    t1, t2 = 0.0015, 0.0012
    ins = 0.0015
    for s in SIDES:
        n = SN[s]
        for bone, off, (w1, w2), t, nm, pp in ((FF, (0, 0), q["frw"], t1, "front", q["fr"]), (RF, REAR, q["rrw"], t2, "rear", q["rr"])):
            p0, p1 = side(pp["p0"], s), side(pp["p1"], s)
            a = unit(sub(p1, p0))
            q0, q1 = [p0[i] + a[i] * ins for i in range(3)], [p1[i] - a[i] * ins for i in range(3)]
            m1, m2 = w1 - 2 * ins, w2 - 2 * ins
            sc.st(tube(f"member-{nm}-rail-{n}", bone, AL, 0.5, q0, q1, m1, m2, c=q["c"], hole=hollow(m1, m2, t), off=off), "front-frame" if nm == "front" else "rear-frame", "member")
    px, py, pw, pt, phw = q["plat"]
    for i in range(4):
        x0 = TREAD_X[i] - q["tm"] + ins
        y0 = (q["ty"][i] - q["tt"] / 2 if den >= 200 else q["ty"][i]) + ins
        th = q["tt"] - 2 * ins
        sc.st(ext(f"member-tread-{i}", FF, AL, 0.55, "xy", rect2(x0 + 0.003, y0, 0.114 + 2 * q["tm"], th), -q["tw"][i] / 2 + ins, q["tw"][i] / 2 - ins, holes=[rect(x0 + 0.06 + q["tm"], y0 + th / 2, 0.106 + 2 * q["tm"], th - 0.006)]), "front-frame", "member")
    pth = pt - 2 * ins
    sc.st(ext("member-platform", FF, AL, 0.4, "xy", rect2(px - q["tm"] + ins, py + ins, pw + 2 * q["tm"] - 2 * ins, pth), -phw + ins, phw - ins, holes=[rect(px + pw / 2, py + ins + pth / 2, pw + 2 * q["tm"] - 2 * ins - 0.004, pth - 0.004)]), "front-frame", "member")
    if q["braces"]:
        for nm, c in (("low", (0.162, 0.22, 0.17)), ("high", (-0.025, 0.85, 0.14))):
            sc.st(ext(f"member-rear-brace-{nm}", RF, AL, 0.6, "xy", circle_pts(0, 0, 0.0105, 16), -c[2], c[2], holes=[circle_pts(0, 0, 0.0093, 16)], at=loc((c[0], c[1], 0), REAR)), "rear-frame", "member")
    for s in SIDES:
        n = SN[s]
        if q["sp"] and den >= 200:
            (x0, y0), (x1, y1), z = q["spr"]
            p0, p1 = (x0, y0, z * s), (x1, y1, z * s)
            a = unit(sub(p1, p0))
            sc.st(rod(f"member-spreader-{n}", FF, ZN, 0.6, [p0[i] + a[i] * 0.01 for i in range(3)], [p1[i] - a[i] * 0.01 for i in range(3)], q["sp"] - 0.004), "front-frame", "member")
        elif q["sp"]:
            for link, bone, off in (("front", FF, (0, 0)), ("rear", RF, REAR)):
                p0, p1 = spreader_end(s, link)
                a = unit(sub(p1, p0))
                sc.st(rod(f"member-spreader-{link}-{n}", bone, ZN, 0.6, [p0[i] + a[i] * 0.004 for i in range(3)], [p1[i] - a[i] * 0.004 for i in range(3)], q["sp"] - 0.002, off=off), "front-frame" if link == "front" else "rear-frame", "member")
        if q["hinge"]:
            sc.st(cyl(f"joint-hinge-{n}", RF, AL, 0.7, "z", (0.0, 0.0, 0.15 * s), 0.047, 0.0235), "rear-frame", "joint")
        if q["knuckle"]:
            sc.st(cyl(f"joint-knuckle-{n}", FF, ZN, 0.7, "z", (-0.09, 0.56, 0.165 * s), 0.059, 0.0135), "front-frame", "joint")
    mr = 0.0075 if den < 200 else 0.5 * 0.00015 * den
    sc.st(ellipsoid("mass-front", FF, AL, 0.5, [mr, mr, mr], [-0.216, q["ty"][2] + (q["tt"] / 2 if den < 200 else 0) + (0.003 if den < 200 else 0), 0.0]), "front-frame", "mass")
    for s in SIDES:
        pp = q["rr"]
        t0 = (0.6 - pp["p0"][1]) / (pp["p1"][1] - pp["p0"][1])
        c = [pp["p0"][i] + (pp["p1"][i] - pp["p0"][i]) * t0 for i in range(3)]
        sc.st(ellipsoid(f"mass-rear-{SN[s]}", RF, AL, 0.5, [max(0.012, mr)] * 3, loc((c[0], c[1], c[2] * s), REAR)), "rear-frame", "mass")
    sf = 0.0015
    for nm, bone, x, z, off, key in (("foot-front-left", FF, -0.365, 0.2, (0, 0), 0), ("foot-front-right", FF, -0.365, -0.2, (0, 0), 0), ("foot-rear-left", RF, 0.23, 0.175, REAR, 1), ("foot-rear-right", RF, 0.23, -0.175, REAR, 1)):
        hx, hz = (0.04, 0.025) if key == 0 else (0.04, 0.0225)
        if q["boxfeet"]:
            w, h, d = q["boxfeet"][key][:3]
            hx, hz = max(w / 2 - 0.0015, 0.00015 * den / 2), max(d / 2 - 0.0015, 0.00015 * den / 2)
        hy = max(0.02, 0.00015 * den)
        sc.st(box(f"support-{nm}", bone, RB, 0.1, [2 * hx, hy, 2 * hz], [x - off[0], hy / 2 - off[1], z]), bone, "support")


DEC = {
    50: (
        "Body: hand-built from the 1:50 drawings in true section: rails as 60 x 45 mm (front) and 50 x 45 mm (rear) bars on the drawn centre lines in all three views (the front pair converges to the top, the rear pair leans back to the hinge), four treads with the drawn nosing, the platform, the top cap with its domed top, the two-link steel spreader on each side (24 mm round bars) with its pivot knuckle, the hinge rivets, two braces between the rear rails and four tapered rubber feet.",
        "Körper: von Hand aus den Zeichnungen 1:50 im wahren Querschnitt gebaut: Holme als Stäbe 60 x 45 mm (vorn) und 50 x 45 mm (hinten) auf den gezeichneten Mittellinien in allen drei Ansichten (das vordere Paar läuft nach oben zusammen, das hintere neigt sich zum Gelenk), vier Stufen mit der gezeichneten Nase, die Plattform, die Kappe mit Wölbung, die zweigliedrige Stahlspreize je Seite (Rundstäbe 24 mm) mit Drehknoten, die Gelenkniete, zwei Streben zwischen den Hinterholmen und vier konische Gummifüße.",
    ),
    100: (
        "Body: the 1:100 drawings in the same construction without the cap, the treads' nosing and the corner chamfers; the spreader links are 30 mm round bars (the drawn 40 mm is pen-widened), the pivot knuckle is below the scale and merged into the links, hinge rivets, braces and tapered feet remain.",
        "Körper: die Zeichnungen 1:100 in derselben Konstruktion ohne Kappe, ohne Stufennase und ohne Kantenfasen; die Spreizenglieder sind Rundstäbe von 30 mm (die gezeichneten 40 mm sind stiftverbreitert), der Drehknoten liegt unter dem Maßstab und ist in die Glieder gelegt, Gelenkniete, Streben und konische Füße bleiben.",
    ),
    200: (
        "Body: modelled against the 1:200 drawings: rails as 60 mm bars (rear pair 100 mm in the elevation, as drawn), treads and platform as the 36 mm and 60 mm slabs the drawn strokes and outline give, one 40 mm round spreader per side from the front to the rear rail (the elevation omits it, the front and plan draw it), four rubber feet as blocks; hinge, braces, cap and nosing are below the scale.",
        "Körper: gegen die Zeichnungen 1:200 modelliert: Holme als Stäbe 60 mm (hinteres Paar 100 mm in der Ansicht, wie gezeichnet), Stufen und Plattform als Platten von 36 mm und 60 mm nach den gezeichneten Strichen und Umrissen, je Seite eine runde Spreize von 40 mm vom vorderen zum hinteren Holm (die Ansicht lässt sie weg, Vorderansicht und Grundriss zeichnen sie), vier Gummifüße als Blöcke; Gelenk, Streben, Kappe und Nase liegen unter dem Maßstab.",
    ),
    500: (
        "Body: modelled against the 1:500 drawings with the pen-floor widths they carry: front rails 134 mm in the elevation and 85 mm across (the front draws 80 mm, the plan 100 mm), rear rails 160 mm, treads and platform as 78.5 mm slabs of the drawn strokes (the strokes were shortened by the half pen at both ends in the asset so that their widened extent is the part), four feet as 80 mm blocks; spreaders, hinge, braces and cap are not drawn and not modelled in the body.",
        "Körper: gegen die Zeichnungen 1:500 mit ihren Stiftbreiten modelliert: Vorderholme 134 mm in der Ansicht und 85 mm quer (die Vorderansicht zeichnet 80 mm, der Grundriss 100 mm), Hinterholme 160 mm, Stufen und Plattform als Platten von 78,5 mm nach den gezeichneten Strichen (die Striche wurden im Asset an beiden Enden um den halben Stift gekürzt, damit ihre verbreiterte Ausdehnung das Teil ist), vier Füße als Blöcke von 80 mm; Spreizen, Gelenk, Streben und Kappe sind nicht gezeichnet und nicht im Körper modelliert.",
    ),
}
TH5 = (
    "Thermal: references to the body solids by region; the two spreaders, which the 1:500 drawings omit, are 75 mm thermal rods between the rails so that the region has a solid. Only the total area is compared at this scale: the pen-floor rails expose more than twice the real surface.",
    "Thermik: Verweise auf die Körper nach Region; die beiden Spreizen, die die Zeichnungen 1:500 weglassen, sind thermische Stäbe von 75 mm zwischen den Holmen, damit die Region einen Körper hat. In diesem Maßstab wird nur die Gesamtfläche verglichen: die stiftbreiten Holme zeigen mehr als die doppelte wirkliche Oberfläche.",
)
TH = (
    "Thermal: every region is the skin of its body solids (references, no copies): rails, cap, treads, platform, rear frame with braces and hinge rivets, spreader links, rubber feet; the contact faces of the feet are not counted.",
    "Thermik: jede Region ist die Haut ihrer Körper (Verweise, keine Kopien): Holme, Kappe, Stufen, Plattform, Hinterholme mit Streben und Gelenknieten, Spreizenglieder, Gummifüße; die Aufstandsflächen der Füße zählen nicht.",
)
ST = (
    "Structure: the rails are hollow aluminium tubes of the real wall (1.5 mm front, 1.2 mm rear) 1.5 mm inside the body skin, treads and platform hollow profiles, braces thin tubes, spreader links straps, hinge and knuckle pins as joints; the masses lie where material is nearest to the declared centres (the front centre of mass lies 16 mm in front of the tread, the rear one on the rails), foot pads as supports.",
    "Tragwerk: die Holme sind hohle Aluminiumrohre mit der echten Wand (1,5 mm vorn, 1,2 mm hinten), 1,5 mm unter der Körperhaut, Stufen und Plattform Hohlprofile, Streben dünne Rohre, Spreizenglieder Bänder, Gelenk- und Knotenbolzen als Gelenke; die Massen liegen dort, wo Material den angegebenen Schwerpunkten am nächsten ist (der vordere Schwerpunkt liegt 16 mm vor der Stufe, der hintere auf den Holmen), Fußplatten als Auflager.",
)

if __name__ == "__main__":
    for a in [a for a in sys.argv[1:] if "=" in a]:
        k, v = a.split("=", 1)
        den, key = k.split(".", 1)
        P[int(den)][key] = eval(v)
    scales = [int(a) for a in sys.argv[1:] if "=" not in a] or [50]
    out = []
    for den in scales:
        sc = build(den)
        thermal_refs(sc, den)
        structure(sc, P[den], den)
        sc.dec(*DEC.get(den, DEC[100]))
        sc.dec(*(TH5 if den == 500 else TH))
        sc.dec(*ST)
        out.append(sc)
    save("objects", "stepladder", out)
