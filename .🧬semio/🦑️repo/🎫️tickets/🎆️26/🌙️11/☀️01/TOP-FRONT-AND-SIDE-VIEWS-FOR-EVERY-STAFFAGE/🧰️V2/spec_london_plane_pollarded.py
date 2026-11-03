from crowns import *
from spec_lombardy_poplar import branch_front

F = "{}-mass"


def lobes():
    L = [lobe("core", "fol-core", "fol-core", 0.0, 0.0, 6.49, 1.38, 1.62, "fol-core-mass", 1)]
    L.append(lobe("top", "fol-top", "fol-top", 0.0, 0.0, 8.04, 1.01, 0.83, "fol-top-mass", 1))
    L += ring("o", 6.25, 2.4, 1.0, 1.27, [0, 180], "fol-w2", "fol-e2", F, 1)
    L += ring("ol", 6.3, 2.28, 1.0, 1.27, [90, 270], "fol-w2", "fol-e2", F, 1)
    L += ring("od", 6.35, 2.2, 0.95, 1.2, [45, 135, 225, 315], "fol-w2", "fol-e2", F, 3)
    L += ring("m", 7.4, 1.28, 0.9, 1.27, [0, 180, 60, 120, 240, 300], "fol-w1", "fol-e1", F, 1, imps=[1, 1, 2, 2, 2, 2])
    L += ring("lo", 5.43, 1.6, 0.88, 0.84, [0, 180, 90, 270], "fol-low-w", "fol-low-e", F, 2, imps=[1, 1, 2, 2])
    for l in L:
        l["att"] = 4.9
    return L


def build(b):
    LB = lobes()
    for sc in ("50", "100", "200"):
        emit_lobes(b, sc, LB)
    for sc in ("50", "100", "200", "500"):
        taper(b, sc, "front", "stem", "trunk", 1.0, "stem", [(-0.67, 0.0), (0.67, 0.0), (0.38, 0.5), (0.33, 3.8), (0.3, 4.02), (-0.3, 4.02), (-0.33, 3.8), (-0.38, 0.5)])
        disc(b, sc, "plan", "stem", "trunk", 1.0, 0.0, 0.0, 0.42, 0.42, "stem")
    for sc in ("50", "100", "200"):
        taper(b, sc, "front", "head", "head", 2.0, "head", [(-0.48, 3.73), (0.48, 3.73), (0.42, 4.5), (-0.42, 4.5)])
        disc(b, sc, "plan", "head", "head", 2.0, 0.0, 0.0, 0.46, 0.46, "head")
    # lateral knuckles and shoots in front (a pollard head sends its knuckles sideways)
    kn = {"w2": (0.5, 4.1, 0.95, 4.6), "w1": (0.3, 4.4, 0.55, 5.1), "c": (0.0, 4.5, 0.05, 5.1), "e1": (-0.3, 4.4, -0.55, 5.1), "e2": (-0.5, 4.1, -0.95, 4.6)}
    for sc in ("50", "100", "200"):
        for k, (y0, z0, y1, z1) in kn.items():
            branch_front(b, sc, "knuckle-" + k, "k-" + k, "knuckle-" + k, y0, z0, y1, z1, 0.2, 0.14, 2.1)
    sh = {"w2a": ("s-w2a", 0.95, 4.6, 2.1, 5.3), "w1a": ("s-w1a", 0.5, 5.0, 0.75, 6.6), "ca": ("s-ca", 0.0, 5.05, 0.0, 7.1), "e1a": ("s-e1a", -0.5, 5.0, -0.75, 6.6), "e2a": ("s-e2a", -0.95, 4.6, -2.1, 5.3)}
    for sc in ("50", "100"):
        for k, (bone, y0, z0, y1, z1) in sh.items():
            branch_front(b, sc, "shoot-" + k, bone, "shoot-" + k[:-1] + "-a", y0, z0, y1, z1, 0.1, 0.05, 2.2)
    for k in ("w2", "w1", "c", "e1", "e2"):
        pass
    for sc in ("50",):
        for az, (bn, nm) in zip((180, 120, 90, 60, 0), (("s-w2a", "w2"), ("s-w1a", "w1"), ("s-ca", "c"), ("s-e1a", "e1"), ("s-e2a", "e2"))):
            a = math.radians(az)
            stroke(b, sc, "plan", "pl-shoot-" + nm, bn, 7.0, [(0.0, 0.0), (1.1 * math.cos(a), 1.1 * math.sin(a)), (2.0 * math.cos(a), 2.0 * math.sin(a))], region="shoot-bark")
    fr = fit(outline(0.0, 0.0, 3.2, 2.0, n=16, k=5, amp=0.06, ph=0.4, bottom=0.9), -3.25, 3.25, 4.5, 8.9)
    b.add("500", "front", "crown", "head", 3.0, d=smooth(fr), like="crown")
    pl = fit(outline(0.0, 0.0, 3.2, 3.2, n=14, k=6, amp=0.06, ph=0.9), -3.38, 3.4, -3.25, 3.25)
    b.add("500", "plan", "crown", "head", 3.0, d=smooth(pl), like="crown")
    taper(b, "500", "front", "head", "head", 2.0, "head", [(-0.5, 3.7), (0.5, 3.7), (0.4, 4.6), (-0.4, 4.6)])


def decisions():
    return {
        "50": {
            "silhouette": ("Front view: the pollarded head with its five knuckles spread sideways and the leaf masses ordered by depth; plan view: the crown from above as rings of rounded lobes round the head.", "Vorderansicht: der Kopfschnitt mit seinen fünf zur Seite gespreizten Knorren und den nach der Tiefe geordneten Laubmassen; Draufsicht: die Krone von oben als Ringe gerundeter Lappen um den Kopf."),
            "retained": ("Trunk, head, knuckles and shoots in front, trunk and head discs and radial shoot lines in plan; each leaf mass again with its own outline and an inner arc on the main masses.", "Stamm, Kopf, Knorren und Triebe vorn, Stamm- und Kopfscheibe und radiale Trieblinien in der Draufsicht; jede Laubmasse mit eigener Kontur und bei den Hauptmassen mit Innenbogen."),
            "merged": ("The diagonal masses between the main masses.", "Die diagonalen Massen zwischen den Hauptmassen."),
            "omitted": ("Bark scars, leaves and the second shoot order in plan.", "Rindennarben, Blätter und die zweite Triebordnung in der Draufsicht."),
        },
        "100": {
            "silhouette": ("Front view with head, knuckles, shoots and the leaf masses of the crown, plan view with the lobed crown disc and the head in the middle.", "Vorderansicht mit Kopf, Knorren, Trieben und den Laubmassen der Krone, Draufsicht mit der gelappten Kronenscheibe und dem Kopf in der Mitte."),
            "retained": ("Main and lateral leaf masses, the shade core, trunk and head.", "Haupt- und Seitenlaubmassen, der Schattenkern, Stamm und Kopf."),
            "merged": ("Diagonal masses are merged into their neighbours.", "Diagonale Massen sind in die Nachbarn einbezogen."),
            "omitted": ("Inner arcs, plan shoot lines and scars.", "Innenbögen, Trieblinien in der Draufsicht und Narben."),
        },
        "200": {
            "silhouette": ("Front and plan view as the main leaf masses round the head.", "Vorder- und Draufsicht als die Hauptlaubmassen um den Kopf."),
            "retained": ("Trunk, head, knuckles, the core, the ring masses and the top mass.", "Stamm, Kopf, Knorren, der Kern, die Ringmassen und die Wipfelmasse."),
            "merged": ("Shoots disappear inside the leaf masses.", "Die Triebe verschwinden in den Laubmassen."),
            "omitted": ("Shoots and every detail line.", "Triebe und alle Detaillinien."),
        },
        "500": {
            "silhouette": ("Front view as one crown mass on the pollarded head and trunk; plan view as one lobed crown disc.", "Vorderansicht als eine Kronenmasse auf Kopfschnitt und Stamm; Draufsicht als eine gelappte Kronenscheibe."),
            "retained": ("Trunk, head and the crown outline.", "Stamm, Kopf und die Kronenkontur."),
            "merged": ("All leaf masses and shoots are one shape.", "Alle Laubmassen und Triebe sind eine Form."),
            "omitted": ("Knuckles and every inner structure.", "Knorren und jede innere Struktur."),
        },
    }
