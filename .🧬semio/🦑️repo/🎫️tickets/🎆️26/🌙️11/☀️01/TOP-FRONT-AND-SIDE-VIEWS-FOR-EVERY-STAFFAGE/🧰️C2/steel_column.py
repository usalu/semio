from lib import *

A = "steel-column"
doc = load(A)
B = "column"
FL = ["column-flange", "column-web"]

# front: seen along the web plane, i.e. onto the flange face: lateral x, height y. plan: x forward (section depth), y lateral (flange width).
def f500():
    return [P("column", B, "body", 1, [(-0.13, 0), (0.13, 0), (0.13, 0.02), (0.1, 0.02), (0.1, 2.98), (0.11, 2.98), (0.11, 3.0), (-0.11, 3.0), (-0.11, 2.98), (-0.1, 2.98), (-0.1, 0.02), (-0.13, 0.02)],
              tone=0.4, regions=["column-flange", "column-web", "base-plate", "cap-plate"], material="steel")]

def p500():
    return [R("base-plate", B, "body", 1, -0.15, -0.13, 0.3, 0.26, tone=0.3, region="base-plate", material="steel"),
            R("cap-plate", B, "body", 2, -0.12, -0.11, 0.24, 0.22, tone=0.4, regions=["cap-plate", "column-flange", "column-web"], material="steel")]

def f200():
    return [R("column-bar", B, "body", 1, -0.1, 0.05, 0.2, 2.9, tone=0.4, regions=FL, material="steel"),
            R("base-plate-slab", B, "body", 2, -0.13, 0, 0.26, 0.05, tone=0.3, region="base-plate", material="steel", pen=0.1),
            R("cap-plate-slab", B, "body", 2, -0.11, 2.95, 0.22, 0.05, tone=0.3, region="cap-plate", material="steel", pen=0.1)]

def p200():
    return [R("base-plate-slab", B, "body", 1, -0.15, -0.13, 0.3, 0.26, tone=0.3, region="base-plate", material="steel", pen=0.1),
            R("cap-plate-slab", B, "body", 2, -0.12, -0.11, 0.24, 0.22, tone=0.3, regions=["cap-plate", "column-flange", "column-web"], material="steel", pen=0.1)]

def f100():
    return [R("column-flange-bar", B, "body", 1, -0.1, 0.035, 0.2, 2.93, tone=0.45, regions=FL, material="steel"),
            R("base-plate-block", B, "body", 2, -0.13, 0, 0.26, 0.035, tone=0.3, region="base-plate", material="steel", pen=0.1),
            R("stiffener-edge", B, "body", 3, -0.01, 0.035, 0.02, 0.165, tone=0.3, region="base-plate", material="steel", pen=0.1),
            R("cap-plate-block", B, "body", 2, -0.11, 2.965, 0.22, 0.035, tone=0.3, region="cap-plate", material="steel", pen=0.1),
            line("splice-joint", B, "detail", 1.5, [(-0.1, 1.5), (0.1, 1.5)], region="column-flange")]

def p100():
    return [R("base-plate-block", B, "body", 1, -0.15, -0.13, 0.3, 0.26, tone=0.3, region="base-plate", material="steel", pen=0.1),
            R("stiffener-a", B, "body", 2, 0.1, -0.01, 0.05, 0.02, tone=0.3, region="base-plate", material="steel", pen=0.1),
            R("stiffener-b", B, "body", 2, -0.15, -0.01, 0.05, 0.02, tone=0.3, region="base-plate", material="steel", pen=0.1),
            R("cap-plate-block", B, "body", 3, -0.12, -0.11, 0.24, 0.22, tone=0.4, regions=["cap-plate", "column-flange", "column-web"], material="steel", pen=0.1)]

def f50():
    s = [R("flange-face", B, "body", 1, -0.1, 0.02, 0.2, 2.96, tone=0.45, region="column-flange", material="steel"),
         R("base-plate", B, "body", 2, -0.13, 0, 0.26, 0.02, tone=0.3, region="base-plate", material="steel"),
         R("stiffener-a", B, "body", 3, -0.008, 0.02, 0.016, 0.09, tone=0.3, region="base-plate", material="steel"),
         R("cap-plate", B, "body", 2, -0.11, 2.98, 0.22, 0.02, tone=0.3, region="cap-plate", material="steel"),
         line("splice-line", B, "detail", 1.5, [(-0.1, 1.5), (0.1, 1.5)], region="column-flange", pen=0.18),
         line("splice-mark", B, "detail", 1.6, [(-0.14, 1.5), (0.14, 1.5)], tone=0.4, region="column-flange")]
    for k, sx in (("a", -1), ("b", 1)):
        s.append(R("anchor-nut-" + k, B, "detail", 3.5, sx * 0.1 - 0.012, 0.02, 0.024, 0.018, tone=0.1, region="base-plate", material="steel"))
        s.append(line("anchor-bolt-" + k, B, "detail", 3.4, [(sx * 0.1, 0.038), (sx * 0.1, 0.075)], tone=0.1, region="base-plate"))
    return s

def p50():
    s = [R("base-plate", B, "body", 1, -0.15, -0.13, 0.3, 0.26, tone=0.3, region="base-plate", material="steel"),
         R("stiffener-a", B, "body", 2, 0.1, -0.008, 0.05, 0.016, tone=0.3, region="base-plate", material="steel"),
         R("stiffener-b", B, "body", 2, -0.15, -0.008, 0.05, 0.016, tone=0.3, region="base-plate", material="steel"),
         R("cap-plate", B, "body", 4, -0.12, -0.11, 0.24, 0.22, tone=0.3, regions=["cap-plate", "column-flange", "column-web"], material="steel")]
    for ix, sx in enumerate((-1, 1)):
        for iy, sy in enumerate((-1, 1)):
            k = "%d%d" % (ix, iy)
            s.append(E("anchor-nut-" + k, B, "detail", 3, sx * 0.125, sy * 0.1, 0.012, tone=0.1, region="base-plate", material="steel"))
            s.append(E("anchor-bolt-" + k, B, "detail", 3.1, sx * 0.125, sy * 0.1, 0.005, tone=0.1, region="base-plate", filled=False))
    return s

EN = {
    500: ("Front and plan: the front is the flange face with the 0.26 m base plate and the 0.22 m head plate as one outline; the plan is the 0.30 x 0.26 m base plate under the 0.24 x 0.22 m head plate, the roof of the column seen from above.",
          "Vorder- und Draufsicht: Die Vorderansicht ist die Flanschfläche mit der 0,26 m breiten Fußplatte und der 0,22 m breiten Kopfplatte in einem Umriss; die Draufsicht zeigt die Fußplatte 0,30 x 0,26 m unter der Kopfplatte 0,24 x 0,22 m, das Dach der Stütze von oben."),
    200: ("Front and plan: the front shows the flange face as one bar between the two plates; the plan is the top view (not the cut sign): head plate over base plate, the profile below the head plate stays hidden.",
          "Vorder- und Draufsicht: Die Vorderansicht zeigt die Flanschfläche als einen Balken zwischen den beiden Platten; die Draufsicht ist die Sicht von oben (nicht das Schnittzeichen): Kopfplatte über der Fußplatte, das Profil unter der Kopfplatte bleibt verdeckt."),
    100: ("Front and plan: the front shows the flange face, the edge of the near gusset (20 mm) and the splice joint; the plan shows the head plate over the base plate with the two gussets sticking out of the head plate on the web axis.",
          "Vorder- und Draufsicht: Die Vorderansicht zeigt die Flanschfläche, die Kante der vorderen Rippe (20 mm) und den Montagestoß; die Draufsicht zeigt die Kopfplatte über der Fußplatte, die beiden Rippen ragen auf der Stegachse unter der Kopfplatte hervor."),
    50: ("Front and plan: the front is the flange face with the near gusset edge-on, the two near anchor nuts and bolts, the splice and the cut-plane marker; the plan is the top view with the head plate (20 mm) over the base plate, two gussets and four anchor nuts with bolt ends; the far nuts are hidden behind the near ones in the front.",
         "Vorder- und Draufsicht: Die Vorderansicht ist die Flanschfläche mit der vorderen Rippe in Kantenansicht, den zwei vorderen Ankermuttern und -bolzen, dem Stoß und der Schnittmarke; die Draufsicht ist die Sicht von oben mit der Kopfplatte (20 mm) über der Fußplatte, zwei Rippen und vier Ankermuttern mit Bolzenenden; die hinteren Muttern sind in der Vorderansicht hinter den vorderen verdeckt."),
}
for sc, (f, p) in {500: (f500, p500), 200: (f200, p200), 100: (f100, p100), 50: (f50, p50)}.items():
    install(A, sc, f(), p(), doc, *EN[sc])
finish(A, doc)
print("ok", A)
