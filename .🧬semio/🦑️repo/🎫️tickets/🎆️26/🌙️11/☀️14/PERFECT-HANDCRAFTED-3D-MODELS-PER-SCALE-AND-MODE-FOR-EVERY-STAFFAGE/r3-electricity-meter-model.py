"""⚡ Round 3, appliances/electricity-meter: writes the handcrafted volumes source `volumes/appliances/electricity-meter.json`.

A single-phase Ferraris meter of the DIN 43857-1 kind, 132 x 212 x 118 mm: base with terminal block, hollow meter cover with window, terminal cover, hanging lug, and behind the window the dial plate with the roller register, the horizontal rotor disc on its upright spindle, the brake magnet and the voltage and current electromagnets on their frame. Every solid is stated by hand in world millimetres and placed in the bone frames by the kit.

Usage: python3 r3-electricity-meter-model.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-electricity-meter-kit.py"))
kit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(kit)
rr, circle, uv, vw, wu, solid, box, plate, turned_z, r6 = kit.rr, kit.circle, kit.uv, kit.vw, kit.wu, kit.solid, kit.box, kit.plate, kit.turned_z, kit.r6
Y0, YD, ZD, ZB = kit.Y0, kit.YD, kit.ZD, kit.ZB

ABS, PC, ALU, STEEL, COPPER = "abs-plastic", "polycarbonate", "aluminium-disc", "steel-zinc", "copper-winding"
CY = 0.024
T_COVER, T_BASE, T_TERMINAL, T_BLOCK, T_STEEL, T_CORE, T_COPPER, T_DIAL, T_DARK = 0.88, 0.7, 0.55, 0.3, 0.45, 0.35, 0.6, 0.93, 0.15


def turned_y(ident, x, z, profile, role, tone, material, **more):
    return solid(ident, "revolve", role, tone, material, profile=[[r6(y - Y0), r6(r)] for (y, r) in profile], axis=[1, 0, 0], origin=[0, r6(-x), r6(z - ZB)], **more)


def turned_x(ident, y, z, profile, role, tone, material, **more):
    return solid(ident, "revolve", role, tone, material, profile=[[r6(x), r6(r)] for (x, r) in profile], axis=[0, -1, 0], origin=[r6(y - Y0), 0, r6(z - ZB)], **more)


def rotor(ident, profile, role, tone, material, **more):
    return solid(ident, "revolve", role, tone, material, bone="disc", profile=[[r6(y - YD), r6(r)] for (y, r) in profile], axis=[1, 0, 0], origin=[0, 0, 0], **more)


def channel(ident, x, z_front, z_back, wall, radius, n, y0, y1, tone):
    outer = [(x, z_back)] + [(x - radius + radius * c, z_front + radius - radius * s) for (c, s) in arc(n)] + [(-x + radius - radius * s, z_front + radius - radius * c) for (c, s) in arc(n)] + [(-x, z_back)]
    xi, zi, ri = x - wall, z_front + wall, max(radius - wall, 0.0)
    inner = [(-xi, z_back)] + [(-xi + ri - ri * c, zi + ri - ri * s) for (c, s) in arc(n)] + [(xi - ri + ri * s, zi + ri - ri * c) for (c, s) in arc(n)] + [(xi, z_back)]
    return solid(ident, "extrude", "body", tone, ABS, profile=vw(outer + inner), plane="yz", **{"from": r6(y0 - Y0), "to": r6(y1 - Y0)})


def lid(ident, x, z_front, z_back, radius, n, y0, y1, tone):
    outer = [(x, z_back)] + [(x - radius + radius * c, z_front + radius - radius * s) for (c, s) in arc(n)] + [(-x + radius - radius * s, z_front + radius - radius * c) for (c, s) in arc(n)] + [(-x, z_back)]
    return solid(ident, "extrude", "body", tone, ABS, profile=vw(outer), plane="yz", **{"from": r6(y0 - Y0), "to": r6(y1 - Y0)})


def arc(n):
    import math
    return [(math.cos(math.radians(90 * k / n)), math.sin(math.radians(90 * k / n))) for k in range(n + 1)]


def lug_outline(n):
    import math
    return [(0.008, 0.086), (0.008, 0.098)] + [(0.008 * math.cos(math.radians(180 * k / n)), 0.098 + 0.008 * math.sin(math.radians(180 * k / n))) for k in range(1, n)] + [(-0.008, 0.098), (-0.008, 0.086)]


def scale_50():
    body = [
        plate("base", rr(0, -0.004, 0.066, 0.098, 0.010), 0.056, 0.059, "body", T_BASE),
        plate("base-rim", rr(0, -0.004, 0.066, 0.098, 0.010), 0.033, 0.056, "body", T_BASE, holes=[rr(0, 0.024, 0.063, 0.067, 0.007)]),
        plate("lug", lug_outline(8), 0.0568, 0.0589, "detail", T_STEEL, STEEL, holes=[circle(0, 0.100, 0.00275, 12)]),
        solid("terminal-block-foot", "extrude", "body", T_BLOCK, ABS, profile=vw([(0.046, -0.004), (0.046, 0.033), (-0.046, 0.033), (-0.046, -0.004)]), holes=[vw(circle(x, 0.012, 0.0036, 12)) for x in (-0.030, -0.010, 0.010, 0.030)], plane="yz", **{"from": r6(-0.102 - Y0), "to": r6(-0.078 - Y0)}),
        box("terminal-block", -0.046, 0.046, -0.078, -0.0495, -0.004, 0.033, "body", T_BLOCK),
        channel("terminal-cover", 0.064, -0.012, 0.033, 0.0025, 0.006, 4, -0.106, -0.0495, T_TERMINAL),
        lid("terminal-cover-top", 0.064, -0.012, 0.033, 0.006, 4, -0.0495, -0.047, T_TERMINAL),
        turned_z("screw-terminal", 0, -0.078, [(-0.016, 0), (-0.016, 0.0045), (-0.0125, 0.0045), (-0.0125, 0.007), (-0.0095, 0.007), (-0.0095, 0)], "detail", 0.3, STEEL),
        plate("cover", rr(0, CY, 0.066, 0.070, 0.012), -0.050, 0.033, "body", T_COVER, holes=[rr(0, CY, 0.0635, 0.0675, 0.0095)], taper=[0.94, 1]),
        plate("cover-shoulder", rr(0, CY, 0.06204, 0.0658, 0.0113), -0.0555, -0.050, "body", T_COVER, holes=[rr(0, CY, 0.0595, 0.0633, 0.0088)], taper=[0.93, 1]),
        plate("cover-bevel", rr(0, CY, 0.0577, 0.0612, 0.0105), -0.0582, -0.0555, "body", T_COVER, holes=[rr(0, CY, 0.0535, 0.0557, 0.006)], taper=[0.915, 1]),
        plate("window-frame", rr(0, CY, 0.050, 0.052, 0.007), -0.059, -0.0565, "body", T_COVER, holes=[rr(0, CY, 0.044, 0.046, 0.005)]),
        box("window", -0.0465, 0.0465, CY - 0.0485, CY + 0.0485, -0.0565, -0.0545, "glazing", 0.85, PC),
        turned_z("seal-screws", -0.050, -0.033, [(-0.0585, 0), (-0.0585, 0.0045), (-0.0572, 0.0045), (-0.0572, 0.006), (-0.046, 0.006), (-0.046, 0)], "detail", 0.3, STEEL, repeat={"count": 2, "translation": [0, -0.100, 0]}),
        plate("plate", rr(0, 0.032, 0.042, 0.032, 0.002, 2), -0.0505, -0.0485, "body", T_DIAL, holes=[rr(0, 0.048, 0.024, 0.0055, 0.001, 1)]),
        turned_x("digits", 0.048, -0.040, [(-0.0222, 0), (-0.0222, 0.007), (0.0142, 0.007), (0.0142, 0)], "detail", 0.95, ABS),
        turned_x("digit-decimal", 0.048, -0.040, [(0.0150, 0), (0.0150, 0.007), (0.0222, 0.007), (0.0222, 0)], "detail", 0.5, ABS),
        box("register", -0.026, 0.026, 0.036, 0.056, -0.033, -0.018, "body", T_DARK),
        box("frame-back", -0.040, 0.040, -0.040, 0.074, 0.040, 0.044, "body", T_STEEL, STEEL),
        turned_z("frame-posts-lower", -0.032, -0.030, [(0.044, 0), (0.044, 0.004), (0.056, 0.004), (0.056, 0)], "detail", T_STEEL, STEEL, repeat={"count": 2, "translation": [0, -0.064, 0]}),
        turned_z("frame-posts-upper", -0.032, 0.064, [(0.044, 0), (0.044, 0.004), (0.056, 0.004), (0.056, 0)], "detail", T_STEEL, STEEL, repeat={"count": 2, "translation": [0, -0.064, 0]}),
        box("frame-shelf-top", -0.030, 0.030, 0.032, 0.036, -0.034, 0.040, "body", T_STEEL, STEEL),
        box("frame-shelf-bottom", -0.036, 0.036, -0.038, -0.034, -0.046, 0.040, "body", T_STEEL, STEEL),
        turned_y("bearing-lower", 0, ZD, [(-0.034, 0), (-0.034, 0.004), (-0.031, 0.004), (-0.030, 0.003), (-0.030, 0)], "detail", 0.3, STEEL),
        box("voltage-core", -0.021, 0.021, -0.007, 0.032, 0.006, 0.026, "body", T_CORE, STEEL),
        box("voltage-coil", -0.015, 0.015, 0.002, 0.024, 0.002, 0.030, "body", T_COPPER, COPPER, radius=0.004),
        box("current-core", -0.020, 0.020, -0.034, -0.0126, 0.006, 0.026, "body", T_CORE, STEEL),
        box("current-coils", -0.0205, -0.0065, -0.031, -0.016, 0.002, 0.030, "body", T_COPPER, COPPER, radius=0.003, repeat={"count": 2, "translation": [0, -0.027, 0]}),
        solid("brake-magnet", "extrude", "body", 0.25, STEEL, profile=wu([(-0.0475, -0.022), (-0.024, -0.022), (-0.024, -0.0125), (-0.043, -0.0125), (-0.043, -0.0075), (-0.024, -0.0075), (-0.024, 0.002), (-0.0475, 0.002)]), plane="zx", **{"from": 0.020, "to": 0.036}),
        box("magnet-bracket", -0.034, -0.022, -0.034, -0.022, -0.046, -0.036, "detail", T_STEEL, STEEL),
        rotor("disc", [(-0.0106, 0), (-0.0106, 0.042), (-0.0094, 0.042), (-0.0094, 0)], "body", 0.86, ALU),
        rotor("hub", [(-0.0136, 0), (-0.0136, 0.004), (-0.0106, 0.005), (-0.0094, 0.005), (-0.0064, 0.004), (-0.0064, 0)], "detail", 0.4, STEEL),
        rotor("spindle", [(-0.030, 0), (-0.030, 0.0015), (0.032, 0.0015), (0.032, 0)], "detail", 0.4, STEEL, slender=True),
        solid("mark", "box", "texture", 0.1, ALU, bone="disc", size=[0.0016, 0.012, 0.005], transform={"translation": [0.020, 0, -0.0397]}, texture=True),
    ]
    thermal = [
        plate("th-cover", rr(0, CY, 0.066, 0.070, 0.012), -0.050, 0.033, "body", T_COVER, taper=[0.94, 1], region="casing", layer="shell"),
        plate("th-cover-shoulder", rr(0, CY, 0.06204, 0.0658, 0.0113), -0.0555, -0.050, "body", T_COVER, taper=[0.93, 1], region="casing", layer="shell", sectionFaces=["-w"]),
        {"id": "th-cover-bevel", "of": "cover-bevel", "region": "casing", "layer": "shell"},
        {"id": "th-window-frame", "of": "window-frame", "region": "casing", "layer": "shell"},
        plate("th-base", rr(0, -0.004, 0.066, 0.098, 0.010), 0.033, 0.059, "body", T_BASE, region="casing", layer="shell"),
        {"id": "th-terminal-block-foot", "of": "terminal-block-foot", "region": "casing", "layer": "shell"},
        {"id": "th-terminal-block", "of": "terminal-block", "region": "casing", "layer": "shell"},
        {"id": "th-terminal-cover", "of": "terminal-cover", "region": "casing", "layer": "shell"},
        {"id": "th-terminal-cover-top", "of": "terminal-cover-top", "region": "casing", "layer": "shell"},
        {"id": "th-window", "of": "window", "region": "window", "layer": "pane", "sectionFaces": ["+w"]},
        {"id": "th-disc", "of": "disc", "region": "disc", "layer": "sheet"},
        {"id": "th-plate", "of": "plate", "region": "plate", "layer": "plate"},
    ]
    structure = [
        {"id": "member-base", "of": "base", "member": "body", "part": "member"},
        {"id": "member-base-rim", "of": "base-rim", "member": "body", "part": "member"},
        {"id": "member-lug", "of": "lug", "member": "body", "part": "member"},
        {"id": "member-frame-back", "of": "frame-back", "member": "body", "part": "member"},
        {"id": "member-frame-posts-lower", "of": "frame-posts-lower", "member": "body", "part": "member"},
        {"id": "member-frame-posts-upper", "of": "frame-posts-upper", "member": "body", "part": "member"},
        {"id": "member-frame-shelf-top", "of": "frame-shelf-top", "member": "body", "part": "member"},
        {"id": "member-frame-shelf-bottom", "of": "frame-shelf-bottom", "member": "body", "part": "member"},
        {"id": "member-spindle", "of": "spindle", "member": "disc", "part": "member"},
        {"id": "joint-bearing-lower", "of": "bearing-lower", "member": "disc", "part": "joint"},
        turned_y("joint-bearing-upper", 0, ZD, [(0.028, 0), (0.028, 0.003), (0.032, 0.003), (0.032, 0)], "detail", 0.3, STEEL, member="disc", part="joint"),
        turned_z("support-wall-screw-top", 0, 0.100, [(0.0535, 0), (0.0535, 0.0035), (0.0545, 0.005), (0.057, 0.005), (0.057, 0.0025), (0.059, 0.0025), (0.059, 0)], "detail", 0.25, STEEL, member="body", part="support", anchor="wall-screw-top"),
        turned_z("support-wall-screw-bottom", -0.0525, Y0, [(0.0295, 0), (0.0295, 0.0035), (0.0305, 0.005), (0.033, 0.005), (0.033, 0.0025), (0.059, 0.0025), (0.059, 0)], "detail", 0.25, STEEL, member="body", part="support", anchor="wall-screw-bottom", repeat={"count": 2, "translation": [0, -0.105, 0]}),
        solid("mass-measuring-system", "ellipsoid", "detail", 0.15, STEEL, radii=[0.012, 0.012, 0.012], transform={"translation": [r6(kit.COM - Y0), 0, 0]}, member="body", part="mass"),
    ]
    decisions = [
        {"en": "Body of 1:50: a single-phase Ferraris meter of 132 by 212 by 118 mm as it is built. The base is a 26 mm tray (3 mm back wall, rim around the measuring chamber) that runs down behind the terminal block; the steel hanging lug with its 5.5 mm hole stands 12 mm above it. The meter cover is a hollow 2.5 mm shell: side walls with 2.8 degrees of draft, a shoulder and a bevel that round the front edge in two facets, a raised window frame and the 2 mm pane behind it; two sealing-screw towers rise from the lower front corners. The terminal cover is a channel with a top wall, open below for the cables, in front of the terminal block, with its sealing screw; the block has its four 7.2 mm bores from below.", "de": "Body bei 1:50: ein einphasiger Ferraris-Zähler von 132 mal 212 mal 118 mm, wie er gebaut ist. Die Grundplatte ist eine 26 mm tiefe Wanne (3 mm Rückwand, Rand um den Messwerkraum), die hinter dem Klemmenblock nach unten läuft; die stählerne Aufhängeöse mit 5,5-mm-Loch steht 12 mm darüber. Die Zählerkappe ist eine hohle 2,5-mm-Schale: Seitenwände mit 2,8 Grad Formschräge, Schulter und Fase runden die Vorderkante in zwei Facetten, davor der erhabene Fensterrahmen und dahinter die 2 mm starke Scheibe; zwei Plombierschraubentürme steigen aus den unteren Vorderecken. Der Klemmendeckel ist eine Rinne mit Deckwand, unten für die Leitungen offen, vor dem Klemmenblock, mit seiner Plombierschraube; der Block hat seine vier 7,2-mm-Bohrungen von unten."},
        {"en": "Behind the window: the dial plate with the register opening, five whole-kilowatt-hour rollers as one drum and the decimal roller, the register case, the die-cast frame (back plate on four posts, top and bottom shelf), the voltage electromagnet above and the current electromagnet with its two coils below the disc, the brake magnet as a C around the rim on its bracket, and the rotor on the disc bone: 84 mm disc of 1.2 mm aluminium, hub, 3 mm spindle from the lower bearing to the upper bearing, and the dark rim mark (texture). The disc is horizontal and turns about the upright spindle (twist).", "de": "Hinter dem Fenster: das Zifferblatt mit der Zählwerksöffnung, fünf Rollen für ganze Kilowattstunden als eine Trommel und die Dezimalrolle, das Zählwerksgehäuse, der Druckgussrahmen (Rückplatte auf vier Stehbolzen, obere und untere Konsole), der Spannungsmagnet über und der Strommagnet mit seinen zwei Spulen unter der Scheibe, der Bremsmagnet als C um den Scheibenrand auf seinem Halter und der Läufer am Scheibenknochen: 84-mm-Scheibe aus 1,2 mm Aluminium, Nabe, 3-mm-Achse vom unteren zum oberen Lager und die dunkle Randmarke (Textur). Die Scheibe liegt waagrecht und dreht um die senkrechte Achse (Verdrehung)."},
        {"en": "Thermal of 1:50: the casing is the outer surface of the cover (wall and shoulder as closed forms, so that the sealed measuring chamber counts no area), bevel, window frame, base, terminal block and terminal cover; the window is the pane, its inner face marked as the face inside the sealed meter; the dial plate over the measuring system and the disc are the real parts behind the pane. Structure of 1:50: the load path is the lug, the base tray, the frame on its posts with both shelves and the spindle between its two bearings; the upper screw through the lug is the pin, the two lower screws beside the terminal block are the roller pair that bears against the board; the mass sits at the centre of mass of the housing bone.", "de": "Thermik bei 1:50: Das Gehäuse ist die Außenfläche der Kappe (Wand und Schulter als geschlossene Formen, damit der plombierte Messwerkraum keine Fläche zählt), Fase, Fensterrahmen, Grundplatte, Klemmenblock und Klemmendeckel; das Fenster ist die Scheibe, ihre Innenseite als Fläche im plombierten Zähler markiert; das Zifferblatt über dem Messwerk und die Läuferscheibe sind die echten Teile hinter der Scheibe. Tragwerk bei 1:50: Der Lastpfad ist die Öse, die Grundplattenwanne, der Rahmen auf seinen Stehbolzen mit beiden Konsolen und die Achse zwischen ihren zwei Lagern; die obere Schraube durch die Öse ist das Festlager, die zwei unteren Schrauben neben dem Klemmenblock sind das Rollenlagerpaar, das sich an der Tafel abstützt; die Masse sitzt im Schwerpunkt des Gehäuseknochens."},
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_100():
    body = [
        plate("base", rr(0, -0.004, 0.066, 0.098, 0.010, 2), 0.033, 0.059, "body", T_BASE),
        plate("lug", [(0.008, 0.086), (0.008, 0.100), (0.004, 0.106), (-0.004, 0.106), (-0.008, 0.100), (-0.008, 0.086)], 0.0567, 0.0589, "body", T_STEEL, STEEL),
        box("terminal-block", -0.046, 0.046, -0.102, -0.0495, -0.004, 0.033, "body", T_BLOCK),
        channel("terminal-cover", 0.064, -0.012, 0.033, 0.0025, 0.004, 1, -0.106, -0.0495, T_TERMINAL),
        lid("terminal-cover-top", 0.064, -0.012, 0.033, 0.004, 1, -0.0495, -0.047, T_TERMINAL),
        plate("cover", rr(0, CY, 0.066, 0.070, 0.012, 2), -0.050, 0.033, "body", T_COVER, holes=[rr(0, CY, 0.0635, 0.0675, 0.0095, 2)], taper=[0.94, 1]),
        plate("cover-bevel", rr(0, CY, 0.062, 0.0658, 0.011, 2), -0.059, -0.050, "body", T_COVER, holes=[rr(0, CY, 0.0515, 0.0538, 0.006, 2)], taper=[0.855, 1]),
        box("window", -0.0457, 0.0457, CY - 0.0477, CY + 0.0477, -0.057, -0.055, "glazing", 0.85, PC),
        plate("plate", [(0.042, 0.0), (0.042, 0.064), (-0.042, 0.064), (-0.042, 0.0)], -0.0465, -0.0445, "body", T_DIAL, holes=[[(0.024, 0.0425), (0.024, 0.0535), (-0.024, 0.0535), (-0.024, 0.0425)]]),
        box("register", -0.026, 0.026, 0.036, 0.056, -0.0445, -0.018, "body", T_DARK),
        box("drive-upper", -0.021, 0.021, -0.007, 0.036, 0.002, 0.030, "body", T_CORE, STEEL),
        box("drive-lower", -0.021, 0.021, -0.038, -0.0126, 0.002, 0.030, "body", T_CORE, STEEL),
        box("frame-back", -0.040, 0.040, -0.040, 0.074, 0.030, 0.056, "body", T_STEEL, STEEL),
        rotor("disc", [(-0.0106, 0), (-0.0106, 0.042), (-0.0094, 0.042), (-0.0094, 0)], "body", 0.86, ALU),
        solid("mark", "box", "texture", 0.1, ALU, bone="disc", size=[0.0022, 0.012, 0.004], transform={"translation": [0.020, 0, -0.0401]}, texture=True),
    ]
    thermal = [
        plate("th-cover", rr(0, CY, 0.066, 0.070, 0.012, 2), -0.050, 0.033, "body", T_COVER, taper=[0.94, 1], region="casing", sectionFaces=["-w"]),
        {"id": "th-cover-bevel", "of": "cover-bevel", "region": "casing"},
        {"id": "th-base", "of": "base", "region": "casing"},
        {"id": "th-terminal-block", "of": "terminal-block", "region": "casing"},
        {"id": "th-terminal-cover", "of": "terminal-cover", "region": "casing"},
        {"id": "th-terminal-cover-top", "of": "terminal-cover-top", "region": "casing"},
        {"id": "th-window", "of": "window", "region": "window", "sectionFaces": ["+w"]},
        {"id": "th-disc", "of": "disc", "region": "disc"},
        {"id": "th-plate", "of": "plate", "region": "plate"},
    ]
    structure = [
        {"id": "member-base", "of": "base", "member": "body", "part": "member"},
        {"id": "member-lug", "of": "lug", "member": "body", "part": "member"},
        {"id": "member-frame-back", "of": "frame-back", "member": "body", "part": "member"},
        box("member-frame-shelves", -0.030, 0.030, -0.038, -0.034, -0.034, 0.030, "body", T_STEEL, STEEL, member="body", part="member", repeat={"count": 2, "translation": [0.070, 0, 0]}),
        rotor("member-spindle", [(-0.030, 0), (-0.030, 0.0015), (0.032, 0.0015), (0.032, 0)], "detail", 0.4, STEEL, slender=True, member="disc", part="member"),
        turned_y("joint-bearing-lower", 0, ZD, [(-0.034, 0), (-0.034, 0.004), (-0.030, 0.004), (-0.030, 0)], "detail", 0.3, STEEL, member="disc", part="joint"),
        turned_z("support-wall-screw-top", 0, 0.100, [(0.053, 0), (0.053, 0.005), (0.057, 0.005), (0.057, 0.0025), (0.059, 0.0025), (0.059, 0)], "detail", 0.25, STEEL, member="body", part="support", anchor="wall-screw-top"),
        turned_z("support-wall-screw-bottom", -0.0525, Y0, [(0.029, 0), (0.029, 0.005), (0.033, 0.005), (0.033, 0.0025), (0.059, 0.0025), (0.059, 0)], "detail", 0.25, STEEL, member="body", part="support", anchor="wall-screw-bottom", repeat={"count": 2, "translation": [0, -0.105, 0]}),
        solid("mass-measuring-system", "ellipsoid", "detail", 0.15, STEEL, radii=[0.014, 0.014, 0.014], transform={"translation": [r6(kit.COM - Y0), 0, 0]}, member="body", part="mass"),
    ]
    decisions = [
        {"en": "Body of 1:100, drawn anew for a 15 mm feature: the base is one 26 mm slab, the lug a plain tab without its hole, the terminal block one piece without bores and the terminal cover a channel with chamfered front edges and a top wall. The cover keeps its hollow shell and its draft, but the front edge is one 45 degree bevel that also forms the splayed window reveal, with the pane set into it. Behind the window: dial plate with register opening, the dark register case behind it, the voltage and current electromagnets as two blocks, the frame back as one block to the base, and the disc with its rim mark (texture). Sealing screws, hub, spindle, brake magnet and posts are left out of the body.", "de": "Body bei 1:100, neu gezeichnet für ein 15-mm-Merkmal: Die Grundplatte ist eine 26 mm starke Platte, die Öse eine schlichte Lasche ohne Loch, der Klemmenblock ein Stück ohne Bohrungen und der Klemmendeckel eine Rinne mit gefasten Vorderkanten und Deckwand. Die Kappe behält ihre hohle Schale und ihre Formschräge, die Vorderkante ist aber eine einzige 45-Grad-Fase, die zugleich die schräge Fensterlaibung bildet, mit der Scheibe darin. Hinter dem Fenster: Zifferblatt mit Zählwerksöffnung, das dunkle Zählwerksgehäuse dahinter, Spannungs- und Strommagnet als zwei Blöcke, die Rahmenrückwand als ein Block bis zur Grundplatte und die Scheibe mit Randmarke (Textur). Plombierschrauben, Nabe, Achse, Bremsmagnet und Stehbolzen entfallen im Body."},
        {"en": "Thermal of 1:100: casing from the closed cover form, bevel, base, terminal block and terminal cover; pane with its inner face marked; dial plate and disc behind it. Structure of 1:100: lug, base slab, frame back and the two shelves as one repeated bar, the slender spindle on its lower bearing, the upper screw as pin and the pair of lower screws as roller, and the mass at the centre of mass.", "de": "Thermik bei 1:100: Gehäuse aus der geschlossenen Kappenform, Fase, Grundplatte, Klemmenblock und Klemmendeckel; Scheibe mit markierter Innenseite; Zifferblatt und Läuferscheibe dahinter. Tragwerk bei 1:100: Öse, Grundplatte, Rahmenrückwand und die zwei Konsolen als ein wiederholter Stab, die schlanke Achse auf ihrem unteren Lager, die obere Schraube als Festlager und das Paar unterer Schrauben als Rollenlager sowie die Masse im Schwerpunkt."},
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_200():
    body = [
        plate("base", rr(0, -0.004, 0.066, 0.098, 0.010, 1), 0.033, 0.059, "body", T_BASE),
        box("terminals", -0.064, 0.064, -0.106, -0.047, -0.012, 0.033, "body", T_TERMINAL),
        plate("cover", rr(0, CY, 0.066, 0.070, 0.012, 1), -0.059, 0.033, "body", T_COVER, holes=[[(0.0468, CY - 0.0489), (0.0468, CY + 0.0489), (-0.0468, CY + 0.0489), (-0.0468, CY - 0.0489)]], taper=[0.94, 1]),
        box("window", -0.044, 0.044, CY - 0.046, CY + 0.046, -0.058, -0.056, "glazing", 0.85, PC),
        box("works", -0.042, 0.042, 0.0, 0.064, -0.0505, 0.030, "body", T_DIAL),
        rotor("disc", [(-0.0106, 0), (-0.0106, 0.042), (-0.0094, 0.042), (-0.0094, 0)], "body", 0.86, ALU),
    ]
    thermal = [
        plate("th-cover", rr(0, CY, 0.066, 0.070, 0.012, 1), -0.059, 0.033, "body", T_COVER, taper=[0.94, 1], region="casing"),
        {"id": "th-base", "of": "base", "region": "casing"},
        {"id": "th-terminals", "of": "terminals", "region": "casing"},
        {"id": "th-window", "of": "window", "region": "window", "sectionFaces": ["+w"]},
        {"id": "th-disc", "of": "disc", "region": "disc"},
        box("th-plate", -0.042, 0.042, 0.0, 0.064, -0.0525, -0.0505, "body", T_DIAL, region="plate", layer="plate"),
    ]
    structure = [
        {"id": "member-base", "of": "base", "member": "body", "part": "member"},
        rotor("member-spindle", [(-0.034, 0), (-0.034, 0.0015), (0.032, 0.0015), (0.032, 0)], "detail", 0.4, STEEL, slender=True, member="disc", part="member"),
        turned_z("support-wall-screw-top", 0, 0.100, [(0.029, 0), (0.029, 0.015), (0.059, 0.015), (0.059, 0)], "detail", 0.25, STEEL, member="body", part="support", anchor="wall-screw-top"),
        turned_z("support-wall-screw-bottom", -0.0525, Y0, [(0.029, 0), (0.029, 0.015), (0.059, 0.015), (0.059, 0)], "detail", 0.25, STEEL, member="body", part="support", anchor="wall-screw-bottom", repeat={"count": 2, "translation": [0, -0.105, 0]}),
        solid("mass-measuring-system", "ellipsoid", "detail", 0.15, STEEL, radii=[0.016, 0.016, 0.016], transform={"translation": [r6(kit.COM - Y0), 0, 0]}, member="body", part="mass"),
    ]
    decisions = [
        {"en": "Body of 1:200, drawn anew for a 30 mm feature: base slab, the terminal cover and block as one set-back block, the cover as a thick hollow shell with its draft, square at the front because the 9 mm rounding is below the feature, its bore as wide as the window opening with the pane in it, and behind it the dial plate with register and measuring system as one light block above the horizontal disc. Lug, screws, rim mark, rollers and the single parts of the measuring system are below the feature and left out.", "de": "Body bei 1:200, neu gezeichnet für ein 30-mm-Merkmal: Grundplatte, Klemmendeckel und Klemmenblock als ein zurückgesetzter Block, die Kappe als dicke hohle Schale mit ihrer Formschräge, vorn kantig, weil die 9-mm-Rundung unter dem Merkmal liegt, ihre Bohrung so weit wie die Fensteröffnung mit der Scheibe darin, und dahinter Zifferblatt mit Zählwerk und Messwerk als ein heller Block über der waagrechten Läuferscheibe. Öse, Schrauben, Randmarke, Rollen und die Einzelteile des Messwerks liegen unter dem Merkmal und entfallen."},
        {"en": "Thermal of 1:200: casing from the closed cover form, base and terminal block; the pane; the disc; the dial plate as its own 2 mm sheet at the front of the block. Structure of 1:200: the base slab as the member between the upper screw (pin) and the pair of lower screws (roller), both as 30 mm studs, the slender spindle, and the mass at the centre of mass; the model is already minimal.", "de": "Thermik bei 1:200: Gehäuse aus der geschlossenen Kappenform, Grundplatte und Klemmenblock; die Scheibe; die Läuferscheibe; das Zifferblatt als eigenes 2-mm-Blech an der Vorderseite des Blocks. Tragwerk bei 1:200: die Grundplatte als Stab zwischen der oberen Schraube (Festlager) und dem Paar unterer Schrauben (Rollenlager), beide als 30-mm-Bolzen, die schlanke Achse und die Masse im Schwerpunkt; das Modell ist bereits minimal."},
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def square(x, y0, y1):
    return [(x, y0), (x, y1), (-x, y1), (-x, y0)]


def scale_500():
    body = [
        box("base", -0.066, 0.066, -0.102, 0.094, 0.033, 0.059, "body", T_BASE),
        box("terminals", -0.066, 0.066, -0.106, -0.046, -0.012, 0.033, "body", T_TERMINAL),
        plate("cover", square(0.066, -0.046, 0.094), -0.059, 0.033, "body", T_COVER, holes=[square(0.044, CY - 0.046, CY + 0.046)]),
        box("window", -0.0455, 0.0455, CY - 0.0475, CY + 0.0475, -0.0575, -0.0555, "glazing", 0.85, PC),
        box("works", -0.042, 0.042, -0.034, 0.064, -0.0505, 0.033, "body", T_DIAL),
    ]
    thermal = [
        plate("th-cover", square(0.066, -0.046, 0.094), -0.059, 0.033, "body", T_COVER, region="casing"),
        {"id": "th-base", "of": "base", "region": "casing"},
        {"id": "th-terminals", "of": "terminals", "region": "casing"},
        {"id": "th-window", "of": "window", "region": "window", "sectionFaces": ["+w"]},
        rotor("th-disc", [(-0.0105, 0), (-0.0105, 0.042), (-0.0095, 0.042), (-0.0095, 0)], "body", 0.86, ALU, region="disc", layer="sheet"),
        box("th-plate", -0.042, 0.042, 0.0, 0.064, -0.0525, -0.0505, "body", T_DIAL, region="plate", layer="plate"),
    ]
    structure = [
        {"id": "member-base", "of": "base", "member": "body", "part": "member"},
        rotor("member-spindle", [(-0.030, 0), (-0.030, 0.0015), (0.032, 0.0015), (0.032, 0)], "detail", 0.4, STEEL, slender=True, member="disc", part="member"),
        turned_z("support-wall-screw-top", 0, 0.100, [(0.047, 0), (0.047, 0.012), (0.059, 0.012), (0.059, 0)], "detail", 0.25, STEEL, member="body", part="support", anchor="wall-screw-top"),
        turned_z("support-wall-screw-bottom", -0.0525, Y0, [(0.047, 0), (0.047, 0.012), (0.059, 0.012), (0.059, 0)], "detail", 0.25, STEEL, member="body", part="support", anchor="wall-screw-bottom", repeat={"count": 2, "translation": [0, -0.105, 0]}),
        solid("mass-measuring-system", "ellipsoid", "detail", 0.15, STEEL, radii=[0.02, 0.02, 0.02], transform={"translation": [r6(kit.COM - Y0), 0, 0]}, member="body", part="mass"),
    ]
    decisions = [
        {"en": "Body of 1:500, a digital scale without a printed drawing (the meter is 0.26 mm by 0.42 mm on paper), drawn anew for a 75 mm feature: the meter is under three features in every direction, so only its three masses and the window remain. The base is a square-cornered 26 mm slab against the wall, the terminal cover and block one block as wide as the base and set back by 47 mm, the cover a square-cornered hollow shell without draft, rounding or bevel (all under the feature) with the 88 by 92 mm window opening and the pane in it, and behind the pane dial plate, register, measuring system and rotor disc as one light block. Lug, screws, the disc as a separate part and every roller are left out of the body.", "de": "Body bei 1:500, ein digitaler Maßstab ohne gedruckte Zeichnung (der Zähler misst auf Papier 0,26 mm mal 0,42 mm), neu gezeichnet für ein 75-mm-Merkmal: Der Zähler ist in jeder Richtung kleiner als drei Merkmale, es bleiben nur seine drei Massen und das Fenster. Die Grundplatte ist eine kantige 26-mm-Platte an der Wand, Klemmendeckel und Klemmenblock sind ein Block in der Breite der Grundplatte, um 47 mm zurückgesetzt, die Kappe ist eine kantige hohle Schale ohne Formschräge, Rundung und Fase (alles unter dem Merkmal) mit der Fensteröffnung von 88 mal 92 mm und der Scheibe darin, und hinter der Scheibe sind Zifferblatt, Zählwerk, Messwerk und Läuferscheibe ein heller Block. Öse, Schrauben, die Scheibe als eigenes Teil und alle Rollen entfallen im Body."},
        {"en": "Thermal of 1:500: casing from the closed cover form, base and terminal block; the pane with its inner face marked; the dial plate as its own 2 mm sheet at the front of the block; the rotor disc as the real 1 mm sheet inside the block, where it lies in the meter. Structure of 1:500: the base slab is the member between the upper screw (pin) and the pair of lower screws (roller), drawn as 24 mm pads let into the back of the base, the upper one standing 6 mm into the slab where the lug hangs, with the slender spindle of the rotor and the mass at the centre of mass; the model is already minimal.", "de": "Thermik bei 1:500: Gehäuse aus der geschlossenen Kappenform, Grundplatte und Klemmenblock; die Scheibe mit markierter Innenseite; das Zifferblatt als eigenes 2-mm-Blech an der Vorderseite des Blocks; die Läuferscheibe als echtes 1-mm-Blech im Block, wo sie im Zähler liegt. Tragwerk bei 1:500: Die Grundplatte ist der Stab zwischen der oberen Schraube (Festlager) und dem Paar unterer Schrauben (Rollenlager), gezeichnet als 24-mm-Teller in der Rückseite der Grundplatte, der obere greift dort, wo die Öse hängt, 6 mm in die Platte, dazu die schlanke Achse des Läufers und die Masse im Schwerpunkt; das Modell ist bereits minimal."},
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    volumes = {"50": scale_50(), "100": scale_100(), "200": scale_200(), "500": scale_500()}
    path = os.path.join(HERE, "volumes", "appliances", "electricity-meter.json")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({"volumes": volumes}, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("written", path, {key: {name: len(value.get(name, [])) for name in ("body", "thermal", "structure")} for key, value in volumes.items()})


if __name__ == "__main__":
    main()
