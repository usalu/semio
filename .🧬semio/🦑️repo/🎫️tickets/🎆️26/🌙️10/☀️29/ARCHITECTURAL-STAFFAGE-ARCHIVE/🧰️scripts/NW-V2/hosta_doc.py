import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from hosta import *

NAMES = list(SPEC)
BL = [f"blade-{n}" for n in NAMES]
PT = [f"petiole-{n}" for n in NAMES]


def sgn(n):
    return 1 if n[0] in "RC" else -1


def offs(pet, bla):
    o = {}
    for n in NAMES:
        s = sgn(n)
        o[f"petiole-{n}"] = {"angleDeg": r4(pet * s)}
        o[f"blade-{n}"] = {"angleDeg": r4(bla * s)}
    return o


def anchors_emitters():
    A = [anchors_support("root-plate", T("Root ball in the bed", "Wurzelballen im Beet"), 0.5, 0.5)]
    out, inn = [], []
    for n in NAMES:
        lst = out if n in OUTER else inn
        aid = f"tp-{'outer' if lst is out else 'inner'}-{len(lst) + 1}"
        A.append(emitter_anchor(aid, f"blade-{n}", LEAF[n]["p2"][0] * 0 + 0.15, 0.0, T(f"Transpiration of blade {n}", f"Transpiration von Spreite {n}")))
        lst.append(aid)
    return A, out, inn


def make():
    A, out, inn = anchors_emitters()
    em = [transpiration("transpiration-outer", T("Transpiration of the outer blades (distributed over four anchors)", "Transpiration der äußeren Spreiten (über vier Anker verteilt)"), out, "blades-outer", up_in_bone(B, "blade-R1"), 0.5),
          transpiration("transpiration-inner", T("Transpiration of the inner blades (distributed over five anchors)", "Transpiration der inneren Spreiten (über fünf Anker verteilt)"), inn, "blades-inner", up_in_bone(B, "blade-R3"), 0.4)]
    regions = [
        leaf_region("blades-outer", T("Outer leaf blades", "Äußere Blattspreiten"), "hosta-blade", 0.45, 0.2, "horizontal-plate-up", 0.7, 1100, 0.0045, 0.7, "c3", "jones-2014"),
        leaf_region("blades-inner", T("Inner leaf blades", "Innere Blattspreiten"), "hosta-blade", 0.6, 0.2, "vertical-plate", 0.45, 1100, 0.004, 0.9, "c3", "jones-2014"),
        passive_region("petioles", T("Petioles", "Blattstiele"), "stem", "hosta-petiole", 0.25, 0.012, "cylinder", 0.5, 11000, "schmid-1991"),
    ]
    regions[2]["hidden"] = T("The petioles are largely covered by the overlapping blades; they show as separate strokes only at 1:50 and 1:100.", "Die Blattstiele sind weitgehend von den überlappenden Spreiten verdeckt; als eigene Formen zeigen sie sich nur bei 1:50 und 1:100.")
    mats = [{"id": "hosta-blade", "title": T("Hosta leaf blade (thin, glaucous-green, ribbed)", "Funkien-Blattspreite (dünn, blaugrün, gerippt)"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "hosta-petiole", "title": T("Succulent petiole (water-rich)", "Saftiger Blattstiel (wasserreich)"), "emissivity": 0.95, "solarAbsorptivity": 0.55, "densityKgM3": 950, "specificHeatJKgK": 3800, "source": "authored-estimate", "reference": "schmid-1991"}]
    def tb(o, i, p, emit, wet=None):
        return bind({"blades-outer": (1.0, o, wet), "blades-inner": (1.0, i, wet), "petioles": (0.3, p, wet)}, {"transpiration-outer": emit[0], "transpiration-inner": emit[1]})
    def winds(v, cf):
        return [wind_load(f"wind-{n}", T(f"Wind drag on blade {n}", f"Winddruck auf Spreite {n}"), f"blade-{n}", 0.12, 0.0, cf, 0.06, v) for n in NAMES]
    rain = [vertical_load(f"water-{n}", "imposed", T(f"Rain water retained on blade {n}", f"Auf Spreite {n} haftendes Regenwasser"), f"blade-{n}", 0.15, 0.0, -9.81 * 0.04, "en-1991-1-3") for n in NAMES]
    poses = [
        pose("summer-full", T("Summer, full leaf", "Sommer, voll belaubt"), T("Shaded summer bed: the blades spread in an arching mound, the outer blades touching the soil. Only dappled sun reaches the outer blades.", "Beschattetes Sommerbeet: Die Spreiten breiten sich zu einem bogigen Hügel aus, die äußeren Spreiten berühren den Boden. Nur Lichtflecken erreichen die äußeren Spreiten."), {}, tb(0.35, 0.1, 0.0, (1.0, 0.8)), winds(1.5, 1.2)),
        pose("spring-emerging", T("Spring, leaves unfurling", "Frühling, Blätter entrollen sich"), T("Young leaves rise in tight upright rolls: petioles lift by 25 degrees and the blades fold up beside them; the mound is narrow and transpires little.", "Junge Blätter steigen in dichten, aufrechten Rollen auf: Die Stiele heben sich um 25 Grad und die Spreiten falten sich daneben; der Hügel ist schmal und transpiriert wenig."), offs(14, 38), tb(0.7, 0.4, 0.2, (0.4, 0.3)), winds(1.5, 0.6)),
        pose("rain-bowed", T("Heavy rain, blades bowed", "Starkregen, Spreiten gebeugt"), T("Rain water on the broad blades bows them downward by their weight; the surface is wet (evaporation, no transpiration need).", "Regenwasser auf den breiten Spreiten beugt sie durch sein Gewicht nach unten; die Oberfläche ist nass (Verdunstung, kein Transpirationsbedarf)."), offs(-6, -14), tb(0.2, 0.1, 0.0, (0.1, 0.1), wet=0.9), rain),
        pose("autumn-collapse", T("Autumn, leaves collapsed", "Herbst, Blätter eingesunken"), T("After the first frost the leaves lose turgor and flop: the petioles fold down and the blades hang to the soil; senescent leaves hardly transpire.", "Nach dem ersten Frost verlieren die Blätter Turgor und sinken ein: Die Stiele klappen nach unten und die Spreiten hängen zum Boden; welkende Blätter transpirieren kaum."), offs(-24, -34), tb(0.5, 0.3, 0.1, (0.05, 0.05), wet=0.2), []),
    ]
    return A, em, regions, mats, poses


def clips(poses):
    def bo(pid):
        return [p for p in poses if p["id"] == pid][0]["bones"]
    specs = {}
    for i, n in enumerate(NAMES):
        specs[f"petiole-{n}"] = (1.2, 40 * i, 1)
        specs[f"blade-{n}"] = (3.5, 40 * i + 60, 1)
    base = {}
    sway = sway_clip(B.list, "wind-flutter", T("Wind flutter", "Windflattern"), T("A deterministic 1.2 s cycle (0.83 Hz): the broad, thin blades nod about their rest position with a phase that runs around the mound; the stiff petioles move little.", "Ein deterministischer Zyklus von 1,2 s (0,83 Hz): Die breiten dünnen Spreiten nicken um ihre Ruhelage mit einer um den Hügel laufenden Phase; die steifen Stiele bewegen sich wenig."), 1.2, base, specs)
    for k in sway["keys"]:
        k["pose"] = "summer-full"
    keys = []
    for t, pid, amp in ((0, "summer-full", 0.5), (1.5, "summer-full", 5.0), (2.6, "summer-full", 8.0), (4.0, "summer-full", 4.0), (6.0, "summer-full", 0.5)):
        ov = {f"blade-{n}": {"angleDeg": r4(clamp(amp * sin(radians(50 * i + 150 * t)) * (-1 if t in (1.5, 2.6) else 1), -40, 40))} for i, n in enumerate(NAMES)}
        keys.append({"t": t, "pose": pid, "ease": "smooth", "bones": ov})
    gust = {"id": "gust", "title": T("Gust of wind", "Windböe"), "description": T("A 6 s gust: the blades flutter harder and bow downwind at the peak of 2.6 s, then settle.", "Eine Böe von 6 s: Die Spreiten flattern stärker und beugen sich am Höhepunkt bei 2,6 s in Windrichtung, dann beruhigen sie sich."), "durationS": 6.0, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "gust", "fromS": 0, "toS": 4.0, "title": T("Gust", "Böe")}, {"id": "settle", "fromS": 4.0, "toS": 6.0, "title": T("Settling", "Abklingen")}], "events": [{"t": 2.6, "kind": "motion", "title": T("Peak flutter", "Größtes Flattern")}],
            "tracks": [{"target": "emitter:transpiration-outer", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 2.6, "v": 0.7}, {"t": 6.0, "v": 1.0}]}]}
    return [sway, gust]


def dec(sil, ret, mer, omi, thin, hat, sty, ove):
    return decisions(*[T(*x) for x in (sil, ret, mer, omi, thin, hat, sty, ove)])


def representations():
    r = {}
    r["500"] = unavailable(500, T("A hosta of 0.9 m by 0.62 m is 1.8 mm by 1.2 mm at 1:500: a dot. Its blades (0.28 m, 0.56 mm) and their overlaps cannot be told apart, so any elevation would be a blob without information beyond the plan symbol.", "Eine Funkie von 0,9 m mal 0,62 m misst bei 1:500 1,8 mm mal 1,2 mm: ein Punkt. Ihre Spreiten (0,28 m, 0,56 mm) und deren Überlagerungen sind nicht unterscheidbar; jede Ansicht wäre ein Fleck ohne mehr Information als das Grundrisssymbol."), T("Leaf blade (width 0.25 m)", "Blattspreite (Breite 0,25 m)"), 0.25, [{"kind": "symbol", "symbol": "plan-extent"}, {"kind": "scale", "denominator": 200}])
    r["200"] = rep(200, 151, 350, pens(0.18, 0.13, None, None, 0.25), [], build_200(), dec(
        ("At 1:200 (1 m is 5 mm) the hosta is a mound of nine overlapping blades, 4.4 mm wide and 3.1 mm high, each blade an ovate pointed outline, the outer blades touching the ground.", "Bei 1:200 (1 m sind 5 mm) ist die Funkie ein Hügel aus neun überlappenden Spreiten, 4,4 mm breit und 3,1 mm hoch, jede Spreite ein eiförmiger spitzer Umriss, die äußeren Spreiten berühren den Boden."),
        ("Retained: blade outlines, their overlap order from the outer to the central blade, the real width and height.", "Beibehalten: Spreitenumrisse, ihre Überlagerungsfolge von den äußeren zur mittleren Spreite, die wahre Breite und Höhe."),
        ("Merged: 2 to 3 real leaves behind each drawn blade.", "Zusammengefasst: 2 bis 3 wirkliche Blätter hinter jeder gezeichneten Spreite."),
        ("Omitted: petioles (covered), ribs, veins and the waviness of the margin.", "Weggelassen: Blattstiele (verdeckt), Rippen, Adern und die Wellung des Randes."),
        ("Petioles (12 mm) are covered by the blades; the ribs (3 mm) are dropped.", "Blattstiele (12 mm) sind von den Spreiten verdeckt; die Rippen (3 mm) entfallen."),
        ("None.", "Keine."),
        ("Outer blades lighter grey, inner blades darker; one body pen.", "Äußere Spreiten hellgrau, innere dunkler; ein Körperstift."),
        ("Each blade maps to one region (outer or inner).", "Jede Spreite entspricht einer Region (außen oder innen).")))
    r["100"] = rep(100, 76, 150, pens(0.2, 0.13, 0.1, None, 0.3), [], build_100(), dec(
        ("At 1:100 (1 m is 10 mm) the nine blades are drawn with their midribs and the petioles appear as short filled stems between blade and crown; blades are 2.6 mm wide.", "Bei 1:100 (1 m sind 10 mm) sind die neun Spreiten mit Mittelrippe gezeichnet und die Blattstiele erscheinen als kurze gefüllte Stiele zwischen Spreite und Wurzelstock; die Spreiten sind 2,6 mm breit."),
        ("Retained: midribs, petioles as filled strips (12 mm becomes 0.25 mm), blade overlap order.", "Beibehalten: Mittelrippen, Blattstiele als gefüllte Streifen (12 mm werden 0,25 mm), Überlagerungsfolge der Spreiten."),
        ("Merged: 2 to 3 real leaves per blade.", "Zusammengefasst: 2 bis 3 wirkliche Blätter je Spreite."),
        ("Omitted: side veins and margin waves.", "Weggelassen: Seitenadern und Randwellen."),
        ("Petioles are drawn at 25 mm (0.25 mm) instead of 12 mm to stay above the minimum feature, a thickening of 13 mm.", "Blattstiele sind mit 25 mm (0,25 mm) statt 12 mm gezeichnet, damit sie das Mindestmerkmal einhalten, eine Verdickung um 13 mm."),
        ("None.", "Keine."),
        ("Blades grey filled, midribs and petioles darker.", "Spreiten grau gefüllt, Mittelrippen und Blattstiele dunkler."),
        ("Blades carry their regions, petioles theirs.", "Spreiten tragen ihre Regionen, Blattstiele ihre eigene.")))
    r["50"] = rep(50, 20, 75, pens(0.25, 0.18, 0.13, None, 0.35), [], build_50(), dec(
        ("At 1:50 (1 m is 20 mm) every blade has its cordate-ovate outline with a slightly wavy margin, a midrib and four pairs of arching side veins, and the petioles appear as tapered strips.", "Bei 1:50 (1 m sind 20 mm) hat jede Spreite ihren herzförmig-eiförmigen Umriss mit leicht welligem Rand, eine Mittelrippe und vier Paare bogiger Seitenadern, und die Blattstiele erscheinen als verjüngte Streifen."),
        ("Retained: wavy blade margin, midrib, side veins, petioles with their taper, overlap order.", "Beibehalten: welliger Spreitenrand, Mittelrippe, Seitenadern, Blattstiele mit ihrer Verjüngung, Überlagerungsfolge."),
        ("Merged: 2 to 3 real leaves per blade.", "Zusammengefasst: 2 bis 3 wirkliche Blätter je Spreite."),
        ("Omitted: the scape and flowers of summer (not part of this asset), blade puckering.", "Weggelassen: Schaft und Blüten des Sommers (nicht Teil dieses Assets), Spreitenkräuselung."),
        ("Petioles (12 mm) are 16 mm wide strips (0.32 mm); veins are 0.13 mm strokes.", "Blattstiele (12 mm) sind Streifen von 16 mm (0,32 mm); Adern sind Striche von 0,13 mm."),
        ("None; veins are strokes.", "Keine; Adern sind Striche."),
        ("Outer blades lighter, inner blades darker, veins as texture strokes.", "Äußere Spreiten heller, innere dunkler, Adern als Texturstriche."),
        ("Blades carry their regions; veins stay neutral.", "Spreiten tragen ihre Regionen; Adern bleiben neutral.")))
    return r


def asset():
    A, em, regions, mats, poses = make()
    sym = {"id": "plan-extent", "title": T("Plan symbol with true clump extent", "Grundrisssymbol mit wahrer Horstausdehnung"), "description": T("A plan sign for planting plans: the true outline of the overlapping leaf blades of the mound (0.9 m across); the replacement for the unavailable 1:500 elevation.", "Ein Grundrisszeichen für Pflanzpläne: der wahre Umriss der überlappenden Spreiten des Hügels (0,9 m Durchmesser); der Ersatz für die nicht verfügbare Ansicht 1:500."), "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": {"elevation": [], "plan": plan_symbol()}, "dimensionalNote": T("The ellipses are the real blades (0.54 m by 0.3 m) seen from above; at 1:500 the whole sign is 1.8 mm across.", "Die Ellipsen sind die wirklichen Spreiten (0,54 m mal 0,3 m) von oben; bei 1:500 misst das ganze Zeichen 1,8 mm.")}
    a = {"id": ID, "category": "vegetation", "rig": {"kind": "botanical", "root": "root-plate", "bones": B.list}, "anchors": A,
         "title": T("Hosta Clump", "Funkienhorst"),
         "summary": T("A clump of hosta (Hosta sieboldiana type) 0.62 m high and 0.9 m wide: a mound of broad, ribbed, glaucous-green blades on petioles that arch out of the crown; the outer blades touch the soil. A low shade perennial for the foreground of planting.", "Ein Horst Funkien (Hosta sieboldiana-Typ) von 0,62 m Höhe und 0,9 m Breite: ein Hügel aus breiten, gerippten, blaugrünen Spreiten auf Stielen, die sich aus dem Wurzelstock wölben; die äußeren Spreiten berühren den Boden. Eine niedrige Schattenstaude für den Vordergrund der Pflanzung."),
         "tags": ["hosta", "perennial", "shade", "broad-leaf", "transpiration", "low"],
         "provenance": provenance(["schmid-1991", "jones-2014", "monteith-unsworth-2013", "en-1991-1-4", "en-1991-1-3"], T("Nine leaf groups (petiole and blade bones, 18 in all), each standing for two to three of the clump's 25 leaves, are authored by hand; the 1:500 elevation is declared unavailable with a plan symbol as the replacement. Leaf areas and conductances are authored estimates guided by Jones (2014) and Schmid (1991).", "Neun Blattgruppen (Stiel- und Spreitenknochen, zusammen 18), die je für zwei bis drei der 25 Blätter des Horsts stehen, sind von Hand entworfen; die Ansicht 1:500 ist als nicht verfügbar erklärt, mit einem Grundrisssymbol als Ersatz. Blattflächen und Leitfähigkeiten sind Autorenschätzungen angelehnt an Jones (2014) und Schmid (1991).")),
         "dimensions": {"heightM": 0.62, "widthM": 0.88, "depthM": 0.85, "massKg": MASS},
         "materials": mats, "representations": representations(), "symbols": [sym], "poses": poses, "clips": clips(poses),
         "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 21, "relativeHumidity": 0.65, "windMS": 0.5, "windDirectionDeg": 0, "solarWM2": 250, "diffuseFraction": 0.6, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": regions, "emitters": em,
                     "assumptions": [T("Leaf regions use one-sided area of the 25 real leaves (1.05 m2 over a ground projection of 0.64 m2): leaf area index 0.7 outer and 0.9 inner. Maximum stomatal conductances (0.0045 and 0.004 m/s) are authored estimates for a shade-tolerant C3 perennial (Jones 2014); thin blades of 0.3 mm give 1.1 kJ/(m2 K). The reference case is a shaded bed (250 W/m2, mostly diffuse).", "Blattregionen verwenden die einseitige Fläche der 25 wirklichen Blätter (1,05 m2 über einer Bodenprojektion von 0,64 m2): Blattflächenindex 0,7 außen und 0,9 innen. Maximale Spaltöffnungsleitfähigkeiten (0,0045 und 0,004 m/s) sind Autorenschätzungen für eine schattenverträgliche C3-Staude (Jones 2014); dünne Spreiten von 0,3 mm ergeben 1,1 kJ/(m2 K). Der Referenzfall ist ein Schattenbeet (250 W/m2, überwiegend diffus)."),
                                     T("Petioles are passive, water-rich (11 kJ/(m2 K)). Emitters release the latent flux of their blade region over four and five anchors on the blades.", "Blattstiele sind passiv und wasserreich (11 kJ/(m2 K)). Emitter geben den latenten Strom ihrer Spreitenregion über vier und fünf Anker auf den Spreiten ab.")]},
         "structure": structure(["base-support"], "One rigid body per evaluated pose, fixed at the root ball; wind drag acts on each blade (0.06 m2) and rain water adds 0.04 kg per blade. Petioles are soft, water-rich beams without authored sections; no failure is predicted.", "Ein starrer Körper je ausgewerteter Pose, am Wurzelballen eingespannt; Winddruck wirkt auf jede Spreite (0,06 m2) und Regenwasser fügt 0,04 kg je Spreite hinzu. Blattstiele sind weiche, wasserreiche Balken ohne angegebene Querschnitte; es wird kein Versagen vorhergesagt."),
         "presentation": presentation("summer-full", "wind-flutter", 50), "exports": EXPORTS, "completion": "authored"}
    return a


if __name__ == "__main__":
    doc = document(asset())
    validate_text(doc)
    print(dump(doc, ID), MASS)
