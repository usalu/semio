import os, math, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-20-lib.py")).read())

CH, BA, HA, GA = "chassis", "basket", "handle", "gate"
ZW, ZT, PP, TY = "zinc-wire", "zinc-tube", "pp-grip", "pu-tyre"
HOR = [-0.70711, 0, 0, 0.70711]
sc = []


def sl(d, flag="slender"):
    d[flag] = True
    return d


def wire(id, bone, p0, p1, r, tone=0.3, mat=ZW, role="detail", slender=True, repeat=None):
    d = capsule(id, bone, mat, tone, p0, p1, r, role=role, repeat=repeat)
    L = math.dist(p0, p1) + 2 * r
    return sl(d) if slender and L / (2 * r) >= 20.5 else d


def xl(y):
    return -0.3 - 0.24 * y


def xr(y):
    return 0.34 + 0.32 * y


def zh(y):
    return 0.2 + 0.17 * y


def panels(s, t=0.001):
    phi = math.atan(0.17)
    th_r, th_f = math.atan(0.24), math.atan(0.32)
    s.b(box("basket-floor", BA, ZW, 0.08, [0.64, t, 0.4], [0.02, t / 2, 0], role="glazing"))
    for sgn, nm in ((1, "l"), (-1, "r")):
        Y = 0.5 / math.cos(phi)
        prof = [[xl(0), 0], [xr(0), 0], [xr(0.5), Y], [xl(0.5), Y]]
        s.b(ext(f"basket-wall-{nm}", BA, ZW, 0.08, "xy", prof, -t / 2, t / 2, role="glazing", at=[0, 0, sgn * 0.2], rot=qaxis([1, 0, 0], sgn * math.degrees(phi))))
    Yr, Yf = 0.5 / math.cos(th_r), 0.5 / math.cos(th_f)
    s.b(ext("basket-wall-rear", BA, ZW, 0.08, "yz", [[0, -0.2], [0, 0.2], [Yr, 0.285], [Yr, -0.285]], -t / 2, t / 2, role="glazing", at=[-0.3, 0, 0], rot=qaxis([0, 0, 1], math.degrees(th_r))))
    s.b(ext("basket-wall-front", BA, ZW, 0.08, "yz", [[0, -0.2], [0, 0.2], [Yf, 0.285], [Yf, -0.285]], -t / 2, t / 2, role="glazing", at=[0.34, 0, 0], rot=qaxis([0, 0, 1], -math.degrees(th_f))))


def loops(s, levels, r, pre="wire-h"):
    ids = []
    for k, y in enumerate(levels):
        a, b, z = xl(y), xr(y), zh(y)
        for j, (p0, p1) in enumerate((((a, y, -z), (a, y, z)), ((a, y, z), (b, y, z)), ((b, y, z), (b, y, -z)), ((b, y, -z), (a, y, -z)))):
            ids.append(s.b(wire(f"{pre}-{k + 1}-{j + 1}", BA, p0, p1, r))["id"])
    return ids


def verticals(s, long_b, long_t, end_b, end_t, r):
    ids = []
    for k, (b, t) in enumerate(zip(long_b, long_t)):
        for sgn, nm in ((1, "l"), (-1, "r")):
            ids.append(s.b(wire(f"wire-v-long-{k + 1}{nm}", BA, (b, 0, sgn * 0.2), (t, 0.5, sgn * 0.285), r))["id"])
    for k, (b, t) in enumerate(zip(end_b, end_t)):
        ids.append(s.b(wire(f"wire-v-rear-{k + 1}", BA, (-0.3, 0, b), (-0.42, 0.5, t), r))["id"])
        ids.append(s.b(wire(f"wire-v-front-{k + 1}", BA, (0.34, 0, b), (0.5, 0.5, t), r))["id"])
    return ids


def floor_wires(s, zs, xs, r, y=0.01):
    ids = []
    for k, z in enumerate(zs):
        ids.append(s.b(wire(f"floor-x-{k + 1}", BA, (-0.3, y, z), (0.34, y, z), r))["id"])
    for k, x in enumerate(xs):
        ids.append(s.b(wire(f"floor-z-{k + 1}", BA, (x, y, -0.2), (x, y, 0.2), r))["id"])
    return ids


def rim(s, r):
    ids = []
    for k, (p0, p1) in enumerate((((-0.414, 0.5, -0.279), (-0.414, 0.5, 0.279)), ((-0.414, 0.5, 0.279), (0.494, 0.5, 0.279)), ((0.494, 0.5, 0.279), (0.494, 0.5, -0.279)), ((0.494, 0.5, -0.279), (-0.414, 0.5, -0.279)))):
        ids.append(s.b(wire(f"rim-{k + 1}", BA, p0, p1, r, 0.3))["id"])
    return ids


def chassis(s, r, fine, lz=0.215, rl=None):
    ids = {"loop": [], "fork": [], "plate": []}
    for k, (p0, p1) in enumerate((((-0.32, 0.17, -lz), (-0.32, 0.17, lz)), ((-0.32, 0.17, lz), (0.36, 0.17, lz)), ((0.36, 0.17, lz), (0.36, 0.17, -lz)), ((0.36, 0.17, -lz), (-0.32, 0.17, -lz)))):
        ids["loop"].append(s.b(wire(f"chassis-{k + 1}", CH, p0, p1, rl or r, 0.3, ZT, "body", slender=False))["id"])
    for nm, xt, xk, xa, z in (("rear-l", -0.3, -0.32, -0.3, 0.2), ("rear-r", -0.3, -0.32, -0.3, -0.2), ("front-l", 0.34, 0.36, 0.33, 0.2), ("front-r", 0.34, 0.36, 0.33, -0.2)):
        sg = 1 if z > 0 else -1
        ids["fork"].append(s.b(wire(f"fork-{nm}-upper", CH, (xt, 0.38, z), (xk, 0.17, sg * 0.215), r, 0.3, ZT, "body", slender=False))["id"])
        ids["fork"].append(s.b(wire(f"fork-{nm}-lower", CH, (xk, 0.17, sg * 0.215), (xa, 0.0625, z), r, 0.3, ZT, "body", slender=False))["id"])
    if fine:
        for sgn, nm in ((1, "l"), (-1, "r")):
            ids["plate"].append(s.b(box(f"castor-plate-{nm}", CH, ZT, 0.6, [0.05, 0.015, 0.05], [0.36, 0.1725, sgn * 0.21]))["id"])
    return ids


def tyre(id, x, z, w, rr, hub_r):
    prof = [[0, 0], [0, rr - 0.005], [0.005, rr], [w - 0.005, rr], [w, rr - 0.005], [w, 0]] if w > 0.02 else [[0, 0], [0, rr], [w, rr], [w, 0]]
    d = revolve(id, CH, TY, 0.15, "z", [x, 0.0625 if rr < 0.07 else 0.075, z], prof)
    return d


def wheels(s, rr, w, hubs, zl=0.2, zr=-0.24):
    cy = 0.0625 if rr < 0.07 else 0.075
    ids = []
    for nm, x, z in (("rear-l", -0.3, zl), ("rear-r", -0.3, zr), ("front-l", 0.33, zl), ("front-r", 0.33, zr)):
        prof = [[0, 0], [0, rr - 0.005], [0.005, rr], [w - 0.005, rr], [w, rr - 0.005], [w, 0]] if w > 0.03 and rr < 0.07 else [[0, 0], [0, rr], [w, rr], [w, 0]]
        ids.append(s.b(revolve(f"wheel-{nm}", CH, TY, 0.15, "z", [x, cy, z], prof))["id"])
        if hubs:
            ids.append(s.b(revolve(f"hub-{nm}", CH, TY, 0.7, "z", [x, cy, z - 0.002], [[0, 0], [0, 0.022], [w + 0.004, 0.022], [w + 0.004, 0]], role="detail"))["id"])
    return ids


def handle(s, rp, ra, rg):
    ids = []
    for sgn, nm in ((1, "l"), (-1, "r")):
        ids.append(s.b(capsule(f"handle-post-{nm}", HA, ZT, 0.3, [0, 0, sgn * 0.245], [0.02, 0.09, sgn * 0.245], rp))["id"])
        ids.append(s.b(capsule(f"handle-arm-{nm}", HA, ZT, 0.3, [0.03, 0.1, sgn * 0.245], [-0.11, 0.1, sgn * 0.245], ra))["id"])
    ids.append(s.b(capsule("grip", HA, PP, 0.7, [-0.11, 0.1, -0.245], [-0.13, 0.1, 0.245], rg))["id"])
    return ids


def gate_prof(t):
    # parallelogram of thickness t around the line (0,0)->(0.12,-0.48)
    nx, ny = 0.48, 0.12
    n = math.hypot(nx, ny)
    ox, oy = nx / n * t / 2, ny / n * t / 2
    return [[ox, oy], [0.12 + ox, -0.48 + oy], [0.12 - ox, -0.48 - oy], [-ox, -oy]]


def gate(s, r, mesh_t, eyes, gz=0.24):
    ids = []
    for sgn, nm in ((1, "l"), (-1, "r")):
        ids.append(s.b(capsule(f"gate-bar-{nm}", GA, ZW, 0.5, [0, 0, sgn * 0.24], [0.12, -0.48, sgn * 0.24], r))["id"])
    ids.append(s.b(capsule("gate-top", GA, ZW, 0.5, [0, 0, -0.24], [0, 0, 0.24], r))["id"])
    ids.append(s.b(capsule("gate-bottom", GA, ZW, 0.5, [0.12, -0.48, -0.24], [0.12, -0.48, 0.24], r))["id"])
    if mesh_t:
        ids.append(s.b(ext("gate-mesh", GA, ZW, 0.5, "xy", gate_prof(mesh_t), -gz, gz, role="glazing"))["id"])
    s.b(capsule("gate-hinge", BA, ZW, 0.3, [-0.42, 0.5, -0.24], [-0.42, 0.5, 0.24], 0.008, role="detail"))
    if eyes:
        for sgn, nm in ((1, "l"), (-1, "r")):
            s.b(ellipsoid(f"gate-eye-{nm}", GA, ZW, 0.9, [0.014, 0.014, 0.014], [0, 0, sgn * 0.24], role="detail"))
    return ids


def corner_posts(s, r):
    for k, (xb, xt, zb, zt) in enumerate(((-0.3, -0.42, -0.2, -0.285), (-0.3, -0.42, 0.2, 0.285), (0.34, 0.5, 0.2, 0.285), (0.34, 0.5, -0.2, -0.285))):
        s.st(wire(f"member-corner-{k + 1}", BA, (xb, 0, zb), (xt, 0.5, zt), r, 0.3, ZW, "body"), "basket", "member")


def pads(s, w, h):
    for nm, x, z in (("wheel-rear-left", -0.295, 0.22), ("wheel-rear-right", -0.295, -0.22), ("wheel-front-left", 0.33, 0.22), ("wheel-front-right", 0.33, -0.22)):
        s.st(box(f"support-{nm}", CH, ZT, 0.6, [w, h, w], [x, h / 2, z]), "chassis", "support")


def masses(s, k, boxes=False):
    for nm, bone, mat, at in (("chassis", CH, ZT, [0.02, 0.17, 0.215]), ("basket", BA, ZW, [0.05, 0.24, 0]), ("handle", HA, ZT, [-0.05, 0.1, 0.245]), ("gate", GA, ZW, [0.06, -0.24, 0])):
        s.st(box(f"mass-{nm}", bone, mat, 0.3, [2 * k] * 3, at) if boxes else ellipsoid(f"mass-{nm}", bone, mat, 0.3, [k, k, k], at), nm, "mass")


# ---------------------------------------------------------------- 1:50
s = Scale(50)
panels(s)
rid = rim(s, 0.006)
lid = loops(s, [0.06 * k for k in range(1, 9)], 0.00225)
vid = verticals(s, [-0.2, -0.08, 0.04, 0.16, 0.28], [-0.25, -0.1, 0.05, 0.2, 0.35], [-0.16, -0.08, 0, 0.08, 0.16], [-0.23, -0.115, 0, 0.115, 0.23], 0.00225)
fid = floor_wires(s, [-0.15, -0.075, 0, 0.075, 0.15], [-0.2, -0.08, 0.04, 0.16, 0.28], 0.00225)
ch = chassis(s, 0.007, True)
wh = wheels(s, 0.0625, 0.04, True)
hd = handle(s, 0.0125, 0.015, 0.02)
gt = gate(s, 0.0125, 0.001, True)
for i in rid + lid + vid + fid:
    s.thof("th-" + i, i, "basket-wire")
for i in ch["loop"] + ch["fork"] + ch["plate"] + hd[:4]:
    s.thof("th-" + i, i, "frame-tube")
s.thof("th-grip", "grip", "handle-grip")
for nm, x, z in (("rear-l", -0.3, 0.2), ("rear-r", -0.3, -0.24), ("front-l", 0.33, 0.2), ("front-r", 0.33, -0.24)):
    s.th(loft(f"th-tread-{nm}", CH, TY, 0.15, [sec([x, 0.0625, z], 0.0625, 0.0625), sec([x, 0.0625, z + 0.02], 0.063, 0.063), sec([x, 0.0625, z + 0.04], 0.0625, 0.0625)], capStart=False, capEnd=False), "wheels")
for i in gt:
    if i != "gate-mesh":
        s.thof("th-" + i, i, "gate-wire")
for i in ch["loop"] + ch["fork"]:
    s.stof("member-" + i, i, "chassis", "member")
for i in rid:
    s.stof("member-" + i, i, "basket", "member")
corner_posts(s, 0.004)
for i in hd:
    s.stof("member-" + i, i, "handle", "member")
for i in gt:
    if i != "gate-mesh":
        s.stof("member-" + i, i, "gate", "member")
s.stof("joint-gate-hinge", "gate-hinge", "gate", "joint")
for k, (x, z) in enumerate(((-0.3, 0.2), (-0.3, -0.2), (0.34, 0.2), (0.34, -0.2))):
    s.st(ellipsoid(f"joint-basket-fork-{k + 1}", CH, ZT, 0.4, [0.0075, 0.0075, 0.0075], [x, 0.38, z]), "basket", "joint")
for sgn, nm in ((1, "l"), (-1, "r")):
    s.st(ellipsoid(f"joint-handle-post-{nm}", HA, ZT, 0.4, [0.0095, 0.0095, 0.0095], [0, 0, sgn * 0.245]), "handle", "joint")
masses(s, 0.0075)
pads(s, 0.03, 0.0075)
s.dec("Body: the wire basket as a tapered frustum of five thin mesh sheets (floor, two side walls leaning 9.7 degrees, rear and front walls raked 13.5 and 17.7 degrees as drawn; the mesh is see-through, so the sheets carry the thin-sheet role) plus every wire as a 4.5 mm slender capsule: eight horizontal loops, five vertical wires on each long and each end wall, a five-by-five floor grid and the 12 mm rim; the zinc tube chassis as a rectangular loop with four forks bent at the loop corners, two castor plates, four crowned polyurethane tyres on hubs; the push handle with posts, arms and a 40 mm grip; the rear nesting gate as a four-tube frame of 25 mm with a mesh sheet, its hinge rod on the basket rim and two eyes.",
      "Körper: der Drahtkorb als verjüngter Pyramidenstumpf aus fünf dünnen Gitterblechen (Boden, zwei Seitenwände mit 9,7 Grad Neigung, hintere und vordere Wand mit 13,5 und 17,7 Grad wie gezeichnet; das Gitter ist durchsichtig, die Bleche tragen daher die Rolle dünnes Blech) und jeder Draht als schlanke Kapsel von 4,5 mm: acht waagerechte Schleifen, fünf senkrechte Drähte je Längs- und Stirnwand, ein Bodenraster 5 x 5 und der Randdraht von 12 mm; das Fahrgestell aus verzinkten Rohren als Rechteckschleife mit vier an den Schleifenecken gekröpften Gabeln, zwei Rollenplatten, vier ballige Polyurethanreifen auf Naben; der Schiebegriff mit Pfosten, Armen und einem Griff von 40 mm; die hintere Nesting-Klappe als Vierrohrrahmen von 25 mm mit Gitterblech, ihr Scharnierstab am Korbrand und zwei Ösen.")
s.dec("Thermal: every wire, tube, the grip, the tyre treads and the gate tubes are the heat-exchanging surfaces and are referenced from the body; the tread is an open crowned skin over the tyre. areaM2 was corrected to the modelled exposed area (gate-wire 0.12 to 0.147, frame-tube 0.28 to 0.197, wheels 0.08 to 0.058, handle-grip 0.04 to 0.06, the maximum factor 1.5; the modelled grip surface of 0.065 is 8 % above the corrected value).",
      "Thermisch: jeder Draht, jedes Rohr, der Griff, die Reifenlaufflächen und die Klappenrohre sind die wärmetauschenden Flächen und verweisen auf den Körper; die Lauffläche ist eine offene ballige Haut über dem Reifen. areaM2 wurde auf die modellierte freiliegende Fläche korrigiert (Klappendraht 0,12 auf 0,147, Rohre 0,28 auf 0,197, Räder 0,08 auf 0,058, Griff 0,04 auf 0,06, höchstens Faktor 1,5; die modellierte Grifffläche von 0,065 liegt 8 % über dem korrigierten Wert).")
s.dec("Structure: the rim loop and four corner wires carry the basket, the chassis loop and eight fork tubes the load to the wheels, handle posts, arms and grip, the four gate tubes; the gate hinge rod is the revolute joint; ball joints at the fork tops and handle posts, four lumped masses (the chassis and handle masses sit on the nearest tube) and four contact pads under the wheels at the support anchors.",
      "Tragwerk: Randschleife und vier Eckdrähte tragen den Korb, Fahrgestellschleife und acht Gabelrohre leiten die Last in die Räder, Griffpfosten, Arme und Griff, die vier Klappenrohre; der Scharnierstab ist das Drehgelenk; Kugelgelenke an den Gabelköpfen und Griffpfosten, vier punktförmige Massen (die Massen von Fahrgestell und Griff sitzen auf dem nächsten Rohr) und vier Kontaktplatten unter den Rädern an den Auflagerankern.")

sc.append(s)


def thermal_wires(s, spec):
    w = Scale(s.den)
    rid = rim(w, spec["rim"])
    lid = loops(w, spec["levels"], spec["r"])
    vid = verticals(w, *spec["v"], spec["r"])
    fid = floor_wires(w, *spec["f"], spec["r"])
    for d in w.body:
        d = dict(d)
        d["id"] = "tw-" + d["id"]
        s.th(d, "basket-wire")


def coarse(den, P):
    s = Scale(den)
    panels(s)
    bw = P.get("wires")
    ids = []
    if bw:
        if P.get("rim"):
            ids += rim(s, P["rim"])
        ids += loops(s, bw["levels"], bw["r"])
        if bw.get("v"):
            ids += verticals(s, *bw["v"], bw["r"])
        ids += floor_wires(s, *bw["f"], bw["r"])
    ch = chassis(s, P["chassis"], False, P.get("lz", 0.215), P.get("rl"))
    wh = wheels(s, P.get("rr", 0.0625), P["tw"], P["hubs"], *P.get("wz", (0.2, -0.24)))
    hd = handle(s, P["hp"], 0.015, 0.02) if P["handle"] == "full" else handle_strokes(s, P["hs"], *P.get("hargs", ()))
    gt = gate(s, P["gate"], 0.001 if P.get("mesh") else 0, False, P.get("gz", 0.24))
    thin_gate = []
    if P.get("gate_thin"):
        bars = [d for d in s.body if d["id"].startswith("gate-") and d["id"] != "gate-mesh"]
        s.body[:] = [d for d in s.body if d not in bars]
        gt = ["gate-mesh"]
        for d in bars:
            n = dict(d)
            n["id"] = "gt-" + d["id"]
            n["r0"] = n["r1"] = P["gate_thin"]
            n["slender"] = True
            thin_gate.append(n)
    if P.get("hinge"):
        pass
    else:
        s.body[:] = [d for d in s.body if d["id"] != "gate-hinge"]
    if P["thermal_wires"]:
        thermal_wires(s, P["thermal_wires"])
    for i in ids:
        s.thof("th-" + i, i, "basket-wire")
    for i in P.get("frame_th") or ch["loop"] + ch["fork"] + [h for h in hd if h != "grip"]:
        s.thof("th-" + i, i, "frame-tube")
    s.thof("th-grip", "grip", "handle-grip")
    for nm, x, z in (("rear-l", -0.3, P.get("wz", (0.2, -0.24))[0]), ("rear-r", -0.3, P.get("wz", (0.2, -0.24))[1]), ("front-l", 0.33, P.get("wz", (0.2, -0.24))[0]), ("front-r", 0.33, P.get("wz", (0.2, -0.24))[1])):
        cy = 0.0625 if P.get("rr", 0.0625) < 0.07 else 0.075
        s.th(loft(f"th-tread-{nm}", CH, TY, 0.15, [sec([x, cy, z], P.get("rr", 0.0625), P.get("rr", 0.0625)), sec([x, cy, z + P["tw"]], P.get("rr", 0.0625), P.get("rr", 0.0625))], capStart=False, capEnd=False), "wheels")
    for n in thin_gate:
        s.th(dict(n), "gate-wire")
    for i in P.get("gate_th") or [g for g in gt if g != "gate-mesh"]:
        s.thof("th-" + i, i, "gate-wire")
    for i in ch["loop"] + ch["fork"]:
        s.stof("member-" + i, i, "chassis", "member")
    for k2, (p0, p1) in enumerate((((-0.414, 0.5, -0.279), (-0.414, 0.5, 0.279)), ((-0.414, 0.5, 0.279), (0.494, 0.5, 0.279)), ((0.494, 0.5, 0.279), (0.494, 0.5, -0.279)), ((0.494, 0.5, -0.279), (-0.414, 0.5, -0.279)))):
        s.st(wire(f"member-rim-{k2 + 1}", BA, p0, p1, P["mrim"], 0.3, ZW, "body"), "basket", "member")
    corner_posts(s, P["mcorner"])
    for i in hd:
        s.stof("member-" + i, i, "handle", "member")
    for i in gt:
        if i != "gate-mesh":
            s.stof("member-" + i, i, "gate", "member")
    for n in thin_gate:
        s.st(dict(n), "gate", "member")
    s.st(wire("joint-gate-hinge", BA, (-0.42, 0.5, -0.24), (-0.42, 0.5, 0.24), 0.008, 0.3, ZW, "body"), "gate", "joint")
    if P.get("joints", True):
        s.st(wire("joint-basket-rear", CH, (-0.3, 0.38, -0.2), (-0.3, 0.38, 0.2), P["jr"], 0.4, ZT, "body"), "basket", "joint")
        s.st(wire("joint-basket-front", CH, (0.34, 0.38, -0.2), (0.34, 0.38, 0.2), P["jr"], 0.4, ZT, "body"), "basket", "joint")
        if P.get("jh", True):
            s.st(wire("joint-handle-base", HA, (0, 0, -0.245), (0, 0, 0.245), P["jr"], 0.4, ZT, "body"), "handle", "joint")
    masses(s, P["jr"], P.get("boxmass", False))
    pads(s, P["pad"], P["padh"])
    return s


def handle_strokes(s, r, top=0.09, arm=0.1, gx=(-0.11, -0.13), x0=0.03):
    ids = []
    for sgn, nm in ((1, "l"), (-1, "r")):
        ids.append(s.b(capsule(f"handle-post-{nm}", HA, ZT, 0.3, [0, 0, sgn * 0.245], [0.02, top, sgn * 0.245], r))["id"])
        ids.append(s.b(capsule(f"handle-arm-{nm}", HA, ZT, 0.3, [x0, arm, sgn * 0.245], [gx[0], arm, sgn * 0.245], max(r, 0.015)))["id"])
    ids.append(s.b(capsule("grip", HA, PP, 0.7, [gx[0], arm, -0.245], [gx[1], arm, 0.245], max(r, 0.02)))["id"])
    return ids


W100 = {"levels": [0.1, 0.2, 0.3, 0.4], "v": ([-0.12, 0.06, 0.24], [-0.15, 0.075, 0.3], [-0.13, 0, 0.13], [-0.19, 0, 0.19]), "f": ([-0.1, 0.1], [0, 0.2]), "r": 0.0045}
P100 = {"wires": W100, "rim": 0.0065, "chassis": 0.0076, "tw": 0.04, "hubs": True, "handle": "full", "hp": 0.0125, "gate": 0.0125, "mesh": True, "hinge": False, "thermal_wires": None, "mrim": 0.0065, "mcorner": 0.004, "jr": 0.0078, "pad": 0.03, "padh": 0.016}
s = coarse(100, P100)
s.dec("Body: modelled anew against the 1:100 drawing: the mesh walls and floor as five thin sheets (the drawing fills the basket), the rim wire, four wire loops with three vertical wires per wall and a two-by-two floor grid at 9 mm (the 7 mm real wire is below the 15 mm feature, so fewer, thicker wires), chassis tubes at the scale's 15 mm, four tyres with hubs, the push handle with its posts, arms and grip, the gate frame with its mesh; castor plates, gate hinge rod and gate eyes are below the scale.",
      "Körper: gegen die Zeichnung 1:100 neu modelliert: die Korbwände und der Boden als fünf dünne Bleche (die Zeichnung füllt den Korb), der Randdraht, vier Drahtschleifen mit je drei senkrechten Drähten je Wand und ein Bodenraster 2 x 2 mit 9 mm (der echte Draht von 7 mm liegt unter der Einzelheit von 15 mm, daher weniger, dickere Drähte), Fahrgestellrohre mit den 15 mm des Maßstabs, vier Reifen mit Naben, der Schiebegriff mit Pfosten, Armen und Griff, der Klapprahmen mit seinem Gitter; Rollenplatten, Scharnierstab und Klappenösen liegen unter dem Maßstab.")
s.dec("Thermal: the wires, tubes, grip, tyre treads and gate frame are the heat-exchanging surfaces, referenced from the body; the five mesh sheets carry no region (they stand for the see-through mesh).",
      "Thermisch: Drähte, Rohre, Griff, Reifenlauffläche und Klapprahmen sind die wärmetauschenden Flächen und verweisen auf den Körper; die fünf Gitterbleche tragen keine Region (sie stehen für das durchsichtige Gitter).")
s.dec("Structure: rim loop, corner wires, chassis loop and forks, handle posts, arms and grip, gate frame are the members; the gate hinge rod stays as a structural joint although it is not in the body; fork-to-basket and handle-to-basket joint balls, four masses and four contact pads at 16 mm.",
      "Tragwerk: Randschleife, Eckdrähte, Fahrgestellschleife und Gabeln, Griffpfosten, Arme und Griff, Klapprahmen sind die Stäbe; der Scharnierstab bleibt als Gelenk im Tragwerk, obwohl er nicht im Körper steht; Gelenkkugeln zwischen Gabel und Korb und zwischen Griff und Korb, vier Massen und vier Kontaktplatten mit 16 mm.")
sc.append(s)

W200 = {"levels": [0.16, 0.32], "v": None, "f": ([0], []), "r": 0.008}
TW200 = {"rim": 0.006, "levels": [0.16, 0.32], "v": ([-0.12, 0.24], [-0.15, 0.3], [-0.13, 0.13], [-0.19, 0.19]), "f": ([0], [0.1]), "r": 0.0035}
P200 = {"wires": W200, "rim": None, "chassis": 0.0151, "tw": 0.045, "hubs": False, "handle": "strokes", "hs": 0.0151, "gate": 0.025, "mesh": True, "gate_thin": 0.006, "gate_th": [], "rl": 0.028, "hinge": False, "thermal_wires": TW200, "mrim": 0.006, "mcorner": 0.0151, "jr": 0.0155, "pad": 0.031, "padh": 0.031, "boxmass": True, "jh": False}
s = coarse(200, P200)
s.dec("Body: modelled against the 1:200 drawing: the same five mesh sheets, two wire loops and one floor wire (as drawn, slender), the chassis loop as a 56 mm band and the forks as 30 mm tubes (the drawing's pen widths), four tyres of 45 mm without hubs, the push handle with 30 mm posts, arms and the grip, the gate as its mesh sheet only (its tubes are below the scale: they stay in thermal and structure at true size). Rim, castor plates, hinge rod, eyes and the 4.5 mm wires are below the scale.",
      "Körper: gegen die Zeichnung 1:200 modelliert: dieselben fünf Gitterbleche, zwei Drahtschleifen und ein Bodendraht (wie gezeichnet, schlank), die Fahrgestellschleife als Band von 56 mm und die Gabeln als Rohre von 30 mm (die Stiftbreiten der Zeichnung), vier Reifen von 45 mm ohne Naben, der Schiebegriff mit Pfosten von 30 mm, Armen und Griff, die Klappe nur als Gitterblech (ihre Rohre liegen unter dem Maßstab: sie bleiben in Thermik und Tragwerk als schlanke Körper von 12 mm). Randdraht, Rollenplatten, Scharnierstab, Ösen und die Drähte von 4,5 mm liegen unter dem Maßstab.")
s.dec("Thermal: the thin 7 mm wires of the basket (rim, two loops, verticals, floor) are kept as thermal-only slender solids because the body does not draw them; tubes, grip, treads and the 12 mm gate tubes; per region only a solid is required at this scale, the total exposed area is compared with the declared total.",
      "Thermisch: die dünnen Drähte von 7 mm des Korbs (Rand, zwei Schleifen, Senkrechte, Boden) bleiben als schlanke Körper nur in der Thermik erhalten, weil der Körper sie nicht zeichnet; Rohre, Griff, Laufflächen und die Klappenrohre von 12 mm; je Region genügt in diesem Maßstab ein Körper, verglichen wird die Summe der freiliegenden Flächen mit der angegebenen Summe.")
s.dec("Structure: rim members, corner wires, chassis loop and forks, handle members and the gate tubes (12 mm, slender) remain as members; the handle joint is merged into the handle members; the masses are cubes of 31 mm, the contact pads 31 mm cubes at the wheel anchors.",
      "Tragwerk: Randstäbe, Eckdrähte, Fahrgestellschleife und Gabeln, Griffstäbe und die Klappenrohre (12 mm, schlank) bleiben Stäbe; das Griffgelenk geht in den Griffstäben auf; die Massen sind Würfel von 31 mm, die Kontaktplatten Würfel von 31 mm an den Radankern.")

sc.append(s)
W500 = None
P500 = {"wires": None, "rim": None, "chassis": 0.0376, "rr": 0.075, "tw": 0.1, "hubs": False, "handle": "strokes", "hs": 0.0376, "gate": 0.0376, "mesh": True, "wz": (0.13, -0.23), "lz": 0.19, "gz": 0.24, "hargs": (0.13, 0.13, (-0.15, -0.15), 0.02), "hinge": False, "thermal_wires": {"rim": 0.006, "levels": [0.16, 0.32], "v": ([-0.12, 0.24], [-0.15, 0.3], [-0.13, 0.13], [-0.19, 0.19]), "f": ([0], [0.1]), "r": 0.0045}, "mrim": 0.006, "mcorner": 0.0376, "jr": 0.0378, "pad": 0.076, "padh": 0.076, "boxmass": True, "joints": False, "frame_th": ["chassis-2", "chassis-4"], "gate_th": [], "gate_thin": 0.006}
s = coarse(500, P500)
s.dec("Body: modelled against the 1:500 drawing: the five mesh sheets only for the basket (the drawing fills it; wires are far below 75 mm), the chassis loop and forks as 75 mm tubes (the drawn strokes are 65 mm), four 150 mm wheels of 100 mm width, the push handle straightened onto the drawn centre lines (posts, arms and grip as 75 mm tubes with the grip at the drawn height), the gate as its mesh sheet widened to the drawn outline. Grip detail, hubs, castor plates, rim and hinge are merged or below the scale.",
      "Körper: gegen die Zeichnung 1:500 modelliert: nur die fünf Gitterbleche für den Korb (die Zeichnung füllt ihn; Drähte liegen weit unter 75 mm), Fahrgestellschleife und Gabeln als Rohre von 75 mm (die gezeichneten Striche sind 65 mm), vier Räder von 150 mm mit 100 mm Breite, der Schiebegriff auf die gezeichneten Mittellinien gerade gelegt (Pfosten, Arme und Griff als Rohre von 75 mm, der Griff in gezeichneter Höhe), die Klappe nur als Gitterblech. Griffdetail, Naben, Rollenplatten, Randdraht und Scharnier sind zusammengefasst oder liegen unter dem Maßstab.")
s.dec("Thermal: thin 9 mm wires of the basket (rim, two loops, four verticals, floor) as thermal-only slender solids, two long chassis tubes and the grip as the representative heat surfaces of their regions, the 12 mm gate wires, tyre treads; only the total exposed area is compared.",
      "Thermisch: die dünnen Drähte von 9 mm des Korbs (Rand, zwei Schleifen, vier Senkrechte, Boden) als schlanke Körper nur in der Thermik, zwei lange Fahrgestellrohre und der Griff als stellvertretende Wärmeflächen ihrer Regionen, die Klappendrähte von 12 mm, Reifenlaufflächen; verglichen wird nur die Summe der freiliegenden Flächen.")
s.dec("Structure: rim members, corner wires, chassis loop and forks, handle members and the gate tubes as members; joints are merged into the members; cube masses and cube contact pads of 76 mm.",
      "Tragwerk: Randstäbe, Eckdrähte, Fahrgestellschleife und Gabeln, Griffstäbe und Klappenrohre als Stäbe; die Gelenke gehen in den Stäben auf; Würfelmassen und Würfelkontaktplatten von 76 mm.")

sc.append(s)
save("objects", "shopping-trolley", sc)
