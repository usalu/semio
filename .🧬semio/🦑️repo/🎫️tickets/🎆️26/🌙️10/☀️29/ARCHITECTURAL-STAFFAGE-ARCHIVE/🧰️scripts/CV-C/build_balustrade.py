import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CV-C")
from lib import *

AID = "glass-balustrade"
doc = load_doc(AID)
a = doc["asset"]
merge_refs(a, [EXTRA_REFS["en-iso-13786"]])
a["dimensions"].update({"heightM": 1.1, "widthM": 0.3, "depthM": 2.0, "massKg": 59.13})
a["summary"] = T(
    "A 1.1 m high laminated glass balustrade panel (2 x 10 mm glass with PVB interlayer, 21.5 mm) cantilevered from a steel floor shoe with gaskets and bolts and topped by a 42.4 mm tube handrail, shown in section; loaded by a handrail line load, a crowd impact and wind.",
    "Ein 1,1 m hohes Brüstungsfeld aus Verbundglas (2 x 10 mm Glas mit PVB-Folie, 21,5 mm), im Stahl-Bodenklemmschuh mit Dichtungen und Schrauben eingespannt und mit einem Rohrhandlauf 42,4 mm bekrönt, im Schnitt gezeigt; belastet durch Handlauf-Linienlast, Menschenanprall und Wind.")
set_material(a, "glass", title=T("Laminated glass 2 x 10 mm with PVB", "Verbundglas 2 x 10 mm mit PVB"), conductivityWmK=1.0, specificHeatJKgK=750, emissivity=0.837, reference="en-12898")
set_material(a, "steel", conductivityWmK=50, specificHeatJKgK=450)
a["thermal"] = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0,
                    "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region("panel-glass", "Laminated glass panel faces", "Verbundglasfeldflächen", "glazing", "passive", "glass", 2.2, 1.1, "vertical-plate", 0.5, [], 40000, "authored-estimate", "en-iso-10456"),
        region("clamp-steel", "Floor shoe, gaskets and bolts", "Bodenklemmschuh, Dichtungen und Schrauben", "surface", "passive", "steel", 0.1, 0.1, "horizontal-plate-up", 0.4, [], 20000, "authored-estimate", "en-iso-10456"),
        region("handrail-steel", "Tube handrail", "Rohrhandlauf", "surface", "passive", "steel", 0.13, 0.0424, "cylinder", 0.9, [], 6000, "authored-estimate", "en-iso-10456"),
    ],
    "emitters": [],
    "assumptions": [
        T("The balustrade is an open exterior element (no conditioned side): the glass reacts with the full laminate thickness of 21.5 mm (heat capacity 40 kJ/(m2 K)), the 42.4 x 2 mm tube handrail much faster (6 kJ/(m2 K)).",
          "Die Brüstung ist ein offenes Außenbauteil (keine konditionierte Seite): das Glas reagiert mit der vollen Verbundglasdicke von 21,5 mm (Wärmekapazität 40 kJ/(m² K)), der Rohrhandlauf 42,4 x 2 mm viel schneller (6 kJ/(m² K))."),
        T("Sun-exposed and shaded faces differ through the pose sunlit fractions: glass and handrail fully sunlit, the shoe half shaded.",
          "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: Glas und Handlauf voll besonnt, der Schuh halb verschattet."),
        T("Reference case: a mild sunny day, all values authored estimates.", "Referenzfall: ein milder sonniger Tag, alle Werte sind Schätzungen."),
    ],
}
bind_poses(a, {"panel-glass": (1, 1.0), "clamp-steel": (1, 0.5), "handrail-steel": (1, 1.0)})

D = Draw(a["rig"]["bones"])
P = "panel"
ALL = ["panel-glass", "clamp-steel", "handrail-steel"]
gl = 0.01075

r500 = unavailable(500,
                   ("A balustrade section is 0.30 x 1.10 m, i.e. 0.6 x 2.2 mm at 1:500; the 21.5 mm glass is 0.04 mm and the 42 mm handrail 0.08 mm thick, far below every printable stroke; a line would overstate the glass by a factor of three.",
                    "Ein Brüstungsschnitt ist 0,30 x 1,10 m groß, bei 1:500 also 0,6 x 2,2 mm; das 21,5-mm-Glas ist 0,04 mm und der 42-mm-Handlauf 0,08 mm dick, weit unter jeder druckbaren Strichstärke; eine Linie würde das Glas um das Dreifache überzeichnen."),
                   ("Laminated glass thickness 21.5 mm", "Verbundglasdicke 21,5 mm"), 0.0215,
                   [{"kind": "scale", "denominator": 200}, {"kind": "symbol", "symbol": "balustrade-plan"}])

r200 = rep(200, [
    D.rect("shoe", P, "body", 2, True, 0.35, -0.15, 0, 0.15, 0.1, region="clamp-steel", material="steel", penMm=0.1),
    D.line("glass", P, "glazing", 3, 0.92, [(0, 0.1), (0, 1.08)], region="panel-glass", material="glass", penMm=0.1),
    D.circ("handrail", P, "body", 4, True, 0.35, 0.0, 1.079, 0.0212, region="handrail-steel", material="steel", penMm=0.1)],
    dec(("A glass line standing on a small steel shoe with a round handrail on top; 5.5 mm tall on paper.",
         "Eine Glaslinie auf einem kleinen Stahlschuh mit rundem Handlauf darauf; 5,5 mm hoch auf dem Papier."),
        ("Shoe (1.5 x 0.5 mm), the glass line (4.9 mm) and the handrail circle (0.21 mm).", "Schuh (1,5 x 0,5 mm), die Glaslinie (4,9 mm) und der Handlaufkreis (0,21 mm)."),
        ("The two glass panes with the PVB layer are one glass line; gaskets and bolts are merged into the shoe.", "Die zwei Glasscheiben mit der PVB-Schicht sind eine Glaslinie; Dichtungen und Schrauben sind im Schuh zusammengefasst."),
        ("Laminate layers, gaskets, bolts, the shoe slot and the handrail wall.", "Verbundschichten, Dichtungen, Schrauben, der Schuhschlitz und die Handlaufwand."),
        ("The 21.5 mm glass is a 0.1 mm pen line (20 mm); the 42.4 mm handrail is a filled circle with a 0.1 mm outline (about 0.1 mm larger than real).",
         "Das 21,5-mm-Glas ist eine Linie mit 0,1 mm Feder (20 mm); der 42,4-mm-Handlauf ist ein gefüllter Kreis mit 0,1 mm Kontur (etwa 0,1 mm größer als real)."),
        ("No hatch.", "Keine Schraffur."),
        ("Dark shoe and handrail, a thin light glass line.", "Dunkler Schuh und Handlauf, eine dünne helle Glaslinie."),
        ("Shoe, glass line and handrail are separate overlay cells; the loaded glass line shows the load path from the handrail to the shoe.",
         "Schuh, Glaslinie und Handlauf sind getrennte Überlagerungszellen; die belastete Glaslinie zeigt den Lastpfad vom Handlauf zum Schuh.")))

r100 = rep(100, [
    D.rect("shoe", P, "body", 2, True, 0.35, -0.15, 0, 0.15, 0.1, region="clamp-steel", material="steel"),
    D.rect("glass", P, "glazing", 3, True, 0.92, -gl, 0.1, gl, 1.08, region="panel-glass", material="glass", penMm=0.1),
    D.circ("handrail", P, "body", 4, True, 0.35, 0.0, 1.079, 0.0212, region="handrail-steel", material="steel")],
    dec(("A light glass strip rising from a dark steel shoe to a round handrail.", "Ein heller Glasstreifen, der aus einem dunklen Stahlschuh zu einem runden Handlauf aufsteigt."),
        ("Shoe (3 x 1 mm), glass strip (0.22 x 9.8 mm) and handrail (0.42 mm).", "Schuh (3 x 1 mm), Glasstreifen (0,22 x 9,8 mm) und Handlauf (0,42 mm)."),
        ("Laminate layers and gaskets are merged into the glass strip and the shoe.", "Verbundschichten und Dichtungen sind im Glasstreifen und im Schuh zusammengefasst."),
        ("Bolts (14 mm, 0.14 mm), the shoe slot and the PVB layer.", "Schrauben (14 mm, 0,14 mm), der Schuhschlitz und die PVB-Schicht."),
        ("The glass strip is 0.215 mm wide with a 0.1 mm outline, so it prints 0.315 mm wide (31.5 mm real vs 21.5 mm).", "Der Glasstreifen ist 0,215 mm breit mit 0,1 mm Kontur und druckt daher 0,315 mm breit (31,5 mm real gegen 21,5 mm)."),
        ("No hatch: the shoe is 1 mm tall, the glass 0.2 mm wide.", "Keine Schraffur: der Schuh ist 1 mm hoch, das Glas 0,2 mm breit."),
        ("Outline 0.25 mm; dark steel, light glass.", "Kontur 0,25 mm; dunkler Stahl, helles Glas."),
        ("Shoe, glass and handrail are separate overlay cells.", "Schuh, Glas und Handlauf sind getrennte Überlagerungszellen.")))

h50 = [H("steel", "Steel, cut", "Stahl, geschnitten", "lines", 45, 0.6, 0.1)]
shoe = [(-0.15, 0), (0.15, 0), (0.15, 0.1), (0.02, 0.1), (0.02, 0.02), (-0.02, 0.02), (-0.02, 0.1), (-0.15, 0.1)]
r50 = rep(50, [
    D.poly("shoe", P, "body", 2, True, 0.35, shoe, region="clamp-steel", material="steel", hatch="steel"),
    D.rect("gasket-a", P, "body", 2.5, True, 0.2, -0.02, 0.02, -gl, 0.1, region="clamp-steel", material="steel", penMm=0.1),
    D.rect("gasket-b", P, "body", 2.5, True, 0.2, gl, 0.02, 0.02, 0.1, region="clamp-steel", material="steel", penMm=0.1),
    D.rect("glass", P, "glazing", 3, True, 0.92, -gl, 0.04, gl, 1.08, region="panel-glass", material="glass", penMm=0.1),
    D.line("pvb-layer", P, "detail", 3.1, 0.3, [(0, 0.1), (0, 1.0)], region="panel-glass", penMm=0.1),
    D.circ("bolt-a", P, "detail", 3, True, 0.15, -0.1, 0.05, 0.007, region="clamp-steel"),
    D.circ("bolt-b", P, "detail", 3, True, 0.15, 0.1, 0.05, 0.007, region="clamp-steel"),
    D.circ("handrail", P, "body", 4, True, 0.35, 0.0, 1.079, 0.0212, region="handrail-steel", material="steel", hatch="steel")],
    dec(("The section of a clamped glass balustrade: a hatched steel shoe with a slot, two gaskets around the laminated glass, bolts, the PVB layer and a hatched tube handrail.",
         "Der Schnitt einer eingespannten Glasbrüstung: ein schraffierter Stahlschuh mit Schlitz, zwei Dichtungen um das Verbundglas, Schrauben, die PVB-Schicht und ein schraffierter Rohrhandlauf."),
    ("Shoe (6 x 2 mm) with its 0.8 mm slot, gaskets (0.18 x 1.6 mm), glass (0.43 x 21 mm) running into the slot, PVB line, two bolts (0.28 mm), handrail (0.85 mm).",
     "Schuh (6 x 2 mm) mit 0,8 mm Schlitz, Dichtungen (0,18 x 1,6 mm), Glas (0,43 x 21 mm) bis in den Schlitz, PVB-Linie, zwei Schrauben (0,28 mm), Handlauf (0,85 mm)."),
    ("The two 10 mm glass panes are one glass body; the PVB layer is one line.", "Die zwei 10-mm-Glasscheiben sind ein Glaskörper; die PVB-Schicht ist eine Linie."),
    ("The 2 mm tube wall (0.04 mm), the setting blocks and the base anchors.", "Die 2-mm-Rohrwand (0,04 mm), die Klotzungen und die Fußanker."),
    ("The 1.5 mm PVB is a 0.1 mm line (5 mm real); the handrail wall is not resolved and the tube is drawn solid with hatch.", "Das 1,5-mm-PVB ist eine 0,1-mm-Linie (5 mm real); die Handlaufwand wird nicht aufgelöst, das Rohr ist voll mit Schraffur gezeichnet."),
    ("Cut steel is hatched at 45 degrees, 0.6 mm (DIN ISO 128-50); glass is not hatched, which is the convention for glass.", "Geschnittener Stahl ist 45° schraffiert, 0,6 mm (DIN ISO 128-50); Glas wird nicht schraffiert, das ist die Konvention für Glas."),
    ("Outline 0.35 mm, glass 0.13 mm, PVB 0.18 mm, hatch 0.1 mm.", "Kontur 0,35 mm, Glas 0,13 mm, PVB 0,18 mm, Schraffur 0,1 mm."),
    ("Shoe group, glass and handrail are separate overlay cells; the PVB line is bound to the glass.", "Schuhgruppe, Glas und Handlauf sind getrennte Überlagerungszellen; die PVB-Linie ist an das Glas gebunden.")), hatches=h50)


def pl(id, pts, closed=True, filled=False, tone=0.5, role="body", **kw):
    d = {"id": id, "bone": "panel", "role": role, "z": 1, "filled": filled, "tone": tone, "kind": "poly", "points": [list(p) for p in pts], "closed": closed}
    d.update(kw)
    return d


plan = [pl("handrail", [(0, -0.0212), (2.0, -0.0212), (2.0, 0.0212), (0, 0.0212)], True, True, 0.35, region="handrail-steel"),
        pl("shoe-outline", [(0, -0.15), (2.0, -0.15), (2.0, 0.15), (0, 0.15)], True, False, 0.5, role="detail", region="clamp-steel"),
        pl("glass-line", [(0, 0), (2.0, 0)], False, False, 0.9, role="glazing", region="panel-glass")]
a["symbols"] = [{
    "id": "balustrade-plan", "title": T("Glass balustrade in plan", "Glasbrüstung im Grundriss"),
    "description": T("The plan sign of a glass balustrade panel seen from above: the tube handrail as a long narrow bar over the glass line, the floor shoe as a dashed-style outline behind it.",
                    "Das Grundrisszeichen eines Glasbrüstungsfeldes von oben: der Rohrhandlauf als langer schmaler Balken über der Glaslinie, der Bodenschuh als Umriss dahinter."),
    "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100, 50],
    "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.1, "ground": 0.25},
    "hatches": [],
    "geometry": {"elevation": [], "plan": plan},
    "dimensionalNote": T("Real size: panel length 2.00 m; handrail 42.4 mm wide, shoe 0.30 m deep. At 1:500 the sign is 4 mm long and the handrail is drawn by its pen.",
                         "Echte Größe: Feldlänge 2,00 m; Handlauf 42,4 mm breit, Schuh 0,30 m tief. Bei 1:500 ist das Zeichen 4 mm lang und der Handlauf entsteht durch die Federbreite."),
}]
a["representations"] = {"500": r500, "200": r200, "100": r100, "50": r50}
a["presentation"]["viewpoints"] = ["elevation"]
out = finish(a)
doc["asset"] = out
print(AID)
validate(out)
save_doc(AID, doc)
