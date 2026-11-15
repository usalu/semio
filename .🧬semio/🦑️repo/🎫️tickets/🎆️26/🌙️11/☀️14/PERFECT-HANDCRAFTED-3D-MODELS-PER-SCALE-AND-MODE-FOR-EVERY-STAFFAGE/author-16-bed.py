import json, sys, copy
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
globals().update({n: getattr(L, n) for n in "box ell cap rev cyl ext loft rep shrink th st ref stref Scale write qz World".split()})

W = World("🛋️furniture", "single-bed")
seed = json.load(open("🗑️generated/author-16/seed-single-bed.json"))["volumes"]["50"]["body"]
PI, BI, QU = "pine-timber", "birch-slat", "quilted-cover"
MATS = [("mat-head", 0.4275, 0.745), ("mat-mid", 1.09, 0.62), ("mat-foot", 1.7225, 0.685)]
SLATS = [("slat-head", 0.06, 0.78), ("slat-mid", 0.78, 1.38), ("slat-foot", 1.38, 2.06)]


def mats(r, h=0.21):
    return [W.box(b, b, QU, 0.85, (x, 0.38, 0), (w, h, 0.82), r=r) for b, x, w in MATS]


def frame(scale):
    b = []
    if scale == 100:
        b.append(W.box("frame", "head", PI, 0.5, (0.03, 0.475, 0), (0.06, 0.95, 0.9), r=0.02))
        b.append(W.box("frame", "foot-post", PI, 0.5, (2.09, 0.275, 0.42), (0.06, 0.55, 0.06), r=0.015, rep=rep(2, (0, 0, -0.84))))
        b.append(W.box("frame", "foot-rail", PI, 0.5, (2.09, 0.51, 0), (0.06, 0.08, 0.9), r=0.015))
        b.append(W.box("frame", "foot-cross-rail", PI, 0.5, (2.09, 0.22, 0), (0.04, 0.16, 0.8)))
        b.append(W.box("frame", "rail", PI, 0.55, (1.06, 0.22, 0.43), (2.04, 0.16, 0.04), rep=rep(2, (0, 0, -0.86))))
        b.append(W.box("frame", "leg-mid", PI, 0.5, (1.06, 0.08, 0), (0.06, 0.16, 0.06)))
    elif scale == 200:
        b.append(W.box("frame", "head", PI, 0.5, (0.03, 0.475, 0), (0.06, 0.95, 0.9)))
        b.append(W.box("frame", "foot-post", PI, 0.5, (2.09, 0.275, 0.42), (0.06, 0.55, 0.06), rep=rep(2, (0, 0, -0.84))))
        b.append(W.box("frame", "foot-rail", PI, 0.5, (2.09, 0.51, 0), (0.06, 0.08, 0.9)))
        b.append(W.box("frame", "foot-cross-rail", PI, 0.5, (2.09, 0.22, 0), (0.04, 0.16, 0.8)))
        b.append(W.box("frame", "rail", PI, 0.55, (1.06, 0.22, 0.43), (2.04, 0.16, 0.04), rep=rep(2, (0, 0, -0.86))))
        b.append(W.box("frame", "leg-mid", PI, 0.5, (1.06, 0.08, 0), (0.06, 0.16, 0.06)))
    else:
        b.append(W.box("frame", "head", PI, 0.5, (0.05, 0.475, 0), (0.1, 0.95, 0.9)))
        b.append(W.box("frame", "foot", PI, 0.5, (2.07, 0.275, 0), (0.1, 0.55, 0.9)))
        b.append(W.box("frame", "rail", PI, 0.55, (1.06, 0.22, 0.4125), (2.0, 0.16, 0.076), rep=rep(2, (0, 0, -0.825))))
        b.append(W.box("frame", "leg-mid", PI, 0.5, (1.05, 0.08, 0), (0.1, 0.16, 0.1)))
    return b


def s50():
    S = Scale(50)
    S.body = [copy.deepcopy(s) for s in seed]
    return S


def s100():
    S = Scale(100)
    b = frame(100)
    for nm, x0, x1 in SLATS:
        b.append(W.box(nm, nm, BI, 0.6, ((x0 + x1) / 2, 0.274, 0), (x1 - x0, 0.022, 0.86)))
    S.body = b + mats(0.03)
    return S


def s200():
    S = Scale(200)
    S.body = frame(200) + mats(0.0)
    return S


def s500():
    S = Scale(500)
    S.body = frame(500) + mats(0.0)
    return S


def region(b):
    i = b["id"]
    if i.startswith("mat"):
        return "mattress", "quilt"
    if i.startswith("slat"):
        return "slats", "slat-birch"
    return "bed-frame", "pine-member"


def thermal(S):
    o = []
    for b in S.body:
        r, l = region(b)
        o.append(ref("th-" + b["id"], b["id"], r, l))
        if S.d == 50 and r == "mattress":
            o.append(th(shrink(b, 0.01), r, "foam-core", id="th-" + b["id"] + "-core"))
    if S.d >= 200:
        for nm, x0, x1 in SLATS:
            o.append(th(W.box(nm, "slats-" + nm[5:], BI, 0.6, ((x0 + x1) / 2, 0.2755 if S.d == 200 else 0.2625, 0), (x1 - x0, 0.031 if S.d == 200 else 0.076, 0.86)), "slats", "slat-birch"))
    return o


def structure(S):
    d = S.d
    o = []
    M = lambda id, of, bone: stref("member-" + id, of, bone, "member")
    o.append(st(W.box("ground", "member-center-beam", PI, 0.3, (1.06, 0.2115, 0), (2.0, 0.103 if d < 500 else 0.075, 0.05 if d < 500 else 0.075)), "ground", "member"))
    ids = {b["id"] for b in S.body}
    if d == 50:
        for x in ("head", "foot-post", "foot-rail", "rail", "leg-mid"):
            o.append(M(x, x, "frame"))
    elif d <= 200:
        for x in ("head", "foot-post", "rail", "leg-mid"):
            o.append(M(x, x, "frame"))
    else:
        for x in ("head", "foot", "rail"):
            o.append(M(x, x, "frame"))
    for nm, x0, x1 in SLATS:
        if nm in ids:
            o.append(M(nm, nm, nm))
        else:
            o.append(st(W.box(nm, "member-" + nm, BI, 0.6, ((x0 + x1) / 2, 0.2705 if d == 200 else 0.2625, 0), (x1 - x0, 0.032 if d == 200 else 0.076, 0.82)), nm, "member"))
    k = {50: 0.02, 100: 0.02, 200: 0.03, 500: 0.04}[d]
    for b, x, w in MATS:
        o.append(st(W.box(b, "member-" + b, "cold-foam", 0.4, (x, 0.38, 0), (w - 2 * k, 0.21 - 2 * k, 0.82 - 2 * k)), b, "member"))
    o.append(st(W.cyl("slat-head", "joint-hinge", PI, 0.2, (0.78, 0.285, -0.3), (0.78, 0.285, 0.3), 0.02 if d < 200 else 0.0305 if d == 200 else 0.0385, role="detail"), "slat-head", "joint"))
    if d <= 100:
        for b, x, w in MATS:
            o.append(st(W.cyl(b, "joint-" + b, PI, 0.2, (x, 0.27, 0), (x, 0.30, 0), 0.012 if d == 50 else 0.0155, role="detail"), b, "joint"))
    sz = {50: (0.008, 0.008), 100: (0.016, 0.016), 200: (0.031, 0.031), 500: (0.076, 0.076)}[d]
    pads = [("foot-head", 0.03, 0, 0.06, 0.5), ("foot-mid", 1.06, 0, 0.06, 0.06), ("foot-foot-l", 2.09, 0.42, 0.06, 0.06), ("foot-foot-r", 2.09, -0.42, 0.06, 0.06)]
    for a, x, z, sx, sz2 in pads:
        sx2 = max(sx, 0.076) if d == 500 else sx
        p = st(W.box("frame", "support-" + a, PI, 0.15, (x, sz[0] / 2, z), (sx2, sz[0], max(sz2, 0.076) if d == 500 else sz2), role="detail"), "frame", "support")
        p["anchor"] = a
        o.append(p)
    return o


DEC = {
    "50": ("Detailed model from the seed: rounded headboard and footboard, two foot posts with rail, cross rail, two side rails and the centre beam, one mid leg, three birch slat groups with their gaps and three rounded quilted mattress sections on the slat bones. Slats are drawn and therefore stay in the body.", "Detailmodell aus dem Startmodell: gerundetes Kopf- und Fußteil, zwei Fußpfosten mit Riegel, Querriegel, zwei Seitenholme und Mittelbalken, ein mittleres Bein, drei Birkenlattengruppen mit Fugen und drei gerundete gesteppte Matratzenteile auf den Lattenknochen. Die Latten sind gezeichnet und bleiben daher im Körper."),
    "100": ("Modelled anew: slat groups merged into one 22 mm plank each as drawn, mattress corners smaller, slat gaps and rounding left out.", "Neu modelliert: Lattengruppen zu je einer 22-mm-Bohle zusammengefasst wie gezeichnet, kleinere Matratzenecken, Lattenfugen und Rundungen entfallen."),
    "200": ("Modelled anew: only frame and mattress bones are drawn, so the slats live in the thermal and structure lists as plates inside the body (hidden region); sharp-edged boxes with 30 mm minimum.", "Neu modelliert: nur Rahmen- und Matratzenknochen sind gezeichnet, deshalb liegen die Latten als Platten im Körper nur in Thermik- und Tragwerkliste (verdeckte Region); scharfkantige Quader mit mindestens 30 mm."),
    "500": ("Modelled anew: head and foot are 100 mm boards (the foot posts and rail are merged), the two rails 75 mm, one 100 mm mid leg, three mattress blocks.", "Neu modelliert: Kopf- und Fußteil sind 100-mm-Bretter (Fußpfosten und Riegel zusammengefasst), die zwei Holme 75 mm, ein 100-mm-Mittelbein, drei Matratzenblöcke."),
}
DEC_T = {
    "50": ("Thermal: the mattress has two nested shells (10 mm quilt = body solid, foam core inside it), slats and frame are single layers. Structure: frame members, centre beam on the ground bone, slat groups, foam cores of the mattress sections as members; slat hinge pin and mattress guide pins as joints; four floor pads at the support anchors; all masses are uniform, so no mass solids.", "Thermik: die Matratze hat zwei verschachtelte Schalen (10 mm Steppdecke = Körper, darin der Schaumkern), Latten und Rahmen sind einzelne Schichten. Tragwerk: Rahmenteile, Mittelbalken am Bodenknochen, Lattengruppen, Schaumkerne der Matratzenteile als Stäbe; Lattenscharnierstift und Matratzenführungen als Gelenke; vier Bodenplatten an den Auflagern; alle Massen sind gleichmäßig verteilt, daher keine Massenkörper."),
    "100": ("Thermal and structure follow the 1:100 body.", "Thermik und Tragwerk folgen dem Körper 1:100."),
    "200": ("Thermal and structure follow the 1:200 body; slats as plates inside the body.", "Thermik und Tragwerk folgen dem Körper 1:200; Latten als Platten im Körper."),
    "500": ("Thermal and structure follow the 1:500 body; 75 mm slat plates; one hinge joint.", "Thermik und Tragwerk folgen dem Körper 1:500; Lattenplatten 75 mm; ein Scharniergelenk."),
}

if __name__ == "__main__":
    sc = {"50": s50(), "100": s100(), "200": s200(), "500": s500()}
    for k, S in sc.items():
        S.thermal = thermal(S)
        S.structure = structure(S)
        S.say(*DEC[k])
        S.say(*DEC_T[k])
    write("furniture", "single-bed", sc)
