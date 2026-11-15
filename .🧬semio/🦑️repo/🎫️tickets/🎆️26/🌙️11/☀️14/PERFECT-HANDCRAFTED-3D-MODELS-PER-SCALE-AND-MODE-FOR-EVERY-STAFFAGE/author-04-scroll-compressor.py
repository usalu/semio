import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-04-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("scroll-compressor")
SS = dict(material="steel-shell", role="body")
CU = dict(material="copper", role="body")
CA = dict(material="cast-aluminium")
PA = dict(material="pa-plastic", role="body")
spiral = [[0.006, 0], [0.0067, 0.00201], [0.00668, 0.00441], [0.00577, 0.00691], [0.00393, 0.0092], [0.00123, 0.01093], [-0.00214, 0.01181], [-0.0059, 0.01158], [-0.00967, 0.01012], [-0.01305, 0.0074], [-0.0156, 0.00356], [-0.01696, -0.00114], [-0.01685, -0.00632], [-0.01512, -0.01151], [-0.01176, -0.01618], [-0.00694, -0.01982], [-0.00099, -0.02198], [0.00562, -0.0223], [0.01231, -0.0206], [0.01845, -0.01687], [0.02343, -0.01128], [0.02667, -0.00422], [0.02775, 0.00376], [0.0264, 0.01199], [0.02259, 0.01974], [0.01649, 0.02625], [0.00851, 0.03085], [-0.00074, 0.03299], [-0.01051, 0.03234]]


def bez(n):
    out = []
    for i in range(n + 1):
        t = i / n
        r = (1 - t) ** 3 * 0.1 + 3 * (1 - t) ** 2 * t * 0.1 + 3 * (1 - t) * t ** 2 * 0.05
        y = 3 * (1 - t) ** 2 * t * 0.042 + 3 * (1 - t) * t ** 2 * 0.06 + t ** 3 * 0.06
        out.append((y, r))
    out[-1] = (out[-1][0], 0.0)
    return out


def hollow(pts, wall):
    outer = [[y, r] for y, r in pts]
    inner = []
    n = len(pts)
    for i, (y, r) in enumerate(pts):
        a = pts[max(i - 1, 0)]
        b = pts[min(i + 1, n - 1)]
        ty, tr = b[0] - a[0], b[1] - a[1]
        l = math.hypot(ty, tr) or 1
        ny, nr = tr / l, -ty / l
        iy, ir = y - wall * ny * (-1), r - wall * nr * (-1)
        inner.append([y + wall * (-nr) * 0 - wall * 0, max(r - wall, 0)])
    inner = [[max(y - wall * (1 if i == n - 1 else 0), 0), rr] for i, (y, rr) in enumerate(inner)]
    return outer + inner[::-1] + [outer[0]]


def plate(half, cr, n=3):
    pts = []
    for (cx, cz, a0) in [(half - cr, half - cr, 0), (-half + cr, half - cr, 90), (-half + cr, -half + cr, 180), (half - cr, -half + cr, 270)]:
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n)
            pts.append([cx + cr * math.cos(a), cz + cr * math.sin(a)])
    return pts


def ring(r, n):
    return [[r * math.cos(2 * math.pi * i / n), r * math.sin(2 * math.pi * i / n)] for i in range(n)]


def volute(pts, y0, z, step):
    return [([p[0], y0 + p[1], z], 0.0) for p in pts[::step]]


S50 = lambda: V.at(50, "body")
S = S50()
S.ext("feet", "shell", "plan", plate(0.14, 0.008), 0, 0.03, holes=[ring(0.096, 48)], tone=0.4, **SS)
S.rev("shell", "shell", [0, 0, 0], [0, 1, 0], [[0, 0], [0, 0.1], [0.334, 0.1], [0.34, 0.094], [0.34, 0]], tone=0.5, **SS)
S.rev("dome", "dome", [0, 0.34, 0], [0, 1, 0], [[y, r] for y, r in bez(8)] + [[0, 0]], tone=0.7, **SS)
S.cyl("discharge-stub", "dome", [0, 0.398, 0], [0, 0.43, 0], 0.014, tone=0.7, **CU)
S.cyl("discharge-flange", "dome", [0, 0.422, 0], [0, 0.43, 0], 0.021, tone=0.7, **CU)
S.cyl("suction-stub", "shell", [-0.15, 0.213, 0], [-0.098, 0.213, 0], 0.013, tone=0.2, **CU)
S.cyl("suction-collar", "shell", [-0.118, 0.213, 0], [-0.098, 0.213, 0], 0.019, tone=0.2, **CU)
S.cyl("suction-crimp", "shell", [-0.15, 0.213, 0], [-0.141, 0.213, 0], 0.017, tone=0.2, **CU)
S.box("terminal", "terminal", [0.118, 0.28, 0], [0.036, 0.06, 0.055], cr=0.004, tone=0.3, **PA)
S.box("terminal-lid", "terminal", [0.12, 0.3, 0], [0.04, 0.01, 0.05], cr=0.002, tone=0.25, **PA)
S.cyl("terminal-gland", "terminal", [0.136, 0.275, 0], [0.15, 0.275, 0], 0.01, tone=0.2, **PA)
S.rev("sight-ring", "shell", [0, 0.18, -0.1], [0, 0, -1], [[0, 0.046], [0, 0.052], [0.008, 0.052], [0.008, 0.046], [0, 0.046]], tone=0.3, **SS)
S.rev("scroll-plate", "rotor", [0, 0.18, -0.1], [0, 0, -1], [[0, 0], [0, 0.045], [0.008, 0.045], [0.008, 0]], tone=0.8, role="body", **CA)
sp = [([p[0], 0.18 + p[1], -0.108], 0.004, 0.004) for p in spiral[::2]]
S.loft("scroll-spiral", "rotor", sp, caps=(True, True), tone=0.1, role="detail", **CA)
S.rev("scroll-hub", "rotor", [0, 0.18, -0.108], [0, 0, -1], [[0, 0], [0, 0.01], [0.008, 0.01], [0.008, 0]], tone=0.2, role="detail", **CA)
S.rev("sight-glass", "shell", [0, 0.18, -0.118], [0, 0, -1], [[0, 0], [0, 0.045], [0.002, 0.045], [0.002, 0]], tone=0.9, material="steel-shell", role="glazing")

T = V.at(50, "thermal")
for a, b, reg in [("th-shell", "shell", "shell"), ("th-dome", "dome", "dome"), ("th-discharge", "discharge-stub", "discharge"), ("th-discharge-flange", "discharge-flange", "discharge"), ("th-suction", "suction-stub", "suction"), ("th-suction-collar", "suction-collar", "suction"), ("th-suction-crimp", "suction-crimp", "suction"), ("th-terminal", "terminal", "terminal"), ("th-terminal-lid", "terminal-lid", "terminal"), ("th-terminal-gland", "terminal-gland", "terminal"), ("th-feet", "feet", "feet"), ("th-scroll-plate", "scroll-plate", "scroll"), ("th-scroll-spiral", "scroll-spiral", "scroll")]:
    T.ref(a, b, region=reg)
T.rev("th-dome-plume-outlet", "dome", [0, 0.4, 0], [0, 1, 0], [[0, 0.054], [0, 0.06], [0.008, 0.06], [0.008, 0.054], [0, 0.054]], tone=0.9, region="dome", tags=["dome-plume"], **SS)

R = V.at(50, "structure")
R.rev("member-shell", "shell", [0, 0, 0], [0, 1, 0], [[0, 0.097], [0, 0.1], [0.34, 0.1], [0.34, 0.097], [0, 0.097]], tone=0.5, member="shell", part="member", **SS)
R.rev("member-dome", "dome", [0, 0.34, 0], [0, 1, 0], hollow(bez(8), 0.003), tone=0.7, member="dome", part="member", **SS)
R.ref("member-terminal", "terminal", member="terminal", part="member")
R.ref("member-rotor", "scroll-plate", member="rotor", part="member")
R.rev("joint-dome-weld", "dome", [0, 0.336, 0], [0, 1, 0], [[0, 0.094], [0, 0.103], [0.008, 0.103], [0.008, 0.094], [0, 0.094]], tone=0.6, member="dome", part="joint", **SS)
R.box("joint-terminal-pad", "terminal", [0.1, 0.28, 0], [0.01, 0.045, 0.05], tone=0.3, member="terminal", part="joint", **SS)
R.rev("joint-rotor-bearing", "rotor", [0, 0.18, -0.092], [0, 0, -1], [[0, 0], [0, 0.015], [0.016, 0.015], [0.016, 0]], tone=0.3, member="rotor", part="joint", **SS)
R.ell("mass-shell", "shell", [0, 0.17, 0], [0.07, 0.1, 0.07], tone=0.6, member="shell", part="mass", **SS)
R.ell("mass-dome", "dome", [0, 0.37, 0], [0.05, 0.025, 0.05], tone=0.6, member="dome", part="mass", **SS)
R.box("base-contact-pad", "shell", [0, 0.004, 0], [0.26, 0.008, 0.26], tone=0.4, member="shell", part="support", anchor="base-contact", **SS)

V.decide(50, "Detailed model: the hermetic shell is a revolved steel can with a 6 mm shoulder under the dome that stands in the hole of the base, a rounded 280 mm plate extruded with a 192 mm hole so that its top is a true annulus around the shell (a plain slab would be mistaken for buried under the shell); the dome is a closed revolve of the elevation Bezier. Discharge stub with flange and suction stub with collar and crimp are copper cylinders, the terminal box has lid and a 10 mm cable gland that completes the 300 mm width. The sight window is a steel ring, a 4 mm sight glass (glazing), an 8 mm orbiting-scroll plate on the rotor bone, its hub and the spiral wrap swept as one 8 mm tube (7.5 mm smallest feature, so the real 3 mm wrap is thickened).",
         "Detailliertes Modell: Das hermetische Gehäuse ist eine gedrehte Stahldose mit einer 6-mm-Schulter unter dem Dom, die im Loch des Sockels steht; der Sockel ist eine gerundete 280-mm-Platte mit 192-mm-Loch, sodass ihre Oberseite ein echter Kreisring um das Gehäuse ist (eine volle Platte gälte als unter dem Gehäuse vergraben); der Dom ist eine geschlossene Drehung der Bezierkurve der Ansicht. Druckstutzen mit Flansch und Saugstutzen mit Muffe und Quetschende sind Kupferzylinder, der Klemmenkasten hat Deckel und eine 10 mm lange Kabelverschraubung, die die Breite von 300 mm vollmacht. Das Schauglas besteht aus Stahlring, 4 mm Glas (glazing), 8 mm dicker Spiralplatte am Knochen rotor, Nabe und der als ein 8-mm-Rohr geführten Spiralwand (kleinste Einzelheit 7,5 mm, die echte 3-mm-Wand ist daher verdickt).")
V.decide(50, "Thermal: every region is a reference to the body solids that carry its surface (single-layer regions are single surfaces: the discharge region is stub and flange, the suction region stub, collar and crimp, the terminal region box, lid and gland; areaM2 is the exposed area of these solids, the foot plate bottom lies in the support plane); the plume emitter is a 120 mm outlet ring above the dome apex. Structure: the shell and dome walls are 3 mm thin-walled revolved members, the terminal box and the scroll plate are members, the weld ring, terminal pad and rotor bearing are joints, shell motor stack and dome oil mass are mass ellipsoids at their centres of mass, the 260 mm contact pad (anchor base-contact) is the support.",
         "Thermik: Jede Region verweist auf die Körper, die ihre Oberfläche tragen (einschichtige Regionen sind einfache Flächen: Druckregion = Stutzen und Flansch, Saugregion = Stutzen, Muffe und Quetschende, Klemmenregion = Kasten, Deckel und Verschraubung; areaM2 ist die freie Außenfläche dieser Körper, die Unterseite der Fußplatte liegt in der Auflagerebene); der Fahnen-Emitter ist ein Austrittsring von 120 mm über dem Domscheitel. Tragwerk: Gehäuse- und Domwand sind 3 mm dünne gedrehte Stäbe, Klemmenkasten und Spiralplatte sind Stäbe, Schweißring, Klemmenfuß und Rotorlager sind Gelenke, Motorpaket im Gehäuse und Ölmasse im Dom sind Massenellipsoide in ihren Schwerpunkten, die 260-mm-Kontaktplatte ist das Auflager.")

S = V.at(100, "body")
S.ext("foot-plate", "shell", "plan", plate(0.14, 0.006, 2), 0, 0.03, holes=[ring(0.096, 32)], tone=0.4, **SS)
S.cyl("shell", "shell", [0, 0, 0], [0, 0.34, 0], 0.1, tone=0.5, **SS)
S.rev("dome", "dome", [0, 0.34, 0], [0, 1, 0], [[y, r] for y, r in bez(5)] + [[0, 0]], tone=0.7, **SS)
S.cyl("discharge-stub", "dome", [0, 0.398, 0], [0, 0.432, 0], 0.0125, tone=0.7, **CU)
S.cyl("suction-stub", "shell", [-0.15, 0.207, 0], [-0.098, 0.207, 0], 0.014, tone=0.2, **CU)
S.box("terminal", "terminal", [0.122, 0.28, 0], [0.044, 0.06, 0.06], tone=0.3, **PA)
S.cyl("scroll-disc", "rotor", [0, 0.18, -0.1], [0, 0.18, -0.115], 0.045, tone=0.8, role="body", **CA)
sp = [([p[0], 0.18 + p[1], -0.115], 0.0075, 0.0075) for p in spiral[::4]]
S.loft("volute", "rotor", sp, tone=0.1, role="detail", **CA)
T = V.at(100, "thermal")
for a, b, reg in [("th-shell", "shell", "shell"), ("th-dome", "dome", "dome"), ("th-discharge", "discharge-stub", "discharge"), ("th-suction", "suction-stub", "suction"), ("th-terminal", "terminal", "terminal"), ("th-feet", "foot-plate", "feet"), ("th-scroll-disc", "scroll-disc", "scroll")]:
    T.ref(a, b, region=reg)
T.cyl("th-discharge-flange", "dome", [0, 0.415, 0], [0, 0.43, 0], 0.0215, tone=0.7, region="discharge", **CU)
T.rev("th-dome-plume-outlet", "dome", [0, 0.4, 0], [0, 1, 0], [[0, 0.045], [0, 0.06], [0.016, 0.06], [0.016, 0.045], [0, 0.045]], tone=0.9, region="dome", tags=["dome-plume"], **SS)
R = V.at(100, "structure")
R.rev("member-shell", "shell", [0, 0, 0], [0, 1, 0], [[0, 0.096], [0, 0.1], [0.34, 0.1], [0.34, 0.096], [0, 0.096]], tone=0.5, member="shell", part="member", **SS)
R.rev("member-dome", "dome", [0, 0.34, 0], [0, 1, 0], hollow(bez(5), 0.004), tone=0.7, member="dome", part="member", **SS)
R.ref("member-terminal", "terminal", member="terminal", part="member")
R.ref("member-rotor", "scroll-disc", member="rotor", part="member")
R.rev("joint-rotor-bearing", "rotor", [0, 0.18, -0.09], [0, 0, -1], [[0, 0], [0, 0.02], [0.02, 0.02], [0.02, 0]], tone=0.3, member="rotor", part="joint", **SS)
R.ell("mass-shell", "shell", [0, 0.17, 0], [0.07, 0.1, 0.07], tone=0.6, member="shell", part="mass", **SS)
R.ell("mass-dome", "dome", [0, 0.37, 0], [0.05, 0.025, 0.05], tone=0.6, member="dome", part="mass", **SS)
R.box("base-contact-pad", "shell", [0, 0.0075, 0], [0.26, 0.015, 0.26], tone=0.4, member="shell", part="support", anchor="base-contact", **SS)
V.decide(100, "Re-drawn against the 1:100 shapes: the shell is a plain cylinder without shoulders, the dome a 5-segment revolve, the flange, collar, crimp, lid and gland vanish into the discharge stub (32 mm), suction stub (34 mm) and the terminal box (44 mm). The sight window is one 20 mm scroll disc on the rotor bone with the spiral merged into one 15 mm volute; ring and glass are merged into the disc.",
         "Neu gezeichnet gegen die Formen 1:100: Das Gehäuse ist ein glatter Zylinder ohne Schultern, der Dom eine Drehung mit 5 Segmenten, Flansch, Muffe, Quetschende, Deckel und Verschraubung gehen in Druckstutzen (32 mm), Saugstutzen (34 mm) und Klemmenkasten (44 mm) auf. Das Schauglas ist eine 20 mm dicke Spiralscheibe am Knochen rotor, die Spirale zu einer 15-mm-Volute zusammengefasst; Ring und Glas gehen in der Scheibe auf.")
V.decide(100, "Thermal: references to the body solids plus a 43 mm discharge flange ring that only the thermal list keeps (the 1:100 drawing merges it into the stub), the plume outlet is a 15 mm ring. Structure: shell and dome as 4 mm thin-walled members, the terminal box and scroll disc as members, one rotor bearing joint (the fixed weld ring and terminal pad are merged into the members), two mass ellipsoids and a 15 mm contact pad.",
         "Thermik: Verweise auf die Körper sowie ein 43-mm-Druckflanschring, den nur die Thermikliste behält (die Zeichnung 1:100 fasst ihn im Stutzen zusammen), der Fahnenaustritt ist ein 15-mm-Ring. Tragwerk: Gehäuse und Dom als 4 mm dünnwandige Stäbe, Klemmenkasten und Spiralscheibe als Stäbe, ein Rotorlager als Gelenk (Schweißring und Klemmenfuß gehen in den Stäben auf), zwei Massenellipsoide und eine 15 mm dicke Kontaktplatte.")

S = V.at(200, "body")
S.ext("foot-plate", "shell", "plan", plate(0.14, 0.01, 1), 0, 0.04, holes=[ring(0.095, 24)], tone=0.4, **SS)
S.cyl("shell", "shell", [0, 0, 0], [0, 0.34, 0], 0.1, tone=0.5, **SS)
S.rev("dome", "dome", [0, 0.34, 0], [0, 1, 0], [[0, 0.1], [0.03, 0.092], [0.052, 0.06], [0.06, 0], [0, 0]], tone=0.7, **SS)
S.cyl("discharge-stub", "dome", [0, 0.396, 0], [0, 0.43, 0], 0.015, tone=0.7, **CU)
S.cyl("suction-stub", "shell", [-0.16, 0.205, 0], [-0.098, 0.205, 0], 0.015, tone=0.2, **CU)
S.box("terminal", "terminal", [0.125, 0.28, 0], [0.05, 0.06, 0.05], tone=0.3, **PA)
T = V.at(200, "thermal")
for a, b, reg in [("th-shell", "shell", "shell"), ("th-dome", "dome", "dome"), ("th-discharge", "discharge-stub", "discharge"), ("th-suction", "suction-stub", "suction"), ("th-terminal", "terminal", "terminal"), ("th-feet", "foot-plate", "feet")]:
    T.ref(a, b, region=reg)
T.cyl("th-scroll-window", "rotor", [0, 0.18, -0.098], [0, 0.18, -0.128], 0.045, tone=0.8, role="body", region="scroll", **CA)
T.cyl("th-discharge-flange", "dome", [0, 0.4, 0], [0, 0.43, 0], 0.018, tone=0.7, region="discharge", **CU)
T.rev("th-dome-plume-outlet", "dome", [0, 0.4, 0], [0, 1, 0], [[0, 0.04], [0, 0.06], [0.03, 0.06], [0.03, 0.04], [0, 0.04]], tone=0.9, region="dome", tags=["dome-plume"], **SS)
R = V.at(200, "structure")
R.cyl("member-shell", "shell", [0, 0, 0], [0, 0.34, 0], 0.1, tone=0.5, member="shell", part="member", **SS)
R.ref("member-dome", "dome", member="dome", part="member")
R.ref("member-terminal", "terminal", member="terminal", part="member")
R.cyl("member-rotor", "rotor", [0, 0.18, -0.09], [0, 0.18, -0.06], 0.04, tone=0.8, member="rotor", part="member", role="body", **CA)
R.rev("joint-rotor-bearing", "rotor", [0, 0.18, -0.09], [0, 0, 1], [[0, 0], [0, 0.03], [0.03, 0.03], [0.03, 0]], tone=0.3, member="rotor", part="joint", **SS)
R.ell("mass-shell", "shell", [0, 0.17, 0], [0.07, 0.1, 0.07], tone=0.6, member="shell", part="mass", **SS)
R.ell("mass-dome", "dome", [0, 0.37, 0], [0.05, 0.03, 0.05], tone=0.6, member="dome", part="mass", **SS)
R.box("base-contact-pad", "shell", [0, 0.015, 0], [0.26, 0.03, 0.26], tone=0.4, member="shell", part="support", anchor="base-contact", **SS)
V.decide(200, "Six solids drawn against the 1:200 shapes: plate, shell cylinder, 4-point dome, two 40 mm pipe stubs and the 50 mm terminal box. The sight window is below the 30 mm smallest feature and is not drawn at this scale; the scroll set lives in structure as a hidden 80 mm disc inside the shell and in thermal as the window disc on the shell face.",
         "Sechs Körper gegen die Formen 1:200: Platte, Gehäusezylinder, Dom mit 4 Punkten, zwei 40-mm-Rohrstutzen und der 50-mm-Klemmenkasten. Das Schauglas liegt unter der kleinsten Einzelheit von 30 mm und wird in diesem Maßstab nicht gezeichnet; der Spiralsatz lebt im Tragwerk als verdeckte 80-mm-Scheibe im Gehäuse und in der Thermik als Fensterscheibe auf der Gehäusefläche.")
V.decide(200, "Thermal: references to the body solids plus the 90 mm scroll window disc (30 mm thick, the generalised sight window and scroll behind it), a 36 mm discharge flange and a 30 mm plume ring. Structure: shell as a solid 30 mm-feature cylinder member (the 3 mm wall is below the feature), dome and terminal box as members, rotor scroll disc member with one bearing joint, two masses, contact pad.",
         "Thermik: Verweise auf die Körper sowie die 90-mm-Schauscheibe (30 mm dick, verallgemeinertes Schauglas mit der Spirale dahinter), ein 36-mm-Druckflansch und ein 30-mm-Fahnenring. Tragwerk: Gehäuse als massiver Zylinderstab (die 3-mm-Wand liegt unter der Einzelheit), Dom und Klemmenkasten als Stäbe, Spiralscheibe als Rotorstab mit einem Lagergelenk, zwei Massen, Kontaktplatte.")
V.write(unavailable=(500,))
