"""Street lamp (author 42): python3 author-42-lamp.py [50 100 200 500]. Hollow tapered pole and arm tubes, thermal skins as closed outer envelopes with their end faces as section faces."""
import os, sys, json
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-42-lib.py")).read())

FD, PO, AR, HD = "foundation", "pole", "arm", "head"
GS, CO, AL, PC = "galv-steel", "concrete", "aluminium-anodised", "polycarbonate"
COVER_Y = [[0.13, 0.12], [0.155, 0.1105], [0.1775, 0.1005], [0.21, 0.092], [0.24, 0.0875], [0.27, 0.0852], [0.3, 0.0843]]
POLE = [[0, 0.085], [0.49993, 0.08345], [0.99944, 0.08132], [1.49812, 0.07867], [1.99556, 0.07556], [2.49132, 0.07202], [2.985, 0.06813], [3.47618, 0.06391], [3.96444, 0.05944], [4.44937, 0.05477], [4.93056, 0.04993], [5.40757, 0.04499], [5.88, 0.04]]
ARM = [[0, 0.035], [0.35, 0.031], [0.8, 0.017], [0.95, 0.010]]
COVER = [[0.13, 0.12], [0.155, 0.1105], [0.1775, 0.1005], [0.21, 0.092], [0.24, 0.0875], [0.27, 0.0852], [0.3, 0.0843]]
HOUSING = [
    {"at": [0, -0.005, 0], "rx": 0.07, "ry": 0.035, "exponent": 3, "rotation": [0.70689, 0.01766, 0.70689, -0.01766]},
    {"at": [0.05, -0.0025, 0], "rx": 0.125, "ry": 0.0575, "exponent": 3, "rotation": [0.70595, 0.04046, 0.70595, -0.04046]},
    {"at": [0.15, 0.009, 0], "rx": 0.13, "ry": 0.071, "exponent": 3, "rotation": [0.7071, 0.00354, 0.7071, -0.00354]},
    {"at": [0.25, 0.01, 0], "rx": 0.125, "ry": 0.068, "exponent": 3, "rotation": [0.69961, -0.10268, 0.69961, 0.10268]},
    {"at": [0.3, -0.005, 0], "rx": 0.08, "ry": 0.045, "exponent": 3, "rotation": [0.7047, -0.05832, 0.7047, 0.05832]},
    {"at": [0.33, -0.01, 0], "rx": 0.04, "ry": 0.015, "exponent": 3, "rotation": [0.7047, -0.05832, 0.7047, 0.05832]},
]


def shell(pts, t, tone=0.4):
    """Closed [axial, radial] ring: the outer profile and its inset by t (flat ends)."""
    return [list(p) for p in pts] + [[p[0], max(p[1] - t, 0.0005)] for p in reversed(pts)]


def tube(id, bone, mat, tone, pts, t, role="body", axis="x", origin=(0, 0, 0), tags=None):
    return revolve(id, bone, mat, tone, axis, origin, shell(pts, t), role=role, tags=tags)


def solid_rev(id, bone, mat, tone, pts, axis="x", origin=(0, 0, 0), role="body", faces=None):
    d = revolve(id, bone, mat, tone, axis, origin, [[pts[0][0], 0]] + [list(p) for p in pts] + [[pts[-1][0], 0]], role=role)
    if faces:
        d["sectionFaces"] = faces
    return d


def texture(d):
    d["texture"] = True
    return d


def nuts(sc, h=0.0125, r=0.014):
    for k, (x, z) in enumerate(((0.12, 0.12), (0.12, -0.12), (-0.12, -0.12), (-0.12, 0.12))):
        sc.b(revolve(f"base-nut-{k}", FD, GS, 0.2, "y", (x, 0.12, z), [[0, 0], [0, r], [h, r], [h, 0]], role="detail"))


def build50():
    sc = Scale(50)
    b = sc.b
    b(box("plinth", FD, CO, 0.65, [0.3, 0.1, 0.3], [0, 0.05, 0]))
    b(box("base-plate", FD, GS, 0.35, [0.3, 0.02, 0.3], [0, 0.11, 0], r=0.002))
    nuts(sc)
    b(tube("base-cover", FD, GS, 0.33, COVER_Y, 0.004, axis="y"))
    b(tube("pole", PO, GS, 0.4, POLE, 0.004))
    b(solid_rev("pole-cap", PO, GS, 0.3, [[5.86, 0.042], [5.87, 0.0415], [5.88, 0.0385], [5.888, 0.032], [5.892, 0.02], [5.893, 0.004]]))
    b(revolve("pole-collar", PO, GS, 0.3, "x", (0, 0, 0), [[5.6, 0.0435], [5.6, 0.055], [5.68, 0.055], [5.68, 0.0435]], role="detail"))
    b(texture(box("pole-hatch", PO, GS, 0.3, [0.5, 0.09, 0.004], [0.8, 0, -0.0815], r=0.002, role="detail")))
    b(tube("arm", AR, AL, 0.5, ARM, 0.003))
    b(rod("arm-brace", AR, AL, 0.45, (0.02, -0.46, 0), (0.62, -0.012, 0), 0.012, hole=0.0105, off=(0, 0)))
    b(ellipsoid("arm-brace-joint", AR, AL, 0.3, [0.02, 0.02, 0.02], [0.62, -0.012, 0], role="detail"))
    b(ext("arm-clamp", AR, AL, 0.3, "zx", rrect(0, 0, 0.1, 0.1, 0.008), -0.05, 0.05, holes=[circle_pts(0, 0, 0.0435, 20)], role="detail"))
    d = loft("head-housing", HD, AL, 0.5, HOUSING)
    b(d)
    b(revolve("head-spigot", HD, AL, 0.35, "x", (-0.04, 0.005, 0), [[0, 0], [0, 0.028], [0.07, 0.028], [0.07, 0]], role="detail"))
    for k in range(6):
        b(texture(box(f"head-fin-{k}", HD, AL, 0.3, [0.004, 0.023, 0.15], [0.15 + 0.028 * k, 0.0775, 0], role="detail")))
    b(ellipsoid("lens", HD, PC, 0.85, [0.135, 0.03, 0.1], [0.165, -0.058, 0], role="glazing"))
    return sc


def thermal50(sc):
    sc.thof("th-plinth", "plinth", "plinth-concrete")
    sc.thof("th-base-plate", "base-plate", "pole-galv")
    for k in range(4):
        sc.thof(f"th-base-nut-{k}", f"base-nut-{k}", "pole-galv")
    sc.th(solid_rev("th-base-cover", FD, GS, 0.33, COVER_Y, axis="y", faces=["-v", "+v"]), "pole-galv")
    sc.th(solid_rev("th-pole", PO, GS, 0.4, POLE, faces=["-u", "+u"]), "pole-galv")
    sc.thof("th-pole-cap", "pole-cap", "pole-galv")
    sc.thof("th-pole-collar", "pole-collar", "pole-galv")
    sc.th(solid_rev("th-arm", AR, AL, 0.5, ARM, faces=["-u", "+u"]), "arm-alu")
    sc.thof("th-arm-brace", "arm-brace", "arm-alu")
    sc.thof("th-arm-brace-joint", "arm-brace-joint", "arm-alu")
    sc.thof("th-arm-clamp", "arm-clamp", "arm-alu")
    sc.thof("th-head-housing", "head-housing", "lamp-housing")
    sc.thof("th-head-spigot", "head-spigot", "lamp-housing")
    sc.thof("th-lens", "lens", "lamp-lens")
    d = revolve("th-lamp-plume", HD, AL, 0.9, "x", (0.1163, -0.05, 0), [[0, 0], [0, 0.06], [0.0075, 0.06], [0.0075, 0]], role="detail", tags=["lamp-plume"])
    d["sectionFaces"] = ["-u", "+u", "-v", "+v", "-w", "+w"]
    sc.th(d, "lamp-housing")


def structure50(sc):
    sc.st(box("member-plinth", FD, CO, 0.65, [0.29, 0.09, 0.29], [0, 0.05, 0]), "foundation", "member")
    sc.st(revolve("member-pole", PO, GS, 0.4, "x", (0, 0, 0), shell([[a, r - 0.001] for a, r in POLE], 0.002)), "pole", "member")
    sc.st(revolve("member-arm", AR, AL, 0.5, "x", (0, 0, 0), shell([[a, r - 0.0008] for a, r in ARM], 0.0015)), "arm", "member")
    sc.st(rod("member-arm-brace", AR, AL, 0.45, (0.02, -0.46, 0), (0.62, -0.012, 0), 0.0105, hole=0.0095), "arm", "member")
    sc.st(box("member-head-core", HD, AL, 0.5, [0.24, 0.06, 0.12], [0.15, 0.0, 0]), "head", "member")
    sc.st(ext("joint-base-ring", FD, GS, 0.3, "zx", circle_pts(0, 0, 0.0845, 24), 0.12, 0.14, holes=[circle_pts(0, 0, 0.0805, 24)]), "foundation", "joint")
    sc.st(ext("joint-arm-clamp", AR, AL, 0.3, "zx", circle_pts(0, 0, 0.0525, 20), -0.04, 0.04, holes=[circle_pts(0, 0, 0.0455, 20)]), "arm", "joint")
    sc.st(ellipsoid("joint-brace", AR, AL, 0.3, [0.015, 0.015, 0.015], [0.62, -0.012, 0]), "arm", "joint")
    sc.st(revolve("joint-head-spigot", HD, AL, 0.35, "x", (-0.035, 0.005, 0), [[0, 0], [0, 0.024], [0.06, 0.024], [0.06, 0]]), "head", "joint")
    sc.st(ellipsoid("mass-foundation", FD, CO, 0.5, [0.05, 0.03, 0.05], [0, 0.11, 0]), "foundation", "mass")
    sc.st(ellipsoid("mass-pole", PO, GS, 0.5, [0.15, 0.03, 0.03], [2.94, 0, 0]), "pole", "mass")
    sc.st(ellipsoid("mass-arm", AR, AL, 0.5, [0.05, 0.015, 0.015], [0.475, 0, 0]), "arm", "mass")
    sc.st(ellipsoid("mass-head", HD, AL, 0.5, [0.03, 0.015, 0.03], [0.115, -0.03, 0]), "head", "mass")
    sc.st(box("support-foundation-support", FD, CO, 0.1, [0.28, 0.02, 0.28], [0, 0.01, 0]), "foundation", "support")


def bez(p0, p1, p2, p3, n):
    out = []
    for k in range(n):
        s = k / (n - 1)
        out.append([round(sum(c * q[j] for c, q in zip(((1 - s) ** 3, 3 * s * (1 - s) ** 2, 3 * s * s * (1 - s), s ** 3), (p0, p1, p2, p3))), 5) for j in (0, 1)])
    return out


POLE100 = bez((0, 0.085), (2, 0.08), (4, 0.06), (5.88, 0.04), 7)
POLE200 = bez((0, 0.09), (2, 0.085), (4, 0.065), (5.88, 0.045), 3)
POLE500 = [[0, 0.07], [5.88, 0.05]]
ARM100 = [[0, 0.035], [0.45, 0.029], [0.95, 0.01]]
HOUSING100 = [HOUSING[0], HOUSING[1], HOUSING[2], HOUSING[4], HOUSING[5]]
HOUSING200 = [HOUSING[0], HOUSING[2], HOUSING[4], HOUSING[5]]
FEAT = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def plume(den):
    f = FEAT[den]
    d = revolve("th-lamp-plume", HD, AL, 0.9, "x", (0.1163, -0.05, 0), [[0, 0], [0, 0.06], [f, 0.06], [f, 0]], role="detail", tags=["lamp-plume"])
    d["sectionFaces"] = ["-u", "+u", "-v", "+v", "-w", "+w"]
    return d


def common_thermal(sc, den, pole, arm, brace=True):
    sc.thof("th-plinth", "plinth", "plinth-concrete")
    sc.th(solid_rev("th-pole", PO, GS, 0.4, pole, faces=["-u", "+u"]), "pole-galv")
    sc.th(solid_rev("th-arm", AR, AL, 0.5, arm, faces=["-u", "+u"]), "arm-alu")
    if brace:
        sc.thof("th-arm-brace", "arm-brace", "arm-alu")
    sc.thof("th-head-housing", "head-housing", "lamp-housing")
    sc.th(plume(den), "lamp-housing")


def build100():
    sc = Scale(100)
    b = sc.b
    b(box("plinth", FD, CO, 0.65, [0.3, 0.12, 0.3], [0, 0.06, 0]))
    b(tube("base-cover", FD, GS, 0.33, COVER_Y[::3], 0.006, axis="y"))
    b(tube("pole", PO, GS, 0.4, POLE100, 0.006))
    b(revolve("pole-collar", PO, GS, 0.3, "x", (0, 0, 0), [[5.6, 0.0435], [5.6, 0.055], [5.68, 0.055], [5.68, 0.0435]], role="detail"))
    b(tube("arm", AR, AL, 0.5, ARM100, 0.004))
    b(rod("arm-brace", AR, AL, 0.45, (0.02, -0.46, 0), (0.62, -0.012, 0), 0.0085, n=12))
    b(loft("head-housing", HD, AL, 0.5, HOUSING100))
    b(ellipsoid("lens", HD, PC, 0.85, [0.135, 0.03, 0.1], [0.165, -0.058, 0], role="glazing"))
    return sc


def thermal100(sc):
    common_thermal(sc, 100, POLE100, ARM100)
    sc.th(solid_rev("th-base-cover", FD, GS, 0.33, COVER_Y[::3], axis="y", faces=["-v", "+v"]), "pole-galv")
    sc.thof("th-pole-collar", "pole-collar", "pole-galv")
    sc.thof("th-lens", "lens", "lamp-lens")


def structure100(sc):
    sc.st(box("member-plinth", FD, CO, 0.65, [0.29, 0.1, 0.29], [0, 0.06, 0]), "foundation", "member")
    sc.st(revolve("member-pole", PO, GS, 0.4, "x", (0, 0, 0), shell([[a, r - 0.001] for a, r in POLE100], 0.004)), "pole", "member")
    sc.st(revolve("member-arm", AR, AL, 0.5, "x", (0, 0, 0), shell([[a, r - 0.0008] for a, r in ARM100], 0.003)), "arm", "member")
    sc.st(rod("member-arm-brace", AR, AL, 0.45, (0.02, -0.46, 0), (0.62, -0.012, 0), 0.0075, hole=0.005, n=12), "arm", "member")
    sc.st(box("member-head-core", HD, AL, 0.5, [0.24, 0.06, 0.12], [0.15, 0.0, 0]), "head", "member")
    sc.st(ext("joint-base-ring", FD, GS, 0.3, "zx", circle_pts(0, 0, 0.0845, 20), 0.12, 0.14, holes=[circle_pts(0, 0, 0.0775, 20)]), "foundation", "joint")
    sc.st(ext("joint-arm-clamp", AR, AL, 0.3, "zx", circle_pts(0, 0, 0.0525, 16), -0.01, 0.01, holes=[circle_pts(0, 0, 0.0455, 16)]), "arm", "joint")
    sc.st(ellipsoid("joint-brace", AR, AL, 0.3, [0.0125, 0.0125, 0.0125], [0.62, -0.012, 0]), "arm", "joint")
    sc.st(ellipsoid("mass-foundation", FD, CO, 0.5, [0.05, 0.03, 0.05], [0, 0.11, 0]), "foundation", "mass")
    sc.st(ellipsoid("mass-pole", PO, GS, 0.5, [0.15, 0.03, 0.03], [2.94, 0, 0]), "pole", "mass")
    sc.st(ellipsoid("mass-arm", AR, AL, 0.5, [0.05, 0.015, 0.015], [0.475, 0, 0]), "arm", "mass")
    sc.st(ellipsoid("mass-head", HD, AL, 0.5, [0.03, 0.015, 0.03], [0.115, -0.03, 0]), "head", "mass")
    sc.st(box("support-foundation-support", FD, CO, 0.1, [0.28, 0.02, 0.28], [0, 0.01, 0]), "foundation", "support")


def build200():
    sc = Scale(200)
    b = sc.b
    b(box("plinth", FD, CO, 0.65, [0.3, 0.12, 0.3], [0, 0.06, 0]))
    b(tube("pole", PO, GS, 0.4, POLE200, 0.012))
    b(tube("arm", AR, AL, 0.5, [[0, 0.025], [0.82, 0.025]], 0.008))
    b(rod("arm-brace", AR, AL, 0.45, (0.02, -0.46, 0), (0.62, -0.012, 0), 0.0165, n=16))
    b(loft("head-housing", HD, AL, 0.5, HOUSING200))
    b(ellipsoid("lens", HD, PC, 0.85, [0.125, 0.012, 0.1], [0.165, -0.06, 0], role="glazing"))
    return sc


def thermal200(sc):
    common_thermal(sc, 200, POLE200, [[0, 0.025], [0.82, 0.025]])
    sc.thof("th-lens", "lens", "lamp-lens")
    sc.th(solid_rev("th-pole-collar", PO, GS, 0.3, [[5.6, 0.055], [5.68, 0.055]], faces=["-u", "+u"]), "pole-galv")


def structure200(sc):
    sc.st(box("member-plinth", FD, CO, 0.65, [0.27, 0.09, 0.27], [0, 0.06, 0]), "foundation", "member")
    sc.st(revolve("member-pole", PO, GS, 0.4, "x", (0, 0, 0), shell([[a, r - 0.002] for a, r in POLE200], 0.01)), "pole", "member")
    sc.st(revolve("member-arm", AR, AL, 0.5, "x", (0, 0, 0), shell([[0.01, 0.023], [0.81, 0.023]], 0.006)), "arm", "member")
    sc.st(rod("member-arm-brace", AR, AL, 0.45, (0.02, -0.46, 0), (0.62, -0.012, 0), 0.0155, hole=0.009, n=16), "arm", "member")
    sc.st(box("member-head-core", HD, AL, 0.5, [0.24, 0.06, 0.12], [0.15, 0.0, 0]), "head", "member")
    sc.st(ellipsoid("mass-foundation", FD, CO, 0.5, [0.05, 0.03, 0.05], [0, 0.11, 0]), "foundation", "mass")
    sc.st(ellipsoid("mass-pole", PO, GS, 0.5, [0.15, 0.03, 0.03], [2.94, 0, 0]), "pole", "mass")
    sc.st(ellipsoid("mass-arm", AR, AL, 0.5, [0.05, 0.015, 0.015], [0.475, 0, 0]), "arm", "mass")
    sc.st(ellipsoid("mass-head", HD, AL, 0.5, [0.03, 0.015, 0.03], [0.115, -0.03, 0]), "head", "mass")
    sc.st(box("support-foundation-support", FD, CO, 0.1, [0.28, 0.03, 0.28], [0, 0.015, 0]), "foundation", "support")


def build500():
    sc = Scale(500)
    b = sc.b
    b(box("plinth", FD, CO, 0.65, [0.3, 0.12, 0.3], [0, 0.06, 0]))
    b(tube("pole", PO, GS, 0.4, POLE500, 0.02))
    b(tube("arm", AR, AL, 0.5, [[0, 0.045], [0.72, 0.045]], 0.015, axis="x"))
    b(box("head", HD, AL, 0.5, [0.33, 0.12, 0.24], [0.165, 0.0, 0]))
    return sc


def thermal500(sc):
    sc.thof("th-plinth", "plinth", "plinth-concrete")
    sc.thof("th-pole", "pole", "pole-galv")
    sc.thof("th-arm", "arm", "arm-alu")
    sc.thof("th-head", "head", "lamp-housing")
    sc.th(ellipsoid("th-lens", HD, PC, 0.85, [0.1, 0.0375, 0.0375], [0.165, -0.0, 0], role="glazing"), "lamp-lens")
    sc.th(plume(500), "lamp-housing")


def structure500(sc):
    sc.st(box("member-plinth", FD, CO, 0.65, [0.27, 0.09, 0.27], [0, 0.06, 0]), "foundation", "member")
    sc.st(revolve("member-pole", PO, GS, 0.4, "x", (0, 0, 0), shell([[0.01, 0.065], [5.87, 0.046]], 0.03)), "pole", "member")
    sc.st(revolve("member-arm", AR, AL, 0.5, "x", (0, 0, 0), shell([[0.01, 0.043], [0.71, 0.043]], 0.02)), "arm", "member")
    sc.st(box("member-head-core", HD, AL, 0.5, [0.3, 0.09, 0.2], [0.165, 0.0, 0]), "head", "member")
    sc.st(ellipsoid("mass-foundation", FD, CO, 0.5, [0.0375, 0.0375, 0.0375], [0, 0.11, 0]), "foundation", "mass")
    sc.st(ellipsoid("mass-pole", PO, GS, 0.5, [0.0375, 0.0375, 0.0375], [2.94, 0, 0]), "pole", "mass")
    sc.st(ellipsoid("mass-arm", AR, AL, 0.5, [0.0375, 0.0375, 0.0375], [0.475, 0, 0]), "arm", "mass")
    sc.st(ellipsoid("mass-head", HD, AL, 0.5, [0.0375, 0.0375, 0.0375], [0.115, -0.0, 0]), "head", "mass")
    sc.st(box("support-foundation-support", FD, CO, 0.1, [0.28, 0.075, 0.28], [0, 0.0375, 0]), "foundation", "support")


DEC = {
    50: (
        "Body: hand-built from the 1:50 drawings: concrete plinth 0.30 x 0.10 m, base plate with four nuts, a flared 4 mm cover shell, the 5.88 m galvanised pole as a hollow revolve of the drawn taper (0.17 m at the base to 0.08 m at the top, 4 mm wall) with domed cap and the arm clamp collar, the aluminium arm as a hollow tapered tube with a ring clamp, a 24 mm brace tube to the pole, the luminaire housing as the drawn loft with its spigot, the polycarbonate lens as a thin glazing dome; the pole hatch and the six cooling fins are texture decals (below every feature).",
        "Körper: von Hand aus den Zeichnungen 1:50 gebaut: Betonsockel 0,30 x 0,10 m, Fußplatte mit vier Muttern, eine ausgestellte Hülle von 4 mm, der 5,88 m lange verzinkte Mast als hohler Drehkörper der gezeichneten Verjüngung (0,17 m unten bis 0,08 m oben, 4 mm Wand) mit gewölbter Kappe und Klemmring für den Ausleger, der Aluminium-Ausleger als hohles konisches Rohr mit Ringklemme, ein Strebenrohr von 24 mm zum Mast, das Leuchtengehäuse als gezeichneter Loft mit seinem Zapfen, die Polycarbonat-Linse als dünne Glaskalotte; Mastklappe und die sechs Kühlrippen sind Texturdecals (unter jeder Schwelle).",
    ),
    100: (
        "Body: the 1:100 drawings: plinth block 0.30 x 0.12 m with the plate merged in (nuts and plate edge are below the scale), the flared cover, the pole as a 6 mm hollow tube on the drawn Bezier taper in six segments, the collar ring, a hollow tapered arm, a 17 mm brace tube, the housing loft in five sections and the glazing lens; cap, hatch, fins and spigot are not drawn.",
        "Körper: die Zeichnungen 1:100: Sockelblock 0,30 x 0,12 m mit eingearbeiteter Platte (Muttern und Plattenkante liegen unter dem Maßstab), die ausgestellte Hülle, der Mast als hohles Rohr von 6 mm auf der gezeichneten Bézier-Verjüngung in sechs Abschnitten, der Klemmring, ein hohler konischer Ausleger, ein Strebenrohr von 17 mm, das Gehäuse als Loft aus fünf Schnitten und die Glaslinse; Kappe, Klappe, Rippen und Zapfen sind nicht gezeichnet.",
    ),
    200: (
        "Body: modelled against the 1:200 drawings: the plinth block, the pole as a 12 mm hollow tube in two segments (0.18 m to 0.09 m), the arm as the 50 mm tube the pen-width stroke shows, a 33 mm brace tube, the housing loft in four sections and a thin glazing lens; cover, collar, cap and details are below the scale.",
        "Körper: gegen die Zeichnungen 1:200 modelliert: der Sockelblock, der Mast als hohles Rohr von 12 mm in zwei Abschnitten (0,18 m auf 0,09 m), der Ausleger als das Rohr von 50 mm, das der stiftbreite Strich zeigt, ein Strebenrohr von 33 mm, das Gehäuse als Loft aus vier Schnitten und eine dünne Glaslinse; Hülle, Klemmring, Kappe und Einzelheiten liegen unter dem Maßstab.",
    ),
    500: (
        "Body: the four drawn parts: the plinth block, the pole as a conical tube of 0.14 m to 0.10 m, the arm as the 90 mm tube of the stroke and the head as the drawn 0.33 x 0.12 x 0.24 m block; brace, lens and plume are not drawn at this scale.",
        "Körper: die vier gezeichneten Teile: der Sockelblock, der Mast als konisches Rohr von 0,14 m auf 0,10 m, der Ausleger als das Rohr von 90 mm des Strichs und der Kopf als der gezeichnete Block 0,33 x 0,12 x 0,24 m; Strebe, Linse und Fahne sind in diesem Maßstab nicht gezeichnet.",
    ),
}
TH = (
    "Thermal: the pole and the base cover are closed outer envelopes with their end faces as section faces (the declared 2.4 m2 is the outer tube surface; a hollow reference would count the inside as well), all other regions reference the body solids.",
    "Thermik: Mast und Fußhülle sind geschlossene Außenhüllen mit ihren Stirnflächen als Schnittflächen (die angegebenen 2,4 m2 sind die äußere Rohrfläche, ein hohler Verweis würde die Innenseite mitzählen), alle anderen Regionen verweisen auf die Körper.",
)
ST = (
    "Structure: the pole and the arm are the real hollow tubes (2 mm and 1.5 mm walls inside the skin), the brace a thin tube, the plinth and the head core blocks inside their bodies; joints are the weld ring at the plate, the clamp ring, the brace pin and the head spigot; masses lumped at the declared centres of the four bones (the 70 kg pole lump at 2.94 m), the fixed support is the plinth footing.",
    "Tragwerk: Mast und Ausleger sind die echten hohlen Rohre (Wände 2 mm und 1,5 mm unter der Haut), die Strebe ein dünnes Rohr, Sockel und Kopfkern Blöcke in ihren Körpern; Gelenke sind der Schweißring an der Platte, der Klemmring, der Strebenbolzen und der Kopfzapfen; Massen in den angegebenen Schwerpunkten der vier Knochen (das 70-kg-Stück des Mastes bei 2,94 m), das feste Auflager ist die Sockelsohle.",
)

if __name__ == "__main__":
    scales = [int(a) for a in sys.argv[1:]] or [50]
    out = []
    for den in scales:
        sc = {50: build50, 100: build100, 200: build200, 500: build500}[den]()
        {50: thermal50, 100: thermal100, 200: thermal200, 500: thermal500}[den](sc)
        {50: structure50, 100: structure100, 200: structure200, 500: structure500}[den](sc)
        sc.dec(*DEC.get(den, DEC[50]))
        sc.dec(*TH)
        sc.dec(*ST)
        out.append(sc)
    save("objects", "street-lamp", out)
