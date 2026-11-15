import json, sys, copy
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
globals().update({n: getattr(L, n) for n in "box ell cap rev cyl ext loft rep shrink th st ref stref Scale write qz World".split()})

W = World("🛋️furniture", "three-seat-sofa")
seed = json.load(open("🗑️generated/author-16/seed-three-seat-sofa.json"))["volumes"]["50"]["body"]
SEED = {s["id"]: s for s in seed}
FA, BE, PW, FO, BA = "upholstery-fabric", "beech-timber", "birch-plywood", "hr-foam", "polyester-batting"
XS = [0.19, 0.7333, 1.2667]
WS = [0.5433, 0.5334, 0.5433]
XC = [x + w / 2 for x, w in zip(XS, WS)]
BACK = [(0, .19), (0, .44), (.6, .45), (.62187, .44688), (.6375, .4375), (.64687, .42188), (.65, .4), (.65, .34), (.64688, .3225), (.6375, .31), (.62187, .3025), (.6, .3)]
BACK8 = [(0, .19), (0, .44), (.6, .45), (.645, .42), (.65, .37), (.645, .32), (.6, .3)]
BACK5 = [(0, .19), (0, .44), (.62, .45), (.65, .35), (.6, .3)]
FEET = [(0.12, 0.36), (0.12, -0.36), (1.88, 0.36), (1.88, -0.36)]
ANCH = ["foot-left-l", "foot-left-r", "foot-right-l", "foot-right-r"]


def backprof(pr):
    return [(0.2 + y, z) for y, z in pr]


def back(i, pr, id=None, bone=None, x0=None, x1=None):
    return W.extx(bone or "back-%d" % (i + 1), id or "back-%d" % (i + 1), FA, 0.62, backprof(pr), XS[i] if x0 is None else x0, XS[i] + WS[i] if x1 is None else x1)


def seat(i, r, id=None, bone=None):
    return W.box(bone or "seat-%d" % (i + 1), id or "seat-%d" % (i + 1), FA, 0.72, (XC[i], 0.315, -0.11), (WS[i], 0.23, 0.62), r=r)


def arms(r, ids=("arm-l", "arm-r")):
    return [W.box("arm-left", ids[0], FA, 0.5, (0.1, 0.36, 0), (0.2, 0.52, 0.9), r=r), W.box("arm-right", ids[1], FA, 0.5, (1.9, 0.36, 0), (0.2, 0.52, 0.9), r=r)]


def skirt(r=0.01):
    return W.box("frame", "skirt", FA, 0.45, (1, 0.17, 0), (1.7, 0.14, 0.86), r=r)


def feet(kind):
    out = []
    for nm, x in (("foot-l", 0.12), ("foot-r", 1.88)):
        if kind == 50:
            out.append(copy.deepcopy(SEED[nm]))
        else:
            out.append(W.rev("frame", nm, BE, 0.4, [[0, 0.025], [0.12, 0.035]], (0, 1, 0), (x, 0, 0.36), rep=rep(2, (0, 0, -0.72))))
    return out


def s50():
    S = Scale(50)
    S.body = [copy.deepcopy(s) for s in seed]
    return S


def s100():
    S = Scale(100)
    S.body = arms(0.09) + [seat(i, 0.03) for i in range(3)] + [back(i, BACK8) for i in range(3)] + [skirt(0)] + feet(100)
    return S


def s200():
    S = Scale(200)
    b = arms(0.03)
    for nm, bone in (("arm-foot-l", "arm-left"), ("arm-foot-r", "arm-right")):
        x = 0.1 if bone == "arm-left" else 1.9
        b.append(W.box(bone, nm, BE, 0.4, (0.12 if bone == "arm-left" else 1.88, 0.05, 0.36), (0.06, 0.1, 0.06), rep=rep(2, (0, 0, -0.72))))
    b += [seat(i, 0) for i in range(3)]
    b.append(back(1, BACK5))
    b.append(back(0, BACK5, id="back-end-l", bone="frame"))
    b.append(back(2, BACK5, id="back-end-r", bone="frame"))
    b.append(skirt(0))
    S.body = b
    return S


def s500():
    S = Scale(500)
    b = arms(0)
    b.append(W.box("seat-2", "seat-block", FA, 0.72, (1.0, 0.315, -0.11), (1.62, 0.23, 0.62)))
    b.append(W.extx("frame", "back-block", FA, 0.62, backprof(BACK5), 0.19, 1.81))
    b.append(W.box("frame", "skirt", FA, 0.45, (1, 0.17, 0), (1.7, 0.14, 0.86)))
    for nm, x in (("foot-l", 0.12), ("foot-r", 1.88)):
        b.append(W.box("frame", nm, BE, 0.4, (x, 0.05, 0.36), (0.08, 0.1, 0.08), rep=rep(2, (0, 0, -0.72))))
    S.body = b
    return S


def region(sid):
    if sid.startswith("arm-foot") or sid.startswith("foot"):
        return "feet"
    if sid.startswith("seat"):
        return "seat-cushions"
    if sid.startswith("back"):
        return "back-cushions"
    if sid.startswith("arm"):
        return "arm-pads"
    if sid == "skirt":
        return "base-rail"
    return "feet"


INNER = {"seat-cushions": "foam-seat", "back-cushions": "foam-back", "arm-pads": "batting-arm", "base-rail": "ply-skirt"}


def thermal(S):
    out = []
    for b in S.body:
        r = region(b["id"])
        if r == "feet":
            out.append(ref("th-" + b["id"], b["id"], r, "beech-foot"))
            continue
        out.append(ref("th-" + b["id"], b["id"], r, "fabric"))
        if S.d == 50:
            out.append(th(shrink(b, 0.001), r, INNER[r], id="th-" + b["id"] + "-core"))
    return out


def structure(S):
    d = S.d
    o = []
    M = lambda s, bone: st(s, bone, "member")
    o.append(M(W.box("ground", "member-rails", BE, 0.3, (1.0, 0.15, -0.33), (1.62, 0.06 if d < 500 else 0.075, 0.05 if d < 500 else 0.075), rep=rep(2, (0, 0, 0.66))), "ground"))
    o.append(M(W.box("frame", "member-deck", PW, 0.3, (1.0, 0.205, 0), (1.62, 0.02 if d < 200 else 0.03 if d == 200 else 0.075, 0.8)), "frame"))
    for nm, bone, x in (("arm-l", "arm-left", 0.1), ("arm-r", "arm-right", 1.9)):
        sz = (0.14, 0.44, 0.8) if d < 500 else (0.14, 0.44, 0.8)
        o.append(M(W.box(bone, "member-" + nm, BE, 0.3, (x, 0.36, 0), sz), bone))
    for i in range(3):
        k = 0.02 if d <= 100 else 0.03 if d == 200 else 0.04
        o.append(M(W.box("seat-%d" % (i + 1), "member-seat-%d" % (i + 1), FO, 0.4, (XC[i], 0.315, -0.11), (WS[i] - 2 * k, 0.23 - 2 * k, 0.62 - 2 * k)), "seat-%d" % (i + 1)))
        o.append(M(W.box("back-%d" % (i + 1), "member-back-%d" % (i + 1), FO, 0.4, (XC[i], 0.55, 0.36), (WS[i] - 2 * k, 0.5, 0.12 if d < 500 else 0.1)), "back-%d" % (i + 1)))
    if d <= 100:
        for i in range(3):
            o.append(st(W.cyl("seat-%d" % (i + 1), "joint-seat-%d" % (i + 1), BE, 0.2, (XC[i], 0.21, -0.11), (XC[i], 0.25, -0.11), 0.03 if d == 50 else 0.0305, role="detail"), "seat-%d" % (i + 1), "joint"))
            if d == 100:
                continue
            o.append(st(W.cyl("back-%d" % (i + 1), "joint-back-%d" % (i + 1), BE, 0.2, (XC[i], 0.21, 0.32), (XC[i], 0.25, 0.32), 0.03 if d == 50 else 0.0305, role="detail"), "back-%d" % (i + 1), "joint"))
    elif d == 200:
        o.append(st(W.cyl("seat-2", "joint-seat-2", BE, 0.2, (1.0, 0.21, -0.11), (1.0, 0.25, -0.11), 0.0305, role="detail"), "seat-2", "joint"))
        o.append(st(W.cyl("back-2", "joint-back-2", BE, 0.2, (1.0, 0.21, 0.32), (1.0, 0.25, 0.32), 0.0305, role="detail"), "back-2", "joint"))
    else:
        o.append(st(W.cyl("seat-2", "joint-seat-2", BE, 0.2, (1.0, 0.2, -0.11), (1.0, 0.28, -0.11), 0.04, role="detail"), "seat-2", "joint"))
    sz = {50: (0.05, 0.008, 0.05), 100: (0.05, 0.016, 0.05), 200: (0.06, 0.031, 0.06), 500: (0.076, 0.076, 0.076)}[d]
    for a, (x, z) in zip(ANCH, FEET):
        p = st(W.box("frame", "support-" + a, BE, 0.15, (x, sz[1] / 2, z), sz, role="detail"), "frame", "support")
        p["anchor"] = a
        o.append(p)
    for nm, bone, x in (("arm-l", "arm-left", 0.1), ("arm-r", "arm-right", 1.9)):
        o.append(st(W.ell(bone, "mass-" + nm, FA, 0.15, (x, 0.36, 0), (0.07, 0.15, 0.2), role="detail"), bone, "mass"))
    if d < 500:
        for i in range(3):
            o.append(st(W.ell("seat-%d" % (i + 1), "mass-seat-%d" % (i + 1), FA, 0.15, (XC[i], 0.315, -0.11), (0.15, 0.05, 0.15), role="detail"), "seat-%d" % (i + 1), "mass"))
            o.append(st(W.ell("back-%d" % (i + 1), "mass-back-%d" % (i + 1), FA, 0.15, (XC[i], 0.55, 0.36), (0.15, 0.15, 0.05 if d < 200 else 0.04), role="detail"), "back-%d" % (i + 1), "mass"))
    else:
        o.append(st(W.ell("seat-2", "mass-seat-2", FA, 0.15, (1.0, 0.315, -0.11), (0.4, 0.0385, 0.25), role="detail"), "seat-2", "mass"))
    return o


DEC = {
    "50": ("Detailed model from the seed: rounded armrests, three crowned-edge seat cushions, three leaning back cushions extruded from the drawn profile, a plywood-faced skirt and four tapered beech feet. Every drawn bone has its own solid.", "Detailmodell aus dem Startmodell: gerundete Armlehnen, drei Sitzpolster mit gerundeter Kante, drei geneigte Rückenpolster aus dem gezeichneten Profil, eine Zarge und vier konische Buchenfüße. Jeder gezeichnete Knochen hat einen eigenen Körper."),
    "100": ("Modelled anew against the 1:100 drawings: smaller corner radii, back profile reduced to seven points, feet as plain tapered revolves; piping and welts are not modelled.", "Neu gegen die Zeichnungen 1:100 modelliert: kleinere Eckenradien, Rückenprofil auf sieben Punkte reduziert, Füße als schlichte konische Drehkörper; Keder und Nähte sind nicht modelliert."),
    "200": ("Modelled anew: sharp seat blocks, the back as three extrusions (centre cushion on its bone, the two ends merged into the frame bone as the drawing does), arms with feet merged into blocks 0.2 m wide.", "Neu modelliert: scharfkantige Sitzblöcke, die Lehne als drei Strangkörper (Mittelkissen auf seinem Knochen, die zwei Enden im Rahmenknochen zusammengefasst wie in der Zeichnung), Arme mit Füßen zu 0,2 m breiten Blöcken zusammengefasst."),
    "500": ("Modelled anew: one seat block on the centre seat bone, one back block, two arm blocks, the skirt and four 0.1 m feet; every part is above the 75 mm feature.", "Neu modelliert: ein Sitzblock auf dem mittleren Sitzknochen, ein Rückenblock, zwei Armblöcke, die Zarge und vier 0,1 m breite Füße; jedes Teil liegt über dem Merkmal von 75 mm."),
}
DEC_T = {
    "50": ("Thermal: references by region; every upholstered region has two nested shells (1 mm fabric = body solid, core inside it: foam, batting or plywood); feet are a single timber layer. Structure: hidden timber rails, plywood deck, arm frames, foam cores of seats and backs as members, prismatic joint pins, four floor pads at the support anchors, masses in arms, seats and backs.", "Thermik: Verweise je Region; jede gepolsterte Region hat zwei verschachtelte Schalen (1 mm Stoff = Körper, darin der Kern aus Schaum, Vlies oder Sperrholz); Füße sind eine einzige Holzschicht. Tragwerk: verdeckte Holzschienen, Sperrholzdeck, Armrahmen, Schaumkerne von Sitzen und Lehnen als Stäbe, prismatische Gelenkstifte, vier Bodenplatten an den Auflagern, Massen in Armen, Sitzen und Lehnen."),
    "100": ("Thermal and structure follow the 1:100 body; cores are thinner, pins simplified.", "Thermik und Tragwerk folgen dem Körper 1:100; die Kerne sind dünner, die Stifte vereinfacht."),
    "200": ("Thermal and structure follow the 1:200 body; seat joints only (the back cores are merged into the frame body).", "Thermik und Tragwerk folgen dem Körper 1:200; nur Sitzgelenke (die Lehnenkerne sind im Rahmenkörper aufgegangen)."),
    "500": ("Thermal and structure follow the merged 1:500 body; one joint and 0.1 m pads.", "Thermik und Tragwerk folgen dem zusammengefassten Körper 1:500; ein Gelenk und 0,1 m große Platten."),
}

if __name__ == "__main__":
    sc = {"50": s50(), "100": s100(), "200": s200(), "500": s500()}
    for k, S in sc.items():
        S.thermal = thermal(S)
        S.structure = structure(S)
        S.say(*DEC[k])
        S.say(*DEC_T[k])
    write("furniture", "three-seat-sofa", sc)
