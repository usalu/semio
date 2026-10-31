import sys, json
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CV-C")
from lib import *

AID = "tilt-window"
doc = load_doc(AID)
a = doc["asset"]
merge_refs(a, [EXTRA_REFS["en-iso-13786"]])
a["dimensions"].update({"heightM": 1.25, "widthM": 0.3, "depthM": 0.9, "massKg": 72.8})
a["summary"] = T(
    "A 0.9 x 1.2 m timber sash with 4-16-4 low-e insulating glass, hinged at its bottom and tilting inward at the top, held by a head stay, drawn in vertical section with its blend-frame sill; conditioned element with interior on the left at 20 degC and a winter reference case.",
    "Ein 0,9 x 1,2 m Holzflügel mit Wärmeschutz-Isolierglas 4-16-4, unten angeschlagen und oben nach innen kippend, durch einen Stützarm gehalten, im Vertikalschnitt mit dem Blendrahmen-Sohlstück gezeichnet; konditioniertes Bauteil mit Innenraum links bei 20 °C und Winter-Referenzfall.")
a["provenance"]["notes"] = T(
    "The sash hinges at the bottom and tips inward (the tilt position of a tilt-and-turn window); the room is on the left, the exterior on the right. The stay is a head roller with horizontal normal. The glass is an insulating unit of two 4 mm panes with a 16 mm gas gap (U_g 1.1 W/(m2 K)); the pane mass is taken as the 24 mm total at glass density, as in the structural model.",
    "Der Flügel ist unten angeschlagen und kippt nach innen (Kippstellung eines Dreh-Kipp-Fensters); der Raum liegt links, außen rechts. Der Stützarm ist eine obere Rolle mit waagerechter Normalen. Das Glas ist eine Isolierverglasung aus zwei 4-mm-Scheiben mit 16 mm Gasfüllung (U_g 1,1 W/(m² K)); die Scheibenmasse ist wie im Tragwerksmodell als 24 mm Gesamtdicke mit Glasdichte angesetzt.")
txt = json.dumps(a["structure"], ensure_ascii=False)
txt = txt.replace("the wind is suction normal to the sash in the section plane, 0.8 kN/m2 over 0.9 m", "the wind is pressure on the exterior face, normal to the sash in the section plane, 0.8 kN/m2 over 0.9 m")
txt = txt.replace("der Wind ist Sog senkrecht zum Flügel in der Schnittebene, 0,8 kN/m² über 0,9 m", "der Wind ist Druck auf die Außenseite, senkrecht zum Flügel in der Schnittebene, 0,8 kN/m² über 0,9 m")
a["structure"] = json.loads(txt)
for p in a["poses"]:
    if p["id"] == "tilted-open":
        p["description"] = T("Sash tilted 15 degrees inward about the bottom hinge; the self-weight moment is taken by the stay.", "Flügel 15 Grad nach innen um das untere Gelenk gekippt; das Eigengewichtsmoment nimmt der Stützarm auf.")
    if p["id"] == "fully-tilted":
        p["title"] = T("Fully tilted with wind pressure", "Voll gekippt mit Winddruck")
        p["description"] = T("Sash at 30 degrees inward under 0.8 kN/m2 wind pressure on the exterior face over 0.9 m width.", "Flügel 30 Grad nach innen unter 0,8 kN/m² Winddruck auf der Außenseite über 0,9 m Breite.")
        for l in p["loads"]:
            l["id"] = "wind-pressure"
            l["title"] = T("Wind pressure 720 N/m along the sash", "Winddruck 720 N/m entlang des Flügels")
for c in a["clips"]:
    for t in c["tracks"]:
        t["target"] = t["target"].replace("wind-suction", "wind-pressure")
    c["description"] = T(c["description"]["en"].replace("suction gust", "pressure gust"), c["description"]["de"].replace("Sogstoß", "Druckstoß"))
    if c["id"] == "tilt-open-close":
        c["description"] = T("The sash tilts inward to 15 degrees, to 30 degrees and returns smoothly to closed.", "Der Flügel kippt nach innen auf 15 Grad, dann auf 30 Grad und kehrt weich in die Schließlage zurück.")
        c["phases"][0]["title"] = T("Tilting open", "Auskippen")

set_material(a, "timber", conductivityWmK=0.13, specificHeatJKgK=1600)
set_material(a, "glass", title=T("Insulating glass 4-16-4 low-e, argon", "Isolierglas 4-16-4 Low-E, Argon"), conductivityWmK=1.0, specificHeatJKgK=750, emissivity=0.837, reference="en-12898")
set_material(a, "glass-unit-cavity", title=T("16 mm argon cavity with low-e coating and warm-edge spacer (equivalent)", "16-mm-Argonfüllung mit Low-E-Beschichtung und warmer Kante (äquivalent)"),
             emissivity=0.84, solarAbsorptivity=0.08, conductivityWmK=0.022, source="authored-estimate")
set_material(a, "edge-zone", title=T("Edge-of-glass zone (spacer and gas), equivalent", "Glasrandzone (Abstandhalter und Gas), äquivalent"), emissivity=0.84, solarAbsorptivity=0.2,
             conductivityWmK=0.05, source="authored-estimate")
glass_pane = lambda i: layer(f"pane-{i}", "Float glass pane 4 mm", "Floatglasscheibe 4 mm", "glass", "glass", 0.004, 0.004, "literature", "en-iso-10456", 7500)
ENV = {"airTempC": 0, "meanRadiantTempC": 0, "skyTempC": -12, "groundTempC": 2, "relativeHumidity": 0.8, "windMS": 3, "windDirectionDeg": 0,
       "solarWM2": 300, "diffuseFraction": 0.25, "pressurePa": 101325}
a["thermal"] = {
    "environment": ENV,
    "operating": [],
    "regions": [
        region("pane-glass", "Insulating glass, centre of pane", "Isolierglas, Scheibenmitte", "glazing", "conditioned", "glass", 0.6, 1.08, "vertical-plate", 0.5, [
            glass_pane("inner"), layer("cavity", "16 mm gas cavity, equivalent resistance 0.731 m2K/W (U_g 1.1)", "16-mm-Gasraum, äquivalenter Widerstand 0,731 m²K/W (U_g 1,1)", "air-gap", "glass-unit-cavity", 0.016, 0.731, "authored-estimate", "en-iso-10077-1", 0), glass_pane("outer")],
               15000, "authored-estimate", "en-iso-10077-1", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
        region("pane-edge", "Insulating glass, edge zone 65 mm (spacer thermal bridge)", "Isolierglas, Randzone 65 mm (Wärmebrücke Abstandhalter)", "glazing", "conditioned", "edge-zone", 0.24, 0.065, "vertical-plate", 0.5, [
            glass_pane("inner"), layer("edge-body", "Edge zone, equivalent resistance 0.405 m2K/W (psi_g 0.04 W/mK over 65 mm)", "Randzone, äquivalenter Widerstand 0,405 m²K/W (psi_g 0,04 W/mK über 65 mm)", "air-gap", "edge-zone", 0.016, 0.405, "authored-estimate", "en-iso-10077-1", 0), glass_pane("outer")],
               15000, "authored-estimate", "en-iso-10077-1", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
        region("frame-timber", "Timber sash frame", "Holzflügelrahmen", "surface", "conditioned", "timber", 0.238, 0.09, "vertical-plate", 0.5, [
            layer("sash-timber", "Softwood sash profile 90 mm", "Nadelholz-Flügelprofil 90 mm", "timber", "timber", 0.09, 0.692, "literature", "en-iso-10456", 24000)],
               24000, "literature", "en-iso-10077-1", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
        region("frame-sill", "Blend-frame sill (outer face)", "Blendrahmen-Sohlstück (Außenseite)", "surface", "conditioned", "timber", 0.09, 0.1, "vertical-plate", 0.4, [
            layer("sill-timber", "Softwood sill 300 mm deep", "Nadelholz-Sohlstück 300 mm tief", "timber", "timber", 0.3, 2.31, "literature", "en-iso-10456", 24000)],
               24000, "literature", "en-iso-10077-1", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
    ],
    "emitters": [],
    "assumptions": [
        T("Winter reference case: air 0 degC, sky -12 degC, wind 3 m/s, 300 W/m2 normal irradiance; interior 20 degC with R_si 0.13 m2K/W. The viewer may select any preset.",
          "Winter-Referenzfall: Luft 0 °C, Himmel -12 °C, Wind 3 m/s, 300 W/m² Normalstrahlung; Innenraum 20 °C mit R_si 0,13 m²K/W. Der Betrachter kann jede Voreinstellung wählen."),
        T("Glass centre: R_si 0.13 + 0.004 + 0.731 + 0.004 + R_se 0.04 gives U_g 1.1 W/(m2 K); the 65 mm edge zone adds psi_g 0.04 W/(mK) (warm-edge spacer), U about 1.7 W/(m2 K); the 90 mm softwood frame gives U_f about 1.2 W/(m2 K) (EN ISO 10077-1).",
          "Glasmitte: R_si 0,13 + 0,004 + 0,731 + 0,004 + R_se 0,04 ergibt U_g 1,1 W/(m² K); die 65 mm breite Randzone fügt psi_g 0,04 W/(mK) hinzu (warme Kante), U etwa 1,7 W/(m² K); der 90-mm-Nadelholzrahmen ergibt U_f etwa 1,2 W/(m² K) (EN ISO 10077-1)."),
        T("The thermal bridges (spacer edge, frame) are approximated by separate regions, as in EN ISO 10077-1 where U_g, U_f and psi_g are combined by area; the equivalent resistances are authored estimates.",
          "Die Wärmebrücken (Abstandhalterrand, Rahmen) sind durch eigene Regionen angenähert, wie in EN ISO 10077-1, wo U_g, U_f und psi_g flächenweise kombiniert werden; die äquivalenten Widerstände sind Schätzungen."),
        T("Sun-exposed and shaded faces differ through the pose sunlit fractions: glass and sash in the sun, the sill half shaded by the reveal.",
          "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: Glas und Flügel in der Sonne, das Sohlstück halb durch die Laibung verschattet."),
        T("The glass thickness is 24 mm in the geometry and the structural mass; only the thermal layers resolve the 4-16-4 build-up.",
          "Die Glasdicke beträgt in Geometrie und Tragwerksmasse 24 mm; nur die thermischen Schichten lösen den Aufbau 4-16-4 auf."),
    ],
}
bind_poses(a, {"pane-glass": (1, 1.0), "pane-edge": (1, 1.0), "frame-timber": (1, 1.0), "frame-sill": (1, 0.5)})
a["clips"].append({
    "id": "sun-and-shade", "title": T("Sun and shade", "Sonne und Schatten"),
    "description": T("A cloud shadow crosses the closed window: glass and sash lose the direct sun and the surface temperatures follow with their heat capacity.",
                    "Ein Wolkenschatten wandert über das geschlossene Fenster: Glas und Flügel verlieren die direkte Sonne und die Oberflächentemperaturen folgen mit ihrer Wärmekapazität."),
    "durationS": 8, "fps": 30, "loop": True, "rootMotion": "none",
    "keys": [{"t": 0, "pose": "closed", "ease": "smooth"}, {"t": 8, "pose": "closed", "ease": "smooth"}],
    "contacts": [{"anchor": "stay", "fromS": 0, "toS": 8}], "phases": [{"id": "shadow", "fromS": 0, "toS": 8, "title": T("Cloud shadow passes", "Wolkenschatten zieht durch")}],
    "events": [{"t": 4, "kind": "thermal", "title": T("Deepest shade", "Tiefster Schatten")}],
    "tracks": [{"target": f"region:{r}:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 4, "v": 0.1}, {"t": 8, "v": 1.0}]} for r in ("pane-glass", "pane-edge", "frame-timber")]})

D = Draw(a["rig"]["bones"])
SI, SA = "sill", "sash"
GLASS = ["pane-glass", "pane-edge"]
SILL = (-0.15, 0.0, 0.15, 0.1)
y_g0, y_g1 = 0.095, 1.205  # glass incl. rebate
edge_lo, edge_hi = 0.175, 1.125

r500 = unavailable(500,
                   ("A window section is 0.30 x 1.25 m, i.e. 0.6 x 2.5 mm at 1:500; the sash is 0.18 mm and the 24 mm glass 0.05 mm thick, below every printable stroke, and any thickening would multiply the real thickness.",
                    "Ein Fensterschnitt ist 0,30 x 1,25 m groß, bei 1:500 also 0,6 x 2,5 mm; der Flügel ist 0,18 mm und das 24-mm-Glas 0,05 mm dick, unter jeder druckbaren Strichstärke, und jede Verdickung würde die echte Dicke vervielfachen."),
                   ("Glass pane thickness 24 mm in a sash of 90 mm", "Scheibendicke 24 mm in einem Flügel von 90 mm"), 0.024,
                   [{"kind": "scale", "denominator": 200}, {"kind": "symbol", "symbol": "window-plan"}, {"kind": "symbol", "symbol": "tilt-opening-sign"}])

r200 = rep(200, [
    D.rect("sill", SI, "body", 1, True, 0.6, *SILL, region="frame-sill", material="timber", penMm=0.13),
    D.rect("rail-bottom", SA, "body", 3, True, 0.6, -0.045, 0.05, 0.045, 0.11, region="frame-timber", material="timber", penMm=0.1),
    D.rect("rail-top", SA, "body", 3, True, 0.6, -0.045, 1.19, 0.045, 1.25, region="frame-timber", material="timber", penMm=0.1),
    D.line("glass", SA, "glazing", 4, 0.92, [(0, 0.11), (0, 1.19)], regions=GLASS, material="glass", penMm=0.1)],
    dec(("A section of a sash with a heavier sill: two small timber blocks at the ends of a single glass line; the sash is 6 mm tall, the sill 1.5 mm wide.",
         "Ein Schnitt durch einen Flügel mit schwererem Sohlstück: zwei kleine Holzblöcke an den Enden einer einzigen Glaslinie; der Flügel ist 6 mm hoch, das Sohlstück 1,5 mm breit."),
        ("Sill block (1.5 x 0.5 mm), the two sash rails (0.45 x 0.3 mm) and the glass pane between them.", "Sohlstück (1,5 x 0,5 mm), die beiden Flügelrahmenhölzer (0,45 x 0,3 mm) und die Glasscheibe dazwischen."),
        ("Both panes, the cavity and the spacer are one glass line; glass centre and 65 mm edge zones share it.", "Beide Scheiben, der Gasraum und der Abstandhalter sind eine Glaslinie; Scheibenmitte und 65-mm-Randzonen teilen sie."),
        ("Handle (it sits on the side stile, outside the section plane), hinge pin, glazing beads, seals and the drip edge.", "Griff (er sitzt am seitlichen Flügelholz, außerhalb der Schnittebene), Gelenkbolzen, Glasleisten, Dichtungen und die Tropfkante."),
        ("The 24 mm glass is a line of 0.1 mm pen (20 mm); the rails have 0.1 mm outlines on 0.45 x 0.3 mm blocks.", "Das 24-mm-Glas ist eine Linie mit 0,1 mm Feder (20 mm); die Rahmenhölzer haben 0,1-mm-Konturen auf 0,45 x 0,3 mm großen Blöcken."),
        ("No hatch: the blocks are too small.", "Keine Schraffur: die Blöcke sind zu klein."),
        ("Dark timber blocks, a thin light glass line.", "Dunkle Holzblöcke, eine dünne helle Glaslinie."),
        ("The glass line carries merged centre and edge values; rails and sill are separate cells.", "Die Glaslinie trägt zusammengefasste Werte für Mitte und Rand; Rahmenhölzer und Sohlstück sind getrennte Zellen.")))

r100 = rep(100, [
    D.rect("sill", SI, "body", 1, True, 0.6, *SILL, region="frame-sill", material="timber"),
    D.rect("rail-bottom", SA, "body", 3, True, 0.6, -0.045, 0.05, 0.045, 0.11, region="frame-timber", material="timber", penMm=0.13),
    D.rect("rail-top", SA, "body", 3, True, 0.6, -0.045, 1.19, 0.045, 1.25, region="frame-timber", material="timber", penMm=0.13),
    D.rect("glass-edge-bottom", SA, "glazing", 4, True, 0.8, -0.012, 0.11, 0.012, edge_lo, region="pane-edge", material="glass", penMm=0.1),
    D.rect("glass-centre", SA, "glazing", 4, True, 0.92, -0.012, edge_lo, 0.012, edge_hi, region="pane-glass", material="glass", penMm=0.1),
    D.rect("glass-edge-top", SA, "glazing", 4, True, 0.8, -0.012, edge_hi, 0.012, 1.19, region="pane-edge", material="glass", penMm=0.1),
    D.circ("hinge-pin", SA, "detail", 5, True, 0.3, 0.0, 0.05, 0.02, region="frame-timber")],
    dec(("A section of the sash as a thin glass band between two rail blocks on a sill block, with the hinge pin.",
         "Ein Schnitt des Flügels als dünnes Glasband zwischen zwei Rahmenblöcken auf einem Sohlstück, mit Gelenkbolzen."),
        ("Sill (3 x 1 mm), rails (0.9 x 0.6 mm), glass band 0.24 x 10.8 mm split into centre and the two 0.65 mm edge zones, hinge pin (0.4 mm).",
         "Sohlstück (3 x 1 mm), Rahmenhölzer (0,9 x 0,6 mm), Glasband 0,24 x 10,8 mm geteilt in Mitte und zwei Randzonen zu 0,65 mm, Gelenkbolzen (0,4 mm)."),
        ("The two panes and the cavity are one glass band; the glazing beads are merged into the rails.", "Die zwei Scheiben und der Gasraum sind ein Glasband; die Glasleisten sind in den Rahmenhölzern zusammengefasst."),
        ("Handle (on the side stile, outside the section plane), pane lines, spacer, seals, rebates and the drip edge.", "Griff (am seitlichen Flügelholz, außerhalb der Schnittebene), Scheibenlinien, Abstandhalter, Dichtungen, Falze und Tropfkante."),
        ("The glass band is a 0.24 mm filled strip with a 0.1 mm outline (0.34 mm total, 34 mm real vs 24 mm).", "Das Glasband ist ein 0,24 mm breiter gefüllter Streifen mit 0,1 mm Kontur (insgesamt 0,34 mm, 34 mm real gegen 24 mm)."),
        ("No hatch: the cut timber is 1 mm and below the printable hatch spacing.", "Keine Schraffur: das geschnittene Holz ist 1 mm klein und unter dem druckbaren Schraffurabstand."),
        ("Cut timber dark, glass light with darker edge zones.", "Geschnittenes Holz dunkel, Glas hell mit dunkleren Randzonen."),
        ("Glass centre and the two edge zones are separate overlay cells, so the colder centre and the warmer spacer edge read differently.",
         "Glasmitte und die beiden Randzonen sind getrennte Überlagerungszellen, so lesen sich die kältere Mitte und der wärmere Abstandhalterrand verschieden.")))

h50 = [H("timber-cross", "Timber, cross section", "Holz, Querschnitt", "cross", 45, 0.8, 0.1)]
rail_b = [(-0.045, 0.05), (0.045, 0.05), (0.045, 0.11), (0.013, 0.11), (0.013, y_g0), (-0.013, y_g0), (-0.013, 0.11), (-0.045, 0.11)]
rail_t = [(-0.045, 1.25), (0.045, 1.25), (0.045, 1.19), (0.013, 1.19), (0.013, y_g1), (-0.013, y_g1), (-0.013, 1.19), (-0.045, 1.19)]
s50 = [
    D.rect("sill", SI, "body", 1, True, 0.6, *SILL, region="frame-sill", material="timber", hatch="timber-cross"),
    D.line("sill-drip", SI, "detail", 1.5, 0.3, [(0.13, 0.0), (0.13, 0.03)], region="frame-sill"),
    D.poly("rail-bottom", SA, "body", 3, True, 0.6, rail_b, region="frame-timber", material="timber", hatch="timber-cross"),
    D.poly("rail-top", SA, "body", 3, True, 0.6, rail_t, region="frame-timber", material="timber", hatch="timber-cross"),
    D.rect("spacer-bottom", SA, "body", 4, True, 0.3, -0.012, y_g0, 0.012, 0.11, region="pane-edge", material="edge-zone", penMm=0.1),
    D.rect("spacer-top", SA, "body", 4, True, 0.3, -0.012, 1.19, 0.012, y_g1, region="pane-edge", material="edge-zone", penMm=0.1)]
for side, x in (("inner", -0.012), ("outer", 0.012)):
    s50 += [
        D.line(f"pane-{side}-edge-bottom", SA, "glazing", 4.2, 0.9, [(x, 0.11), (x, edge_lo)], region="pane-edge", penMm=0.1),
        D.line(f"pane-{side}-centre", SA, "glazing", 4.2, 0.9, [(x, edge_lo), (x, edge_hi)], region="pane-glass", penMm=0.1),
        D.line(f"pane-{side}-edge-top", SA, "glazing", 4.2, 0.9, [(x, edge_hi), (x, 1.19)], region="pane-edge", penMm=0.1)]
s50 += [
    D.circ("hinge-pin", SA, "detail", 5, True, 0.3, 0.0, 0.05, 0.02, region="frame-timber"),
]
r50 = rep(50, s50, dec(
    ("The section through the sash and the sill: two hatched timber rails with glass rebates, an insulating unit drawn as two pane lines with a gas gap and spacers, the hinge and the cross-hatched sill with a drip.",
     "Der Schnitt durch Flügel und Sohlstück: zwei schraffierte Holzrahmenhölzer mit Glasfalzen, eine Isolierverglasung als zwei Scheibenlinien mit Gasraum und Abstandhaltern, das Gelenk und das kreuzschraffierte Sohlstück mit Tropfkante."),
    ("Rails 4.5 x 3 mm with 0.65 mm rebates, pane lines 0.24 mm apart, spacers (0.5 x 0.75 mm), glass centre and 65 mm edge zones, hinge pin, sill (15 x 5 mm) with drip.",
     "Rahmenhölzer 4,5 x 3 mm mit 0,65 mm Falz, Scheibenlinien 0,24 mm Abstand, Abstandhalter (0,5 x 0,75 mm), Glasmitte und 65-mm-Randzonen, Gelenkbolzen, Sohlstück (15 x 5 mm) mit Tropfkante."),
    ("Both panes are drawn only as lines; the argon gap is the space between them.", "Beide Scheiben sind nur als Linien gezeichnet; der Argonraum ist der Zwischenraum."),
    ("Glazing beads, gaskets, the stay arm (the frame head is not modelled) and the sealing bulbs.", "Glasleisten, Dichtungen, der Stützarm (der Blendrahmenkopf ist nicht modelliert) und die Dichtungsprofile."),
    ("The 4 mm panes are 0.1 mm pen lines (5 mm real) at their true 20 mm distance (0.4 mm hole); the spacers are 0.5 x 0.75 mm blocks.",
     "Die 4-mm-Scheiben sind 0,1-mm-Linien (5 mm real) in ihrem wahren Abstand von 20 mm (0,4 mm); die Abstandhalter sind 0,5 x 0,75 mm große Blöcke."),
    ("Cut timber is cross-hatched at 45 degrees, 0.8 mm (DIN ISO 128-50 timber across the grain); glass stays unhatched with lines for the panes.",
     "Geschnittenes Holz ist 45° kreuzschraffiert, 0,8 mm (DIN ISO 128-50 Holz quer zur Faser); Glas bleibt ungeschraffiert mit Linien für die Scheiben."),
    ("Outline 0.35 mm, details 0.18 mm, glazing 0.13 mm, hatch 0.1 mm; cut timber hatched, spacers dark.", "Kontur 0,35 mm, Details 0,18 mm, Verglasung 0,13 mm, Schraffur 0,1 mm; geschnittenes Holz schraffiert, Abstandhalter dunkel."),
    ("Rails, sill, spacer edge zones and the glass centre are separate overlay cells; the pane lines are bound to the zone they lie in.",
     "Rahmenhölzer, Sohlstück, Randzonen am Abstandhalter und Glasmitte sind getrennte Überlagerungszellen; die Scheibenlinien sind an die Zone gebunden, in der sie liegen.")), hatches=h50)


def pl(id, pts, closed=False, filled=False, tone=0.5, role="body", bone="sill", **kw):
    d = {"id": id, "bone": bone, "role": role, "z": 1, "filled": filled, "tone": tone, "kind": "poly", "points": [list(p) for p in pts], "closed": closed}
    d.update(kw)
    return d


plan = [pl("wall-left", [(-0.15, -0.15), (0.0, -0.15), (0.0, 0.15), (-0.15, 0.15)], True, True, 0.6),
        pl("wall-right", [(0.9, -0.15), (1.05, -0.15), (1.05, 0.15), (0.9, 0.15)], True, True, 0.6),
        pl("frame-interior", [(0.0, -0.06), (0.9, -0.06)], role="detail", region="frame-sill"),
        pl("glass-line", [(0.0, 0.0), (0.9, 0.0)], role="glazing", region="pane-glass"),
        pl("frame-exterior", [(0.0, 0.06), (0.9, 0.06)], role="detail", region="frame-sill")]
fac = [pl("window-outline", [(0, 0), (0.9, 0), (0.9, 1.2), (0, 1.2)], True, False, 0.5, region="frame-timber"),
       pl("tilt-line-a", [(0, 1.2), (0.45, 0.0)], role="detail"),
       pl("tilt-line-b", [(0.9, 1.2), (0.45, 0.0)], role="detail")]
sy_pens = {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.1, "ground": 0.25}
a["symbols"] = [
    {"id": "window-plan", "title": T("Window in plan", "Fenster im Grundriss"),
     "description": T("The conventional plan sign of a window in a wall opening: the wall cut at the reveals, three lines across the opening for the frames and the glass (DIN 1356-1).",
                     "Das übliche Grundrisszeichen eines Fensters in einer Wandöffnung: die Wand an den Laibungen geschnitten, drei Linien quer über die Öffnung für die Rahmen und das Glas (DIN 1356-1)."),
     "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100, 50], "pens": sy_pens, "hatches": [],
     "geometry": {"elevation": [], "plan": plan},
     "dimensionalNote": T("Real size: opening 0.90 m wide in a 0.30 m wall; the three lines are 60 mm apart, which is 0.12 mm at 1:500 and merges into one line.",
                          "Echte Größe: Öffnung 0,90 m breit in einer 0,30 m dicken Wand; die drei Linien liegen 60 mm auseinander, bei 1:500 sind das 0,12 mm und sie verschmelzen zu einer Linie.")},
    {"id": "tilt-opening-sign", "title": T("Tilt sign in the facade elevation", "Kippzeichen in der Fassadenansicht"),
     "description": T("The facade-view opening sign of a bottom-hinged tilt sash: two lines from the head corners that meet at the hinge side (DIN 1356-1).",
                     "Das Öffnungszeichen eines unten angeschlagenen Kippflügels in der Fassadenansicht: zwei Linien von den oberen Ecken, die sich auf der Bandseite treffen (DIN 1356-1)."),
     "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50], "pens": sy_pens, "hatches": [],
     "geometry": {"elevation": fac},
     "dimensionalNote": T("Real size: the sash outline 0.90 x 1.20 m in the facade plane; the lines carry only the opening direction.",
                          "Echte Größe: der Flügelumriss 0,90 x 1,20 m in der Fassadenebene; die Linien tragen nur die Öffnungsrichtung.")}]
a["representations"] = {"500": r500, "200": r200, "100": r100, "50": r50}
a["presentation"]["viewpoints"] = ["elevation"]
out = finish(a)
doc["asset"] = out
print(AID)
validate(out)
save_doc(AID, doc)
