from lib import *

A = "wheelchair-ramp"
doc = load(A)
B, HR = "body", "handrail"
POSTS = [(1.5, 0.1), (3.3, 0.25), (5.1, 0.4), (6.6, 0.4), (8.4, 0.55), (10.2, 0.7)]
RAIL = ("handrail-steel", "steel")
RAISE = 0.8288
Y = 0.65


def front(sc):
    s = [P("ground", B, "ground", 0, [(-0.9, 0), (0.9, 0)], closed=False, tone=0.5, filled=False)]
    s.append(R("ramp", B, "body", 10, -0.7, 0, 1.4, 0.6, tone=0.5 if sc == 200 else 0.55, regions=["ramp-surface", "ramp-side"] if sc != 500 else ["ramp-surface", "ramp-side", "kerb"], material="concrete", hatch="concrete-stipple" if sc in (100, 50) else None))
    if sc == 500:
        for k, sy in (("a", 1), ("b", -1)):
            s.append(line("handrail-" + k, HR, "body", 2, [(sy * Y, 0.6), (sy * Y, 1.47)], region="handrail-steel", material="steel", pen=0.13))
        return s
    for k, sy in (("a", 1), ("b", -1)):
        s.append(R("kerb-" + k, B, "body", 2, 0.6 if sy > 0 else -0.7, 0.6, 0.1, 0.1, tone=0.4 if sc == 200 else 0.45, region="kerb", material="concrete"))
    if sc == 200:
        for k, sy in (("a", 1), ("b", -1)):
            s.append(line("handrail-" + k, HR, "detail", 3, [(sy * Y, 0.6), (sy * Y, 1.45)], region="handrail-steel", material="steel", pen=0.18))
        return s
    for k, sy in (("a", 1), ("b", -1)):
        s.append(R("handrail-" + k, HR, "body" if sc == 100 else "detail", 5, sy * Y - 0.0212, RAISE, 0.0424, 1.4712 - RAISE, tone=0.35, region="handrail-steel", material="steel", pen=0.13 if sc == 100 else 0.18))
        if sc == 100:
            s.append(R("posts-" + k, HR, "detail", 3, sy * Y - 0.025, 0.6, 0.05, 0.829, tone=0.3, region="handrail-steel", material="steel"))
    if sc == 100:
        return s
    for k, sy in (("a", 1), ("b", -1)):
        for i, (x, y0) in enumerate(POSTS):
            s.append(R("post-%s-%d" % (k, i), HR, "detail", 3 + i * 0.01, sy * Y - 0.025, y0, 0.05, 0.729, tone=0.35, region="handrail-steel", material="steel"))
            s.append(R("plate-%s-%d" % (k, i), HR, "detail", 4 + i * 0.01, sy * Y - 0.05, y0, 0.1, 0.02, tone=0.3, region="handrail-steel", material="steel"))
        s.append(line("rail-axis-" + k, HR, "texture", 6, [(sy * Y, 0.85), (sy * Y, 1.45)], tone=0.2, region="handrail-steel", pen=0.1))
    s.append(line("landing-edge", B, "detail", 11, [(-0.6, 0.3), (0.6, 0.3)], tone=0.4, region="ramp-side"))
    return s


def arrow(x0, x1, y=0.0, h=0.12):
    return [(x0, y), (x1, y)], [(x1 - h, y + h * 0.6), (x1, y), (x1 - h, y - h * 0.6)]


def plan(sc):
    if sc == 500:
        s = [R("ramp", B, "body", 1, 1.5, -0.7, 10.2, 1.4, tone=0.5, regions=["ramp-surface", "ramp-side", "kerb"], material="concrete")]
        for k, sy in (("a", 1), ("b", -1)):
            s.append(line("handrail-" + k, HR, "body", 2, [(1.2, sy * Y), (10.5, sy * Y)], region="handrail-steel", material="steel", pen=0.13))
        return s
    s = [R("ramp", B, "body", 1, 1.5, -0.7, 10.2, 1.4, tone=0.5 if sc == 200 else 0.55, regions=["ramp-surface", "ramp-side"], material="concrete", hatch="concrete-stipple" if sc in (100, 50) else None)]
    for k, sy in (("a", 1), ("b", -1)):
        s.append(R("kerb-" + k, B, "body", 2, 1.5, 0.6 if sy > 0 else -0.7, 8.7, 0.1, tone=0.4 if sc == 200 else 0.45, region="kerb", material="concrete"))
    for i, xj in enumerate((5.1, 6.6, 10.2)):
        s.append(line("joint-%d" % i, B, "detail", 2.5, [(xj, -0.6), (xj, 0.6)], tone=0.4, region="ramp-side"))
    if sc == 200:
        for k, sy in (("a", 1), ("b", -1)):
            s.append(line("handrail-" + k, HR, "detail", 3, [(1.2, sy * Y), (10.5, sy * Y)], region="handrail-steel", material="steel", pen=0.18))
            for i, (x, _) in enumerate(POSTS):
                s.append(R("post-%s-%d" % (k, i), HR, "detail", 3, x - 0.025, sy * Y - 0.025, 0.05, 0.05, tone=0.3, region="handrail-steel", material="steel"))
        return s
    for k, sy in (("a", 1), ("b", -1)):
        s.append(C("handrail-" + k, HR, "body" if sc == 100 else "detail", 5, 1.2, sy * Y, 10.5, sy * Y, 0.0212, tone=0.35, region="handrail-steel", material="steel", pen=0.13 if sc == 100 else 0.18))
        for i, (x, _) in enumerate(POSTS):
            if sc == 100:
                s.append(R("post-%s-%d" % (k, i), HR, "detail", 4, x - 0.025, sy * Y - 0.025, 0.05, 0.05, tone=0.3, region="handrail-steel", material="steel"))
            else:
                s.append(R("plate-%s-%d" % (k, i), HR, "detail", 3.5, x - 0.06, sy * Y - 0.05, 0.12, 0.1, tone=0.3, region="handrail-steel", material="steel"))
                s.append(R("post-%s-%d" % (k, i), HR, "detail", 4, x - 0.025, sy * Y - 0.025, 0.05, 0.05, tone=0.35, region="handrail-steel", material="steel"))
    for i, (a, b) in enumerate(((1.6, 5.0), (6.7, 10.1))):
        shaft, head = arrow(a, b, 0, 0.18)
        s.append(line("walking-line-%d" % i, B, "detail", 2.6, shaft, tone=0.3, region="ramp-surface"))
        s.append(line("walking-arrow-%d" % i, B, "detail", 2.6, head, tone=0.3, region="ramp-surface"))
    if sc == 50:
        s.append(E("walking-start-0", B, "detail", 2.7, 1.6, 0, 0.05, tone=0.2, region="ramp-surface"))
        s.append(E("walking-start-1", B, "detail", 2.7, 6.7, 0, 0.05, tone=0.2, region="ramp-surface"))
    return s


EN = {
    500: ("Front and plan: the front is the ramp seen from the top landing, the end face as one block 0.6 m high with the two handrails as lines; the plan is the 10.2 x 1.4 m ramp body with both handrails as lines. The wheelchair carrier has no shape and so does not appear in any view.",
          "Vorder- und Draufsicht: Die Vorderansicht ist die Rampe vom oberen Podest, die Stirnfläche als ein Block 0,6 m hoch mit den beiden Handläufen als Linien; die Draufsicht ist der Rampenkörper 10,2 x 1,4 m mit beiden Handläufen als Linien. Der Rollstuhlträger hat keine Form und erscheint daher in keiner Ansicht."),
    200: ("Front and plan: the front shows the ramp block, the two kerbs and per side one handrail line and one post column; the plan shows the ramp body, the kerbs, the joints of the two landings and per side the handrail line with its six posts.",
          "Vorder- und Draufsicht: Die Vorderansicht zeigt den Rampenblock, die beiden Radabweiser und je Seite eine Handlauflinie und eine Pfostenreihe; die Draufsicht zeigt den Rampenkörper, die Radabweiser, die Fugen der beiden Podeste und je Seite die Handlauflinie mit ihren sechs Pfosten."),
    100: ("Front and plan: the front shows the end face of the top landing, the kerbs above it, the handrail tubes (42.4 mm) rising to 1.47 m and the post columns; the plan is the top view with concrete stipple, kerbs, landing joints, tube handrails with square posts and the walking line with arrow on both runs.",
          "Vorder- und Draufsicht: Die Vorderansicht zeigt die Stirnfläche des oberen Podests, die Radabweiser darüber, die Handlaufrohre (42,4 mm) bis 1,47 m und die Pfostenreihen; die Draufsicht ist die Sicht von oben mit Betonpunktraster, Radabweisern, Podestfugen, Rohrhandläufen mit quadratischen Pfosten und der Lauflinie mit Pfeil auf beiden Läufen."),
    50: ("Front and plan: the front stacks the six posts and base plates of each side at their real heights (the nearest post, at the top landing, is the highest) beside the rail tube with its axis line; the plan shows each post on its base plate, the tube handrails, the walking lines with start dots and arrows and the landing joints. The 1:12 slope is read from the arrows and the joints, since the plan shows no height.",
         "Vorder- und Draufsicht: Die Vorderansicht stapelt die sechs Pfosten und Fußplatten jeder Seite in ihren wahren Höhen (der nächste Pfosten am oberen Podest ist der höchste) neben dem Handlaufrohr mit seiner Achslinie; die Draufsicht zeigt jeden Pfosten auf seiner Fußplatte, die Rohrhandläufe, die Lauflinien mit Startpunkten und Pfeilen und die Podestfugen. Das Gefälle 1:12 liest man an den Pfeilen und den Fugen, da der Grundriss keine Höhe zeigt."),
}
for sc in (500, 200, 100, 50):
    install(A, sc, front(sc), plan(sc), doc, *EN[sc])
finish(A, doc)
print("ok", A)
