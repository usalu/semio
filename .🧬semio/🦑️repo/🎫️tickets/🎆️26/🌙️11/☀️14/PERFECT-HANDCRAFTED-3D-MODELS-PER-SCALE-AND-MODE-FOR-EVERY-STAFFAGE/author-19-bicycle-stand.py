import os, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-19-lib.py")).read())
H, C, S = "hoop", "crossbar", "stand"
M = "galv-steel"
X, YB, R, RO, RI = 0.35082, 0.59585, 0.18, 0.02415, 0.02095
XI = 0.32665
SO, SI = 0.0225, 0.0193


def half_rings(pfx, bone, tone, left, ro, ri, n=12):
    out = []
    for name, sgn in (("a", 1), ("b", -1)):
        pts = [[sgn * ro * math.sin(math.pi * i / n), R + ro * math.cos(math.pi * i / n)] for i in range(n + 1)]
        pts += [[sgn * ri * math.sin(math.pi * (n - i) / n), R + ri * math.cos(math.pi * (n - i) / n)] for i in range(n + 1)]
        pts.append(pts[0])
        d = revolve(f"{pfx}-{name}", bone, M, tone, "z", [0, 0, 0], pts, sweep=math.pi / 2)
        d["transform"] = {"translation": vec([(-1 if left else 1) * (X - R), YB, 0])}
        if left:
            d["transform"]["rotation"] = qaxis([0, 0, 1], 90)
        out.append(d)
    return out


def tube_y(id, bone, x, y0, y1, ro, ri, tone=0.45):
    return revolve(id, bone, M, tone, "y", [x, y0, 0], [[0, ri], [y1 - y0, ri], [y1 - y0, ro], [0, ro], [0, ri]])


def tube_x(id, bone, x0, x1, y, ro, ri, tone=0.45):
    return revolve(id, bone, M, tone, "x", [x0, y, 0], [[0, ri], [x1 - x0, ri], [x1 - x0, ro], [0, ro], [0, ri]])


def collar_g(id, x, h=0.008, tone=0.3):
    d = revolve(id, H, M, tone, "y", [x, 0, 0], [[0, 0], [0, 0.0242], [0.002, 0.02465], [h - 0.002, 0.02465], [h, 0.0242], [h, 0], [0, 0]], role="detail")
    return d


def collar_w(id, x, tone=0.3):
    return revolve(id, C, M, tone, "x", [x, 0, 0], [[-0.0045, 0], [-0.0045, 0.0242], [-0.0025, 0.02465], [0.0025, 0.02465], [0.0045, 0.0242], [0.0045, 0], [-0.0045, 0]], role="detail")


sc = []
# ---- 1:50
s = Scale(50)
s.b(loft("hoop", H, M, 0.45, hoop_sections(10, RO, RO)))
s.b(collar_g("ground-collar-l", -X))
s.b(collar_g("ground-collar-r", X))
s.b(cyl("crossbar", C, M, 0.45, "x", [0, 0, 0], 2 * X, RO))
s.b(collar_w("weld-collar-l", -0.31265))
s.b(collar_w("weld-collar-r", 0.31265))
s.thof("th-hoop", "hoop", "hoop-galv")
s.thof("th-ground-collar-l", "ground-collar-l", "hoop-galv")
s.thof("th-ground-collar-r", "ground-collar-r", "hoop-galv")
s.thof("th-crossbar", "crossbar", "crossbar-galv")
s.thof("th-weld-collar-l", "weld-collar-l", "crossbar-galv")
s.thof("th-weld-collar-r", "weld-collar-r", "crossbar-galv")
for sg, nm in ((-1, "l"), (1, "r")):
    s.st(tube_y(f"root-{nm}", S, sg * X, 0, 0.0483, SO, SI), S, "member")
    s.st(tube_y(f"leg-{nm}", H, sg * X, 0.0483, YB, SO, SI), H, "member")
for d in half_rings("bend-l", H, 0.45, True, SO, SI) + half_rings("bend-r", H, 0.45, False, SO, SI):
    s.st(d, H, "member")
s.st(tube_x("top", H, -(X - R), X - R, YB + R, SO, SI), H, "member")
s.st(tube_x("crossbar-tube", C, -XI, XI, 0, SO, SI), C, "member")
s.stof("hoop-joint-l", "ground-collar-l", H, "joint")
s.stof("hoop-joint-r", "ground-collar-r", H, "joint")
s.stof("weld-joint-l", "weld-collar-l", C, "joint")
s.stof("weld-joint-r", "weld-collar-r", C, "joint")
for sg, nm in ((-1, "left"), (1, "right")):
    s.st(box(f"support-leg-{nm}-block", H, M, 0.15, [0.0483, 0.015, 0.0483], [sg * X, -0.0075, 0]), H, "support")
s.dec("Body: the 48.3 mm tube hoop as one loft of 23 round sections along its real centre line (vertical legs to 596 mm, two 180 mm radius bends, 342 mm top rail) instead of two halves, ground collars with a chamfered edge at the foot, the 48.3 mm cross rail as a cylinder through to the leg axes (its ends are buried in the legs, as welded) and two chamfered weld rings on it. The tube is modelled as the closed outer envelope: its 3.2 mm wall is below the 7.5 mm feature and no drawing shows the bore.",
      "Körper: der Bügel aus Rohr 48,3 mm als eine Loft aus 23 runden Schnitten entlang der wirklichen Mittellinie (senkrechte Beine bis 596 mm, zwei Bögen mit 180 mm Radius, 342 mm Oberholm) statt aus zwei Hälften, Bodenmanschetten mit gefaster Kante am Fuß, der Querholm 48,3 mm als Zylinder bis zu den Beinachsen (seine Enden sind in den Beinen begraben wie angeschweißt) und zwei gefaste Schweißringe darauf. Das Rohr ist als geschlossene Außenhülle modelliert: seine Wand von 3,2 mm liegt unter der Einzelheit 7,5 mm, und keine Zeichnung zeigt die Bohrung.")
s.dec("Thermal: both regions are exactly body surfaces (single 3.2 mm steel wall layer, so one surface solid each, B2): hoop-galv = hoop loft and ground collars, crossbar-galv = cross rail and weld rings, referenced; there are no emitters and no hidden regions.",
      "Thermisch: beide Regionen sind genau Körperflächen (eine Stahlwand von 3,2 mm, also je ein Oberflächenkörper, B2): hoop-galv = Bügelloft und Bodenmanschetten, crossbar-galv = Querholm und Schweißringe, referenziert; es gibt keine Emitter und keine verdeckten Regionen.")
s.dec("Structure: every member bone is a real hollow tube of 3.2 mm wall, modelled 1.5 mm inside the body envelope (45 mm outer instead of 48.3 mm, so that it does not coincide with the body skin and bury its exposed area; A = 4.3 cm2 instead of 4.6 cm2): the first 48.3 mm of each leg is the fixed-end zone of the stand bone, the legs, four half-ring pieces for the two 180 mm bends (a revolve cannot carry a ring section, so each bend is split on its plane), the top rail and the cross rail between the leg faces. Joints are the ground collars and the weld rings (referenced from the body), supports are two fixed blocks 48.3 x 15 x 48.3 mm under the leg axes at the footing. Masses are distributed along the tubes, so there are no extra mass solids.",
      "Tragwerk: jede Stabachse ist ein echtes Hohlrohr mit 3,2 mm Wand, 1,5 mm innerhalb der Körperhülle modelliert (45 mm außen statt 48,3 mm, damit es nicht mit der Körperhaut zusammenfällt und deren freie Fläche begräbt; A = 4,3 cm2 statt 4,6 cm2): die ersten 48,3 mm jedes Beins sind die Einspannzone der Standachse, dann die Beine, vier Halbringstücke für die beiden Bögen mit 180 mm Radius (eine Rotation trägt keinen Ringschnitt, daher ist jeder Bogen in seiner Ebene geteilt), der Oberholm und der Querholm zwischen den Beinflächen. Gelenke sind die Bodenmanschetten und die Schweißringe (aus dem Körper referenziert), Auflager sind zwei feste Blöcke 48,3 x 15 x 48,3 mm unter den Beinachsen am Fundament. Die Massen sind entlang der Rohre verteilt, daher gibt es keine zusätzlichen Massenkörper.")
sc.append(s)



def path_sections(pts, r):
    out = []
    n = len(pts)
    for i, p in enumerate(pts):
        a = pts[max(i - 1, 0)]
        b = pts[min(i + 1, n - 1)]
        phi = math.atan2(b[1] - a[1], b[0] - a[0])
        out.append(sec([p[0], p[1], 0], r, r, rotation=qz(phi)))
    return out


def quad(p0, c, p1, k):
    return [[(1 - t) ** 2 * p0[0] + 2 * (1 - t) * t * c[0] + t * t * p1[0], (1 - t) ** 2 * p0[1] + 2 * (1 - t) * t * c[1] + t * t * p1[1]] for t in [i / k for i in range(k + 1)]]


def hoop_path_q(k, r, y0=0.0):
    left = quad([-X, YB], [-X, 0.77585], [-0.19, 0.77585], k)
    right = [[-p[0], p[1]] for p in reversed(left)]
    return path_sections([[-X, y0]] + left + right + [[X, y0]], r)


def flatten(d, n=10):
    tok = d.replace(",", " ").split()
    pts, i, cur = [], 0, None
    while i < len(tok):
        c = tok[i]
        if c in "ML":
            cur = [float(tok[i + 1]), float(tok[i + 2])]
            pts.append(cur)
            i += 3
        elif c == "C":
            v = [float(x) for x in tok[i + 1:i + 7]]
            p0 = cur
            for k in range(1, n + 1):
                u = k / n
                pts.append([(1 - u) ** 3 * p0[j] + 3 * (1 - u) ** 2 * u * v[j] + 3 * (1 - u) * u * u * v[2 + j] + u ** 3 * v[4 + j] for j in range(2)])
            cur = [v[4], v[5]]
            i += 7
        else:
            i += 1
    return pts


def support(s, h, w=0.0483):
    for sg, nm in ((-1, "left"), (1, "right")):
        s.st(box(f"support-leg-{nm}-block", H, M, 0.15, [w, h, w], [sg * X, -h / 2, 0]), H, "support")


def rod(id, bone, x, y0, y1, r, tone=0.45):
    return revolve(id, bone, M, tone, "y", [x, y0, 0], [[0, 0], [0, r], [y1 - y0, r], [y1 - y0, 0], [0, 0]])


# ---- 1:100
s = Scale(100)
s.b(loft("hoop", H, M, 0.45, hoop_sections(3, RO, RO)))
s.b(cyl("crossbar", C, M, 0.45, "x", [0, 0, 0], 2 * XI + 0.004, RO))
s.thof("th-hoop", "hoop", "hoop-galv")
s.thof("th-crossbar", "crossbar", "crossbar-galv")
for sg, nm in ((-1, "l"), (1, "r")):
    s.st(tube_y(f"root-{nm}", S, sg * X, 0, 0.0483, SO, SI), S, "member")
    s.st(tube_y(f"leg-{nm}", H, sg * X, 0.0483, YB, SO, SI), H, "member")
for d in half_rings("bend-l", H, 0.45, True, SO, SI, 8) + half_rings("bend-r", H, 0.45, False, SO, SI, 8):
    s.st(d, H, "member")
s.st(tube_x("top", H, -(X - R), X - R, YB + R, SO, SI), H, "member")
s.st(tube_x("crossbar-tube", C, -XI, XI, 0, SO, SI), C, "member")
for sg, nm in ((-1, "l"), (1, "r")):
    s.st(revolve(f"weld-joint-{nm}", C, M, 0.3, "x", [sg * 0.31265, 0, 0], [[-0.0076, 0], [-0.0076, SO], [0.0076, SO], [0.0076, 0], [-0.0076, 0]], role="detail"), C, "joint")
support(s, 0.03)
s.dec("Body: the hoop is one loft of 11 round sections (a 3-segment chord per 180 mm bend instead of 10, sagitta 6 mm = under half the 15 mm feature) with the legs and top rail unchanged at 48.3 mm; the ground collars (8 mm) and the weld rings (9 mm) are below the 15 mm feature and merged into the tube, the cross rail is a plain cylinder 2 mm into each leg. The 1:100 drawing shows the weld only as a line, the leg only as a dot in plan.",
      "Körper: der Bügel ist eine Loft aus 11 runden Schnitten (3 Sehnen je Bogen von 180 mm statt 10, Pfeilhöhe 6 mm = unter der halben Einzelheit von 15 mm), Beine und Oberholm unverändert mit 48,3 mm; die Bodenmanschetten (8 mm) und die Schweißringe (9 mm) liegen unter der Einzelheit 15 mm und sind im Rohr aufgegangen, der Querholm ist ein glatter Zylinder, 2 mm in jedes Bein. Die Zeichnung 1:100 zeigt die Naht nur als Linie, das Bein im Grundriss nur als Punkt.")
s.dec("Thermal: hoop-galv and crossbar-galv are referenced from the two body tubes (single steel wall layer, B2); both regions keep a solid.",
      "Thermisch: hoop-galv und crossbar-galv werden aus den beiden Körperrohren referenziert (eine Stahlwandschicht, B2); beide Regionen behalten einen Körper.")
s.dec("Structure: the same hollow tube members as at 1:50 (45 x 3.2 mm, inside the envelope), the eight bend pieces reduced to 8 sectors, the ground joint merged into the fixed end zone, the two weld joints kept as 15 mm rings, supports are two 30 mm fixed blocks (two features) under the leg axes.",
      "Tragwerk: dieselben Hohlrohrstäbe wie bei 1:50 (45 x 3,2 mm, innerhalb der Hülle), die Bogenstücke mit 8 Sektoren, das Bodengelenk in der Einspannzone aufgegangen, die beiden Schweißgelenke als Ringe von 15 mm behalten, Auflager sind zwei feste Blöcke von 30 mm (zwei Einzelheiten) unter den Beinachsen.")
sc.append(s)

# ---- 1:200
s = Scale(200)
s.b(loft("hoop", H, M, 0.45, hoop_path_q(3, RO)))
s.b(cyl("crossbar", C, M, 0.45, "x", [0, 0, 0], 2 * X, RO))
s.thof("th-hoop", "hoop", "hoop-galv")
s.thof("th-crossbar", "crossbar", "crossbar-galv")
RQ = 0.0196
for sg, nm in ((-1, "l"), (1, "r")):
    s.st(rod(f"root-{nm}", S, sg * X, 0, 0.0483, RQ), S, "member")
s.st(loft("hoop-member", H, M, 0.45, hoop_path_q(2, RQ, 0.0483)), H, "member")
s.st(cyl("crossbar-member", C, M, 0.45, "x", [0, 0, 0], 2 * XI, RQ), C, "member")
support(s, 0.06)
s.dec("Body: the hoop follows the 1:200 drawing, whose bends are quadratic curves (185 mm across, 204 mm high, not the 180 mm circle of the larger scales): one loft of 9 round sections, 48.3 mm tube (above the 30 mm feature, so drawn true size), cross rail as a cylinder to the leg axes. Collars, welds and the bend-tangent marks are gone.",
      "Körper: der Bügel folgt der Zeichnung 1:200, deren Bögen quadratische Kurven sind (185 mm breit, 204 mm hoch, nicht der Kreis von 180 mm der größeren Maßstäbe): eine Loft aus 9 runden Schnitten, Rohr 48,3 mm (über der Einzelheit 30 mm, also in wahrer Größe), der Querholm als Zylinder bis zu den Beinachsen. Manschetten, Nähte und Tangentenmarken entfallen.")
s.dec("Thermal: both regions referenced from the body tubes.",
      "Thermisch: beide Regionen aus den Körperrohren referenziert.")
s.dec("Structure: the hollow sections are replaced by solid rods of equal bending stiffness (39.2 mm: I = pi/64 (48.3^4 - 41.9^4)), one lofted rod for the whole hoop above the fixed end zones, one for the cross rail, the end zones as short rods, supports two 60 mm blocks (two features); bends and welds are merged into the rods.",
      "Tragwerk: die Hohlquerschnitte sind durch Vollstäbe gleicher Biegesteifigkeit ersetzt (39,2 mm: I = pi/64 (48,3^4 - 41,9^4)), ein Loftstab für den ganzen Bügel über den Einspannzonen, einer für den Querholm, die Einspannzonen als kurze Stäbe, Auflager zwei Blöcke von 60 mm (zwei Einzelheiten); Bögen und Nähte gehen in den Stäben auf.")
sc.append(s)

# ---- 1:500
s = Scale(500)
arch = flatten([x for x in asset_of("bicycle-stand")["representations"]["500"]["geometry"]["elevation"] if x["id"] == "hoop"][0]["d"])
s.b(ext("hoop", H, M, 0.5, "xy", arch, -RO, RO))
s.b(box("crossbar", C, M, 0.5, [0.46, 0.15, 0.0483]))
s.th(loft("th-hoop", H, M, 0.45, hoop_sections(2, RO, RO)), "hoop-galv")
s.th(cyl("th-crossbar", C, M, 0.45, "x", [0, 0, 0], 2 * X, RO), "crossbar-galv")
RR = 0.02415
for sg, nm in ((-1, "l"), (1, "r")):
    s.st(box(f"root-{nm}", S, M, 0.45, [0.0752, 0.0752, 0.0752], [sg * X, 0.0376, 0]), S, "member")
s.st(loft("hoop-member", H, M, 0.45, hoop_path_q(2, RR, 0.075)), H, "member")
s.st(cyl("crossbar-member", C, M, 0.45, "x", [0, 0, 0], 2 * XI, RR), C, "member")
support(s, 0.15, 0.075)
s.dec("Body: the 1:500 drawing enlarges the 48.3 mm tube to a 160 mm wide band in elevation (the pen floor of 2 features) while front and plan keep the true 48.3 mm depth, so the hoop is one flat arch slab 160 x 48.3 mm extruded from the drawn outline, and the cross rail a 460 x 150 x 48.3 mm bar; the front now also draws the cross rail end-on (omission repaired in the asset, as at 1:100 and 1:200).",
      "Körper: die Zeichnung 1:500 vergrößert das Rohr 48,3 mm im Aufriss zu einem Band von 160 mm (Stiftgrenze von 2 Einzelheiten), während Ansicht und Grundriss die wahre Tiefe 48,3 mm behalten; der Bügel ist daher eine flache Bogenplatte 160 x 48,3 mm, aus dem gezeichneten Umriss extrudiert, der Querholm ein Balken 460 x 150 x 48,3 mm; die Ansicht zeichnet den Querholm jetzt auch stirnseitig (Auslassung im Asset behoben, wie bei 1:100 und 1:200).")
s.dec("Thermal: the two regions are referenced from the two body solids.",
      "Thermisch: die beiden Regionen sind aus den beiden Körpern referenziert.")
s.dec("Structure: three solid rods of the drawn tube size (48.3 mm), supports two 150 mm blocks.",
      "Tragwerk: drei Vollstäbe in der gezeichneten Rohrgröße (48,3 mm), Auflager zwei Blöcke von 150 mm.")
sc.append(s)
save_asset("bicycle-stand", sc)
