from crowns import *
from decs import std_dec
from spec_lombardy_poplar import branch_front

L0 = [
    ("l-b", -2.59, 0.3, 5.77, 1.44, 1.05, 1), ("r-b", 2.56, -0.3, 5.94, 1.5, 1.16, 1), ("l-a", -3.04, -0.4, 9.04, 1.55, 1.32, 1), ("r-a", 3.1, 0.4, 8.92, 1.5, 1.13, 1),
    ("c-a", -2.33, 0.5, 9.47, 1.48, 1.31, 1), ("l-top", -1.94, 0.3, 10.81, 1.65, 1.32, 1), ("r-top", 1.89, -0.3, 10.92, 1.69, 1.34, 1), ("c-b", 1.67, 0.2, 11.23, 1.37, 1.25, 1), ("c-top", -0.56, 0.0, 11.81, 1.75, 1.22, 1),
]
LAT = [
    ("l-b", 0.3, 2.0, 5.8, 1.4, 1.05, 1), ("r-b", -0.3, -2.0, 5.9, 1.45, 1.1, 1), ("l-a", -0.9, 1.9, 9.0, 1.5, 1.2, 1), ("r-a", 0.9, -1.9, 8.9, 1.45, 1.1, 1),
    ("c-a", -0.5, -1.7, 9.5, 1.4, 1.2, 1), ("c-b", 0.6, 1.6, 11.2, 1.3, 1.2, 1), ("l-top", -0.5, 1.2, 10.9, 1.5, 1.2, 2), ("r-top", 0.5, -1.2, 11.0, 1.5, 1.2, 2),
    ("l-b", -1.6, -1.4, 5.5, 1.2, 0.95, 3), ("r-b", 1.6, 1.4, 5.6, 1.2, 1.0, 3), ("c-top", 0.8, 0.9, 12.0, 1.3, 1.1, 3),
]


def lobes():
    L = []
    for nm, x, y, z, rh, rv, imp in L0:
        L.append(lobe("e-" + nm, "fol-" + nm, "fol-" + nm, x, y, z, rh, rv, "fol-" + nm, imp))
    for i, (nm, x, y, z, rh, rv, imp) in enumerate(LAT):
        L.append(lobe("s%d-%s" % (i, nm), "fol-" + nm, "fol-" + nm, x, y, z, rh, rv, "fol-" + nm, imp))
    for l in L:
        l["att"] = l["z"] - 1.6
    return L


ST = [("stem-l-1", 0.05, 0.3, 0.95, 5.15, 0.17, 0.13), ("stem-l-2", 0.95, 5.15, 1.35, 9.63, 0.13, 0.06), ("stem-c-1", 0.0, 0.3, 0.0, 5.8, 0.15, 0.12), ("stem-c-2", 0.0, 5.8, -0.25, 10.6, 0.12, 0.05), ("stem-r-1", -0.05, 0.3, -0.9, 5.11, 0.17, 0.13), ("stem-r-2", -0.9, 5.11, -1.3, 9.58, 0.13, 0.06)]


def build(b):
    LB = lobes()
    for sc in ("50", "100", "200"):
        emit_lobes(b, sc, LB)
    limbs(b, "50", LB, "branch-bark")
    limbs(b, "100", LB, "branch-bark")
    for sc in ("50", "100", "200"):
        taper(b, sc, "front", "stool", "root-plate", 2.0, "stool", [(-0.5, 0.0), (0.5, 0.0), (0.2, 0.35), (-0.2, 0.35)])
        disc(b, sc, "plan", "stool", "root-plate", 2.0, 0.0, 0.0, 0.5, 0.5, "stool")
        for nm, y0, z0, y1, z1, w0, w1 in ST:
            if sc == "200" and nm.endswith("2"):
                continue
            like = "stem-" + nm[5]
            if sc == "200":
                branch_front(b, sc, like, nm, like, y0, z0, y1 * 1.5, 9.6, w0, w1, 2.0)
            else:
                branch_front(b, sc, nm, nm, like, y0, z0, y1, z1, w0, w1, 2.0)
    for nm, a in (("stem-l-1", 160), ("stem-c-1", 90), ("stem-r-1", -20)):
        disc(b, "50", "plan", nm, nm, 2.1, 0.12 * math.cos(math.radians(a)), 0.12 * math.sin(math.radians(a)), 0.09, 0.09, "stem-" + nm[5])
    taper(b, "500", "front", "stems", "stem-c-1", 1.0, "stems", [(-0.8, 0.0), (0.8, 0.0), (0.2, 5.5), (-0.2, 5.8), (-1.2, 5.5), (-1.0, 3.0)])
    fr = fit(outline(0.0, 0.0, 3.5, 4.0, n=18, k=5, amp=0.06, ph=0.8, bottom=0.8), -3.5, 3.5, 4.5, 13.02)
    b.add("500", "front", "crown", "stem-c-2", 2.0, d=smooth(fr), like="crown")
    pl = fit(outline(0.0, 0.0, 4.0, 3.2, n=16, k=6, amp=0.07, ph=0.2), -4.59, 4.67, -3.5, 3.5)
    b.add("500", "plan", "crown", "stem-c-2", 2.0, d=smooth(pl), like="crown")
    disc(b, "500", "plan", "stems", "stem-c-1", 1.0, 0.0, 0.0, 0.55, 0.55, "stems")


def decisions():
    return std_dec(dict(
        front=("the three-stem crown seen from the side, stems leaning sideways and the leaf masses ordered by depth", "die dreistämmige Krone von der Seite, die Stämme seitlich geneigt und die Laubmassen nach der Tiefe geordnet"),
        trunk=("the three stems and the stool", "den drei Stämmen und dem Stock"),
        plan=("the crown from above as a ring of overlapping lobed masses round the stool", "die Krone von oben als Ring überlappender gelappter Massen um den Stock"),
        base=("the stool and the stem bases", "der Stock und die Stammfüße"),
        front100=("the stems in two segments and the leaf masses as overlapping lobes", "den Stämmen in zwei Abschnitten und den Laubmassen als überlappenden Lappen"),
        plan100=("the lobed crown disc over the stool", "der gelappten Kronenscheibe über dem Stock"),
        main200=("one mass per branch group", "eine Masse je Astgruppe"),
        trunk500=("three slim stems", "drei schlanken Stämmen")))
