import importlib.util, os, math, json, glob
here = os.path.dirname(os.path.abspath(__file__))
os.environ.setdefault("FRAMES", os.path.join(here, "🗑️generated", "author-35", "pumpjack.frames.json"))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-04-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("oil-pumpjack")
F = V.frames
ST = dict(material="painted-steel", role="body")
CI = dict(material="cast-iron", role="body")
CO = dict(material="concrete", role="body")
MO = dict(material="motor-steel", role="body")


def SL(length, width):
    return {"slender": True} if length / width >= 20.5 else {}


def W(bone, p):
    f = F[bone]
    return [f["o"][k] + p[0] * f["u"][k] + p[1] * f["v"][k] + p[2] * f["w"][k] for k in range(3)]


def AX(bone):
    f = F[bone]
    return [f["u"], f["v"], f["w"]]


def LB(id, bone, c, size, **m):
    return V.box(id, bone, W(bone, c), size, axes=AX(bone), **m)


def LC(id, bone, a, b, r, **m):
    return V.cyl(id, bone, W(bone, a), W(bone, b), r, **m)


def LX(id, bone, plane, profile, a0, a1, holes=None, **m):
    d = V._meta(id, bone, "extrude", m)
    d["profile"] = [lib.rv(p) for p in profile]
    if holes:
        d["holes"] = [[lib.rv(p) for p in h] for h in holes]
    d["plane"] = plane
    d["from"], d["to"] = lib.r5(min(a0, a1)), lib.r5(max(a0, a1))
    return V.put(d)


def inset(P, t):
    n = len(P)
    area = sum(P[i][0] * P[(i + 1) % n][1] - P[(i + 1) % n][0] * P[i][1] for i in range(n))
    sg = 1 if area > 0 else -1
    lines = []
    for i in range(n):
        a, b = P[i], P[(i + 1) % n]
        dx, dy = b[0] - a[0], b[1] - a[1]
        l = math.hypot(dx, dy)
        nx, ny = -dy / l * sg, dx / l * sg
        lines.append(((a[0] + nx * t, a[1] + ny * t), (dx, dy)))
    out = []
    for i in range(n):
        (p1, d1), (p2, d2) = lines[i - 1], lines[i]
        den = d1[0] * d2[1] - d1[1] * d2[0]
        u = ((p2[0] - p1[0]) * d2[1] - (p2[1] - p1[1]) * d2[0]) / den
        out.append((p1[0] + d1[0] * u, p1[1] + d1[1] * u))
    return out


def offset_strip(pts, t):
    top = list(pts)
    return top + [(x, y - t) for x, y in reversed(top)]


BEAM = [(-2.15, -0.2), (-2.15, 0.22), (-1, 0.32), (1, 0.32), (2.8, 0.18), (2.8, -0.18), (1, -0.32), (-1, -0.32)]
BEAM_TOP = [(-2.15, 0.22), (-1, 0.32), (1, 0.32), (2.8, 0.18)]
BEAM_BOT = [(-2.15, -0.2), (-1, -0.32), (1, -0.32), (2.8, -0.18)]
HEAD = [(-2.81908, 1.02606), (-2.89778, 0.77646), (-2.95442, 0.52094), (-2.98858, 0.26147), (-3, 0), (-2.98858, -0.26147), (-2.95442, -0.52094), (-2.89778, -0.77646), (-2.81908, -1.02606), (-2.1, -0.45), (-2.1, 0.45)]
HEAD_ARC = [(-2.81908, 1.02606), (-2.89778, 0.77646), (-2.95442, 0.52094), (-2.98858, 0.26147), (-3, 0), (-2.98858, -0.26147), (-2.95442, -0.52094), (-2.89778, -0.77646), (-2.81908, -1.02606)]
HEAD_WEB = [(-2.54734, 0.73044), (-2.60421, 0.4905), (-2.63853, 0.24632), (-2.65, 0), (-2.63853, -0.24632), (-2.60421, -0.4905), (-2.54734, -0.73044), (-2.3, -0.3), (-2.3, 0.3)]
CWT = [(-0.88257, 0.68954), (-0.98428, 0.53442), (-1.05898, 0.36464), (-1.10464, 0.18485), (-1.12, 0), (-1.10464, -0.18485), (-1.05898, -0.36464), (-0.98428, -0.53442), (-0.88257, -0.68954), (-0.2758, -0.21548), (-0.33093, -0.11395), (-0.35, 0), (-0.33093, 0.11395), (-0.2758, 0.21548)]


def wellhead(scale):
    V.at(scale, "body")
    if scale == 50:
        V.cyl("wellhead-base", "base", [-3.0, 0, 0], [-3.0, 0.5, 0], 0.17, **ST)
        V.cyl("wellhead-flange", "base", [-3.0, 0.5, 0], [-3.0, 0.53, 0], 0.2, **ST)
        V.cyl("wellhead-valve", "base", [-3.0, 0.5, 0], [-3.0, 1.0, 0], 0.1, **ST)
        V.cyl("stuffing-box", "base", [-3.0, 1.0, 0], [-3.0, 1.28, 0], 0.08, **ST)
        V.cyl("flowline", "base", [-2.83, 0.68, 0], [-2.0, 0.68, 0], 0.06, **ST)
    elif scale == 100:
        V.cyl("wellhead-base", "base", [-3.0, 0, 0], [-3.0, 0.5, 0], 0.17, **ST)
        V.cyl("wellhead-top", "base", [-3.0, 0.5, 0], [-3.0, 1.28, 0], 0.1, **ST)
    else:
        V.cyl("wellhead", "base", [-3.0, 0, 0], [-3.0, 1.28, 0], 0.15 if scale == 200 else 0.17, **ST)
    r = {50: 0.025, 100: 0.025, 200: 0.025, 500: 0.05}[scale]
    V.cyl("polished-rod", "base", [-3.0, 1.28, 0], [-3.0, 4.4, 0], r, **SL(3.12, 2 * r), **ST)


def foundation(scale):
    V.at(scale, "body")
    V.box("slab", "base", [2.0, 0.15, 0], [7.8, 0.3, 2.4], tone=0.82, **CO)
    if scale == 50:
        for nm, c in (("l", 0.85), ("r", -0.85)):
            LX(f"skid-beam-{nm}", "skid", "yz", [(0, c - 0.1), (0, c + 0.1), (0.015, c + 0.1), (0.015, c + 0.006), (0.225, c + 0.006), (0.225, c + 0.1), (0.24, c + 0.1), (0.24, c - 0.1), (0.225, c - 0.1), (0.225, c - 0.006), (0.015, c - 0.006), (0.015, c - 0.1)], 0.0, 7.2, tone=0.5, **ST)
        LB("skid-cross", "skid", [0.3, 0.12, 0], [0.15, 0.22, 1.5], rep={"count": 5, "step": [1.6, 0, 0]}, tone=0.5, **ST)
        LB("skid-deck", "skid", [3.6, 0.245, 0], [7.2, 0.01, 2.0], tone=0.5, **ST)
    else:
        d = {100: 0.015, 200: 0.03, 500: 0.075}[scale]
        h = 0.25 - d
        LB("skid-beam", "skid", [3.6, h / 2, 0.85], [7.2, h, 0.2], rep={"count": 2, "step": [0, 0, -1.7]}, tone=0.5, **ST)
        LB("skid-cross", "skid", [0.3, h / 2, 0], [0.15, h - 0.02, 1.5], rep={"count": 5, "step": [1.6, 0, 0]}, tone=0.5, **ST)
        LB("skid-deck", "skid", [3.6, h + d / 2, 0], [7.2, d, 2.0], tone=0.5, **ST)


def post(scale):
    V.at(scale, "body")
    base = {"fl": (-1, 1), "fr": (-1, -1), "rl": (1, 1), "rr": (1, -1)}
    e = {50: 6, 100: 5, 200: 4, 500: 4}[scale]
    for nm, (sx, sz) in base.items():
        a = [sx * (1.48 + 0.048), 0.43, sz * (0.875 + 0.0238)]
        b = [sx * 0.04, 4.15, sz * 0.16]
        ry0, ry1 = (0.125, 0.105) if scale == 500 else (0.11, 0.093)
        secs = [(a, 0.075, ry0, e), (b, 0.06, ry1, e)]
        V.loft(f"leg-{nm}", "post", secs, hint=(0, 0, 1), tone=0.62, **ST)
    if scale <= 200:
        for nm, z in (("l", 0.65), ("r", -0.65)):
            V.box(f"brace-x-{nm}", "post", [0, 1.68, z], [1.9, 0.16, 0.1], tone=0.55, **ST)
        for nm, x in (("f", -1.03), ("r", 1.03)):
            V.box(f"brace-z-{nm}", "post", [x, 1.68, 0], [0.1, 0.16, 1.24], tone=0.55, **ST)
    if scale == 50:
        V.cyl("saddle", "post", [0, 4.28, -0.28], [0, 4.28, 0.28], 0.16, tone=0.3, **ST)


def beam_parts(scale):
    V.at(scale, "body")
    tw, tf, tc, tfa = {50: (0.04, 0.04, 0.04, 0.04), 100: (0.05, 0.05, 0.05, 0.05), 200: (0.06, 0.06, 0.07, 0.07), 500: (0.12, 0.1, 0.1, 0.1)}[scale]
    LX("beam-web", "beam", "xy", BEAM, -tw / 2, tw / 2, tone=0.7, **ST)
    LX("beam-flange-top", "beam", "xy", offset_strip(BEAM_TOP, tf), -0.18, 0.18, tone=0.7, **ST)
    LX("beam-flange-bottom", "beam", "xy", [(x, y + tf) for x, y in BEAM_BOT] + list(reversed(BEAM_BOT)), -0.18, 0.18, tone=0.7, **ST)
    LX("horsehead-face", "beam", "xy", HEAD_ARC + [(x + tfa, y) for x, y in reversed(HEAD_ARC)], -0.35, 0.35, tone=0.78, **ST)
    for nm, (a, b) in (("l", (0.35 - tc, 0.35)), ("r", (-0.35, -0.35 + tc))):
        LX(f"horsehead-cheek-{nm}", "beam", "xy", HEAD, a, b, tone=0.78, **ST)
    LX("horsehead-cap-top", "beam", "xy", [(-2.81908, 1.02606), (-2.1, 0.45), (-2.1, 0.45 - tfa), (-2.81908, 1.02606 - tfa)], -0.35, 0.35, tone=0.78, **ST)
    LX("horsehead-cap-bottom", "beam", "xy", [(-2.81908, -1.02606), (-2.1, -0.45), (-2.1, -0.45 + tfa), (-2.81908, -1.02606 + tfa)], -0.35, 0.35, tone=0.78, **ST)
    if scale == 50:
        LC("centre-bearing", "beam", [0, 0, -0.3], [0, 0, 0.3], 0.2, tone=0.3, **ST)
        LC("tail-pin", "beam", [2.7, 0, -0.3], [2.7, 0, 0.3], 0.14, tone=0.3, **ST)
        LX("horsehead-web", "beam", "xy", HEAD_WEB, -0.015, 0.015, tone=0.15, **ST)
    if scale == 100:
        LC("centre-bearing", "beam", [0, 0, -0.3], [0, 0, 0.3], 0.2, tone=0.3, **ST)


def linkage(scale):
    V.at(scale, "body")
    for nm, z in (("l", 0.575), ("r", -0.575)):
        LB(f"pitman-{nm}", "pitman", [0, -1.325, z], [0.14 if scale < 500 else 0.2, 2.65, 0.15], cr=0.02 if scale == 50 else None, tone=0.6, **ST)
        if scale <= 100:
            LC(f"pitman-eye-{nm}", "pitman", [0, -2.65, z - 0.075], [0, -2.65, z + 0.075], 0.12, tone=0.3, **ST)
    LB("carrier", "rod", [0, 0, 0], [0.6, 0.16, 0.3], tone=0.3, **ST)
    if scale <= 200:
        LC("crank-hub", "crank", [0, 0, -0.8], [0, 0, 0.8], 0.2 if scale == 50 else 0.14, tone=0.3, **CI)
    for nm, z in (("l", 0.575), ("r", -0.575)):
        LB(f"crank-arm-{nm}", "crank", [0.475, 0, z], [0.95, 0.28, 0.25], tone=0.58, **CI)
        LX(f"counterweight-{nm}", "crank", "xy", CWT, z - 0.125, z + 0.125, tone=0.4, **CI)
        if scale <= 100:
            LC(f"crank-pin-{nm}", "crank", [0.85, 0, z - 0.125], [0.85, 0, z + 0.125], 0.1, tone=0.25, **CI)


def machinery(scale):
    V.at(scale, "body")
    LB("gearbox", "gearbox", [0, -0.4, 0], [1.4, 1.3, 1.2], cr=0.1 if scale <= 100 else None, tone=0.62, **CI)
    LB("pedestal", "gearbox", [0, -1.125, 0], [1.1, 0.15, 1.1], tone=0.5, **CI)
    G = [(0.7, -0.15), (1.75, -0.53), (1.75, -0.97), (0.7, -1.05)]
    tg = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[scale]
    LX("belt-guard-ring", "gearbox", "xy", G, -0.95, -0.6, holes=[inset(G, tg)], tone=0.45, **ST)
    LX("belt-guard-plate-out", "gearbox", "xy", G, -0.95, -0.95 + tg, tone=0.45, **ST)
    LX("belt-guard-plate-in", "gearbox", "xy", G, -0.6 - tg, -0.6, tone=0.45, **ST)
    V.loft("motor", "motor", [(W("motor", [-0.55, 0.1, 0]), 0.4, 0.35, 6), (W("motor", [0.55, 0.1, 0]), 0.4, 0.35, 6)], hint=(0, 0, 1), tone=0.68, **MO)
    LB("motor-feet", "motor", [0, -0.3, 0], [1.0, 0.1, 0.9], tone=0.5, **MO)
    V.loft("motor-cowl", "motor", [(W("motor", [0.55, 0.1, 0]), 0.3, 0.25), (W("motor", [0.75, 0.1, 0]), 0.3, 0.25)], hint=(0, 0, 1), tone=0.5, **MO)
    if scale == 50:
        LB("terminal-box", "motor", [0, 0.55, 0], [0.3, 0.2, 0.3], tone=0.5, **MO)
        LC("motor-sheave", "motor", [-0.65, 0.1, -0.85], [-0.65, 0.1, -0.7], 0.2, tone=0.3, **ST)


for sc in (50, 100, 200, 500):
    foundation(sc)
    wellhead(sc)
    post(sc)
    beam_parts(sc)
    linkage(sc)
    machinery(sc)


def region_of(i):
    for pre, reg in (("slab", "base"), ("skid", "base"), ("wellhead", "wellhead"), ("stuffing", "wellhead"), ("flowline", "wellhead"), ("belt-guard", "steel"), ("gearbox", "gearbox"), ("pedestal", "gearbox"), ("crank", "crank"), ("counterweight", "crank"), ("motor-sheave", "steel"), ("motor", "motor"), ("terminal", "motor")):
        if i.startswith(pre):
            return reg
    return "steel"


def derive(scale):
    body = V.data[scale]["body"]
    ids = {b["id"] for b in body}
    V.at(scale, "thermal")
    for b in body:
        V.ref("th-" + b["id"], b["id"], region=region_of(b["id"]), **({"slender": True} if b.get("slender") else {}))
    V.at(scale, "structure")

    def mem(id, of, bone):
        if of in ids:
            V.ref("member-" + id, of, member=bone, part="member", **({"slender": True} if any(b["id"] == of and b.get("slender") for b in body) else {}))

    for nm in ("fl", "fr", "rl", "rr"):
        mem(f"leg-{nm}", f"leg-{nm}", "post")
    for nm in ("x-l", "x-r", "z-f", "z-r"):
        mem(f"brace-{nm}", f"brace-{nm}", "post")
    for n in ("beam", "beam-web", "beam-flange-top", "beam-flange-bottom"):
        mem(n, n, "beam")
    for nm in ("l", "r"):
        mem(f"pitman-{nm}", f"pitman-{nm}", "pitman")
        mem(f"crank-arm-{nm}", f"crank-arm-{nm}", "crank")
    for n, bone in (("carrier", "rod"), ("polished-rod", "base"), ("gearbox", "gearbox"), ("motor", "motor"), ("slab", "base"), ("skid-beam", "skid"), ("skid-beam-l", "skid"), ("skid-beam-r", "skid")):
        mem(n, n, bone)
    for n, bone in (("centre-bearing", "beam"), ("tail-pin", "beam"), ("crank-hub", "crank"), ("saddle", "post")):
        if n in ids:
            V.ref("joint-" + n, n, member=bone, part="joint")
    rj = {50: 0.2, 100: 0.2, 200: 0.22, 500: 0.25}[scale]
    if scale >= 200:
        LC("joint-beam", "beam", [0, 0, -0.3], [0, 0, 0.3], rj, tone=0.3, member="beam", part="joint", **ST)
        LC("joint-crank", "crank", [0, 0, -0.5], [0, 0, 0.5], rj, tone=0.3, member="crank", part="joint", **CI)
    if scale == 100:
        V.ref("joint-crank", "crank-pin-l", member="crank", part="joint")
    for nm in ("l", "r"):
        V.ref(f"mass-counterweight-{nm}", f"counterweight-{nm}", member="crank", part="mass")
    mb = {50: [6.6, 0.2, 2.0], 100: [6.4, 0.2, 1.9], 200: [6.2, 0.2, 1.8], 500: [6.0, 0.2, 1.7]}[scale]
    V.box("mass-base", "base", [2.0, 0.15, 0], mb, tone=0.4, member="base", part="mass", **CO)
    mg = {50: [0.5, 0.4, 0.4], 100: [0.5, 0.38, 0.38], 200: [0.48, 0.36, 0.36], 500: [0.45, 0.35, 0.35]}[scale]
    V.ell("mass-gearbox", "gearbox", W("gearbox", [0, -0.4, 0]), mg, tone=0.4, member="gearbox", part="mass", **CI)
    mm = {50: [0.3, 0.25, 0.25], 100: [0.3, 0.24, 0.24], 200: [0.28, 0.23, 0.23], 500: [0.26, 0.22, 0.22]}[scale]
    V.ell("mass-motor", "motor", W("motor", [0, 0.1, 0]), mm, tone=0.4, member="motor", part="mass", **MO)


for sc in (50, 100, 200, 500):
    derive(sc)
EN = {
 50: ("Detailed model of the unit in its mid-stroke pose: concrete slab; skid of two 240 mm I-beams (15 mm flanges, 12 mm web) with five cross members and a 10 mm deck; wellhead as base, flange, valve, stuffing box and flowline cylinders with the 50 mm polished rod as a slender solid; four inclined legs as rounded-rectangle lofts from the foot corners (+-1.48 m, +-0.875 m) to the saddle, two braces along x and two along z at 1.68 m, saddle bearing; walking beam as a 40 mm web plus two 40 mm flange plates along its outline with centre bearing and tail pin; horsehead as a curved face plate, two cheeks, two cap plates and a web; two pitman arms with eyes, rod carrier; cast gearbox with rounded edges on its pedestal, belt guard as a ring and two 7.5 mm plates, barrel-shaped motor with feet, cowl, terminal box and sheave; crank hub, two arms, two pins and two cast counterweight crescents.",
      "Thermal: every body solid is referenced by the region its 2D shape names (each region has one layer, so no nested shells), no hidden regions and no emitters. Structure: legs, braces, web and flanges of the beam, pitman arms, crank arms, polished rod, rod carrier, gearbox housing, motor, slab and skid beams are members; saddle, centre bearing, tail pin and crank hub are joints; the counterweights and compact boxes and ellipsoids inside slab, gearbox and motor are masses; the asset declares no supports."),
 100: ("Redrawn against 1:100: the skid is two box beams of 0.2 x 0.235 m with five cross members and a 15 mm deck, the beam is web, top and bottom flange of 50 mm, the horsehead face plate, cheeks and caps are 50 mm, the wellhead is a base and a 0.2 m top cylinder (no flowline, as drawn), crank hub 0.28 m, belt guard plates 15 mm; legs are rounded-rectangle lofts; no saddle, tail pin or horsehead web.",
       "Thermal and structure derived from the body like at 1:50; the structure keeps the legs, braces, beam parts, arms, housing and motor as members, the centre bearing and one crank pin as joints, and smaller masses than 1:50."),
 200: ("Redrawn against 1:200: skid beams, cross members and a 30 mm deck, beam web and flanges of 60 mm, horsehead plates of 70 mm, the wellhead one 0.3 m cylinder, no bearings, pins or eyes; belt guard ring and plates of 30 mm; legs, braces, arms, counterweights and crank arms as at 1:100 in simpler sections.",
       "Thermal and structure derived from the body; joints are 440 mm cylinders at beam and crank, masses of the base, gearbox and motor are smaller than at 1:100."),
 500: ("Merged masses: skid of two 175 mm beams, cross members and a 75 mm deck, beam web 120 mm and flanges 100 mm, horsehead plates 100 mm, polished rod 100 mm, belt guard plates 75 mm, legs 0.12 m wide; pitman arms 0.2 m wide as drawn; no braces, bearings or eyes.",
       "Thermal and structure derived from the body; the joints are 500 mm cylinders, the masses smaller again."),
}
DE = {
 50: ("Detailmodell der Anlage in der Pose Hubmitte: Betonplatte; Rahmen aus zwei Doppel-T-Trägern von 240 mm (Flansche 15 mm, Steg 12 mm) mit fünf Querträgern und einem Deckblech von 10 mm; Bohrlochkopf aus Fuß, Flansch, Ventil, Stopfbuchse und Flowline als Zylinder mit der 50-mm-Polierstange als schlankem Körper; vier geneigte Beine als Lofts mit gerundetem Rechteck von den Fußecken (+-1,48 m, +-0,875 m) zum Sattel, zwei Streben in x und zwei in z bei 1,68 m, Sattellager; Balancier aus 40-mm-Steg und zwei 40-mm-Flanschplatten entlang des Umrisses mit Mittellager und Schwanzbolzen; Pferdekopf als gekrümmte Frontplatte, zwei Wangen, zwei Deckplatten und ein Steg; zwei Schubstangen mit Augen, Stangenträger; gegossenes Getriebe mit gerundeten Kanten auf seinem Sockel, Riemenschutz aus Ring und zwei 7,5-mm-Platten, tonnenförmiger Motor mit Füßen, Haube, Klemmkasten und Riemenscheibe; Kurbelnabe, zwei Arme, zwei Bolzen und zwei gegossene Gegengewichtssicheln.",
      "Thermik: Jeder Körper ist durch die Region seiner 2D-Form verwiesen (jede Region hat eine Schicht, daher keine verschachtelten Schalen), keine versteckten Regionen, keine Emitter. Tragwerk: Beine, Streben, Steg und Flansche des Balanciers, Schubstangen, Kurbelarme, Polierstange, Stangenträger, Getriebegehäuse, Motor, Platte und Rahmenträger sind Stäbe; Sattel, Mittellager, Schwanzbolzen und Kurbelnabe sind Gelenke; die Gegengewichte sowie kompakte Kästen und Ellipsoide in Platte, Getriebe und Motor sind Massen; das Asset nennt keine Auflager."),
 100: ("Neu gezeichnet gegen 1:100: Der Rahmen besteht aus zwei Trägern von 0,2 x 0,235 m mit fünf Querträgern und 15-mm-Deckblech, der Balancier aus Steg, Ober- und Unterflansch von 50 mm, Frontplatte, Wangen und Deckplatten des Pferdekopfs sind 50 mm, der Bohrlochkopf ist ein Fuß und ein 0,2-m-Zylinder oben (wie gezeichnet ohne Flowline), die Kurbelnabe 0,28 m, die Riemenschutzplatten 15 mm; die Beine sind Lofts mit gerundetem Rechteck; kein Sattel, kein Schwanzbolzen, kein Pferdekopfsteg.",
        "Thermik und Tragwerk wie bei 1:50 aus dem Körper abgeleitet; das Tragwerk behält Beine, Streben, Balancierteile, Arme, Gehäuse und Motor als Stäbe, das Mittellager und einen Kurbelbolzen als Gelenke und kleinere Massen als bei 1:50."),
 200: ("Neu gezeichnet gegen 1:200: Rahmenträger, Querträger und 30-mm-Deckblech, Balanciersteg und Flansche 60 mm, Pferdekopfplatten 70 mm, der Bohrlochkopf ein Zylinder von 0,3 m, keine Lager, Bolzen oder Augen; Riemenschutz aus Ring und Platten von 30 mm; Beine, Streben, Arme, Gegengewichte und Kurbelarme wie bei 1:100 in einfacheren Querschnitten.",
        "Thermik und Tragwerk aus dem Körper abgeleitet; die Gelenke sind 440-mm-Zylinder an Balancier und Kurbel, die Massen von Platte, Getriebe und Motor kleiner als bei 1:100."),
 500: ("Zusammengefasste Massen: Rahmen aus zwei Trägern von 175 mm, Querträgern und 75-mm-Deckblech, Balanciersteg 120 mm und Flansche 100 mm, Pferdekopfplatten 100 mm, Polierstange 100 mm, Riemenschutzplatten 75 mm, Beine 0,12 m breit; Schubstangen 0,2 m breit wie gezeichnet; keine Streben, Lager oder Augen.",
        "Thermik und Tragwerk aus dem Körper abgeleitet; die Gelenke sind 500-mm-Zylinder, die Massen nochmals kleiner."),
}
for sc in (50, 100, 200, 500):
    for k in (0, 1):
        V.decide(sc, EN[sc][k], DE[sc][k])
V.write()
