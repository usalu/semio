import random
from kit import *

BX = {1: (-1.57, -0.93), 2: (-1.07, -0.43), 3: (-0.57, 0.07), 4: (-0.07, 0.57), 5: (0.43, 1.07), 6: (0.93, 1.57)}
TOP = {1: 1.88, 2: 1.85, 3: 1.88, 4: 1.85, 5: 1.88, 6: 1.85}
TONE50 = {1: 0.5, 2: 0.46, 3: 0.5, 4: 0.46, 5: 0.5, 6: 0.46}
HALF = 0.35


def block_front(scale, n, near):
    w = HALF + (0.0 if near else 0.005 * (1 if n % 2 else -1) + 0.01)
    top = TOP[n]
    pts = [(-w + 0.04, 0.34), (-w, 0.7), (-w - 0.01, 1.3), (-w + 0.03, top - 0.12), (-w + 0.12, top), (w - 0.12, top), (w - 0.03, top - 0.12), (w + 0.01, 1.3), (w, 0.7), (w - 0.04, 0.34)]
    tone = 0.6 if scale > 50 else TONE50[n]
    return poly(f"block-{n}", f"block-{n}", "body", 2.1 + 0.1 * (n if near else 0) - (0.5 if not near else 0), tone, pts, region="leaf-face")


def build(scale):
    front, plan = [], []
    rnd = random.Random(5)
    if scale == 500:
        front.append(rect("plinth", "root-plate", "body", 1.0, 0.15, -0.33, 0.0, 0.66, 0.4, region="stem-bark"))
        front.append(poly("hedge", "root-plate", "body", 2.0, 0.6, [(-0.31, 0.34), (-0.37, 0.8), (-0.37, 1.6), (-0.3, 1.88), (0.3, 1.88), (0.37, 1.6), (0.37, 0.8), (0.31, 0.34)], regions=["leaf-face", "leaf-core"]))
        plan.append(rect("plinth", "root-plate", "body", 1.0, 0.15, -1.5, -0.3, 3.0, 0.6, region="stem-bark"))
        plan.append(rect("hedge", "root-plate", "body", 2.0, 0.6, -1.57, -HALF, 3.14, 2 * HALF, regions=["leaf-face", "leaf-core"], r=0.08))
        return front, plan
    if scale >= 100 or scale == 200:
        if scale != 50:
            front.append(rect("plinth", "root-plate", "body", 1.0, 0.15, -0.33, 0.0, 0.66, 0.4, region="stem-bark"))
            plan.append(rect("plinth", "root-plate", "body", 1.0, 0.15, -1.5, -0.3, 3.0, 0.6, region="stem-bark"))
    if scale == 50:
        front.append(poly("strip", "root-plate", "detail", 2.0, 0.2, [(-0.4, 0.02), (0.4, 0.02)], closed=False))
        plan.append(poly("strip", "root-plate", "detail", 2.0, 0.2, [(-1.55, -0.35), (1.55, -0.35)], closed=False))
    plan.append(rect("core-strip", "root-plate", "body", 1.5, 0.4, -1.5, -0.28, 3.0, 0.56, region="leaf-core"))
    for n in (1, 2, 3, 4, 5, 6):
        near = n == 6
        front.append(block_front(scale, n, near))
    for n in (1, 2, 3, 4, 5):
        pass
    front.append(rect("core-6", "block-6", "body", 3.1, 0.4, -0.2, 0.55, 0.4, 1.0, region="leaf-core", **({"hatch": "shade-lines"} if scale in (100, 50) else {})))
    for n in (1, 2, 3, 4, 5, 6):
        x0, x1 = BX[n]
        tone = 0.6 if scale > 50 else TONE50[n]
        plan.append(rect(f"block-{n}", f"block-{n}", "body", 2.1 + 0.1 * n, tone, x0, -HALF, x1 - x0, 2 * HALF, region="leaf-face", r=0.1))
        if scale <= 100:
            plan.append(rect(f"core-{n}", f"block-{n}", "body", 2.6 + 0.1 * n - 0.05, 0.4, x0 + 0.1, -0.12, x1 - x0 - 0.2, 0.24, region="leaf-core", **({"hatch": "shade-lines"} if scale == 100 else {})) if False else rect(f"core-{n}", f"block-{n}", "body", 2.6 + 0.1 * n - 0.05, 0.4, x0 + 0.1, -0.12, x1 - x0 - 0.2, 0.24, region="leaf-core"))
    if scale <= 100:
        for n in range(1, 7):
            x0, x1 = BX[n]
            for k in range(3):
                y = 0.9 + 0.3 * k
                front.append(poly(f"leaf-{n}-{k + 1}", f"block-{n}" if n == 6 else f"block-{n}", "texture", 9.0, 0.2, [(-0.2 + 0.05 * k, y), (0.0, y + 0.03), (0.2 - 0.05 * k, y)], closed=False)) if n == 6 else None
            for k in range(3):
                xx = x0 + 0.15 + 0.17 * k
                plan.append(poly(f"leaf-{n}-{k + 1}", f"block-{n}", "texture", 9.0, 0.2, [(xx, -0.2 + 0.2 * rnd.random()), (xx + 0.12, 0.0 + 0.2 * rnd.random())], closed=False))
        front.append(poly("stem-6", "stem-6", "detail", 3.0, 0.2, [(0.0, 0.0), (0.0, 0.34)], closed=False)) if scale == 100 else None
    if scale == 50:
        for n in range(1, 7):
            for j, off in enumerate((-0.13, 0.12)):
                front.append(rect(f"stem-{n}-{'ab'[j]}", f"stem-{n}", "body", 3.0 + 0.01 * n, 0.35, off - 0.04 + 0.01 * n, 0.0, 0.08, 0.4, region="stem-bark"))
            front.append(poly(f"stem-{n}-bark", f"stem-{n}", "texture", 3.2 + 0.01 * n, 0.1, [(-0.1 + 0.01 * n, 0.08), (-0.1 + 0.01 * n, 0.36)], closed=False))
        for k in range(4):
            xx = BX[6][0] + 0.12 * k
            plan.append(poly(f"block-6-fine-{k + 1}", "block-6", "texture", 9.1, 0.3, [(xx, -0.3), (xx + 0.1, 0.3)], closed=False))
        for k in range(4):
            front.append(poly(f"block-6-fine-{k + 1}", "block-6", "texture", 9.1, 0.3, [(-0.28 + 0.14 * k, 0.5), (-0.25 + 0.14 * k, 1.7)], closed=False))
        front.append(poly("core-gap", "block-3", "detail", 4.59, 0.3, [(-0.18, 0.6), (-0.18, 1.4), (0.18, 1.4), (0.18, 0.6)], closed=False, region="leaf-core"))
        plan.append(poly("core-gap", "block-3", "detail", 4.59, 0.3, [(-0.5, -0.2), (0.0, -0.2), (0.0, 0.2), (-0.5, 0.2)], closed=True, region="leaf-core") if False else poly("core-gap", "block-3", "detail", 4.59, 0.3, [(-0.5, -0.15), (0.0, -0.15), (0.0, 0.15), (-0.5, 0.15)], closed=False, region="leaf-core"))
    return front, plan


NOTES = {
    500: ("Front: the plinth and the end of the hedge as one tall, slightly tapering section. Plan: the plinth and one clipped band of the hedge top, 0.7 m deep.",
          "Vorderansicht: der Sockel und das Heckenende als ein hoher, leicht verjüngter Querschnitt. Draufsicht: der Sockel und ein beschnittenes Band der Heckenoberseite, 0,7 m tief."),
    200: ("Front: the plinth, the near end block with its core and the five farther blocks peeking out as a ragged outline. Plan: six clipped tops with the dark gaps of the core between them.",
          "Vorderansicht: der Sockel, der nahe Endblock mit Kern und die fünf ferneren Blöcke als unruhiger Umriss. Draufsicht: sechs beschnittene Oberseiten mit den dunklen Kernfugen dazwischen."),
    100: ("Front adds the hatched core, leaf strokes and the stem; plan adds the hatched core of every block and leaf strokes.",
          "Die Vorderansicht ergänzt den schraffierten Kern, Blattstriche und den Stamm; die Draufsicht ergänzt den schraffierten Kern jedes Blocks und Blattstriche."),
    50: ("Without plinth: twelve stems with bark lines in front and the ground strip; the blocks get fine clipping strokes and a core gap seen in front and plan.",
         "Ohne Sockel: zwölf Stämme mit Rindenlinien in der Vorderansicht und der Bodenstreifen; die Blöcke erhalten feine Schnittstriche und eine Kernlücke in Vorder- und Draufsicht."),
}

if __name__ == "__main__":
    views = {sc: build(sc) for sc in (500, 200, 100, 50)}
    install("hornbeam-hedge", views, NOTES)
    extents("hornbeam-hedge")
