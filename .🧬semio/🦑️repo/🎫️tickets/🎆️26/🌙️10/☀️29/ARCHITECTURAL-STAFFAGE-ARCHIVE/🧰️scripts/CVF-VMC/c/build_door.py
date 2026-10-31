import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CVF-VMC/c")
from lib import *

AID = "sliding-door"
doc = load_doc(AID)
a = doc["asset"]
merge_refs(a, [EXTRA_REFS["en-iso-13786"]])
a["dimensions"].update({"heightM": 2.15, "widthM": 3.1, "depthM": 0.1, "massKg": 86.75})
a["summary"] = T(
    "A 1.0 m wide, 1.9 m high single-glazed (10 mm toughened glass) steel-framed leaf hung from a carriage in a 3.0 m hollow-section head track between two steel jambs; conditioned element with interior at 20 degC and a winter reference case. The adjacent bays of the opening are not modelled.",
    "Ein 1,0 m breiter, 1,9 m hoher einfachverglaster (10 mm ESG) Stahlrahmenflügel an einem Laufwagen in einer 3,0 m langen Hohlprofil-Laufschiene zwischen zwei Stahlpfosten; konditioniertes Bauteil mit Innenraum 20 °C und Winter-Referenzfall. Die übrigen Felder der Öffnung sind nicht modelliert.")
set_material(a, "steel", conductivityWmK=50, specificHeatJKgK=450)
set_material(a, "glass", conductivityWmK=1.0, specificHeatJKgK=750, emissivity=0.837, reference="en-12898")
set_material(a, "frame-cavity", title=T("Hollow steel profile with enclosed air cavity", "Stahlhohlprofil mit eingeschlossenem Luftraum"), emissivity=0.85, solarAbsorptivity=0.5,
             conductivityWmK=0.3, source="authored-estimate")
HF = region
cavity = lambda: layer("cavity", "Enclosed air in the hollow section, equivalent resistance 0.17 m2K/W", "Eingeschlossene Luft im Hohlprofil, äquivalenter Widerstand 0,17 m²K/W", "air-gap", "frame-cavity", 0.05, 0.17, "authored-estimate", "en-iso-10077-1", 0)
steel_skin = lambda i: layer(f"skin-{i}", "Steel wall 4 mm", "Stahlwand 4 mm", "metal", "steel", 0.004, 0.00008, "literature", "en-iso-10456", 14100)
a["thermal"] = {
    "environment": {"airTempC": 0, "meanRadiantTempC": 0, "skyTempC": -12, "groundTempC": 2, "relativeHumidity": 0.8, "windMS": 3, "windDirectionDeg": 0,
                    "solarWM2": 300, "diffuseFraction": 0.25, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region("leaf-glass", "Leaf glass, single 10 mm toughened", "Flügelglas, einfach 10 mm ESG", "glazing", "conditioned", "glass", 1.62, 1.8, "vertical-plate", 0.5, [
            layer("glass-pane", "Toughened glass 10 mm", "Einscheibensicherheitsglas 10 mm", "glass", "glass", 0.01, 0.01, "literature", "en-iso-10456", 18750)],
            18750, "literature", "en-iso-6946", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
        region("leaf-frame", "Leaf frame, hollow steel profile (thermal bridge around the glass)", "Flügelrahmen, Stahlhohlprofil (Wärmebrücke um das Glas)", "surface", "conditioned", "steel", 0.28, 1.9, "vertical-plate", 0.5,
               [steel_skin("a"), cavity(), steel_skin("b")], 28000, "authored-estimate", "en-iso-10077-1", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
        region("track-steel", "Head track, RHS 100x50x4", "Laufschiene, RHS 100x50x4", "surface", "conditioned", "steel", 0.3, 3.0, "vertical-plate", 0.4,
               [steel_skin("a"), cavity(), steel_skin("b")], 28000, "authored-estimate", "en-iso-10077-1", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
        region("jamb-steel", "Steel jambs", "Stahlpfosten", "surface", "conditioned", "steel", 0.42, 2.1, "vertical-plate", 0.5,
               [steel_skin("a"), cavity(), steel_skin("b")], 28000, "authored-estimate", "en-iso-10077-1", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
    ],
    "emitters": [],
    "assumptions": [
        T("Winter reference case: air 0 degC, sky -12 degC, wind 3 m/s, 300 W/m2 normal irradiance; interior 20 degC with R_si 0.13 m2K/W. The viewer may select any preset.",
          "Winter-Referenzfall: Luft 0 °C, Himmel -12 °C, Wind 3 m/s, 300 W/m² Normalstrahlung; Innenraum 20 °C mit R_si 0,13 m²K/W. Der Betrachter kann jede Voreinstellung wählen."),
        T("The single 10 mm pane gives U about 5.9 W/(m2 K) (R_si 0.13 + 0.01 + R_se 0.04); the steel hollow sections with an enclosed air cavity (R 0.17) give about 2.9 W/(m2 K), so the frame is the warmer element on the cold side only by its larger resistance. The frame and track regions approximate the thermal bridges around the glass (authored estimate, EN ISO 10077-1 would resolve them).",
          "Die einfache 10-mm-Scheibe ergibt U etwa 5,9 W/(m² K) (R_si 0,13 + 0,01 + R_se 0,04); die Stahlhohlprofile mit eingeschlossenem Luftraum (R 0,17) ergeben etwa 2,9 W/(m² K), der Rahmen ist also nur durch seinen größeren Widerstand das wärmere Bauteil auf der kalten Seite. Die Rahmen- und Schienenregionen nähern die Wärmebrücken um das Glas an (Schätzung, EN ISO 10077-1 würde sie auflösen)."),
        T("The track and jambs are the steel frame of the opening: they separate interior from exterior like the leaf and are modelled as conditioned hollow sections.",
          "Schiene und Pfosten sind der Stahlrahmen der Öffnung: sie trennen wie der Flügel Innen von Außen und sind als konditionierte Hohlprofile modelliert."),
        T("Sun-exposed and shaded faces differ through the pose sunlit fractions: glass and leaf frame in the sun, track half shaded by the lintel, jambs half shaded.",
          "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: Glas und Flügelrahmen in der Sonne, Schiene zur Hälfte vom Sturz verschattet, Pfosten halb verschattet."),
    ],
}
bind_poses(a, {"leaf-glass": (1, 1.0), "leaf-frame": (1, 1.0), "track-steel": (1, 0.5), "jamb-steel": (1, 0.5)})
for cid, ttl in (("sun-and-shade", None),):
    a["clips"].append({
        "id": "sun-and-shade", "title": T("Sun and shade", "Sonne und Schatten"),
        "description": T("A cloud shadow crosses the door: the glass and the leaf frame lose the direct sun and the surface temperatures follow with their heat capacity.",
                         "Ein Wolkenschatten wandert über die Tür: Glas und Flügelrahmen verlieren die direkte Sonne und die Oberflächentemperaturen folgen mit ihrer Wärmekapazität."),
        "durationS": 8, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "closed", "ease": "smooth"}, {"t": 8, "pose": "closed", "ease": "smooth"}],
        "contacts": [], "phases": [{"id": "shadow", "fromS": 0, "toS": 8, "title": T("Cloud shadow passes", "Wolkenschatten zieht durch")}],
        "events": [{"t": 4, "kind": "thermal", "title": T("Deepest shade", "Tiefster Schatten")}],
        "tracks": [{"target": f"region:{r}:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 4, "v": 0.1}, {"t": 8, "v": 1.0}]} for r in ("leaf-glass", "leaf-frame")]})

D = Draw(a["rig"]["bones"])
TR, LF = "track", "leaf"
LEAF = ["leaf-glass", "leaf-frame"]
u_frame = [(-0.05, 0), (0.1, 0), (0.1, 2.0), (2.9, 2.0), (2.9, 0), (3.05, 0), (3.05, 2.15), (-0.05, 2.15)]

r500 = rep(500, [
    D.poly("portal", TR, "body", 1, True, 0.45, u_frame, regions=["track-steel", "jamb-steel"], material="steel"),
    D.rect("leaf", LF, "body", 2, True, 0.8, 1.0, 0.1, 2.0, 2.05, regions=LEAF, material="glass")],
    dec(("An inverted steel U (jambs and head track, 3.10 x 2.15 m, 6.2 x 4.3 mm) with a light leaf rectangle (2 x 3.9 mm) hanging in it; the leaf moves along the track.",
         "Ein umgekehrtes Stahl-U (Pfosten und Laufschiene, 3,10 x 2,15 m, 6,2 x 4,3 mm) mit einem hellen Flügelrechteck (2 x 3,9 mm), das darin hängt; der Flügel bewegt sich entlang der Schiene."),
        ("Opening width, head height, leaf width 1.0 m and leaf position.", "Öffnungsbreite, Sturzhöhe, Flügelbreite 1,0 m und Flügelposition."),
        ("Jambs and head track are one U outline; glass, frame and hanger plates are one leaf rectangle that reaches up to the track (hangers merged).",
         "Pfosten und Laufschiene sind eine U-Kontur; Glas, Rahmen und Hängelaschen sind ein Flügelrechteck, das bis zur Schiene reicht (Laschen zusammengefasst)."),
        ("Frame profile, handle, hangers, carriage, 0.1 m floor gap.", "Rahmenprofil, Griff, Laschen, Laufwagen, 0,1 m Bodenfuge."),
        ("The 0.1 m jambs and rail (0.2 mm) are drawn 0.15 m (0.3 mm, inside the opening) so the filled U keeps a printable width; the 50 mm hanger gap is closed by extending the leaf 50 mm (0.1 mm).",
         "Die 0,1-m-Pfosten und -Schiene (0,2 mm) sind 0,15 m (0,3 mm, nach innen) gezeichnet, damit das gefüllte U eine druckbare Breite behält; die 50-mm-Hängefuge ist durch 50 mm (0,1 mm) Flügelverlängerung geschlossen."),
        ("No hatch.", "Keine Schraffur."),
        ("Dark U frame, light leaf reads as glass.", "Dunkles U-Rahmen, heller Flügel liest sich als Glas."),
        ("Overlays colour the U (track and jambs merged) and the leaf (glass and frame merged); the leaf cell moves with the leaf.",
         "Überlagerungen färben das U (Schiene und Pfosten zusammengefasst) und den Flügel (Glas und Rahmen zusammengefasst); die Flügelzelle wandert mit dem Flügel.")))

h200 = []
r200 = rep(200, [
    D.rect("jamb-west", TR, "body", 1, True, 0.45, -0.05, 0, 0.05, 2.05, region="jamb-steel", material="steel"),
    D.rect("jamb-east", TR, "body", 1, True, 0.45, 2.95, 0, 3.05, 2.05, region="jamb-steel", material="steel"),
    D.rect("track", TR, "body", 1, True, 0.45, -0.05, 2.05, 3.05, 2.15, region="track-steel", material="steel"),
    D.rect("leaf-frame-200", LF, "body", 2, True, 0.5, 1.0, 0.1, 2.0, 2.05, region="leaf-frame", material="steel"),
    D.rect("leaf-glass-200", LF, "glazing", 3, True, 0.92, 1.075, 0.175, 1.925, 1.975, region="leaf-glass", material="glass")],
    dec(("Jambs and head track as three steel bars, the leaf as a framed light pane that reaches up to the rail (hangers merged).",
         "Pfosten und Laufschiene als drei Stahlbalken, der Flügel als gerahmte helle Scheibe, die bis zur Schiene reicht (Laschen zusammengefasst)."),
        ("Three frame bars (0.5 mm), leaf frame (5 x 9.75 mm), glass inset by 0.375 mm.", "Drei Rahmenbalken (0,5 mm), Flügelrahmen (5 x 9,75 mm), um 0,375 mm eingerücktes Glas."),
        ("Leaf frame sections of 50 mm (0.25 mm) are one frame shape with the glass inset line.", "Flügelrahmenprofile von 50 mm (0,25 mm) bilden eine Rahmenform mit der Glaseinrückung."),
        ("Handle, carriage wheels (hidden in the rail), brush seals and the spacer.", "Griff, Laufwagenrollen (in der Schiene verdeckt), Bürstendichtungen und der Abstandhalter."),
        ("The 50 mm hanger gap is closed by extending the leaf frame up to the rail; the glass is inset 75 mm instead of 50 mm so frame and glass outlines stay 0.15 mm apart.", "Die 50-mm-Hängefuge ist durch Verlängerung des Flügelrahmens bis zur Schiene geschlossen; das Glas ist um 75 statt 50 mm eingerückt, damit Rahmen- und Glaskontur 0,15 mm Abstand halten."),
        ("No hatch; glass is a light fill.", "Keine Schraffur; Glas ist eine helle Füllung."),
        ("Outline 0.18 mm, glass outline 0.1 mm; steel dark, glass light.", "Kontur 0,18 mm, Glaskontur 0,1 mm; Stahl dunkel, Glas hell."),
        ("Jambs and track are separate cells from the leaf frame and the glass, so the glass-versus-frame difference is readable.",
         "Pfosten und Schiene sind von Flügelrahmen und Glas getrennte Zellen, so ist der Unterschied Glas gegen Rahmen lesbar.")))

r100 = rep(100, [
    D.rect("jamb-west", TR, "body", 1, True, 0.45, -0.05, 0, 0.05, 2.05, region="jamb-steel", material="steel"),
    D.rect("jamb-east", TR, "body", 1, True, 0.45, 2.95, 0, 3.05, 2.05, region="jamb-steel", material="steel"),
    D.rect("track", TR, "body", 1, True, 0.45, -0.05, 2.05, 3.05, 2.15, region="track-steel", material="steel"),
    D.rect("leaf-frame", LF, "body", 2, True, 0.5, 1.0, 0.1, 2.0, 2.0, region="leaf-frame", material="steel"),
    D.rect("leaf-glass", LF, "glazing", 3, True, 0.92, 1.05, 0.15, 1.95, 1.95, region="leaf-glass", material="glass"),
    D.line("glass-reflection-a", LF, "texture", 3.1, 0.3, [(1.2, 0.5), (1.8, 1.1)], region="leaf-glass"),
    D.line("glass-reflection-b", LF, "texture", 3.1, 0.3, [(1.3, 0.4), (1.8, 0.9)], region="leaf-glass"),
    D.rect("hanger-a", LF, "body", 2.5, True, 0.3, 1.32, 2.0, 1.36, 2.05, region="leaf-frame", material="steel", penMm=0.1),
    D.rect("hanger-b", LF, "body", 2.5, True, 0.3, 1.64, 2.0, 1.68, 2.05, region="leaf-frame", material="steel", penMm=0.1),
    D.rect("handle", LF, "body", 3.2, True, 0.3, 1.9, 0.95, 1.94, 1.2, region="leaf-frame", material="steel", penMm=0.1)],
    dec(("The door elevation as a steel portal with a framed glass leaf, a pair of hanger plates and a pull handle.",
         "Die Türansicht als Stahlportal mit gerahmtem Glasflügel, einem Laschenpaar und einem Griff."),
        ("Jambs and rail (1 mm), leaf frame (10 x 19 mm) with 0.5 mm frame width, the glass pane, two hanger plates (0.4 x 0.5 mm), handle (0.4 x 2.5 mm).",
         "Pfosten und Schiene (1 mm), Flügelrahmen (10 x 19 mm) mit 0,5 mm Rahmenbreite, die Glasscheibe, zwei Laschen (0,4 x 0,5 mm), Griff (0,4 x 2,5 mm)."),
        ("The leaf frame profile is one frame outline around the glass.", "Das Flügelprofil ist eine Rahmenkontur um das Glas."),
        ("Carriage wheels inside the rail, bottom seal, rail screws.", "Laufwagenrollen in der Schiene, Bodendichtung, Schienenschrauben."),
        ("Hanger plates and handle have 0.1 mm outlines (about 0.2 mm wider than real).", "Laschen und Griff haben 0,1-mm-Konturen (etwa 0,2 mm breiter als real)."),
        ("No material hatch: this is a front elevation, not a cut; glass shows two reflection lines at 0.1 mm (DIN 1356-1).",
         "Keine Materialschraffur: dies ist eine Vorderansicht, kein Schnitt; Glas zeigt zwei Reflexlinien mit 0,1 mm (DIN 1356-1)."),
        ("Outline 0.25 mm, glass 0.1 mm; dark steel, light glass.", "Kontur 0,25 mm, Glas 0,1 mm; dunkler Stahl, helles Glas."),
        ("Leaf frame, leaf glass, track and jambs are separate overlay cells; the leaf cells slide with the leaf.", "Flügelrahmen, Flügelglas, Schiene und Pfosten sind getrennte Überlagerungszellen; die Flügelzellen gleiten mit dem Flügel.")))

s50 = [
    D.rect("jamb-west", TR, "body", 1, True, 0.45, -0.05, 0, 0.05, 2.05, region="jamb-steel", material="steel"),
    D.rect("jamb-east", TR, "body", 1, True, 0.45, 2.95, 0, 3.05, 2.05, region="jamb-steel", material="steel"),
    D.rect("track", TR, "body", 1, True, 0.45, -0.05, 2.05, 3.05, 2.15, region="track-steel", material="steel"),
    D.line("jamb-west-seal", TR, "detail", 1.5, 0.3, [(0.035, 0.0), (0.035, 2.05)], region="jamb-steel"),
    D.line("jamb-east-seal", TR, "detail", 1.5, 0.3, [(2.965, 0.0), (2.965, 2.05)], region="jamb-steel"),
]
for i, x in enumerate((0.3, 1.0, 2.0, 2.7)):
    s50.append(D.circ(f"track-screw-{i}", TR, "detail", 1.6, True, 0.2, x, 2.10, 0.012, region="track-steel"))
s50 += [
    D.rect("leaf-frame", LF, "body", 2, True, 0.5, 1.0, 0.1, 2.0, 2.0, region="leaf-frame", material="steel"),
    D.rect("leaf-glass", LF, "glazing", 3, True, 0.92, 1.05, 0.15, 1.95, 1.95, region="leaf-glass", material="glass"),
    D.line("glass-reflection-a", LF, "texture", 3.1, 0.3, [(1.2, 0.5), (1.8, 1.1)], region="leaf-glass"),
    D.line("glass-reflection-b", LF, "texture", 3.1, 0.3, [(1.3, 0.4), (1.8, 0.9)], region="leaf-glass"),
    D.line("glass-reflection-c", LF, "texture", 3.1, 0.3, [(1.15, 1.2), (1.45, 1.5)], region="leaf-glass"),
    D.rect("hanger-a", LF, "body", 2.5, True, 0.3, 1.32, 2.0, 1.36, 2.05, region="leaf-frame", material="steel"),
    D.rect("hanger-b", LF, "body", 2.5, True, 0.3, 1.64, 2.0, 1.68, 2.05, region="leaf-frame", material="steel"),
    D.circ("hanger-bolt-a", LF, "detail", 2.6, True, 0.1, 1.34, 2.025, 0.008, region="leaf-frame"),
    D.circ("hanger-bolt-b", LF, "detail", 2.6, True, 0.1, 1.66, 2.025, 0.008, region="leaf-frame"),
    D.rect("handle-base", LF, "body", 3.2, True, 0.3, 1.89, 0.93, 1.95, 1.22, region="leaf-frame", material="steel"),
    D.rect("handle-grip", LF, "body", 3.3, True, 0.25, 1.9, 0.98, 1.93, 1.17, region="leaf-frame", material="steel"),
    D.rect("bottom-seal", LF, "body", 2.2, True, 0.2, 1.0, 0.08, 2.0, 0.1, region="leaf-frame", material="steel", penMm=0.1),
]
r50 = rep(50, s50, dec(
    ("A framed single-glazed sliding leaf hanging from two plates in a steel head track between two jambs with weather seals; all visible fixings are drawn.",
     "Ein gerahmter einfachverglaster Schiebeflügel, an zwei Laschen in einer Stahl-Laufschiene zwischen zwei Pfosten mit Dichtungen hängend; alle sichtbaren Befestigungen sind gezeichnet."),
    ("Jambs and rail (2 mm) with seal lines and four rail screws, leaf frame (20 x 38 mm, 1 mm frame width), glass, three reflection lines, hanger plates with bolts, handle with base plate, bottom brush seal.",
     "Pfosten und Schiene (2 mm) mit Dichtungslinien und vier Schienenschrauben, Flügelrahmen (20 x 38 mm, 1 mm Rahmenbreite), Glas, drei Reflexlinien, Hängelaschen mit Bolzen, Griff mit Grundplatte, untere Bürstendichtung."),
    ("The 4 mm steel walls of the hollow profiles are not resolved; the frame is one filled shape per element.", "Die 4-mm-Stahlwände der Hohlprofile werden nicht aufgelöst; der Rahmen ist je Element eine gefüllte Form."),
    ("Carriage wheels (hidden inside the closed rail), glazing gaskets, corner welds.", "Laufwagenrollen (in der geschlossenen Schiene verdeckt), Glasdichtungen, Eckschweißnähte."),
    ("Bolts are 0.32 mm dots (16 mm), seal lines 0.18 mm; the 20 mm bottom seal is 0.4 mm tall with a 0.1 mm outline.",
     "Bolzen sind Punkte von 0,32 mm (16 mm), Dichtungslinien 0,18 mm; die 20-mm-Bodendichtung ist 0,4 mm hoch mit 0,1 mm Kontur."),
    ("No hatch: front elevation; glazing is shown by the light fill and three reflection lines.", "Keine Schraffur: Vorderansicht; Verglasung zeigt sich durch die helle Füllung und drei Reflexlinien."),
    ("Outline 0.35 mm, details 0.18 mm, glazing 0.13 mm, reflection 0.1 mm.", "Kontur 0,35 mm, Details 0,18 mm, Verglasung 0,13 mm, Reflex 0,1 mm."),
    ("Glass, leaf frame (with hangers, handle and seal), head track and jambs are separate overlay cells, so the cold glass and the warmer frame read differently.",
     "Glas, Flügelrahmen (mit Laschen, Griff und Dichtung), Laufschiene und Pfosten sind getrennte Überlagerungszellen; kaltes Glas und wärmerer Rahmen lesen sich verschieden.")))

def pl(id, pts, closed=False, filled=False, tone=0.5, **kw):
    d = {"id": id, "bone": "track", "role": kw.pop("role", "body"), "z": 1, "filled": filled, "tone": tone, "kind": "poly", "points": [list(p) for p in pts], "closed": closed}
    d.update(kw)
    return d


plan = [pl("jamb-west", [(-0.05, -0.1), (0.05, -0.1), (0.05, 0.1), (-0.05, 0.1)], True, True, 0.3, region="jamb-steel"),
        pl("jamb-east", [(2.95, -0.1), (3.05, -0.1), (3.05, 0.1), (2.95, 0.1)], True, True, 0.3, region="jamb-steel"),
        pl("track-line-a", [(0.05, -0.05), (2.95, -0.05)], role="detail", region="track-steel"),
        pl("track-line-b", [(0.05, 0.05), (2.95, 0.05)], role="detail", region="track-steel"),
        pl("leaf", [(1.0, -0.02), (2.0, -0.02), (2.0, 0.02), (1.0, 0.02)], True, True, 0.8, role="glazing", region="leaf-glass"),
        pl("slide-arrow", [(2.05, 0.0), (2.9, 0.0)], role="detail"),
        pl("slide-arrow-head", [(2.82, -0.05), (2.9, 0.0), (2.82, 0.05)], role="detail")]
a["symbols"] = [{
    "id": "sliding-door-plan",
    "title": T("Sliding door in plan", "Schiebetür im Grundriss"),
    "description": T("The conventional plan sign of a sliding door: the leaf as a thin glass rectangle in its track, the jambs as filled squares and an arrow in the sliding direction (DIN 1356-1).",
                    "Das übliche Grundrisszeichen einer Schiebetür: der Flügel als dünnes Glasrechteck in seiner Schiene, die Pfosten als gefüllte Quadrate und ein Pfeil in Schieberichtung (DIN 1356-1)."),
    "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50],
    "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.1, "ground": 0.25},
    "hatches": [],
    "geometry": {"elevation": [], "plan": plan},
    "dimensionalNote": T("Real size: 3.10 m between jamb faces by 0.20 m deep; the leaf is 1.00 m long and 40 mm thick, the arrow marks the slide direction and has no dimension.",
                         "Echte Größe: 3,10 m über die Pfostenkanten bei 0,20 m Tiefe; der Flügel ist 1,00 m lang und 40 mm dick, der Pfeil markiert die Schieberichtung und hat keine Abmessung."),
}]
a["representations"] = {"500": r500, "200": r200, "100": r100, "50": r50}
a["presentation"]["viewpoints"] = ["elevation"]
out = finish(a)
doc["asset"] = out
print(AID)
validate(out)
save_doc(AID, doc)
