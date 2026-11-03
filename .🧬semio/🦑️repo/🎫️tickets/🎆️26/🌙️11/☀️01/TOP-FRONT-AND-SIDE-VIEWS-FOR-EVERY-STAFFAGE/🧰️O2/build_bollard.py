import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "retractable-bollard"
doc = load(A)

STEEL, FILM, AMBER, IRON, HYD = "steel-grey", "retro-film", "pc-amber", "cast-iron-frame", "steel-hyd"
CAP500 = "M -0.1 -0.13 L -0.1 -0.05 C -0.1 -0.01 -0.05 0 0 0 C 0.05 0 0.1 -0.01 0.1 -0.05 L 0.1 -0.13 Z"
CAP100 = "M -0.1 -0.07 C -0.1 -0.02 -0.05 0 0 0 C 0.05 0 0.1 -0.02 0.1 -0.07 Z"


def build(scale):
    f, p = [], []
    if scale == 500:
        f += [
            rect(-0.1, -1.15, 0.2, 1.06, id="shaft", bone="shaft", role="body", z=3.5, tone=0.5, regions=["shaft-steel", "reflector-band"], material=STEEL),
            path(CAP500, id="cap", bone="shaft", role="body", z=3.6, tone=0.5, regions=["shaft-steel", "beacon-led"], material=STEEL),
            poly([(-0.1, 0), (-0.1, -1.25), (0.1, -1.25), (0.1, 0)], closed=False, id="sleeve", bone="sleeve", role="body", z=2.2, tone=0.3, filled=False, region="sleeve-steel", pen=0.13),
            rect(-0.21, -0.11, 0.11, 0.13, id="frame-l", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            rect(0.1, -0.11, 0.11, 0.13, id="frame-r", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            rect(-0.05, -0.95, 0.26, 0.4, id="power-unit", bone="hydraulic", role="body", z=3, tone=0.5, region="hydraulic-unit", material=HYD),
        ]
        p += [
            rect(-0.21, -0.21, 0.42, 0.42, rad=0.02, id="plan-frame", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            rect(0.1, -0.05, 0.26, 0.26, id="power-unit", bone="hydraulic", role="body", z=2, tone=0.8, region="hydraulic-unit", material=HYD),
            ellipse(0, 0, 0.1, 0.1, id="plan-cap", bone="shaft", role="body", z=3.6, tone=0.5, regions=["shaft-steel", "beacon-led"], material=STEEL),
        ]
    elif scale == 200:
        f += [
            rect(-0.1, -1.15, 0.2, 0.9, id="shaft", bone="shaft", role="body", z=3.5, tone=0.5, region="shaft-steel", material=STEEL, pen=0.256),
            rect(-0.1, -0.25, 0.2, 0.12, id="reflector", bone="shaft", role="body", z=3.55, tone=0.08, region="reflector-band", material=FILM, pen=0.332),
            path(CAP500, id="cap", bone="shaft", role="body", z=3.6, tone=0.5, regions=["shaft-steel", "beacon-led"], material=STEEL),
            poly([(-0.1, 0), (-0.1, -1.25), (0.1, -1.25), (0.1, 0)], closed=False, id="sleeve", bone="sleeve", role="body", z=2.2, tone=0.3, filled=False, region="sleeve-steel", pen=0.28),
            rect(-0.21, -0.08, 0.11, 0.095, id="frame-l", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON, pen=0.256),
            rect(0.1, -0.08, 0.11, 0.095, id="frame-r", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON, pen=0.256),
            rect(-0.05, -0.95, 0.26, 0.4, id="power-unit", bone="hydraulic", role="body", z=3, tone=0.5, region="hydraulic-unit", material=HYD, pen=0.282),
            rect(-0.06, -1.25, 0.12, 0.55, id="ram", bone="hydraulic", role="body", z=2.4, tone=0.7, region="hydraulic-unit", material=HYD, pen=0.356),
        ]
        p += [
            rect(-0.21, -0.21, 0.42, 0.42, rad=0.02, id="plan-frame", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON, pen=0.256),
            rect(0.1, -0.05, 0.26, 0.26, id="power-unit", bone="hydraulic", role="body", z=2, tone=0.8, region="hydraulic-unit", material=HYD, pen=0.282),
            ellipse(0, 0, 0.1, 0.1, id="plan-cap", bone="shaft", role="body", z=3.6, tone=0.5, regions=["shaft-steel", "beacon-led"], material=STEEL, pen=0.256),
        ]
    elif scale == 100:
        f += [
            rect(-0.1, -1.15, 0.2, 0.91, id="shaft", bone="shaft", role="body", z=3.5, tone=0.5, region="shaft-steel", material=STEEL),
            rect(-0.1, -0.24, 0.2, 0.11, id="reflector", bone="shaft", role="body", z=3.55, tone=0.08, region="reflector-band", material=FILM),
            rect(-0.1, -0.13, 0.2, 0.06, id="beacon", bone="shaft", role="body", z=3.6, tone=0.7, region="beacon-led", material="pc-amber"),
            path(CAP100, id="cap", bone="shaft", role="body", z=3.65, tone=0.5, region="shaft-steel", material=STEEL),
            poly([(-0.15, 0), (-0.15, -1.25), (0.15, -1.25), (0.15, 0)], closed=False, id="sleeve", bone="sleeve", role="body", z=2.2, tone=0.3, filled=False, region="sleeve-steel", pen=0.13),
            rect(-0.2, -0.07, 0.055, 0.085, id="frame-l", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            rect(0.145, -0.07, 0.055, 0.085, id="frame-r", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            rect(-0.01, -0.95, 0.21, 0.4, id="power-unit", bone="hydraulic", role="body", z=3, tone=0.5, region="hydraulic-unit", material=HYD),
            poly([(-0.01, -0.8), (0.2, -0.8)], closed=False, id="motor-seam", bone="hydraulic", role="detail", z=3.2, tone=0.3, filled=False, region="hydraulic-unit"),
            rect(0.025, -0.675, 0.14, 0.07, id="motor", bone="hydraulic", role="detail", z=3.3, tone=0.8, region="hydraulic-unit"),
            rect(-0.06, -1.25, 0.12, 0.55, id="ram", bone="hydraulic", role="body", z=2.4, tone=0.7, region="hydraulic-unit", material=HYD),
        ]
        p += [
            rect(-0.2, -0.2, 0.4, 0.4, rad=0.02, id="plan-frame", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            ellipse(0, 0, 0.15, 0.15, id="plan-sleeve", bone="sleeve", role="body", z=3.2, tone=0.3, region="sleeve-steel"),
            rect(0.15, -0.01, 0.21, 0.21, id="power-unit", bone="hydraulic", role="body", z=2, tone=0.8, region="hydraulic-unit", material=HYD),
            rect(0.22, 0.025, 0.07, 0.14, id="motor", bone="hydraulic", role="detail", z=2.2, tone=0.5, region="hydraulic-unit"),
            ellipse(0, 0, 0.1, 0.1, id="plan-cap", bone="shaft", role="body", z=3.6, tone=0.5, region="shaft-steel", material=STEEL),
        ]
    else:
        f += [
            rect(-0.1, -1.15, 0.2, 0.91, id="shaft", bone="shaft", role="body", z=3.5, tone=0.5, region="shaft-steel", material=STEEL),
            poly([(-0.065, -1.1), (-0.065, -0.28)], closed=False, id="shaft-highlight", bone="shaft", role="texture", z=3.7, tone=0.3, filled=False, region="shaft-steel"),
            rect(-0.1, -0.24, 0.2, 0.11, id="reflector", bone="shaft", role="body", z=3.55, tone=0.08, region="reflector-band", material=FILM, hatch="chevron"),
            rect(-0.1, -0.13, 0.2, 0.06, id="beacon", bone="shaft", role="body", z=3.6, tone=0.7, region="beacon-led", material=AMBER),
            ellipse(-0.06, -0.1, 0.01, 0.01, id="led-0", bone="shaft", role="detail", z=3.7, tone=0.1, region="beacon-led"),
            ellipse(0, -0.1, 0.01, 0.01, id="led-1", bone="shaft", role="detail", z=3.7, tone=0.1, region="beacon-led"),
            ellipse(0.06, -0.1, 0.01, 0.01, id="led-2", bone="shaft", role="detail", z=3.7, tone=0.1, region="beacon-led"),
            path(CAP100, id="cap", bone="shaft", role="body", z=3.65, tone=0.5, region="shaft-steel", material=STEEL),
            poly([(-0.09, -0.05), (0.09, -0.05)], closed=False, id="cap-seam", bone="shaft", role="texture", z=3.8, tone=0.3, filled=False, region="shaft-steel"),
            poly([(-0.15, 0), (-0.15, -1.25), (0.15, -1.25), (0.15, 0)], closed=False, id="sleeve", bone="sleeve", role="body", z=2.2, tone=0.3, filled=False, region="sleeve-steel", pen=0.25),
            rect(-0.2, -0.07, 0.055, 0.085, id="frame-l", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            rect(0.145, -0.07, 0.055, 0.085, id="frame-r", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            ellipse(-0.175, -0.02, 0.01, 0.01, id="frame-bolt-0", bone="sleeve", role="detail", z=3.1, tone=0.2, region="cover-frame"),
            ellipse(0.175, -0.02, 0.01, 0.01, id="frame-bolt-1", bone="sleeve", role="detail", z=3.1, tone=0.2, region="cover-frame"),
            poly([(-0.1, -0.015), (-0.145, -0.015)], closed=False, id="brush-seal-l", bone="sleeve", role="texture", z=3.1, tone=0.3, filled=False, region="cover-frame"),
            poly([(0.1, -0.015), (0.145, -0.015)], closed=False, id="brush-seal-r", bone="sleeve", role="texture", z=3.1, tone=0.3, filled=False, region="cover-frame"),
            rect(-0.01, -0.95, 0.21, 0.4, rad=0.01, id="power-unit", bone="hydraulic", role="body", z=3, tone=0.5, region="hydraulic-unit", material=HYD),
            poly([(-0.01, -0.8), (0.2, -0.8)], closed=False, id="motor-seam", bone="hydraulic", role="detail", z=3.2, tone=0.3, filled=False, region="hydraulic-unit"),
            rect(0.025, -0.675, 0.14, 0.07, rad=0.01, id="motor", bone="hydraulic", role="detail", z=3.3, tone=0.8, region="hydraulic-unit"),
            ellipse(0.03, -0.7, 0.015, 0.015, id="hose-port", bone="hydraulic", role="detail", z=3.3, tone=0.3, filled=False, region="hydraulic-unit"),
            rect(-0.06, -1.25, 0.12, 0.55, id="ram", bone="hydraulic", role="body", z=2.4, tone=0.7, region="hydraulic-unit", material=HYD),
            poly([(0, -0.7), (0, -0.45)], closed=False, id="ram-rod", bone="hydraulic", role="detail", z=2.5, tone=0.3, filled=False, region="hydraulic-unit"),
        ]
        p += [
            rect(-0.2, -0.2, 0.4, 0.4, rad=0.02, id="plan-frame", bone="sleeve", role="body", z=3, tone=0.6, region="cover-frame", material=IRON),
            ellipse(-0.175, -0.175, 0.01, 0.01, id="frame-bolt-0", bone="sleeve", role="detail", z=3.1, tone=0.2, region="cover-frame"),
            ellipse(0.175, -0.175, 0.01, 0.01, id="frame-bolt-1", bone="sleeve", role="detail", z=3.1, tone=0.2, region="cover-frame"),
            ellipse(-0.175, 0.175, 0.01, 0.01, id="frame-bolt-2", bone="sleeve", role="detail", z=3.1, tone=0.2, region="cover-frame"),
            ellipse(0.175, 0.175, 0.01, 0.01, id="frame-bolt-3", bone="sleeve", role="detail", z=3.1, tone=0.2, region="cover-frame"),
            ellipse(0, 0, 0.15, 0.15, id="plan-sleeve", bone="sleeve", role="body", z=3.2, tone=0.3, region="sleeve-steel"),
            ellipse(0, 0, 0.125, 0.125, id="plan-brush-seal", bone="sleeve", role="texture", z=3.3, tone=0.3, filled=False, region="cover-frame"),
            rect(0.15, -0.01, 0.21, 0.21, rad=0.01, id="power-unit", bone="hydraulic", role="body", z=2, tone=0.8, region="hydraulic-unit", material=HYD),
            rect(0.22, 0.025, 0.07, 0.14, rad=0.01, id="motor", bone="hydraulic", role="detail", z=2.2, tone=0.5, region="hydraulic-unit"),
            poly([(0.2, 0.0), (0.2, 0.19)], closed=False, id="unit-lid-seam", bone="hydraulic", role="detail", z=2.1, tone=0.3, filled=False, region="hydraulic-unit"),
            ellipse(0, 0, 0.1, 0.1, id="plan-cap", bone="shaft", role="body", z=3.6, tone=0.5, region="shaft-steel", material=STEEL),
            ellipse(0, 0, 0.085, 0.085, id="plan-cap-seam", bone="shaft", role="texture", z=3.8, tone=0.3, filled=False, region="shaft-steel"),
        ]
    return f, p


T = {
    500: (
        ("Front view: the shaft with its domed cap as one 0.4 mm column in front of a plain sleeve outline, two cover-frame blocks either side, and the power unit offset to the left of the axis so it is not hidden behind the shaft. Plan view: the 0.84 mm cover frame as one square, the shaft cap as one disc and the buried power unit as a light block beside it.",
         "Vorderansicht: der Schaft mit gewölbter Kappe als eine 0,4 mm breite Säule vor der schlichten Hülsenkontur, beidseits ein Abdeckrahmenblock und die Antriebseinheit links der Achse, damit der Schaft sie nicht verdeckt. Draufsicht: der 0,84 mm breite Abdeckrahmen als ein Quadrat, die Schaftkappe als eine Scheibe und die eingegrabene Antriebseinheit als helle Fläche daneben."),
        ("Front view: shaft, cap, sleeve outline, two frame blocks and the power unit. Plan view: frame square, cap disc, power unit.",
         "Vorderansicht: Schaft, Kappe, Hülsenkontur, zwei Rahmenblöcke und die Antriebseinheit. Draufsicht: Rahmenquadrat, Kappenscheibe, Antriebseinheit."),
        ("Front view: the white band and the cap share the shaft column; the four frame blocks of the plan are one square. Plan view: shaft cap and beacon read as one disc.",
         "Vorderansicht: Leuchtband und Kappe teilen sich die Schaftsäule; die vier Rahmenblöcke des Grundrisses sind ein Quadrat. Draufsicht: Schaftkappe und Leuchtring sind eine Scheibe."),
        ("Front and plan view: the ram, hose and motor of the buried unit are not drawn; from above the sleeve ring is hidden by the cap disc.",
         "Vorder- und Draufsicht: Zylinder, Schlauch und Motor der eingegrabenen Einheit sind nicht gezeichnet; von oben verdeckt die Kappenscheibe den Hülsenring."),
    ),
    200: (
        ("Front view: shaft, white reflector band and domed cap rise from the sleeve outline as in the side view, the cover-frame blocks sit at the ground line, and the left-offset power unit hangs beside the ram. Plan view: the square cover frame, the shaft cap disc and the buried power unit; the shaft only travels vertically, so the plan stays still while the bollard rises.",
         "Vorderansicht: Schaft, weißes Reflexband und gewölbte Kappe steigen wie in der Seitenansicht aus der Hülsenkontur, die Rahmenblöcke liegen in Geländehöhe, und die nach links versetzte Antriebseinheit hängt neben dem Zylinder. Draufsicht: der quadratische Abdeckrahmen, die Kappenscheibe des Schafts und die eingegrabene Antriebseinheit; der Schaft bewegt sich nur senkrecht, der Grundriss bleibt beim Ausfahren deshalb unverändert."),
        ("Front view: reflector band, ram, power unit. Plan view: frame, cap, power unit.",
         "Vorderansicht: Reflexband, Zylinder, Antriebseinheit. Draufsicht: Rahmen, Kappe, Antriebseinheit."),
        ("Front view: the two frame blocks are the section through one square cover frame. Plan view: sleeve and shaft share one disc.",
         "Vorderansicht: die zwei Rahmenblöcke sind der Schnitt durch einen quadratischen Abdeckrahmen. Draufsicht: Hülse und Schaft teilen sich eine Scheibe."),
        ("Front and plan view: no hose, no motor and no seam lines.",
         "Vorder- und Draufsicht: kein Schlauch, kein Motor und keine Fugenlinien."),
    ),
    100: (
        ("Front view: shaft with white band, amber beacon band and domed cap over the 0.30 m sleeve outline, frame blocks at ground level, and the power unit with motor seam and the end-on motor, drawn left of the axis in front of the ram. Plan view: the 0.40 m square cover frame with its round sleeve opening, the 0.20 m cap disc and the buried power unit with its motor beside the frame.",
         "Vorderansicht: der Schaft mit weißem Band, bernsteinfarbenem Leuchtband und gewölbter Kappe über der 0,30 m breiten Hülsenkontur, Rahmenblöcke in Geländehöhe und die Antriebseinheit mit Motornaht und quer liegendem Motor, links der Achse vor dem Zylinder. Draufsicht: der quadratische Abdeckrahmen von 0,40 m mit der runden Hülsenöffnung, die Kappenscheibe von 0,20 m und die eingegrabene Antriebseinheit mit Motor neben dem Rahmen."),
        ("Front view: reflector, beacon band, cap, motor seam, motor. Plan view: sleeve opening, motor.",
         "Vorderansicht: Reflexband, Leuchtband, Kappe, Motornaht, Motor. Draufsicht: Hülsenöffnung, Motor."),
        ("Plan view: the cap and the beacon band are one disc; ram and hose lie under the cover frame.",
         "Draufsicht: Kappe und Leuchtband sind eine Scheibe; Zylinder und Schlauch liegen unter dem Abdeckrahmen."),
        ("Front view: hose is not drawn (it runs in the viewing direction). Plan view: ram and hose are hidden below the frame.",
         "Vorderansicht: der Schlauch ist nicht gezeichnet (er verläuft in Blickrichtung). Draufsicht: Zylinder und Schlauch liegen verdeckt unter dem Rahmen."),
    ),
    50: (
        ("Front view: shaft with highlight line, chevron-hatched reflector band, beacon band with three LEDs, domed cap with seam, brush seals and bolts on the frame blocks, the 0.30 m sleeve outline, and the left-offset power unit with motor seam, end-on motor, hose port, ram and ram rod. Plan view: the 0.40 m cover frame with four bolts, the round sleeve opening with the brush-seal ring, the 0.20 m cap disc with its seam ring, and the buried power unit with its motor and lid seam.",
         "Vorderansicht: der Schaft mit Glanzlinie, chevronschraffiertem Reflexband, Leuchtband mit drei LEDs, gewölbter Kappe mit Fuge, Bürstendichtungen und Schrauben an den Rahmenblöcken, die 0,30 m breite Hülsenkontur und die nach links versetzte Antriebseinheit mit Motornaht, quer liegendem Motor, Schlauchanschluss, Zylinder und Kolbenstange. Draufsicht: der Abdeckrahmen von 0,40 m mit vier Schrauben, die runde Hülsenöffnung mit dem Bürstendichtungsring, die Kappenscheibe von 0,20 m mit Fugenring und die eingegrabene Antriebseinheit mit Motor und Deckelfuge."),
        ("Front view: LEDs, seals, bolts, motor, hose port, ram rod. Plan view: four bolts, brush-seal ring, cap seam, unit lid seam.",
         "Vorderansicht: LEDs, Dichtungen, Schrauben, Motor, Schlauchanschluss, Kolbenstange. Draufsicht: vier Schrauben, Bürstendichtungsring, Kappenfuge, Deckelfuge der Einheit."),
        ("Plan view: the cap and the beacon band are one disc; the ram lies hidden below the frame.",
         "Draufsicht: Kappe und Leuchtband sind eine Scheibe; der Zylinder liegt verdeckt unter dem Rahmen."),
        ("Front view: the hose is seen end-on as a port ring. Plan view: ram, hose and LEDs are hidden.",
         "Vorderansicht: der Schlauch erscheint stirnseitig als Anschlussring. Draufsicht: Zylinder, Schlauch und LEDs sind verdeckt."),
    ),
}

for scale in (500, 200, 100, 50):
    f, p = build(scale)
    s, ret, mer, omi = T[scale]
    install(doc, scale, f, p, {"silhouette": s, "retained": ret, "merged": mer, "omitted": omi})
finish(doc)
save(A, doc)
print("[DEBUG] bollard written")
