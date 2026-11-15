"""Storage crate stack (author 42): python3 author-42-crate.py [50 100 200 500]. Crates are hollow tapered trays on a foot ring with a stacking-ledge rim; thermal regions are the outer envelopes (the declared 0.6 m2 per crate is the outer wall area)."""
import os, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-42-lib.py")).read())

MAT = "hdpe-crate"
N = 5
TAPER = 1.09


def zx(w, u, c=0.0):
    """Profile in the zx plane: first coordinate w (along z), second u (along x)."""
    return rrect(0, 0, w, u, c) if c else rect(0, 0, w, u)


def taper(d, k):
    d["taperTo"] = [1, k]
    return d


def tray(i, y0, y1, w, u, wall, c=0.006, tone=0.5, hollow=True, k=None, ids="tray", role="body"):
    d = ext(f"{ids}-{i}", f"crate-{i}", MAT, tone, "zx", zx(w, u, c), y0, y1, holes=[zx(w - 2 * wall, u - 2 * wall, max(0.0015, c - wall))] if hollow else None, role=role)
    if k:
        taper(d, k)
    return d


def ring(i, y0, y1, w, u, w2, u2, c, tone, ids):
    return ext(f"{ids}-{i}", f"crate-{i}", MAT, tone, "zx", zx(w, u, c), y0, y1, holes=[zx(w2, u2, c)])


def decals(i, y):
    """Handle slots as texture decals (their depth is below every feature): three on each long wall, one on each end."""
    s = 1 + (TAPER - 1) * (y - 0.02) / 0.26
    zo, xo = 0.17 * s, 0.28 * s
    bone = f"crate-{i}"
    out = []
    for nm, sz, at, rep in (("a", [0.1, 0.04, 0.002], [-0.17, y, zo], 3), ("b", [0.1, 0.04, 0.002], [-0.17, y, -zo], 3), ("c", [0.002, 0.04, 0.14], [xo, y, 0], None), ("d", [0.002, 0.04, 0.14], [-xo, y, 0], None)):
        d = box(f"slot-{nm}-{i}", bone, MAT, 0.1, sz, at, role="detail", repeat={"count": rep, "translation": [0.17, 0, 0]} if rep else None)
        d["texture"] = True
        out.append(d)
    return out


def ribs(i):
    out = []
    for nm, sz in (("x", [0.5, 0.004, 0.006]), ("z", [0.006, 0.004, 0.28])):
        d = box(f"floor-rib-{nm}-{i}", f"crate-{i}", MAT, 0.4, sz, [0, 0.034, 0], role="detail")
        d["texture"] = True
        out.append(d)
    return out


def build50():
    sc = Scale(50)
    for i in range(1, N + 1):
        sc.b(ring(i, 0, 0.02, 0.3, 0.54, 0.284, 0.524, 0.006, 0.35, "foot"))
        sc.b(box(f"floor-{i}", f"crate-{i}", MAT, 0.45, [0.56, 0.012, 0.34], [0, 0.026, 0], r=0.004))
        sc.b(tray(i, 0.02, 0.28, 0.34, 0.56, 0.004, k=TAPER))
        sc.b(ring(i, 0.28, 0.3, 0.4, 0.62, 0.284, 0.524, 0.006, 0.55, "rim"))
        for d in decals(i, 0.14):
            sc.b(d)
        for d in ribs(i):
            sc.b(d)
    return sc


def faces(d):
    d["sectionFaces"] = ["+v", "-v"]
    return d


def thermal50(sc):
    for i in range(1, N + 1):
        sc.th(faces(tray(i, 0.02, 0.28, 0.34, 0.56, 0, k=TAPER, hollow=False, ids="th-envelope")), f"crate-{i}")
        sc.th(faces(tray(i, 0.28, 0.3, 0.4, 0.62, 0, hollow=False, ids="th-rim-edge")), f"crate-{i}")
        sc.th(faces(tray(i, 0.0, 0.02, 0.3, 0.54, 0, hollow=False, ids="th-foot-edge")), f"crate-{i}")


def structure50(sc):
    for i in range(1, N + 1):
        bone = f"crate-{i}"
        sc.st(ring(i, 0.001, 0.019, 0.298, 0.538, 0.286, 0.526, 0.006, 0.35, "member-foot"), bone, "member")
        sc.st(taper(ext(f"member-tray-{i}", bone, MAT, 0.5, "zx", zx(0.338, 0.558, 0.006), 0.021, 0.279, holes=[zx(0.334, 0.554, 0.003)]), TAPER), bone, "member")
        sc.st(ring(i, 0.281, 0.299, 0.398, 0.618, 0.286, 0.526, 0.006, 0.55, "member-rim"), bone, "member")
        sc.st(box(f"member-floor-{i}", bone, MAT, 0.45, [0.55, 0.01, 0.33], [0, 0.026, 0]), bone, "member")
        sc.st(ellipsoid(f"mass-{i}", bone, MAT, 0.5, [0.04, 0.04, 0.04], [0, 0.15, 0]), bone, "mass")
        if i < N:
            sc.st(ring(i, 0.29, 0.299, 0.298, 0.538, 0.286, 0.526, 0.006, 0.7, "joint-stack"), bone, "joint")
    sc.st(ring(1, 0.0, 0.01, 0.296, 0.536, 0.288, 0.528, 0.006, 0.1, "support-base-contact"), "crate-1", "support")


def build100():
    sc = Scale(100)
    for i in range(1, N + 1):
        sc.b(box(f"floor-{i}", f"crate-{i}", MAT, 0.45, [0.56, 0.02, 0.34], [0, 0.01, 0]))
        sc.b(tray(i, 0.02, 0.275, 0.34, 0.56, 0.006, c=0.0, k=TAPER))
        sc.b(ring(i, 0.275, 0.3, 0.4, 0.62, 0.284, 0.524, 0.0, 0.55, "rim"))
    return sc


def thermal100(sc):
    for i in range(1, N + 1):
        sc.th(faces(tray(i, 0.02, 0.275, 0.34, 0.56, 0, c=0.008, k=TAPER, hollow=False, ids="th-envelope")), f"crate-{i}")
        sc.th(faces(tray(i, 0.275, 0.3, 0.4, 0.62, 0, c=0.008, hollow=False, ids="th-rim-edge")), f"crate-{i}")


def structure100(sc):
    for i in range(1, N + 1):
        bone = f"crate-{i}"
        sc.st(taper(ext(f"member-tray-{i}", bone, MAT, 0.5, "zx", zx(0.336, 0.556), 0.022, 0.273, holes=[zx(0.328, 0.548)]), TAPER), bone, "member")
        sc.st(ring(i, 0.2765, 0.2985, 0.398, 0.618, 0.286, 0.526, 0.0, 0.55, "member-rim"), bone, "member")
        sc.st(box(f"member-floor-{i}", bone, MAT, 0.45, [0.55, 0.015, 0.33], [0, 0.01, 0]), bone, "member")
        sc.st(ellipsoid(f"mass-{i}", bone, MAT, 0.5, [0.04, 0.04, 0.04], [0, 0.15, 0]), bone, "mass")
        if i < N:
            sc.st(ring(i, 0.283, 0.2985, 0.398, 0.618, 0.286, 0.526, 0.0, 0.7, "joint-stack"), bone, "joint")
    sc.st(box("support-base-contact", "crate-1", MAT, 0.1, [0.54, 0.015, 0.3], [0, 0.0075, 0]), "crate-1", "support")


def build200():
    sc = Scale(200)
    for i in range(1, N + 1):
        sc.b(box(f"floor-{i}", f"crate-{i}", MAT, 0.45, [0.56, 0.03, 0.34], [0, 0.015, 0]))
        sc.b(tray(i, 0.03, 0.3, 0.34, 0.56, 0.015, c=0.008, k=1.14))
    return sc


def thermal200(sc):
    for i in range(1, N + 1):
        sc.th(faces(tray(i, 0.03, 0.3, 0.34, 0.56, 0, c=0.008, k=1.14, hollow=False, ids="th-envelope")), f"crate-{i}")


def structure200(sc):
    for i in range(1, N + 1):
        bone = f"crate-{i}"
        sc.st(taper(ext(f"member-tray-{i}", bone, MAT, 0.5, "zx", zx(0.336, 0.556, 0.008), 0.032, 0.298, holes=[zx(0.31, 0.53, 0.006)]), 1.14), bone, "member")
        sc.st(ellipsoid(f"mass-{i}", bone, MAT, 0.5, [0.05, 0.05, 0.05], [0, 0.15, 0]), bone, "mass")
    sc.st(box("support-base-contact", "crate-1", MAT, 0.1, [0.54, 0.03, 0.3], [0, 0.015, 0]), "crate-1", "support")


def build500():
    sc = Scale(500)
    for i in range(1, N + 1):
        sc.b(box(f"crate-block-{i}", f"crate-{i}", MAT, 0.5, [0.62, 0.3, 0.4], [0, 0.15, 0]))
    return sc


def thermal500(sc):
    for i in range(1, N + 1):
        sc.thof(f"th-crate-block-{i}", f"crate-block-{i}", f"crate-{i}")


def structure500(sc):
    for i in range(1, N + 1):
        bone = f"crate-{i}"
        sc.st(ext(f"member-shell-{i}", bone, MAT, 0.5, "zx", zx(0.396, 0.616), 0.002, 0.298, holes=[zx(0.3, 0.52)]), bone, "member")
        sc.st(ellipsoid(f"mass-{i}", bone, MAT, 0.5, [0.0375, 0.0375, 0.0375], [0, 0.15, 0]), bone, "mass")
    sc.st(box("support-base-contact", "crate-1", MAT, 0.1, [0.5, 0.075, 0.3], [0, 0.0375, 0]), "crate-1", "support")


DEC = {
    50: (
        "Body: five identical HDPE crates on their own bones, each hand-built from the 1:50 drawings: a 20 mm foot ring (8 mm wall, 0.54 x 0.30 m), a 12 mm floor plate, the 4 mm wall tray tapering from 0.56 x 0.34 m at the floor to 0.61 x 0.37 m at the rim, and the 20 mm rim ring 0.62 x 0.40 m whose inner opening is a stacking ledge that carries the foot ring of the next crate (the drawn foot, wall and rim sizes do not otherwise touch each other). Handle slots and floor ribs are texture decals (below every feature).",
        "Körper: fünf gleiche PE-HD-Kisten auf eigenen Knochen, je von Hand aus den Zeichnungen 1:50 gebaut: ein Fußring von 20 mm (8 mm Wand, 0,54 x 0,30 m), eine Bodenplatte von 12 mm, die Wannenwand von 4 mm, die sich von 0,56 x 0,34 m am Boden auf 0,61 x 0,37 m am Rand weitet, und der Randring von 20 mm 0,62 x 0,40 m, dessen innere Öffnung als Stapelleiste den Fußring der nächsten Kiste trägt (die gezeichneten Fuß-, Wand- und Randmaße berühren sich sonst nicht). Griffschlitze und Bodenrippen sind Texturdecals (unter jeder Schwelle).",
    ),
    100: (
        "Body: the 1:100 drawings build each crate as the 20 mm floor plate, the 6 mm wall tray tapering to the drawn 0.60 x 0.38 m top and the 25 mm rim ring with its stacking ledge; foot ring, handle slots and floor ribs are below the scale or not drawn.",
        "Körper: die Zeichnungen 1:100 bauen jede Kiste als Bodenplatte von 20 mm, die Wannenwand von 6 mm, die sich auf die gezeichnete Oberkante 0,60 x 0,38 m weitet, und den Randring von 25 mm mit seiner Stapelleiste; Fußring, Griffschlitze und Bodenrippen liegen unter dem Maßstab oder sind nicht gezeichnet.",
    ),
    200: (
        "Body: each crate as one 15 mm wall tray tapering from 0.56 x 0.34 m to the drawn 0.62 x 0.40 m on a 30 mm floor plate (the drawing shows one tapered outline per crate; foot, rim and slots are merged into it).",
        "Körper: jede Kiste als eine Wanne mit 15 mm Wand, die sich von 0,56 x 0,34 m auf die gezeichneten 0,62 x 0,40 m weitet, auf einer Bodenplatte von 30 mm (die Zeichnung zeigt eine geneigte Kontur je Kiste; Fuß, Rand und Schlitze sind darin zusammengefasst).",
    ),
    500: (
        "Body: each crate is the drawn 0.62 x 0.30 x 0.40 m block, five of them stacked.",
        "Körper: jede Kiste ist der gezeichnete Block 0,62 x 0,30 x 0,40 m, fünf übereinander.",
    ),
}
TH = (
    "Thermal: the declared 0.6 m2 per crate is the outer wall area, so each region is the outer envelope of its crate (tapered wall block, rim and foot ring edges) with its top and bottom faces as section faces; the inside of the tray, the floor faces and the open top are not an exchange surface of the stack.",
    "Thermik: die angegebenen 0,6 m2 je Kiste sind die äußere Wandfläche, daher ist jede Region die Außenhülle ihrer Kiste (Wandblock, Rand- und Fußringkanten) mit Ober- und Unterseite als Schnittflächen; Wanneninnenseite, Bodenflächen und offene Oberseite sind keine Austauschfläche des Stapels.",
)
ST = (
    "Structure: per crate the foot ring, the wall tray (2 mm wall inside the 4 mm skin), the rim ledge and the floor plate are members; the stack joint is the ring where a foot ring meets the ledge below, the support is the foot ring of the lowest crate, one mass lump of 1.9 kg at the declared centre of each crate.",
    "Tragwerk: je Kiste sind Fußring, Wannenwand (2 mm Wand innerhalb der 4 mm Haut), Randleiste und Bodenplatte Stäbe; die Stapelfuge ist der Ring, wo ein Fußring auf die Leiste darunter trifft, das Auflager ist der Fußring der untersten Kiste, ein Massenstück von 1,9 kg im angegebenen Schwerpunkt jeder Kiste.",
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
    save("objects", "storage-crate-stack", out)
