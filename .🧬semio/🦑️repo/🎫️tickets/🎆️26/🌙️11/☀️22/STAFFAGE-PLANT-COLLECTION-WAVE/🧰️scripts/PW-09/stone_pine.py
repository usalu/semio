"""🌲 Stone pine (Pinus pinea), umbrella pine of about 20 m (PW-09): species re-authoring of the morphed `scots-pine`.

Species data and sources: `../../📝️PW-09.md`. Geometry: `stone_pine_geometry.py`.
"""
import math

import stone_pine_geometry as geometry
from species import rename_ids, replace_words, retone, scale_loads, section_titles, set_texts

ID = "stone-pine"
TEMPLATE = geometry.TEMPLATE
WORDS = [r"Scots(?! pine of this archive)", r"(?<!der )Waldkiefer(?! dieses Archivs)", r"sylvestris", r"orange", r"Spiegelrinde", r"two-tone", r"zweifarbig", r"flaking"]
IDS = {
    "scots-pine": ID, "scots-pine-symbol": "stone-pine-symbol",
    "pine-bark-lower": "stone-pine-bark", "pine-bark-upper": "stone-pine-bark-upper", "pine-needles-sun": "stone-pine-needles-sun",
    "pine-needles-shade": "stone-pine-needles-shade", "pine-wood": "stone-pine-wood",
}
ENVIRONMENT = {"airTempC": 32, "meanRadiantTempC": 36, "skyTempC": 21, "groundTempC": 50, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 850, "diffuseFraction": 0.12, "pressurePa": 101325}
REGION_BONES = {"bark-lower": ["root-plate", "trunk-1", "stub-w", "stub-e"], "bark-upper": ["trunk-2", "trunk-3", "leader"], "limb-bark": ["limb-w3", "limb-e3", "limb-w1", "limb-e1", "limb-w2", "limb-e2"]}
TITLES = {
    "root-plate": ("Root plate", "Wurzelteller"),
    "trunk-1": ("Bole, lower", "Stamm, unten"),
    "trunk-2": ("Bole, middle", "Stamm, Mitte"),
    "trunk-3": ("Bole, upper (crown fork)", "Stamm, oben (Kronengabel)"),
    "leader": ("Crown top branch", "Gipfelast"),
    "stub-w": ("Dead branch stub, west", "Abgestorbener Aststummel, West"),
    "stub-e": ("Dead branch stub, east", "Abgestorbener Aststummel, Ost"),
    "limb-w3": ("Limb, lower west", "Ast, unten West"), "limb-e3": ("Limb, lower east", "Ast, unten Ost"),
    "limb-w1": ("Limb, middle west", "Ast, Mitte West"), "limb-e1": ("Limb, middle east", "Ast, Mitte Ost"),
    "limb-w2": ("Limb, upper west", "Ast, oben West"), "limb-e2": ("Limb, upper east", "Ast, oben Ost"),
    "pl-w3": ("Needle plate, lower west", "Nadelplatte, unten West"), "pl-e3": ("Needle plate, lower east", "Nadelplatte, unten Ost"),
    "pl-w1": ("Needle plate, middle west", "Nadelplatte, Mitte West"), "pl-e1": ("Needle plate, middle east", "Nadelplatte, Mitte Ost"),
    "pl-w2": ("Needle plate, upper west", "Nadelplatte, oben West"), "pl-e2": ("Needle plate, upper east", "Nadelplatte, oben Ost"),
    "pl-top": ("Needle plate, crown top", "Nadelplatte, Kronendach"),
}


def lateral(asset, ids):
    by = {b["id"]: b for b in asset["rig"]["bones"]}
    return sum(math.pi * math.sqrt(4 * by[i]["section"]["areaM2"] / math.pi) * by[i]["lengthM"] for i in ids)


def materials(asset):
    m = {x["id"]: x for x in asset["materials"]}
    m["stone-pine-bark"].update({"title": {"en": "Thick red-brown plated bark with deep grey furrows", "de": "Dicke rotbraune Plattenborke mit tiefen grauen Furchen"}, "solarAbsorptivity": 0.75, "densityKgM3": 500})
    m["stone-pine-bark-upper"].update({"title": {"en": "Red-brown plated bark of the upper bole and the limbs", "de": "Rotbraune Plattenborke des oberen Stamms und der Äste"}, "solarAbsorptivity": 0.68, "densityKgM3": 480})
    m["stone-pine-needles-sun"].update({"title": {"en": "Stone pine needles, sunlit canopy (stiff pairs, 10 to 20 cm)", "de": "Pinienadeln, besonntes Kronendach (steife Paare, 10 bis 20 cm)"}, "solarAbsorptivity": 0.52, "source": "authored-estimate", "reference": "jones-2014"})
    m["stone-pine-needles-shade"].update({"title": {"en": "Stone pine needles, shaded underside of the canopy", "de": "Pinienadeln, beschattete Unterseite des Kronendachs"}, "solarAbsorptivity": 0.3})
    m["stone-pine-wood"].update({"title": {"en": "Green stone pine wood", "de": "Frisches Pinienholz"}, "densityKgM3": 900, "source": "authored-estimate", "reference": "wood-handbook-2021"})


def tone(item, t):
    mat = str(item.get("material", "")) + str(item.get("region", ""))
    if "needles" in mat:
        return max(0.2, t - 0.04)
    if "bark-upper" in mat or "limb-bark" in mat:
        return max(0.2, t - 0.08)
    return t


def thermal(asset, old_lateral):
    t = asset["thermal"]
    t["environment"] = dict(ENVIRONMENT)
    t["referenceEnvironment"] = "summer-hot-sun"
    regions = {x["id"]: x for x in t["regions"]}
    for rid, ids in REGION_BONES.items():
        regions[rid]["areaM2"] = round(regions[rid]["areaM2"] * lateral(asset, ids) / old_lateral[rid], 2)
    regions["bark-lower"].update({"title": {"en": "Lower bole, thick red-brown plated bark", "de": "Unterer Stamm, dicke rotbraune Plattenborke"}, "characteristicLengthM": 0.65, "heatCapacityJM2K": 80000})
    regions["bark-upper"].update({"title": {"en": "Upper bole and crown fork, red-brown plated bark", "de": "Oberer Stamm und Kronengabel, rotbraune Plattenborke"}, "characteristicLengthM": 0.4})
    regions["limb-bark"].update({"title": {"en": "Bark of the rising limbs", "de": "Rinde der aufsteigenden Äste"}})
    regions["needles-sun"].update({"title": {"en": "Needles of the sunlit upper canopy", "de": "Nadeln des besonnten oberen Kronendachs"}, "areaM2": 190, "characteristicLengthM": 0.0015, "heatCapacityJM2K": 1300, "leaf": {"stomatalConductanceMaxMS": 0.003, "leafAreaIndex": 1.3, "photosynthesis": "c3"}, "reference": "mutke-2012"})
    regions["needles-shade"].update({"title": {"en": "Needles of the shaded underside and the lower plates", "de": "Nadeln der beschatteten Unterseite und der unteren Platten"}, "areaM2": 110, "characteristicLengthM": 0.0015, "heatCapacityJM2K": 1300, "leaf": {"stomatalConductanceMaxMS": 0.002, "leafAreaIndex": 0.75, "photosynthesis": "c3"}, "reference": "mutke-2012"})
    t["emitters"][0]["title"] = {"en": "Transpiration of the sun needles (distributed over the canopy)", "de": "Transpiration der Sonnennadeln (über das Kronendach verteilt)"}
    t["emitters"][1]["title"] = {"en": "Transpiration of the shade needles (distributed over the canopy)", "de": "Transpiration der Schattennadeln (über das Kronendach verteilt)"}
    t["assumptions"][0] = {
        "en": "Leaf regions use one-sided needle area; the leaf area index is that area over the crown ground projection (about 145 m2 for the 14 m umbrella, 300 m2 of needles). Stone pine needles grow in stiff pairs of 10 to 20 cm and about 1.5 mm width; the maximum stomatal conductance (0.003 m/s sun, 0.002 m/s shade) is an authored estimate for a drought-adapted Mediterranean pine, guided by Jones (2014) and Mutke et al. (2012). Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.",
        "de": "Blattregionen verwenden die einseitige Nadelfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden (etwa 145 m2 für den 14 m breiten Schirm, 300 m2 Nadeln). Pinienadeln wachsen in steifen Paaren von 10 bis 20 cm Länge und etwa 1,5 mm Breite; die maximale Spaltöffnungsleitfähigkeit (0,003 m/s Sonne, 0,002 m/s Schatten) ist eine Autorenschätzung für eine an Trockenheit angepasste mediterrane Kiefer, angelehnt an Jones (2014) und Mutke et al. (2012). Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben.",
    }


def poses(asset):
    pads = geometry.BONES["pl-w1"][0] ** 2
    scale_loads(asset, "wind", pads)
    scale_loads(asset, "snow", pads)
    for p in asset["poses"]:
        for ld in p["loads"]:
            if ld["id"] == "wind-trunk-1":
                ld["distributed"]["wxNPerM"] = round(ld["distributed"]["wxNPerM"] * geometry.BONES["trunk-1"][1] / pads, 3)
                ld["reference"] = "en-1991-1-4"
            if ld["kind"] in ("wind", "snow") and ld["id"] != "wind-trunk-1":
                b = ld["bone"]
                en, de = TITLES[b]
                ld["title"] = {"en": f"{'Drag on' if ld['kind'] == 'wind' else 'Snow on'} the {en[0].lower() + en[1:]}", "de": f"{'Widerstand am' if ld['kind'] == 'wind' else 'Schnee auf dem'} {de}"}


def provenance(asset):
    pv = asset["provenance"]
    pv["author"] = "Semio staffage vegetation content worker PW-09"
    pv["method"] = "handcrafted"
    pv["created"] = "2026-11-22"
    refs = [x for x in pv["references"] if x["id"] not in ("pretzsch-2015",)]
    refs += [
        {"id": "mutke-2012", "citation": "Mutke, S., Calama, R., González-Martínez, S. C. et al. (2012). Mediterranean stone pine: botany and horticulture. Horticultural Reviews 39, 153-201. Umbrella habit, size, needles, cones and growth of Pinus pinea; used as a guide, values here are authored estimates."},
        {"id": "mutke-2005", "citation": "Mutke, S., Gordo, J. and Gil, L. (2005). Variability of Mediterranean stone pine cone production: yield loss as response to climate change. Agricultural and Forest Meteorology 132(3-4), 263-272."},
    ]
    pv["references"] = refs


def bone_titles(asset):
    for b in asset["rig"]["bones"]:
        en, de = TITLES[b["id"]]
        b["title"] = {"en": en, "de": de}
        if "section" in b:
            b["section"]["title"] = {"en": f"{en}, d = 0 m", "de": f"{de}, d = 0 m"}
    section_titles(asset)


def n_en(x):
    return f"{x:.1f}"


def n_de(x):
    return f"{x:.1f}".replace(".", ",")


def textual(asset):
    d = asset["dimensions"]
    H, W, D = d["heightM"], d["widthM"], d["depthM"]
    by = {b["id"]: b for b in asset["rig"]["bones"]}
    dbole = math.sqrt(4 * by["trunk-1"]["section"]["areaM2"] / math.pi)
    fork = by["root-plate"]["lengthM"] + by["trunk-1"]["lengthM"]
    replace_words(asset, [("Scots pine", "stone pine"), ("Scots Pine", "Stone Pine"), ("orange upper bole", "upper bole"), ("the two-tone bole", "the bole"), ("two-tone bole", "bole"),
                          ("flaking bark", "plated bark"), ("flake marks", "plate marks"), ("orange", "red-brown")],
                  [("Waldkiefer", "Pinie"), ("oranger Spiegelrinde", "rotbrauner Plattenborke"), ("orange Spiegelrinde", "rotbraune Plattenborke"), ("Spiegelrinde", "Plattenborke"),
                   ("zweifarbiger Schaft", "Schaft"), ("zweifarbige Schaft", "Schaft"), ("zweifarbigen Schaft", "Schaft"), ("der orange obere", "der obere"), ("orangen", "rotbraunen"), ("oranger", "rotbrauner"), ("orange", "rotbraune")])
    T = {
        ".title": ("Stone Pine, Umbrella Crown", "Pinie, Schirmkrone"),
        ".summary": (
            f"A mature stone pine (Pinus pinea) of {n_en(H)} m: a long, straight bole of {dbole:.2f} m in thick red-brown plated bark that forks at about {n_en(fork)} m into a few heavy, steeply rising limbs, carrying the dense, flat-topped umbrella crown {n_en(W)} m across that marks Mediterranean avenues and coasts.",
            f"Eine ausgewachsene Pinie (Pinus pinea) von {n_de(H)} m: ein langer, gerader Stamm von {dbole:.2f} m".replace(".", ",", 1) + f" in dicker rotbrauner Plattenborke, der sich in etwa {n_de(fork)} m in wenige schwere, steil aufsteigende Äste gabelt und die dichte, flache Schirmkrone von {n_de(W)} m Durchmesser trägt, die mediterrane Alleen und Küsten prägt.",
        ),
        ".provenance.notes": (
            f"Rig and drawings are derived from the handcrafted Scots pine of this archive by an exact per-bone and world-diagonal morph (bole lengthened 1.5 times and thickened 1.3 times, the crown from the top of the bole flattened to 0.55 and widened 1.1 and 1.4 times, the needle plates enlarged 1.25 to 1.35 times and carried to their new places without being flattened) and re-authored for an open-grown stone pine of about 100 years: a clear bole of {dbole:.2f} m to about {n_en(fork)} m and an umbrella crown {n_en(W)} by {n_en(D)} m. Stone pines commonly reach 15 to 25 m with crowns of 10 to 20 m. Masses are volume times green density plus needle and twig estimates; the heavy cones (up to 0.4 kg each, three years to ripen) are not drawn.",
            f"Rig und Zeichnungen sind aus der handgezeichneten Waldkiefer dieses Archivs durch eine exakte Abbildung je Knochen und weltdiagonale Abbildungen abgeleitet (Stamm 1,5-fach verlängert und 1,3-fach verdickt, die Krone ab dem Stammkopf auf 0,55 abgeflacht und 1,1- und 1,4-fach verbreitert, die Nadelplatten 1,25- bis 1,35-fach vergrößert und ohne Abflachung an ihre neuen Orte getragen) und für eine freistehende Pinie von etwa 100 Jahren neu bearbeitet: ein astfreier Stamm von {dbole:.2f} m".replace(".", ",", 1) + f" bis etwa {n_de(fork)} m und eine Schirmkrone von {n_de(W)} auf {n_de(D)} m. Pinien erreichen meist 15 bis 25 m mit Kronen von 10 bis 20 m. Massen sind Volumen mal Frischdichte zuzüglich Schätzungen für Nadeln und Zweige; die schweren Zapfen (bis 0,4 kg, drei Jahre bis zur Reife) sind nicht gezeichnet.",
        ),
        ".symbols[stone-pine-symbol].description": (
            "A conventional plan sign of the plant for site, avenue and landscape plans: outline of the true umbrella extent and the stem section. It is a separate symbol and is never used silently for a pictorial scale.",
            "Ein konventionelles Grundrisszeichen der Pflanze für Lage-, Alleen- und Landschaftspläne: Umriss der wahren Schirmausdehnung und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.",
        ),
        ".symbols[stone-pine-symbol].dimensionalNote": (
            f"A conventional simplification of the plant drawn at its true extent in model metres in all three views ({W:.2f} m wide, {D:.2f} m deep and {H:.2f} m high): the outline is the crown, the stem is drawn at its real position and thickness. It is a symbol, never used silently for a pictorial scale.",
            f"Eine konventionelle Vereinfachung der Pflanze in wahrer Ausdehnung in Modellmetern in allen drei Ansichten ({W:.2f} m breit, {D:.2f} m tief und {H:.2f} m hoch)".replace(".", ",") + ": der Umriss ist die Krone, der Stamm ist an seiner wirklichen Lage und in seiner wirklichen Dicke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.",
        ),
        ".poses[calm].title": ("Calm hot summer day", "Windstiller heißer Sommertag"),
        ".poses[calm].description": (
            "Light air of 1.5 m/s at 10 m on a hot, dry summer day: the bole stands vertical and the umbrella is at rest; the sunlit top of the canopy shades its underside.",
            "Leiser Zug von 1,5 m/s in 10 m Höhe an einem heißen, trockenen Sommertag: der Stamm steht senkrecht und der Schirm ruht; das besonnte Kronendach beschattet seine Unterseite.",
        ),
        ".poses[windy].title": ("Fresh breeze", "Frische Brise"),
        ".poses[windy].description": (
            "Beaufort 5, 8 m/s at 10 m (about 1.15 times at crown height), cd 0.4 on the frontal area of each needle plate: all of the drag acts on the high, wide umbrella, so the base moment is large for the load. The bole bends progressively, the plates swing up to 7 degrees.",
            "Beaufort 5, 8 m/s in 10 m Höhe (etwa 1,15-fach in Kronenhöhe), cd 0,4 auf die Anströmfläche jeder Nadelplatte: der ganze Widerstand wirkt am hohen, breiten Schirm, das Fußmoment ist daher für die Last groß. Der Stamm biegt sich zunehmend, die Platten schwingen bis zu 7 Grad.",
        ),
        ".poses[storm].title": ("Gale", "Sturm"),
        ".poses[storm].description": (
            "Beaufort 9, 22 m/s at 10 m, cd 0.25 after needle streamlining (Mayhead 1973 for pine crowns): several kilonewtons on the umbrella crown and a base moment far above that of a narrow-crowned pine of the same height; the critical case for windthrow of stone pines on shallow soils. Joints approach their limits.",
            "Beaufort 9, 22 m/s in 10 m Höhe, cd 0,25 nach Stromlinienanpassung der Nadeln (Mayhead 1973 für Kiefernkronen): mehrere Kilonewton auf die Schirmkrone und ein Fußmoment weit über dem einer schmalkronigen Kiefer gleicher Höhe; der maßgebende Fall für den Windwurf von Pinien auf flachgründigen Böden. Die Gelenke erreichen fast ihre Grenzen.",
        ),
        ".poses[snow].title": ("Wet snow on the umbrella", "Nassschnee auf dem Schirm"),
        ".poses[snow].description": (
            "A rare Mediterranean snowfall on the evergreen umbrella: 15 kg/m2 of wet snow on the plan area of each flat needle plate (order of magnitude after EN 1991-1-3) bows the rising limbs and tips the plates outwards and down; wind is calm. The flat crown collects far more snow than a conical conifer, the reason for the limb breakage of stone pines in heavy snow.",
            "Ein seltener mediterraner Schneefall auf dem immergrünen Schirm: 15 kg/m2 Nassschnee auf der Grundrissfläche jeder flachen Nadelplatte (Größenordnung nach EN 1991-1-3) biegt die aufsteigenden Äste und kippt die Platten nach außen und unten; Windstille. Die flache Krone sammelt weit mehr Schnee als ein kegelförmiger Nadelbaum, der Grund für die Astbrüche von Pinien bei starkem Schneefall.",
        ),
        ".clips[wind-sway].description": (
            f"A deterministic 3.6 s sway cycle (first mode of a tall bole with a heavy, wide umbrella, estimated 0.28 Hz) about the fresh-breeze lean: the bole bends gently, the needle plates lag behind and swing up to 3 degrees.",
            "Ein deterministischer Schwingungszyklus von 3,6 s (erste Eigenform eines hohen Stamms mit schwerem, breitem Schirm, geschätzt 0,28 Hz) um die Neigung bei frischer Brise: der Stamm biegt sich sanft, die Nadelplatten folgen verzögert und schwingen bis zu 3 Grad.",
        ),
        ".clips[diurnal-transpiration].description": (
            "A 24 h cycle of a hot summer day compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night and partly closed at midday; needle and bark temperatures are not authored but follow from the thermal solver.",
            "Ein 24-h-Zyklus eines heißen Sommertags auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei Windstille. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen, mittags teilweise; Nadel- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser.",
        ),
        ".representations.500.decisions.silhouette": (
            f"At 1:500 (1 m is 2 mm) the stone pine is a long bare bole with its root flare and, above about {n_en(fork)} m, six bare rising limbs carrying the dense, flat-topped umbrella of nine needle plates, drawn as one crown shape {W * 2:.0f} mm wide; the drawings are the true views of the 1:500 body. Plan: the umbrella as one outline.",
            f"Bei 1:500 (1 m sind 2 mm) ist die Pinie ein langer kahler Stamm mit Wurzelanlauf und über etwa {n_de(fork)} m sechs kahle, aufsteigende Äste, die den dichten, flachen Schirm aus neun Nadelplatten tragen, gezeichnet als eine Kronenform von {W * 2:.0f} mm Breite; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers. Grundriss: der Schirm als ein Umriss.",
        ),
        ".representations.500.decisions.retained": (
            "Retained: the bole in two bark tones (thick plated lower bark, the red-brown plates above), the six limbs as tapered outlines, the umbrella outline.",
            "Beibehalten: der Stamm in zwei Rindentönen (dicke Plattenborke unten, die rotbraunen Platten darüber), die sechs Äste als verjüngte Umrisse, der Schirmumriss.",
        ),
        ".representations.500.decisions.styles": (
            "Light line: closed outline only. Dark solid: the umbrella reads as one dark mass, the bole lighter. Both stay readable at 40 mm printed height or less.",
            "Helle Linie: nur geschlossener Umriss. Dunkle Fläche: der Schirm liest sich als eine dunkle Masse, der Stamm heller. Beides bleibt bei 40 mm Druckhöhe oder weniger lesbar.",
        ),
        ".representations.200.decisions.hatching": (
            f"No hatching. Internal edges: the change of bark between the lower and the upper bole at {n_en(fork)} m is the only internal edge.",
            f"Keine Schraffur. Innenkanten: der Rindenwechsel zwischen unterem und oberem Stamm in {n_de(fork)} m ist die einzige Innenkante.",
        ),
        ".representations.200.decisions.overlays": (
            "Every thermal region keeps its own shape: bark-lower and bark-upper (two bole paths); overlays follow the bone-bound masses, so sway and loads act on the drawn shapes.",
            "Jede thermische Region behält ihre eigene Form: bark-lower und bark-upper (zwei Stammpfade); Overlays folgen den knochengebundenen Massen, sodass Schwanken und Lasten auf die gezeichneten Formen wirken.",
        ),
        ".volumes.50.decisions[0]": (
            "Body at 1:50: the root flare with five surface roots, the long lower bole in thick plated bark with two knots, the two dead stubs (one forked) and a snag, the upper bole in three parts with whorl swellings, two snags and two knots, six crooked main limbs with branch collars that rise steeply and turn out towards their ends, and nine dense needle plates that close into the flat umbrella. Derived from the Scots pine of this archive (bole lengthened and thickened, crown flattened and widened, plates enlarged).",
            "Körper bei 1:50: der Wurzelanlauf mit fünf Oberflächenwurzeln, der lange untere Stamm in dicker Plattenborke mit zwei Astknoten, die zwei abgestorbenen Stummel (einer gegabelt) und ein Aststumpf, der obere Stamm in drei Teilen mit Quirlverdickungen, zwei Aststümpfen und zwei Astknoten, sechs knorrige Hauptäste mit Astringen, die steil aufsteigen und sich zu ihren Enden hin nach außen wenden, und neun dichte Nadelplatten, die sich zum flachen Schirm schließen. Abgeleitet aus der Waldkiefer dieses Archivs (Stamm verlängert und verdickt, Krone abgeflacht und verbreitert, Platten vergrößert).",
        ),
    }
    set_texts(asset, T)


def build(doc, measure=None):
    a = doc["asset"]
    old_lateral = {rid: lateral(a, ids) for rid, ids in REGION_BONES.items()}
    geometry.morph(a)
    if measure is not None:
        from morph import measured_dimensions
        a["dimensions"].update(measured_dimensions(measure))
    a["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"]))
    rename_ids(doc, IDS)
    a = doc["asset"]
    a["tags"] = ["tree", "evergreen", "conifer", "mediterranean", "umbrella-crown", "flat-crown", "wind", "snow", "transpiration"]
    materials(a)
    retone(a, tone)
    thermal(a, old_lateral)
    poses(a)
    provenance(a)
    bone_titles(a)
    textual(a)
    return doc


DISTINCTION = (
    "A tall Mediterranean stone pine with a long, straight bole in red-brown plated bark and a dense, flat-topped umbrella crown on a few rising limbs.",
    "Eine hohe mediterrane Pinie mit langem, geradem Stamm in rotbrauner Plattenborke und einer dichten, flachen Schirmkrone auf wenigen aufsteigenden Ästen.",
)
