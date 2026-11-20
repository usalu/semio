"""🧊 Round 3, appliances/decentralised-hrv-fan: every solid of every authored scale and list, stated by hand; writes `volumes/appliances/decentralised-hrv-fan.json`.

Usage: python3 r3-decentralised-hrv-fan-volumes.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-decentralised-hrv-fan-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

SOURCE = os.path.join(HERE, "volumes", "appliances", "decentralised-hrv-fan.json")
AXIS = [0, 0, 1]
QUARTER = {"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": round(math.pi / 2, 7)}
THIRD = {"count": 3, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 3, 7)}
PIN = (-k.SCREW, k.SCREW)
ROLLER = (k.SCREW, -k.SCREW)
STEEL = "motor-housing"
CERAMIC = "ceramic-honeycomb"
EPP = "epp-foam"


def solid(id, kind, bone, material="abs-plastic", role="body", tone=0.7, **fields):
    """🧩 One solid with the common members first."""
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def rev(id, bone, profile, origin=(0, 0, 0), axis=AXIS, loop=True, **kw):
    """🌀 A revolve of (axial, radial) points; `loop` repeats the first point so the profile is a closed ring section."""
    pts = [k.pt(p) for p in profile]
    if loop:
        pts.append(pts[0])
    return solid(id, "revolve", bone, profile=pts, axis=list(axis), origin=k.pt(origin), **kw)


def square(id, z0, z1, hole, tone=0.88, half=k.PLATE / 2, radius=k.PLATE_R):
    """⬜ A rounded square of the base plate pushed from z0 to z1 with one hole."""
    return solid(id, "extrude", "plate", tone=tone, profile=k.rounded_square(half, radius), holes=[list(reversed(hole))], plane="xy", **{"from": z0, "to": z1})


def tube(id, outer, inner, z0, z1, tone=0.7, n=72):
    """⭕ A round tube of the sleeve as a pushed annulus of `n` sides, so that its outline stays round at any viewer quality."""
    return solid(id, "extrude", "ring", tone=tone, profile=k.circle(outer, n), holes=[list(reversed(k.circle(inner, n)))], plane="xy", **{"from": z0, "to": z1})


def honeycomb(radius, pitch, rings, cell, z0, z1):
    """🍯 The ceramic block: a cylinder with its channels as hexagonal holes right through."""
    holes = [list(reversed(k.hexagon(cx, cy, cell))) for cx, cy in k.cell_centres(pitch, rings)]
    return solid("core", "extrude", "core", material=CERAMIC, tone=0.62, profile=k.circle(radius), holes=holes, plane="xy", **{"from": z0, "to": z1})


def blade(i, **kw):
    """🪭 Blade i (1-based) as a loft of pitched symmetric lens sections."""
    return solid(f"blade-{i}", "loft", "rotor", tone=0.82, sections=k.blade_sections(i - 1, **kw), capStart=True, capEnd=True)


def strut(length, width, depth, centre, z, radius=None):
    """➖ The three radial struts of the motor mount: the first points up, the others follow at 120 degrees."""
    extra = {"cornerRadius": radius} if radius else {}
    return solid("strut", "box", "ring", tone=0.6, size=[width, length, depth], transform={"translation": [0, k.r6(centre), z]}, repeat=THIRD, **extra)


def ref(id, of, **kw):
    """🔗 A reference to a body solid of the same scale."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def th(s, region, layer=None, tags=None):
    """🌡️ A thermal solid of its own."""
    s = dict(s)
    s["region"] = region
    if layer:
        s["layer"] = layer
    if tags:
        s["tags"] = tags
    return s


def st(s, member, part, anchor=None):
    """🏗️ A structural solid of its own."""
    s = dict(s)
    s["member"] = member
    s["part"] = part
    if anchor:
        s["tags"] = [anchor]
        s["anchor"] = anchor
    return s


def pin(z0, z1, d):
    """📌 The upper left wall screw: a pin along w from its head through the plate into the wall plug."""
    return st(rev("support-wall-screw-top", "plate", [(z0, 0), (z0, d / 2), (z1, d / 2), (z1, 0)], origin=(PIN[0], PIN[1], 0), loop=False, material=STEEL, role="detail", tone=0.25), "plate", "support", "wall-screw-top")


def roller(length, d, zc):
    """🛼 The lower right wall screw: a roller lying on the wall face along u."""
    return st(rev("support-wall-screw-bottom", "plate", [(0, 0), (0, d / 2), (length, d / 2), (length, 0)], origin=(ROLLER[0] - length / 2, ROLLER[1], zc), axis=(1, 0, 0), loop=False, material=STEEL, role="detail", tone=0.25), "plate", "support", "wall-screw-bottom")


def mass(id, bone, r, z):
    """⚫ A concentrated mass at its centre of mass."""
    return st(solid(id, "ellipsoid", bone, role="detail", tone=0.15, radii=[r, r, r], transform={"translation": [0, 0, z]}), bone, "mass")


def outlet(id, tag, z0, z1, r):
    """💨 An outlet patch: a thin disc on the face of the ceramic where the air leaves it."""
    return th(rev(id, "core", [(z0, 0), (z0, r), (z1, r), (z1, 0)], loop=False, material=CERAMIC, tone=0.62), "core", "wall", [tag])


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def scale50():
    """🔎 1:50: base tray with collar, screws and terminal box; fan housing with inlet radius; sleeve with outer collar; EPP jacket; honeycomb of 37 channels; three struts; five reversible blades on the external-rotor can."""
    lip = k.arc(-0.114, 0.094, 0.006, 180, 270, 6)
    body = [
        square("plate", k.Z_FLOOR, k.Z_WALL, k.circle(k.HOUSING_OUT)),
        square("plate-rim", k.Z_FACE, k.Z_FLOOR, k.rounded_square(k.PLATE / 2 - k.SHELL, k.PLATE_R - k.SHELL)),
        solid("screw-boss", "revolve", "plate", role="detail", tone=0.8, profile=[k.pt(p) for p in ((k.Z_FLOOR - 0.0075, 0), (k.Z_FLOOR - 0.0075, k.BOSS_R), (k.Z_FLOOR, k.BOSS_R), (k.Z_FLOOR, 0))], axis=AXIS, origin=[k.SCREW, k.SCREW, 0], repeat=QUARTER),
        solid("screw", "ellipsoid", "plate", material=STEEL, role="detail", tone=0.45, radii=[0.004, 0.004, 0.00375], transform={"translation": [k.SCREW, k.SCREW, k.Z_FLOOR - 0.0075]}, repeat=QUARTER),
        solid("terminal", "box", "plate", role="detail", tone=0.8, size=[0.06, 0.02, 0.01], cornerRadius=0.002, transform={"translation": [0, -0.1125, -0.1125]}),
        rev("ring", "ring", [(k.Z_STAGE, k.THROAT), (k.Z_STAGE, k.HOUSING_OUT), (k.Z_FLOOR, k.HOUSING_OUT), (k.Z_FLOOR, k.SLEEVE_OUT), (k.Z_FACE, k.SLEEVE_OUT)] + lip),
        tube("sleeve", k.SLEEVE_OUT, k.SLEEVE_IN, k.Z_WALL, k.Z_END),
        tube("rim", k.RIM_R, k.SLEEVE_OUT, k.Z_RIM, k.Z_END, tone=0.6),
        rev("jacket", "ring", [(k.Z_STAGE, k.THROAT), (k.Z_STAGE, k.HOUSING_OUT), (k.Z_JACKET, k.HOUSING_OUT), (k.Z_JACKET, k.SEAT_R), (k.Z_CORE1, k.SEAT_R), (k.Z_CORE1, k.CORE_R), (k.Z_CORE0, k.CORE_R)], material=EPP, tone=0.12),
        honeycomb(k.CORE_R, k.CELL, 3, 0.0105, k.Z_CORE0, k.Z_CORE1),
        strut(0.0715, 0.008, 0.012, 0.05375, k.Z_STRUT, 0.002),
        rev("stator", "ring", [(-0.082, 0), (-0.082, 0.02), (-0.05, 0.02), (-0.05, 0)], loop=False, tone=0.6),
    ] + [blade(i) for i in range(1, 6)] + [
        rev("hub", "rotor", [(k.Z_HUB0, 0), (k.Z_HUB0, 0.019), (-0.1022, 0.0235), (-0.1005, 0.0265), (-0.098, k.HUB_R), (k.Z_HUB1, k.HUB_R), (k.Z_HUB1, 0)], loop=False, material=STEEL, tone=0.55),
        solid("hub-cap", "ellipsoid", "rotor", material=STEEL, role="detail", tone=0.9, radii=[0.01, 0.01, 0.004], transform={"translation": [0, 0, k.Z_HUB0]}),
    ]
    thermal = [ref(f"th-{s}", s, region="casing", layer="shell") for s in ("plate", "plate-rim", "screw-boss", "terminal", "ring", "sleeve", "rim", "strut", "stator")]
    thermal += [ref("th-core", "core", region="core", layer="wall")]
    thermal += [ref(f"th-blade-{i}", f"blade-{i}", region="blades", layer="blade") for i in range(1, 6)]
    thermal += [ref("th-hub", "hub", region="hub", layer="can"), ref("th-hub-cap", "hub-cap", region="hub", layer="can")]
    thermal += [outlet("th-supply-in", "supply-in", k.Z_CORE0 - 0.001, k.Z_CORE0 + 0.001, k.CORE_R), outlet("th-exhaust-out", "exhaust-out", k.Z_CORE1 - 0.001, k.Z_CORE1 + 0.001, k.SEAT_R)]
    structure = [
        ref("member-plate", "plate", member="plate", part="member"),
        ref("member-sleeve", "sleeve", member="ring", part="member"),
        ref("member-ring", "ring", member="ring", part="member"),
        ref("member-strut", "strut", member="ring", part="member"),
        ref("member-stator", "stator", member="ring", part="member"),
        ref("member-jacket", "jacket", member="core", part="member"),
        st(rev("member-core", "core", [(k.Z_CORE0, 0), (k.Z_CORE0, k.CORE_R), (k.Z_CORE1, k.CORE_R), (k.Z_CORE1, 0)], loop=False, material=CERAMIC, role="detail", tone=0.4), "core", "member"),
        ref("member-hub", "hub", member="rotor", part="member"),
        st(rev("member-shaft", "rotor", [(-0.099, 0), (-0.099, 0.004), (-0.054, 0.004), (-0.054, 0)], loop=False, material=STEEL, role="detail", tone=0.25), "rotor", "member"),
        st(rev("joint-plate-ring", "ring", [(-0.105, 0.093), (-0.105, 0.101), (-0.0975, 0.101), (-0.0975, 0.093)], role="detail", tone=0.25), "ring", "joint"),
        st(rev("joint-core-seat", "core", [(k.Z_CORE1 - 0.002, 0.072), (k.Z_CORE1 - 0.002, 0.082), (k.Z_JACKET + 0.001, 0.082), (k.Z_JACKET + 0.001, 0.072)], role="detail", tone=0.25), "core", "joint"),
        st(rev("joint-bearing", "rotor", [(-0.07, 0), (-0.07, 0.009), (-0.058, 0.009), (-0.058, 0)], loop=False, material=STEEL, role="detail", tone=0.25), "rotor", "joint"),
        mass("mass-plate", "plate", 0.0076, -0.111),
        mass("mass-ring", "ring", 0.0107, -0.012),
        mass("mass-core", "core", 0.014, 0.013),
        mass("mass-rotor", "rotor", 0.0056, -0.088),
        pin(k.Z_FLOOR - 0.01, k.Z_WALL + 0.012, 0.008),
        roller(0.012, 0.008, k.Z_WALL - 0.004),
    ]
    decisions = [
        text("Own 1:50 model of a push-pull wall unit with its inner cover and filter taken off, axis w, room at -w: the base of the inner cover is a moulded ABS tray of 0.26 m with a 2.5 mm rim 20 mm high and a floor that is modelled 7.5 mm thick (the 2.5 mm shell with its ribs and the foam gasket to the wall, the printable feature), four 14 mm screw bosses with their screw heads in the corners and the terminal box at the bottom; the fan housing stands in it with a 196 mm collar, a 6 mm inlet radius and a 176 mm throat, and runs into the 196 mm sleeve of 3 mm wall that crosses the wall and ends in the 208 mm collar for the outer hood.",
             "Eigenes 1:50-Modell eines Pendellüfters bei abgenommener Innenblende und ohne Filter, Achse w, Raum bei -w: der Sockel der Innenblende ist eine gespritzte ABS-Wanne von 0,26 m mit einem 2,5 mm starken, 20 mm hohen Rand und einem Boden, der 7,5 mm stark modelliert ist (die Schale von 2,5 mm mit ihren Rippen und der Schaumdichtung zur Wand, das druckbare Merkmal), vier Schraubdomen von 14 mm mit ihren Schraubenköpfen in den Ecken und dem Klemmkasten unten; darin steht das Ventilatorgehäuse mit einem Kragen von 196 mm, einem Einlaufradius von 6 mm und einem Hals von 176 mm und steckt in der Hülse von 196 mm mit 3 mm Wand, die die Wand durchquert und im Kragen von 208 mm für die Außenhaube endet."),
        text("Behind the fan the black EPP jacket narrows with a 45 degree funnel to the ceramic block of 160 mm by 100 mm, which a lip at its outer end holds; the honeycomb is generalised as the drawing shows it, 37 channels of 18 mm with 3.5 mm walls instead of the real 3 mm cells. The reversible impeller has five radial, unswept blades of symmetric lens section (3 to 2 mm thick, chord 34 to 56 mm, pitched 36 to 24 degrees) so that both directions work alike; they are rooted in a 7.6 mm boss inside the 56 mm external-rotor can with its domed front and cap and run 2 mm inside the throat. Three struts of 8 by 12 mm carry the 40 mm stator boss that reaches into the can.",
             "Hinter dem Ventilator verengt sich der schwarze EPP-Mantel mit einem Trichter von 45 Grad auf den Keramikblock von 160 mm mal 100 mm, den eine Lippe am Außenende hält; die Wabe ist so verallgemeinert, wie die Zeichnung sie zeigt: 37 Kanäle von 18 mm mit 3,5 mm Wand statt der echten Zellen von 3 mm. Das umkehrbare Laufrad hat fünf radiale, ungepfeilte Flügel mit symmetrischem Linsenquerschnitt (3 bis 2 mm stark, Sehne 34 bis 56 mm, 36 bis 24 Grad angestellt), damit beide Drehrichtungen gleich arbeiten; sie wurzeln mit einer Verdickung von 7,6 mm im Außenläuferbecher von 56 mm mit gewölbter Front und Kappe und laufen 2 mm innerhalb des Halses. Drei Streben von 8 mal 12 mm tragen den Statorzapfen von 40 mm, der in den Becher greift."),
        text("Thermal: casing = tray, rim, bosses, terminal box, fan housing, sleeve, collar, struts and stator boss by reference, core = the honeycomb, blades and hub by reference, each with its single layer; the EPP jacket and the steel screw heads belong to no region and are left out. The two outlets are 2 mm patches on the faces of the ceramic: supply-in on the room face, exhaust-out on the outer face inside the retaining lip. Structure: tray floor, sleeve, fan housing, struts and stator boss, the jacket and the ceramic block as a plain cylinder, the can on an 8 mm shaft; joints at the tray bore, the seat of the core and the bearing; four masses at their centres; the upper left screw is the pin from its head into the wall plug, the lower right one the roller on the wall face.",
             "Thermik: Gehäuse = Wanne, Rand, Dome, Klemmkasten, Ventilatorgehäuse, Hülse, Kragen, Streben und Statorzapfen als Verweis, Kern = die Wabe, Flügel und Nabe als Verweis, je mit ihrer einen Schicht; der EPP-Mantel und die stählernen Schraubenköpfe gehören zu keiner Region und fehlen. Die zwei Auslässe sind Flächen von 2 mm auf den Stirnseiten der Keramik: Zuluft auf der Raumseite, Abluft auf der Außenseite innerhalb der Haltelippe. Tragwerk: Wannenboden, Hülse, Ventilatorgehäuse, Streben und Statorzapfen, der Mantel und der Keramikblock als glatter Zylinder, der Becher auf einer Welle von 8 mm; Gelenke an der Wannenbohrung, am Sitz des Kerns und am Lager; vier Massen in ihren Schwerpunkten; die Schraube oben links ist das Festlager vom Kopf bis in den Dübel, die unten rechts das Rollenlager auf der Wandfläche."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    """🔎 1:100: a 20 mm plate, one housing tube with the outer collar, the EPP liner, a honeycomb of 19 channels, three 15 mm struts, five generalised blades on a plain hub."""
    body = [
        square("plate", k.Z_FACE, k.Z_WALL, k.circle(k.SLEEVE_OUT, 48)),
        rev("ring", "ring", [(k.Z_FACE, k.THROAT), (k.Z_FACE, k.SLEEVE_OUT), (0.1, k.SLEEVE_OUT), (0.1, k.RIM_R), (k.Z_END, k.RIM_R), (k.Z_END, k.THROAT)]),
        rev("jacket", "ring", [(k.Z_STAGE, k.CORE_R), (k.Z_STAGE, k.THROAT), (k.Z_JACKET, k.THROAT), (k.Z_JACKET, k.CORE_R)], material=EPP, tone=0.12),
        honeycomb(k.CORE_R, k.CELL_100, 2, 0.0145, k.Z_CORE0, k.Z_CORE1),
        strut(0.0885, 0.015, 0.015, 0.04475, -0.0575),
    ] + [blade(i, chord=(0.036, 0.056), pitch=(40.0, 22.0), thick=(0.005, 0.004), boss=0.0152, stations=6, tip=0.085, hub=0.03, root=0.02) for i in range(1, 6)] + [
        rev("hub", "rotor", [(-0.105, 0), (-0.105, 0.024), (-0.099, 0.03), (-0.065, 0.03), (-0.065, 0)], loop=False, material=STEEL, tone=0.55),
    ]
    thermal = [ref(f"th-{s}", s, region="casing", layer="shell") for s in ("plate", "ring", "strut")]
    thermal += [ref("th-core", "core", region="core", layer="wall")]
    thermal += [ref(f"th-blade-{i}", f"blade-{i}", region="blades", layer="blade") for i in range(1, 6)]
    thermal += [ref("th-hub", "hub", region="hub", layer="can")]
    thermal += [outlet("th-supply-in", "supply-in", k.Z_CORE0 - 0.002, k.Z_CORE0 + 0.001, k.CORE_R), outlet("th-exhaust-out", "exhaust-out", k.Z_CORE1 - 0.001, k.Z_CORE1 + 0.002, k.CORE_R)]
    structure = [
        ref("member-plate", "plate", member="plate", part="member"),
        ref("member-ring", "ring", member="ring", part="member"),
        ref("member-strut", "strut", member="ring", part="member"),
        st(rev("member-core", "core", [(k.Z_CORE0, 0), (k.Z_CORE0, k.CORE_R), (k.Z_CORE1, k.CORE_R), (k.Z_CORE1, 0)], loop=False, material=CERAMIC, role="detail", tone=0.4), "core", "member"),
        ref("member-hub", "hub", member="rotor", part="member"),
        st(rev("member-shaft", "rotor", [(-0.1, 0), (-0.1, 0.0075), (-0.05, 0.0075), (-0.05, 0)], loop=False, material=STEEL, role="detail", tone=0.25), "rotor", "member"),
        st(rev("joint-bearing", "rotor", [(-0.066, 0), (-0.066, 0.012), (-0.051, 0.012), (-0.051, 0)], loop=False, material=STEEL, role="detail", tone=0.25), "rotor", "joint"),
        mass("mass-plate", "plate", 0.0076, -0.11),
        mass("mass-ring", "ring", 0.0107, -0.012),
        mass("mass-core", "core", 0.014, 0.013),
        mass("mass-rotor", "rotor", 0.0075, -0.088),
        pin(k.Z_FACE - 0.004, k.Z_WALL + 0.01, 0.015),
        roller(0.015, 0.015, -0.1175),
    ]
    decisions = [
        text("Own 1:100 model against the 1:100 shapes: the tray is one 20 mm plate with the 196 mm bore, fan housing, sleeve and outer collar are one tube with the 176 mm throat and a 20 mm collar of 208 mm, and the EPP jacket is a plain liner without funnel and lip; rim, bosses, screws, terminal box, inlet radius and hub cap are merged away.",
             "Eigenes 1:100-Modell gegen die 1:100-Formen: die Wanne ist eine Platte von 20 mm mit der Bohrung von 196 mm, Ventilatorgehäuse, Hülse und Außenkragen sind ein Rohr mit dem Hals von 176 mm und einem 20 mm tiefen Kragen von 208 mm, der EPP-Mantel ist eine glatte Auskleidung ohne Trichter und Lippe; Rand, Dome, Schrauben, Klemmkasten, Einlaufradius und Nabenkappe gehen darin auf."),
        text("The ceramic block keeps its size and shows 19 channels of 25 mm; the five radial blades keep pitch and twist but are generalised to lens sections of 5 to 4 mm with a 15 mm root boss buried in the plain 60 mm hub; the three struts are 15 mm bars that meet behind the hub and take in the stator boss.",
             "Der Keramikblock behält seine Größe und zeigt 19 Kanäle von 25 mm; die fünf radialen Flügel behalten Anstellung und Verwindung, sind aber auf Linsenquerschnitte von 5 bis 4 mm verallgemeinert, mit einer in der glatten Nabe von 60 mm verborgenen Wurzelverdickung von 15 mm; die drei Streben sind Stäbe von 15 mm, die sich hinter der Nabe treffen und den Statorzapfen aufnehmen."),
        text("Thermal by reference to plate, tube, struts, honeycomb, blades and hub, with the two outlets as 3 mm patches on the faces of the ceramic; the EPP liner belongs to no region. Structure: plate, tube and struts, the ceramic block as a plain cylinder, the hub on a 15 mm shaft with its bearing, four masses and the two corner screws at 15 mm: the pin runs through the plate into the wall, the roller lies half sunk in the plate face.",
             "Thermik als Verweis auf Platte, Rohr, Streben, Wabe, Flügel und Nabe, mit den zwei Auslässen als Flächen von 3 mm auf den Stirnseiten der Keramik; die EPP-Auskleidung gehört zu keiner Region. Tragwerk: Platte, Rohr und Streben, der Keramikblock als glatter Zylinder, die Nabe auf einer Welle von 15 mm mit ihrem Lager, vier Massen und die zwei Eckschrauben mit 15 mm: das Festlager geht durch die Platte in die Wand, das Rollenlager liegt halb versenkt in der Plattenfront."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    """🔎 1:200: plate, one thick tube, the ceramic as a plain disc, three broad radial blades on an 80 mm hub, three 30 mm struts."""
    bore = 0.082
    body = [
        square("plate", k.Z_FACE, k.Z_WALL, k.circle(k.SLEEVE_OUT, 32)),
        rev("ring", "ring", [(k.Z_FACE, bore), (k.Z_FACE, k.SLEEVE_OUT), (k.Z_END, k.SLEEVE_OUT), (k.Z_END, bore)]),
        rev("core", "core", [(-0.04, 0), (-0.04, bore), (0.06, bore), (0.06, 0)], loop=False, material=CERAMIC, tone=0.62),
        solid("strut", "box", "ring", tone=0.6, size=[0.03, 0.166, 0.03], transform={"translation": [0, 0, -0.055]}),
    ] + [solid(f"blade-{i}", "extrude", "rotor", tone=0.82, profile=k.planform(i, 3, 0.036, 0.08, (0.045, 0.085), phi0=math.radians(30)), plane="xy", **{"from": -0.105, "to": -0.075}) for i in range(1, 4)] + [
        rev("hub", "rotor", [(-0.108, 0), (-0.108, 0.04), (-0.07, 0.04), (-0.07, 0)], loop=False, material=STEEL, tone=0.55),
    ]
    thermal = [ref(f"th-{s}", s, region="casing", layer="shell") for s in ("plate", "ring", "strut")]
    thermal += [ref("th-core", "core", region="core", layer="wall")]
    thermal += [ref(f"th-blade-{i}", f"blade-{i}", region="blades", layer="blade") for i in range(1, 4)]
    thermal += [ref("th-hub", "hub", region="hub", layer="can")]
    thermal += [outlet("th-supply-in", "supply-in", -0.043, -0.039, bore), outlet("th-exhaust-out", "exhaust-out", 0.059, 0.063, bore)]
    structure = [
        ref("member-plate", "plate", member="plate", part="member"),
        ref("member-ring", "ring", member="ring", part="member"),
        ref("member-strut", "strut", member="ring", part="member"),
        ref("member-core", "core", member="core", part="member"),
        ref("member-hub", "hub", member="rotor", part="member"),
        mass("mass-unit", "core", 0.0166, -0.009),
        pin(k.Z_FACE - 0.005, k.Z_WALL + 0.005, 0.03),
        roller(0.03, 0.03, -0.105),
    ]
    decisions = [
        text("Own 1:200 model: the plate at its 20 mm with the 196 mm bore and one thick tube of 196 mm with a 164 mm bore that takes in fan housing, sleeve, liner and outer collar; the ceramic is a plain disc of 164 mm by 100 mm without channels.",
             "Eigenes 1:200-Modell: die Platte mit ihren 20 mm und der Bohrung von 196 mm und ein dickes Rohr von 196 mm mit 164 mm Bohrung, in dem Ventilatorgehäuse, Hülse, Auskleidung und Außenkragen aufgehen; die Keramik ist eine glatte Scheibe von 164 mm mal 100 mm ohne Kanäle."),
        text("Five pitched blades cannot be shown at 30 mm: the drawing gives three broad radial blades, modelled as their plan-forms pushed through the 30 mm deep fan stage on an 80 mm hub; the three struts are merged into one upright bridge bar of 30 mm (the printable floor) behind the hub, which takes in the stator: three bars of that width would read as three more blades.",
             "Fünf angestellte Flügel lassen sich mit 30 mm nicht zeigen: die Zeichnung gibt drei breite radiale Flügel, modelliert als ihre Grundrisse durch die 30 mm tiefe Ventilatorstufe auf einer Nabe von 80 mm; die drei Streben sind zu einem stehenden Brückensteg von 30 mm (die druckbare Untergrenze) hinter der Nabe zusammengefasst, in dem der Stator aufgeht: drei Stäbe dieser Breite läsen sich als drei weitere Flügel."),
        text("Thermal by reference with the two outlets as 4 mm patches on the faces of the disc; structure: plate, tube, bridge bar, ceramic disc and hub as members, one mass of the whole unit at its centre and the two corner screws at 30 mm; joints are merged into the members.",
             "Thermik als Verweis mit den zwei Auslässen als Flächen von 4 mm auf den Stirnseiten der Scheibe; Tragwerk: Platte, Rohr, Brückensteg, Keramikscheibe und Nabe als Stäbe, eine Masse des ganzen Geräts in seinem Schwerpunkt und die zwei Eckschrauben mit 30 mm; Gelenke gehen in den Stäben auf."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    """💾 Write the source."""
    volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": {"status": "unavailable", "denominator": 500}}
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("written", SOURCE, {s: (len(v.get("body", [])), len(v.get("thermal", [])), len(v.get("structure", []))) for s, v in volumes.items()})


if __name__ == "__main__":
    main()
