from crowns import *
from decs import std_dec
from spec_lombardy_poplar import branch_front

# (pad, x, y, z, rx, ry, rv, imp, attach z)
PADS = [
    ("pl-w3", -3.0, 0.0, 13.45, 2.65, 1.9, 1.08, 1, 12.0), ("pl-e3", 3.03, 0.0, 14.52, 2.87, 2.0, 1.16, 1, 12.8),
    ("pl-w1", -3.14, 0.0, 17.09, 2.81, 2.0, 1.24, 1, 14.3), ("pl-e1", 3.13, 0.0, 18.15, 3.07, 2.1, 1.21, 1, 15.3),
    ("pl-w2", -1.46, 0.0, 19.17, 2.15, 1.8, 1.04, 1, 16.9), ("pl-e2", 1.73, 0.0, 20.14, 2.39, 1.9, 1.13, 1, 17.9),
    ("pl-top", 0.0, 0.0, 21.06, 2.0, 1.8, 1.1, 1, 19.0),
    ("pl-w3", 0.3, 2.3, 13.0, 2.0, 1.7, 1.0, 1, 12.0), ("pl-e3", -0.2, -2.3, 14.0, 2.0, 1.7, 1.05, 1, 12.8),
    ("pl-w1", 0.5, 2.2, 17.4, 2.1, 1.75, 1.1, 1, 14.3), ("pl-e1", -0.3, -2.2, 18.5, 2.1, 1.75, 1.1, 1, 15.3),
    ("pl-w2", 0.2, 1.5, 19.4, 1.6, 1.5, 0.95, 2, 16.9), ("pl-e2", -0.2, -1.5, 20.3, 1.6, 1.5, 1.0, 2, 17.9),
    ("pl-e1", 2.3, -1.7, 16.0, 2.0, 1.5, 1.0, 3, 14.3), ("pl-w1", -2.3, 1.7, 19.0, 1.9, 1.5, 1.0, 3, 14.3),
    ("pl-w3", -2.0, -1.5, 14.8, 1.8, 1.4, 0.9, 3, 12.0), ("pl-e3", 2.0, 1.5, 12.9, 1.8, 1.4, 0.9, 3, 12.8),
]
LIMB = {"pl-w3": "limb-w3", "pl-e3": "limb-e3", "pl-w1": "limb-w1", "pl-e1": "limb-e1", "pl-w2": "limb-w2", "pl-e2": "limb-e2"}


def lobes():
    L = []
    for i, (bn, x, y, z, rx, ry, rv, imp, att) in enumerate(PADS):
        l = lobe("%s-%d" % (bn[3:], i), bn, bn, x, y, z, rx, rv, bn, imp, rhy=ry)
        l["att"] = att
        L.append(l)
    return L


def build(b):
    LB = lobes()
    for sc in ("50", "100", "200"):
        emit_lobes(b, sc, LB)
    limbs(b, "50", LB, "limb-bark")
    limbs(b, "100", LB, "limb-bark")
    for sc in ("50", "100", "200", "500"):
        taper(b, sc, "front", "bole-lower", "trunk-1", 2.0, "bole-lower", [(-0.9, 0.0), (0.9, 0.0), (0.5, 0.6), (0.4, 7.4), (-0.4, 7.4), (-0.5, 0.6)])
        taper(b, sc, "front", "bole-upper", "trunk-2", 2.0, "bole-upper", [(-0.4, 7.4), (0.4, 7.4), (0.14, 20.4), (-0.14, 20.4)])
        disc(b, sc, "plan", "bole-lower", "trunk-1", 1.0, 0.0, 0.0, 0.7, 0.7, "bole-lower")
    if True:
        for sc in ("50", "100"):
            for nm, y0, z0, y1, z1, bn in (("stub-w", 0.1, 5.6, 0.6, 5.85, "stub-w"), ("stub-e", -0.1, 6.6, -0.55, 6.8, "stub-e")):
                branch_front(b, sc, nm, bn, nm, y0, z0, y1, z1, 0.08, 0.06, 2.1)
    for sc in ("50", "100", "200"):
        for bn, nm in (("limb-w3", "limb-w3"), ("limb-e3", "limb-e3"), ("limb-w1", "limb-w1"), ("limb-e1", "limb-e1"), ("limb-w2", "limb-w2"), ("limb-e2", "limb-e2")):
            sgn = 1 if "w" in nm else -1
            att = {"limb-w3": 11.9, "limb-e3": 12.7, "limb-w1": 14.2, "limb-e1": 15.2, "limb-w2": 16.8, "limb-e2": 17.8}[nm]
            branch_front(b, sc, nm, bn, nm, 0.0, att, sgn * 2.0, att + 1.1, 0.16, 0.07, 1.5)
    fr = fit(outline(0.0, 0.0, 4.0, 5.0, n=16, k=3, amp=0.07, ph=0.5, top=0.95, bottom=0.85), -4.0, 4.0, 11.6, 22.2)
    b.add("500", "front", "crown", "trunk-3", 3.0, d=smooth(fr), like="crown")
    pl = fit(outline(0.0, 0.0, 6.0, 4.0, n=16, k=5, amp=0.06, ph=1.3), -5.95, 6.2, -4.0, 4.0)
    b.add("500", "plan", "crown", "trunk-3", 3.0, d=smooth(pl), like="crown")


def decisions():
    return std_dec(dict(
        front=("the crown seen from the side as flat needle pads on the limbs, the pads ordered by depth", "die Krone von der Seite als flache Nadelpolster auf den Ästen, nach der Tiefe geordnet"),
        trunk=("the straight bole with its two bark zones, the limbs and two dead stubs", "dem geraden Schaft mit seinen zwei Rindenzonen, den Ästen und zwei toten Stummeln"),
        plan=("the crown from above as an oval of overlapping needle pads, wider along the wind axis than across", "die Krone von oben als Oval überlappender Nadelpolster, entlang der Windachse breiter als quer"),
        base=("the bole disc", "die Schaftscheibe"),
        front100=("the bole, the limbs and the needle pads as flat lobes", "dem Schaft, den Ästen und den Nadelpolstern als flachen Lappen"),
        plan100=("the oval crown of lobed pads", "dem ovalen Kronenumriss aus gelappten Polstern"),
        main200=("the seven pads and the lateral pads of the limb whorls", "den sieben Polstern und den seitlichen Polstern der Astquirle"),
        trunk500=("the straight bole", "dem geraden Schaft")))
