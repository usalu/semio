import sys, os
__here = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(__here, "author-39-lib.py")).read())
only = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
CO, ST = "concrete", "steel"
BO, HR = "body", "handrail"
KERB = [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [10.2, 0.7], [6.6, 0.4], [5.1, 0.4], [1.5, 0.1]]
POURS = [("run-1", [[1.5, 0], [5.1, 0.3], [5.1, 0]]), ("landing-middle", [[5.1, 0], [5.1, 0.3], [6.6, 0.3], [6.6, 0]]), ("run-2", [[6.6, 0], [6.6, 0.3], [10.2, 0.6], [10.2, 0]]), ("landing-top", [[10.2, 0], [10.2, 0.6], [11.7, 0.6], [11.7, 0]])]
RAIL = [(0, 0.85), (0.3, 0.85), (3.9, 1.15), (5.4, 1.15), (9.0, 1.45), (9.3, 1.45)]
R = 0.0212
POSTS = [(0.3, 0.1), (2.1, 0.25), (3.9, 0.4), (5.4, 0.4), (7.2, 0.55), (9.0, 0.7)]
WALL, WALL_IN = 0.0197, 0.0171
TOP = ["-v", "-u", "+u", "-w", "+w"]
SIDE = ["-v", "+v", "-u", "+u"]


RAIL3 = [(0, 0.85), (3.9, 1.15), (5.4, 1.15), (9.3, 1.45)]
NAMES5 = ("rail-ext-start", "rail-run-1", "rail-landing", "rail-run-2", "rail-ext-end")
NAMES3 = ("rail-run-1", "rail-landing", "rail-run-2")


def path_of(mode):
    return (RAIL, NAMES5) if mode in ("full", "five") else (RAIL3, NAMES3)


def rail_caps(s, mode, slender=False, r=R):
    pts, names = path_of(mode)
    out = []
    for k, nm in enumerate(names):
        a, b = pts[k], pts[k + 1]
        d = capsule(f"{nm}", HR, ST, 0.4, [a[0], a[1], 0.65], [b[0], b[1], 0.65], r, repeat={"count": 2, "translation": [0, 0, -1.3]})
        if slender and math.hypot(b[0] - a[0], b[1] - a[1]) / (2 * r) >= 20:
            d["slender"] = True
        out.append(s.b(d))
    if mode == "full":
        for nm, x, y in (("start", 0, 0.85), ("end", 9.3, 1.45)):
            out.append(s.b(capsule(f"rail-return-{nm}", HR, ST, 0.4, [x, y, 0.65], [x, y - 0.1, 0.65], r, repeat={"count": 2, "translation": [0, 0, -1.3]})))
    return out


def tube_ring(id, p0, p1, slender=False, rod=None):
    L = math.hypot(p1[0] - p0[0], p1[1] - p0[1])
    ax = [(p1[0] - p0[0]) / L, (p1[1] - p0[1]) / L, 0]
    prof = [[0, rod], [L, rod]] if rod else [[0, WALL_IN], [0, WALL], [L, WALL], [L, WALL_IN], [0, WALL_IN]]
    d = revolve(id, HR, ST, 0.5, ax, [p0[0], p0[1], 0.65], prof, repeat={"count": 2, "translation": [0, 0, -1.3]})
    if slender and L / 0.0424 >= 20:
        d["slender"] = True
    return d


def tubes(s, mode, slender=False, rod=None):
    pts, names = path_of(mode)
    for k, nm in enumerate(names):
        s.st(tube_ring(f"m-{nm}", pts[k], pts[k + 1], slender, rod), HR, "member")


def pads(s, thick, wide_only=False):
    for i, (cx, a, b) in enumerate(((2.5, 1.5, 3.5), (5, 4, 6), (8, 7, 9), (11, 10, 11.7))):
        s.st(box(f"support-ground-{i}", BO, CO, 0.5, [b - a, thick, 1.4], [(a + b) / 2, thick / 2, 0]), BO, "support")


def thermal_ramp(s, parts):
    for nm in parts:
        s.thof(f"th-top-{nm}", nm, "ramp-surface")
        s.thermal[-1]["sectionFaces"] = TOP
        s.thof(f"th-side-{nm}", nm, "ramp-side")
        s.thermal[-1]["sectionFaces"] = SIDE


def build50():
    s = Scale(50)
    for nm, pr in POURS:
        s.b(ext(f"pour-{nm}", BO, CO, 0.55, "xy", pr, -0.7, 0.7))
    s.b(ext("kerb-a", BO, CO, 0.65, "xy", KERB, 0.6, 0.7))
    s.b(ext("kerb-b", BO, CO, 0.65, "xy", KERB, -0.7, -0.6))
    rail_caps(s, "full")
    for k, (x, y) in enumerate(POSTS):
        s.b(box(f"post-{k}", HR, ST, 0.35, [0.05, 0.709, 0.05], [x, y + 0.02 + 0.3545, 0.65], repeat={"count": 2, "translation": [0, 0, -1.3]}))
        s.b(box(f"plate-{k}", HR, ST, 0.3, [0.12, 0.02, 0.1], [x, y + 0.01, 0.65], repeat={"count": 2, "translation": [0, 0, -1.3]}))
    pours = [f"pour-{nm}" for nm, _ in POURS]
    skins = [("run-1", [[1.5, 0], [5.1, 0.3], [5.1, 0.2], [2.7, 0]], 0.6), ("landing-middle", [[5.1, 0.2], [5.1, 0.3], [6.6, 0.3], [6.6, 0.2]], 0.6), ("run-2", [[6.6, 0.2], [6.6, 0.3], [10.2, 0.6], [10.2, 0.5]], 0.6), ("landing-top", [[10.2, 0.5], [10.2, 0.6], [11.7, 0.6], [11.7, 0.5]], 0.7)]
    for nm, pr, hw in skins:
        sk = ext(f"th-skin-{nm}", BO, CO, 0.55, "xy", pr, -hw, hw)
        sk["sectionFaces"] = TOP
        s.th(sk, "ramp-surface")
        s.thof(f"th-side-pour-{nm}", f"pour-{nm}", "ramp-side")
        s.thermal[-1]["sectionFaces"] = SIDE
    s.thof("th-kerb-a", "kerb-a", "kerb")
    s.thermal[-1]["sectionFaces"] = ["+w", "-v", "-u", "+u"]
    s.thof("th-kerb-b", "kerb-b", "kerb")
    s.thermal[-1]["sectionFaces"] = ["-w", "-v", "-u", "+u"]
    for r in s.body:
        if r["id"].startswith(("rail-", "post-", "plate-")):
            s.thof(f"th-{r['id']}", r["id"], "handrail-steel")
    for nm in pours:
        s.stof(f"m-{nm}", nm, BO, "member")
    tubes(s, "five")
    for k in range(len(POSTS)):
        s.stof(f"m-post-{k}", f"post-{k}", HR, "member")
        s.stof(f"j-plate-{k}", f"plate-{k}", HR, "joint")
    s.st(ellipsoid("mass-body", BO, CO, 0.4, [0.3, 0.12, 0.5], [8.127, 0.219, 0]), BO, "mass")
    pads(s, 0.05)
    s.dec("Body: the ramp as built: a solid reinforced-concrete wedge on the ground, poured in four sections that the 2D marks with joint lines (run 1 at 1:12 from 1.5 to 5.1 m, the middle landing to 6.6 m, run 2 to 10.2 m, the top landing to 11.7 m; 1.4 m wide, 0.30 / 0.60 m high), the two 100 mm wheel-guard kerbs on the slab edges over the two runs and the middle landing, and the handrails on both sides: a 42.4 mm tube bent to follow the runs with 0.30 m extensions and a 0.10 m return down at each end (capsule chain, the round ends form the kinks), six posts 50 x 50 mm per side standing on 120 x 100 x 20 mm base plates on the kerb tops (the plates lie flat on the 8.3 % slope, up to 5 mm in or out of the kerb). The painted walking markers of the plan are not physical and not modelled.",
          "Körper: die Rampe wie gebaut: ein massiver Stahlbetonkeil auf dem Boden, in vier Abschnitten betoniert, die die 2D mit Fugenlinien markiert (Lauf 1 mit 1:12 von 1,5 bis 5,1 m, das Zwischenpodest bis 6,6 m, Lauf 2 bis 10,2 m, das obere Podest bis 11,7 m; 1,4 m breit, 0,30 / 0,60 m hoch), die beiden Radabweiser 100 mm auf den Plattenrändern über beiden Läufen und dem Zwischenpodest und die Handläufe auf beiden Seiten: ein Rohr 42,4 mm, den Läufen folgend gebogen, mit Verlängerungen von 0,30 m und einer Rückführung von 0,10 m nach unten an jedem Ende (Kapselkette, die runden Enden bilden die Knicke), je Seite sechs Pfosten 50 x 50 mm auf Fußplatten 120 x 100 x 20 mm auf den Bordoberkanten (die Platten liegen flach auf dem Gefälle von 8,3 %, bis 5 mm im oder über dem Bord). Die aufgemalten Gehmarken des Grundrisses sind nicht körperlich und nicht modelliert.")
    s.dec("Thermal: the top region is the slab top of the four pours (sides, ends and underside are section faces, the underside lies on the ground), the side region the two side faces of the pours (3.51 m2 each, as the elevation area), the kerb region the top and the inner face of each kerb (outer face flush with the slab side), the handrail region references every tube, post and plate.",
          "Thermik: die Oberseitenregion ist die Plattenoberseite der vier Abschnitte (Seiten, Stirnen und Unterseite sind Schnittflächen, die Unterseite liegt am Boden), die Seitenregion die beiden Seitenflächen der Abschnitte (je 3,51 m2, wie die Fläche der Ansicht), die Bordregion Oberseite und Innenseite jedes Bords (Außenseite bündig mit der Plattenseite), die Handlaufregion verweist auf jedes Rohr, jeden Pfosten und jede Platte.")
    s.dec("Structure: the four pours as the members of the body bone (the section is the equivalent 1.4 m x 150 mm slab), the handrail tube as hollow 42.4 x 2.6 mm segments (wall 1.5 mm inside the body skin), posts as members and base plates as joints, a mass ellipsoid at the declared centre of mass of the concrete (8.13 m, 0.22 m), the four ground bearings as 50 mm contact pads under the declared 2 m footprints (the last one cut at the end of the ramp, 10 to 11.7 m).",
          "Tragwerk: die vier Abschnitte als Stäbe des Körperknochens (der Querschnitt ist die Ersatzplatte 1,4 m x 150 mm), das Handlaufrohr als hohle Stücke 42,4 x 2,6 mm (Wand 1,5 mm innerhalb der Körperhaut), Pfosten als Stäbe und Fußplatten als Gelenke, ein Massenellipsoid im angegebenen Schwerpunkt des Betons (8,13 m, 0,22 m), die vier Bodenauflager als Kontaktplatten 50 mm unter den angegebenen Aufstandsflächen von 2 m (die letzte am Rampenende gekürzt, 10 bis 11,7 m).")
    return s


def build100():
    s = Scale(100)
    s.b(ext("slab", BO, CO, 0.55, "xy", [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0]], -0.7, 0.7))
    s.b(ext("kerb-a", BO, CO, 0.65, "xy", KERB, 0.6, 0.7))
    s.b(ext("kerb-b", BO, CO, 0.65, "xy", KERB, -0.7, -0.6))
    rail_caps(s, "five")
    for k, (x, y) in enumerate(POSTS):
        s.b(box(f"post-{k}", HR, ST, 0.35, [0.05, 0.729, 0.05], [x, y + 0.0645 + 0.3, 0.65], repeat={"count": 2, "translation": [0, 0, -1.3]}))
    thermal_ramp(s, ["slab"])
    s.thof("th-kerb-a", "kerb-a", "kerb")
    s.thermal[-1]["sectionFaces"] = ["+w", "-v", "-u", "+u"]
    s.thof("th-kerb-b", "kerb-b", "kerb")
    s.thermal[-1]["sectionFaces"] = ["-w", "-v", "-u", "+u"]
    for r in s.body:
        if r["id"].startswith(("rail-", "post-")):
            s.thof(f"th-{r['id']}", r["id"], "handrail-steel")
    s.stof("m-slab", "slab", BO, "member")
    tubes(s, "three")
    for k in range(len(POSTS)):
        s.stof(f"m-post-{k}", f"post-{k}", HR, "member")
    s.st(ellipsoid("mass-body", BO, CO, 0.4, [0.32, 0.1, 0.55], [8.127, 0.219, 0]), BO, "mass")
    pads(s, 0.06)
    s.dec("Body: the pours are merged into one slab wedge (the 2D keeps the joint lines as strokes), kerbs as before; the handrail tubes without the end returns, posts as 50 mm bars to the underside of the rail, base plates (20 mm, below the drawn detail) merged into the posts.",
          "Körper: die Abschnitte sind zu einem Plattenkeil zusammengefasst (die 2D hält die Fugenlinien als Striche), Borde wie zuvor; die Handlaufrohre ohne die Endrückführungen, Pfosten als Stäbe 50 mm bis zur Handlaufunterseite, Fußplatten (20 mm, unter dem gezeichneten Detail) in die Pfosten übergegangen.")
    s.dec("Thermal: references with the same section faces as at 1:50 (one slab instead of four pours).",
          "Thermik: Verweise mit denselben Schnittflächen wie bei 1:50 (eine Platte statt vier Abschnitte).")
    s.dec("Structure: slab as member, hollow tube segments, posts, the mass ellipsoid and four ground pads; the joints (base plates) are merged into the posts.",
          "Tragwerk: Platte als Stab, hohle Rohrstücke, Pfosten, das Massenellipsoid und vier Bodenplatten; die Gelenke (Fußplatten) gehen in die Pfosten über.")
    return s


def build200():
    s = Scale(200)
    s.b(ext("slab", BO, CO, 0.55, "xy", [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0]], -0.7, 0.7))
    s.b(ext("kerb-a", BO, CO, 0.65, "xy", KERB, 0.6, 0.7))
    s.b(ext("kerb-b", BO, CO, 0.65, "xy", KERB, -0.7, -0.6))
    rail_caps(s, "three", slender=True)
    for k, (x, y) in enumerate(POSTS):
        pst = box(f"post-{k}", HR, ST, 0.35, [0.036, 0.75, 0.036], [x, y + 0.375, 0.65], repeat={"count": 2, "translation": [0, 0, -1.3]})
        pst["slender"] = True
        s.b(pst)
    thermal_ramp(s, ["slab"])
    s.thof("th-kerb-a", "kerb-a", "kerb")
    s.thermal[-1]["sectionFaces"] = ["+w", "-v", "-u", "+u"]
    s.thof("th-kerb-b", "kerb-b", "kerb")
    s.thermal[-1]["sectionFaces"] = ["-w", "-v", "-u", "+u"]
    for r in s.body:
        if r["id"].startswith(("rail-", "post-")):
            s.thof(f"th-{r['id']}", r["id"], "handrail-steel")
    s.stof("m-slab", "slab", BO, "member")
    tubes(s, "three", slender=True)
    for k in range(len(POSTS)):
        s.stof(f"m-post-{k}", f"post-{k}", HR, "member")
    pads(s, 0.07)
    s.dec("Body: slab wedge and kerbs as at 1:100; the handrail tubes are the true 42.4 mm tubes (`slender`, the 2D draws them as lines), posts 50 mm.",
          "Körper: Plattenkeil und Borde wie bei 1:100; die Handlaufrohre sind die wahren Rohre 42,4 mm (`slender`, die 2D zeichnet sie als Linien), Pfosten 50 mm.")
    s.dec("Thermal: as at 1:100.", "Thermik: wie bei 1:100.")
    s.dec("Structure: slab, hollow rail segments and posts as members, four ground pads of 50 mm; the mass ellipsoid is merged into the slab member.",
          "Tragwerk: Platte, hohle Handlaufstücke und Pfosten als Stäbe, vier Bodenplatten 50 mm; das Massenellipsoid geht im Plattenstab auf.")
    return s


def build500():
    s = Scale(500)
    s.b(ext("slab", BO, CO, 0.55, "xy", [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0]], -0.7, 0.7))
    rail_caps(s, "three", slender=True, r=0.0325)
    s.thof("th-top-slab", "slab", "ramp-surface")
    s.thermal[-1]["sectionFaces"] = TOP
    s.thof("th-side-slab", "slab", "ramp-side")
    s.thermal[-1]["sectionFaces"] = SIDE
    s.thof("th-kerb", "slab", "kerb")
    s.thermal[-1]["sectionFaces"] = ["-v", "+v", "-u", "+u", "-w", "+w"]
    for r in s.body:
        if r["id"].startswith("rail-"):
            s.thof(f"th-{r['id']}", r["id"], "handrail-steel")
    s.st(ext("m-slab", BO, CO, 0.5, "xy", [[2.4, 0.002], [2.4, 0.075], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0.002]], -0.7, 0.7), BO, "member")
    tubes(s, "three", slender=True, rod=0.0176)
    pads(s, 0.08)
    s.dec("Body: two masses: the slab wedge (kerbs, posts and joint lines are below the 75 mm feature and not drawn) and the two handrail tubes at their true 42.4 mm (`slender`, the 2D draws them as lines).",
          "Körper: zwei Massen: der Plattenkeil (Borde, Pfosten und Fugenlinien liegen unter dem Merkmal von 75 mm und sind nicht gezeichnet) und die zwei Handlaufrohre in ihrem wahren Durchmesser 42,4 mm (`slender`, die 2D zeichnet sie als Linien).")
    s.dec("Thermal: slab top and sides as at 1:100; the kerb region references the slab with all faces as section faces (the kerbs are merged into it, only the total is compared at this scale); rails reference the body.",
          "Thermik: Plattenoberseite und Seiten wie bei 1:100; die Bordregion verweist auf die Platte mit allen Flächen als Schnittflächen (die Borde sind darin aufgegangen, in diesem Maßstab wird nur die Summe verglichen); die Handläufe verweisen auf den Körper.")
    s.dec("Structure: the slab member starts where the wedge is as thick as the 75 mm feature (2.4 m, as the drawing of this scale did before the fix below), rails as solid rods of equal stiffness (35 mm: I = 7.6e-8 m4 of the tube), four 80 mm ground pads; posts are merged into the rails.",
          "Tragwerk: der Plattenstab beginnt, wo der Keil so dick ist wie das Merkmal von 75 mm (2,4 m, wie die Zeichnung dieses Maßstabs vor der unten genannten Korrektur), Handläufe als Vollstäbe gleicher Steifigkeit (35 mm: I = 7,6e-8 m4 des Rohrs), vier Bodenplatten 80 mm; die Pfosten gehen in den Handläufen auf.")
    return s


B = {50: build50, 100: build100, 200: build200, 500: build500}
save("components", "wheelchair-ramp", [B[k]() for k in only])
