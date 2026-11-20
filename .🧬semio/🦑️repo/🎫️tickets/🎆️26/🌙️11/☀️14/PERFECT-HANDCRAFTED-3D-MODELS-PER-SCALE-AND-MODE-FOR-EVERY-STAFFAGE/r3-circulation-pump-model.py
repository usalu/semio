"""🧊 Round 3, appliances/circulation-pump: every solid of every authored scale and list, stated by hand; writes `volumes/appliances/circulation-pump.json`.

Usage: python3 r3-circulation-pump-model.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-circulation-pump-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

SOURCE = os.path.join(HERE, "volumes", "appliances", "circulation-pump.json")
QUARTER = {"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": round(math.pi / 2, 7)}
IRON, ALU, PA, GLASS, CU, BRASS, STEEL = "cast-iron", "aluminium", "polyamide", "window-glass", "copper", "brass", "steel"


def solid(id, kind, bone, material, role="body", tone=0.5, **fields):
    """🧩 One solid with the common members first."""
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def rev(id, bone, material, profile, origin, axis, loop=False, **kw):
    """🌀 A revolve of (axial, radial) points; without `loop` the profile is closed through the axis, with it the first point is repeated so the ring section is revolved."""
    pts = [k.pt(p) for p in profile]
    pts = pts + [pts[0]] if loop else [[pts[0][0], 0.0]] + pts + [[pts[-1][0], 0.0]]
    return solid(id, "revolve", bone, material, profile=pts, axis=list(axis), origin=k.pt(origin), **kw)


def shaft(id, bone, material, profile, **kw):
    """🎯 A revolve about the motor shaft (w through the bone origin of the motor and rotor bones)."""
    return rev(id, bone, material, profile, (0, 0, 0), (0, 0, 1), **kw)


def pipe(id, side, material, profile, **kw):
    """🚿 A revolve about the pipe axis, running outwards from the shaft plane to the left (side -1) or right (+1)."""
    return rev(id, "housing", material, profile, (k.XH, 0, k.WP), (side, 0, 0), **kw)


def volute(sections, tone=0.42):
    """🐚 The cast volute: circular sections about a centre that is offset from the shaft (the spiral widens towards the outlet and the bottom), closing in a rounded back."""
    return solid("casing", "loft", "housing", IRON, tone=tone, sections=[{"at": k.pt((k.XH + k.CX, k.CY, w)), "rx": r, "ry": r, "exponent": 2} for w, r in sections], capStart=True, capEnd=True)


def plate(id, bone, material, w0, w1, tone, x0=0.0):
    """🔲 A rounded square flange plate about the shaft."""
    return solid(id, "extrude", bone, material, tone=tone, profile=k.rounded_square(k.FL, k.FL_R, cx=x0), plane="xy", **{"from": w0, "to": w1})


def nut(id, side, x0, x1, taper=None, tone=0.5):
    """🔩 A slice of the hexagonal union nut between x0 and x1 from the shaft plane; `taper` chamfers it."""
    a, b = sorted((side * x0, side * x1))
    s = solid(id, "extrude", "housing", BRASS, tone=tone, profile=k.hexagon(), plane="yz", transform={"translation": [0, 0, k.WP]}, **{"from": k.r5(k.XH + a), "to": k.r5(k.XH + b)})
    if taper:
        s["taperTo"] = list(taper if side > 0 else taper[::-1])
    return s


def box(id, y0, y1, dx, dw, tone, corner, material=PA, wc=k.TW, **kw):
    """📦 A box of the terminal bone between the heights y0 and y1 above the shaft, dx wide along the pipe and dw long along the shaft about wc."""
    return solid(id, "box", "terminal", material, tone=tone, size=[k.r5(y1 - y0), dx, dw], cornerRadius=corner, transform={"translation": k.pt(((y0 + y1) / 2 - k.TY, 0, wc))}, **kw)


def ref(id, of, **kw):
    """🔗 A reference to a body solid of the same scale."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def th(s, region, layer, tags=None):
    """🌡️ A thermal solid of its own."""
    s = dict(s)
    s.update(region=region, layer=layer)
    if tags:
        s["tags"] = tags
    return s


def st(s, member, part, anchor=None):
    """🏗️ A structural solid of its own."""
    s = dict(s)
    s.update(member=member, part=part)
    if anchor:
        s["tags"] = [anchor]
        s["anchor"] = anchor
    return s


def refs(names, prefix, **kw):
    """🔗 References to several body solids."""
    return [ref(f"{prefix}-{n}", n, **kw) for n in names]


def plume(y, dx=0.054, dw=0.060, wc=k.TW):
    """♨️ The outlet patch of the motor plume: the top of the terminal box lid, where the warm air of the motor head leaves upwards."""
    return th(solid("th-motor-plume", "box", "terminal", PA, tone=0.8, size=[0.001, dx, dw], cornerRadius=0.0004, transform={"translation": k.pt((y - k.TY + 0.0003, 0, wc))}), "motor", "head", ["motor-plume"])


def stub_cut(id, side, x0):
    """✂️ The thermal surface of a copper tail: its outer wall; the end is a cut through the continuing pipe."""
    s = pipe(id, side, CU, [(x0, k.R_STUB), (k.X_END, k.R_STUB)], tone=0.7)
    s["sectionFaces"] = ["+u" if side > 0 else "-u"]
    return th(s, "casing", "shell")


def supports(d, length, inner=k.R_STUB):
    """📌 The pin (a clamp collar around the left tail) and the roller (a roller pin across the pipe axis at the right tail)."""
    pin = pipe("support-pipe-left", -1, IRON, [(k.XH - length / 2, inner), (k.XH - length / 2, inner + d / 2), (k.XH + length / 2, inner + d / 2), (k.XH + length / 2, inner)], loop=True, role="detail", tone=0.25)
    roller = rev("support-pipe-right", "housing", IRON, [(-0.02, min(d, 0.015) / 2 if inner == k.R_STUB else d / 2), (0.02, min(d, 0.015) / 2 if inner == k.R_STUB else d / 2)], (2 * k.XH, 0, k.WP), (0, 0, 1), role="detail", tone=0.25)
    return [st(pin, "housing", "support", "pipe-left"), st(roller, "housing", "support", "pipe-right")]


def mass(id, bone, at, r):
    """⚫ A concentrated mass at its centre of mass."""
    return st(solid(id, "ellipsoid", bone, IRON, role="detail", tone=0.15, radii=[r, r, r], transform={"translation": k.pt(at)}), bone, "mass")


def rotor_assembly(d, pack=True):
    """🧲 The wet rotor: shaft, rotor pack in the can and the impeller in the volute."""
    r = d / 2
    profile = [(-0.056, r), (-0.050, r), (-0.050, 0.017), (-0.008, 0.017), (-0.008, r), (0.020, r), (0.020, 0.031), (0.030, 0.031), (0.034, 0.012)] if pack else [(-0.056, r), (0.020, r), (0.020, 0.031), (0.034, 0.031)]
    return st(shaft("member-rotor", "rotor", IRON, profile, role="detail", tone=0.25), "rotor", "member")


def ring_plate(id, bone, material, w0, w1, hole, x0=0.0):
    """⭕ A flange plate as it carries: the rounded square with the bore of the rotor chamber."""
    s = plate(id, bone, material, w0, w1, 0.25, x0)
    s["holes"] = [[k.pt((x0 + hole * math.cos(-2 * math.pi * i / 32), hole * math.sin(-2 * math.pi * i / 32))) for i in range(32)]]
    s["role"] = "detail"
    return s


def shell(id, bone, material, profile, x0=0.0, y0=0.0, loop=False):
    """🥫 A load-bearing shell about the shaft direction: a cup (closed through the axis at its back) or, with `loop`, an open tube."""
    return rev(id, bone, material, profile, (x0, y0, 0), (0, 0, 1), loop=loop, role="detail", tone=0.25)


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def scale50():
    """🔎 1:50: the cast housing with its flange, port necks and threads, the unions with hexagon nuts and copper tails, the flanged motor with four screws, bezel, sight glass and indicator, the terminal box with saddle, lid, gland and lamp."""
    body = [
        volute(k.VOLUTE_50),
        plate("flange", "housing", IRON, k.W_HOUSING, k.W_FLANGE, 0.38, k.XH),
        pipe("neck-l", -1, IRON, k.NECK_50, tone=0.42),
        pipe("neck-r", 1, IRON, k.NECK_50, tone=0.42),
    ]
    for name, side in (("l", -1), ("r", 1)):
        body += [
            nut(f"union-{name}", side, *k.X_NUT),
            pipe(f"stub-{name}", side, CU, [(0.091, k.R_BORE), (0.091, k.R_STUB), (k.X_END, k.R_STUB), (k.X_END, k.R_BORE)], loop=True, tone=0.7),
        ]
    body += [
        plate("motor-flange", "motor", ALU, k.W_MOTOR, k.W_HOUSING, 0.55),
        shaft("motor-head", "motor", ALU, k.STATOR_50, tone=0.62),
        rev("screw", "motor", STEEL, [(-0.006, 0.005), (0.001, 0.005)], (k.SCREW, k.SCREW, 0), (0, 0, 1), role="detail", tone=0.3, repeat=QUARTER),
        shaft("bezel", "motor", ALU, [(k.W_FRONT, k.R_PANE), (k.W_FRONT, 0.0295), (-0.066, k.R_BEZEL), (-0.058, k.R_BEZEL), (-0.058, k.R_PANE)], loop=True, role="detail", tone=0.5),
        shaft("window", "motor", GLASS, [(-0.0665, k.R_PANE), (-0.0645, k.R_PANE)], role="glazing", tone=0.18),
        shaft("dial", "motor", PA, [(-0.0606, k.R_PANE), (-0.0598, k.R_PANE)], role="texture", tone=0.25, texture=True),
        shaft("hub", "rotor", PA, [(-0.0640, 0.0055), (-0.0605, 0.0055)], role="detail", tone=0.35, texture=True),
        solid("pointer", "extrude", "rotor", PA, role="detail", tone=0.95, profile=[[0, -0.006], [0.024, 0], [0, 0.006]], plane="xy", texture=True, **{"from": -0.0630, "to": -0.0612}),
        solid("tail", "extrude", "rotor", PA, role="detail", tone=0.7, profile=[[0, 0.0045], [-0.011, 0], [0, -0.0045]], plane="xy", texture=True, **{"from": -0.0630, "to": -0.0612}),
        box("terminal-neck", 0.028, 0.044, 0.056, 0.056, 0.7, 0.002),
        box("terminal-box", 0.042, 0.062, 0.060, 0.066, 0.78, 0.004),
        box("terminal-lid", 0.061, k.BOX_TOP, 0.062, 0.068, 0.82, 0.004),
        rev("gland", "terminal", PA, k.GLAND_50, (0.052 - k.TY, 0.030, k.TW), (0, 1, 0), role="detail", tone=0.4),
        solid("led", "ellipsoid", "terminal", PA, role="detail", tone=0.95, radii=[0.0045, 0.0045, 0.00375], transform={"translation": k.pt((0.068 - k.TY, 0, k.TW - 0.034))}),
    ]
    thermal = refs(["casing", "flange", "neck-l", "neck-r", "union-l", "union-r"], "th", region="casing", layer="shell")
    thermal += [stub_cut("th-stub-l", -1, 0.091), stub_cut("th-stub-r", 1, 0.091)]
    thermal += refs(["motor-flange", "motor-head", "screw", "bezel"], "th", region="motor", layer="head")
    thermal += refs(["terminal-neck", "terminal-box", "terminal-lid", "gland", "led"], "th", region="terminal-box", layer="box")
    thermal += [ref("th-rotor-window", "window", region="rotor-window", layer="pane", sectionFaces=["+w"]), plume(k.BOX_TOP)]
    port = [(0.030, k.R_BORE), (0.030, 0.0215), (0.064, 0.0195), (0.090, 0.0195), (0.090, k.R_BORE)]
    structure = [
        st(shell("member-volute", "housing", IRON, [(0.062, 0.043), (0.012, 0.043), (0.012, 0.0462), (0.014, 0.048), (0.057, 0.048), (0.061, 0.0468), (0.064, 0.044), (0.066, 0.0395), (0.067, 0.033)], k.XH + k.CX, k.CY), "housing", "member"),
        st(pipe("member-port-l", -1, IRON, port, loop=True, role="detail", tone=0.25), "housing", "member"),
        st(pipe("member-port-r", 1, IRON, port, loop=True, role="detail", tone=0.25), "housing", "member"),
        st(ring_plate("member-flange", "housing", IRON, k.W_HOUSING, k.W_FLANGE, 0.039, k.XH), "housing", "member"),
        ref("joint-union-l", "union-l", member="housing", part="joint"),
        ref("joint-union-r", "union-r", member="housing", part="joint"),
        st(ring_plate("member-motor-flange", "motor", ALU, k.W_MOTOR, k.W_HOUSING, 0.039), "motor", "member"),
        st(shell("member-stator", "motor", ALU, [(-0.056, 0.039), (-0.056, 0.042), (0.004, 0.042), (0.004, 0.039)], loop=True), "motor", "member"),
    ]
    structure += [
        st(rev("joint-motor", "motor", STEEL, [(-0.006, 0.003), (0.014, 0.003)], (k.SCREW, k.SCREW, 0), (0, 0, 1), role="detail", tone=0.25, repeat=QUARTER), "motor", "joint"),
        st(box("member-terminal", 0.042, 0.050, 0.060, 0.066, 0.25, 0.002, role="detail"), "terminal", "member"),
        ref("joint-terminal", "terminal-neck", member="terminal", part="joint"),
        rotor_assembly(0.008),
        st(shaft("joint-rotor-front", "rotor", IRON, [(-0.0595, 0.009), (-0.052, 0.009)], role="detail", tone=0.25), "rotor", "joint"),
        st(shaft("joint-rotor-rear", "rotor", IRON, [(0.003, 0.009), (0.0105, 0.009)], role="detail", tone=0.25), "rotor", "joint"),
        mass("mass-housing", "housing", (k.XH, 0, 0.036), 0.018),
        mass("mass-motor", "motor", (0, 0, -0.028), 0.0152),
    ] + supports(0.015, 0.008)
    decisions = [
        text("The pump of the UPS 25-60 180 class is built as it is made: a cast-iron housing whose volute is a drum about a centre 3 mm below and 1.5 mm beside the shaft (the spiral widens towards the outlet), closed by a rounded back 32 mm behind the pipe axis, with a square flange, two tapering port necks and the G 1 1/2 thread ends 180 mm apart; on each thread sits a brass union nut of 52 mm across flats with chamfered ends and a hollow 28 mm copper tail.",
             "Die Pumpe der Klasse UPS 25-60 180 ist so gebaut, wie sie gefertigt wird: ein Graugussgehäuse, dessen Spirale eine Trommel um einen Mittelpunkt 3 mm unter und 1,5 mm neben der Welle ist (die Spirale weitet sich zum Austritt), hinten 32 mm hinter der Rohrachse gerundet geschlossen, mit quadratischem Flansch, zwei verjüngten Stutzenhälsen und den Gewindeenden G 1 1/2 im Abstand von 180 mm; auf jedem Gewinde sitzt eine Messing-Überwurfmutter mit Schlüsselweite 52 mm und gefasten Enden und ein hohles Kupfer-Einlegeteil von 28 mm."),
        text("The motor is bolted on with four M6 socket screws through its square aluminium flange; the stator can of 84 mm ends in a rounded front with the bezel, the 2 mm sight glass set 0.5 mm back and, behind it on the rotor bone, the dial, hub, pointer and tail of the indicator. The terminal box stands on a saddle on the can: lower shell, wider lid, a cable gland with dome nut on the left wall and the lamp on the lid front.",
             "Der Motor ist mit vier M6-Innensechskantschrauben durch seinen quadratischen Aluminiumflansch angeschraubt; der Statortopf von 84 mm endet in einer gerundeten Front mit der Blende, dem 0,5 mm zurückgesetzten 2-mm-Schauglas und dahinter, am Rotorknochen, Zifferblatt, Nabe, Zeiger und Gegenstück der Anzeige. Der Klemmenkasten steht auf einem Sattel auf dem Topf: Unterschale, breiterer Deckel, eine Kabelverschraubung mit Hutmutter an der linken Wand und die Leuchte an der Deckelfront."),
        text("Thermal: every region is its body surface in its single layer; the copper tails count with their outer wall only, their ends are cuts through the continuing pipe; the plume outlet is a patch on the lid top. Structure: the load path is the pipe run through the casting, the two port tubes with their bore and the 5 mm volute shell with its flange, with the unions as joints; the 3 mm stator shell hangs on its flange and four screws, the terminal box is its floor tray on the saddle, and in the open shells the rotor shows as the real shaft with rotor pack and impeller in two bearings; the left tail is clamped (pin), the right one runs on a roller pin.",
             "Thermik: jede Region ist ihre Körperoberfläche in ihrer einen Schicht; die Kupfer-Einlegeteile zählen nur mit der Außenwand, ihre Enden sind Schnitte durch das weiterlaufende Rohr; der Auslass der Wärmefahne ist ein Feld auf dem Deckel. Tragwerk: der Lastpfad ist der Rohrstrang durch das Gussteil, die zwei Stutzenrohre mit ihrer Bohrung und die 5-mm-Spiralschale mit ihrem Flansch, mit den Verschraubungen als Gelenken; die 3-mm-Statorschale hängt an ihrem Flansch und vier Schrauben, der Klemmenkasten ist seine Bodenwanne auf dem Sattel, und in den offenen Schalen zeigt sich der Rotor als echte Welle mit Rotorpaket und Laufrad in zwei Lagern; das linke Einlegeteil ist geklemmt (Festlager), das rechte läuft auf einem Rollenbolzen."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    """🔍 1:100: volute drum with a chamfered back, one flange plate, plain necks, hexagon nuts, solid tails, the motor can with a chamfer and the sight-glass cap, one terminal block with the gland."""
    body = [
        volute(k.VOLUTE_100),
        plate("flange", "motor", ALU, k.W_MOTOR, k.W_FLANGE, 0.55),
        pipe("neck-l", -1, IRON, k.NECK_100, tone=0.42),
        pipe("neck-r", 1, IRON, k.NECK_100, tone=0.42),
        nut("union-l", -1, *k.X_NUT),
        nut("union-r", 1, *k.X_NUT),
        pipe("stub-l", -1, CU, [(0.090, k.R_STUB), (k.X_END, k.R_STUB)], tone=0.7),
        pipe("stub-r", 1, CU, [(0.090, k.R_STUB), (k.X_END, k.R_STUB)], tone=0.7),
        shaft("motor-head", "motor", ALU, k.STATOR_100, tone=0.62),
        shaft("window", "motor", GLASS, [(k.W_FRONT, k.R_PANE), (-0.065, k.R_PANE)], role="glazing", tone=0.18),
        box("terminal-box", 0.028, k.BOX_TOP, 0.062, 0.064, 0.78, 0.004, wc=k.TW + 0.002),
        rev("gland", "terminal", PA, [(-0.002, 0.010), (0.017, 0.010)], (0.052 - k.TY, 0.030, k.TW), (0, 1, 0), tone=0.4),
    ]
    for s in body[6:8]:
        s["sectionFaces"] = ["-u" if s["id"].endswith("l") else "+u"]
    thermal = refs(["casing", "neck-l", "neck-r", "union-l", "union-r"], "th", region="casing", layer="shell")
    thermal += [ref("th-stub-l", "stub-l", region="casing", layer="shell", sectionFaces=["-u"]), ref("th-stub-r", "stub-r", region="casing", layer="shell", sectionFaces=["+u"])]
    thermal += refs(["flange", "motor-head"], "th", region="motor", layer="head")
    thermal += refs(["terminal-box", "gland"], "th", region="terminal-box", layer="box")
    thermal += [ref("th-rotor-window", "window", region="rotor-window", layer="pane", sectionFaces=["+w"]), plume(k.BOX_TOP, 0.050, 0.052, k.TW + 0.002)]
    structure = [st(shell("member-volute", "housing", IRON, [(0.052, 0.033), (0.015, 0.033), (0.015, 0.048), (0.060, 0.048), (0.067, 0.041)], k.XH + k.CX, k.CY), "housing", "member")]
    structure += refs(["neck-l", "neck-r"], "member", member="housing", part="member")
    structure += [
        ref("joint-union-l", "union-l", member="housing", part="joint"),
        ref("joint-union-r", "union-r", member="housing", part="joint"),
        st(shell("member-motor", "motor", ALU, [(-0.061, 0.027), (-0.061, 0.042), (0.0, 0.042), (0.0, 0.027)], loop=True), "motor", "member"),
        st(ring_plate("joint-motor", "motor", ALU, k.W_MOTOR, k.W_FLANGE, 0.033), "motor", "joint"),
        st(box("member-terminal", 0.040, 0.055, 0.056, 0.060, 0.25, 0.0, role="detail", wc=k.TW + 0.002), "terminal", "member"),
        rotor_assembly(0.015),
        mass("mass-housing", "housing", (k.XH, 0, 0.036), 0.018),
        mass("mass-motor", "motor", (0, 0, -0.028), 0.0152),
    ] + supports(0.015, 0.015)
    decisions = [
        text("Authored anew for a 15 mm feature: the volute is the same offset drum with one chamfer for the rounded back; the two bolted flanges are one 15 mm plate, the screws are left out; each port neck is a plain 41 mm cylinder, each union one hexagon prism without chamfers, each copper tail a solid 28 mm cylinder.",
             "Für ein 15-mm-Merkmal neu gebaut: die Spirale ist dieselbe versetzte Trommel mit einer Fase für den gerundeten Rücken; die zwei verschraubten Flansche sind eine 15-mm-Platte, die Schrauben entfallen; jeder Stutzenhals ist ein glatter Zylinder von 41 mm, jede Verschraubung ein Sechskantprisma ohne Fasen, jedes Kupfer-Einlegeteil ein voller Zylinder von 28 mm."),
        text("The motor can keeps its 84 mm and a 4 mm chamfer at the front; bezel and sight glass are one dark cap of 57 mm, the indicator behind it is not modelled, as it is not drawn. Saddle, shell and lid of the terminal box are one rounded block; the gland is a plain 20 mm cylinder.",
             "Der Motortopf behält seine 84 mm und vorn eine 4-mm-Fase; Blende und Schauglas sind eine dunkle Kappe von 57 mm, die Anzeige dahinter ist nicht modelliert, weil sie nicht gezeichnet ist. Sattel, Schale und Deckel des Klemmenkastens sind ein gerundeter Block; die Kabelverschraubung ist ein glatter Zylinder von 20 mm."),
        text("Thermal: the merged solids carry their regions, the tails count without their cut ends, the plume outlet stays the patch on the box top. Structure: the volute as a cup with a 15 mm wall and the necks as the pipe-run member with the unions as joints, the stator as a 15 mm tube on the bored flange plate as its joint, the base of the terminal box, the rotor as a 15 mm shaft with rotor pack and impeller, the masses of housing and motor, a clamp collar and a roller pin of 15 mm.",
             "Thermik: die zusammengefassten Körper tragen ihre Regionen, die Einlegeteile zählen ohne ihre Schnittenden, der Auslass der Wärmefahne bleibt das Feld auf dem Kasten. Tragwerk: die Spirale als Topf mit 15 mm Wand und die Hälse als Stab des Rohrstrangs mit den Verschraubungen als Gelenken, der Stator als 15-mm-Rohr an der gebohrten Flanschplatte als seinem Gelenk, der Sockel des Klemmenkastens, der Rotor als 15-mm-Welle mit Rotorpaket und Laufrad, die Massen von Gehäuse und Motor, eine Klemmschelle und ein Rollenbolzen von 15 mm."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    """🔭 1:200: the volute drum, the motor drum, one pipe bar through the housing and the terminal block."""
    body = [
        volute(k.VOLUTE_200),
        shaft("motor-head", "motor", ALU, [(k.W_FRONT, k.R_STATOR), (k.W_HOUSING, k.R_STATOR)], tone=0.62),
        rev("pipe", "housing", BRASS, [(0, 0.022), (2 * k.X_END, 0.022)], (k.XH - k.X_END, 0, k.WP), (1, 0, 0), tone=0.6, sectionFaces=["-u", "+u"]),
        box("terminal-box", 0.030, k.BOX_TOP, 0.062, 0.064, 0.78, 0.0, wc=k.TW + 0.002),
    ]
    thermal = [
        ref("th-casing", "casing", region="casing", layer="shell"),
        ref("th-pipe", "pipe", region="casing", layer="shell", sectionFaces=["-u", "+u"]),
        ref("th-motor-head", "motor-head", region="motor", layer="head"),
        ref("th-terminal-box", "terminal-box", region="terminal-box", layer="box"),
        th(shaft("th-rotor-window", "motor", GLASS, [(k.W_FRONT - 0.0005, k.R_PANE), (k.W_FRONT + 0.0015, k.R_PANE)], role="glazing", tone=0.18, sectionFaces=["+w"]), "rotor-window", "pane"),
        plume(k.BOX_TOP, 0.062, 0.064, k.TW + 0.002),
    ]
    structure = [
        st(rev("member-housing", "housing", IRON, [(0, 0.015), (2 * k.XH, 0.015)], (0, 0, k.WP), (1, 0, 0), role="detail", tone=0.25), "housing", "member"),
        st(shaft("member-motor", "motor", ALU, [(-0.060, 0.030), (-0.060, 0.042), (0.008, 0.042), (0.008, 0.030)], loop=True, role="detail", tone=0.25), "motor", "member"),
        st(shaft("member-rotor", "rotor", IRON, [(-0.055, 0.015), (0.034, 0.015)], role="detail", tone=0.25), "rotor", "member"),
        st(box("member-terminal", 0.040, 0.070, 0.040, 0.040, 0.25, 0.0, role="detail"), "terminal", "member"),
        mass("mass-pump", "housing", (k.XH, 0.005, 0.0067), 0.022),
    ] + supports(0.03, 0.03, 0.015)
    decisions = [
        text("Authored anew for a 30 mm feature as four masses: the offset volute drum with the flange in it and one chamfer at the back, the motor drum of 84 mm up to the sight glass, one pipe bar of 44 mm through the housing for necks, unions and tails (its ends are cuts through the pipe), and the terminal block.",
             "Für ein 30-mm-Merkmal neu aus vier Massen gebaut: die versetzte Spiraltrommel samt Flansch mit einer Fase am Rücken, die Motortrommel von 84 mm bis zum Schauglas, ein Rohrbalken von 44 mm durch das Gehäuse für Hälse, Verschraubungen und Einlegeteile (seine Enden sind Schnitte durch das Rohr) und der Klemmenblock."),
        text("Thermal: drum, bar, motor and block carry their regions; the sight glass is a flush disc on the motor front because the body has merged it; the plume outlet is the patch on the block.",
             "Thermik: Trommel, Balken, Motor und Block tragen ihre Regionen; das Schauglas ist eine bündige Scheibe an der Motorfront, weil der Körper es zusammengefasst hat; der Auslass der Wärmefahne ist das Feld auf dem Block."),
        text("Structure: a 30 mm beam along the pipe axis between the supports, the motor as the stator shell cantilevering from it, the rotor shaft inside, a post for the terminal box, one mass of 2.8 kg at the common centre of mass, clamp collar and roller of 30 mm. The model is minimal.",
             "Tragwerk: ein 30-mm-Balken auf der Rohrachse zwischen den Lagern, der Motor als daran auskragende Statorschale, die Rotorwelle darin, ein Pfosten für den Klemmenkasten, eine Masse von 2,8 kg im gemeinsamen Schwerpunkt, Klemmschelle und Rolle von 30 mm. Das Modell ist minimal."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": {"status": "unavailable", "denominator": 500}}
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    for key in ("50", "100", "200"):
        v = volumes[key]
        print(key, *(f"{name} {len(v[name])}" for name in ("body", "thermal", "structure")))


if __name__ == "__main__":
    main()
