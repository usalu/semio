import json, sys, copy
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
globals().update({n: getattr(L, n) for n in "box ell cap rev cyl ext loft rep shrink th st ref stref Scale write qz World".split()})

W = World("🛋️furniture", "picnic-table")
seed = json.load(open("🗑️generated/author-16/seed-picnic-table.json"))["volumes"]["50"]["body"]
SEED = {s["id"]: s for s in seed}
PI, GA = "pine-timber", "steel-galvanised"
LEGL = [(-0.25, 0.71), (-0.19, 0.71), (-0.66, 0), (-0.72, 0)]
LEGR = [(0.19, 0.71), (0.25, 0.71), (0.72, 0), (0.66, 0)]
LEGL5 = [(-0.25, 0.71), (-0.19, 0.71), (-0.66, 0), (-0.708, 0), (-0.72, 0.012)]


def legs(prof_l, prof_r, t, z0, nm=("leg-l", "leg-r")):
    return [W.ext("frame", nm[0], PI, 0.55, prof_l, z0 - t / 2, z0 + t / 2, rep=rep(2, (0, 0, -2 * z0))), W.ext("frame", nm[1], PI, 0.55, prof_r, z0 - t / 2, z0 + t / 2, rep=rep(2, (0, 0, -2 * z0)))]


def hinges(sx, sy, sz, zc=0.725):
    return [W.box("frame", "hinge-l", GA, 0.2, (-0.46, 0.39, zc), (sx, sy, sz), rep=rep(2, (0, 0, -2 * zc))), W.box("frame", "hinge-r", GA, 0.2, (0.46, 0.39, zc), (sx, sy, sz), rep=rep(2, (0, 0, -2 * zc)))]


def s50():
    S = Scale(50)
    S.body = [copy.deepcopy(s) for s in seed]
    return S


def s100():
    S = Scale(100)
    b = legs(LEGL, LEGR, 0.045, 0.8)
    b.append(W.box("frame", "top-beam", PI, 0.5, (0, 0.6875, 0.8), (0.6, 0.045, 0.045), rep=rep(2, (0, 0, -1.6))))
    b.append(W.box("frame", "beam", PI, 0.5, (0, 0.3775, 0.8), (0.96, 0.045, 0.045), rep=rep(2, (0, 0, -1.6))))
    b += hinges(0.08, 0.1, 0.11)
    b.append(W.box("top", "top", PI, 0.7, (0, 0.73, 0), (0.7, 0.04, 1.8), r=0.004))
    b.append(W.box("bench-r", "bench-r", PI, 0.7, (0.58, 0.42, 0), (0.36, 0.04, 1.7), r=0.008))
    b.append(W.box("bench-l", "bench-l", PI, 0.7, (-0.58, 0.42, 0), (0.36, 0.04, 1.7), r=0.008))
    S.body = b
    return S


def s200():
    S = Scale(200)
    b = legs(LEGL, LEGR, 0.05, 0.8)
    b.append(W.box("frame", "beam", PI, 0.5, (0, 0.375, 0.8), (0.96, 0.05, 0.05), rep=rep(2, (0, 0, -1.6))))
    b += hinges(0.08, 0.1, 0.11)
    b.append(W.box("top", "slab", PI, 0.7, (0, 0.7075, 0), (0.7, 0.085, 1.8)))
    b.append(W.box("bench-r", "bench-r", PI, 0.7, (0.58, 0.425, 0), (0.36, 0.05, 1.7)))
    b.append(W.box("bench-l", "bench-l", PI, 0.7, (-0.58, 0.425, 0), (0.36, 0.05, 1.7)))
    S.body = b
    return S


def s500():
    S = Scale(500)
    t = 0.08
    b = legs([(-0.285, 0.7), (-0.175, 0.7), (-0.62, 0), (-0.73, 0)], [(0.175, 0.7), (0.285, 0.7), (0.73, 0), (0.62, 0)], t, 0.8)
    b += hinges(0.08, 0.1, 0.11)
    b.append(W.box("top", "slab", PI, 0.7, (0, 0.7, 0), (0.7, 0.1, 1.8)))
    b.append(W.box("bench-r", "bench-r", PI, 0.7, (0.555, 0.4, 0), (0.41, 0.075, 1.7)))
    b.append(W.box("bench-l", "bench-l", PI, 0.7, (-0.555, 0.4, 0), (0.41, 0.075, 1.7)))
    S.body = b
    return S


def region(b):
    i = b["id"]
    if i.startswith("top-plank") or i in ("top", "slab"):
        return "table-top", "top-plank"
    if i.startswith("bench"):
        return "bench-seats", "seat-plank"
    if i.startswith("hinge"):
        return "hinges", "hinge-plate"
    return "a-frame", "frame-member"


def thermal(S):
    return [ref("th-" + b["id"], b["id"], *region(b)) for b in S.body]


ANCH = [("foot-left-l", -0.684, 0.8), ("foot-left-r", -0.684, -0.8), ("foot-right-l", 0.684, 0.8), ("foot-right-r", 0.684, -0.8)]


def structure(S):
    d = S.d
    o = []
    t = {50: 0.0475, 100: 0.0475, 200: 0.05, 500: 0.077}[d]
    o.append(st(W.box("ground", "member-feet", PI, 0.3, (-0.668, 0.0155 if d < 500 else 0.0385, 0.8), (0.036 if d < 500 else 0.09, 0.031 if d < 500 else 0.077, t), rep=rep(2, (0, 0, -1.6))), "ground", "member"))
    o.append(st(W.box("ground", "member-feet-r", PI, 0.3, (0.668, 0.0155 if d < 500 else 0.0385, 0.8), (0.036 if d < 500 else 0.09, 0.031 if d < 500 else 0.077, t), rep=rep(2, (0, 0, -1.6))), "ground", "member"))
    o.append(stref("member-leg-l", "leg-l", "frame", "member"))
    o.append(stref("member-leg-r", "leg-r", "frame", "member"))
    if d < 500:
        o.append(stref("member-beam", "beam", "frame", "member"))
    if d <= 100:
        o.append(stref("member-top-beam", "top-beam", "frame", "member"))
    if d == 50:
        o.append(stref("member-top-plank", "top-plank", "top", "member"))
    else:
        o.append(stref("member-top", "top" if d == 100 else "slab", "top", "member"))
    o.append(stref("member-bench-r", "bench-r", "bench-r", "member"))
    o.append(stref("member-bench-l", "bench-l", "bench-l", "member"))
    o.append(stref("joint-hinge-r", "hinge-r", "bench-r", "joint"))
    o.append(stref("joint-hinge-l", "hinge-l", "bench-l", "joint"))
    sz = {50: (0.06, 0.008, 0.05), 100: (0.06, 0.016, 0.05), 200: (0.06, 0.031, 0.05), 500: (0.09, 0.076, 0.076)}[d]
    for a, x, z in ANCH:
        p = st(W.box("frame", "support-" + a, PI, 0.15, (x, sz[1] / 2, z), sz, role="detail"), "frame", "support")
        p["anchor"] = a
        o.append(p)
    o.append(st(W.ell("frame", "mass-frame", PI, 0.15, (0, 0.73, 0.8), (0.2, 0.015 if d < 500 else 0.0385, 0.03 if d < 500 else 0.0385), role="detail", rep=rep(2, (0, 0, -1.6))), "frame", "mass"))
    o.append(st(W.ell("top", "mass-top", PI, 0.15, (0, 0.73, 0), (0.25, 0.015 if d < 500 else 0.0385, 0.6), role="detail", rep=None), "top", "mass"))
    for nm, bone, x in (("bench-r", "bench-r", 0.58), ("bench-l", "bench-l", -0.58)):
        o.append(st(W.ell(bone, "mass-" + nm, PI, 0.15, (x, 0.42 if d < 500 else 0.4, 0), (0.15, 0.015 if d < 500 else 0.0385, 0.6), role="detail"), bone, "mass"))
    return o


DEC = {
    "50": ("Detailed model from the seed: four 45 mm A-frame leg boards as extrusions of the drawn raked profile, two top beams and two stretcher beams, galvanised hinge blocks, five tabletop planks and two rounded bench boards on their hinge bones.", "Detailmodell aus dem Startmodell: vier 45 mm starke A-Rahmenbeine als Strangkörper des gezeichneten geneigten Profils, zwei obere und zwei untere Querbalken, verzinkte Scharnierblöcke, fünf Tischplatten und zwei gerundete Bankbretter auf ihren Scharnierknochen."),
    "100": ("Modelled anew: tabletop planks merged into one slab, benches as one board each; leg profile reduced to a four-point parallelogram; screws and bolt heads are left out.", "Neu modelliert: Tischplatten zu einer Platte zusammengefasst, Bänke als je ein Brett; Beinprofil auf ein Parallelogramm mit vier Punkten reduziert; Schrauben und Bolzenköpfe entfallen."),
    "200": ("Modelled anew: top slab and top beams merged into one 85 mm slab as drawn, benches 50 mm, legs 50 mm; no corner radii.", "Neu modelliert: Platte und obere Balken zu einer 85-mm-Platte zusammengefasst wie gezeichnet, Bänke 50 mm, Beine 50 mm; keine Eckenradien."),
    "500": ("Modelled anew: legs 75 mm thick, slab 100 mm, benches 75 mm; the drawing shows legs and benches as hairlines thinner than the 75 mm feature, so they are heavier than drawn.", "Neu modelliert: Beine 75 mm dick, Platte 100 mm, Bänke 75 mm; die Zeichnung zeigt Beine und Bänke als Haarlinien unter dem Merkmal von 75 mm, sie sind daher kräftiger als gezeichnet."),
}
DEC_T = {
    "50": ("Thermal: references by material (planks, bench boards, frame members, hinge plates, single layers). Structure: leg boards, beams and plank/bench boards as members, hinge blocks as joints, foot blocks as the ground-bone members, four floor pads at the support anchors, masses in frame, top and benches.", "Thermik: Verweise nach Material (Bretter, Bankbretter, Rahmenteile, Scharnierplatten, einzelne Schichten). Tragwerk: Beinbretter, Balken und Platten-/Bankbretter als Stäbe, Scharnierblöcke als Gelenke, Fußblöcke als Stäbe des Bodenknochens, vier Bodenplatten an den Auflagern, Massen in Rahmen, Platte und Bänken."),
    "100": ("Thermal and structure follow the 1:100 body.", "Thermik und Tragwerk folgen dem Körper 1:100."),
    "200": ("Thermal and structure follow the 1:200 body; the top beams are merged into the slab.", "Thermik und Tragwerk folgen dem Körper 1:200; die oberen Balken sind in der Platte aufgegangen."),
    "500": ("Thermal and structure follow the 1:500 body; the stretcher beams are merged into the legs.", "Thermik und Tragwerk folgen dem Körper 1:500; die unteren Balken sind in den Beinen aufgegangen."),
}

if __name__ == "__main__":
    sc = {"50": s50(), "100": s100(), "200": s200(), "500": s500()}
    for k, S in sc.items():
        S.thermal = thermal(S)
        S.structure = structure(S)
        S.say(*DEC[k])
        S.say(*DEC_T[k])
    write("furniture", "picnic-table", sc)
