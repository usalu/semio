from author_37_lib import *

PD = Ctx("pedestal", "cast-iron", None, "body", 0.6)
BS = Ctx("basin", "stainless-basin", None, "body", 0.5)
BR = Ctx("bubbler", "chrome-brass", None, "body", 0.7)
BT = Ctx("button", "chrome-brass", None, "body", 0.8)
JT = Ctx("jet", "water", None, "body", 0.5)
ST = Ctx("pedestal", "cast-iron", None, "detail", 0.15)
ALL6 = ["+u", "-u", "+v", "-v", "+w", "-w"]

PED = [(0, 0.2), (0.04, 0.2), (0.055, 0.17), (0.075, 0.16), (0.08, 0.15), (0.7, 0.1), (0.72, 0.11), (0.745, 0.09)]


def r_at(y, pts=PED):
    for (y0, r0), (y1, r1) in zip(pts, pts[1:]):
        if y0 <= y <= y1:
            return r0 + (r1 - r0) * (y - y0) / ((y1 - y0) or 1)
    return pts[-1][1]


def bowl(n, thick, rim=0.125, bot=0.0, c1=(0.2, 0.05), c2=(0.12, 0.0), solid=False):
    p0, p3 = (0.2, rim), (0.0, bot)
    pts = bezier(p0, c1, c2, p3, n)
    outer = [[y, r] for r, y in pts]
    if solid:
        return outer + [[rim, 0.0]]
    inner = []
    for k in range(n + 1):
        t = k / n
        dr, dy = bezier_d(p0, c1, c2, p3, t)
        l = math.hypot(dr, dy)
        nr, ny = -dy / l, dr / l
        r, y = pts[k]
        inner.append([y - ny * thick, max(0.0, r - nr * thick)])
    return outer + inner[::-1]


def bowl_y(r, bot=0.0, rim=0.125, c1=(0.2, 0.05), c2=(0.12, 0.0), thick=0.0):
    pts = bezier((0.2, rim), c1, c2, (0.0, bot), 400)
    best = min(pts, key=lambda p: abs(p[0] - r))
    return best[1] + thick


def collar(sid, y, rs, t=0.0075):
    return ring(sid, PD, [[y - t / 2, rs - 0.003], [y - t / 2, rs + 0.0045], [y + t / 2, rs + 0.0045], [y + t / 2, rs - 0.003]], "v")


def gooseneck(n_bend, colw, rn0, top_rn, base_y=-0.005, coarse=False):
    cx = -0.005
    st = [(cx, base_y, 90, colw, rn0), (cx, 0.04, 90, colw, rn0), (cx, 0.075, 90, colw, rn0)]
    C, R = (0.026, 0.095), 0.031
    for k in range(n_bend + 1):
        th = 180 - 90 * k / n_bend
        f = k / n_bend
        st.append((C[0] + R * math.cos(math.radians(th)), C[1] + R * math.sin(math.radians(th)), th - 90, colw - 0.0003 * f, rn0 + (top_rn - rn0) * f))
    st += [(0.045, 0.1262, -3, 0.0275, top_rn), (0.07, 0.1215, -17, 0.026, top_rn - 0.0006), (0.09, 0.1085, -42, 0.023, top_rn - 0.001), (0.097, 0.098, -72, 0.018, top_rn - 0.0013)]
    return st


def jet_stations(n, r0, r1):
    out = []
    p0, c1, c2, p3 = (0, 0), (0.025, 0.045), (0.095, 0.045), (0.13, -0.085)
    for k in range(n + 1):
        t = k / n
        x, y = [(1 - t) ** 3 * p0[i] + 3 * (1 - t) ** 2 * t * c1[i] + 3 * (1 - t) * t ** 2 * c2[i] + t ** 3 * p3[i] for i in range(2)]
        dx, dy = bezier_d(p0, c1, c2, p3, t)
        r = r0 + (r1 - r0) * t
        out.append((x, y, math.degrees(math.atan2(dy, dx)), r, r))
    return out[::-1]


def pad(h):
    return tag(cyl("support-base-fixed", ST(member="pedestal", part="support", tone=0.1), "v", (0, h / 2, 0), h, 0.19), tags=["base-fixed"])


def ped_member(sid, inset, wall, y0=0.045, y1=0.74):
    ys = [y0] + [y for y in (0.055, 0.075, 0.08, 0.7, 0.72) if y > y0] + [y1]
    o = [[y, r_at(y) - inset] for y in ys]
    i = [[y, r_at(y) - inset - wall] for y in ys]
    return ring(sid, ST(member="pedestal", part="member"), o + i[::-1], "v")


def jet_emit(x, y, rr=0.006, h=0.0076):
    s = ext("th-jet-spray", Ctx("bubbler", "water", "water-jet", "detail", 0.5)(layer="stream"), ngon(rr, 16, 0, x), "zx", y, y + h)
    s["sectionFaces"] = ALL6
    s["tags"] = ["jet-spray"]
    return s


BOLTS = [26.4, 63.6, 116.4, 153.6, -26.4, -63.6, -116.4, -153.6]

# ------------------------------------------------------------ 1:50
body50 = [
    rev("pedestal", PD, [[y, r] for y, r in PED] + [[0.745, 0], [0, 0]], "v"),
    collar("collar-0", 0.2, r_at(0.2)),
    collar("collar-1", 0.42, r_at(0.42)),
    collar("collar-2", 0.62, r_at(0.62)),
]
for k, a in enumerate(BOLTS):
    body50.append(ell("bolt-%d" % k, PD(tone=0.4, role="detail"), (0.2 * math.sin(math.radians(a)), 0.02, 0.2 * math.cos(math.radians(a))), (0.008, 0.008, 0.008)))
yin = bowl_y(0.04, thick=0.0015)
body50 += [
    ring("basin", BS, bowl(14, 0.0015), "v"),
    ring("rim-bead", BS(role="detail", tone=0.55), [[0.1255 + 0.00376 * math.sin(2 * math.pi * k / 12), 0.2 + 0.00376 * math.cos(2 * math.pi * k / 12)] for k in range(12)], "v"),
    ext("strainer", BS(role="detail", tone=0.35), ngon(0.022, 20, 0, 0.04), "zx", yin - 0.003, yin + 0.0045, holes=[ngon(0.0035, 8, 0.012 * math.sin(a), 0.04 + 0.012 * math.cos(a)) for a in [k * math.pi / 3 for k in range(6)]] + [ngon(0.0035, 8, 0, 0.04)]),
    box("bracket", BR(tone=0.6), (-0.048, -0.0075, -0.02), (0.02, 0.0, 0.02), 0.003),
    ploft("bubbler", BR, gooseneck(4, 0.0275, 0.0255, 0.014)),
    ring("bubbler-collar", BR(tone=0.8, role="detail"), [[0.0365, 0.024], [0.0365, 0.0315], [0.044, 0.0315], [0.044, 0.024]], "v", origin=(-0.005, 0, 0)),
    ext("nozzle-ring", BR(tone=0.85, role="detail"), ngon(0.016, 20, 0, 0.1), "zx", 0.0985, 0.1075, holes=[ngon(0.007, 14, 0, 0.1)]),
    ell("button", BT, (0.006, 0, 0), (0.02, 0.025, 0.02)),
    ext("escutcheon", BR(bone="pedestal", tone=0.75, role="detail"), ellipse_pts(0.66, 0, 0.032, 0.027), "yz", 0.0935, 0.1085, holes=[ellipse_pts(0.66, 0, 0.0265, 0.0215)]),
    ploft("jet", JT, jet_stations(10, 0.006, 0.005)),
    ell("drop-0", JT(role="detail"), (0.115, -0.04, 0), (0.007, 0.007, 0.007)),
    ell("drop-1", JT(role="detail"), (0.14, -0.07, 0), (0.007, 0.007, 0.007)),
]
L1 = lambda a, b: th(a, b[0], b[1], b[2])
th50 = [
    th("th-pedestal", "pedestal", "pedestal-cast", "casting"),
    *[th("th-collar-%d" % k, "collar-%d" % k, "pedestal-cast", "casting") for k in range(3)],
    *[th("th-bolt-%d" % k, "bolt-%d" % k, "pedestal-cast", "casting") for k in range(8)],
    th("th-basin", "basin", "basin-steel", "sheet"),
    th("th-rim-bead", "rim-bead", "basin-steel", "sheet"),
    th("th-strainer", "strainer", "basin-steel", "sheet"),
    th("th-bracket", "bracket", "bubbler-brass", "tube"),
    th("th-bubbler", "bubbler", "bubbler-brass", "tube"),
    th("th-bubbler-collar", "bubbler-collar", "bubbler-brass", "tube"),
    th("th-nozzle-ring", "nozzle-ring", "bubbler-brass", "tube"),
    th("th-button", "button", "bubbler-brass", "tube"),
    th("th-escutcheon", "escutcheon", "bubbler-brass", "tube"),
    th("th-jet", "jet", "water-jet", "stream"),
    th("th-drop-0", "drop-0", "water-jet", "stream"),
    th("th-drop-1", "drop-1", "water-jet", "stream"),
    jet_emit(0.1, 0.092),
]
st50 = [
    ped_member("member-pedestal", 0.003, 0.012),
    rev("member-flange", ST(member="pedestal", part="member"), [[0.004, 0], [0.004, 0.194], [0.036, 0.194], [0.036, 0]], "v"),
    stm("member-basin", "basin", "basin", "member"),
    stm("member-bracket", "bracket", "bubbler", "member"),
    stm("member-bubbler", "bubbler", "bubbler", "member"),
    stm("member-jet", "jet", "jet", "member"),
    stm("joint-rim-bead", "rim-bead", "basin", "joint"),
    stm("joint-nozzle-ring", "nozzle-ring", "bubbler", "joint"),
    stm("joint-escutcheon", "escutcheon", "button", "joint"),
    rev("joint-basin-socket", ST(member="basin", part="joint"), [[0.73, 0], [0.73, 0.075], [0.745, 0.075], [0.745, 0]], "v"),
    ell("mass-pedestal", ST(member="pedestal", part="mass", tone=0.2), (0, 0.3, 0), (0.08, 0.12, 0.08)),
    pad(0.0075),
]
D50 = [
    text("Own 1:50 model of the cast drinking fountain against the 1:50 elevation, front and plan: the cast pedestal as one revolve of the drawn profile (flange with chamfered steps, shaft tapering from 0.15 to 0.10 m, flared cap), three raised collar rings on the shaft at 0.20, 0.42 and 0.62 m and eight dome-headed anchor bolts on the flange side at the angles that the elevation (x = 0.179 m) and the front show; the stainless basin as a true 1.5 mm hollow revolve of the drawn Bezier bowl with a rolled 9 mm rim bead and a perforated 44 mm strainer at the drain off-centre; the goose-neck bubbler as one loft of 15 stations along its drawn centre line (round column that turns into a flattened arch and a downward nozzle end), on a rim bracket that clamps it to the basin wall, with a collar ring and a hollow nozzle ring; the domed push button in its elliptical escutcheon ring; the water jet as a tapering 12 to 10 mm loft along the drawn arc with two drops (pose: drinking, button pressed 10 mm).",
         "Eigenes 1:50-Modell des gusseisernen Trinkbrunnens nach Ansicht, Aufriss und Grundriss von 1:50: der Gusssockel als eine Drehfläche des gezeichneten Profils (Flansch mit gefasten Stufen, Schaft von 0,15 auf 0,10 m verjüngt, ausgestellter Kopf), drei erhabene Kragenringe am Schaft bei 0,20, 0,42 und 0,62 m und acht Domschrauben am Flanschrand in den Winkeln, die Ansicht (x = 0,179 m) und Aufriss zeigen; das Edelstahlbecken als echte hohle 1,5-mm-Drehfläche der gezeichneten Bézier-Schale mit gerolltem 9-mm-Randwulst und einem gelochten 44-mm-Sieb am außermittigen Ablauf; der Schwanenhals-Sprudler als ein Loft aus 15 Stationen entlang seiner gezeichneten Mittellinie (runde Säule, die in einen flachen Bogen und ein nach unten weisendes Düsenende übergeht) auf einer Randkonsole, die ihn an der Beckenwand klemmt, mit Kragenring und hohlem Düsenring; der gewölbte Drucktaster in seinem elliptischen Rosettenring; der Wasserstrahl als sich von 12 auf 10 mm verjüngender Loft entlang des gezeichneten Bogens mit zwei Tropfen (Pose: drinking, Taster 10 mm gedrückt)."),
    text("The pedestal is a hollow 12 mm casting in reality (mass 62 kg): the body keeps its envelope, the structure carries the hollow section. Thermal by reference: cast-iron region = pedestal, collars and bolts (the base face on the footing counts no area); basin region = sheet, bead and strainer, exposed on both faces; bubbler region = neck, bracket, collar, nozzle ring, button and escutcheon; water-jet region = jet and drops plus the 12 mm emitter disc jet-spray at the nozzle anchor (all faces section faces: it is the outlet of the moisture plume).",
         "Der Sockel ist in Wirklichkeit ein hohler 12-mm-Guss (Masse 62 kg): der Körper behält seine Hülle, das Tragwerk trägt den hohlen Querschnitt. Thermik als Verweis: Gussregion = Sockel, Kragen und Schrauben (die Fläche auf dem Fundament zählt nicht); Beckenregion = Blech, Wulst und Sieb, beidseitig frei; Sprudlerregion = Hals, Konsole, Kragen, Düsenring, Taster und Rosette; Strahlregion = Strahl und Tropfen plus die 12-mm-Emitterscheibe jet-spray am Düsenanker (alle Flächen Schnittflächen: sie ist der Auslass der Feuchtefahne)."),
    text("Structure: hollow cast shaft (12 mm wall, 3 mm inside the envelope) and the flange disc as members of the pedestal, basin sheet, bracket, goose-neck and jet as members, rim bead, nozzle ring, escutcheon and the basin socket ring as joints, a mass ellipsoid for the 62 kg of the pedestal at 0.30 m and the 0.38 m fixed block under the flange as the support of the anchor base-fixed.",
         "Tragwerk: hohler Gussschaft (12 mm Wand, 3 mm innerhalb der Hülle) und die Flanschscheibe als Stäbe des Sockels, Beckenblech, Konsole, Schwanenhals und Strahl als Stäbe, Randwulst, Düsenring, Rosette und der Beckensockelring als Gelenke, ein Massenellipsoid für die 62 kg des Sockels bei 0,30 m und der Festblock unter dem Flansch als Auflager des Ankers base-fixed."),
]

# ------------------------------------------------------------ 1:100
PED100 = [(0, 0.2), (0.05, 0.2), (0.06, 0.17), (0.08, 0.15), (0.7, 0.1), (0.745, 0.09)]
body100 = [
    rev("pedestal", PD, [[y, r] for y, r in PED100] + [[0.745, 0], [0, 0]], "v"),
    ring("basin", BS, bowl(10, 0.015), "v"),
    box("bracket", BR(tone=0.6), (-0.048, -0.015, -0.0225), (0.02, 0.0, 0.0225), 0.004),
    ploft("bubbler", BR, gooseneck(3, 0.0275, 0.0255, 0.0145)),
    ell("button", BT, (0.006, 0, 0), (0.02, 0.025, 0.02)),
    ploft("jet", JT, jet_stations(6, 0.009, 0.0085)),
]
th100 = [
    th("th-pedestal", "pedestal", "pedestal-cast", "casting"),
    th("th-basin", "basin", "basin-steel", "sheet"),
    th("th-bracket", "bracket", "bubbler-brass", "tube"),
    th("th-bubbler", "bubbler", "bubbler-brass", "tube"),
    th("th-button", "button", "bubbler-brass", "tube"),
    th("th-jet", "jet", "water-jet", "stream"),
    jet_emit(0.1, 0.092, 0.0075, 0.0151),
]
st100 = [
    ped_member("member-pedestal", 0.004, 0.015, 0.06),
    stm("member-basin", "basin", "basin", "member"),
    stm("member-bracket", "bracket", "bubbler", "member"),
    stm("member-bubbler", "bubbler", "bubbler", "member"),
    stm("member-jet", "jet", "jet", "member"),
    rev("joint-basin-socket", ST(member="basin", part="joint"), [[0.725, 0], [0.725, 0.075], [0.745, 0.075], [0.745, 0]], "v"),
    ell("mass-pedestal", ST(member="pedestal", part="mass", tone=0.2), (0, 0.3, 0), (0.08, 0.12, 0.08)),
    pad(0.015),
]
D100 = [
    text("Own 1:100 model against the 1:100 drawings: one pedestal revolve (flange with a 15 mm step and a chamfer, straight taper to the cap; the two collar lines, the bolts and the flute hatch are below the 15 mm feature and merged), the basin as a 15 mm hollow bowl revolve (both faces stay exposed, as the 1.5 mm sheet of 1:50), a 15 mm rim bracket, the goose-neck as a loft of 11 stations (flattened arch without collar and nozzle ring), the push button and an 18 mm jet loft along the drawn arc; drops and strainer are merged.",
         "Eigenes 1:100-Modell gegen die 1:100-Zeichnungen: eine Sockel-Drehfläche (Flansch mit 15-mm-Stufe und Fase, gerade Verjüngung zum Kopf; die zwei Kragenlinien, die Schrauben und die Riffelschraffur liegen unter dem 15-mm-Merkmal und sind zusammengefasst), das Becken als 15-mm-Hohlschale (beide Flächen bleiben frei wie beim 1,5-mm-Blech von 1:50), eine 15-mm-Randkonsole, der Schwanenhals als Loft aus 11 Stationen (flacher Bogen ohne Kragen und Düsenring), der Drucktaster und ein 18-mm-Strahl entlang des gezeichneten Bogens; Tropfen und Sieb sind zusammengefasst."),
    text("Thermal by reference (pedestal, basin, bracket and neck, button, jet) plus the 15 mm emitter disc at the nozzle; structure: hollow shaft with 15 mm wall, basin, bracket, goose-neck and jet as members, the socket ring as joint, a pedestal mass and a 15 mm fixed block.",
         "Thermik als Verweis (Sockel, Becken, Konsole und Hals, Taster, Strahl) plus die 15-mm-Emitterscheibe an der Düse; Tragwerk: hohler Schaft mit 15 mm Wand, Becken, Konsole, Schwanenhals und Strahl als Stäbe, der Sockelring als Gelenk, eine Sockelmasse und ein 15-mm-Festblock."),
]

# ------------------------------------------------------------ 1:200
PED200 = [(0, 0.2), (0.07, 0.2), (0.07, 0.15), (0.7, 0.1), (0.745, 0.09)]
body200 = [
    rev("pedestal", PD, [[y, r] for y, r in PED200] + [[0.745, 0], [0, 0]], "v"),
    ring("basin", BS, bowl(8, 0.03), "v"),
    box("bracket", BR(tone=0.6), (-0.048, -0.03, -0.03), (0.02, 0.0, 0.03)),
    ploft("bubbler", BR, [(0.0025, -0.005, 90, 0.03, 0.0225), (0.0025, 0.07, 90, 0.03, 0.0225), (0.012, 0.113, 40, 0.03, 0.0185), (0.045, 0.126, -3, 0.03, 0.015), (0.09, 0.108, -42, 0.027, 0.015), (0.097, 0.098, -72, 0.02, 0.015)]),
    box("button", BT, (-0.0125, -0.02, -0.02), (0.0125, 0.02, 0.02), 0.004),
    ploft("jet", JT, jet_stations(4, 0.015, 0.015)),
]
th200 = [
    th("th-pedestal", "pedestal", "pedestal-cast", "casting"),
    th("th-basin", "basin", "basin-steel", "sheet"),
    th("th-bracket", "bracket", "bubbler-brass", "tube"),
    th("th-bubbler", "bubbler", "bubbler-brass", "tube"),
    th("th-button", "button", "bubbler-brass", "tube"),
    th("th-jet", "jet", "water-jet", "stream"),
    jet_emit(0.1, 0.085, 0.015, 0.03),
]
st200 = [
    ped_member("member-pedestal", 0.01, 0.03, 0.08),
    stm("member-basin", "basin", "basin", "member"),
    stm("member-bracket", "bracket", "bubbler", "member"),
    stm("member-bubbler", "bubbler", "bubbler", "member"),
    stm("member-jet", "jet", "jet", "member"),
    ell("mass-pedestal", ST(member="pedestal", part="mass", tone=0.2), (0, 0.3, 0), (0.08, 0.12, 0.08)),
    pad(0.03),
]
D200 = [
    text("Own 1:200 model: one stepped pedestal revolve (flange 70 mm, straight taper), the basin as a 30 mm hollow bowl, the goose-neck as a six-station loft with 30 mm sections on a 30 mm rim bracket, the push button as a 25 x 40 x 40 mm block and a 30 mm jet loft along the arc (the drawing's 26 mm stroke, raised to the feature); collars, bolts, strainer, rim bead, collar and nozzle rings are merged.",
         "Eigenes 1:200-Modell: eine gestufte Sockel-Drehfläche (Flansch 70 mm, gerade Verjüngung), das Becken als 30-mm-Hohlschale, der Schwanenhals als Loft aus sechs Stationen mit 30-mm-Schnitten auf einer 30-mm-Randkonsole, der Drucktaster als Block 25 x 40 x 40 mm und ein 30-mm-Strahl entlang des Bogens (der 26-mm-Strich der Zeichnung, auf das Merkmal angehoben); Kragen, Schrauben, Sieb, Randwulst, Kragen- und Düsenring sind zusammengefasst."),
    text("Thermal by reference plus a 30 mm emitter disc; structure: 30 mm cast shaft wall, basin, bracket, goose-neck and jet as members, one pedestal mass and a 30 mm fixed block; joints are merged into the members.",
         "Thermik als Verweis plus eine 30-mm-Emitterscheibe; Tragwerk: 30-mm-Gussschaftwand, Becken, Konsole, Schwanenhals und Strahl als Stäbe, eine Sockelmasse und ein 30-mm-Festblock; Gelenke sind in den Stäben aufgegangen."),
]

# ------------------------------------------------------------ 1:500
PED500 = [(0, 0.2), (0.12, 0.2), (0.12, 0.14), (0.72, 0.1)]
body500 = [
    rev("pedestal", PD, [[y, r] for y, r in PED500] + [[0.72, 0], [0, 0]], "v"),
    rev("basin", BS, bowl(8, 0, bot=-0.02, c1=(0.2, 0.04), c2=(0.12, -0.02), solid=True), "v"),
    box("bubbler", BR(tone=0.7), (-0.04, 0.0, -0.03), (0.11, 0.14, 0.03)),
    box("button", BT, (-0.02, -0.04, -0.02), (0.02, 0.04, 0.02)),
    ploft("jet", JT, jet_stations(2, 0.0375, 0.0375)),
]
th500 = [
    th("th-pedestal", "pedestal", "pedestal-cast", "casting"),
    th("th-basin", "basin", "basin-steel", "sheet"),
    th("th-bubbler", "bubbler", "bubbler-brass", "tube"),
    th("th-button", "button", "bubbler-brass", "tube"),
    th("th-jet", "jet", "water-jet", "stream"),
    jet_emit(0.1, 0.07, 0.0375, 0.075),
]
st500 = [
    stm("member-pedestal", "pedestal", "pedestal", "member"),
    stm("member-basin", "basin", "basin", "member"),
    stm("member-bubbler", "bubbler", "bubbler", "member"),
    stm("member-jet", "jet", "jet", "member"),
    pad(0.075),
]
D500 = [
    text("Own 1:500 model of the fountain sign: a flared pedestal revolve (0.40 m flange 0.12 m high, straight shaft to 0.10 m), the basin as a solid lens down to the drawn -0.02 m, the goose-neck as one 0.15 x 0.14 x 0.06 m block as drawn, the push button as a 40 x 80 x 40 mm block and the jet as a 75 mm loft along the arc; every detail below 75 mm is merged.",
         "Eigenes 1:500-Modell des Brunnen-Zeichens: ein ausgestellter Sockel als Drehfläche (Flansch 0,40 m, 0,12 m hoch, gerader Schaft auf 0,10 m), das Becken als Vollkörper bis zum gezeichneten -0,02 m, der Schwanenhals als ein Block 0,15 x 0,14 x 0,06 m wie gezeichnet, der Drucktaster als Block 40 x 80 x 40 mm und der Strahl als 75-mm-Loft entlang des Bogens; jedes Detail unter 75 mm ist zusammengefasst."),
    text("Thermal by reference plus a 75 mm emitter disc; structure: the five bones as members (pedestal, basin lens, block, jet) and a 75 mm fixed block. Pedestal and basin are solid at this scale, so the exposed area is the outer surface and the lens top.",
         "Thermik als Verweis plus eine 75-mm-Emitterscheibe; Tragwerk: die Knochen als Stäbe (Sockel, Beckenlinse, Block, Strahl) und ein 75-mm-Festblock. Sockel und Becken sind in diesem Maßstab voll, die freie Fläche ist die Außenfläche und die Linsenoberseite."),
]
write("drinking-fountain", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
    500: dict(body=body500, thermal=th500, structure=st500, decisions=D500),
}, category="objects")
