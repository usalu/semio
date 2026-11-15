import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-03-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("expansion-valve")
BR = dict(material="brass", role="body")
CU = dict(material="copper", role="body")


def bez(r0, r1, h, n=9):
    out = []
    for i in range(n):
        t = i / (n - 1)
        x = (1 - t) ** 3 * r0 + 3 * (1 - t) ** 2 * t * r0 + 3 * (1 - t) * t ** 2 * r1
        y = 3 * (1 - t) ** 2 * t * 0.018 + 3 * (1 - t) * t ** 2 * h + t ** 3 * h
        out.append([y, x])
    return out


def bulb_profile(r, h, c):
    return [[0, 0], [0, r - c], [c * 0.3, r - c * 0.3], [c, r], [h - c, r], [h - c * 0.3, r - c * 0.3], [h, r - c], [h, 0]]


def dome(r, h, n=8, y0=0.0):
    out = []
    for i in range(n + 1):
        t = i / n
        a = t * math.pi / 2
        out.append([y0 + h * math.sin(a), r * math.cos(a) ** 0.9])
    return out


def head_profile(flange, r, h, boss, boss_h):
    pts = [[0, r], [flange, r]]
    top = h - boss_h
    n = 7
    for i in range(1, n + 1):
        a = i / n * math.pi / 2
        pts.append([flange + (top - flange) * math.sin(a), r * math.cos(a) ** 0.85 if i < n else boss])
    pts.append([h, boss])
    return pts


def s50():
    V.box("body", "body", [0, 0.025, 0], [0.064, 0.05, 0.06], cr=0.003, tone=0.5, **BR)
    V.cyl("inlet", "body", [-0.078, 0.029, 0], [-0.03, 0.029, 0], 0.009, tone=0.4, **CU)
    V.cyl("outlet", "body", [0.03, 0.019, 0], [0.078, 0.019, 0], 0.009, tone=0.4, **CU)
    V.rev("head", "head", [0, 0.05, 0], [0, 1, 0], bez(0.025, 0.014, 0.028), tone=0.55, **BR)
    V.cyl("stem", "stem", [0, 0.078, 0], [0, 0.09, 0], 0.01, tone=0.3, **BR)
    V.rev("bulb", "bulb", [0.108, 0.015, 0.018], [0, 1, 0], bulb_profile(0.01, 0.035, 0.004), tone=0.5, **CU)
    path = [[0.018, 0.072, 0.0], [0.05, 0.088, 0.012], [0.09, 0.082, 0.02], [0.108, 0.05, 0.018]]
    for i in range(3):
        V.cap(f"capillary-{i + 1}", "head", path[i], path[i + 1], 0.0008, tone=0.1, slender=True, material="copper", role="detail")
    V.to("thermal")
    for s, r in [("body", "body"), ("inlet", "inlet"), ("outlet", "outlet"), ("head", "head"), ("stem", "head"), ("bulb", "bulb"), ("capillary-1", "capillary"), ("capillary-2", "capillary"), ("capillary-3", "capillary")]:
        V.ref("th-" + s, s, r)
    V.to("structure")
    V.ref("m-body", "body", "body", None) if False else None
    for s, bone in [("body", "body"), ("head", "head"), ("stem", "stem"), ("bulb", "bulb")]:
        V.lists["structure"].append({"id": "member-" + s, "of": s, "member": bone, "part": "member"})
    V.cyl("joint-head-body", "head", [0, 0.046, 0], [0, 0.054, 0], 0.0235, tone=0.55, member="head", part="joint", **BR)
    V.cyl("joint-stem-head", "stem", [0, 0.074, 0], [0, 0.082, 0], 0.0085, tone=0.3, member="stem", part="joint", **BR)
    V.ell("joint-bulb-body", "bulb", [0.108, 0.03, 0.018], [0.0075, 0.0075, 0.0075], tone=0.5, member="bulb", part="joint", **CU)
    V.box("support-base-contact", "body", [0, 0.00375, 0], [0.05, 0.0075, 0.05], tone=0.3, member="body", part="support", anchor="base-contact", **BR)
    V.ell("mass-body", "body", [0, 0.025, 0], [0.012, 0.012, 0.012], tone=0.2, member="body", part="mass", **BR)
    V.ell("mass-head", "head", [0, 0.065, 0], [0.01, 0.008, 0.01], tone=0.2, member="head", part="mass", **BR)
    V.ell("mass-bulb", "bulb", [0.108, 0.032, 0.018], [0.0075, 0.0075, 0.0075], tone=0.2, member="bulb", part="mass", **CU)
    V.decide("Every solid stands for one drawn part: a forged brass body block with broken edges, two copper connection stubs, a revolved diaphragm head with its adjusting stem, the sensing bulb as an upright rounded cylinder and the capillary as three round bars through the 3D points read from all three drawings; the capillary keeps its real 1.6 mm bore, which is below the 7.5 mm printing limit and is kept because it is the only link of the bulb.",
             "Jeder Körper steht für ein gezeichnetes Teil: ein geschmiedeter Messingblock mit gebrochenen Kanten, zwei Kupferanschlussstutzen, ein gedrehter Membrankopf mit Einstellspindel, der Fühler als stehender Zylinder mit gerundeten Kanten und die Kapillare als drei Rundstäbe durch die 3D-Punkte aus allen drei Zeichnungen; die Kapillare behält ihren echten Durchmesser von 1,6 mm, der unter der Druckgrenze von 7,5 mm liegt, weil sie die einzige Verbindung des Fühlers ist.")
    V.decide("Thermal: every region is a single metal skin, so each thermal solid is a reference to the body solid that carries that skin (body block, stubs, head with stem, bulb, capillary bars); no hidden regions or emitters exist.",
             "Thermik: Jede Region ist eine einzelne Metallhaut, daher verweist jeder Thermikkörper auf den Körper, der diese Haut trägt (Block, Stutzen, Kopf mit Spindel, Fühler, Kapillarstäbe); es gibt weder verdeckte Regionen noch Emitter.")
    V.decide("Structure: the four fixed bones carry the weight as the solid body block, head, stem and bulb; joints are short collars at the head seat, the stem sleeve and the bulb clamp; the base is a 50 mm by 50 mm contact pad inside the underside of the block; point masses sit at the centres of mass of body, head and bulb.",
             "Tragwerk: Die vier festen Knochen tragen das Gewicht als Block, Kopf, Spindel und Fühler; Gelenke sind kurze Bunde am Kopfsitz, an der Spindelhülse und an der Fühlerschelle; der Fuß ist eine Kontaktfläche von 50 mm mal 50 mm in der Blockunterseite; Punktmassen liegen in den Schwerpunkten von Körper, Kopf und Fühler.")
    V.finish(50)


def s100():
    V.box("body", "body", [0, 0.025, 0], [0.064, 0.05, 0.06], cr=0.003, tone=0.5, **BR)
    V.cyl("inlet", "body", [-0.078, 0.029, 0], [-0.03, 0.029, 0], 0.011, tone=0.4, **CU)
    V.cyl("outlet", "body", [0.03, 0.019, 0], [0.078, 0.019, 0], 0.011, tone=0.4, **CU)
    V.rev("head", "head", [0, 0.05, 0], [0, 1, 0], bez(0.025, 0.014, 0.028, 7), tone=0.55, **BR)
    V.rev("bulb", "bulb", [0.108, 0.015, 0.018], [0, 1, 0], bulb_profile(0.012, 0.035, 0.006), tone=0.5, **CU)
    V.to("thermal")
    for s, r in [("body", "body"), ("inlet", "inlet"), ("outlet", "outlet"), ("head", "head"), ("bulb", "bulb")]:
        V.ref("th-" + s, s, r)
    path = [[0.018, 0.072, 0.0], [0.05, 0.088, 0.012], [0.09, 0.082, 0.02], [0.108, 0.05, 0.018]]
    for i in range(3):
        V.cap(f"th-capillary-{i + 1}", "head", path[i], path[i + 1], 0.0008, tone=0.1, slender=True, material="copper", role="detail", region="capillary")
    V.to("structure")
    for s in ["body", "head", "bulb"]:
        V.lists["structure"].append({"id": "member-" + s, "of": s, "member": s, "part": "member"})
    V.cyl("member-stem", "stem", [0, 0.062, 0], [0, 0.077, 0], 0.0075, tone=0.3, member="stem", part="member", **BR)
    V.cyl("joint-head-body", "head", [0, 0.044, 0], [0, 0.06, 0], 0.0235, tone=0.55, member="head", part="joint", **BR)
    V.box("support-base-contact", "body", [0, 0.0075, 0], [0.05, 0.015, 0.05], tone=0.3, member="body", part="support", anchor="base-contact", **BR)
    V.ell("mass-body", "body", [0, 0.025, 0], [0.015, 0.015, 0.015], tone=0.2, member="body", part="mass", **BR)
    V.decide("At 1:100 the valve is five solids: the body block, both stubs (widened to the 22 mm of the plan), the head dome and the bulb; the 12 mm adjusting stem is merged into the dome (the 1:100 drawing has no stem; 78 mm height against 90 mm stays within the 15 mm limit), while the edge softening drops below the 15 mm limit and vanishes; the 1.6 mm capillary is dropped from the body as in the 1:100 drawing.",
             "Bei 1:100 besteht das Ventil aus fünf Körpern: Block, beide Stutzen (auf die 22 mm des Grundrisses verbreitert), Kopfkuppel und Fühler; die 12 mm hohe Spindel ist in der Kuppel aufgegangen (die 1:100-Zeichnung hat keine Spindel; 78 mm Höhe gegen 90 mm bleiben innerhalb der Grenze von 15 mm), die Kantenbrechungen liegen unter der Grenze von 15 mm und entfallen; die 1,6 mm dicke Kapillare entfällt im Körper wie in der 1:100-Zeichnung.")
    V.decide("Thermal: all regions except the capillary reference their body solid; the capillary exists only in the thermal list, as the same three true-size bars as at 1:50, because its region needs a surface although the 1:100 body (like the 1:100 drawing) no longer shows it.",
             "Thermik: Alle Regionen außer der Kapillare verweisen auf ihren Körper; die Kapillare gibt es nur in der Thermikliste, als dieselben drei maßgetreuen Stäbe wie bei 1:50, weil ihre Region eine Oberfläche braucht, obwohl der 1:100-Körper (wie die 1:100-Zeichnung) sie nicht mehr zeigt.")
    V.decide("Structure: body block, head dome, a stem column inside the dome and bulb as members; one head collar as joint; the base contact pad (50 mm by 15 mm by 50 mm) inside the underside; one mass at the block centre which carries 0.38 kg of the 0.45 kg together with the head.",
             "Tragwerk: Block, Kopfkuppel, eine Spindelsäule in der Kuppel und Fühler als Stäbe; ein Kopfbund als Gelenk; die Kontaktfläche (50 mm mal 15 mm mal 50 mm) in der Unterseite; eine Masse im Blockschwerpunkt, die zusammen mit dem Kopf 0,38 kg der 0,45 kg trägt.")
    V.finish(100)


s50(); s100()
V.unavailable(200); V.unavailable(500)
V.write()
