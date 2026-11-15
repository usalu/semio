import json, sys, copy
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
globals().update({n: getattr(L, n) for n in "box ell cap rev cyl ext loft rep shrink th st ref stref Scale write qz World".split()})

W = World("🛋️furniture", "operating-table")
seed = json.load(open("🗑️generated/author-16/seed-operating-table.json"))["volumes"]["50"]["body"]
SS, PS, RC, PF, AL, GL, SC = "stainless-steel", "powder-coated-steel", "rubber-castor", "pad-foam", "light-aluminium", "light-lens-glass", None
HX, HY = 0.67, 1.66


def s50():
    S = Scale(50)
    S.body = [copy.deepcopy(s) for s in seed]
    return S


def castors(bone, prefix, r, w, xs, zc, y, rp=None):
    out = []
    for nm, x in zip("ab", xs):
        out.append(W.rev(bone, "%s-castor-%s" % (prefix, nm), RC, 0.2, [[-w / 2, r], [w / 2, r]], (0, 0, 1), (x, y, zc), rep=rep(2, (0, 0, -2 * zc))))
    return out


def head(prof, lens=None):
    o = [W.rev("light-head", "head-housing" if lens is not None else "head", AL, 0.35, prof, (0, 1, 0), (HX, 0, 0))]
    if lens:
        o.append(W.rev("light-head", "head-lens", GL, 0.9, [[lens[0], 0.24], [lens[1], 0.24]], (0, 1, 0), (HX, 0, 0)))
    return o


def s100():
    S = Scale(100)
    b = [W.box("base", "tb-plate", PS, 0.5, (0, 0.115, 0), (0.8, 0.09, 0.4), r=0.01)]
    b += castors("base", "tb", 0.04, 0.04, (-0.32, 0.32), 0.17, 0.04)
    b += [W.box("column", "col-lower", SS, 0.6, (0, 0.28, 0), (0.3, 0.24, 0.3)), W.box("column", "col-upper", SS, 0.7, (0, 0.57, 0), (0.21, 0.34, 0.21)), W.box("column", "col-head", SS, 0.5, (0, 0.76, 0), (0.32, 0.04, 0.4))]
    b += [W.box("top", "top-carrier", SS, 0.55, (0, 0.8, 0), (1.96, 0.04, 0.4)), W.box("top", "top-pad-head", PF, 0.3, (-0.811, 0.86, 0), (0.378, 0.08, 0.5), r=0.02), W.box("top", "top-pad-back", PF, 0.3, (-0.335, 0.86, 0), (0.566, 0.08, 0.5), r=0.02), W.box("top", "top-pad-seat", PF, 0.3, (0.075, 0.86, 0), (0.246, 0.08, 0.5), r=0.02), W.box("top", "top-pad-leg", PF, 0.3, (0.601, 0.86, 0), (0.798, 0.08, 0.5), r=0.02)]
    b.append(W.box("stand-base", "st-base", PS, 0.45, (1.17, 0.0825, 0), (0.56, 0.045, 0.5), r=0.01))
    b += castors("stand-base", "st", 0.03, 0.03, (0.95, 1.39), 0.2, 0.03)
    b.append(W.cyl("stand-column", "st-column", PS, 0.5, (1.17, 0.105, 0), (1.17, 1.88, 0), 0.035))
    b.append(W.box("stand-arm", "st-arm", PS, 0.5, (0.92, 1.9225, 0), (0.52, 0.045, 0.045)))
    b.append(W.cyl("stand-arm", "st-rod", PS, 0.5, (HX, 1.78, 0), (HX, 1.9, 0), 0.015))
    b += head([[1.675, 0.25], [1.7, 0.25], [1.755, 0.17], [1.78, 0]], (1.655, 1.675))
    b.append(W.cyl("light-head", "head-handle", AL, 0.4, (HX, 1.6, 0), (HX, 1.66, 0), 0.02))
    S.body = b
    return S


def s200():
    S = Scale(200)
    b = [W.box("base", "tb-plate", PS, 0.5, (0, 0.08, 0), (0.8, 0.16, 0.4), r=0.02)]
    b.append(W.box("column", "col", SS, 0.65, (0, 0.47, 0), (0.32, 0.62, 0.32)))
    b.append(W.box("top", "top", PF, 0.35, (0, 0.84, 0), (2.0, 0.12, 0.5), r=0.02))
    b.append(W.box("stand-base", "st-base", PS, 0.45, (1.17, 0.0525, 0), (0.56, 0.105, 0.5), r=0.02))
    b.append(W.cyl("stand-column", "st-column", PS, 0.5, (1.17, 0.105, 0), (1.17, 1.9, 0), 0.04))
    b.append(W.box("stand-arm", "st-arm", PS, 0.5, (0.92, 1.92, 0), (0.54, 0.06, 0.06)))
    b.append(W.cyl("stand-arm", "st-rod", PS, 0.5, (HX, 1.78, 0), (HX, 1.9, 0), 0.02))
    b += head([[1.66, 0.25], [1.7, 0.25], [1.755, 0.17], [1.78, 0]])
    S.body = b
    return S


def s500():
    S = Scale(500)
    b = [W.box("base", "tb-plate", PS, 0.5, (0, 0.08, 0), (0.8, 0.16, 0.4))]
    b.append(W.box("column", "col", SS, 0.65, (0, 0.47, 0), (0.28, 0.62, 0.28)))
    b.append(W.box("top", "top", PF, 0.35, (0, 0.84, 0), (2.0, 0.12, 0.5)))
    b.append(W.box("stand-base", "st-base", PS, 0.45, (1.17, 0.0525, 0), (0.56, 0.105, 0.5)))
    b.append(W.box("stand-column", "st-column", PS, 0.5, (1.17, 1.0, 0), (0.08, 1.79, 0.08)))
    b.append(W.box("stand-arm", "st-arm", PS, 0.5, (0.92, 1.92, 0), (0.54, 0.075, 0.075)))
    b += head([[1.66, 0.25], [1.7, 0.25], [1.755, 0.17], [1.78, 0]])
    S.body = b
    return S


def region(sid):
    if sid.startswith("top-pad") or sid in ("top",):
        return "table-pad", "foam" if False else None
    if sid == "top-carrier":
        return "table-frame", "carrier"
    if sid.startswith("col"):
        return "table-column", "cover"
    if sid.startswith("tb-"):
        return "table-base", "shell"
    if sid.startswith("head-lens"):
        return "light-lens", "lens"
    if sid.startswith("head"):
        return "light-housing", "housing"
    return "stand", "tube"


def thermal(S):
    o = []
    ids = {b["id"] for b in S.body}
    for b in S.body:
        r, l = region(b["id"])
        if r == "table-pad":
            o.append(ref("th-" + b["id"], b["id"], r, "vinyl"))
            if S.d == 50:
                o.append(th(shrink(b, 0.001), r, "foam", id="th-" + b["id"] + "-foam"))
        else:
            o.append(ref("th-" + b["id"], b["id"], r, l))
    d = S.d
    if "top-carrier" not in ids:
        o.append(th(W.box("top", "top-carrier", SS, 0.55, (0, 0.8, 0), (1.96, 0.04 if d < 500 else 0.076, 0.4)), "table-frame", "carrier"))
    if "head-lens" not in ids:
        o.append(th(W.rev("light-head", "head-lens", GL, 0.9, [[1.66, 0.24], [1.66 + (0.015 if d < 200 else 0.031 if d == 200 else 0.076), 0.24]], (0, 1, 0), (HX, 0, 0)), "light-lens", "lens"))
    ry = {50: 0.02, 100: 0.025, 200: 0.035, 500: 0.0385}[d]
    o.append(th(W.ell("light-head", "light-plume", AL, 0.35, (HX, 1.775, 0), (0.2, ry, 0.2), role="detail"), "light-housing", "housing"))
    return o


ANCH = [("castor-table-rear-left", "base", -0.32, 0.15), ("castor-table-rear-right", "base", -0.32, -0.15), ("castor-table-front-left", "base", 0.32, 0.15), ("castor-table-front-right", "base", 0.32, -0.15), ("castor-stand-rear-left", "stand-base", 0.95, 0.185), ("castor-stand-rear-right", "stand-base", 0.95, -0.185), ("castor-stand-front-left", "stand-base", 1.39, 0.185), ("castor-stand-front-right", "stand-base", 1.39, -0.185)]


def structure(S):
    d = S.d
    o = []
    ids = {b["id"] for b in S.body}
    mem = lambda of, bone, id=None: stref("member-" + (id or of), of, bone, "member")
    o.append(mem("tb-plate", "base"))
    if d <= 100:
        o += [mem("col-lower", "column"), mem("col-upper", "column"), mem("col-head", "column")]
        o.append(mem("top-carrier", "top"))
    else:
        o.append(mem("col", "column"))
        o.append(st(W.box("top", "member-top-carrier", SS, 0.55, (0, 0.8, 0), (1.96, 0.04 if d < 500 else 0.076, 0.4)), "top", "member"))
    o += [mem("st-base", "stand-base"), mem("st-column", "stand-column"), mem("st-arm", "stand-arm")]
    o.append(mem("head-housing" if d == 100 or d == 50 else "head", "light-head"))
    pr = {50: 0.03, 100: 0.0305, 200: 0.0305, 500: 0.0385}[d]
    o.append(st(W.cyl("top", "joint-top-tilt", SS, 0.15, (0, 0.78, -0.18), (0, 0.78, 0.18), 0.02 if d < 200 else 0.0155 if d == 200 else 0.0385, role="detail"), "top", "joint")) if d < 500 else o.append(st(W.cyl("top", "joint-top-tilt", SS, 0.15, (0, 0.8, -0.2), (0, 0.8, 0.2), 0.0385, role="detail"), "top", "joint"))
    if d <= 100:
        o.append(st(W.ell("stand-arm", "joint-arm", PS, 0.15, (0.69, 1.92, 0), (0.02, 0.02, 0.02), role="detail"), "stand-arm", "joint"))
        o.append(st(W.cyl("stand-column", "joint-collar", PS, 0.15, (1.17, 1.88, 0), (1.17, 1.96, 0), 0.045, role="detail"), "stand-column", "joint")) if d == 50 else None
    sz = {50: (0.03, 0.008, 0.03), 100: (0.03, 0.016, 0.03), 200: (0.04, 0.031, 0.04), 500: (0.08, 0.076, 0.08)}[d]
    for a, bone, x, z in ANCH:
        p = st(W.box(bone, "support-" + a, PS, 0.15, (x, sz[1] / 2, z), sz, role="detail"), bone, "support")
        p["anchor"] = a
        o.append(p)
    M = lambda id, bone, c, r: st(W.ell(bone, "mass-" + id, PS, 0.15, c, r, role="detail"), bone, "mass")
    f = 1.0 if d < 200 else 1.5 if d == 200 else 2.5
    o.append(M("base", "base", (0, 0.115, 0), (0.25, 0.03 if d < 500 else 0.0385, 0.12 if d < 500 else 0.15)))
    o.append(M("column", "column", (0, 0.3, 0), (0.1 if d < 500 else 0.12, 0.08, 0.1 if d < 500 else 0.12)))
    o.append(M("top", "top", (0, 0.86, 0), (0.25, 0.028 if d < 200 else 0.0385 if d == 500 else 0.032, 0.2)))
    o.append(M("stand-base", "stand-base", (1.17, 0.083, 0), (0.12, 0.02 if d < 100 else 0.0155 if d == 100 else 0.03 if d == 200 else 0.0385, 0.12)))
    o.append(M("stand-column", "stand-column", (1.17, 0.2, 0), (0.02 if d < 200 else 0.031 if d == 200 else 0.0385, 0.05, 0.02 if d < 200 else 0.031 if d == 200 else 0.0385)))
    if d < 500:
        o.append(M("stand-arm", "stand-arm", (0.92, 1.92, 0), (0.1, 0.015 if d < 200 else 0.031, 0.015 if d < 200 else 0.031)))
        o.append(M("light-head", "light-head", (HX, 1.72, 0), (0.15, 0.03, 0.15)))
    return o


DEC = {
    "50": ("Detailed model from the seed: lofted table base plate with two castors, stainless column (lofted lower part, box upper part, head plate), carrier plate with four rounded pad segments, stand with lofted base, revolved column and collar, boxed arm with knob, rod, domed aluminium lamp housing, lens and handle.", "Detailmodell aus dem Startmodell: gelofteter Tischfuß mit zwei Rollen, Edelstahlsäule (geloftetes Unterteil, Oberteil als Quader, Kopfplatte), Trägerplatte mit vier gerundeten Polstersegmenten, Stativ mit geloftetem Fuß, gedrehter Säule und Kragen, Quaderarm mit Knauf, Stange, gewölbtem Leuchtengehäuse aus Aluminium, Linse und Griff."),
    "100": ("Modelled anew: column in three plain boxes, castors as plain cylinders, one pad slab, simpler lamp dome; collar and knob are merged.", "Neu modelliert: Säule aus drei schlichten Quadern, Rollen als schlichte Zylinder, eine Polsterplatte, einfachere Leuchtenkuppel; Kragen und Knauf sind zusammengefasst."),
    "200": ("Modelled anew: base plate reaches the floor (castors merged), one 0.32 m column box, one 120 mm tabletop slab (carrier and pad merged), stand base to the floor, 80 mm column, 60 mm arm, 40 mm rod, one dome for housing and lens.", "Neu modelliert: Fußplatte reicht bis zum Boden (Rollen zusammengefasst), ein Säulenquader 0,32 m, eine 120-mm-Tischplatte (Träger und Polster zusammengefasst), Stativfuß bis zum Boden, 80-mm-Säule, 60-mm-Arm, 40-mm-Stange, eine Kuppel für Gehäuse und Linse."),
    "500": ("Modelled anew: all parts are boxes of at least 75 mm: base plate, 0.28 m column, tabletop slab, stand base, 80 mm stand column, 75 mm arm, lamp dome; rod and castors are merged.", "Neu modelliert: alle Teile sind Quader von mindestens 75 mm: Fußplatte, 0,28-m-Säule, Tischplatte, Stativfuß, 80-mm-Stativsäule, 75-mm-Arm, Leuchtenkuppel; Stange und Rollen sind zusammengefasst."),
}
DEC_T = {
    "50": ("Thermal: references by region; the pad has two nested shells (1 mm vinyl = body solid, foam core inside); the lamp plume emitter is a flat disc of the emitter diameter 0.4 m on the housing dome. Structure: base plate, column parts, carrier, stand parts and housing as members; table tilt pin, arm knob and collar as joints; eight castor pads at the support anchors; masses at base, column, top, stand parts and lamp.", "Thermik: Verweise je Region; das Polster hat zwei verschachtelte Schalen (1 mm Vinyl = Körper, darin der Schaumkern); der Leuchten-Plume-Emitter ist eine flache Scheibe mit dem Emitterdurchmesser 0,4 m auf der Gehäusekuppel. Tragwerk: Fußplatte, Säulenteile, Träger, Stativteile und Gehäuse als Stäbe; Kippstift, Armknauf und Kragen als Gelenke; acht Rollenplatten an den Auflagern; Massen an Fuß, Säule, Platte, Stativteilen und Leuchte."),
    "100": ("Thermal and structure follow the 1:100 body.", "Thermik und Tragwerk folgen dem Körper 1:100."),
    "200": ("Thermal and structure follow the 1:200 body; the carrier plate and the lens are thermal-only solids inside the merged bodies.", "Thermik und Tragwerk folgen dem Körper 1:200; Trägerplatte und Linse sind reine Thermikkörper in den zusammengefassten Körpern."),
    "500": ("Thermal and structure follow the 1:500 body; carrier plate and lens are thermal-only solids of 75 mm.", "Thermik und Tragwerk folgen dem Körper 1:500; Trägerplatte und Linse sind reine Thermikkörper von 75 mm."),
}

if __name__ == "__main__":
    sc = {"50": s50(), "100": s100(), "200": s200(), "500": s500()}
    for k, S in sc.items():
        S.thermal = thermal(S)
        S.structure = structure(S)
        S.say(*DEC[k])
        S.say(*DEC_T[k])
    write("furniture", "operating-table", sc)
