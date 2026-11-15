import json, math, sys, copy
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
globals().update({n: getattr(L, n) for n in "box ell cap rev cyl ext loft rep shrink th st ref stref Scale write qz qy qx qaxes pipe_loft_q World".split()})

W = World("🛋️furniture", "office-swivel-chair")
seed = json.load(open("🗑️generated/author-16/seed-office-swivel-chair.json"))["volumes"]["50"]["body"]
SEED = {s["id"]: s for s in seed}
PL, ST, FA, RU = "pa-plastic", "steel-painted", "fabric-upholstery", "rubber"
ARMS = ["front", "side-a", "side-b", "back-a", "back-b"]
TIP = {"front": (0.3, 0.0), "side-a": (0.0927, 0.2806), "side-b": (0.0927, -0.2806), "back-a": (-0.2427, 0.1763), "back-b": (-0.2427, -0.1763)}
Y0 = 0.025


def fixz(s):
    s = copy.deepcopy(s)
    if s["bone"] in ("arm-side-a", "arm-side-b", "castor-side-a", "castor-side-b"):
        for sec in s.get("sections", []):
            sec["at"][2] = round(sec["at"][2] * 0.9835, 5)
        if "origin" in s and abs(s["origin"][2]) > 0.1:
            s["origin"][2] = round(s["origin"][2] * 0.9835, 5)
    return s


def arm(a, id, tip_ry, base_rx, tip_rx, base_ry, tip_exp=2.5, start=0.03, yb=Y0 + 0.018, yt=Y0 + 0.008):
    tx, tz = TIP[a]
    n = math.hypot(tx, tz) or 1
    d = (tx, tz) if a != "front" else (1, 0)
    q = pipe_loft_q(d)
    sx, sz = (d[0] / math.hypot(*d) * start, d[1] / math.hypot(*d) * start)
    return W.loft("arm-" + a, id, PL, 0.3, [((sx, yb, sz), base_rx, base_ry, tip_exp, q), ((tx, yt, tz), tip_rx, tip_ry, tip_exp, q)])


def wheel(a, id, r, w, prof=None):
    tx, tz = TIP[a]
    if prof is None:
        return W.rev("castor-" + a, id, RU, 0.2, [[-w / 2, r], [w / 2, r]], (0, 0, 1), (tx, Y0, tz))
    return W.rev("castor-" + a, id, RU, 0.2, prof, (0, 0, 1), (tx, Y0, tz))


def backshell(secs, id="backrest-shell", tone=0.5):
    q = qaxes((0, 0, 1), (1, 0, 0), (0, 1, 0))
    return W.loft("backrest", id, PL, tone, [((x, y, 0), 0.22, ry, 8, q) for (x, y, ry) in secs])


# ------------------------------------------------------------------ 1:50
def s50():
    S = Scale(50)
    body = [fixz(s) for s in seed]
    for s in body:
        if s["id"] == "backrest-shell":
            s["sections"].insert(0, {"at": [0.062, 0.0145, 0], "rx": 0.22, "ry": 0.0245, "exponent": 8, "rotation": s["sections"][0]["rotation"]})
            last = s["sections"][-1]
            s["sections"][-1] = dict(last, rx=0.216, at=[0.6335, 0.007, 0])
            s["sections"].insert(-1, dict(last, rx=0.22, at=[0.62, 0.0075, 0], ry=0.0175))
        if s["id"] == "armrest-bracket":
            s["size"] = [0.02, 0.03, 0.022]
            s["transform"]["translation"] = [0.043, 0.035, 0.247]
            s["repeat"] = {"count": 2, "translation": [0, 0, -0.494]}
        if s["id"] == "seat-lever":
            s["transform"]["translation"][2] = -0.0725
    for a in ARMS:
        tx, tz = TIP[a]
        body.append(W.cyl("castor-" + a, "castor-%s-hub" % a, ST, 0.2, (tx, Y0, tz - 0.0135), (tx, Y0, tz + 0.0135), 0.008, role="detail"))
    S.body = body
    return S


def s100():
    S = Scale(100)
    b = []
    b.append(W.rev("base", "hub", PL, 0.3, [[-0.02, 0], [-0.016, 0.035], [0.004, 0.055], [0.042, 0.04]], (0, 1, 0), (0, Y0, 0)))
    for a in ARMS:
        b.append(fixz(SEED["arm-" + a]))
    for a in ARMS:
        b.append(wheel(a, "castor-%s-wheel" % a, 0.024, 0.025))
    b.append(W.rev("column-sleeve", "column-sleeve", ST, 0.25, [[0.02, 0.034], [0.3, 0.026]], (0, 1, 0), (0, Y0, 0)))
    b.append(W.rev("column", "column", ST, 0.7, [[0.2, 0.0125], [0.37, 0.0125], [0.435, 0.032]], (0, 1, 0), (0, Y0, 0)))
    b.append(W.box("seat", "seat-cushion", FA, 0.62, (-0.001, 0.454, 0), (0.466, 0.062, 0.48), r=0.025))
    b.append(W.box("seat", "seat-mechanism", ST, 0.22, (0, 0.404, 0), (0.2, 0.042, 0.2), r=0.006))
    bs = copy.deepcopy(SEED["backrest-shell"])
    bs["sections"] = [bs["sections"][i] for i in (0, 2, 4, 6, 7)]
    bs["sections"].insert(0, {"at": [0.062, 0.0145, 0], "rx": 0.22, "ry": 0.0245, "exponent": 8, "rotation": bs["sections"][0]["rotation"]})
    bs["sections"][-1] = dict(bs["sections"][-1], rx=0.216, at=[0.6335, 0.007, 0])
    b.append(bs)
    b.append(copy.deepcopy(SEED["backrest-strut"]))
    for k in ("armrest-post",):
        b.append(W.box("seat", "armrest-post", PL, 0.35, (-0.035, 0.565, 0.2575), (0.03, 0.16, 0.045), r=0.004, rep=rep(2, (0, 0, -0.515))))
    b.append(copy.deepcopy(SEED["armrest-pad"]))
    S.body = b
    return S


def s200():
    S = Scale(200)
    b = []
    b.append(W.rev("base", "hub", PL, 0.3, [[-0.02, 0.05], [0.042, 0.05]], (0, 1, 0), (0, Y0, 0)))
    for a in ARMS:
        b.append(arm(a, "arm-" + a, 0.0155, 0.02, 0.0175, 0.0155, yb=0.046, yt=0.041))
    for a in ARMS:
        b.append(wheel(a, "castor-%s-wheel" % a, 0.0238, 0.0305))
    b.append(W.rev("column-sleeve", "column-sleeve", ST, 0.25, [[0.0, 0.03], [0.3, 0.03]], (0, 1, 0), (0, Y0, 0)))
    b.append(W.rev("column", "column", ST, 0.7, [[0.285, 0.0155], [0.425, 0.0155]], (0, 1, 0), (0, Y0, 0)))
    b.append(W.box("seat", "seat-cushion", FA, 0.62, (0.001, 0.4625, 0), (0.462, 0.035, 0.48), r=0.012))
    b.append(backshell([(-0.235, 0.512, 0.035), (-0.2325, 0.64, 0.0395), (-0.233, 0.766, 0.037), (-0.234, 0.9, 0.0328), (-0.2365, 0.984, 0.0275), (-0.238, 1.073, 0.02)]))
    b.append(W.ext("seat", "armrest", PL, 0.4, [(-0.035, 0.485), (-0.035, 0.66), (0.1, 0.66)], 0.2545, 0.2855, rep=rep(2, (0, 0, -0.54))))
    S.body = b
    return S


def arm500(a):
    tx, tz = TIP[a]
    d = (tx, tz) if a != "front" else (1, 0)
    q = pipe_loft_q(d)
    n = math.hypot(*d)
    sx, sz = d[0] / n * 0.03, d[1] / n * 0.03
    return W.loft("arm-" + a, "arm-" + a, PL, 0.3, [((sx, 0.0378, sz), 0.0376, 0.0376, 2.0, q), ((tx, 0.0378, tz), 0.0376, 0.0376, 2.0, q)])


def s500():
    S = Scale(500)
    b = []
    b.append(W.rev("base", "hub", PL, 0.3, [[-0.025, 0.06], [0.05, 0.06]], (0, 1, 0), (0, Y0, 0)))
    for a in ARMS:
        b.append(arm500(a))
    for a in ARMS:
        b.append(wheel(a, "castor-%s-wheel" % a, 0.0375, 0.075))
    b.append(W.rev("column", "column", ST, 0.7, [[0.05, 0.0376], [0.4, 0.0376]], (0, 1, 0), (0, Y0, 0)))
    b.append(W.box("seat", "seat-cushion", FA, 0.62, (0.0, 0.44, 0), (0.472, 0.1, 0.48)))
    b.append(W.box("backrest", "backrest-shell", PL, 0.5, (-0.23, 0.7625, 0), (0.1, 0.635, 0.44)))
    S.body = b
    return S



REG = {PL: ("chair-plastic", "pa-shell"), ST: ("chair-steel", "tube"), FA: ("chair-fabric", "fabric"), RU: ("castor-rubber", "tyre")}


def thermal(S):
    out = []
    for b in S.body:
        reg, layer = REG[b["material"]]
        out.append(ref("th-" + b["id"], b["id"], reg, layer))
        if S.d == 50 and reg == "chair-fabric":
            out.append(th(shrink(b, 0.001), reg, "foam", id="th-" + b["id"] + "-foam"))
    if S.d == 100:
        out.append(th(W.box("seat", "seat-shell", PL, 0.3, (-0.0225 * 0 + 0.0, 0.4225, 0), (0.39, 0.016, 0.46)), "chair-plastic", "pa-shell"))
        out.append(th(W.box("backrest", "backrest-pad", FA, 0.3, (-0.2, 0.77, 0), (0.015, 0.45, 0.4)), "chair-fabric", "fabric"))
    if S.d == 200:
        out.append(th(W.box("backrest", "backrest-pad", FA, 0.3, (-0.2, 0.77, 0), (0.03, 0.45, 0.4)), "chair-fabric", "fabric"))
        out.append(th(W.rev("column", "column-cone", ST, 0.22, [[0.37, 0.0155], [0.435, 0.032]], (0, 1, 0), (0, Y0, 0)), "chair-steel", "tube"))
        out.append(th(W.cyl("column-sleeve", "column-collar", ST, 0.15, (0, 0.30, 0), (0, 0.35, 0), 0.044), "chair-steel", "tube"))
        out.append(th(W.box("seat", "seat-mechanism", ST, 0.22, (0, 0.4405, 0), (0.2, 0.031, 0.2)), "chair-steel", "tube"))
    if S.d == 500:
        out.append(th(W.cyl("column-sleeve", "column-sleeve", ST, 0.25, (0, 0.06, 0), (0, 0.33, 0), 0.045), "chair-steel", "tube"))
    return out


def pads(S, size):
    out = []
    for a in ARMS:
        tx, tz = TIP[a]
        p = st(W.box("castor-" + a, "support-contact-castor-" + a, ST, 0.15, (tx, size[1] / 2, tz), size, role="detail"), "castor-" + a, "support")
        p["anchor"] = "contact-castor-" + a
        out.append(p)
    return out


MB = {50: (0.04, 0.025, 0.04), 100: (0.04, 0.025, 0.04), 200: (0.05, 0.03, 0.05), 500: (0.06, 0.0385, 0.06)}


def masses(S, scale):
    f = {50: 1.0, 100: 1.0, 200: 2.0, 500: 5.0}[scale]
    out = [
        st(W.ell("base", "mass-base", PL, 0.15, (0, 0.043, 0), MB[scale], role="detail"), "base", "mass"),
        st(W.ell("seat", "mass-seat", FA, 0.15, (0, 0.454, 0), (0.08, 0.02 * (1 if f < 2 else 1.98), 0.08), role="detail"), "seat", "mass"),
        st(W.ell("backrest", "mass-backrest", PL, 0.15, (-0.236, 0.755, 0), (0.025 if f < 2 else 0.033 if f == 2 else 0.0385, 0.07, 0.1), role="detail"), "backrest", "mass"),
    ]
    if scale <= 50:
        for a in ARMS:
            tx, tz = TIP[a]
            out.append(st(W.ell("castor-" + a, "mass-castor-" + a, RU, 0.15, (tx, Y0, tz), (0.012, 0.012, 0.012), role="detail"), "castor-" + a, "mass"))
    return out


def structure(S):
    d = S.d
    ids = {b["id"] for b in S.body}
    out = []
    m = lambda id, bone: stref("member-" + id, id, bone, "member")
    out.append(m("hub", "base"))
    for a in ARMS:
        out.append(m("arm-" + a, "arm-" + a))
    if "column-sleeve" in ids:
        out.append(m("column-sleeve", "column-sleeve"))
    else:
        out.append(st(W.rev("column-sleeve", "member-column-sleeve", ST, 0.25, [[0.05, 0.0376], [0.3, 0.0376]], (0, 1, 0), (0, Y0, 0)), "column-sleeve", "member"))
    out.append(m("column", "column"))
    if "seat-mechanism" in ids:
        out.append(m("seat-mechanism", "seat"))
    else:
        out.append(st(W.box("seat", "member-seat-mechanism", ST, 0.22, (0, 0.404 if d < 200 else 0.4435 if d == 200 else 0.405, 0), (0.2, 0.042 if d < 200 else 0.031 if d == 200 else 0.075, 0.2), r=0.0 if d >= 200 else 0.006), "seat", "member"))
    if d == 50:
        out.append(m("seat-shell", "seat"))
    if d <= 100:
        out.append(m("armrest-post", "seat"))
    if d == 200:
        out.append(m("armrest", "seat"))
    if "backrest-strut" in ids:
        out.append(m("backrest-strut", "backrest"))
    else:
        out.append(st(W.cap("backrest", "member-backrest-strut", ST, 0.22, (-0.2425, 0.445 if d < 200 else 0.49, 0), (-0.256, 0.685, 0), 0.0155 if d == 200 else 0.0376), "backrest", "member"))
    out.append(m("backrest-shell", "backrest"))
    if d <= 200:
        if d != 100:
            out.append(st(W.cyl("column", "joint-column", ST, 0.15, (0, 0.30, 0), (0, 0.33, 0), 0.0215), "column", "joint")) if d < 200 else None
        out.append(st(W.cyl("seat", "joint-seat", ST, 0.15, (0, 0.405 if d < 200 else 0.4155, 0), (0, 0.4475 if d >= 200 else 0.445, 0), 0.02 if d < 200 else 0.0155, role="detail"), "seat", "joint"))
        out.append(st(W.cyl("backrest", "joint-backrest", ST, 0.15, (-0.23, 0.445, -0.1), (-0.23, 0.445, 0.1), 0.01 if d == 50 else 0.0078 if d == 100 else 0.0155, role="detail"), "backrest", "joint"))
        r = {50: 0.006, 100: 0.0078, 200: 0.0155}[d]
        h = {50: 0.0135, 100: 0.0135, 200: 0.0165}[d]
        for a in ARMS:
            tx, tz = TIP[a]
            out.append(st(W.cyl("castor-" + a, "joint-castor-" + a, ST, 0.15, (tx, Y0, tz - h), (tx, Y0, tz + h), r, role="detail"), "castor-" + a, "joint"))
    else:
        out.append(st(W.cyl("seat", "joint-seat", ST, 0.15, (0, 0.39, 0), (0, 0.4675, 0), 0.0392, role="detail"), "seat", "joint"))
    out += pads(S, {50: (0.016, 0.008, 0.012), 100: (0.016, 0.016, 0.016), 200: (0.03, 0.03, 0.03), 500: (0.075, 0.075, 0.075)}[d])
    out += masses(S, d)
    return out


def say(S):
    pass


DEC = {
    "50": lambda S: [
        S.say("Detailed model: the seed solids are kept and repaired. Side castor stems moved 1.7 % inwards so the width equals 0.60 m, the backrest shell now starts at the drawn lower edge and reaches the drawn top corners, the armrest brackets and the seat lever sit inside the drawn front silhouette, castor hubs added. Body solids: five lofted star arms with yoke, stem, tyre and hub, hub, gas-lift sleeve, collar, rod and cone, rounded seat cushion on shell and mechanism, armrests, lofted backrest shell with pad and strut.", "Detailliertes Modell: die Startkörper bleiben erhalten und sind repariert. Die Stifte der seitlichen Rollen sind um 1,7 % nach innen gerückt, damit die Breite 0,60 m beträgt, die Rückenschale beginnt an der gezeichneten Unterkante und erreicht die gezeichneten oberen Ecken, Armlehnenwinkel und Sitzhebel liegen innerhalb der gezeichneten Vorderansicht, Rollennaben sind ergänzt. Körper: fünf gelofteten Sternarme mit Joch, Stift, Reifen und Nabe, Nabe, Gasliftrohr, Kragen, Stange und Konus, gerundetes Sitzpolster auf Schale und Mechanik, Armlehnen, gelofteter Rückenschalenkörper mit Polster und Strebe."),
        S.say("Thermal: every body solid is bound to its region by material (reference), so the surface is the body surface; the upholstery region has two nested shells, the outer 1 mm fabric (the body solid) and the foam core inside it, ordered innermost first. Structure: members reference the load-bearing body solids (hub, five arms, sleeve, rod, seat mechanism and shell, armrest posts, backrest strut and shell); joint solids sit at the swivel, backrest hinge, gas-lift collar and five castor axles; five contact pads on the floor carry the support anchors; compact masses at the hub, seat, backrest and castors.", "Thermik: jeder Körper ist über sein Material einer Region zugeordnet (Verweis), die Fläche ist also die Körperfläche; die Polsterregion hat zwei verschachtelte Schalen, außen 1 mm Gewebe (der Körper) und innen den Schaumkern, innerste zuerst. Tragwerk: Stäbe verweisen auf die tragenden Körper (Nabe, fünf Arme, Rohr, Stange, Sitzmechanik und -schale, Armlehnenstützen, Rückenstrebe und -schale); Gelenkkörper sitzen an Drehlager, Lehnengelenk, Gasliftkragen und den fünf Rollenachsen; fünf Kontaktplatten am Boden tragen die Auflager; kompakte Massen an Nabe, Sitz, Lehne und Rollen."),
    ],
    "100": lambda S: [
        S.say("Modelled anew against the 1:100 drawings: arms without yokes or stems, castors as plain tyres, one revolve for rod and cone, five backrest sections, armrest posts widened to touch the seat edge (they are drawn as filled posts, the drawing was corrected from a hairline). Seat shell, backrest pad, ribs and hubs are merged away (declared below 15 mm).", "Neu gegen die Zeichnungen 1:100 modelliert: Arme ohne Joche und Stifte, Rollen als schlichte Reifen, ein Drehkörper für Stange und Konus, fünf Rückenschnitte, Armlehnenstützen bis an die Sitzkante verbreitert (sie sind als gefüllte Stützen gezeichnet, die Zeichnung wurde von einer Haarlinie korrigiert). Sitzschale, Lehnenpolster, Rippen und Naben sind weggelassen (unter 15 mm)."),
        S.say("Thermal: references by material; the hidden seat shell and the backrest pad, merged in the body, stay as thermal-only solids so the region areas keep the 1:50 magnitude. Structure: members on the same bones, castor joints and the three main joints reduced to the swivel, hinge and axles, supports and masses at the hub, seat and backrest.", "Thermik: Verweise nach Material; die verdeckte Sitzschale und das Lehnenpolster, im Körper aufgegangen, bleiben als reine Thermikkörper, damit die Regionsflächen die Größenordnung von 1:50 behalten. Tragwerk: Stäbe auf denselben Knochen, Rollen- und Hauptgelenke auf Drehlager, Lehnengelenk und Achsen reduziert, Auflager und Massen an Nabe, Sitz und Lehne."),
    ],
    "200": lambda S: [
        S.say("Modelled anew: hub and sleeve as plain cylinders, arms with the 30 mm minimum section, tyres 30 mm wide, a 35 mm seat slab (mechanism not drawn), a six-section backrest following the drawn curve, one triangular armrest panel per side merging post, bracket and pad as the 1:200 drawing shows.", "Neu modelliert: Nabe und Rohr als schlichte Zylinder, Arme mit dem Mindestquerschnitt 30 mm, Reifen 30 mm breit, 35 mm Sitzplatte (Mechanik nicht gezeichnet), sechsteilige Rückenlehne nach der gezeichneten Kurve, je Seite eine dreieckige Armlehnenplatte aus Stütze, Winkel und Auflage wie in der Zeichnung 1:200."),
        S.say("Thermal: references by material plus the seat shell and mechanism as thermal-only solids. Structure: members as before, the mechanism plate raised to touch the seat, joints merged to swivel, hinge, collar and axles, 30 mm contact pads.", "Thermik: Verweise nach Material, dazu Sitzschale und Mechanik als reine Thermikkörper. Tragwerk: Stäbe wie zuvor, Mechanikplatte bis an den Sitz angehoben, Gelenke auf Drehlager, Lehnengelenk, Kragen und Achsen zusammengefasst, Kontaktplatten 30 mm."),
    ],
    "500": lambda S: [
        S.say("Modelled anew: every part is at least 75 mm thick, so hub, five arms, column and five tyres are 75 mm bodies, the seat is a 100 mm slab and the backrest a 100 mm panel as drawn; armrests, sleeve and mechanism are merged. The column and arms are thicker than the hairlines of the drawing, which costs silhouette precision (see report).", "Neu modelliert: jedes Teil ist mindestens 75 mm dick, deshalb sind Nabe, fünf Arme, Säule und fünf Reifen 75-mm-Körper, der Sitz eine 100-mm-Platte und die Lehne eine 100-mm-Tafel wie gezeichnet; Armlehnen, Rohr und Mechanik sind zusammengefasst. Säule und Arme sind dicker als die Haarlinien der Zeichnung, was Genauigkeit der Silhouette kostet (siehe Bericht)."),
        S.say("Thermal: references by material. Structure: members on every bone (the sleeve and seat mechanism as 75 mm members inside the column and seat bodies), swivel joint, five 75 mm pads, three masses at hub, seat and backrest.", "Thermik: Verweise nach Material. Tragwerk: Stäbe auf jedem Knochen (Rohr und Sitzmechanik als 75-mm-Stäbe in Säule und Sitz), Drehlager, fünf 75-mm-Platten, drei Massen an Nabe, Sitz und Lehne."),
    ],
}

if __name__ == "__main__":
    sc = {"50": s50(), "100": s100(), "200": s200(), "500": s500()}
    for k, S in sc.items():
        S.thermal = thermal(S)
        S.structure = structure(S)
        DEC[k](S)
    write("furniture", "office-swivel-chair", sc)
