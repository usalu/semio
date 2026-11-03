from crowns import *

F = "{}-mass"


def lobes():
    L = []
    L.append(lobe("core-low", "fol-core-low", "fol-core-low", 0.0, 0.0, 2.05, 0.84, 0.55, "fol-core-low-mass", 1))
    L.append(lobe("core-up", "fol-core-up", "fol-core-up", 0.06, 0.0, 3.11, 0.84, 0.40, "fol-core-up-mass", 1))
    t1 = ring("t1", 1.72, 1.46, 1.12, 0.27, [0, 180, 90, 270], "fol-t1w", "fol-t1e", F, 1, jit=0.0)
    t1 += ring("t1d", 1.80, 1.15, 0.95, 0.25, [45, 135, 225, 315], "fol-t1w", "fol-t1e", F, 3, jit=0.05)
    t2 = ring("t2", 2.55, 1.50, 1.06, 0.27, [0, 180, 60, 120, 240, 300], "fol-t2w", "fol-t2e", F, 1, imps=[1, 1, 2, 2, 2, 2])
    t3 = ring("t3", 3.30, 0.62, 0.88, 0.27, [0, 180, 90, 270], "fol-t3w", "fol-t3e", F, 1, imps=[1, 1, 2, 2])
    top = [lobe("top", "fol-top", "fol-top", 0.05, 0.0, 3.98, 0.94, 0.44, "fol-top-mass", 1)]
    for l in t1 + t2 + t3:
        l["att"] = l["z"] - 0.55
    return L + t1 + t2 + t3 + top


def build(b):
    LB = lobes()
    for sc in SCALES:
        if sc == "500":
            continue
        emit_lobes(b, sc, LB)
        limbs(b, sc, LB, "limb-bark")
    # stems, front
    for sc in ("50", "100", "200"):
        taper(b, sc, "front", "root-collar", "root-plate", 1.0, "root-collar", [(-0.3, 0.0), (0.3, 0.0), (0.11, 0.26), (-0.11, 0.26)])
        taper(b, sc, "front", "wood-stem-c", "stem-c", 1.2, "wood-stem-c", [(-0.085, 0.25), (0.085, 0.25), (0.055, 2.95), (-0.055, 2.95)])
        taper(b, sc, "front", "wood-stem-w", "stem-w", 1.21, "wood-stem-w", [(0.0, 0.2), (0.15, 0.2), (0.66, 2.43), (0.57, 2.43)])
        taper(b, sc, "front", "wood-stem-e", "stem-e", 1.22, "wood-stem-e", [(-0.14, 0.2), (0.0, 0.2), (-0.53, 2.57), (-0.62, 2.57)])
        disc(b, sc, "plan", "root-collar", "root-plate", 1.0, 0.0, 0.0, 0.3, 0.3, "root-collar")
        if sc != "200":
            taper(b, sc, "front", "wood-stem-c-top", "stem-c-top", 1.25, "wood-stem-c", [(-0.045, 2.95), (0.045, 2.95), (0.02, 3.95), (-0.02, 3.95)])
    for sc in ("50", "100"):
        disc(b, sc, "plan", "wood-stem-w", "stem-w", 1.1, -0.04, 0.34, 0.1, 0.1, "wood-stem-w")
        disc(b, sc, "plan", "wood-stem-e", "stem-e", 1.1, 0.05, -0.34, 0.09, 0.09, "wood-stem-e")
        disc(b, sc, "plan", "wood-stem-c", "stem-c", 1.1, 0.0, 0.0, 0.09, 0.09, "wood-stem-c")
    # 1:500 silhouettes
    sil_front = [(-2.2, 1.45), (-0.9, 1.5), (0.9, 1.5), (2.2, 1.45), (2.65, 1.75), (2.35, 2.1), (2.55, 2.45), (2.35, 2.85), (1.5, 3.1), (1.45, 3.45), (0.95, 3.6), (0.9, 4.1), (0.45, 4.42), (-0.45, 4.42), (-0.9, 4.1), (-0.95, 3.6), (-1.45, 3.45), (-1.5, 3.1), (-2.35, 2.85), (-2.55, 2.45), (-2.35, 2.1), (-2.65, 1.75)]
    sil_front = fit(sil_front, -2.6, 2.6, 1.45, 4.42)
    taper(b, "500", "front", "stems", "stem-c", 1.0, "stems", [(-0.3, 0.0), (0.3, 0.0), (0.09, 2.95), (-0.09, 2.95)])
    b.add("500", "front", "crown", "stem-c", 3.0, d=smooth(sil_front), like="crown")
    sil_plan = outline(0.0, 0.0, 2.45, 2.45, n=14, k=7, amp=0.06, ph=0.7, k2=3, amp2=0.03)
    sil_plan = fit(sil_plan, -2.58, 2.68, -2.6, 2.6)
    disc(b, "500", "plan", "stems", "root-plate", 1.0, 0.0, 0.0, 0.3, 0.3, "stems")
    b.add("500", "plan", "crown", "stem-c", 3.0, d=smooth(sil_plan), like="crown")


def decisions():
    return {
        "50": {
            "silhouette": ("Front view: the same three layered tiers seen from the side, wider tiers behind, nearer ones in front; plan view: the crown from above as a stack of lobed discs with the top tuft in the middle.", "Vorderansicht: dieselben drei Etagen von der Seite, die weiter hinten liegenden Etagen verdeckt von den vorderen; Draufsicht: die Krone von oben als Stapel gelappter Scheiben mit dem Wipfelbüschel in der Mitte."),
            "retained": ("In the front view three stems and the root collar, in the plan view the collar and the stem tops; every foliage pad is drawn again in both views with its own outline and, on the main pads, an inner arc, plus thin limb lines from the stems to the pads.", "In der Vorderansicht drei Stämme und der Wurzelhals, in der Draufsicht Wurzelhals und Stammenden; jedes Laubpolster ist in beiden Ansichten mit eigener Kontur und bei den Hauptpolstern mit einem Innenbogen neu gezeichnet, dazu dünne Astlinien von den Stämmen zu den Polstern."),
            "merged": ("The small diagonal pads between the main pads of a tier overlap into the tier outline.", "Die kleinen diagonalen Polster zwischen den Hauptpolstern einer Etage gehen in die Etagenkontur über."),
            "omitted": ("Single leaves and lenticels are not repeated in the front and plan views.", "Einzelblätter und Lentizellen werden in Vorder- und Draufsicht nicht wiederholt."),
        },
        "100": {
            "silhouette": ("Front view with the tiers as overlapping lobed pads, plan view with the lobed crown disc from above.", "Vorderansicht mit den Etagen als überlappenden gelappten Polstern, Draufsicht mit der gelappten Kronenscheibe von oben."),
            "retained": ("Stems and root collar in front, the collar and stem tops in plan, all main pads of the four tiers and the top tuft, thin limb lines under the pads.", "Stämme und Wurzelhals vorn, Wurzelhals und Stammenden in der Draufsicht, alle Hauptpolster der vier Etagen und das Wipfelbüschel, dünne Astlinien unter den Polstern."),
            "merged": ("The diagonal pads of the tiers are merged into the neighbouring pads.", "Die diagonalen Polster der Etagen sind in die benachbarten Polster einbezogen."),
            "omitted": ("Single leaves, lenticels and the inner arcs of the lower pads are left out.", "Einzelblätter, Lentizellen und die Innenbögen der unteren Polster entfallen."),
        },
        "200": {
            "silhouette": ("Front and plan view as the main foliage masses only: the tier pads and the top tuft, as lobed ovals from the side and from above.", "Vorder- und Draufsicht nur mit den Hauptlaubmassen: den Etagenpolstern und dem Wipfelbüschel als gelappten Ovalen von der Seite und von oben."),
            "retained": ("Three stems and the root collar in front; the pads of the three tiers, the core and the top in both views.", "Drei Stämme und der Wurzelhals vorn; die Polster der drei Etagen, der Kern und der Wipfel in beiden Ansichten."),
            "merged": ("Diagonal pads and the stem tuft are part of the tier pads.", "Diagonale Polster und das Stammbüschel gehören zu den Etagenpolstern."),
            "omitted": ("Limb lines, bark lines and inner arcs.", "Astlinien, Rindenlinien und Innenbögen."),
        },
        "500": {
            "silhouette": ("Front view as one layered crown silhouette on three slender stems; plan view as one lobed crown disc on the root collar.", "Vorderansicht als eine geschichtete Kronensilhouette auf drei schlanken Stämmen; Draufsicht als eine gelappte Kronenscheibe über dem Wurzelhals."),
            "retained": ("The stepped outline of the tiers in front and the round lobed outline from above.", "Die gestufte Kontur der Etagen vorn und die runde gelappte Kontur von oben."),
            "merged": ("All pads and the top tuft are one shape; the three stems are one trunk base.", "Alle Polster und das Wipfelbüschel sind eine Form; die drei Stämme sind ein Stammfuß."),
            "omitted": ("Limbs, bark lines and every inner structure.", "Äste, Rindenlinien und jede innere Struktur."),
        },
    }
