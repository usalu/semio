import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-36-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("heat-pump-outdoor-unit")
PC = dict(material="powder-coat", role="body")
PCD = dict(material="powder-coat", role="detail")
GS = dict(material="galvanised-steel", role="body")
GSD = dict(material="galvanised-steel", role="detail")
AL = dict(material="aluminium", role="detail")
RB = dict(material="rubber", role="body")
rect = lambda y0, y1, z0, z1: [(y0, z0), (y1, z0), (y1, z1), (y0, z1)]
FY, FZ = 0.8, 0.12
circ = lambda y, z, r, n=32: [(y + r * math.sin(2 * math.pi * k / n), z + r * math.cos(2 * math.pi * k / n)) for k in range(n)]
EPS = 1.03


def feet(h=0.09):
    V.box("foot-left", "unit", [0, h / 2, 0.43], [0.34, h, 0.18], tone=0.2, region="feet", **RB)
    V.box("foot-right", "unit", [0, h / 2, -0.43], [0.34, h, 0.18], tone=0.2, region="feet", **RB)


def shell(t, top_t, front_t, fr_hole, coil_win=True, y0=0.32, top=1.41):
    V.box("rear-wall", "unit", [-0.225 + t / 2, (y0 + top) / 2, 0], [t, top - y0, 1.1], tone=0.55, region="casing", **PC)
    V.box("top-wall", "unit", [0, top - top_t / 2, 0], [0.45, top_t, 1.1], tone=0.55, region="casing", **PC)
    for side, z0, z1 in [("left", 0.55 - t, 0.55), ("right", -0.55, -0.55 + t)]:
        V.ext(f"side-wall-{side}", "unit", "side", [(-0.225, y0), (0.225, y0), (0.225, top), (-0.225, top)], z0, z1, holes=[[(-0.13, 0.42), (0.09, 0.42), (0.09, 1.28), (-0.13, 1.28)]] if coil_win else None, tone=0.55, region="casing", **PC)
    V.ext("front-panel", "unit", "front", rect(y0, top, -0.55, 0.55), 0.225 - front_t, 0.225, holes=[circ(FY, FZ, 0.33)] if fr_hole else None, tone=0.55, region="casing", **PC)


def blades(count_thick, n_sec=5):
    fan_x = 0.19
    phi = math.radians(54.5)
    radial = [0, math.sin(phi), math.cos(phi)]
    tang = [0, -math.cos(phi), math.sin(phi)]
    pitch = math.radians(25)
    chord = [math.sin(pitch), math.cos(pitch) * tang[1], math.cos(pitch) * tang[2]]
    secs = []
    for r, rx in [(0.065, 0.02), (0.11, 0.036), (0.17, 0.043), (0.22, 0.036), (0.262, 0.018)][:n_sec]:
        secs.append(([fan_x, FY + r * radial[1], FZ + r * radial[2]], rx, count_thick, 2.4))
    V.loft("fan-blade", "unit", secs, hint=chord, tone=0.5, region="fan-guard", material="aluminium", role="detail", rep=dict(count=5, angle=-2 * math.pi / 5, axis=[1, 0, 0], origin=[fan_x, FY, FZ]))


def torus(id, r, x, wire, n=24, **kw):
    t = 2 * math.pi / n
    V.cap(id, "unit", [x, FY, FZ + r], [x, FY - r * math.sin(t), FZ + r * math.cos(t)], wire, rep=dict(count=n, angle=t, axis=[1, 0, 0], origin=[x, FY, FZ]), **kw)


def lid_vane(lid_t, vane_t, hinges, cr):
    V.box("lid", "lid", [0, 1.45 - lid_t / 2, 0], [0.47, lid_t, 1.12], cr=cr, tone=0.35, region="top-lid", **PC)
    if hinges:
        V.cyl("lid-hinge-front", "lid", [-0.215, 1.43, 0.25], [-0.215, 1.43, 0.5], 0.02 if hinges == 1 else 0.0155, tone=0.3, region="top-lid", **PCD)
        V.cyl("lid-hinge-rear", "lid", [-0.215, 1.43, -0.5], [-0.215, 1.43, -0.25], 0.02 if hinges == 1 else 0.0155, tone=0.3, region="top-lid", **PCD)
    V.box("vane", "vane", [0.36, 0.8, 0.12], [0.24, vane_t, 0.72], cr=min(cr * 4, 0.02) if cr else None, tone=0.3, region="discharge-vane", **PC)
    if hinges:
        V.cyl("vane-hinge", "vane", [0.26, 0.8, -0.24], [0.26, 0.8, 0.48], 0.012 if hinges == 1 else 0.0155, tone=0.3, region="discharge-vane", **PCD)


def inner(coil_t=0.22, tray=0.07, comp=0.17):
    V.box("base-tray", "unit", [0, 0.08 + tray / 2, 0], [0.45, tray, 1.1], tone=0.3, region="base-tray", **GS)
    V.box("compressor-panel", "unit", [0, 0.15 + comp / 2 + 0.0, 0], [0.45, comp, 1.1], cr=0.004, tone=0.4, region="compressor-casing", **PC)
    V.box("coil", "unit", [-0.02, 0.85, 0], [coil_t, 0.86, 1.06], tone=0.25, region="coil-guard", **AL)


def structure(scale, f):
    V.to("structure")
    for s in [s["id"] for s in V.lists["body"] if s["bone"] == "unit" and s["id"] not in ("fan-blade", "fan-motor", "motor-stay", "fan-hub", "guard-spoke", "guard-skirt", "guard-cap", "guard-rim", "fan-ring-outer", "fan-ring-inner", "fan-ring", "guard-bell", "panel-handle", "fan-disc", "fan-guard")]:
        V.member(s, "unit")
    for s in ("lid", "vane"):
        V.member(s, s)
    for sid, bone in [("lid-hinge-front", "lid"), ("lid-hinge-rear", "lid"), ("vane-hinge", "vane")]:
        if scale == 100 and sid == "lid-hinge-rear":
            continue
        if any(x["id"] == sid for x in V.lists["body"]):
            V.member(sid, bone, "joint", "joint-" + sid[:-0 or None])
    if not any(x["id"] == "vane-hinge" for x in V.lists["body"]):
        V.cyl("joint-vane-hinge", "vane", [0.26, 0.8, -0.2], [0.26, 0.8, 0.44], max(0.012, f * 0.6), tone=0.3, member="vane", part="joint", **PCD)
        V.cyl("joint-lid-hinge", "lid", [-0.215, 1.43, 0.25], [-0.215, 1.43, 0.5], max(0.02, f * 0.7), tone=0.3, member="lid", part="joint", **PCD)
    for side, z in (("l", 0.43), ("r", -0.43)):
        V.box(f"support-contact-{side}", "unit", [0, f / 2, z], [0.34, f, 0.18], tone=0.2, member="unit", part="support", anchor=f"contact-{side}", **RB)
    V.ell("mass-unit", "unit", [0, 0.6, 0], [0.12, 0.25, 0.3], tone=0.2, member="unit", part="mass", **PC)


def emitters(f):
    V.to("thermal")
    t = max(f, 0.0075) * EPS
    V.cyl("th-discharge-heating", "unit", [0.262, FY, FZ], [0.262 + t, FY, FZ], 0.25, tone=0.5, region="fan-guard", tags=["discharge-heating"], **GSD)
    V.cyl("th-discharge-cooling", "unit", [0.262 + t, FY, FZ], [0.262 + 2 * t, FY, FZ], 0.25, tone=0.5, region="fan-guard", tags=["discharge-cooling"], **GSD)
    V.cyl("th-defrost-vapour", "unit", [-0.035, 1.3, 0], [-0.035, 1.3 + t, 0], 0.2, tone=0.5, region="coil-guard", tags=["defrost-vapour"], **AL)


def s50():
    feet(0.09)
    inner()
    shell(0.015, 0.015, 0.02, True)
    V.box("panel-handle", "unit", [0.12, 0.85, -0.5525], [0.04, 0.1, 0.008], cr=0.003, tone=0.2, region="casing", **PC)
    V.cyl("fan-motor", "unit", [-0.02, FY, FZ], [0.16, FY, FZ], 0.055, tone=0.15, region="fan-guard", **GSD)
    V.box("motor-stay", "unit", [-0.03, 0.865, FZ], [0.02, 1.09, 0.03], tone=0.3, region="fan-guard", **GSD)
    V.rev("fan-hub", "unit", [0.16, FY, FZ], [1, 0, 0], [[0, 0.07], [0.05, 0.07], [0.068, 0.045], [0.075, 0]], tone=0.15, region="fan-guard", **GSD)
    blades(0.004)
    torus("guard-rim", 0.345, 0.222, 0.008, n=36, tone=0.3, region="fan-guard", **GS)
    torus("fan-ring-outer", 0.27, 0.26, 0.004, tone=0.3, region="fan-guard", **GSD)
    torus("fan-ring-inner", 0.19, 0.26, 0.004, tone=0.3, region="fan-guard", **GSD)
    V.cyl("guard-cap", "unit", [0.25, FY, FZ], [0.262, FY, FZ], 0.07, tone=0.3, region="fan-guard", **GSD)
    V.cap("guard-spoke", "unit", [0.26, FY, FZ + 0.07], [0.26, FY, FZ + 0.3], 0.003, tone=0.3, region="fan-guard", slender=True, rep=dict(count=8, angle=2 * math.pi / 8, axis=[1, 0, 0], origin=[0.26, FY, FZ]), **GSD)
    V.ext("guard-bell", "unit", "front", circ(FY, FZ, 0.345, 48), 0.222, 0.26, holes=[circ(FY, FZ, 0.3375, 48)], tone=0.3, region="fan-guard", **GS)
    lid_vane(0.04, 0.05, 1, 0.004)
    V.auto_thermal()
    emitters(0.0075)
    structure(50, 0.0077)
    V.decide("The casing is a thin-walled shell of 15 mm rear and top walls, two side walls with the coil window and a 20 mm front panel with the round fan opening, standing on a base tray and the compressor compartment; the coil block behind the windows and the fan behind the opening are modelled, so the unit can be looked into. The fan has five pitched blades as one loft with a rotational repeat about the fan axis (centre z 0.12, y 0.8 as in the front view), a spinner hub and a motor on a stay; the wire guard is a rim ring, two rings, eight spokes and twelve skirt rods, the spokes true size 6 mm slender wires and the rods 7.6 mm.",
             "Das Gehäuse ist eine dünnwandige Hülle aus 15 mm dicker Rück- und Deckwand, zwei Seitenwänden mit Registerfenster und einem 20 mm dicken Frontblech mit runder Ventilatoröffnung auf Bodenblech und Verdichterfach; Registerblock hinter den Fenstern und Ventilator hinter der Öffnung sind modelliert, man kann also hineinsehen. Der Ventilator hat fünf angestellte Flügel als ein Loft mit Rotationswiederholung um die Ventilatorachse (Mitte z 0,12, y 0,8 wie in der Vorderansicht), eine Nabe mit Haube und einen Motor an einer Strebe; das Drahtgitter besteht aus Randring, zwei Ringen, acht Speichen und zwölf Mantelstäben, Speichen als schlanke 6-mm-Drähte in Originalgröße und die Stäbe mit 7,6 mm.")
    V.decide("The fan guard is a rolled rim, two wire rings, eight spokes, a cap and a 38 mm bell-mouth sleeve (a thin tube): the sleeve fills the 45 mm depth that the elevation draws as a solid guard profile, so the side silhouette no longer shows a 35 mm hole between the casing and the rings; the conical skirt rods of the first version are gone. The contact anchors now describe the two rubber feet of the drawings (0.34 m by 0.18 m, left and right), and the two support pads lie under exactly those feet.",
             "Das Lüftergitter besteht aus gerolltem Rand, zwei Drahtringen, acht Speichen, einer Kappe und einer 38 mm tiefen Einlaufhülse (ein dünnes Rohr): die Hülse füllt die 45 mm Tiefe, die die Seitenansicht als massives Gitterprofil zeichnet, sodass die Seitenansicht keine 35-mm-Lücke zwischen Gehäuse und Ringen mehr zeigt; die kegeligen Mantelstäbe der ersten Fassung entfallen. Die Aufstandsanker beschreiben jetzt die zwei Gummifüße der Zeichnungen (0,34 m mal 0,18 m, links und rechts), und die zwei Auflager liegen genau unter diesen Füßen.")
    V.decide("The lid and the vane carry their hinge rods on their own bones so they swing about the rig axes without leaving the casing; the lid and vane are 56 mm and 24 mm wider than the casing as drawn; two rubber feet carry the unit.",
             "Deckel und Lamelle tragen ihre Scharnierstangen auf den eigenen Knochen und schwenken um die Achsen des Rigs, ohne das Gehäuse zu verlassen; Deckel und Lamelle sind wie gezeichnet 56 mm und 24 mm breiter als das Gehäuse; zwei Gummifüße tragen das Gerät.")
    V.decide("Thermal: every region is one thin skin, so each thermal solid references its body solid (casing walls and handle, coil block, compressor panel, tray, fan parts, lid, vane, feet); the heating and cooling discharges are two 7.7 mm discs of 500 mm in front of the guard and the defrost vapour a 400 mm disc at the coil top, each tagged with its emitter id on the region it leaves from.",
             "Thermik: Jede Region ist eine dünne Haut, daher verweist jeder Thermikkörper auf seinen Körper (Gehäusewände und Griff, Registerblock, Verdichterblech, Wanne, Ventilatorteile, Deckel, Lamelle, Füße); die Heiz- und Kühlausblasung sind zwei 7,7 mm dicke Scheiben von 500 mm vor dem Gitter und der Abtaudampf eine 400-mm-Scheibe an der Registeroberseite, jeweils mit der Emitter-Id an der Region markiert, aus der sie austreten.")
    V.decide("Structure: walls, front panel, tray, compressor compartment, coil block, lid, vane and feet carry the load; the lid knuckles and the vane rod are the hinge joints, two 340 mm by 180 mm pads sit under the feet at the two contact anchors and one mass of 141 kg sits in the middle of the casing.",
             "Tragwerk: Wände, Frontblech, Wanne, Verdichterfach, Registerblock, Deckel, Lamelle und Füße tragen die Last; die Deckelknoten und die Lamellenstange sind die Scharniergelenke, zwei Auflager von 340 mm mal 180 mm sitzen unter den Füßen an den beiden Aufstandsankern, eine Masse von 141 kg sitzt in der Gehäusemitte.")
    V.finish(50)


def s100():
    feet(0.09)
    inner(0.22, 0.07, 0.17)
    shell(0.02, 0.02, 0.025, True)
    V.cyl("fan-motor", "unit", [-0.02, FY, FZ], [0.16, FY, FZ], 0.055, tone=0.15, region="fan-guard", **GSD)
    V.rev("fan-hub", "unit", [0.16, FY, FZ], [1, 0, 0], [[0, 0.07], [0.05, 0.07], [0.068, 0.045], [0.075, 0]], tone=0.15, region="fan-guard", **GSD)
    blades(0.0078, 4)
    torus("guard-rim", 0.345, 0.222, 0.0155, n=24, tone=0.3, region="fan-guard", **GS)
    torus("fan-ring", 0.24, 0.26, 0.0155, n=18, tone=0.3, region="fan-guard", **GSD)
    V.cyl("guard-cap", "unit", [0.24, FY, FZ], [0.262, FY, FZ], 0.07, tone=0.3, region="fan-guard", **GSD)
    V.ext("guard-bell", "unit", "front", circ(FY, FZ, 0.345, 48), 0.222, 0.26, holes=[circ(FY, FZ, 0.3295, 48)], tone=0.3, region="fan-guard", **GS)
    lid_vane(0.04, 0.05, 2, 0.01)
    V.auto_thermal()
    emitters(0.0155)
    structure(100, 0.0155)
    V.decide("At 1:100 the shell keeps the same parts with 20 mm walls and a 25 mm front panel, the five blades have 15.6 mm thick sections and four sections each, the wire guard is reduced to the rim and one 31 mm ring (spokes, skirt rods, motor stay, handle and the second ring are below 15 mm), hinge rods are 31 mm.",
             "Bei 1:100 behält die Hülle dieselben Teile mit 20 mm Wänden und 25 mm Frontblech, die fünf Flügel haben 15,6 mm dicke Schnitte und je vier Schnitte, das Drahtgitter schrumpft auf den Randring und einen 31-mm-Ring (Speichen, Mantelstäbe, Motorstrebe, Griff und der zweite Ring liegen unter 15 mm), Scharnierstangen sind 31 mm dick.")
    V.decide("The bell-mouth sleeve of 1:50 stays as a 38 mm tube of 15.5 mm wall.",
             "Die Einlaufhülse von 1:50 bleibt als 38-mm-Rohr mit 15,5 mm Wand.")
    V.decide("Thermal: as at 1:50 for the remaining parts; the emitter discs are 15.5 mm thick.",
             "Thermik: wie bei 1:50 für die verbleibenden Teile; die Emitterscheiben sind 15,5 mm dick.")
    V.decide("Structure: the same members as at 1:50 without the removed parts, one lid knuckle (the rear knuckle shares the front joint axis), joints and masses sized to 15 mm, pads 15.5 mm thick.",
             "Tragwerk: dieselben Stäbe wie bei 1:50 ohne die entfallenen Teile, ein Deckelknoten (der hintere teilt die Achse des vorderen), Gelenke und Massen auf 15 mm bemessen, Auflager 15,5 mm dick.")
    V.finish(100)


def s200():
    feet(0.09)
    V.box("base-tray", "unit", [0, 0.115, 0], [0.45, 0.07, 1.1], tone=0.3, region="base-tray", **GS)
    V.box("compressor-panel", "unit", [0, 0.235, 0], [0.45, 0.17, 1.1], tone=0.4, region="compressor-casing", **PC)
    V.box("coil", "unit", [-0.02, 0.85, 0], [0.22, 0.86, 1.06], tone=0.25, region="coil-guard", **AL)
    shell(0.031, 0.031, 0.031, False, coil_win=False)
    V.cyl("fan-guard", "unit", [0.215, FY, FZ], [0.26, FY, FZ], 0.35, tone=0.3, region="fan-guard", **GS)
    lid_vane(0.05, 0.06, 0, 0.0)
    V.auto_thermal()
    emitters(0.031)
    structure(200, 0.031)
    V.decide("At 1:200 the unit is a plain box shell (30 mm walls, solid front panel) with the base tray, compressor compartment and coil block inside; the fan opening, blades, hub, rings, spokes, handle and hinge rods are gone and the fan is one 45 mm deep guard disc of 700 mm in front of the panel as the 1:200 drawing shows it; lid and vane are plain 50 and 60 mm plates.",
             "Bei 1:200 ist das Gerät eine glatte Kastenhülle (30 mm Wände, geschlossenes Frontblech) mit Wanne, Verdichterfach und Registerblock innen; Ventilatoröffnung, Flügel, Nabe, Ringe, Speichen, Griff und Scharnierstangen entfallen, und der Ventilator ist eine 45 mm tiefe Gitterscheibe von 700 mm vor dem Blech, wie die 1:200-Zeichnung sie zeigt; Deckel und Lamelle sind glatte Platten von 50 und 60 mm.")
    V.decide("Thermal: references to the body parts; the fan-guard region is the disc; discharge and defrost emitters are 31 mm discs in front of the disc and at the coil top.",
             "Thermik: Verweise auf die Körperteile; die Region fan-guard ist die Scheibe; Ausblas- und Abtau-Emitter sind 31 mm dicke Scheiben vor der Scheibe und an der Registeroberseite.")
    V.decide("Structure: shell walls, tray, compressor compartment, coil, lid and vane as members, one hinge rod each for lid and vane as joints, two 31 mm pads under the feet and one mass.",
             "Tragwerk: Hüllenwände, Wanne, Verdichterfach, Register, Deckel und Lamelle als Stäbe, je eine Scharnierstange für Deckel und Lamelle als Gelenk, zwei 31 mm dicke Auflager unter den Füßen und eine Masse.")
    V.finish(200)


def s500():
    feet(0.09)
    V.box("base-tray", "unit", [0, 0.1275, 0], [0.45, 0.0755, 1.1], tone=0.3, region="base-tray", **GS)
    V.box("compressor-panel", "unit", [0, 0.2425, 0], [0.45, 0.155, 1.1], tone=0.4, region="compressor-casing", **PC)
    shell(0.0755, 0.0755, 0.0755, False, coil_win=False, y0=0.32, top=1.35)
    V.cyl("fan-disc", "unit", [0.1845, FY, FZ], [0.26, FY, FZ], 0.35, tone=0.3, region="fan-guard", **GS)
    lid_vane(0.1, 0.12, 0, 0.0)
    V.auto_thermal()
    V.to("thermal")
    V.box("th-coil", "unit", [-0.02, 0.85, 0], [0.22, 0.86, 1.06], tone=0.25, region="coil-guard", **AL)
    emitters(0.0755)
    structure(500, 0.0755)
    V.decide("At 1:500 the unit is one 485 mm deep block (the casing with its walls, panels and fan guard merged, as the 1:500 drawing does), two rubber feet of 90 mm, a 700 mm fan disc on the front face, a 100 mm lid plate and a 120 mm vane plate; walls, the shell opening, blades, rings, hinge rods and all fittings vanish into the block.",
             "Bei 1:500 ist das Gerät ein 485 mm tiefer Block (Gehäuse mit Wänden, Blechen und Lüftergitter verschmolzen, wie die 1:500-Zeichnung), zwei 90-mm-Gummifüße, eine 700-mm-Lüfterscheibe an der Frontfläche, eine 100 mm dicke Deckelplatte und eine 120 mm dicke Lamellenplatte; Wände, Gehäuseöffnung, Flügel, Ringe, Scharnierstangen und alle Beschläge gehen im Block auf.")
    V.decide("Minimal: the thermal list is already the smallest complete set, one solid per region plus the three emitters.",
             "Minimal: Die Thermikliste ist bereits der kleinste vollständige Satz, ein Körper je Region plus die drei Emitter.")
    V.decide("Thermal: coil, tray and compressor compartment have no surface in the block and are real solids inside it; the other regions reference the block, disc, plates and feet; emitters are 75 mm discs.",
             "Thermik: Register, Wanne und Verdichterfach haben im Block keine Oberfläche und sind echte Körper in ihm; die übrigen Regionen verweisen auf Block, Scheibe, Platten und Füße; Emitter sind 75 mm dicke Scheiben.")
    V.decide("Structure: block, lid, vane and feet are the members, hinge joints are 75 mm rods, two 75 mm pads under the feet and one mass in the block centre.",
             "Tragwerk: Block, Deckel, Lamelle und Füße sind die Stäbe, Scharniergelenke sind 75-mm-Stangen, zwei 75 mm dicke Auflager unter den Füßen und eine Masse in der Blockmitte.")
    V.finish(500)


s50(); s100(); s200(); s500()
V.write([50, 100, 200, 500])
