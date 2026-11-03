from lib import *

A = "stair-flight"
doc = load(A)
ST = "stringer"
ANG = math.radians(38.853374)
HW = 0.6


def wp(p):
    return (p[0] * math.cos(ANG) - p[1] * math.sin(ANG), p[0] * math.sin(ANG) + p[1] * math.cos(ANG))


def els(sc):
    return {s["id"]: s for s in doc["asset"]["representations"][str(sc)]["geometry"]["elevation"]}


def quads(sc):
    e = els(sc if sc in (200, 100, 50) else 200)
    out = []
    for i in range(1, 13):
        k = [k for k in e if k.endswith("-%02d" % i) and k.split("-")[0] in ("tread",)] or [k for k in e if k == "tread-%02d" % i or k == "tread-%d" % i]
        s = e[k[0]]
        out.append([wp(p) for p in s["points"]])
    return out


def stringer(sc):
    e = els(sc if sc in (200, 100, 50) else 200)
    s = [v for k, v in e.items() if k.startswith("stringer")][0]
    return [wp(p) for p in s["points"]]


def rng(pts, i):
    v = [p[i] for p in pts]
    return min(v), max(v)


def front(sc):
    qs, sg = quads(sc), stringer(sc)
    y0, y1 = rng(sg, 1)
    s = [R("stringer", ST, "body", 1, -0.08, y0, 0.16, y1 - y0, tone=0.4 if sc == 50 else 0.45, region="stringer-steel", material="steel", hatch="steel" if sc in (100, 50) else None)]
    if sc == 500:
        s = [R("stringer", ST, "body", 1, -0.08, y0, 0.16, y1 - y0, tone=0.4, region="stringer-steel", material="steel")]
    for i, q in enumerate(qs, 1):
        a, b = rng(q, 1)
        s.append(R("tread-%02d" % i, ST, "body", 2 + i * 0.01, -HW, a, 2 * HW, b - a, tone=0.6, region="tread-top", material="timber", hatch="grain" if sc == 50 else None))
    if sc == 50:
        for i, q in enumerate(qs, 1):
            a, b = rng(q, 1)
            s.append(R("gusset-%02d" % i, ST, "body", 1.5, -0.03, a - 0.08, 0.06, 0.08, tone=0.3, region="stringer-steel", material="steel"))
            s.append(E("screw-%02d" % i, ST, "detail", 4, -0.03, (a + b) / 2, 0.009, tone=0.2, region="tread-top", material="steel", filled=False))
            s.append(E("screw-%02d-b" % i, ST, "detail", 4, 0.03, (a + b) / 2, 0.009, tone=0.2, region="tread-top", material="steel", filled=False))
    if sc == 100:
        for i, q in enumerate(qs, 1):
            a, b = rng(q, 1)
            s.append(R("bracket-%02d" % i, ST, "detail", 1.5, -0.03, a - 0.08, 0.06, 0.08, tone=0.3, region="stringer-steel", material="steel", filled=False))
    s.append(P("ground", ST, "ground", 0, [(-0.8, 0), (0.8, 0)], closed=False, filled=False, tone=0.5))
    return s


def plan(sc):
    qs, sg = quads(sc), stringer(sc)
    x0, x1 = rng(sg, 0)
    tx = [rng(q, 0) for q in qs]
    xmin, xmax = min(x0, tx[0][0]), max(x1, tx[-1][1])
    if sc == 500:
        return [R("flight", ST, "body", 1, xmin, -HW, xmax - xmin, 2 * HW, tone=0.6, regions=["tread-top", "stringer-steel"], material="timber"),
                line("walking-line", ST, "body", 2, [(xmin + 0.2, 0), (xmax - 0.2, 0), (xmax - 0.5, 0.25), (xmax - 0.2, 0), (xmax - 0.5, -0.25)], tone=0.2, region="tread-top", pen=0.13)]
    s = [R("stringer", ST, "body", 1, x0, -0.08, x1 - x0, 0.16, tone=0.45, region="stringer-steel", material="steel", hatch="steel" if sc in (100, 50) else None)]
    for i, (a, b) in enumerate(tx, 1):
        s.append(R("tread-%02d" % i, ST, "body", 2 + i * 0.01, a, -HW, b - a, 2 * HW, tone=0.6, region="tread-top", material="timber", hatch=None))
    for i, (a, b) in enumerate(tx, 1):
        s.append(line("tread-edge-%02d" % i, ST, "detail", 6, [(a, -HW), (a, HW)], tone=0.5, region="tread-top"))
    xa, xb = tx[0][0] + 0.15, tx[-1][1] - 0.15
    s.append(line("walking-line", ST, "detail", 5, [(xa, 0), (xb, 0)], tone=0.2, region="tread-top"))
    s.append(line("walking-arrow", ST, "detail", 5, [(xb - 0.25, 0.1), (xb, 0), (xb - 0.25, -0.1)], tone=0.2, region="tread-top"))
    s.append(E("walking-start", ST, "detail", 5, xa, 0, 0.04, tone=0.2, region="tread-top"))
    if sc == 50:
        for i, (a, b) in enumerate(tx, 1):
            q = qs[i - 1]
            y_hi = max(q, key=lambda p: p[1])[0]
            s.append(E("screw-%02d-a" % i, ST, "detail", 4, a + 0.05, 0.0, 0.009, tone=0.2, region="tread-top", material="steel", filled=False))
            s.append(E("screw-%02d-b" % i, ST, "detail", 4, b - 0.05, 0.0, 0.009, tone=0.2, region="tread-top", material="steel", filled=False))
    return s


EN = {
    500: ("Front and plan: the front is seen from the top landing down the flight, the central stringer as a bar with the twelve tread edges as bars across it; the plan is the flight footprint 1.2 m wide with the walking line and arrow.",
          "Vorder- und Draufsicht: Die Vorderansicht blickt vom oberen Podest den Lauf hinab, die Mittelwange als Balken mit den zwölf Stufenkanten als Querbalken; die Draufsicht ist der Laufgrundriss, 1,2 m breit, mit Lauflinie und Pfeil."),
    200: ("Front and plan: the front stacks twelve tread edges (1.2 m wide, at their true heights) in front of the 0.16 m stringer; the plan shows the stringer and twelve overlapping treads with the walking line (start dot, arrow upward, DIN 1356-1). The walker has no shape.",
          "Vorder- und Draufsicht: Die Vorderansicht stapelt zwölf Stufenkanten (1,2 m breit, in wahren Höhen) vor der Wange 0,16 m; die Draufsicht zeigt die Wange und zwölf sich überlappende Stufen mit der Lauflinie (Startpunkt, Pfeil nach oben, DIN 1356-1). Die Person hat keine Form."),
    100: ("Front and plan: the front adds the hatched stringer web and the twelve brackets as outlines; the plan shows the stringer with steel hatch at both ends of the flight, the treads with their nosing overlap and the walking line with start dot and arrow, as in the usual stair plan at 1:100.",
          "Vorder- und Draufsicht: Die Vorderansicht ergänzt den schraffierten Wangensteg und die zwölf Konsolen als Umrisse; die Draufsicht zeigt die Wange mit Stahlschraffur an beiden Laufenden, die Stufen mit ihrer Vorderkantenüberdeckung und die Lauflinie mit Startpunkt und Pfeil wie im üblichen Treppengrundriss 1:100."),
    50: ("Front and plan: the front shows the treads with timber grain, the gussets under them and the screws; the plan shows the twelve treads without hatching so that the tread edges stay readable, two screws per tread, the stringer below and the walking line with dot and arrow at 1:50 stair convention (tread lines, line of travel).",
         "Vorder- und Draufsicht: Die Vorderansicht zeigt die Stufen mit Holzmaserung, die Knotenbleche darunter und die Schrauben; die Draufsicht zeigt die zwölf Stufen ohne Schraffur, damit die Trittkanten lesbar bleiben, zwei Schrauben je Stufe, die Wange darunter und die Lauflinie mit Punkt und Pfeil nach der Treppenkonvention 1:50 (Trittlinien, Lauflinie)."),
}
for sc in (500, 200, 100, 50):
    install(A, sc, front(sc), plan(sc), doc, *EN[sc])
finish(A, doc)
print("ok", A)
