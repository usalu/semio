import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CVF-VMC/c")
from lib import *

AID = "stair-flight"
doc = load_doc(AID)
a = doc["asset"]
merge_refs(a, [EXTRA_REFS["en-iso-13786"]])
G, N, RISE = 0.3, 12, 2.9
R = RISE / N
M = RISE / (G * N)
TOPY = lambda x: M * x - 0.04
BOTY = lambda x: M * x - 0.2455
FOOT0, FOOT1 = 0.04 / M, 0.2455 / M
y_head = RISE - 0.04
x_head_top = (y_head + 0.04) / M
x_head_bot = (y_head + 0.2455) / M
a["dimensions"].update({"heightM": RISE, "widthM": round(x_head_bot + 0.03, 3), "depthM": 1.2, "massKg": 166.2})
a["summary"] = T(
    "A 12-step straight steel stair flight (rise 241.7 mm, going 300 mm) with an HEA 160 stringer, open risers, welded gusset brackets and 40 mm timber treads, pinned at the lower landing and on a roller at the upper landing.",
    "Ein gerader Stahltreppenlauf mit 12 Stufen (Steigung 241,7 mm, Auftritt 300 mm) auf einer Wange HEA 160, offenen Setzstufen, angeschweißten Knaggen und 40 mm dicken Holzstufen, unten Festlager, oben Rollenlager.")
set_material(a, "steel", conductivityWmK=50, specificHeatJKgK=450)
set_material(a, "timber", conductivityWmK=0.13, specificHeatJKgK=1600)
a["thermal"] = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0,
                    "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region("stringer-steel", "Stringer and gusset steel", "Wange und Knaggen Stahl", "surface", "passive", "steel", 4.2, 0.2, "vertical-plate", 0.5, [], 26000, "authored-estimate", "en-iso-10456"),
        region("tread-top", "Timber tread surfaces", "Holzstufenoberflächen", "surface", "passive", "timber", 4.75, 0.33, "horizontal-plate-up", 0.95, [], 24000, "authored-estimate", "en-iso-13786"),
    ],
    "emitters": [],
    "assumptions": [
        T("The stringer is an HEA 160 (about 0.9 m2 of surface per metre) with an average steel thickness of 7.5 mm; tread tops are 12 timber boards of 0.33 x 1.2 m.",
          "Die Wange ist ein HEA 160 (etwa 0,9 m² Oberfläche je Meter) mit mittlerer Stahldicke 7,5 mm; die Stufenoberseiten sind 12 Holzbohlen 0,33 x 1,2 m."),
        T("Timber treads store heat in an effective depth of 36 mm (EN ISO 13786, half the 24 h penetration depth), so they warm slowly in the sun and stay cooler than the stringer.",
          "Holzstufen speichern Wärme in einer wirksamen Tiefe von 36 mm (EN ISO 13786, die Hälfte der 24-h-Eindringtiefe), sie erwärmen sich in der Sonne langsam und bleiben kühler als die Wange."),
        T("Sun-exposed and shaded faces differ through the pose sunlit fractions: treads fully sunlit, stringer half in shade of the treads.",
          "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: Stufen voll besonnt, Wange halb im Schatten der Stufen."),
        T("Reference case: a mild sunny day, all values authored estimates.", "Referenzfall: ein milder sonniger Tag, alle Werte sind Schätzungen."),
    ],
}
bind_poses(a, {"stringer-steel": (1, 0.5), "tread-top": (1, 1.0)})

D = Draw(a["rig"]["bones"])
S = "stringer"
BOTH = ["stringer-steel", "tread-top"]
stringer_pts = [(FOOT0, 0), (FOOT1, 0), (x_head_bot, y_head), (x_head_top, y_head)]
tx = lambda k: (G * (k - 1) - 0.03, G * k)

ytop = lambda x: M * x - 0.04
tread_x = lambda k: (G * (k - 1) - 0.03, G * k)
band_pts = [(FOOT0, 0), (FOOT1, 0), (x_head_bot, y_head), (x_head_top, y_head)]

# 1:500: one stringer band plus twelve tread strokes
yc = lambda x: M * x - 0.1428
s500 = [D.line("stringer-stroke", S, "body", 1, 0.4, [(0.1773, 0.0), (3.727, y_head)], region="stringer-steel", material="steel", penMm=0.35),
        D.line("head-bearing", S, "body", 1, 0.4, [(3.6, y_head), (3.855, y_head)], region="stringer-steel", material="steel", penMm=0.35)]
for k in range(1, N + 1):
    x0, x1 = tread_x(k)
    s500.append(D.line(f"step-{k:02d}", S, "body", 2, 0.6, [(x0, R * k - 0.02), (x1, R * k - 0.02)], region="tread-top", material="timber"))
r500 = rep(500, s500, dec(
    ("A dark sloping stringer stroke (0.35 mm, the 160 mm HEA depth) with twelve short tread strokes above it; the open risers stay open, so the flight reads as a sloping band with a rhythm of twelve steps.",
     "Ein dunkler geneigter Wangenstrich (0,35 mm, die Tiefe 160 mm der HEA) mit zwölf kurzen Stufenstrichen darüber; die offenen Setzstufen bleiben offen, der Lauf liest sich als geneigtes Band mit zwölf Stufen im Rhythmus."),
    ("Rise 2.90 m, run 3.60 m, 12 treads (0.66 mm long, 0.48 mm apart vertically), stringer foot and head seat; overall extent 3.89 x 2.90 m.",
     "Steigung 2,90 m, Lauflänge 3,60 m, 12 Stufen (0,66 mm lang, 0,48 mm senkrechter Abstand), Wangenfuß und Auflager; Gesamtmaß 3,89 x 2,90 m."),
    ("Stringer flanges, web and gussets are one stroke; the landing bearing is a short stroke at the head.", "Wangenflansche, Steg und Knaggen sind ein Strich; das Auflager am Kopf ist ein kurzer Strich."),
    ("Tread thickness, nosing details, gussets and flange lines.", "Stufendicke, Nasendetails, Knaggen und Flanschlinien."),
    ("Each 40 mm tread is a stroke of the body pen 0.13 mm (65 mm, thicker than real); a sawtooth silhouette was rejected because its 0.6 x 0.5 mm teeth are thinner than the printable feature.",
     "Jede 40-mm-Stufe ist ein Strich mit dem Körperstift 0,13 mm (65 mm, dicker als real); eine Sägezahnsilhouette wurde verworfen, weil ihre Zähne mit 0,6 x 0,5 mm dünner sind als das druckbare Merkmal."),
    ("No hatch: the stringer is a single stroke. A filled parallelogram band was rejected because its 39 degree end tips are thinner than the printable feature.", "Keine Schraffur: die Wange ist ein einzelner Strich. Ein gefülltes Parallelogrammband wurde verworfen, weil seine Endspitzen von 39 Grad dünner sind als das druckbare Merkmal."),
    ("Dark band reads as steel, tread strokes as the timber layer on top.", "Das dunkle Band liest sich als Stahl, die Stufenstriche als Holzlage darüber."),
    ("The stringer band carries the stringer-steel value; every tread stroke carries the tread-top value on its own.", "Das Wangenband trägt den Wert Wangenstahl; jeder Stufenstrich trägt für sich den Wert Stufenoberseite.")))

# 1:200: band with a lighter stripe of flange, twelve filled treads
yb2 = y_head - 0.1
xb2 = (yb2 + 0.2455) / M
band200 = [(0.12, 0), (FOOT1, 0), (xb2, yb2), (x_head_bot, yb2), (x_head_bot, y_head), (x_head_top, y_head)]
s200 = [D.poly("stringer-beam", S, "body", 1, True, 0.4, band200, region="stringer-steel", material="steel")]
for k in range(1, N + 1):
    x0, x1 = tread_x(k)
    s200.append(D.rect(f"tread-board-{k:02d}", S, "body", 2, True, 0.6, x0, R * k - 0.06, x1, R * k, region="tread-top", material="timber"))
r200 = rep(200, s200, dec(
    ("A sloped stringer beam (1.0 mm deep) carrying twelve filled tread boards that overhang the nosing; open risers keep light between the boards.",
     "Ein geneigter Wangenträger (1,0 mm tief) mit zwölf gefüllten Stufenbohlen, die über die Nase überstehen; offene Setzstufen lassen Licht zwischen den Bohlen."),
    ("Stringer, 12 tread boards (1.65 mm long, 0.3 mm thick) with the 30 mm nosing overhang, 1.2 mm apart vertically.", "Wange, 12 Stufenbohlen (1,65 mm lang, 0,3 mm dick) mit 30 mm Nasenüberstand, 1,2 mm senkrechter Abstand."),
    ("Gussets and stringer flanges are merged into the stringer beam.", "Knaggen und Wangenflansche gehen im Wangenträger auf."),
    ("Risers (the stair has none), gussets, flange lines, grain.", "Setzstufen (der Lauf hat keine), Knaggen, Flanschlinien, Maserung."),
    ("The 40 mm tread is drawn 60 mm thick (0.3 mm), the smallest filled feature that prints with a margin; it rests on the stringer line as in reality.",
     "Die 40-mm-Stufe ist 60 mm dick gezeichnet (0,3 mm), die kleinste gefüllte Einzelheit, die mit Reserve druckt; sie ruht wie in Wirklichkeit auf der Wangenlinie."),
    ("No hatch: the stringer is 1 mm deep.", "Keine Schraffur: die Wange ist 1 mm tief."),
    ("Dark beam reads as steel, lighter boards as timber; outline 0.18 mm.", "Der dunkle Träger liest sich als Stahl, die helleren Bohlen als Holz; Kontur 0,18 mm."),
    ("Stringer and each tread board are separate overlay cells.", "Wange und jede Stufenbohle sind getrennte Überlagerungszellen.")))

# 1:100: hatched stringer, filled treads, gussets as outlines
h100 = [H("steel", "Steel", "Stahl", "lines", 45, 0.6, 0.1)]
band100 = stringer_pts
s100 = [D.poly("stringer-hatched", S, "body", 1, True, 0.45, band100, region="stringer-steel", material="steel", hatch="steel")]
for k in range(1, N + 1):
    x0, x1 = tread_x(k)
    s100.append(D.rect(f"tread-plank-{k:02d}", S, "body", 2, True, 0.6, x0, R * k - 0.04, x1, R * k, region="tread-top", material="timber", penMm=0.1))
    xg = G * k
    s100.append(D.poly(f"bracket-{k:02d}", S, "detail", 1.5, False, 0.3, [(xg - 0.12, R * k - 0.04), (xg, R * k - 0.04), (xg - 0.12, ytop(xg - 0.12))], True, region="stringer-steel", material="steel", penMm=0.1))
r100 = rep(100, s100, dec(
    ("A steel-hatched stringer with twelve tread planks on it; under every plank an outlined gusset triangle shows how the plank is carried; risers open.",
     "Eine stahlschraffierte Wange mit zwölf Stufenbohlen darauf; unter jeder Bohle zeigt ein umrissenes Knaggendreieck, wie die Bohle getragen wird; Setzstufen offen."),
    ("Stringer (1.6 mm deep, steel hatch), 12 planks of 3.3 x 0.4 mm with nosing overhang, 2.4 mm rise, 12 gusset outlines (1.2 x 0.97 mm).",
     "Wange (1,6 mm tief, Stahlschraffur), 12 Bohlen zu 3,3 x 0,4 mm mit Nasenüberstand, 2,4 mm Steigung, 12 Knaggenumrisse (1,2 x 0,97 mm)."),
    ("Stringer flanges are merged into the stringer shape.", "Wangenflansche sind in der Wangenform zusammengefasst."),
    ("Flange lines, welds and grain.", "Flanschlinien, Schweißnähte und Maserung."),
    ("The gussets are unfilled outlines (pen 0.1 mm): a filled triangle of 1.2 x 1 mm has tips thinner than the printable feature.", "Die Knaggen sind ungefüllte Umrisse (Stift 0,1 mm): ein gefülltes Dreieck von 1,2 x 1 mm hat Spitzen, die dünner sind als das druckbare Merkmal."),
    ("Steel hatch at 45 degrees, 0.6 mm spacing, on the stringer (material indication, not a cut; DIN 1356-1).", "Stahlschraffur 45°, 0,6 mm Abstand, auf der Wange (Materialandeutung, kein Schnitt; DIN 1356-1)."),
    ("Outline 0.25 mm; hatched stringer, lighter planks, gusset outlines 0.1 mm.", "Kontur 0,25 mm; schraffierte Wange, hellere Bohlen, Knaggenumrisse 0,1 mm."),
    ("Stringer, each plank and each gusset outline are separate overlay cells (gussets share the stringer value).", "Wange, jede Bohle und jeder Knaggenumriss sind getrennte Überlagerungszellen (Knaggen teilen den Wert der Wange).")), hatches=h100)

# 1:50: web, open-top flange edges dropped (they would sit 0.18 mm from the outline), treads with grain hatch, filled gussets, screws, welds
h50 = [H("grain", "Timber, longitudinal", "Holz, Längsschnitt", "lines", 0, 0.4, 0.1), H("steel", "Steel", "Stahl", "lines", 45, 0.8, 0.1)]
s50 = [D.poly("stringer-web", S, "body", 1, True, 0.5, stringer_pts, region="stringer-steel", material="steel", hatch="steel")]
for k in range(1, N + 1):
    x0, x1 = tread_x(k)
    s50.append(D.rect(f"tread-{k:02d}", S, "body", 3, True, 0.6, x0, R * k - 0.04, x1, R * k, region="tread-top", material="timber", hatch="grain"))
    xg = G * k
    s50.append(D.poly(f"gusset-{k:02d}", S, "body", 2, True, 0.3, [(xg - 0.12, R * k - 0.04), (xg - 0.03, R * k - 0.04), (xg - 0.03, ytop(xg - 0.03)), (xg - 0.12, ytop(xg - 0.12))], region="stringer-steel", material="steel"))
    for j, fx in enumerate((x0 + 0.05, x1 - 0.05)):
        s50.append(D.circ(f"screw-{k:02d}-{'ab'[j]}", S, "detail", 3.2, False, 0.2, fx, R * k - 0.02, 0.009, region="tread-top", material="steel", penMm=0.1))
r50 = rep(50, s50, dec(
    ("The side elevation of the stair: an HEA 160 stringer web with steel hatch, twelve timber treads with longitudinal grain hatch and two countersunk fixing screws each, and a welded gusset bracket under every tread, risers open.",
     "Die Seitenansicht des Laufs: ein Wangensteg HEA 160 mit Stahlschraffur, zwölf Holzstufen mit Faserschraffur und je zwei Senkschrauben sowie unter jeder Stufe eine angeschweißte Knagge, Setzstufen offen."),
    ("Stringer web, 12 treads (6.6 x 0.8 mm) with 30 mm nosing overhang, 12 trapezoid gussets (1.8 x 1.9 mm, tip cut 30 mm short of the tread end), 24 fixing screws (0.36 mm).",
     "Wangensteg, 12 Stufen (6,6 x 0,8 mm) mit 30 mm Nasenüberstand, 12 Trapezknaggen (1,8 x 1,9 mm, Spitze 30 mm vor dem Stufenende gekappt), 24 Befestigungsschrauben (0,36 mm)."),
    ("Gusset and stringer share the steel region and hide each other's inner outline, so they read as one welded member.", "Knagge und Wange teilen sich die Stahlregion und verdecken gegenseitig ihre Innenkontur, sie lesen sich als ein geschweißtes Bauteil."),
    ("Weld seams, the stringer fillets and flange edge lines (9 mm = 0.18 mm from the outline would close to a 0.04 mm white sliver) and the second stringer behind.",
     "Schweißnähte, die Hohlkehlen der Wange und die Flanschkantenlinien (9 mm = 0,18 mm von der Kontur würden eine 0,04 mm weiße Lücke bilden) und die zweite Wange dahinter."),
    ("Screw heads are unfilled circles of 0.1 mm pen (18 mm diameter would be 0.36 mm).", "Schraubenköpfe sind ungefüllte Kreise mit 0,1-mm-Stift (18 mm Durchmesser wären 0,36 mm)."),
    ("Timber longitudinal hatch (0 degrees, 0.4 mm) on the treads, steel hatch (45 degrees, 0.8 mm) on the stringer web: material indication in the elevation, not a cut (DIN ISO 128-50).",
     "Holz längs (0°, 0,4 mm) auf den Stufen, Stahlschraffur (45°, 0,8 mm) auf dem Wangensteg: Materialandeutung in der Ansicht, kein Schnitt (DIN ISO 128-50)."),
    ("Outline 0.35 mm, screws 0.1 mm, hatches 0.1 mm; dark steel, lighter timber.", "Kontur 0,35 mm, Schrauben 0,1 mm, Schraffuren 0,1 mm; dunkler Stahl, hellere Holzstufen."),
    ("Stringer, gussets and each tread are separate overlay cells; screws carry the tread value.", "Wange, Knaggen und jede Stufe sind getrennte Überlagerungszellen; Schrauben tragen den Stufenwert.")), hatches=h50)

# plan symbol (DIN 1356-1): flight outline, tread lines, walking line with arrow, cut break line; plan metres, x = run, y = width
def pl(id, pts, closed=False, filled=False, role="body", pen=None, **kw):
    d = {"id": id, "bone": "stringer", "role": role, "z": 1, "filled": filled, "tone": 0.5, "kind": "poly", "points": [list(p) for p in pts], "closed": closed}
    if pen:
        d["penMm"] = pen
    d.update(kw)
    return d


wl = 1.2
plan = [pl("flight-outline", [(0, 0), (3.6, 0), (3.6, wl), (0, wl)], True, region="tread-top")]
for k in range(1, N):
    plan.append(pl(f"tread-edge-{k:02d}", [(G * k, 0), (G * k, wl)], False, role="detail", region="tread-top"))
plan += [pl("walking-line", [(0.15, wl / 2), (3.45, wl / 2)], False, role="detail"),
         pl("walking-arrow", [(3.3, wl / 2 - 0.08), (3.45, wl / 2), (3.3, wl / 2 + 0.08)], False, role="detail"),
         pl("walking-start", [(0.15, wl / 2 - 0.03), (0.18, wl / 2), (0.15, wl / 2 + 0.03)], True, True, role="detail")]
a["symbols"] = [{
    "id": "flight-plan",
    "title": T("Stair flight in plan with walking line", "Treppenlauf im Grundriss mit Lauflinie"),
    "description": T("The conventional plan sign of a straight flight (DIN 1356-1): outline, one line per riser, the walking line with a start dot and an arrow pointing up.",
                    "Das übliche Grundrisszeichen eines geraden Laufs (DIN 1356-1): Umriss, eine Linie je Steigung, die Lauflinie mit Startpunkt und Pfeil nach oben."),
    "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50],
    "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    "hatches": [],
    "geometry": {"elevation": [], "plan": plan},
    "dimensionalNote": T("Real size: 3.60 m run (12 goings of 0.30 m) by 1.20 m width; the cut line at 1 m above the floor is not drawn. At 1:200 the sign is 18 x 6 mm.",
                         "Echte Größe: 3,60 m Lauflänge (12 Auftritte zu 0,30 m) bei 1,20 m Breite; die Schnittlinie 1 m über dem Boden ist nicht gezeichnet. Bei 1:200 ist das Zeichen 18 x 6 mm groß."),
}]
a["representations"] = {"500": r500, "200": r200, "100": r100, "50": r50}
a["presentation"]["viewpoints"] = ["elevation"]
out = finish(a)
doc["asset"] = out
print(AID, a["dimensions"])
validate(out)
save_doc(AID, doc)
