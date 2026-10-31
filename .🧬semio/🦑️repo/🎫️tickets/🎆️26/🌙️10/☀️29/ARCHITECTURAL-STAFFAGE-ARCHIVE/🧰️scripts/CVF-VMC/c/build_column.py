import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CVF-VMC/c")
from lib import *

AID = "steel-column"
doc = load_doc(AID)
a = doc["asset"]
merge_refs(a, [EXTRA_REFS["en-iso-13786"]])
a["dimensions"].update({"heightM": 3.0, "widthM": 0.3, "depthM": 0.2, "massKg": 183.93})
a["summary"] = T(
    "A 3.0 m HEB 200 steel column (S235) on a welded base plate with stiffeners and anchor bolts, fixed at the footing and capped by a head plate; loaded axially, eccentrically and by lateral wind. Seen onto the web, so the sunlit flange edges and the self-shaded web chambers read separately.",
    "Eine 3,0 m hohe Stahlstütze HEB 200 (S235) auf geschweißter Fußplatte mit Rippen und Ankerschrauben, am Fundament eingespannt und mit Kopfplatte; axial, außermittig und unter Windlast belastet. Die Ansicht blickt auf den Steg, so sind die besonnten Flanschkanten und die selbstverschatteten Stegkammern getrennt lesbar.")
set_material(a, "steel", conductivityWmK=50, specificHeatJKgK=450, emissivity=0.85)
a["thermal"] = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0,
                    "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region("column-flange", "Flange outer faces and edges", "Flanschaußenseiten und -kanten", "surface", "passive", "steel", 1.38, 3.0, "vertical-plate", 0.6, [], 53000, "authored-estimate", "en-iso-10456"),
        region("column-web", "Web and inner flange faces (chambers)", "Steg und Flanschinnenseiten (Kammern)", "surface", "passive", "steel", 2.13, 0.17, "vertical-plate", 0.25, [], 32000, "authored-estimate", "en-iso-10456"),
        region("base-plate", "Base plate, stiffeners and anchor nuts", "Fußplatte, Rippen und Ankermuttern", "surface", "passive", "steel", 0.12, 0.3, "horizontal-plate-up", 0.3, [], 70000, "authored-estimate", "en-iso-10456"),
        region("cap-plate", "Head plate", "Kopfplatte", "surface", "passive", "steel", 0.1, 0.1, "horizontal-plate-up", 1.0, [], 70000, "authored-estimate", "en-iso-10456"),
    ],
    "emitters": [],
    "assumptions": [
        T("HEB 200 has about 1.15 m2 of surface per metre: flange outer faces and edges 0.46 m2/m, web and inner flange faces 0.71 m2/m; the web chambers see little sky and sun (view factor 0.25), so they lag the flanges.",
          "HEB 200 hat etwa 1,15 m² Oberfläche je Meter: Flanschaußenseiten und -kanten 0,46 m²/m, Steg und Flanschinnenseiten 0,71 m²/m; die Stegkammern sehen wenig Himmel und Sonne (Sichtfaktor 0,25) und hinken den Flanschen nach."),
        T("Steel conducts well, so the areal heat capacity uses the full plate thickness (flange 15 mm, web 9 mm, plates 20 mm) with c = 450 J/(kg K).",
          "Stahl leitet gut, deshalb gilt für die flächenbezogene Wärmekapazität die volle Blechdicke (Flansch 15 mm, Steg 9 mm, Platten 20 mm) mit c = 450 J/(kg K)."),
        T("Sun-exposed and shaded faces differ through the pose sunlit fractions: flange faces 60 %, web chambers 30 %, head plate fully, base plate 10 %.",
          "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: Flanschflächen 60 %, Stegkammern 30 %, Kopfplatte voll, Fußplatte 10 %."),
        T("Reference case: a mild sunny day, all values authored estimates.", "Referenzfall: ein milder sonniger Tag, alle Werte sind Schätzungen."),
    ],
}
bind_poses(a, {"column-flange": (1, 0.6), "column-web": (1, 0.3), "base-plate": (1, 0.1), "cap-plate": (1, 1.0)})

D = Draw(a["rig"]["bones"])
C = "column"
ALL = ["column-flange", "column-web", "base-plate", "cap-plate"]
BAR = ["column-flange", "column-web"]
env_pts = [(-0.15, 0), (0.15, 0), (0.15, 0.02), (0.1, 0.02), (0.1, 2.98), (0.12, 2.98), (0.12, 3.0), (-0.12, 3.0), (-0.12, 2.98), (-0.1, 2.98), (-0.1, 0.02), (-0.15, 0.02)]

r500 = rep(500, [D.poly("column", C, "body", 1, True, 0.4, env_pts, regions=ALL, material="steel")], dec(
    ("One composed vertical bar 0.20 m wide and 3.00 m high (0.4 x 6 mm) with the 20 mm base and head plates as step-outs of the outline.",
     "Ein zusammengesetzter senkrechter Balken, 0,20 m breit und 3,00 m hoch (0,4 x 6 mm), mit den 20-mm-Fuß- und Kopfplatten als Absätzen der Kontur."),
    ("Height, flange width 0.20 m and the plate widths 0.30 m and 0.24 m.", "Höhe, Flanschbreite 0,20 m und die Plattenbreiten 0,30 m und 0,24 m."),
    ("Flanges, web, plates, stiffeners and nuts are one solid.", "Flansche, Steg, Platten, Rippen und Muttern sind ein Körper."),
    ("Flange edge lines, fillets, stiffeners, anchor nuts and bolts, splice line.", "Flanschkanten, Hohlkehlen, Rippen, Ankermuttern und -bolzen, Stoßlinie."),
    ("The stiffeners (0.12 m) and plates (0.04 mm) are below printable size and carried only in the step-out of the outline.",
     "Rippen (0,12 m) und Platten (0,04 mm) liegen unter der druckbaren Größe und stecken nur in den Absätzen der Kontur."),
    ("No hatch: the elevation is not cut and the bar is 0.4 mm wide.", "Keine Schraffur: die Ansicht ist nicht geschnitten und der Balken 0,4 mm breit."),
    ("Dark solid bar reads as steel.", "Der dunkle Vollbalken liest sich als Stahl."),
    ("Overlays colour the whole bar with the area-weighted values of flange, web, base and head plate (labelled merged).",
     "Überlagerungen färben den ganzen Balken mit den flächengewichteten Werten von Flansch, Steg, Fuß- und Kopfplatte (als zusammengefasst gekennzeichnet).")))

r200 = rep(200, [
    D.rect("column-bar", C, "body", 1, True, 0.4, -0.1, 0.05, 0.1, 2.95, regions=BAR, material="steel"),
    D.rect("base-plate-slab", C, "body", 2, True, 0.3, -0.15, 0, 0.15, 0.05, region="base-plate", material="steel", penMm=0.1),
    D.rect("cap-plate-slab", C, "body", 2, True, 0.3, -0.12, 2.95, 0.12, 3.0, region="cap-plate", material="steel", penMm=0.1)],
    dec(("A 1 mm wide steel bar with a wider base plate and a head plate.", "Ein 1 mm breiter Stahlbalken mit breiterer Fußplatte und Kopfplatte."),
        ("Column bar, base plate (1.5 mm) and head plate (1.2 mm) at their true widths.", "Stützenbalken, Fußplatte (1,5 mm) und Kopfplatte (1,2 mm) in wahrer Breite."),
        ("Flanges and web are one bar shape.", "Flansche und Steg bilden eine Balkenform."),
        ("Stiffeners, nuts, bolts, fillet and flange edge lines, splice line.", "Rippen, Muttern, Bolzen, Hohlkehlen- und Flanschkantenlinien, Stoßlinie."),
        ("The 20 mm plates are drawn 50 mm thick (0.25 mm, the smallest filled feature that prints with a margin); the bar is shortened by the same 30 mm per end so the overall height stays 3.00 m.",
         "Die 20-mm-Platten sind 50 mm dick gezeichnet (0,25 mm, die kleinste gefüllte Einzelheit, die mit Reserve druckt); der Balken ist an jedem Ende um dieselben 30 mm gekürzt, sodass die Gesamthöhe 3,00 m bleibt."),
        ("No hatch.", "Keine Schraffur."),
        ("Dark bar, lighter plates.", "Dunkler Balken, hellere Platten."),
        ("Bar (merged flange and web), base plate and head plate are separate overlay cells.", "Balken (Flansch und Steg zusammengefasst), Fuß- und Kopfplatte sind getrennte Überlagerungszellen.")))

trap100 = lambda s: [(s * 0.1, 0.035), (s * 0.15, 0.035), (s * 0.13, 0.2), (s * 0.1, 0.2)]
r100 = rep(100, [
    D.rect("column-web-bar", C, "body", 1, True, 0.45, -0.1, 0.035, 0.1, 2.965, regions=BAR, material="steel"),
    D.rect("base-plate-block", C, "body", 2, True, 0.3, -0.15, 0, 0.15, 0.035, region="base-plate", material="steel", penMm=0.1),
    D.poly("stiffener-left", C, "body", 2, True, 0.3, trap100(-1), region="base-plate", material="steel", penMm=0.1),
    D.poly("stiffener-right", C, "body", 2, True, 0.3, trap100(1), region="base-plate", material="steel", penMm=0.1),
    D.rect("cap-plate-block", C, "body", 2, True, 0.3, -0.12, 2.965, 0.12, 3.0, region="cap-plate", material="steel", penMm=0.1),
    D.line("splice-joint", C, "detail", 1.5, 0.3, [(-0.1, 1.5), (0.1, 1.5)], region="column-web")],
    dec(("A 2 mm wide column bar, a base plate with two stiffener gussets and a head plate; the steel column reads as an H-section seen onto its web.",
         "Ein 2 mm breiter Stützenbalken, Fußplatte mit zwei Rippen und Kopfplatte; die Stahlstütze liest sich als H-Profil mit Blick auf den Steg."),
        ("Plates, two trapezoid gussets (1 x 1.5 mm) and the splice joint at 1.50 m.",
         "Platten, zwei Trapezrippen (1 x 1,5 mm) und der Montagestoß bei 1,50 m."),
        ("Flange and web share the bar shape; the 15 mm flange edges (0.15 mm) are merged into the outline.", "Flansch und Steg teilen sich die Balkenform; die 15-mm-Flanschkanten (0,15 mm) gehen in der Kontur auf."),
        ("Flange edge lines, fillet lines, anchor nuts and bolts (0.36 x 0.48 mm).", "Flanschkantenlinien, Hohlkehlenlinien, Ankermuttern und -bolzen (0,36 x 0,48 mm)."),
        ("The 20 mm plates are drawn 35 mm thick (0.35 mm, above the 0.2 mm feature minimum); the bar is shortened accordingly. The gussets are drawn as trapezoids with a 30 mm flat top, so no filled tip is thinner than the printable feature.",
         "Die 20-mm-Platten sind 35 mm dick gezeichnet (0,35 mm, über dem Mindestmerkmal von 0,2 mm); der Balken ist entsprechend gekürzt. Die Rippen sind als Trapeze mit 30 mm breiter Oberkante gezeichnet, sodass keine gefüllte Spitze dünner ist als das druckbare Merkmal."),
        ("No hatch: the elevation is not a cut.", "Keine Schraffur: die Ansicht ist kein Schnitt."),
        ("Outline 0.25 mm; dark column, lighter plates.", "Kontur 0,25 mm; dunkle Stütze, hellere Platten."),
        ("Bar cell (merged flange and web), plates with stiffeners and head plate; the bar carries the merged value.", "Balkenzelle (Flansch und Steg zusammengefasst), Platten mit Rippen und Kopfplatte; der Balken trägt den zusammengefassten Wert.")))

trap50 = lambda s: [(s * 0.1, 0.02), (s * 0.15, 0.02), (s * 0.13, 0.11), (s * 0.1, 0.11)]
nut = lambda s: (min(s * 0.137, s * 0.113), 0.02, max(s * 0.137, s * 0.113), 0.038)
s50 = [
    D.rect("web", C, "body", 1, True, 0.6, -0.085, 0.14, 0.085, 2.98, region="column-web", material="steel"),
    D.rect("flange-a", C, "body", 1.1, True, 0.45, -0.1, 0.02, -0.085, 2.98, region="column-flange", material="steel"),
    D.rect("flange-b", C, "body", 1.1, True, 0.45, 0.085, 0.02, 0.1, 2.98, region="column-flange", material="steel"),
    D.rect("web-foot", C, "body", 1, True, 0.6, -0.085, 0.02, 0.085, 0.14, region="column-web", material="steel"),
    D.line("fillet-a", C, "detail", 1.5, 0.3, [(-0.0655, 0.14), (-0.0655, 2.98)], region="column-web", penMm=0.1),
    D.line("fillet-b", C, "detail", 1.5, 0.3, [(0.0655, 0.14), (0.0655, 2.98)], region="column-web", penMm=0.1),
    D.rect("base-plate", C, "body", 2, True, 0.3, -0.15, 0, 0.15, 0.02, region="base-plate", material="steel"),
    D.poly("stiffener-a", C, "body", 2, True, 0.3, trap50(-1), region="base-plate", material="steel"),
    D.poly("stiffener-b", C, "body", 2, True, 0.3, trap50(1), region="base-plate", material="steel"),
    D.rect("anchor-nut-a", C, "detail", 2.5, True, 0.1, *nut(-1), region="base-plate", material="steel"),
    D.rect("anchor-nut-b", C, "detail", 2.5, True, 0.1, *nut(1), region="base-plate", material="steel"),
    D.line("anchor-bolt-a", C, "detail", 2.4, 0.1, [(-0.125, 0.038), (-0.125, 0.075)], region="base-plate"),
    D.line("anchor-bolt-b", C, "detail", 2.4, 0.1, [(0.125, 0.038), (0.125, 0.075)], region="base-plate"),
    D.rect("cap-plate", C, "body", 2, True, 0.3, -0.12, 2.98, 0.12, 3.0, region="cap-plate", material="steel"),
    D.line("splice-line", C, "detail", 1.5, 0.3, [(-0.1, 1.5), (0.1, 1.5)], region="column-web"),
    D.line("splice-mark", C, "detail", 1.6, 0.4, [(-0.14, 1.5), (0.14, 1.5)], region="column-web"),
]
# the splice mark is a cut-plane marker line; it sticks out of the flanges by 0.04 m (0.8 mm)
r50 = rep(50, s50, dec(
    ("The H-section seen onto its web: flange edges, web with fillet lines, base plate with two stiffener gussets, anchor nuts and bolts, head plate and a splice joint; the column reads as a rolled steel member.",
     "Das H-Profil mit Blick auf den Steg: Flanschkanten, Steg mit Hohlkehlenlinien, Fußplatte mit zwei Rippen, Ankermuttern und -bolzen, Kopfplatte und ein Montagestoß; die Stütze liest sich als gewalztes Stahlprofil."),
    ("Flange thickness 15 mm (0.3 mm), fillet lines, 20 mm plates at true thickness (0.4 mm), trapezoid gussets 0.05 x 0.09 m (30 mm flat top), nuts 18 x 24 mm (0.36 x 0.48 mm), bolts, splice and cut-plane marker.",
     "Flanschdicke 15 mm (0,3 mm), Hohlkehlenlinien, 20-mm-Platten in wahrer Dicke (0,4 mm), Trapezrippen 0,05 x 0,09 m (30 mm Oberkante), Muttern 18 x 24 mm (0,36 x 0,48 mm), Bolzen, Stoß und Schnittmarke."),
    ("Web faces of both chambers share the web shape; the plate, stiffeners and nuts share the base-plate region.", "Die Stegflächen beider Kammern teilen sich die Stegform; Platte, Rippen und Muttern teilen sich die Region Fußplatte."),
    ("Weld seams, the anchor bolt thread and the grout joint below the plate.", "Schweißnähte, das Ankerbolzengewinde und die Mörtelfuge unter der Platte."),
    ("The 9 mm web thickness is not drawn (view onto the web); bolt shanks are 0.1 mm pen lines (5 mm real, thinner than the 24 mm bolt).",
     "Die Stegdicke 9 mm wird nicht gezeichnet (Blick auf den Steg); Bolzenschäfte sind 0,1-mm-Linien (5 mm real, dünner als der 24-mm-Bolzen)."),
    ("No hatch: the elevation is not a cut; the cut section is the plan symbol.", "Keine Schraffur: die Ansicht ist kein Schnitt; der Schnitt ist das Grundrisssymbol."),
    ("Outline 0.35 mm, details 0.18 mm; flanges darker, web lighter.", "Kontur 0,35 mm, Details 0,18 mm; Flansche dunkler, Steg heller."),
    ("Flange edges, web (fillet lines), base plate group and head plate are separate overlay cells, so sunlit flanges and shaded web chambers show differently.",
     "Flanschkanten, Steg (Hohlkehlenlinien), Fußplattengruppe und Kopfplatte sind getrennte Überlagerungszellen, so erscheinen besonnte Flansche und verschattete Stegkammern verschieden.")))
# the cut-plane marker widens the bounding box slightly: keep it inside the plate width
s50[-1] = D.line("splice-mark", C, "detail", 1.6, 0.4, [(-0.14, 1.5), (0.14, 1.5)], region="column-web")

# plan symbol: HEB 200 cut (h = b = 200, tf 15, tw 9), web along x; coordinates are plan metres around the column axis
h, tf, tw, b = 0.2, 0.015, 0.009, 0.2
ipts = [(-0.1, -0.1), (0.1, -0.1), (0.1, -0.085), (0.0045, -0.085), (0.0045, 0.085), (0.1, 0.085), (0.1, 0.1), (-0.1, 0.1), (-0.1, 0.085), (-0.0045, 0.085), (-0.0045, -0.085), (-0.1, -0.085)]


def plan_poly(id, pts, **kw):
    d = {"id": id, "bone": "column", "role": "body", "z": 1, "filled": True, "tone": 0.3, "kind": "poly", "points": [list(p) for p in pts], "closed": True}
    d.update(kw)
    return d


a["symbols"] = [{
    "id": "section-plan",
    "title": T("HEB 200 column, cut in plan", "Stütze HEB 200, im Grundriss geschnitten"),
    "description": T("The conventional plan sign of a steel column: the rolled H-section cut above the base plate, hatched at 1:50 and solid black at 1:100 and 1:200 (DIN 1356-1, DIN ISO 128-50).",
                    "Das übliche Grundrisszeichen einer Stahlstütze: das gewalzte H-Profil über der Fußplatte geschnitten, bei 1:50 schraffiert und bei 1:100 und 1:200 schwarz gefüllt (DIN 1356-1, DIN ISO 128-50)."),
    "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50],
    "pens": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.25},
    "hatches": [H("steel", "Steel", "Stahl", "lines", 45, 0.6, 0.1)],
    "geometry": {"elevation": [], "plan": [plan_poly("section", ipts, region="column-flange", material="steel", hatch="steel")]},
    "dimensionalNote": T("Real size: 200 x 200 mm with 15 mm flanges and a 9 mm web; the fillets of the rolled section are not drawn. At 1:200 the sign is 1 mm square.",
                         "Echte Größe: 200 x 200 mm mit 15 mm Flanschen und 9 mm Steg; die Hohlkehlen des Walzprofils sind nicht gezeichnet. Bei 1:200 ist das Zeichen 1 mm groß."),
}]
a["representations"] = {"500": r500, "200": r200, "100": r100, "50": r50}
a["presentation"]["viewpoints"] = ["elevation"]
out = finish(a)
doc["asset"] = out
print(AID)
validate(out)
save_doc(AID, doc)
