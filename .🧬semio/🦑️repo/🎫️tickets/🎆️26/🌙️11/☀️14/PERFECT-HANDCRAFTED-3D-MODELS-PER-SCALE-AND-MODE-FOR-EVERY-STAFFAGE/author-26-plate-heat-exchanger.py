import importlib.util, os
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("c26", os.path.join(here, "author-26-common.py"))
c = importlib.util.module_from_spec(spec); spec.loader.exec_module(c)
V = c.Volume("plate-heat-exchanger")
GS = dict(material="galvanised-steel", role="body")
AL = dict(material="aluminium-plate", role="body")
RECT = lambda y0, y1, z0, z1: [[y0, z0], [y1, z0], [y1, z1], [y0, z1]]
SQ = lambda h: [[-h, -h], [h, -h], [h, h], [-h, h]]
COLLARS = {"upper-left": (-1, 0.4), "lower-left": (-1, 0.06), "upper-right": (1, 0.4), "lower-right": (1, 0.06)}
EMIT = {"supply-air": ("upper-right", 1), "exhaust-air": ("lower-left", -1)}
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}


def body(sc, base, ring, post, wall_x, cwall, damper_t, seat=None, hold=None, stack=None, hinge=False, panel=False):
    b = V.at(sc, "body")
    b.box("frame-base", "frame", [0, base / 2, 0], [0.5, base, 0.5], tone=0.35, **GS)
    b.ext("frame-top", "frame", "plan", SQ(0.25), 0.6 - ring, 0.6, holes=[SQ(0.2)], tone=0.35, **GS)
    ph = 0.6 - ring - base
    for n, (x, z) in ({} if panel else {"rear-left": (-1, -1), "rear-right": (1, -1), "front-left": (-1, 1), "front-right": (1, 1)}).items():
        b.box(f"frame-post-{n}", "frame", [x * (0.25 - post / 2), base + ph / 2, z * (0.25 - post / 2)], [post, ph, post], tone=0.35, **GS)
    holes = [RECT(y + cwall, y + 0.15 - cwall, -0.15 + cwall, 0.15 - cwall) for y in (0.4, 0.06)]
    hz = 0.25 if panel else 0.2
    for n, x0, x1 in (("left", -0.25, -0.25 + wall_x), ("right", 0.25 - wall_x, 0.25)):
        b.ext(f"end-wall-{n}", "frame", "front", RECT(base, 0.6 - ring, -hz, hz), x0, x1, holes=holes, tone=0.35, **GS)
    if seat:
        b.box("pack-end-plate-bottom", "frame", [0, base + seat / 2, 0], [0.44, seat, 0.4], tone=0.3, **GS)
    if hold:
        b.box("pack-end-plate-top", "frame", [0, 0.6 - ring - hold / 2, 0], [0.44, hold, 0.4], tone=0.3, **GS)
    if stack:
        y0 = base + (seat or 0)
        n, step = stack
        b.box("plate-pack", "plates", [0, y0 + step / 2, 0], [0.44, step, 0.4], rep={"count": n // 2, "step": [0, 2 * step, 0]}, tone=0.7, **AL)
        b.box("plate-pack-channels", "plates", [0, y0 + 1.5 * step, 0], [0.44, step, 0.38], rep={"count": n // 2, "step": [0, 2 * step, 0]}, tone=0.6, **AL)
    else:
        y0 = base
        y1 = 0.6 - ring
        pw = 0.4 if panel else 0.44
        b.box("plate-pack", "plates", [0, (y0 + y1) / 2, 0], [pw, y1 - y0, 0.4], tone=0.7, **AL)
    for n, (sx, y) in COLLARS.items():
        x0, x1 = (-0.31, -0.25) if sx < 0 else (0.25, 0.31)
        b.ext(f"collar-{n}", "ports", "front", RECT(y, y + 0.15, -0.15, 0.15), x0, x1, holes=[RECT(y + cwall, y + 0.15 - cwall, -0.15 + cwall, 0.15 - cwall)], tone=0.4, **GS)
    b.box("damper", "damper", [0, 0.6 + damper_t / 2, 0], [0.4, damper_t, 0.4], tone=0.3, **GS)
    if hinge:
        b.cap("damper-hinge", "damper", [-0.21, 0.61, -0.2], [-0.21, 0.61, 0.2], 0.01, role="detail", slender=True, tone=0.1, material="galvanised-steel")


def thermal(sc, extra_frame=()):
    t = V.at(sc, "thermal")
    ids = {s["id"] for s in V.data[sc]["body"]}
    regs = {"frame": ["frame-base", "frame-top", "frame-post-rear-left", "frame-post-rear-right", "frame-post-front-left", "frame-post-front-right", "end-wall-left", "end-wall-right", "pack-end-plate-bottom", "pack-end-plate-top"],
            "plates": ["plate-pack", "plate-pack-channels"],
            "ports": [f"collar-{n}" for n in COLLARS],
            "damper": ["damper", "damper-hinge"]}
    for r, lst in regs.items():
        for s in lst:
            if s in ids:
                t.ref(f"th-{s}", s, region=r)
    f = FEATURE[sc]
    for e, (n, sx) in EMIT.items():
        y = COLLARS[n][1]
        wall = {50: 0.01, 100: 0.015, 200: 0.03}[sc]
        x0, x1 = (0.31 - f * 1.001, 0.31) if sx > 0 else (-0.31, -0.31 + f * 1.001)
        t.ext(f"th-{e}-outlet", "ports", "front", RECT(y + wall, y + 0.15 - wall, -0.15 + wall, 0.15 - wall), x0, x1, region="ports", tags=[e], sectionFaces=["-u", "+u"], material="galvanised-steel", role="body", tone=0.9)


def structure(sc, joints, pack_mass, frame_mass, pad_t):
    r = V.at(sc, "structure")
    ids = {s["id"] for s in V.data[sc]["body"]}
    for s in ["frame-base", "frame-top", "frame-post-rear-left", "frame-post-rear-right", "frame-post-front-left", "frame-post-front-right", "end-wall-left", "end-wall-right"]:
        if s in ids:
            r.ref(f"st-{s}", s, member="frame", part="member")
    r.ref("st-plate-pack", "plate-pack", member="plates", part="member")
    if "plate-pack-channels" in ids:
        r.ref("st-plate-pack-channels", "plate-pack-channels", member="plates", part="member")
    for n in COLLARS:
        r.ref(f"st-collar-{n}", f"collar-{n}", member="ports", part="member")
    r.ref("st-damper", "damper", member="damper", part="member")
    if joints:
        for s, m in (("damper-hinge", "damper"), ("pack-end-plate-bottom", "plates"), ("pack-end-plate-top", "plates")):
            if s in ids:
                r.ref(f"st-joint-{s}", s, member=m, part="joint")
    pack_mass(r)
    frame_mass(r)
    r.box("base-contact-pad", "frame", [0, pad_t / 2, 0], [0.5 - 2 * pad_t, pad_t, 0.5 - 2 * pad_t], tone=0.4, member="frame", part="support", anchor="base-contact", **GS)


body(50, 0.02, 0.02, 0.05, 0.03, 0.01, 0.014, seat=0.01, hold=0.01, stack=(12, 0.045), hinge=True)
thermal(50)
structure(50, True,
          lambda r: r.box("mass-plates", "plates", [0, 0.3, 0], [0.3, 0.38, 0.28], tone=0.7, member="plates", part="mass", **AL),
          lambda r: r.box("mass-frame", "frame", [0, 0.3, 0], [0.1, 0.1, 0.1], tone=0.35, member="frame", part="mass", **GS), 0.0075)
V.decide(50, "Detailed model: a galvanised casing of base slab (20 mm), top ring (20 mm, opening 0.4 m by 0.4 m), four 50 mm corner posts and two 30 mm end walls whose 0.28 m by 0.13 m openings line up with the four 60 mm duct collars (rectangular stubs with 10 mm walls); the pack is closed by a 10 mm bottom and a 10 mm top end plate (0.44 m by 0.4 m) that bridge it to base slab and top ring, so nothing floats and the frame interior has no gap. The plate pack is twelve aluminium sub-stacks of 45 mm (each stands for about 17 of the 200 real plates of 0.3 mm at 2.5 mm pitch, far below the 7.5 mm smallest feature), alternately 0.40 m and 0.38 m deep so that the front and back plate edges read as the alternating open and closed channel ends of a cross-flow pack, open between the posts; the bypass flap is a 14 mm plate on a 20 mm hinge barrel along the left edge.",
         "Detailliertes Modell: Ein verzinktes Gehäuse aus Bodenplatte (20 mm), Deckring (20 mm, Öffnung 0,4 m mal 0,4 m), vier 50-mm-Eckpfosten und zwei 30-mm-Stirnwänden, deren Öffnungen von 0,28 m mal 0,13 m auf die vier 60 mm langen Kanalstutzen (Rechteckstutzen mit 10 mm Wand) fluchten; das Paket wird von einer 10 mm dicken unteren und oberen Endplatte (0,44 m mal 0,4 m) abgeschlossen, die es mit Bodenplatte und Deckring verbinden, nichts schwebt und im Gehäuse bleibt keine Lücke. Das Plattenpaket besteht aus zwölf Aluminium-Teilstapeln zu 45 mm (jeder steht für etwa 17 der 200 echten Platten von 0,3 mm im Abstand von 2,5 mm, weit unter der kleinsten Einzelheit von 7,5 mm), abwechselnd 0,40 m und 0,38 m tief, sodass die Plattenkanten vorn und hinten wie die wechselnd offenen und geschlossenen Kanalenden eines Kreuzstrompakets lesbar sind und zwischen den Pfosten offen liegen; die Bypass-Klappe ist eine 14-mm-Platte auf einer 20-mm-Scharniertrommel am linken Rand.")
V.decide(50, "Thermal: every region is a set of references to the body solids that carry its surface (frame: casing parts and the two end plates; plates: the pack; ports: the four collars; damper: flap and hinge); each region has one layer, so each is a single surface and no nested shell exists. The two vent emitters are 7.5 mm outlet plates inside the mouth of the upper-right and lower-left collars (their faces are declared section faces: an air outlet plane carries no surface). Structure: posts, base, ring and end walls are the frame members, the pack, the collars and the flap are the members of their bones, hinge and the two end plates are joints, the 12 kg pack and the 5 kg casing are compact mass boxes at their centres, the 0.5 m contact pad (anchor base-contact) is the support.",
         "Thermik: Jede Region ist eine Menge von Verweisen auf die Körper, die ihre Oberfläche tragen (frame: Gehäuseteile und die beiden Endplatten; plates: das Paket; ports: die vier Stutzen; damper: Klappe und Scharnier); jede Region hat eine Schicht, also eine einfache Fläche ohne verschachtelte Schalen. Die beiden Lüftungsemitter sind 7,5-mm-Austrittsplatten im Mund des Stutzens oben rechts und unten links (ihre Flächen sind als Schnittflächen erklärt: eine Austrittsebene trägt keine Oberfläche). Tragwerk: Pfosten, Boden, Ring und Stirnwände sind die Rahmenstäbe, Paket, Stutzen und Klappe die Stäbe ihrer Knochen, Scharnier und die beiden Endplatten sind Gelenke, das 12-kg-Paket und das 5-kg-Gehäuse sind kompakte Massenquader in ihren Schwerpunkten, die 0,5 m große Kontaktplatte (Anker base-contact) ist das Auflager.")

body(100, 0.02, 0.02, 0.05, 0.03, 0.015, 0.02, stack=None)
thermal(100)
structure(100, True,
          lambda r: r.box("mass-plates", "plates", [0, 0.3, 0], [0.3, 0.4, 0.26], tone=0.7, member="plates", part="mass", **AL),
          lambda r: r.box("mass-frame", "frame", [0, 0.3, 0], [0.09, 0.09, 0.09], tone=0.35, member="frame", part="mass", **GS), 0.015)
r = V.at(100, "structure")
r.cap("joint-damper-hinge", "damper", [-0.21, 0.61, -0.2], [-0.21, 0.61, 0.2], 0.01, slender=True, tone=0.1, member="damper", part="joint", material="galvanised-steel", role="body")
V.decide(100, "Redrawn against the 1:100 shapes: the plate pack is one 0.44 m by 0.56 m by 0.4 m aluminium block that fills the casing between base slab and top ring (the 10 mm end plates of 1:50 are below the 15 mm smallest feature and merge into it), the collars are 60 mm stubs with 15 mm walls, the damper is a single 20 mm flap whose hinge barrel is merged into it; casing, posts and end walls keep their 1:50 sizes because they are already above 15 mm.",
         "Neu gezeichnet gegen die Formen 1:100: Das Plattenpaket ist ein einziger Aluminiumblock von 0,44 m mal 0,56 m mal 0,4 m, der das Gehäuse zwischen Bodenplatte und Deckring ausfüllt (die 10-mm-Endplatten von 1:50 liegen unter der kleinsten Einzelheit von 15 mm und gehen darin auf), die Stutzen sind 60-mm-Stutzen mit 15 mm Wand, die Klappe ist eine einzige 20-mm-Platte, deren Scharniertrommel in ihr aufgeht; Gehäuse, Pfosten und Stirnwände behalten ihre Maße von 1:50, da sie schon über 15 mm liegen.")
V.decide(100, "Thermal: references to the body solids of each region, vent emitters as 15 mm outlet plates in the collar mouths. Structure: the same frame, pack, collar and flap members as references, one hinge joint, a pack mass ellipsoid and a casing mass ellipsoid, a 15 mm contact pad as support.",
         "Thermik: Verweise auf die Körper jeder Region, Lüftungsemitter als 15-mm-Austrittsplatten im Stutzenmund. Tragwerk: dieselben Rahmen-, Paket-, Stutzen- und Klappenstäbe als Verweise, ein Scharniergelenk, ein Massenellipsoid für das Paket und eines für das Gehäuse, eine 15 mm dicke Kontaktplatte als Auflager.")

body(200, 0.03, 0.03, 0.05, 0.05, 0.03, 0.04, stack=None, panel=True)
thermal(200)
structure(200, False,
          lambda r: r.box("mass-plates", "plates", [0, 0.3, 0], [0.32, 0.4, 0.3], tone=0.7, member="plates", part="mass", **AL),
          lambda r: r.box("mass-frame", "frame", [0, 0.3, 0], [0.08, 0.08, 0.08], tone=0.35, member="frame", part="mass", **GS), 0.03)
V.decide(200, "Five masses against the 1:200 shapes: the casing (30 mm base slab and top ring, two 50 mm end panels that take over the corner posts), the single aluminium block of the pack, four 60 mm collar stubs with 30 mm walls, and one 40 mm flap (the drawn thickness) whose hinge is merged into it; the 30 mm smallest feature leaves no end plates, posts or barrel.",
         "Fünf Massen gegen die Formen 1:200: das Gehäuse (30-mm-Bodenplatte und Deckring, zwei 50-mm-Stirnwände, die die Eckpfosten übernehmen), der einzelne Aluminiumblock des Pakets, vier 60 mm lange Stutzen mit 30 mm Wand und eine 40-mm-Klappe (die gezeichnete Dicke), deren Scharnier in ihr aufgeht; die kleinste Einzelheit von 30 mm lässt keine Endplatten, Pfosten und keine Trommel zu.")
V.decide(200, "Thermal: references to the body solids, vent emitters as 30 mm plates in the collar mouths. Structure: the frame, pack, collar and flap members, no separate joints (merged into the members), two mass boxes and a 30 mm contact pad.",
         "Thermik: Verweise auf die Körper, Lüftungsemitter als 30-mm-Platten im Stutzenmund. Tragwerk: Rahmen-, Paket-, Stutzen- und Klappenstäbe, keine eigenen Gelenke (in den Stäben zusammengefasst), zwei Massenquader und eine 30 mm dicke Kontaktplatte.")
V.write(unavailable=(500,))
