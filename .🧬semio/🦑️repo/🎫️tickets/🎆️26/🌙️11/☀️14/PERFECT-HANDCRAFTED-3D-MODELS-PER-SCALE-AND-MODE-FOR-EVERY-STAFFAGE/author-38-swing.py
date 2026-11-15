import os, sys, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-19-lib.py")).read())
GR, LL, LR, BR, BM, CL, CR, ST = "ground", "leg-left", "leg-right", "brace", "beam", "chain-left", "chain-right", "seat"
GA, RB = "steel-galvanised", "rubber"
ZL, ZR = 0.56, -0.56
CZ = {CL: 0.23, CR: -0.23}
LEN, AL, AR_ = 2.64, math.radians(60.5), math.radians(119.5)
ALL6 = ["-u", "+u", "-v", "+v", "-w", "+w"]


def mat2quat(cx, cy, cz):
    m = [[cx[0], cy[0], cz[0]], [cx[1], cy[1], cz[1]], [cx[2], cy[2], cz[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [r6(c) for c in q]


def leg_sections(bone, z, r, plate_top=0.012, rs=None):
    rs = r if rs is None else rs
    ang = AL if bone == LL else AR_
    up_u, up_v = math.sin(ang), math.cos(ang)
    e = (up_v, -up_u)
    q = mat2quat((0, 0, 1), (e[0], e[1], 0), (up_u, up_v, 0))
    u0 = plate_top / up_u
    k = abs(up_v)
    return [sec([u0, 0, z], r, rs / up_u, rotation=q), sec([LEN, 0, z], r, rs)], u0


def leg(id, bone, z, r=0.03, role="body", tone=0.45, pt=0.012, rs=None):
    secs, u0 = leg_sections(bone, z, r, pt, rs)
    return loft(id, bone, GA, tone, secs, role=role)


def tube_u(id, bone, mat, tone, u0, u1, z, ro, ri, role="detail", v=0.0):
    return revolve(id, bone, mat, tone, "x", [u0, v, z], [[0, ri], [u1 - u0, ri], [u1 - u0, ro], [0, ro], [0, ri]], role=role)


def collar(id, bone, mat, tone, u0, length, z, r, c=0.004, role="detail"):
    pr = [[0, 0], [0, r - c], [c, r], [length - c, r], [length, r - c], [length, 0], [0, 0]]
    return revolve(id, bone, mat, tone, "x", [u0, 0, z], pr, role=role)


def seat_loft(id, tone=0.2, e=4, scale=1.0, k=5):
    zs = [(-0.26, 0.17, 0.02), (-0.25, 0.205, 0.028), (0.0, 0.21, 0.029), (0.25, 0.205, 0.028), (0.26, 0.17, 0.02)]
    if k == 3:
        zs = [(-0.26, 0.19, 0.027), (0.0, 0.21, 0.029), (0.26, 0.19, 0.027)]
    return loft(id, ST, RB, tone, [sec([0, -0.069, z], rx, ry, e) for z, rx, ry in zs])


def plate_box(id, x, z, size=(0.18, 0.012, 0.1), r=0.002):
    return box(id, GR, GA, 0.15, list(size), [x, size[1] / 2, z], r if r else None)


def foot_positions():
    return [("left-l", 0.0, 0.55, LL), ("left-r", 0.0, -0.55, LL), ("right-l", 2.6, 0.55, LR), ("right-r", 2.6, -0.55, LR)]


def th(s, sid, of, region):
    s.thof("th-" + sid, of, region)


sc = []
# ---- 1:50
s = Scale(50)
side = {"l": ZL, "r": ZR}
for sn, z in side.items():
    for bn, bone in (("left", LL), ("right", LR)):
        s.b(leg(f"leg-{bn}-{sn}", bone, z))
        s.b(collar(f"leg-{bn}-clamp-{sn}", bone, GA, 0.3, 2.3, 0.1, z, 0.04))
        s.b(revolve(f"leg-{bn}-weld-{sn}", bone, GA, 0.3, "x", [0.276, 0, z], [[0, 0], [0, 0.03], [0.002, 0.0315], [0.006, 0.0315], [0.008, 0.03], [0.008, 0]], role="detail"))
for nm, x, z, bone in foot_positions():
    s.b(plate_box(f"foot-{nm}", x, z))
for sn, z in side.items():
    d = capsule(f"brace-{sn}", BR, GA, 0.45, [0, 0, z], [1.92, 0, z], 0.02)
    d["slender"] = True
    s.b(d)
    for k, u in (("1", 0.005), ("2", 1.915)):
        s.b(box(f"brace-clamp-{sn}{k}", BR, GA, 0.3, [0.05, 0.062, 0.064], [u, 0, z], 0.005, role="detail"))
s.b(revolve("beam", BM, GA, 0.45, "z", [0, 0, -0.6], [[0, 0], [0, 0.05], [1.2, 0.05], [1.2, 0]]))
for bone in (CL, CR):
    z = CZ[bone]
    s.b(revolve(f"hanger-collar-{bone[6:]}", BM, GA, 0.3, "z", [0, 0, z - 0.025], [[0, 0.048], [0, 0.0575], [0.05, 0.0575], [0.05, 0.048], [0, 0.048]], role="detail"))
    tab = [[-0.025, -0.045], [-0.025, -0.085], [-0.015, -0.095], [0.015, -0.095], [0.025, -0.085], [0.025, -0.045]]
    s.b(ext(f"hanger-tab-{bone[6:]}", BM, GA, 0.3, "xy", tab, z - 0.00625, z + 0.00625, holes=[circle_pts(0, -0.07, 0.01, 14)], role="detail"))
    s.b(capsule(f"{bone}-rod", bone, GA, 0.5, [0.07, 0, z], [1.9, 0, z], 0.0055))
    s.b(ellipsoid(f"shackle-{bone[6:]}", ST, GA, 0.3, [0.011, 0.027, 0.011], [0, -0.025, z], role="detail"))
s.b(seat_loft("seat"))


def thermal_from_body(s, skip=()):
    for b in s.body:
        if b["id"] in skip:
            continue
        reg = "seat-rubber" if b["bone"] == ST and b["id"] == "seat" else ("chain-steel" if ("rod" in b["id"] or "shackle" in b["id"]) else "frame-galv")
        s.thof("th-" + b["id"], b["id"], reg)


def supports(s, f, den):
    h = {50: 0.015, 100: 0.015, 200: 0.03, 500: 0.075}[den]
    rr = {50: 0.0075, 100: 0.0075, 200: 0.015, 500: 0.0375}[den]
    bw = 0.06 if den < 500 else 0.09
    for nm, x, z, bone in foot_positions():
        kind = "pin" if nm.startswith("left") else "roller"
        sid = f"support-{kind}-{nm}"
        if kind == "pin":
            d = ext(sid, GR, GA, 0.15, "xy", [[-bw / 2, 0], [bw / 2, 0], [0, -h]], z - 0.05, z + 0.05)
            d["transform"] = {"translation": [x, 0, 0]}
        else:
            d = revolve(sid, GR, GA, 0.15, "z", [x, -rr, z - 0.05], [[0, 0], [0, rr], [0.1, rr], [0.1, 0]])
        s.st(d, bone, "support")


def masses(s, with_seat=True, den=50):
    s.st(ellipsoid("mass-beam", BM, GA, 0.2, [0.04, 0.04, 0.15], [0, 0, 0], role="detail"), BM, "mass")
    rl, rb, rs_ = {500: (0.03, 0.0375, 0.029)}.get(den, (0.02, 0.015, 0.02))
    for sn, z in side.items():
        for bn, bone in (("left", LL), ("right", LR)):
            s.st(ellipsoid(f"mass-leg-{bn}-{sn}", bone, GA, 0.2, [0.12, rl, rl], [1.32, 0, z], role="detail"), bone, "mass")
        s.st(ellipsoid(f"mass-brace-{sn}", BR, GA, 0.2, [0.1, rb, rb], [0.96, 0, z], role="detail"), BR, "mass")
    if with_seat and den < 500:
        s.st(ellipsoid("mass-seat", ST, RB, 0.2, [0.12, rs_, 0.15], [0, -0.065, 0], role="detail"), ST, "mass")


def structure50(s):
    for sn, z in side.items():
        for bn, bone in (("left", LL), ("right", LR)):
            s.st(tube_u(f"leg-{bn}-tube-{sn}", bone, GA, 0.45, 0.04, 2.64, z, 0.0285, 0.0255), bone, "member")
            s.stof(f"joint-leg-{bn}-clamp-{sn}", f"leg-{bn}-clamp-{sn}", bone, "joint")
        s.st(tube_u(f"brace-tube-{sn}", BR, GA, 0.45, 0.0, 1.92, z, 0.0185, 0.016), BR, "member")
        for k in ("1", "2"):
            s.stof(f"joint-brace-clamp-{sn}{k}", f"brace-clamp-{sn}{k}", BR, "joint")
    s.st(revolve("beam-tube", BM, GA, 0.45, "z", [0, 0, -0.598], [[0, 0.0445], [1.196, 0.0445], [1.196, 0.0485], [0, 0.0485], [0, 0.0445]], role="detail"), BM, "member")
    for bone in (CL, CR):
        z = CZ[bone]
        d = capsule(f"{bone}-member", bone, GA, 0.5, [0.07, 0, z], [1.9, 0, z], 0.0039, role="detail")
        d["slender"] = True
        s.st(d, bone, "member")
        s.stof(f"joint-{bone}-hanger", f"hanger-tab-{bone[6:]}", bone, "joint")
        s.stof(f"joint-{bone}-shackle", f"shackle-{bone[6:]}", ST, "joint")
    s.stof("seat-member", "seat", ST, "member")
    supports(s, 0, 50)
    masses(s)


thermal_from_body(s)
structure50(s)
s.dec("tbd", "tbd")
sc.append(s)


def slender_tube(id, bone, u0, u1, z, ro, ri):
    d = tube_u(id, bone, GA, 0.45, u0, u1, z, ro, ri)
    d["slender"] = True
    return d


def structure_coarse(s, den, joints):
    for sn, z in side.items():
        for bn, bone in (("left", LL), ("right", LR)):
            d = slender_tube(f"leg-{bn}-tube-{sn}", bone, 0.04, 2.64, z, 0.0285, 0.0255)
            s.st(d, bone, "member")
            if joints:
                s.st(tube_u(f"joint-leg-{bn}-{sn}", bone, GA, 0.3, 2.3, 2.4, z, 0.0295, 0.0285), bone, "joint")
        s.st(slender_tube(f"brace-tube-{sn}", BR, 0.0, 1.92, z, 0.0185, 0.016), BR, "member")
    s.st(revolve("beam-tube", BM, GA, 0.45, "z", [0, 0, -0.598], [[0, 0.0445], [1.196, 0.0445], [1.196, 0.0485], [0, 0.0485], [0, 0.0445]], role="detail"), BM, "member")
    for bone in (CL, CR):
        z = CZ[bone]
        d = capsule(f"{bone}-member", bone, GA, 0.5, [0.07, 0, z], [1.9, 0, z], 0.0039, role="detail")
        d["slender"] = True
        s.st(d, bone, "member")
    s.stof("seat-member", "seat", ST, "member")
    supports(s, 0, den)
    masses(s, True, den)


def feet(s):
    for nm, x, z, bone in foot_positions():
        s.b(plate_box(f"foot-{nm}", x, z))


def slender_leg(id, bone, z, kind, pt=0.012, rl=0.03, rs=None, slim=True):
    if kind == "loft":
        d = leg(id, bone, z, r=rl, pt=pt, rs=rs)
    else:
        ang = AL if bone == LL else AR_
        d = revolve(id, bone, GA, 0.45, "x", [0.012 / math.sin(ang), 0, z], [[0, 0], [0, rl], [LEN - 0.012 / math.sin(ang), rl], [LEN - 0.012 / math.sin(ang), 0]])
    if slim:
        d["slender"] = True
    return d


def slender_cyl(id, bone, u0, u1, z, r, tone=0.45):
    d = revolve(id, bone, GA, tone, "x", [u0, 0, z], [[0, 0], [0, r], [u1 - u0, r], [u1 - u0, 0]])
    d["slender"] = True
    return d


def chain_ellipse(id, bone, z, rw, rs, u0=0.07, u1=1.9):
    return loft(id, bone, GA, 0.5, [sec([u0, 0, z], rw, rs), sec([u1, 0, z], rw, rs)])


def coarse_body(s, legs, brace, chain, seat, rb=0.02, rc=(0.0055, 0.0055), pt=0.012, rl=0.03, rs=None, rbs=None, slimleg=False, cu=None):
    for sn, z in side.items():
        for bn, bone in (("left", LL), ("right", LR)):
            s.b(slender_leg(f"leg-{bn}-{sn}", bone, z, legs, pt, rl, rs, slimleg))
    feet(s)
    for sn, z in side.items():
        if brace == "ell":
            d = loft(f"brace-{sn}", BR, GA, 0.45, [sec([0, 0, z], rb, rbs), sec([1.92, 0, z], rb, rbs)])
            d["slender"] = True
            s.b(d)
        elif brace == "cap":
            d = capsule(f"brace-{sn}", BR, GA, 0.45, [0, 0, z], [1.92, 0, z], rb)
            d["slender"] = True
            s.b(d)
        else:
            s.b(slender_cyl(f"brace-{sn}", BR, 0.0, 1.92, z, rb))
    s.b(revolve("beam", BM, GA, 0.45, "z", [0, 0, -0.6], [[0, 0], [0, 0.05], [1.2, 0.05], [1.2, 0]]))
    for bone in (CL, CR):
        z = CZ[bone]
        if chain == "cap":
            s.b(capsule(f"{bone}-rod", bone, GA, 0.5, [0.07, 0, z], [1.9, 0, z], 0.0055))
        else:
            s.b(chain_ellipse(f"{bone}-rod", bone, z, rc[0], rc[1], *(cu if cu else (0.07, 1.9))))
    if seat == "box":
        s.b(box("seat", ST, RB, 0.2, [0.42, 0.058, 0.52], [0, -0.069, 0]))
    else:
        s.b(seat_loft("seat", k=3))


# ---- 1:100
s = Scale(100)
coarse_body(s, "loft", "cap", "cap", "loft")
thermal_from_body(s)
structure_coarse(s, 100, True)
s.dec("tbd", "tbd")
sc.append(s)
# ---- 1:200
s = Scale(200)
coarse_body(s, "loft", "cap", "ell", "box", rb=0.013, rc=(0.02, 0.035), pt=0.0, rl=0.029)
thermal_from_body(s)
structure_coarse(s, 200, False)
s.dec("tbd", "tbd")
sc.append(s)
# ---- 1:500
s = Scale(500)
coarse_body(s, "loft", "ell", "ell", "box", rb=0.02, rbs=0.045, rc=(0.015, 0.075), cu=(-0.04, 1.975), pt=0.012, rl=0.03, rs=0.045)
thermal_from_body(s)
structure_coarse(s, 500, False)
s.dec("tbd", "tbd")
sc.append(s)


DEC = {
    50: [
        ("Body: two real A-frames in the planes z = +-0.56 m. Each leg is one loft of a 60 mm tube on its true centre line (u = 2.64 m at 60.5 / 119.5 degrees) whose foot is cut by the horizontal plane of the 12 mm foot plate (a first elliptical section tilted into that plane: the drawn mitre, no wedge gap) and whose head runs into the 100 mm beam; clamp collars with chamfers at u = 2.3 m, weld rings at u = 0.28 m (the drawn weld lines), 180 x 100 x 12 mm foot plates, the 40 mm brace between the leg axes with cast cross-clamp blocks, the beam as a 1.2 m revolve about w with a hanger saddle ring and an eye tab with a real 20 mm hole per chain (the chain top sits in the hole), the 11 mm chains as slender capsules, oval shackles and the rubber seat as a loft of 5 rounded sections (420 x 58 x 520 mm, rounded plank section and corners). The chain links the 2D draws as detail are not solids: a 5.5 mm wire ring is below the 7.5 mm feature.",
         "Körper: zwei echte A-Rahmen in den Ebenen z = +-0,56 m. Jedes Bein ist eine Loft eines 60-mm-Rohres auf seiner wahren Mittellinie (u = 2,64 m bei 60,5 / 119,5 Grad), deren Fuß von der waagerechten Ebene der 12-mm-Fußplatte geschnitten wird (ein erster elliptischer Schnitt in diese Ebene gekippt: der gezeichnete Gehrungsschnitt, keine Keillücke) und dessen Kopf in den 100-mm-Holm läuft; Klemmmanschetten mit Fasen bei u = 2,3 m, Schweißringe bei u = 0,28 m (die gezeichneten Schweißlinien), Fußplatten 180 x 100 x 12 mm, die 40-mm-Strebe zwischen den Beinachsen mit gegossenen Kreuzklemmen, der Holm als 1,2-m-Rotation um w mit Aufhängesattel und Augenlasche mit echtem 20-mm-Loch je Kette (das Kettenende sitzt im Loch), die 11-mm-Ketten als schlanke Kapseln, ovale Schäkel und der Gummisitz als Loft aus 5 gerundeten Schnitten (420 x 58 x 520 mm, gerundeter Brettschnitt und Ecken). Die in 2D als Detail gezeichneten Kettenglieder sind keine Körper: ein Drahtring von 5,5 mm liegt unter der Einzelheit 7,5 mm."),
        ("Thermal: three single-layer regions (B2), all references to body solids: frame-galv = legs, clamps, welds, foot plates, braces, brace clamps, beam, hanger rings and tabs; chain-steel = both chains and shackles; seat-rubber = the seat. No emitters, no hidden regions. Root fix of areaM2 (C16, within 1.5): chain-steel 0.1 -> 0.13 (two 11 mm chains of 1.83 m expose 0.126 m2). Not applied (beyond 1.5, listed in area-review.md): frame-galv declared 1.3 m2, the honestly modelled tubes expose 3.10 m2 (four 60 mm legs of 2.64 m alone are 1.99 m2); seat-rubber declared 0.12 m2, the 420 x 520 mm plank exposes 0.49 m2 (top alone 0.22 m2).",
         "Thermik: drei einschichtige Regionen (B2), alle Referenzen auf Körper: frame-galv = Beine, Manschetten, Schweißringe, Fußplatten, Streben, Strebenklemmen, Holm, Aufhängeringe und Laschen; chain-steel = beide Ketten und Schäkel; seat-rubber = der Sitz. Keine Emitter, keine verdeckten Regionen. Korrektur von areaM2 (C16, innerhalb 1,5): chain-steel 0,1 -> 0,13 (zwei 11-mm-Ketten von 1,83 m haben 0,126 m2). Nicht angewandt (über 1,5, in area-review.md gelistet): frame-galv mit 1,3 m2 angegeben, die ehrlich modellierten Rohre haben 3,10 m2 (vier 60-mm-Beine von 2,64 m allein 1,99 m2); seat-rubber mit 0,12 m2 angegeben, das Brett 420 x 520 mm hat 0,49 m2 (die Oberseite allein 0,22 m2)."),
        ("Structure: every eligible bone is a member on its real line. Legs: 60 x 3 mm tubes (inside the 1.5 mm skin of the body), braces: 40 x 2.5 mm tubes between the leg axes, beam: 100 x 4 mm tube, chains: two slender rods of 7.8 mm (the area of two 5.5 mm wires), seat: the rubber plank itself. Joints: clamp collars and brace clamps (fixed), the eye tabs (chain pivots at the beam) and the shackles (seat pivots). Supports: four anchors at the foot plates, a wedge under each pin foot (pin-left-*) and a 15 mm roller under each roller foot (roller-right-*) with their anchors, below the ground contact plane. Masses: beam (20 kg), the four legs (5.85 kg each in their tubes), the two braces and the seat.",
         "Tragwerk: jede tragende Achse ist ein Stab auf ihrer wirklichen Linie. Beine: Rohre 60 x 3 mm (innerhalb der 1,5-mm-Haut des Körpers), Streben: Rohre 40 x 2,5 mm zwischen den Beinachsen, Holm: Rohr 100 x 4 mm, Ketten: zwei schlanke Stäbe von 7,8 mm (die Fläche zweier 5,5-mm-Drähte), Sitz: das Gummibrett selbst. Gelenke: Klemmmanschetten und Strebenklemmen (fest), die Augenlaschen (Kettendrehpunkte am Holm) und die Schäkel (Sitzdrehpunkte). Auflager: vier Anker an den Fußplatten, je ein Keil unter jedem Festlager (pin-left-*) und eine 15-mm-Rolle unter jedem Rollenlager (roller-right-*) mit ihren Ankern, unter der Aufstandsebene. Massen: Holm (20 kg), die vier Beine (je 5,85 kg in ihren Rohren), die beiden Streben und der Sitz."),
    ],
    100: [
        ("Body: modelled anew against the 1:100 drawing: clamps, weld rings, hangers, eye tabs and shackles (7.5-100 mm detail, not drawn at this scale) are merged into the tubes; the legs keep their 60 mm tube with the mitred foot, the braces are slender 40 mm capsules (the drawing shows a stroke), chains slender 11 mm, the seat a loft of three sections with the drawn rounded plank section, foot plates and beam unchanged.",
         "Körper: neu gegen die Zeichnung 1:100 modelliert: Klemmen, Schweißringe, Aufhänger, Augenlaschen und Schäkel (Details von 7,5-100 mm, in diesem Maßstab nicht gezeichnet) sind in die Rohre aufgegangen; die Beine behalten ihr 60-mm-Rohr mit dem Gehrungsfuß, die Streben sind schlanke 40-mm-Kapseln (die Zeichnung zeigt einen Strich), die Ketten schlank mit 11 mm, der Sitz eine Loft aus drei Schnitten mit dem gezeichneten gerundeten Brettschnitt, Fußplatten und Holm unverändert."),
        ("Thermal: references to the same body solids by region (frame-galv, chain-steel, seat-rubber); areas as at 1:50 (frame 2.90 m2 against the declared 1.3, seat 0.48 against 0.12: see area-review.md, not applied).",
         "Thermik: Referenzen auf dieselben Körper nach Region (frame-galv, chain-steel, seat-rubber); Flächen wie bei 1:50 (Rahmen 2,90 m2 gegen 1,3 angegeben, Sitz 0,48 gegen 0,12: siehe area-review.md, nicht angewandt)."),
        ("Structure: the same true tubes, rods and pivots without the eye-tab and shackle joints (merged into the members), four doubler sleeves as leg-to-beam joints, a 15 mm roller and wedge at each foot, the masses of beam, legs, braces and seat.",
         "Tragwerk: dieselben wahren Rohre, Stäbe und Drehpunkte ohne die Gelenke Augenlasche und Schäkel (in die Stäbe aufgegangen), vier Dopplerhülsen als Gelenke Bein-Holm, je eine 15-mm-Rolle und ein Keil an jedem Fuß, die Massen von Holm, Beinen, Streben und Sitz."),
    ],
    200: [
        ("Body: modelled anew against the 1:200 drawing, which draws the legs, brace and chain as strokes of 0.25 / 0.13 / 0.35 mm pen and the seat as a plain rectangle: legs as 58 mm tubes with the mitred foot (the drawn 60 mm strips, one raster cell thinner), the brace as a slender 26 mm capsule (its drawn pen width, thinner than the real 40 mm), the chains as 40 x 70 mm oval rods (front and plan draw them 40 mm wide, the elevation 70 mm: the two views cannot both be met by one round rod), the seat a box 420 x 58 x 520 mm, plate and beam unchanged.",
         "Körper: neu gegen die Zeichnung 1:200 modelliert, die Beine, Strebe und Kette als Striche mit 0,25 / 0,13 / 0,35 mm Federbreite und den Sitz als glattes Rechteck zeichnet: Beine als 58-mm-Rohre mit Gehrungsfuß (die gezeichneten 60-mm-Streifen, eine Rasterzelle schmaler), die Strebe als schlanke 26-mm-Kapsel (ihre gezeichnete Federbreite, dünner als die wahren 40 mm), die Ketten als ovale Stäbe 40 x 70 mm (Vorderansicht und Grundriss zeichnen sie 40 mm breit, der Aufriss 70 mm: ein runder Stab erfüllt nicht beide Ansichten), der Sitz ein Quader 420 x 58 x 520 mm, Platte und Holm unverändert."),
        ("Thermal: references by region; every region has a solid and only the sum is compared at this scale: the sum of the exposed areas is 2.5 times the declared sum (frame 2.68 against 1.3, seat 0.55 against 0.12, and the 70 mm oval chains 0.65 m2 against 0.13 because the scale draws the chain thicker than it is); not applied, see area-review.md.",
         "Thermik: Referenzen nach Region; jede Region hat einen Körper und in diesem Maßstab zählt nur die Summe: die Summe der freien Flächen ist das 2,5-fache der angegebenen Summe (Rahmen 2,68 gegen 1,3, Sitz 0,55 gegen 0,12 und die ovalen 70-mm-Ketten 0,65 m2 gegen 0,13, weil der Maßstab die Kette dicker zeichnet als sie ist); nicht angewandt, siehe area-review.md."),
        ("Structure: the real tubes (60 x 3 mm legs, 40 x 2.5 mm braces, 100 x 4 mm beam) and the 7.8 mm chain rods inside the thicker body solids, the rubber seat, no joint solids (merged into the members), 30 mm roller and wedge supports, masses as before.",
         "Tragwerk: die wirklichen Rohre (Beine 60 x 3 mm, Streben 40 x 2,5 mm, Holm 100 x 4 mm) und die 7,8-mm-Kettenstäbe innerhalb der dickeren Körper, das Gummibrett, keine Gelenkkörper (in die Stäbe aufgegangen), Auflager mit 30-mm-Rolle und Keil, Massen wie zuvor."),
    ],
    500: [
        ("Body: modelled anew against the 1:500 drawing, whose elevation strokes are 90 mm (legs, brace) and 150 mm (chain) wide while front and plan draw the legs 60 mm and the chains 30 mm: legs and braces are 60 x 90 mm oval tubes (lateral width of front and plan, sagittal width of the elevation stroke), the chains 30 x 150 mm oval bars from the beam centre to 75 mm below the chain end (the stroke caps), seat a box, beam 100 mm, foot plates 180 x 12 x 100 mm. These widths are the drawing's, not the object's; true sizes are in the structure list.",
         "Körper: neu gegen die Zeichnung 1:500 modelliert, deren Aufrissstriche 90 mm (Beine, Strebe) und 150 mm (Kette) breit sind, während Vorderansicht und Grundriss die Beine mit 60 mm und die Ketten mit 30 mm zeichnen: Beine und Streben sind ovale Rohre 60 x 90 mm (seitliche Breite von Vorderansicht und Grundriss, Tiefe des Aufrissstrichs), die Ketten ovale Stäbe 30 x 150 mm von der Holmmitte bis 75 mm unter das Kettenende (die Strichkappen), der Sitz ein Quader, der Holm 100 mm, die Fußplatten 180 x 12 x 100 mm. Diese Breiten sind die der Zeichnung, nicht die des Objekts; wahre Größen stehen in der Tragwerksliste."),
        ("Thermal: references by region; the sum of exposed areas is 3.5 times the declared sum (frame 3.71, chain 1.26 for the 150 mm bars against 0.13, seat 0.55 against 0.12): the declared frame and seat areas are mis-estimates (area-review.md) and the scale draws the chain 14 times thicker than it is.",
         "Thermik: Referenzen nach Region; die Summe der freien Flächen ist das 3,5-fache der angegebenen Summe (Rahmen 3,71, Kette 1,26 für die 150-mm-Stäbe gegen 0,13, Sitz 0,55 gegen 0,12): die angegebenen Flächen von Rahmen und Sitz sind Fehlschätzungen (area-review.md), und der Maßstab zeichnet die Kette 14 mal dicker als sie ist."),
        ("Structure: true tubes at true size (60 x 3 mm legs, 40 x 2.5 mm braces: slender, so exempt from the 75 mm feature), 100 x 4 mm beam tube, 7.8 mm chain rods, the rubber seat as member, 75 mm contact pads, masses of beam, legs and braces (the seat mass of 2.8 kg is below 10 % and merged into the seat member); the leg masses are 60 mm ellipsoids (the feature limit of the leg drawing) inside the 60 x 90 mm body legs.",
         "Tragwerk: wahre Rohre in wahrer Größe (Beine 60 x 3 mm, Streben 40 x 2,5 mm: schlank, daher von der Einzelheit 75 mm ausgenommen), Holmrohr 100 x 4 mm, 7,8-mm-Kettenstäbe, das Gummibrett als Stab, Aufstandsplatten 75 mm, Massen von Holm, Beinen und Streben (die Sitzmasse von 2,8 kg liegt unter 10 % und ist im Sitzstab aufgegangen); die Beinmassen sind 60-mm-Ellipsoide (die Grenze der Beinzeichnung) innerhalb der Körperbeine 60 x 90 mm."),
    ],
}
for s_ in sc:
    s_.decisions = []
    for en, de in DEC[s_.den]:
        s_.dec(en, de)
save_asset("playground-swing", sc)
