import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "stepladder"
doc = load(A)
AL, STEEL, RUB = "alu-mill", "steel-zinc", "rubber-black"
TREADS = [0.238, 0.488, 0.738, 0.988]
TX = [(-0.41, -0.29), (-0.37, -0.25), (-0.33, -0.21), (-0.29, -0.17)]


def outer(y):
    return 0.23 - 0.05 * y


def inner(y, w):
    return outer(y) - w


def rails_front(w, top=0.155, **kw):
    out = []
    for side, name in ((1, "l"), (-1, "r")):
        pts = [(side * 0.23, 0.0), (side * (0.23 - w), 0.0), (side * (top - w), 1.5), (side * top, 1.5)]
        out.append(poly(pts, id=f"front-rail-{name}", bone="front-frame", role="body", z=3, tone=0.5, region="front-rails", material=AL, **kw))
    return out


def rails_rear(w, ids=("rear-rail-l", "rear-rail-r"), top=0.145, **kw):
    out = []
    for side, name in zip((1, -1), ids):
        pts = [(side * 0.2, 0.0), (side * (0.2 - w), 0.0), (side * (top - w), 1.24), (side * top, 1.24)]
        out.append(poly(pts, id=name, bone="rear-frame", role="body", z=2.8, tone=0.5, region="rear-frame", material=AL, **kw))
    return out


def build(scale):
    f, p = [], []
    if scale == 500:
        f += rails_front(0.08, top=0.17)
        for side, name in ((1, "l"), (-1, "r")):
            f[-1 if side < 0 else -2]["id"] = f[-1 if side < 0 else -2]["id"]
        f += rails_rear(0.08, ids=("rear-rail-l", "rear-rail-r"), top=0.16)
        for i, y in enumerate(TREADS):
            f.append(poly([(-inner(y + 0.012, 0.1), y + 0.012), (inner(y + 0.012, 0.1), y + 0.012)], closed=False, id=f"tread-{i}", bone="front-frame", role="body", z=3.2, tone=0.3, filled=False, region="treads", material=AL, pen=0.13))
        f += [
            poly([(-0.165, 1.25), (0.165, 1.25)], closed=False, id="platform", bone="front-frame", role="body", z=3.3, tone=0.3, filled=False, region="platform", material=AL, pen=0.25),
            poly([(0.16, 0.02), (0.235, 0.02)], closed=False, id="foot-front-l", bone="front-frame", role="body", z=2.9, tone=0.3, filled=False, region="feet", material=RUB, pen=0.18),
            poly([(-0.16, 0.02), (-0.235, 0.02)], closed=False, id="foot-front-r", bone="front-frame", role="body", z=2.9, tone=0.3, filled=False, region="feet", material=RUB, pen=0.18),
        ]
        p += [
            capsule(-0.36, 0.185, -0.12, 0.1, 0.05, 0.05, id="front-rail-l", bone="front-frame", role="body", z=3, tone=0.5, region="front-rails", material=AL),
            capsule(-0.36, -0.185, -0.12, -0.1, 0.05, 0.05, id="front-rail-r", bone="front-frame", role="body", z=3, tone=0.5, region="front-rails", material=AL),
            capsule(0.22, 0.17, -0.1, 0.1, 0.05, 0.05, id="rear-rail-l", bone="rear-frame", role="body", z=2.8, tone=0.5, regions=["rear-frame", "spreaders"], material=AL),
            capsule(0.22, -0.17, -0.1, -0.1, 0.05, 0.05, id="rear-rail-r", bone="rear-frame", role="body", z=2.8, tone=0.5, regions=["rear-frame", "spreaders"], material=AL),
            poly([(-0.41, 0.17), (-0.17, 0.12), (-0.17, -0.12), (-0.41, -0.17)], id="treads", bone="front-frame", role="body", z=3.2, tone=0.55, region="treads", material=AL),
            rect(-0.23, -0.165, 0.27, 0.33, id="platform", bone="front-frame", role="body", z=3.3, tone=0.4, region="platform", material=AL),
        ]
    elif scale == 200:
        f += rails_front(0.06)
        f += rails_rear(0.06, ids=("rear-rail-l", "rear-rail-r"))
        for i, y in enumerate(TREADS):
            f.append(poly([(-inner(y + 0.012, 0.06), y + 0.012), (inner(y + 0.012, 0.06), y + 0.012)], closed=False, id=f"tread-{i}", bone="front-frame", role="body", z=3.2, tone=0.3, filled=False, region="treads", material=AL, pen=0.18))
        f += [
            rect(-0.165, 1.205, 0.33, 0.06, id="platform", bone="front-frame", role="body", z=3.3, tone=0.4, region="platform", material=AL),
            rect(0.16, 0.0, 0.075, 0.075, id="foot-front-l", bone="front-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            rect(-0.235, 0.0, 0.075, 0.075, id="foot-front-r", bone="front-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            rect(0.13, 0.55, 0.07, 0.1, id="spreader-l", bone="front-frame", role="body", z=3.1, tone=0.6, regions=["spreaders"], material=STEEL),
            rect(-0.2, 0.55, 0.07, 0.1, id="spreader-r", bone="front-frame", role="body", z=3.1, tone=0.6, regions=["spreaders"], material=STEEL),
        ]
        p += [
            capsule(-0.36, 0.2, -0.12, 0.13, 0.03, 0.03, id="front-rail-l", bone="front-frame", role="body", z=3, tone=0.5, region="front-rails", material=AL),
            capsule(-0.36, -0.2, -0.12, -0.13, 0.03, 0.03, id="front-rail-r", bone="front-frame", role="body", z=3, tone=0.5, region="front-rails", material=AL),
            capsule(0.225, 0.18, -0.1, 0.12, 0.03, 0.03, id="rear-rail-l", bone="rear-frame", role="body", z=2.8, tone=0.5, region="rear-frame", material=AL),
            capsule(0.225, -0.18, -0.1, -0.12, 0.03, 0.03, id="rear-rail-r", bone="rear-frame", role="body", z=2.8, tone=0.5, region="rear-frame", material=AL),
            capsule(-0.26, 0.18, 0.087, 0.18, 0.02, 0.02, id="spreader-l", bone="front-frame", role="body", z=2.7, tone=0.6, region="spreaders", material=STEEL),
            capsule(-0.26, -0.18, 0.087, -0.18, 0.02, 0.02, id="spreader-r", bone="front-frame", role="body", z=2.7, tone=0.6, region="spreaders", material=STEEL),
        ]
        for i, y in enumerate(TREADS):
            x0, x1 = TX[i]
            p.append(rect(x0, -inner(y, 0.06), x1 - x0, 2 * inner(y, 0.06), id=f"tread-{i}", bone="front-frame", role="body", z=3.2 + 0.01 * i, tone=0.55, region="treads", material=AL))
        p += [
            rect(-0.23, -0.165, 0.27, 0.33, id="platform", bone="front-frame", role="body", z=3.3, tone=0.4, region="platform", material=AL),
            rect(-0.41, 0.16, 0.09, 0.075, id="foot-front-l", bone="front-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            rect(-0.41, -0.235, 0.09, 0.075, id="foot-front-r", bone="front-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            rect(0.18, 0.15, 0.09, 0.06, id="foot-rear-l", bone="rear-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            rect(0.18, -0.21, 0.09, 0.06, id="foot-rear-r", bone="rear-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
        ]
    else:
        w = 0.045
        f += rails_front(w)
        f += rails_rear(w)
        f += [
            poly([(-0.17, 0.208), (0.17, 0.208), (0.17, 0.232), (-0.17, 0.232)], id="rear-brace-0", bone="rear-frame", role="body" if scale == 50 else "detail", z=2.85, tone=0.6, region="rear-frame", material=AL),
            poly([(-0.15, 0.838), (0.15, 0.838), (0.15, 0.862), (-0.15, 0.862)], id="rear-brace-1", bone="rear-frame", role="body" if scale == 50 else "detail", z=2.85, tone=0.6, region="rear-frame", material=AL),
        ]
        for i, y in enumerate(TREADS):
            iw = inner(y + 0.015, w) + 0.005
            f.append(rect(-iw, y, 2 * iw, 0.03, id=f"tread-{i}", bone="front-frame", role="body", z=3.2, tone=0.55, region="treads", material=AL))
            if scale == 50:
                for n, k in enumerate((-0.6, -0.2, 0.2, 0.6)):
                    f.append(poly([(k * iw, y + 0.007), (k * iw, y + 0.023)], closed=False, id=f"tread-{i}-rib-{n}", bone="front-frame", role="texture", z=3.6, tone=0.3, filled=False, region="treads", material=AL))
        f += [
            rect(-0.165, 1.225, 0.33, 0.04, rad=0.006 if scale == 50 else None, id="platform", bone="front-frame", role="body", z=3.3, tone=0.4, region="platform", material=AL),
            rect(0.12, 0.53, 0.06, 0.108, id="spreader-l", bone="front-frame", role="body", z=3.1, tone=0.6, region="spreaders", material=STEEL),
            rect(-0.18, 0.53, 0.06, 0.108, id="spreader-r", bone="front-frame", role="body", z=3.1, tone=0.6, region="spreaders", material=STEEL),
            ellipse(0.15, 1.2, 0.025, 0.025, id="hinge-l", bone="rear-frame", role="detail", z=3.6, tone=0.7, region="platform"),
            ellipse(-0.15, 1.2, 0.025, 0.025, id="hinge-r", bone="rear-frame", role="detail", z=3.6, tone=0.7, region="platform"),
            path("M 0.175 0 L 0.235 0 L 0.23 0.05 L 0.18 0.05 Z", id="foot-front-l", bone="front-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            path("M -0.175 0 L -0.235 0 L -0.23 0.05 L -0.18 0.05 Z", id="foot-front-r", bone="front-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
        ]
        if scale == 50:
            f += [
                poly([(0.16, 0.12), (0.18, 1.35)], closed=False, id="rail-groove-l", bone="front-frame", role="texture", z=3.5, tone=0.3, filled=False, region="front-rails", material=AL),
                poly([(-0.16, 0.12), (-0.18, 1.35)], closed=False, id="rail-groove-r", bone="front-frame", role="texture", z=3.5, tone=0.3, filled=False, region="front-rails", material=AL),
                path("M -0.1 1.49 C -0.1 1.5033 0.1 1.5033 0.1 1.49 L 0.1 1.44 L -0.1 1.44 Z", id="top-cap", bone="front-frame", role="body", z=3.3, tone=0.2, region="front-rails", material=AL),
                poly([(0.17, 0.01), (0.23, 0.01)], closed=False, id="foot-tread-l", bone="front-frame", role="texture", z=3.6, tone=0.3, filled=False, region="feet"),
                poly([(-0.17, 0.01), (-0.23, 0.01)], closed=False, id="foot-tread-r", bone="front-frame", role="texture", z=3.6, tone=0.3, filled=False, region="feet"),
            ]
        p += [
            capsule(-0.36, 0.2075, -0.12, 0.1325, 0.0225, 0.0225, id="front-rail-l", bone="front-frame", role="body", z=3, tone=0.5, region="front-rails", material=AL),
            capsule(-0.36, -0.2075, -0.12, -0.1325, 0.0225, 0.0225, id="front-rail-r", bone="front-frame", role="body", z=3, tone=0.5, region="front-rails", material=AL),
            capsule(0.225, 0.1775, -0.1, 0.1225, 0.0225, 0.0225, id="rear-rail-l", bone="rear-frame", role="body", z=2.8, tone=0.5, region="rear-frame", material=AL),
            capsule(0.225, -0.1775, -0.1, -0.1225, 0.0225, 0.0225, id="rear-rail-r", bone="rear-frame", role="body", z=2.8, tone=0.5, region="rear-frame", material=AL),
            capsule(0.16, -0.17, 0.16, 0.17, 0.012, 0.012, id="rear-brace-0", bone="rear-frame", role="detail", z=2.85, tone=0.6, region="rear-frame", material=AL),
            capsule(0.105, -0.14, 0.105, 0.14, 0.012, 0.012, id="rear-brace-1", bone="rear-frame", role="detail", z=2.85, tone=0.6, region="rear-frame", material=AL),
            capsule(-0.26, 0.18, -0.06, 0.18, 0.016, 0.016, id="spreader-front-l", bone="front-frame", role="body", z=2.7, tone=0.6, region="spreaders", material=STEEL),
            capsule(-0.26, -0.18, -0.06, -0.18, 0.016, 0.016, id="spreader-front-r", bone="front-frame", role="body", z=2.7, tone=0.6, region="spreaders", material=STEEL),
            capsule(-0.126, 0.15, 0.087, 0.15, 0.016, 0.016, id="spreader-rear-l", bone="rear-frame", role="body", z=2.7, tone=0.6, region="spreaders", material=STEEL),
            capsule(-0.126, -0.15, 0.087, -0.15, 0.016, 0.016, id="spreader-rear-r", bone="rear-frame", role="body", z=2.7, tone=0.6, region="spreaders", material=STEEL),
        ]
        for i, y in enumerate(TREADS):
            x0, x1 = TX[i]
            iw = inner(y + 0.015, w) + 0.005
            p.append(rect(x0, -iw, x1 - x0, 2 * iw, rad=0.008 if scale == 50 else None, id=f"tread-{i}", bone="front-frame", role="body", z=3.2 + 0.01 * i, tone=0.55, region="treads", material=AL))
            if scale == 50:
                for k in range(3):
                    p.append(poly([(x0 + 0.03 + 0.03 * k, -iw + 0.02), (x0 + 0.03 + 0.03 * k, iw - 0.02)], closed=False, id=f"tread-{i}-rib-{k}", bone="front-frame", role="texture", z=3.25 + 0.01 * i, tone=0.3, filled=False, region="treads", material=AL))
        p += [
            rect(-0.23, -0.165, 0.27, 0.33, rad=0.008 if scale == 50 else None, id="platform", bone="front-frame", role="body", z=3.3, tone=0.4, region="platform", material=AL),
            capsule(-0.13, -0.15, -0.13, 0.15, 0.025, 0.025, id="hinge", bone="rear-frame", role="detail", z=3.6, tone=0.7, region="platform"),
            rect(-0.405, 0.175, 0.09, 0.06, id="foot-front-l", bone="front-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            rect(-0.405, -0.235, 0.09, 0.06, id="foot-front-r", bone="front-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            rect(0.18, 0.15, 0.09, 0.055, id="foot-rear-l", bone="rear-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
            rect(0.18, -0.205, 0.09, 0.055, id="foot-rear-r", bone="rear-frame", role="body", z=2.9, tone=0.9, region="feet", material=RUB),
        ]
    return f, p


T = {
    500: (
        ("Front view: two splayed rails 0.2 mm wide, four tread lines, the platform line, the two rear rails behind them and two foot lines. Plan view: both rail pairs as four capsules converging towards the platform, the treads merged into one trapezoid and the platform as one rectangle; the ladder is rigid, so every pose draws the same.",
         "Vorderansicht: zwei gespreizte Holme von 0,2 mm Breite, vier Stufenlinien, die Plattformlinie, die zwei Hinterholme dahinter und zwei Fußlinien. Draufsicht: beide Holmpaare als vier Kapseln, die zur Plattform zusammenlaufen, die Stufen zu einem Trapez verschmolzen und die Plattform als ein Rechteck; die Leiter ist starr, jede Pose sieht gleich aus."),
        ("Front and plan view: rails, treads, platform; feet in the front view only.",
         "Vorder- und Draufsicht: Holme, Stufen, Plattform; Füße nur in der Vorderansicht."),
        ("Front and plan view: spreaders, hinge and braces are not drawn; the plan omits the feet (0.1 mm).",
         "Vorder- und Draufsicht: Spreizen, Scharnier und Streben sind nicht gezeichnet; die Draufsicht lässt die Füße (0,1 mm) weg."),
    ),
    200: (
        ("Front view: splayed front rails and rear rails behind, four tread lines, the platform slab, the two feet and the two spreader plates. Plan view: four rail capsules, four staggered treads, the platform, two spreader bars and the four feet; the platform covers the highest tread.",
         "Vorderansicht: gespreizte Vorderholme und Hinterholme dahinter, vier Stufenlinien, die Plattformplatte, die zwei Füße und die zwei Spreizplatten. Draufsicht: vier Holmkapseln, vier gestaffelte Stufen, die Plattform, zwei Spreizstäbe und die vier Füße; die Plattform verdeckt die oberste Stufe."),
        ("Front view: spreader plates. Plan view: four feet, two spreader bars.",
         "Vorderansicht: Spreizplatten. Draufsicht: vier Füße, zwei Spreizstäbe."),
        ("Front and plan view: no braces, no hinge, no tread ribs.",
         "Vorder- und Draufsicht: keine Streben, kein Scharnier, keine Stufenriffelung."),
    ),
    100: (
        ("Front view: splayed front rails with four filled treads, the platform, spreader plates, two hinge pins, rubber feet, and the rear rails with two braces seen between the treads. Plan view: front and rear rail capsules, four staggered treads, the platform over the top tread, four spreader bars split at the hinge, two rear braces, the hinge pin and the four feet.",
         "Vorderansicht: gespreizte Vorderholme mit vier gefüllten Stufen, die Plattform, Spreizplatten, zwei Scharnierbolzen, Gummifüße und die Hinterholme mit zwei Streben zwischen den Stufen. Draufsicht: Kapseln der Vorder- und Hinterholme, vier gestaffelte Stufen, die Plattform über der obersten Stufe, vier am Scharnier geteilte Spreizstäbe, zwei Hinterstreben, der Scharnierbolzen und die vier Füße."),
        ("Front view: braces, hinge pins. Plan view: braces, hinge pin, spreader halves.",
         "Vorderansicht: Streben, Scharnierbolzen. Draufsicht: Streben, Scharnierbolzen, Spreizhälften."),
        ("Front and plan view: rail grooves, tread ribs, top cap and foot tread are not drawn.",
         "Vorder- und Draufsicht: Holmrillen, Stufenriffelung, Kopfkappe und Fußprofil sind nicht gezeichnet."),
    ),
    50: (
        ("Front view: splayed rails with grooves, four ribbed treads, the rounded platform, top cap, spreader plates, hinge pins, ribbed rubber feet and, behind the treads, the rear rails with both braces. Plan view: rounded rail capsules, four ribbed treads that step back towards the platform, the rounded platform, front and rear spreader bars, both braces, the hinge pin and four feet.",
         "Vorderansicht: gespreizte Holme mit Rillen, vier geriffelte Stufen, die abgerundete Plattform, Kopfkappe, Spreizplatten, Scharnierbolzen, profilierte Gummifüße und, hinter den Stufen, die Hinterholme mit beiden Streben. Draufsicht: abgerundete Holmkapseln, vier geriffelte Stufen, die zur Plattform zurückspringen, die abgerundete Plattform, vordere und hintere Spreizstäbe, beide Streben, der Scharnierbolzen und vier Füße."),
        ("Front view: grooves, ribs, top cap, foot tread. Plan view: tread ribs.",
         "Vorderansicht: Rillen, Riffelung, Kopfkappe, Fußprofil. Draufsicht: Stufenriffelung."),
        ("Front and plan view: rivets and the label plate are not drawn.",
         "Vorder- und Draufsicht: Nieten und das Typenschild sind nicht gezeichnet."),
    ),
}

for scale in (500, 200, 100, 50):
    f, p = build(scale)
    s, ret, omi = T[scale]
    install(doc, scale, f, p, {"silhouette": s, "retained": ret, "omitted": omi})
finish(doc)
save(A, doc)
print("[DEBUG] ladder written")
