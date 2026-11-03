from crowns import *
from decs import std_dec
from spec_lombardy_poplar import branch_front

F = "{}-mass"


def lobes():
    L = [lobe("core-low", "fol-core-low", "fol-core-low", 0.0, 0.0, 6.72, 2.25, 1.99, "fol-core-low-mass", 1),
         lobe("core-high", "fol-core-high", "fol-core-high", 0.0, 0.0, 10.39, 1.71, 1.99, "fol-core-high-mass", 1),
         lobe("top", "fol-top", "fol-top", 0.09, 0.0, 12.68, 1.38, 1.5, "fol-top-mass", 1)]
    L += ring("r1", 5.1, 2.72, 1.81, 1.29, [0, 180], "fol-w1", "fol-e1", F, 1)
    L += ring("r1l", 5.2, 2.62, 1.81, 1.29, [90, 270], "fol-w1", "fol-e1", F, 1)
    L += ring("r1d", 5.3, 2.2, 1.6, 1.2, [45, 135, 225, 315], "fol-w1", "fol-e1", F, 3)
    L += ring("r2", 7.6, 2.77, 1.65, 1.45, [0, 180], "fol-w2", "fol-e2", F, 1)
    L += ring("r2l", 7.7, 2.62, 1.65, 1.45, [90, 270], "fol-w2", "fol-e2", F, 1)
    L += ring("r2d", 7.9, 2.0, 1.5, 1.3, [45, 135, 225, 315], "fol-w2", "fol-e2", F, 2)
    L += ring("r3", 10.2, 1.9, 1.5, 1.4, [0, 180], "fol-w3", "fol-e3", F, 1)
    L += ring("r3l", 10.3, 1.85, 1.5, 1.4, [90, 270], "fol-w3", "fol-e3", F, 1)
    L += ring("r4", 11.95, 1.01, 1.18, 1.2, [0, 180, 90, 270], "fol-w4", "fol-e4", F, 1, imps=[1, 1, 2, 2])
    L.append(lobe("epi-e", "fol-epi-e", "fol-epi-e", 0.95, 0.0, 1.98, 0.51, 0.54, "fol-epi-e-mass", 1))
    L.append(lobe("epi-w", "fol-epi-w", "fol-epi-w", -0.7, 0.0, 2.52, 0.46, 0.46, "fol-epi-w-mass", 1))
    L.append(lobe("epi-s", "fol-epi-w", "fol-epi-w", 0.1, 0.8, 2.2, 0.42, 0.42, "fol-epi-w-mass", 2))
    for l in L:
        l["att"] = max(l["z"] - 1.8, 3.0)
    return L


def build(b):
    LB = lobes()
    for sc in ("50", "100", "200"):
        emit_lobes(b, sc, LB)
    limbs(b, "50", LB, "limb-bark")
    limbs(b, "100", LB, "limb-bark")
    for sc in ("50", "100", "200", "500"):
        taper(b, sc, "front", "stem", "trunk", 1.0, "stem", [(-0.62, 0.0), (0.62, 0.0), (0.34, 0.5), (0.26, 2.9), (0.2, 6.5), (-0.2, 6.5), (-0.26, 2.9), (-0.34, 0.5)])
        disc(b, sc, "plan", "stem", "trunk", 1.0, 0.0, 0.0, 0.45, 0.45, "stem")
    for sc in ("50", "100", "200"):
        taper(b, sc, "front", "wood-ld2", "ld2", 2.03, "wood-ld2", [(-0.2, 6.1), (0.2, 6.1), (0.12, 9.7), (-0.12, 9.7)])
        taper(b, sc, "front", "wood-ld3", "ld3", 2.04, "wood-ld3", [(-0.12, 9.5), (0.12, 9.5), (0.08, 12.7), (-0.08, 12.7)])
        if sc != "200":
            taper(b, sc, "front", "wood-ld4", "ld4", 2.05, "wood-ld4", [(-0.08, 12.5), (0.08, 12.5), (0.03, 14.0), (-0.03, 14.0)])
        for nm, y1, a0, a1 in (("lw1a", 1.3, 3.2, 4.4), ("le1a", -1.4, 3.4, 4.6), ("lw2a", 1.6, 5.4, 7.0), ("le2a", -1.5, 5.7, 7.3), ("lw3a", 1.4, 7.8, 9.3), ("le3a", -1.3, 8.3, 9.8)):
            branch_front(b, sc, "wood-" + nm, nm, "wood-" + nm, 0.0, a0, y1, a1, 0.13, 0.06, 2.1)
    for sc in ("50", "100"):
        for nm, bone, y0, z0, y1, z1 in (("epi-e", "epi-e", 0.1, 1.0, 0.6, 2.0), ("epi-w", "epi-w", -0.1, 2.0, -0.5, 2.5)):
            branch_front(b, sc, nm, bone, "stem", y0, z0, y1, z1, 0.05, 0.03, 1.1)
    fr = fit(outline(0.0, 0.0, 4.5, 6.0, n=18, k=5, amp=0.06, ph=0.3, bottom=0.8), -4.5, 4.5, 3.4, 14.21)
    b.add("500", "front", "crown", "ld1", 3.0, d=smooth(fr), like="crown")
    pl = fit(outline(0.0, 0.0, 4.4, 4.4, n=16, k=7, amp=0.06, ph=0.9), -4.56, 4.61, -4.5, 4.5)
    b.add("500", "plan", "crown", "ld1", 3.0, d=smooth(pl), like="crown")
    disc(b, "500", "plan", "epicormic", "trunk", 1.0, 0.3, 0.2, 0.5, 0.5, "epicormic")
    taper(b, "500", "front", "epicormic", "trunk", 2.0, "epicormic", [(-0.5, 1.4), (0.6, 1.4), (0.6, 2.6), (-0.5, 2.6)])


def decisions():
    return std_dec(dict(
        front=("the round crown seen from the side as five rings of rounded leaf masses on the straight leader", "die runde Krone von der Seite als fünf Ringe gerundeter Laubmassen am geraden Leittrieb"),
        trunk=("the trunk, the leader sections, the lateral limbs and two epicormic shoots", "dem Stamm, den Leittriebabschnitten, den Seitenästen und zwei Wasserreisern"),
        plan=("the round crown from above as concentric rings of lobed masses round the leader", "die runde Krone von oben als konzentrische Ringe gelappter Massen um den Leittrieb"),
        base=("the trunk disc", "die Stammscheibe"),
        front100=("trunk, leader, limbs and the leaf masses as ringed lobes", "Stamm, Leittrieb, Ästen und den Laubmassen als geringelten Lappen"),
        plan100=("the round lobed crown with the shade core in the middle", "der runden gelappten Krone mit dem Schattenkern in der Mitte"),
        main200=("the two shade cores and one mass per ring position", "den beiden Schattenkernen und einer Masse je Ringposition"),
        trunk500=("the trunk with its epicormic tuft", "dem Stamm mit seinem Wasserreiserbüschel")))
