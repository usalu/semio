import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CV-C")
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

# 1:500: stepped solid, open triangles under the treads closed
sol = [(0, 0), (FOOT1, 0), (x_head_bot, y_head), (x_head_top, y_head), (G * N, R * N)]
for k in range(N, 0, -1):
    sol.append((G * (k - 1), R * k))
    sol.append((G * (k - 1), R * (k - 1)))
sol = [p for p in sol]
sol[-1] = (0, 0)
r500 = rep(500, [D.poly("stair-solid", S, "body", 1, True, 0.4, sol[:-1], regions=BOTH, material="steel")], dec(
    ("One stepped solid: the stair is a wedge with a sawtooth top edge (12 steps of 0.60 x 0.48 mm) on a sloped underside.",
     "Ein gestufter Vollkörper: der Lauf ist ein Keil mit Sägezahn-Oberkante (12 Stufen zu 0,60 x 0,48 mm) auf geschräger Unterseite."),
    ("Rise 2.90 m, run 3.60 m, 12 steps, stringer foot and head seat; overall extent 3.86 x 2.90 m.", "Steigung 2,90 m, Lauflänge 3,60 m, 12 Stufen, Wangenfuß und Auflager; Gesamtmaß 3,86 x 2,90 m."),
    ("Stringer, gussets and tread boards are one solid; the open triangles below the treads (0.3 x 0.2 m, 0.6 x 0.4 mm) are closed.",
     "Wange, Knaggen und Stufenbohlen sind ein Körper; die offenen Dreiecke unter den Stufen (0,3 x 0,2 m, 0,6 x 0,4 mm) sind geschlossen."),
    ("Tread thickness, nosing overhang (30 mm), gussets, brackets and flange lines.", "Stufendicke, Nasenüberstand (30 mm), Knaggen, Konsolen und Flanschlinien."),
    ("The 40 mm treads (0.08 mm) merge into the sawtooth edge; there are no thin free parts.", "Die 40-mm-Stufen (0,08 mm) gehen in der Sägezahnkante auf; es gibt keine dünnen freien Teile."),
    ("No hatch: solid poché only.", "Keine Schraffur: nur Vollton-Poché."),
    ("Dark filled wedge reads as a steel stair; the stepped top edge carries the stair identity.", "Der dunkle Keil liest sich als Stahltreppe; die gestufte Oberkante trägt die Treppenidentität."),
    ("The whole wedge is coloured with the area-weighted value of stringer steel and tread tops (labelled merged).", "Der ganze Keil wird mit dem flächengewichteten Wert von Wangenstahl und Stufenoberseiten gefärbt (als zusammengefasst gekennzeichnet).")))

stringer_pts2 = [(0.02 / M, 0), (0.2255 / M, 0), ((y_head + 0.2255) / M, y_head), ((y_head + 0.02) / M, y_head)]
# 1:200: stringer band + 12 tread lines (open risers stay open)
s200 = [D.poly("stringer", S, "body", 1, True, 0.4, stringer_pts2, region="stringer-steel", material="steel")]
for k in range(1, N + 1):
    x0, x1 = tx(k)
    s200.append(D.line(f"tread-{k:02d}", S, "body", 2, 0.6, [(x0, R * k - 0.02), (x1, R * k - 0.02)], region="tread-top", material="timber", penMm=0.18))
r200 = rep(200, s200, dec(
    ("A sloped stringer band with twelve separate tread lines above it; the open risers keep the sawtooth rhythm readable.",
     "Ein geneigtes Wangenband mit zwölf getrennten Stufenlinien darüber; die offenen Setzstufen halten den Sägezahnrhythmus lesbar."),
    ("Stringer band (1.0 mm tall) with horizontal ends, 12 treads at 0.33 m (1.65 mm), 1.2 mm between treads.", "Wangenband (1,0 mm hoch) mit waagerechten Enden, 12 Stufen zu 0,33 m (1,65 mm), 1,2 mm zwischen den Stufen."),
    ("Gussets and stringer flanges are merged into the band.", "Knaggen und Wangenflansche gehen im Band auf."),
    ("Risers (the stair has none), gussets, nosings and flange lines.", "Setzstufen (der Lauf hat keine), Knaggen, Nasen und Flanschlinien."),
    ("The 40 mm tread is a 0.18 mm pen line (36 mm real, 4 mm thinner).", "Die 40-mm-Stufe ist eine Linie mit 0,18 mm Feder (36 mm real, 4 mm dünner)."),
    ("No hatch: the band is 1 mm wide.", "Keine Schraffur: das Band ist 1 mm breit."),
    ("Dark band reads as steel, treads as lighter lines.", "Das dunkle Band liest sich als Stahl, die Stufen als hellere Linien."),
    ("Stringer band and tread lines are separate overlay cells.", "Wangenband und Stufenlinien sind getrennte Überlagerungszellen.")))

# 1:100
h100 = [H("steel", "Steel", "Stahl", "lines", 45, 0.6, 0.1)]
s100 = [D.poly("stringer", S, "body", 1, True, 0.45, stringer_pts2, region="stringer-steel", material="steel", hatch="steel")]
for k in range(1, N + 1):
    x0, x1 = tx(k)
    s100.append(D.rect(f"tread-{k:02d}", S, "body", 2, True, 0.6, x0, R * k - 0.04, x1, R * k, region="tread-top", material="timber", penMm=0.1))
    s100.append(D.poly(f"gusset-{k:02d}", S, "body", 1.5, True, 0.3, [(G * k - 0.12, R * k - 0.04), (G * k, R * k - 0.04), (G * k - 0.12, M * (G * k - 0.12) - 0.02)], region="stringer-steel", material="steel", penMm=0.1))
r100 = rep(100, s100, dec(
    ("A hatched steel stringer with twelve filled tread boards standing on it with open risers.", "Eine schraffierte Stahlwange mit zwölf gefüllten Stufenbohlen darauf, mit offenen Setzstufen."),
    ("Stringer (1.6 mm wide, steel hatch), 12 treads of 3.3 x 0.4 mm with nosing overhang, 2.4 mm rise, 12 gusset triangles (1.2 x 1.0 mm) that join treads and stringer.", "Wange (1,6 mm breit, Stahlschraffur), 12 Stufen zu 3,3 x 0,4 mm mit Nasenüberstand, 2,4 mm Steigung, 12 Knaggendreiecke (1,2 x 1,0 mm), die Stufen und Wange verbinden."),
    ("Stringer flanges are merged into the band.", "Wangenflansche sind im Band zusammengefasst."),
    ("Flange lines and grain.", "Flanschlinien und Maserung."),
    ("The 40 mm tread is a 0.4 mm filled rectangle with a 0.1 mm outline.", "Die 40-mm-Stufe ist ein 0,4 mm hohes Rechteck mit 0,1 mm Kontur."),
    ("Steel hatch at 45 degrees, 0.6 mm spacing, on the stringer (not a cut: material indication only, DIN 1356-1).", "Stahlschraffur 45°, 0,6 mm Abstand, auf der Wange (kein Schnitt: nur Materialandeutung, DIN 1356-1)."),
    ("Outline 0.25 mm; hatched stringer, lighter treads.", "Kontur 0,25 mm; schraffierte Wange, hellere Stufen."),
    ("Stringer and each tread are separate overlay cells.", "Wange und jede Stufe sind eigene Überlagerungszellen.")), hatches=h100)

# 1:50
perp = 0.009
vt = perp / math.cos(math.atan(M))
xa = (0.04 + vt) / M
xe = (y_head + 0.2455 - vt) / M
s50 = [D.poly("stringer-web", S, "body", 1, True, 0.5, stringer_pts, region="stringer-steel", material="steel"),
       D.line("flange-top-edge", S, "detail", 1.5, 0.3, [(xa, 0.0), (x_head_top, y_head - vt)], region="stringer-steel"),
       D.line("flange-bottom-edge", S, "detail", 1.5, 0.3, [(FOOT1, vt), (xe, y_head)], region="stringer-steel")]
for k in range(1, N + 1):
    x0, x1 = tx(k)
    s50.append(D.rect(f"tread-{k:02d}", S, "body", 3, True, 0.6, x0, R * k - 0.04, x1, R * k, region="tread-top", material="timber"))
    s50.append(D.line(f"tread-{k:02d}-grain", S, "texture", 3.1, 0.3, [(x0 + 0.03, R * k - 0.02), (x1 - 0.03, R * k - 0.02)], region="tread-top"))
    xg = G * k
    s50.append(D.poly(f"gusset-{k:02d}", S, "body", 2, True, 0.3, [(xg - 0.12, R * k - 0.04), (xg, R * k - 0.04), (xg - 0.12, TOPY(xg - 0.12))], region="stringer-steel", material="steel"))
r50 = rep(50, s50, dec(
    ("The side elevation of the stair: an HEA 160 stringer with its flange edge lines, twelve timber treads with grain, and a welded gusset bracket under every tread, risers open.",
     "Die Seitenansicht des Laufs: eine Wange HEA 160 mit ihren Flanschkantenlinien, zwölf Holzstufen mit Maserung und unter jeder Stufe eine angeschweißte Knagge, Setzstufen offen."),
    ("Stringer web face, top and bottom flange lines, 12 treads (6.6 x 0.8 mm) with 30 mm nosing overhang, 12 gussets (2.4 x 1.9 mm), grain lines.",
     "Wangensteg, obere und untere Flanschlinie, 12 Stufen (6,6 x 0,8 mm) mit 30 mm Nasenüberstand, 12 Knaggen (2,4 x 1,9 mm), Maserungslinien."),
    ("Gusset and stringer share the steel region; the two flange lines are one detail pair.", "Knagge und Wange teilen sich die Stahlregion; die beiden Flanschlinien sind ein Detailpaar."),
    ("Weld seams, tread fixings, the stringer fillets and the second stringer of the pair behind.", "Schweißnähte, Stufenbefestigungen, die Hohlkehlen der Wange und die zweite Wange des Paares dahinter."),
    ("Flange thickness 9 mm (0.18 mm) is a 0.18 mm detail line; grain lines are 0.1 mm.", "Die Flanschdicke 9 mm (0,18 mm) ist eine 0,18-mm-Detaillinie; Maserungslinien sind 0,1 mm."),
    ("No hatch: the web face is an elevation, not a cut; timber is indicated by grain lines along the treads.", "Keine Schraffur: der Steg ist eine Ansicht, kein Schnitt; Holz ist durch Maserungslinien entlang der Stufen angedeutet."),
    ("Outline 0.35 mm, flange lines 0.18 mm, grain 0.1 mm; dark steel, lighter timber.", "Kontur 0,35 mm, Flanschlinien 0,18 mm, Maserung 0,1 mm; dunkler Stahl, hellere Holzstufen."),
    ("Stringer, gussets and each tread are separate overlay cells.", "Wange, Knaggen und jede Stufe sind eigene Überlagerungszellen.")), hatches=[])

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
