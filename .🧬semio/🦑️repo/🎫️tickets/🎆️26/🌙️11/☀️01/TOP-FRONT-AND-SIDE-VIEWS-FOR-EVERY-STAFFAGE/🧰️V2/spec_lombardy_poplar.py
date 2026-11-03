from crowns import *

LEVELS = [("1", 4.77, 6.76, 0.0), ("2", 9.66, 11.69, 0.0), ("3", 15.07, 16.99, 0.0), ("4", 19.8, 21.62, 0.0)]
RV = 3.2


def lobes():
    L = []
    for n, zl, zr, _ in LEVELS:
        rv = RV - 0.1 * (int(n) - 1)
        L.append(lobe("l" + n, "fol-l" + n, "fol-l" + n, -0.53, 0.0, zl, 1.57, rv, "fol-l" + n, 1, rhy=1.5))
        L.append(lobe("r" + n, "fol-r" + n, "fol-r" + n, 0.50, 0.0, zr, 1.55, rv, "fol-r" + n, 1, rhy=1.5))
        L.append(lobe("ll" + n, "fol-l" + n, "fol-l" + n, -0.05, 0.62, zl + 0.9, 1.45, rv - 0.2, "fol-l" + n, 1, rhy=1.55))
        L.append(lobe("rl" + n, "fol-r" + n, "fol-r" + n, 0.05, -0.62, zr - 0.9, 1.45, rv - 0.2, "fol-r" + n, 1, rhy=1.55))
        L.append(lobe("ld" + n, "fol-l" + n, "fol-l" + n, 0.2, -0.45, zl + 1.6, 1.3, rv - 0.5, "fol-l" + n, 3, rhy=1.3))
        L.append(lobe("rd" + n, "fol-r" + n, "fol-r" + n, -0.2, 0.45, zr - 1.5, 1.3, rv - 0.5, "fol-r" + n, 3, rhy=1.3))
    L.append(lobe("top", "fol-top", "fol-top", -0.13, 0.0, 23.16, 1.04, 2.84, "fol-top", 1, rhy=1.1))
    for l in L:
        l["att"] = l["z"] - 1.2
    return L


def branch_front(b, sc, nm, bone, like, y0, z0, y1, z1, w0, w1, z):
    dx, dy = y1 - y0, z1 - z0
    ln = math.hypot(dx, dy)
    nx, ny = -dy / ln, dx / ln
    pts = [(y0 - nx * w0, z0 - ny * w0), (y0 + nx * w0, z0 + ny * w0), (y1 + nx * w1, z1 + ny * w1), (y1 - nx * w1, z1 - ny * w1)]
    taper(b, sc, "front", nm, bone, z, like, pts)


def build(b):
    LB = lobes()
    for sc in ("50", "100", "200"):
        emit_lobes(b, sc, LB)
    # bole: front
    taper(b, "50", "front", "root-flare", "root-plate", 3.0, "root-flare", [(-0.62, 0.0), (0.62, 0.0), (0.3, 0.3), (-0.3, 0.3)])
    for n, (y0, y1, w0, w1) in {"1": (0.3, 6.3, 0.3, 0.26), "2": (6.3, 12.3, 0.26, 0.2), "3": (12.3, 18.3, 0.2, 0.15), "4": (18.3, 24.6, 0.15, 0.06)}.items():
        taper(b, "50", "front", "trunk-" + n, "trunk-" + n, 3.1, "trunk-" + n, [(-w0, y0), (w0, y0), (w1, y1), (-w1, y1)])
    taper(b, "100", "front", "bole", "trunk-1", 2.0, "bole", [(-0.62, 0.0), (0.62, 0.0), (0.3, 0.3), (0.3, 6.3), (0.14, 24.6), (-0.14, 24.6), (-0.3, 6.3), (-0.3, 0.3)])
    taper(b, "200", "front", "bole", "trunk-1", 2.0, "bole", [(-0.55, 0.0), (0.55, 0.0), (0.27, 0.35), (0.12, 24.6), (-0.12, 24.6), (-0.27, 0.35)])
    taper(b, "500", "front", "bole", "trunk-1", 1.0, "bole", [(-0.6, 0.0), (0.6, 0.0), (0.25, 0.4), (0.1, 24.6), (-0.1, 24.6), (-0.25, 0.4)])
    # lateral branches in front (not the elevation's fan)
    br = {"50": 0.14, "100": 0.12}
    for sc, w in br.items():
        for n, zl, zr, _ in LEVELS:
            bl = {"1": 3.3, "2": 8.1, "3": 13.8, "4": 19.8}[n]
            brr = {"1": 5.1, "2": 10.1, "3": 15.8, "4": 21.8}[n]
            branch_front(b, sc, "br-l" + n, "br-l" + n, "br-l" + n, 0.0, bl, 0.95, bl + 2.2, w, 0.06, 2.4)
            branch_front(b, sc, "br-r" + n, "br-r" + n, "br-r" + n, 0.0, brr, -0.95, brr + 2.2, w, 0.06, 2.4)
    for sc in ("50", "100", "200"):
        disc(b, sc, "plan", "bole", "trunk-1", 1.0, 0.0, 0.0, 0.5 if sc == "50" else 0.45, 0.5 if sc == "50" else 0.45, "bole" if sc != "50" else "trunk-1")
    # hints in plan at 1:50: branches
    if True:
        for n, zl, zr, _ in LEVELS:
            stroke(b, "50", "plan", "pl-br-l" + n, "br-l" + n, 7.0, [(0.0, 0.0), (-0.3, 0.3), (-0.5, 0.45)], region="branch-bark", role="texture")
            stroke(b, "50", "plan", "pl-br-r" + n, "br-r" + n, 7.0, [(0.0, 0.0), (0.3, -0.3), (0.5, -0.45)], region="branch-bark", role="texture")
    # 1:500
    fr = fit(outline(0.0, 12.9, 1.8, 13.1, n=18, k=4, amp=0.05, ph=1.0, top=0.995, bottom=0.78), -2.2, 2.2, 1.1, 26.0)
    b.add("500", "front", "crown", "trunk-2", 2.0, d=smooth(fr), like="crown")
    pl = fit(outline(0.0, 0.0, 2.0, 2.0, n=14, k=5, amp=0.05, ph=0.4), -2.11, 2.05, -2.2, 2.2)
    b.add("500", "plan", "crown", "trunk-2", 2.0, d=smooth(pl), like="crown")
    disc(b, "500", "plan", "bole", "trunk-1", 1.0, 0.0, 0.0, 0.5, 0.5, "bole")


def decisions():
    return {
        "50": {
            "silhouette": ("Front view: the same narrow column seen from the side with the eight branch levels staggered in height and the lateral limbs rising from the trunk; plan view: the column from above as four stacked rings of rounded lobes round the trunk.", "Vorderansicht: dieselbe schmale Säule von der Seite mit den acht gegeneinander versetzten Astetagen und den seitlichen Ästen am Stamm; Draufsicht: die Säule von oben als vier übereinanderliegende Ringe aus gerundeten Lappen um den Stamm."),
            "retained": ("Root flare and four trunk segments with the limbs bound to the same bones as the side view; every leaf mass drawn again with its own outline and, on the main masses, an inner arc; short branch lines in the plan.", "Wurzelanlauf und vier Stammabschnitte mit den Ästen an denselben Knochen wie in der Seitenansicht; jede Laubmasse mit eigener Kontur und bei den Hauptmassen mit einem Innenbogen neu gezeichnet; kurze Astlinien in der Draufsicht."),
            "merged": ("Plan view: the small diagonal masses lie inside the rings of the main masses.", "Draufsicht: die kleinen diagonalen Massen liegen innerhalb der Ringe der Hauptmassen."),
            "omitted": ("Bark plugs and single leaves in both views.", "Rindenpfropfen und Einzelblätter in beiden Ansichten."),
        },
        "100": {
            "silhouette": ("Front view of the column with staggered leaf masses on a single bole; plan view as rounded lobes in rings round the trunk.", "Vorderansicht der Säule mit versetzten Laubmassen an einem einzigen Schaft; Draufsicht als gerundete Lappen in Ringen um den Stamm."),
            "retained": ("The bole and the eight limbs in front, the trunk disc in the plan; the main and the lateral leaf masses of every level.", "Der Schaft und die acht Äste vorn, die Stammscheibe in der Draufsicht; die Haupt- und die seitlichen Laubmassen jeder Etage."),
            "merged": ("The diagonal masses are part of the lateral masses.", "Die diagonalen Massen gehören zu den seitlichen Massen."),
            "omitted": ("Inner arcs and branch lines in the plan.", "Innenbögen und Astlinien in der Draufsicht."),
        },
        "200": {
            "silhouette": ("Front and plan view as four levels of leaf masses round a single bole.", "Vorder- und Draufsicht als vier Etagen aus Laubmassen um einen einzigen Schaft."),
            "retained": ("Bole and trunk disc, the main leaf masses of each level and the top mass.", "Schaft und Stammscheibe, die Hauptlaubmassen jeder Etage und die Wipfelmasse."),
            "merged": ("Limbs vanish inside the leaf masses.", "Die Äste verschwinden in den Laubmassen."),
            "omitted": ("Limbs and all detail lines.", "Äste und alle Detaillinien."),
        },
        "500": {
            "silhouette": ("Front view as one slim crown column on the bole; plan view as one rounded crown disc.", "Vorderansicht als eine schlanke Kronensäule auf dem Schaft; Draufsicht als eine gerundete Kronenscheibe."),
            "retained": ("The column outline in front and the lobed disc outline from above, with the trunk disc.", "Die Säulenkontur vorn und die gelappte Scheibenkontur von oben, mit der Stammscheibe."),
            "merged": ("All leaf masses and limbs are one crown shape.", "Alle Laubmassen und Äste sind eine Kronenform."),
            "omitted": ("Every inner structure.", "Jede innere Struktur."),
        },
    }
