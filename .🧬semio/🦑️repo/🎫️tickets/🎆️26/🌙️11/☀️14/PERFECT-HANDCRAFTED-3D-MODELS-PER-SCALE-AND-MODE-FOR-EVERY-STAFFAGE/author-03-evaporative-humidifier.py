import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-03-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("evaporative-humidifier")
GS = dict(material="galvanised-steel", role="body")
WA = dict(material="water", role="body")
rect = lambda y0, y1, z0, z1: [(y0, z0), (y1, z0), (y1, z1), (y0, z1)]
prect = lambda x0, x1, z0, z1: [(x0, z0), (x1, z0), (x1, z1), (x0, z1)]


def casing(t, tray_h, roof_t):
    V.box("sump-tray", "casing", [0, tray_h / 2, 0], [0.48, tray_h, 0.38], tone=0.3, **GS)
    V.box("side-panel-front", "casing", [0, 0.3, 0.2 - t / 2], [0.5, 0.6, t], tone=0.3, **GS)
    V.box("side-panel-rear", "casing", [0, 0.3, -0.2 + t / 2], [0.5, 0.6, t], tone=0.3, **GS)
    V.ext("roof", "casing", "plan", prect(-0.25, 0.25, -0.2, 0.2), 0.6 - roof_t, 0.6, holes=[prect(-0.2, 0.2, -0.18, 0.18)], tone=0.3, **GS)
    for side, x0, x1 in [("left", -0.25, -0.25 + t), ("right", 0.25 - t, 0.25)]:
        V.ext(f"end-wall-{side}", "casing", "front", rect(tray_h, 0.6 - roof_t, -0.2 + t, 0.2 - t), x0, x1, holes=[rect(0.09, 0.47, -0.17, 0.17)], tone=0.3, **GS)


def s50():
    casing(0.01, 0.02, 0.02)
    V.box("water", "water", [0, 0.045, 0], [0.44, 0.05, 0.38], cr=0.003, tone=0.45, **WA)
    V.box("pad", "pad", [0, 0.28, 0], [0.4, 0.4, 0.36], tone=0.7, material="cellulose", role="body")
    V.box("trough-floor", "trough", [0, 0.48375, 0], [0.42, 0.0075, 0.38], tone=0.4, material="pp-plastic", role="body")
    V.ext("trough-walls", "trough", "plan", prect(-0.21, 0.21, -0.19, 0.19), 0.4875, 0.53, holes=[prect(-0.2, 0.2, -0.18, 0.18)], tone=0.4, material="pp-plastic", role="body")
    V.box("film", "pad", [0, 0.4915, 0], [0.38, 0.008, 0.34], tone=0.85, **WA)
    for k, z in enumerate([-0.1, 0.0, 0.1]):
        V.ell(f"drop-row-{k + 1}", "drip", [-0.1, 0.5, z], [0.01, 0.013, 0.01], tone=0.85, rep=dict(count=3, step=[0.1, 0, 0]), **WA)
    V.to("thermal")
    for s in ["sump-tray", "side-panel-front", "side-panel-rear", "roof", "end-wall-left", "end-wall-right"]:
        V.ref("th-" + s, s, "casing")
    V.ref("th-pad", "pad", "pad")
    V.ref("th-film", "film", "pad")
    for k in (1, 2, 3):
        V.ref(f"th-drop-row-{k}", f"drop-row-{k}", "pad")
    V.ref("th-trough-floor", "trough-floor", "trough")
    V.ref("th-trough-walls", "trough-walls", "trough")
    V.ref("th-water", "water", "sump")
    V.cyl("th-outlet-air", "casing", [0.2425, 0.3, 0], [0.25, 0.3, 0], 0.15, tone=0.85, region="pad", tags=["outlet-air"], material="cellulose", role="body")
    structure(0.0075, 0.0075, 0.02, 50)
    V.decide("Every solid stands for one drawn part: the galvanised casing is a shell of sump tray, two 10 mm side panels, a roof ring and two end walls with a window onto the pad (the silhouettes still fill the drawn rectangles); the pad is one cellulose block 400 by 400 by 360 mm standing 10 mm above the sump water; the trough is a 7.5 mm floor with a 42 mm wall ring holding an 8 mm water film; the nine drops are ellipsoids written as three rows with repeat on the prismatic drip bone, so they fall through the pad together.",
             "Jeder Körper steht für ein gezeichnetes Teil: Das verzinkte Gehäuse ist eine Hülle aus Wanne, zwei 10 mm dicken Seitenwänden, einem Dachrahmen und zwei Stirnwänden mit Fenster zur Matte (die Silhouetten füllen weiter die gezeichneten Rechtecke); die Matte ist ein Zelluloseblock 400 mal 400 mal 360 mm, 10 mm über dem Sumpfwasser; die Rinne ist ein 7,5 mm dicker Boden mit einem 42 mm hohen Wandring und einem 8 mm Wasserfilm; die neun Tropfen sind Ellipsoide in drei Reihen mit repeat auf dem Schubknochen drip und fallen gemeinsam durch die Matte.")
    V.decide("Thermal: every region is one surface; casing panels, pad block, film, drops, trough and sump water reference their body solids, and the air outlet is a 300 mm disc of 7.5 mm in the end window that carries the emitter outlet-air on the pad region. The authored areas of casing (0.9 m2) and trough (0.1 m2) were corrected to the surfaces of the solids.",
             "Thermik: Jede Region ist eine Oberfläche; Gehäusewände, Mattenblock, Film, Tropfen, Rinne und Sumpfwasser verweisen auf ihre Körper, und der Luftaustritt ist eine 300-mm-Scheibe von 7,5 mm im Stirnfenster, die den Emitter outlet-air an der Mattenregion trägt. Die angegebenen Flächen von Gehäuse (0,9 m2) und Rinne (0,1 m2) wurden auf die Flächen der Körper korrigiert.")
    V.decide("Structure: the casing panels, the pad block, the trough and the sump water are the members (they reference their body solids), two rails carry the pad above the water, two brackets hang the trough from the end walls, a column joint sits at the drip bone and the water bears on the tray; the base contact is a 0.5 by 0.4 m pad inside the tray, masses sit at the casing, pad and water centres (at 1:100 pad and casing carry one merged pad mass less, at 1:200 the water mass is merged into the casing mass).",
             "Tragwerk: Gehäusewände, Mattenblock, Rinne und Sumpfwasser sind die Stäbe (sie verweisen auf ihre Körper), zwei Leisten tragen die Matte über dem Wasser, zwei Konsolen hängen die Rinne an die Stirnwände, ein Gelenk sitzt am Tropfenknochen und das Wasser liegt auf der Wanne; der Fußkontakt ist eine Fläche von 0,5 mal 0,4 m in der Wanne, Massen sitzen in den Mittelpunkten von Gehäuse, Matte und Wasser (bei 1:100 entfällt die Mattenmasse, bei 1:200 geht auch die Wassermasse in der Gehäusemasse auf).")
    V.finish(50)


def structure(f, sup_h, tray, scale):
    V.to("structure")
    for s in ["sump-tray", "side-panel-front", "side-panel-rear", "roof", "end-wall-left", "end-wall-right"]:
        V.lists["structure"].append({"id": "member-" + s, "of": s, "member": "casing", "part": "member"})
    V.lists["structure"].append({"id": "member-pad", "of": "pad", "member": "pad", "part": "member"})
    V.lists["structure"].append({"id": "member-trough-floor", "of": "trough-floor", "member": "trough", "part": "member"})
    V.lists["structure"].append({"id": "member-trough-walls", "of": "trough-walls", "member": "trough", "part": "member"})
    V.lists["structure"].append({"id": "member-water", "of": "water", "member": "water", "part": "member"})
    k = max(f, 0.01)
    V.box("joint-pad-rails", "pad", [0, 0.07, -0.17], [0.4, k, k], tone=0.5, member="pad", part="joint", rep=dict(count=2, step=[0, 0, 0.34]), material="galvanised-steel", role="body")
    V.box("joint-trough-brackets", "trough", [-0.215, 0.47, 0], [k, 0.02 if scale == 50 else 0.03, 0.2], tone=0.5, member="trough", part="joint", rep=dict(count=2, step=[0.43, 0, 0]), material="galvanised-steel", role="body")
    if scale < 200:
        V.ell("joint-drip", "drip", [0, 0.5, 0], [0.012, 0.012, 0.012], tone=0.5, member="water", part="joint", material="water", role="body") if False else None
    V.box("joint-water-tray", "water", [0, 0.02, 0], [0.4, k, 0.34], tone=0.5, member="water", part="joint", material="galvanised-steel", role="body")
    V.box("support-base-contact", "casing", [0, sup_h / 2, 0], [0.5, sup_h, 0.4], tone=0.3, member="casing", part="support", anchor="base-contact", **GS)
    V.ell("mass-casing", "casing", [0, 0.3, 0], [0.1, 0.12, 0.08], tone=0.2, member="casing", part="mass", **GS)
    if scale < 100:
        V.ell("mass-pad", "pad", [0, 0.29, 0], [0.1, 0.1, 0.09], tone=0.2, member="pad", part="mass", material="cellulose", role="body")
    if scale < 200:
        V.ell("mass-water", "water", [0, 0.05, 0], [0.12, 0.025 if scale == 50 else 0.0225, 0.09], tone=0.2, member="water", part="mass", **WA)


def s100():
    casing(0.015, 0.02, 0.02)
    V.box("water", "water", [0, 0.065, 0], [0.44, 0.05, 0.38], cr=0.005, tone=0.45, **WA) if False else V.box("water", "water", [0, 0.045, 0], [0.44, 0.05, 0.38], cr=0.005, tone=0.45, **WA)
    V.box("pad", "pad", [0, 0.28, 0], [0.4, 0.4, 0.36], tone=0.7, material="cellulose", role="body")
    V.box("film", "pad", [0, 0.503, 0], [0.38, 0.016, 0.34], tone=0.85, **WA)
    V.box("trough-floor", "trough", [0, 0.4875, 0], [0.42, 0.015, 0.38], tone=0.4, material="pp-plastic", role="body")
    V.ext("trough-walls", "trough", "plan", prect(-0.21, 0.21, -0.19, 0.19), 0.495, 0.53, holes=[prect(-0.195, 0.195, -0.175, 0.175)], tone=0.4, material="pp-plastic", role="body")
    V.ell("drop-front", "drip", [-0.1, 0.5, -0.1], [0.014, 0.018, 0.014], tone=0.85, rep=dict(count=2, step=[0.2, 0, 0]), **WA)
    V.ell("drop-rear", "drip", [-0.1, 0.5, 0.1], [0.014, 0.018, 0.014], tone=0.85, rep=dict(count=2, step=[0.2, 0, 0]), **WA)
    V.ell("drop-centre", "drip", [0, 0.5, 0], [0.014, 0.018, 0.014], tone=0.85, **WA)
    V.to("thermal")
    for s in ["sump-tray", "side-panel-front", "side-panel-rear", "roof", "end-wall-left", "end-wall-right"]:
        V.ref("th-" + s, s, "casing")
    V.ref("th-pad", "pad", "pad")
    V.ref("th-film", "film", "pad")
    for s in ["drop-front", "drop-rear", "drop-centre"]:
        V.ref("th-" + s, s, "pad")
    V.ref("th-trough-floor", "trough-floor", "trough")
    V.ref("th-trough-walls", "trough-walls", "trough")
    V.ref("th-water", "water", "sump")
    V.cyl("th-outlet-air", "casing", [0.235, 0.3, 0], [0.25, 0.3, 0], 0.15, tone=0.85, region="pad", tags=["outlet-air"], material="cellulose", role="body")
    structure(0.015, 0.015, 0.02, 100)
    V.decide("At 1:100 the shell keeps the same parts with 15 mm walls and a 20 mm roof ring, the trough floor is 15 mm, the water film thickens to 16 mm, and the nine drops become five (a quincunx of two pairs and one centre drop, as drawn in plan) of 28 mm by 36 mm.",
             "Bei 1:100 behält die Hülle dieselben Teile mit 15 mm Wänden und einem 20 mm Dachrahmen, der Rinnenboden ist 15 mm dick, der Wasserfilm wird 16 mm dick, und aus den neun Tropfen werden fünf (zwei Paare und ein mittlerer Tropfen, wie im Grundriss gezeichnet) von 28 mm mal 36 mm.")
    V.decide("Thermal: references to the body solids as at 1:50 with the film thickened to 16 mm; the outlet disc is 15 mm thick.",
             "Thermik: Verweise auf die Körper wie bei 1:50 mit dem auf 16 mm verdickten Film; die Austrittsscheibe ist 15 mm dick.")
    V.decide("Structure: the same members and joints as at 1:50 with 15 mm rails and brackets; the contact pad is 15 mm deep.",
             "Tragwerk: dieselben Stäbe und Gelenke wie bei 1:50 mit 15-mm-Leisten und -Konsolen; die Kontaktfläche ist 15 mm tief.")
    V.finish(100)


def s200():
    t = 0.03
    V.box("sump-tray", "casing", [0, 0.02, 0], [0.48, 0.04, 0.38], tone=0.3, **GS)
    V.box("side-panel-front", "casing", [0, 0.3, 0.2 - t / 2], [0.5, 0.6, t], tone=0.3, **GS)
    V.box("side-panel-rear", "casing", [0, 0.3, -0.2 + t / 2], [0.5, 0.6, t], tone=0.3, **GS)
    V.ext("roof", "casing", "plan", prect(-0.25, 0.25, -0.2, 0.2), 0.57, 0.6, holes=[prect(-0.2, 0.2, -0.17, 0.17)], tone=0.3, **GS)
    for side, x0, x1 in [("left", -0.25, -0.22), ("right", 0.22, 0.25)]:
        V.ext(f"end-wall-{side}", "casing", "front", rect(0.04, 0.57, -0.17, 0.17), x0, x1, holes=[rect(0.1, 0.47, -0.14, 0.14)], tone=0.3, **GS)
    V.box("water", "water", [0, 0.065, 0], [0.44, 0.05, 0.38], tone=0.45, **WA)
    V.box("pad", "pad", [0, 0.285, 0], [0.4, 0.37, 0.36], tone=0.7, material="cellulose", role="body")
    V.box("trough-floor", "trough", [0, 0.495, 0], [0.42, 0.03, 0.38], tone=0.4, material="pp-plastic", role="body")
    V.ext("trough-walls", "trough", "plan", prect(-0.21, 0.21, -0.19, 0.19), 0.51, 0.54, holes=[prect(-0.18, 0.18, -0.16, 0.16)], tone=0.4, material="pp-plastic", role="body")
    V.to("thermal")
    for s in ["sump-tray", "side-panel-front", "side-panel-rear", "roof", "end-wall-left", "end-wall-right"]:
        V.ref("th-" + s, s, "casing")
    V.ref("th-pad", "pad", "pad")
    V.ref("th-trough-floor", "trough-floor", "trough")
    V.ref("th-trough-walls", "trough-walls", "trough")
    V.ref("th-water", "water", "sump")
    V.cyl("th-outlet-air", "casing", [0.22, 0.3, 0], [0.25, 0.3, 0], 0.15, tone=0.85, region="pad", tags=["outlet-air"], material="cellulose", role="body")
    structure(0.03, 0.03, 0.04, 200)
    V.decide("At 1:200 the casing shell has 30 mm walls, a 40 mm tray and a 30 mm roof ring, the pad is a single 370 mm block, the trough a 30 mm floor with a 30 mm wall ring and the sump water one slab; drops, film and the 5 mm window details are gone, the drip bone has no drawn shape.",
             "Bei 1:200 hat die Gehäusehülle 30 mm Wände, eine 40 mm Wanne und einen 30 mm Dachrahmen, die Matte ist ein einziger 370-mm-Block, die Rinne ein 30 mm Boden mit einem 30 mm Wandring und das Sumpfwasser eine Platte; Tropfen, Film und die kleinen Fensterdetails entfallen, der Tropfenknochen hat keine gezeichnete Form.")
    V.decide("Thermal: references as before, the outlet disc is 30 mm thick; the pad region keeps its surface from the pad block and the outlet disc only.",
             "Thermik: Verweise wie zuvor, die Austrittsscheibe ist 30 mm dick; die Mattenregion behält ihre Fläche nur aus Mattenblock und Austrittsscheibe.")
    V.decide("Structure: the shell, pad, trough and water members and the rails, brackets and contact pad of 30 mm.",
             "Tragwerk: Hülle, Matte, Rinne und Wasser als Stäbe sowie die Leisten, Konsolen und die 30 mm Kontaktfläche.")
    V.finish(200)


s50(); s100(); s200()
V.unavailable(500)
V.write()
