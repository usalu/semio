"""🌱 Round 3, appliances/electricity-meter: builds the candidate asset document with the root corrections (dimensions, materials, rig, anchors, the 2D drawings of every scale and view, poses, clips, thermal and structural data, texts) and the volumes source, as `🗑️generated/r3-electricity-meter/🔣️.json`.

The catalog file is read, never written, here; `r3-electricity-meter-apply.py` writes the validated candidate. Every shape is drawn by hand in world millimetres as the true orthographic view of the 1:N body and placed in the bone frames by the kit.

Usage: python3 r3-electricity-meter-roots.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-electricity-meter-kit.py"))
kit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(kit)
r6, Y0, YD, ZD, ZB = kit.r6, kit.Y0, kit.YD, kit.ZD, kit.ZB
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electricity-meter/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "electricity-meter.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-electricity-meter", "🔣️.json")
CY = 0.024
ABS, PC, ALU, STEEL = "abs-plastic", "polycarbonate", "aluminium-disc", "steel-zinc"
AREAS = {"casing": 0.139, "window": 0.0096, "disc": 0.0107, "plate": 0.0104}


def tx(en, de):
    return {"en": en, "de": de}


def sh(ident, bone, role, z, filled, tone, geometry, region=None, material=None, **extra):
    out = {"id": ident, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, **geometry}
    if region is not None:
        out["region"] = region
    if material is not None:
        out["material"] = material
    out.update(extra)
    return out


def rect_e(x0, y0, w, h, r=None):
    out = {"kind": "rect", "x": r6(y0 - Y0), "y": r6(-(x0 + w)), "w": r6(h), "h": r6(w)}
    if r is not None:
        out["r"] = r
    return out


def rect_p(x0, y0, w, h, r=None):
    out = {"kind": "rect", "x": r6(x0), "y": r6(y0), "w": r6(w), "h": r6(h)}
    if r is not None:
        out["r"] = r
    return out


def poly_e(points, closed=True, y0=Y0):
    return {"kind": "poly", "points": kit.uv(points, y0), "closed": closed}


def poly_p(points, closed=True):
    return {"kind": "poly", "points": [[r6(a), r6(b)] for (a, b) in points], "closed": closed}


def dot_e(x, y, r):
    return {"kind": "ellipse", "cx": r6(y - Y0), "cy": r6(-x), "rx": r, "ry": r}


def lug(n):
    return [(0.008, 0.086), (0.008, 0.098)] + [(0.008 * math.cos(math.radians(180 * k / n)), 0.098 + 0.008 * math.sin(math.radians(180 * k / n))) for k in range(1, n)] + [(-0.008, 0.098), (-0.008, 0.086)]


REVOLVED = {"kind": "revolved", "axisBone": "disc", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
DISC = poly_e([(0, -0.0106), (-0.042, -0.0106), (-0.042, -0.0094), (0, -0.0094)], True, YD)
HUB = poly_e([(0, -0.0136), (-0.004, -0.0136), (-0.005, -0.0106), (-0.005, -0.0094), (-0.004, -0.0064), (0, -0.0064)], True, YD)


def mark(half_thickness, z0, z1):
    return {"geometry": poly_e([(0.006, -0.010 - half_thickness), (0.006, -0.010 + half_thickness), (-0.006, -0.010 + half_thickness), (-0.006, -0.010 - half_thickness)], True, YD), "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}


def rotor_shapes(scale):
    out = [sh("disc", "disc", "body", 3.45, True, 0.86, DISC, "disc", ALU, **({"penMm": 0.2} if scale == 200 else {}), solid=REVOLVED)]
    if scale == 50:
        out.append(sh("hub", "disc", "detail", 3.5, True, 0.4, HUB, "disc", STEEL, solid=REVOLVED))
        m = mark(0.0008, -0.0422, -0.0372)
        out.append(sh("mark", "disc", "body", 3.55, True, 0.1, m["geometry"], "disc", ALU, solid=m["solid"]))
    if scale == 100:
        m = mark(0.0011, -0.0421, -0.0381)
        out.append(sh("mark", "disc", "body", 3.55, True, 0.1, m["geometry"], "disc", ALU, solid=m["solid"]))
    return out


def geometry_50():
    elevation = [
        sh("back-plate", "body", "body", 3, True, 0.7, rect_e(-0.066, -0.102, 0.132, 0.196, 0.010), "casing", ABS),
        sh("eye", "body", "body", 3.05, True, 0.45, poly_e(lug(4)), "casing", STEEL),
        sh("body", "body", "body", 3.1, True, 0.88, rect_e(-0.066, -0.046, 0.132, 0.140, 0.012), "casing", ABS),
        sh("face-edge", "body", "detail", 3.15, False, 0.35, rect_e(-0.0528, CY - 0.056, 0.1056, 0.112, 0.009), "casing", ABS),
        sh("rim", "body", "body", 3.2, True, 0.25, rect_e(-0.044, CY - 0.046, 0.088, 0.092, 0.005), "window", PC),
        sh("plate", "body", "body", 3.3, True, 0.93, rect_e(-0.042, 0.0, 0.084, 0.064, 0.002), "plate", ABS),
        sh("counter", "body", "body", 3.35, True, 0.1, rect_e(-0.024, 0.0425, 0.048, 0.011), "plate", ABS),
        sh("digits", "body", "detail", 3.4, True, 0.95, rect_e(-0.0222, 0.0435, 0.0364, 0.009), "plate", ABS),
        sh("digit-decimal", "body", "detail", 3.4, True, 0.5, rect_e(0.0150, 0.0435, 0.0072, 0.009), "plate", ABS),
        sh("plate-line-1", "body", "detail", 3.4, False, 0.25, poly_e([(-0.032, 0.030), (0.032, 0.030)], False), "plate", ABS),
        sh("plate-line-2", "body", "detail", 3.4, False, 0.25, poly_e([(-0.032, 0.021), (0.032, 0.021)], False), "plate", ABS),
        sh("plate-line-3", "body", "detail", 3.4, False, 0.25, poly_e([(-0.032, 0.012), (0.032, 0.012)], False), "plate", ABS),
        *rotor_shapes(50),
        sh("magnet", "body", "detail", 3.6, True, 0.25, rect_e(-0.036, -0.022, 0.016, 0.022), None, STEEL),
        sh("terminals", "body", "body", 3.6, True, 0.55, rect_e(-0.064, -0.106, 0.128, 0.059, 0.006), "casing", ABS),
        sh("screw-left", "body", "detail", 3.7, True, 0.3, dot_e(-0.050, -0.033, 0.0045), "casing", STEEL),
        sh("screw-right", "body", "detail", 3.7, True, 0.3, dot_e(0.050, -0.033, 0.0045), "casing", STEEL),
        sh("screw-terminal", "body", "detail", 3.7, True, 0.3, dot_e(0, -0.078, 0.0045), "casing", STEEL),
    ]
    front = [
        sh("back-plate", "body", "body", 3, True, 0.7, rect_p(0.033, -0.102, 0.026, 0.196), "casing", ABS),
        sh("eye", "body", "body", 3.05, True, 0.45, rect_p(0.057, 0.094, 0.002, 0.012), "casing", STEEL),
        sh("body", "body", "body", 3.1, True, 0.88, poly_p([(0.033, -0.046), (-0.050, -0.0418), (-0.0555, -0.0372), (-0.0582, -0.032), (-0.0582, -0.028), (-0.059, -0.028), (-0.059, 0.076), (-0.0582, 0.076), (-0.0582, 0.080), (-0.0555, 0.0852), (-0.050, 0.0898), (0.033, 0.094)]), "casing", ABS),
        sh("seal-seam", "body", "detail", 3.4, False, 0.3, poly_p([(0.033, -0.046), (0.033, 0.094)], False), "casing", ABS),
        sh("terminals", "body", "body", 3.2, True, 0.55, rect_p(-0.012, -0.106, 0.045, 0.059), "casing", ABS),
        sh("screw-right", "body", "detail", 3.05, True, 0.3, rect_p(-0.0585, -0.039, 0.0125, 0.012), "casing", STEEL),
        sh("screw-terminal", "body", "detail", 3.3, True, 0.3, rect_p(-0.016, -0.0825, 0.004, 0.009), "casing", STEEL),
    ]
    plan = [
        sh("back-plate", "body", "body", 3, True, 0.7, rect_p(-0.066, 0.033, 0.132, 0.026), "casing", ABS),
        sh("body", "body", "body", 3.1, True, 0.88, poly_p([(-0.066, 0.033), (-0.06204, -0.050), (-0.0577, -0.0555), (-0.0528, -0.0582), (-0.050, -0.0582), (-0.050, -0.059), (0.050, -0.059), (0.050, -0.0582), (0.0528, -0.0582), (0.0577, -0.0555), (0.06204, -0.050), (0.066, 0.033)]), "casing", ABS),
        sh("seal-seam", "body", "detail", 3.4, False, 0.3, poly_p([(-0.066, 0.033), (0.066, 0.033)], False), "casing", ABS),
        sh("eye", "body", "body", 3.3, True, 0.45, rect_p(-0.008, 0.057, 0.016, 0.002), "casing", STEEL),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def geometry_100():
    elevation = [
        sh("back-plate", "body", "body", 3, True, 0.7, rect_e(-0.066, -0.102, 0.132, 0.196, 0.010), "casing", ABS),
        sh("eye", "body", "body", 3.05, True, 0.45, poly_e([(0.008, 0.086), (0.008, 0.100), (0.004, 0.106), (-0.004, 0.106), (-0.008, 0.100), (-0.008, 0.086)]), "casing", STEEL),
        sh("body", "body", "body", 3.1, True, 0.88, rect_e(-0.066, -0.046, 0.132, 0.140, 0.012), "casing", ABS),
        sh("rim", "body", "body", 3.2, True, 0.25, rect_e(-0.044, CY - 0.046, 0.088, 0.092, 0.005), "window", PC),
        sh("plate", "body", "body", 3.3, True, 0.93, rect_e(-0.042, 0.0, 0.084, 0.064), "plate", ABS),
        sh("counter", "body", "body", 3.35, True, 0.1, rect_e(-0.024, 0.0425, 0.048, 0.011), "plate", ABS),
        *rotor_shapes(100),
        sh("terminals", "body", "body", 3.6, True, 0.55, rect_e(-0.064, -0.106, 0.128, 0.059, 0.004), "casing", ABS),
    ]
    front = [
        sh("back-plate", "body", "body", 3, True, 0.7, rect_p(0.033, -0.102, 0.026, 0.196), "casing", ABS),
        sh("eye", "body", "body", 3.05, True, 0.45, rect_p(0.057, 0.094, 0.002, 0.012), "casing", STEEL),
        sh("body", "body", "body", 3.1, True, 0.88, poly_p([(0.033, -0.046), (-0.050, -0.0418), (-0.059, -0.0323), (-0.059, 0.0803), (-0.050, 0.0898), (0.033, 0.094)]), "casing", ABS),
        sh("terminals", "body", "body", 3.2, True, 0.55, rect_p(-0.012, -0.106, 0.045, 0.059), "casing", ABS),
    ]
    plan = [
        sh("back-plate", "body", "body", 3, True, 0.7, rect_p(-0.066, 0.033, 0.132, 0.026), "casing", ABS),
        sh("body", "body", "body", 3.1, True, 0.88, poly_p([(-0.066, 0.033), (-0.062, -0.050), (-0.053, -0.059), (0.053, -0.059), (0.062, -0.050), (0.066, 0.033)]), "casing", ABS),
        sh("eye", "body", "body", 3.3, True, 0.45, rect_p(-0.008, 0.057, 0.016, 0.002), "casing", STEEL),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def geometry_200():
    elevation = [
        sh("body", "body", "body", 3, True, 0.88, rect_e(-0.066, -0.102, 0.132, 0.196, 0.010), "casing", ABS),
        sh("rim", "body", "body", 3.2, True, 0.25, rect_e(-0.044, CY - 0.046, 0.088, 0.092), "window", PC),
        sh("plate", "body", "body", 3.3, True, 0.93, rect_e(-0.042, 0.0, 0.084, 0.064), "plate", ABS),
        *rotor_shapes(200),
        sh("terminals", "body", "body", 3.6, True, 0.55, rect_e(-0.064, -0.106, 0.128, 0.059), "casing", ABS),
    ]
    front = [
        sh("back-plate", "body", "body", 2.9, True, 0.7, rect_p(0.033, -0.102, 0.026, 0.196), "casing", ABS, penMm=0.18),
        sh("body", "body", "body", 3, True, 0.88, poly_p([(0.033, 0.094), (-0.059, 0.0898), (-0.059, -0.0418), (0.033, -0.046)]), "casing", ABS),
        sh("terminals", "body", "body", 3.2, True, 0.55, rect_p(-0.012, -0.106, 0.045, 0.059), "casing", ABS),
    ]
    plan = [
        sh("back-plate", "body", "body", 2.9, True, 0.7, rect_p(-0.066, 0.033, 0.132, 0.026), "casing", ABS, penMm=0.18),
        sh("body", "body", "body", 3, True, 0.88, poly_p([(-0.066, 0.033), (-0.062, -0.059), (0.062, -0.059), (0.066, 0.033)]), "casing", ABS),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


DECISIONS = {
    "50": {
        "silhouette": tx(
            "Face view, 2.6 mm by 4.2 mm on paper: a single-phase Ferraris meter of 0.132 m by 0.212 m. The grey base shows as a 2 mm margin and carries the steel hanging lug on top; the light cover with its rounded corners fills the upper 0.14 m, the darker terminal cover the lower 0.059 m. In the cover sits the 88 mm by 92 mm window: behind it the white dial plate with the dark register opening and its rollers, and below the plate the edge of the horizontal rotor disc with its dark mark and the brake magnet around the rim. Side view, 2.4 mm by 4.2 mm: the 26 mm base against the wall, the cover with its draft, shoulder and bevel 92 mm in front of it, the terminal cover set back by 47 mm, the lug as a 2 mm strap and the sealing-screw tower on the lower front corner. Plan, 2.6 mm by 2.4 mm: base, the cover narrowing to its front face, the seam between them and the lug.",
            "Ansicht von vorn, auf Papier 2,6 mm mal 4,2 mm: ein einphasiger Ferraris-Zähler von 0,132 m mal 0,212 m. Die graue Grundplatte zeigt sich als 2 mm breiter Rand und trägt oben die stählerne Aufhängeöse; die helle Kappe mit ihren gerundeten Ecken füllt die oberen 0,14 m, der dunklere Klemmendeckel die unteren 0,059 m. In der Kappe sitzt das Fenster von 88 mm mal 92 mm: dahinter das weiße Zifferblatt mit der dunklen Zählwerksöffnung und ihren Rollen, unter dem Blatt die Kante der waagrechten Läuferscheibe mit ihrer dunklen Marke und der Bremsmagnet um den Scheibenrand. Seitenansicht, 2,4 mm mal 4,2 mm: die 26 mm tiefe Grundplatte an der Wand, davor 92 mm die Kappe mit Formschräge, Schulter und Fase, der um 47 mm zurückgesetzte Klemmendeckel, die Öse als 2 mm starkes Band und der Plombierschraubenturm an der unteren Vorderecke. Grundriss, 2,6 mm mal 2,4 mm: Grundplatte, die zur Front schmaler werdende Kappe, die Fuge dazwischen und die Öse."),
        "retained": tx(
            "Base, hanging lug, cover with the edge of its flat front face, window, dial plate with three text lines, register opening with the five white rollers as one strip and the darker decimal roller, the 84 mm disc edge-on with hub and rim mark, brake magnet, terminal cover, the two sealing screws of the cover and the one of the terminal cover.",
            "Grundplatte, Aufhängeöse, Kappe mit der Kante ihrer ebenen Front, Fenster, Zifferblatt mit drei Textzeilen, Zählwerksöffnung mit den fünf weißen Rollen als ein Streifen und der dunkleren Dezimalrolle, die 84-mm-Scheibe von der Kante gesehen mit Nabe und Randmarke, Bremsmagnet, Klemmendeckel, die zwei Plombierschrauben der Kappe und die eine des Klemmendeckels."),
        "merged": tx(
            "The five whole-kilowatt-hour rollers are one light strip, since a single roller of 7 mm is 0.14 mm on paper; shoulder and bevel of the cover are the two facets of one rounded edge.",
            "Die fünf Rollen der ganzen Kilowattstunden sind ein heller Streifen, da eine einzelne Rolle von 7 mm auf Papier 0,14 mm misst; Schulter und Fase der Kappe sind die zwei Facetten einer gerundeten Kante."),
        "omitted": tx(
            "The spindle, the electromagnets and the frame behind the dial plate, the terminal block and its bores under the terminal cover, the sealing wires and the print of the dial. From the side and from above the window is flush with the cover and nothing behind it is seen; from above the terminal cover lies hidden under the cover.",
            "Die Achse, die Elektromagnete und der Rahmen hinter dem Zifferblatt, der Klemmenblock und seine Bohrungen unter dem Klemmendeckel, die Plombierdrähte und der Aufdruck des Zifferblatts. Von der Seite und von oben ist das Fenster bündig mit der Kappe und nichts dahinter zu sehen; von oben liegt der Klemmendeckel verdeckt unter der Kappe."),
        "thinElements": tx(
            "The disc is 1.2 mm thick (0.02 mm on paper) and reads through its outline pen; its rim mark is 12 mm wide; the lug is a 2 mm strap in the side view and the plan; the text lines are 0.13 mm detail strokes.",
            "Die Scheibe ist 1,2 mm stark (0,02 mm auf Papier) und liest sich über ihren Umrissstift; ihre Randmarke ist 12 mm breit; die Öse ist in Seitenansicht und Grundriss ein 2 mm starkes Band; die Textzeilen sind Detailstriche von 0,13 mm."),
        "hatching": tx("None; the window is a dark tone.", "Keine; das Fenster ist ein dunkler Ton."),
        "styles": tx(
            "Base 0.7, cover 0.88, window 0.25, dial plate 0.93, register opening 0.1 with rollers 0.95 and 0.5, disc 0.86, mark 0.1, magnet 0.25, terminal cover 0.55, lug 0.45, screws 0.3; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
            "Grundplatte 0,7, Kappe 0,88, Fenster 0,25, Zifferblatt 0,93, Zählwerksöffnung 0,1 mit Rollen 0,95 und 0,5, Scheibe 0,86, Marke 0,1, Magnet 0,25, Klemmendeckel 0,55, Öse 0,45, Schrauben 0,3; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": tx(
            "Casing, window, disc and dial plate are four regions: in the thermal view the plate over the measuring system warms slightly with the load. The disc and its mark are solids on the rotor bone, which twists about the upright spindle, so the mark passes from left to right across the window.",
            "Gehäuse, Fenster, Scheibe und Zifferblatt sind vier Regionen: in der Thermalansicht erwärmt sich das Blatt über dem Messwerk mit der Last leicht. Scheibe und Marke sind Körper am Rotorknochen, der sich um die senkrechte Achse verdreht, sodass die Marke von links nach rechts durch das Fenster läuft."),
    },
    "100": {
        "silhouette": tx(
            "Face view, 1.3 mm by 2.1 mm on paper: base margin with the lug as a tab, the light cover, the dark window with the white dial plate, the dark register slot and the disc edge below, and the darker terminal cover. Side view, 1.2 mm by 2.1 mm: base, the cover with one bevel at its front edge, the set-back terminal cover and the lug. Plan, 1.3 mm by 1.2 mm: base, the tapering cover and the lug.",
            "Ansicht von vorn, auf Papier 1,3 mm mal 2,1 mm: Grundplattenrand mit der Öse als Lasche, die helle Kappe, das dunkle Fenster mit dem weißen Zifferblatt, dem dunklen Zählwerksschlitz und der Scheibenkante darunter sowie der dunklere Klemmendeckel. Seitenansicht, 1,2 mm mal 2,1 mm: Grundplatte, die Kappe mit einer Fase an der Vorderkante, der zurückgesetzte Klemmendeckel und die Öse. Grundriss, 1,3 mm mal 1,2 mm: Grundplatte, die sich verjüngende Kappe und die Öse."),
        "retained": tx(
            "Base, lug, cover, window, dial plate, register slot, disc with rim mark, terminal cover.",
            "Grundplatte, Öse, Kappe, Fenster, Zifferblatt, Zählwerksschlitz, Scheibe mit Randmarke, Klemmendeckel."),
        "merged": tx(
            "The rollers join the dark register slot; shoulder and bevel of the cover are one 45 degree bevel; the hub joins the disc.",
            "Die Rollen gehen im dunklen Zählwerksschlitz auf; Schulter und Fase der Kappe sind eine 45-Grad-Fase; die Nabe geht in der Scheibe auf."),
        "omitted": tx(
            "Sealing screws, brake magnet, text lines, the edge of the front face and the seam between base and cover.",
            "Plombierschrauben, Bremsmagnet, Textzeilen, die Kante der Front und die Fuge zwischen Grundplatte und Kappe."),
        "thinElements": tx(
            "The disc edge stays 1.2 mm and reads through the 0.18 mm outline; the 12 mm rim mark is drawn 2.2 mm thick so that it prints; the register slot is 11 mm high.",
            "Die Scheibenkante bleibt 1,2 mm und liest sich über den 0,18-mm-Umriss; die 12 mm breite Randmarke ist 2,2 mm stark gezeichnet, damit sie druckt; der Zählwerksschlitz ist 11 mm hoch."),
        "hatching": tx("None.", "Keine."),
        "styles": tx(
            "Same tones as 1:50 without rollers, screws and magnet; one body pen of 0.18 mm.",
            "Gleiche Töne wie bei 1:50 ohne Rollen, Schrauben und Magnet; ein Körperstift von 0,18 mm."),
        "overlays": tx(
            "Casing, window, disc and dial plate stay four regions; the mark still passes the window with the twist of the rotor.",
            "Gehäuse, Fenster, Scheibe und Zifferblatt bleiben vier Regionen; die Marke läuft weiter mit der Verdrehung des Rotors durch das Fenster."),
    },
    "200": {
        "silhouette": tx(
            "Face view, 0.66 mm by 1.0 mm on paper: one light body of base and cover, the dark window with the light dial plate and the disc edge below it, and the darker terminal cover. Side view, 0.59 mm by 1.0 mm: the base as a 26 mm strip against the wall, the cover with its draft and the set-back terminal block. Plan, 0.66 mm by 0.59 mm: base strip and cover.",
            "Ansicht von vorn, auf Papier 0,66 mm mal 1,0 mm: ein heller Körper aus Grundplatte und Kappe, das dunkle Fenster mit dem hellen Zifferblatt und der Scheibenkante darunter sowie der dunklere Klemmendeckel. Seitenansicht, 0,59 mm mal 1,0 mm: die Grundplatte als 26 mm breiter Streifen an der Wand, die Kappe mit ihrer Formschräge und der zurückgesetzte Klemmenblock. Grundriss, 0,66 mm mal 0,59 mm: Grundplattenstreifen und Kappe."),
        "retained": tx("Housing, window, dial plate, disc, terminal cover.", "Gehäuse, Fenster, Zifferblatt, Scheibe, Klemmendeckel."),
        "merged": tx(
            "Base and cover are one form in the face view; the cover keeps its draft and loses its front rounding; terminal cover and terminal block are one block; dial plate, register and measuring system are one light block behind the window.",
            "Grundplatte und Kappe sind in der Ansicht von vorn eine Form; die Kappe behält ihre Formschräge und verliert ihre Vorderrundung; Klemmendeckel und Klemmenblock sind ein Block; Zifferblatt, Zählwerk und Messwerk sind ein heller Block hinter dem Fenster."),
        "omitted": tx(
            "Hanging lug (12 mm), register slot, rollers, rim mark, hub, screws and magnet: all below the 30 mm feature.",
            "Aufhängeöse (12 mm), Zählwerksschlitz, Rollen, Randmarke, Nabe, Schrauben und Magnet: alle unter dem 30-mm-Merkmal."),
        "thinElements": tx(
            "The disc edge stays 1.2 mm and is drawn with a 0.2 mm pen so that it prints as a line; the 26 mm base strip of the side view and plan carries a 0.18 mm pen.",
            "Die Scheibenkante bleibt 1,2 mm und ist mit einem 0,2-mm-Stift gezeichnet, damit sie als Linie druckt; der 26 mm breite Grundplattenstreifen in Seitenansicht und Grundriss trägt einen 0,18-mm-Stift."),
        "hatching": tx("None.", "Keine."),
        "styles": tx(
            "Housing 0.88, window 0.25, dial plate 0.93, disc 0.86, terminal cover 0.55; one body pen of 0.13 mm.",
            "Gehäuse 0,88, Fenster 0,25, Zifferblatt 0,93, Scheibe 0,86, Klemmendeckel 0,55; ein Körperstift von 0,13 mm."),
        "overlays": tx(
            "Casing, window, disc and dial plate stay four regions; the disc is axisymmetric, so its twist shows no change at this scale.",
            "Gehäuse, Fenster, Scheibe und Zifferblatt bleiben vier Regionen; die Scheibe ist rotationssymmetrisch, ihre Verdrehung zeigt bei diesem Maßstab keine Änderung."),
    },
}


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    asset["summary"] = tx(
        "A single-phase electricity meter of the Ferraris type, 0.132 m wide and 0.212 m high, seen from the room: a base with terminal block, a light cover with a window onto the dial plate, the roller register and the edge of the horizontal aluminium rotor disc, whose dark mark passes from left to right with the load, and a terminal cover below.",
        "Ein einphasiger Stromzähler der Bauart Ferraris, 0,132 m breit und 0,212 m hoch, vom Raum aus gesehen: eine Grundplatte mit Klemmenblock, eine helle Kappe mit einem Fenster auf das Zifferblatt, das Rollenzählwerk und die Kante der waagrechten Läuferscheibe aus Aluminium, deren dunkle Marke mit der Last von links nach rechts läuft, und darunter ein Klemmendeckel.")
    provenance = asset["provenance"]
    references = [r for r in provenance["references"] if r["id"] not in ("nzr-ws-ferraris", "din-43857-1-1978")]
    references.append({"id": "nzr-ws-ferraris", "citation": "NZR Nordwestdeutsche Zählerrevision, Ferraris WS single-phase two-wire meter, technical data: 120 to 145 mm by 210 to 270 mm by 100 to 150 mm, about 1.0 to 1.5 kg, mounted on a meter cross to DIN 43857, class 2, 10(40) A and 10(60) A, mechanical roller register 5/0, 6/0 or 6/1 (read 2026-11-20).", "url": "https://www.nzr.de/leistungen/messtechnik/elektrizitaet/zaehlerkreuz/ferraris-ws"})
    references.append({"id": "din-43857-1-1978", "citation": "DIN 43857-1:1978-09 Watthour meters in moulded insulation case without instrument transformers, up to 60 A rated maximum current; principal dimensions for single-phase meters (three-point fixing: a hanging lug and two lower screws)."})
    provenance["references"] = references
    provenance["notes"] = tx(
        "A single-phase electromechanical (Ferraris) meter of class 2 as it hangs on a meter board, 0.132 m by 0.212 m and 0.118 m deep with a mass of 1.35 kg (inside the ranges of the NZR data for this kind; authored estimate). From the wall forward: a 26 mm base that runs down into the terminal block, the hanging lug above it, the hollow cover with its window, and the terminal cover below. Behind the window sit the dial plate with the opening of the six-roller register (five whole kilowatt hours and the decimal), and under the plate the 84 mm aluminium disc, which lies horizontal on its upright spindle and is seen edge-on; its dark rim mark passes from left to right. The brake magnet grips the rim, the voltage electromagnet sits above and the current electromagnet below the disc. The origin is the centre of the envelope. The housing bone runs from the lower screws to the lug in the plane of the centre of mass; the rotor bone is the spindle and turns by twist. The clip shows the real turning rate of about 60 degrees per second at 1 kW slowed to 90 and 180 degrees per second. The own consumption of a meter of this class is below 2 W in the voltage circuit and about 1 W in the current circuit at full load; the model releases 2.4 W at level 1 into the dial plate over the measuring system. The fixing distances (158 mm upright, 105 mm between the lower screws) are remembered from DIN 43857-1 and were not read from the standard.",
        "Ein einphasiger elektromechanischer Zähler (Ferraris) der Klasse 2, wie er auf einer Zählertafel hängt, 0,132 m mal 0,212 m und 0,118 m tief mit einer Masse von 1,35 kg (innerhalb der Bereiche der NZR-Daten für diese Bauart; Schätzwert). Von der Wand nach vorn: eine 26 mm tiefe Grundplatte, die nach unten in den Klemmenblock läuft, darüber die Aufhängeöse, die hohle Kappe mit ihrem Fenster und darunter der Klemmendeckel. Hinter dem Fenster sitzen das Zifferblatt mit der Öffnung des sechsstelligen Rollenzählwerks (fünf ganze Kilowattstunden und die Dezimale) und unter dem Blatt die 84 mm große Aluminiumscheibe, die waagrecht auf ihrer senkrechten Achse liegt und von der Kante gesehen wird; ihre dunkle Randmarke läuft von links nach rechts. Der Bremsmagnet umgreift den Rand, der Spannungsmagnet sitzt über, der Strommagnet unter der Scheibe. Der Ursprung ist die Mitte der Hüllbox. Der Gehäuseknochen läuft von den unteren Schrauben zur Öse in der Ebene des Schwerpunkts; der Rotorknochen ist die Achse und dreht durch Verdrehung. Der Clip zeigt die echte Drehrate von etwa 60 Grad je Sekunde bei 1 kW verlangsamt auf 90 und 180 Grad je Sekunde. Der Eigenverbrauch eines Zählers dieser Klasse liegt unter 2 W im Spannungspfad und bei etwa 1 W im Strompfad bei voller Last; das Modell setzt auf Stufe 1 2,4 W in das Zifferblatt über dem Messwerk frei. Die Befestigungsmaße (158 mm senkrecht, 105 mm zwischen den unteren Schrauben) sind aus DIN 43857-1 erinnert und nicht aus der Norm gelesen.")
    asset["dimensions"] = {"heightM": 0.212, "widthM": 0.132, "depthM": 0.118, "massKg": 1.35}
    materials = [m for m in asset["materials"] if m["id"] not in ("steel-zinc", "copper-winding")]
    materials.append({"id": "steel-zinc", "title": tx("Zinc-plated steel", "Verzinkter Stahl"), "emissivity": 0.25, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "source": "authored-estimate", "reference": "incropera-2018"})
    materials.append({"id": "copper-winding", "title": tx("Enamelled copper winding", "Lackierte Kupferwicklung"), "emissivity": 0.8, "solarAbsorptivity": 0.7, "densityKgM3": 8900, "conductivityWmK": 390, "specificHeatJKgK": 385, "source": "authored-estimate", "reference": "incropera-2018"})
    asset["materials"] = materials
    asset["rig"] = {"kind": "mechanical", "root": "body", "bones": [
        {"id": "body", "parent": None, "x": 0, "y": Y0, "z": ZB, "angleDeg": 90, "title": tx("Meter base from the lower screws to the hanging lug", "Zählergrundplatte von den unteren Schrauben zur Aufhängeöse"), "lengthM": r6(kit.LUG - Y0), "joint": {"type": "fixed"}, "mass": {"kg": 1.325, "comX": r6(kit.COM - Y0), "comY": 0, "distribution": "point", "source": "authored-estimate"}},
        {"id": "disc", "parent": "body", "x": r6(YD - Y0), "y": 0, "z": r6(ZD - ZB), "angleDeg": 0, "title": tx("Rotor: upright spindle with the aluminium disc and its mark", "Läufer: senkrechte Achse mit der Aluminiumscheibe und ihrer Marke"), "lengthM": 0.062, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -720, "twistMaxDeg": 720}, "mass": {"kg": 0.025, "comX": 0.02, "comY": 0, "distribution": "point", "source": "authored-estimate"}},
    ]}
    asset["anchors"] = [
        {"id": "wall-screw-top", "bone": "body", "x": r6(kit.LUG - Y0), "y": 0, "z": r6(kit.WALL - ZB), "kind": "support", "title": tx("Upper screw through the hanging lug", "Obere Schraube durch die Aufhängeöse"), "support": "pin", "normalDeg": 0, "tiltDeg": -90, "footprintLengthM": 0.01, "footprintWidthM": 0.01},
        {"id": "wall-screw-bottom", "bone": "body", "x": 0, "y": 0, "z": r6(kit.WALL - ZB), "kind": "support", "title": tx("Pair of lower screws beside the terminal block, 105 mm apart", "Paar unterer Schrauben neben dem Klemmenblock, 105 mm auseinander"), "support": "roller", "normalDeg": 0, "tiltDeg": -90, "footprintLengthM": 0.01, "footprintWidthM": 0.115, "frictionCoefficient": 0.4},
    ]
    representations = asset["representations"]
    for scale, geometry in (("50", geometry_50()), ("100", geometry_100()), ("200", geometry_200())):
        representations[scale]["geometry"] = geometry
        representations[scale]["decisions"] = DECISIONS[scale]
    representations["500"]["reason"] = tx(
        "At 1:500 the meter is 0.26 mm by 0.42 mm on paper, well under the 3 mm a recognisable drawing needs, and its smallest drawn part, the decimal roller of the register of 7 mm, is 0.014 mm, below the 0.2 mm office-laser minimum feature; draw it at 1:200 or larger.",
        "Bei 1:500 ist der Zähler 0,26 mm mal 0,42 mm groß auf Papier, deutlich unter den 3 mm, die eine erkennbare Zeichnung braucht, und sein kleinstes gezeichnetes Teil, die Dezimalrolle des Zählwerks von 7 mm, misst 0,014 mm, unter dem Mindestmerkmal von 0,2 mm des Büro-Laserdrucks; bei 1:200 oder größer zeichnen.")
    representations["500"]["limitingFeature"] = tx("Decimal roller of the register, 7 mm", "Dezimalrolle des Zählwerks, 7 mm")
    representations["500"]["limitingSizeM"] = 0.0072
    twist = {"idle": None, "half-load": 18, "full-load": 36}
    for pose in asset["poses"]:
        pose["bones"] = {} if twist[pose["id"]] is None else {"disc": {"twistDeg": twist[pose["id"]]}}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            if "bones" in key:
                key["bones"] = {bone: {"twistDeg": offset.get("angleDeg", offset.get("twistDeg"))} for bone, offset in key["bones"].items()}
    regions = {region["id"]: region for region in asset["thermal"]["regions"]}
    for ident, area in AREAS.items():
        regions[ident]["areaM2"] = area
    regions["casing"]["title"] = tx("Cover, base and terminal cover", "Kappe, Grundplatte und Klemmendeckel")
    regions["casing"]["characteristicLengthM"] = 0.212
    regions["window"]["title"] = tx("Window pane of the cover", "Fensterscheibe der Kappe")
    regions["window"]["characteristicLengthM"] = 0.092
    regions["disc"]["title"] = tx("Aluminium rotor disc", "Läuferscheibe aus Aluminium")
    regions["disc"]["geometry"] = "horizontal-plate-up"
    regions["plate"]["title"] = tx("Dial plate over the measuring system", "Zifferblatt über dem Messwerk")
    regions["plate"]["characteristicLengthM"] = 0.064
    asset["thermal"]["assumptions"] = [tx(
        "The own consumption is lumped in the dial plate over the measuring system and scales with the load level; the real voltage coil also draws about 1.4 W at no load, which the linear model leaves out.",
        "Der Eigenverbrauch ist im Zifferblatt über dem Messwerk zusammengefasst und skaliert mit der Laststufe; die echte Spannungsspule nimmt auch ohne Last etwa 1,4 W auf, was das lineare Modell auslässt.")]
    asset["structure"]["assumptions"] = [tx(
        "The 1.35 kg meter hangs on the upper screw through the hanging lug (pin) and bears against the board at the pair of lower screws, 158 mm below (roller normal to the wall); the centre of mass lies 44 mm in front of the wall.",
        "Der 1,35 kg schwere Zähler hängt an der oberen Schraube durch die Aufhängeöse (Festlager) und stützt sich 158 mm darunter am Paar unterer Schrauben an der Tafel ab (Rollenlager senkrecht zur Wand); der Schwerpunkt liegt 44 mm vor der Wand.")]
    for pose in asset["poses"]:
        if pose["id"] == "half-load":
            pose["description"] = tx("The disc turns at half the speed of the full load; the current coils warm the dial plate.", "Die Scheibe dreht mit halber Drehzahl der vollen Last; die Stromspulen erwärmen das Zifferblatt.")
        if pose["id"] == "full-load":
            pose["description"] = tx("The disc turns at full speed and the dial plate takes the full 2.4 W of the measuring system.", "Die Scheibe dreht mit voller Drehzahl und das Zifferblatt nimmt die vollen 2,4 W des Messwerks auf.")
    asset["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print("written", OUT)


if __name__ == "__main__":
    main()
