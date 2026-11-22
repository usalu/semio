"""🍈 Fig tree (Ficus carica), multi-stem, broad, about 5 m (PW-09): species re-authoring of the morphed `japanese-maple`.

Species data and sources: `../../📝️PW-09.md`. Geometry: `fig_geometry.py`.
"""
import math

import fig_geometry as geometry
from species import rename_ids, replace_words, retone, scale_loads, section_titles, set_texts

ID = "fig-tree"
TEMPLATE = geometry.TEMPLATE
WORDS = [r"[Mm]aple(?! of this archive)", r"(?<!Fächer)Ahorn(?! dieses Archivs)", r"palmatum", r"Japan(?!ese maple of this archive)", r"[Aa]utumn colou?r", r"[Hh]erbstf[äa]rb", r"seven-lobed", r"siebenlappig", r"tier", r"Etage", r"layered", r"geschichtet"]
IDS = {
    "japanese-maple": ID, "japanese-maple-symbol": "fig-tree-symbol",
    "maple-bark": "fig-bark", "maple-bark-young": "fig-bark-young", "maple-leaf-sun": "fig-leaf-sun", "maple-leaf-shade": "fig-leaf-shade", "maple-wood": "fig-wood",
    "summer-calm": "calm", "summer-breeze": "windy", "autumn-colour": "fruit-laden", "breeze-sway": "wind-sway",
}
ENVIRONMENT = {"airTempC": 32, "meanRadiantTempC": 36, "skyTempC": 21, "groundTempC": 50, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 850, "diffuseFraction": 0.12, "pressurePa": 101325}
REGION_BONES = {"stem-bark": ["root-plate", "stem-c", "stem-w", "stem-e", "stem-c-top"], "limb-bark": ["t1w", "t1e", "t2w", "t2e", "t3w", "t3e"]}
FRUIT_KG = 15
FOLIAGE_BONES = ["fol-core-low", "fol-core-up", "fol-t1w", "fol-t1e", "fol-t2w", "fol-t2e", "fol-t3w", "fol-t3e", "fol-top"]
TITLES = {
    "root-plate": ("Root collar", "Wurzelhals"),
    "stem-c": ("Central stem", "Mittlerer Stamm"), "stem-w": ("West stem", "Weststamm"), "stem-e": ("East stem", "Oststamm"),
    "stem-c-top": ("Central stem, top", "Mittlerer Stamm, oben"),
    "t1w": ("Lower west limb", "Unterer Westast"), "t1e": ("Lower east limb", "Unterer Ostast"),
    "t2w": ("Middle west limb", "Mittlerer Westast"), "t2e": ("Middle east limb", "Mittlerer Ostast"),
    "t3w": ("Upper west limb", "Oberer Westast"), "t3e": ("Upper east limb", "Oberer Ostast"),
    "fol-core-low": ("Shaded lower core", "Beschatteter Kern unten"), "fol-core-up": ("Shaded upper core", "Beschatteter Kern oben"),
    "fol-t1w": ("Lower west leaf mass", "Untere westliche Laubmasse"), "fol-t1e": ("Lower east leaf mass", "Untere östliche Laubmasse"),
    "fol-t2w": ("Middle west leaf mass", "Mittlere westliche Laubmasse"), "fol-t2e": ("Middle east leaf mass", "Mittlere östliche Laubmasse"),
    "fol-t3w": ("Upper west leaf mass", "Obere westliche Laubmasse"), "fol-t3e": ("Upper east leaf mass", "Obere östliche Laubmasse"),
    "fol-top": ("Crown top leaf mass", "Laubmasse Kronenspitze"),
}
LOAD_TITLES = {
    "core-low": ("shaded lower core", "beschatteten Kern unten"), "core-up": ("shaded upper core", "beschatteten Kern oben"),
    "t1w": ("lower west leaf mass", "unteren westlichen Laubmasse"), "t1e": ("lower east leaf mass", "unteren östlichen Laubmasse"),
    "t2w": ("middle west leaf mass", "mittleren westlichen Laubmasse"), "t2e": ("middle east leaf mass", "mittleren östlichen Laubmasse"),
    "t3w": ("upper west leaf mass", "oberen westlichen Laubmasse"), "t3e": ("upper east leaf mass", "oberen östlichen Laubmasse"),
    "top": ("crown top leaf mass", "Laubmasse der Kronenspitze"),
}


def lateral(asset, ids):
    by = {b["id"]: b for b in asset["rig"]["bones"]}
    return sum(math.pi * math.sqrt(4 * by[i]["section"]["areaM2"] / math.pi) * by[i]["lengthM"] for i in ids)


def materials(asset):
    m = {x["id"]: x for x in asset["materials"]}
    m["fig-bark"].update({"title": {"en": "Fig stem bark, smooth and silver-grey", "de": "Feigenstammrinde, glatt und silbergrau"}, "solarAbsorptivity": 0.5, "densityKgM3": 550})
    m["fig-bark-young"].update({"title": {"en": "Smooth grey-green bark of the limbs and shoots", "de": "Glatte graugrüne Rinde der Äste und Triebe"}, "solarAbsorptivity": 0.55, "densityKgM3": 600})
    m["fig-leaf-sun"].update({"title": {"en": "Fig sun leaves (large, lobed, rough)", "de": "Feigen-Sonnenblätter (groß, gelappt, rau)"}, "solarAbsorptivity": 0.55, "source": "authored-estimate", "reference": "jones-2014"})
    m["fig-leaf-shade"].update({"title": {"en": "Fig shade leaves (effective)", "de": "Feigen-Schattenblätter (wirksam)"}, "solarAbsorptivity": 0.28})
    m["fig-wood"].update({"title": {"en": "Green fig wood (soft, light, latex-bearing)", "de": "Frisches Feigenholz (weich, leicht, milchsaftführend)"}, "densityKgM3": 800, "conductivityWmK": 0.13, "elasticModulusPa": 4000000000, "strengthPa": 28000000})


def tone(item, t):
    mat = str(item.get("material", "")) + str(item.get("region", ""))
    if "leaf" in mat:
        return max(0.2, t - 0.06)
    if "stem-bark" in mat or mat.startswith("fig-bark") or "limb-bark" in mat:
        return min(0.62, t + 0.12)
    return t


def thermal(asset, old_lateral):
    t = asset["thermal"]
    t["environment"] = dict(ENVIRONMENT)
    t["referenceEnvironment"] = "summer-hot-sun"
    regions = {x["id"]: x for x in t["regions"]}
    for rid, ids in REGION_BONES.items():
        regions[rid]["areaM2"] = round(regions[rid]["areaM2"] * lateral(asset, ids) / old_lateral[rid], 2)
    regions["stem-bark"].update({"title": {"en": "Smooth silver-grey bark of the stems", "de": "Glatte silbergraue Rinde der Stämme"}, "characteristicLengthM": 0.2, "heatCapacityJM2K": 35000})
    regions["limb-bark"].update({"title": {"en": "Bark of the limbs and shoots", "de": "Rinde der Äste und Triebe"}, "characteristicLengthM": 0.07, "heatCapacityJM2K": 18000})
    regions["leaf-sun"].update({"title": {"en": "Sun leaves of the outer crown", "de": "Sonnenblätter der Außenkrone"}, "areaM2": 72, "characteristicLengthM": 0.12, "geometry": "horizontal-plate-up", "heatCapacityJM2K": 1000, "leaf": {"stomatalConductanceMaxMS": 0.006, "leafAreaIndex": 1.9, "photosynthesis": "c3"}, "reference": "flaishman-2008"})
    regions["leaf-shade"].update({"title": {"en": "Shade leaves of the inner crown", "de": "Schattenblätter der Innenkrone"}, "areaM2": 48, "characteristicLengthM": 0.14, "heatCapacityJM2K": 800, "leaf": {"stomatalConductanceMaxMS": 0.0035, "leafAreaIndex": 1.3, "photosynthesis": "c3"}, "reference": "flaishman-2008"})
    t["assumptions"][0] = {
        "en": "Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection (about 38 m2 for the 7 m crown, 120 m2 of leaves). Fig leaves are large (10 to 25 cm), three- to five-lobed, thick and rough; the maximum stomatal conductance (0.006 m/s sun, 0.0035 m/s shade) is an authored estimate in the upper broadleaf range of a well-watered Mediterranean fruit tree, guided by Jones (2014) and Flaishman et al. (2008). Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.",
        "de": "Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden (etwa 38 m2 für die 7 m breite Krone, 120 m2 Blätter). Feigenblätter sind groß (10 bis 25 cm), drei- bis fünflappig, dick und rau; die maximale Spaltöffnungsleitfähigkeit (0,006 m/s Sonne, 0,0035 m/s Schatten) ist eine Autorenschätzung im oberen Laubholzbereich eines gut versorgten mediterranen Obstbaums, angelehnt an Jones (2014) und Flaishman et al. (2008). Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben.",
    }
    t["assumptions"][3] = {
        "en": "Leaf shedding (the fig is bare from November to April) and the ripening of the figs are not geometric: a pose cannot hide shapes, so the seasonal pose changes the thermal binding (sun, stomata) and the loads only; the figs are a load, not a drawn region.",
        "de": "Laubfall (die Feige ist von November bis April kahl) und das Reifen der Feigen sind nicht geometrisch: Eine Pose kann keine Formen ausblenden, die jahreszeitliche Pose ändert daher nur die thermische Bindung (Sonne, Spaltöffnungen) und die Lasten; die Feigen sind eine Last, keine gezeichnete Region.",
    }


def poses(asset):
    asset["poses"] = [p for p in asset["poses"] if p["id"] != "autumn-snow"]
    by = {p["id"]: p for p in asset["poses"]}
    calm = by["calm"]
    fruit = by["fruit-laden"]
    fruit["thermal"] = {"regions": {k: dict(v) for k, v in calm["thermal"]["regions"].items()}, "emitters": {k: round(v * 0.85, 3) for k, v in calm["thermal"]["emitters"].items()}}
    masses = {b["id"]: b["mass"]["kg"] for b in asset["rig"]["bones"] if b["id"] in FOLIAGE_BONES}
    outer = [b for b in FOLIAGE_BONES if not b.startswith("fol-core")]
    total = sum(masses[b] for b in outer)
    for b in outer:
        key = b.replace("fol-", "")
        en, de = LOAD_TITLES[key]
        wind = next(ld for ld in fruit["loads"] if ld["id"] == f"wind-{b}")
        fruit["loads"].append({"id": f"fruit-{b}", "kind": "carried", "title": {"en": f"Figs on the {en}", "de": f"Feigen an der {de}"}, "bone": b, "x": wind["x"], "y": wind["y"], "fxN": 0, "fyN": round(-FRUIT_KG * 9.80665 * masses[b] / total, 1), "source": "authored-estimate", "reference": "fig-yield"})
    scale_loads(asset, "wind", geometry.A[0] * geometry.A[1] * geometry.FOLIAGE ** 2 * 1.15)
    for p in asset["poses"]:
        for ld in p["loads"]:
            if ld["id"] == "wind-trunk":
                ld["distributed"]["wxNPerM"] = round(ld["distributed"]["wxNPerM"] * 1.3 * geometry.A[2] / (geometry.A[0] * geometry.A[1] * geometry.FOLIAGE ** 2 * 1.15), 3)
            if ld["kind"] == "wind":
                key = ld["id"].replace("wind-fol-", "").replace("wind-", "")
                if key in LOAD_TITLES:
                    ld["title"] = {"en": f"Drag on the {LOAD_TITLES[key][0]}", "de": f"Widerstand an der {LOAD_TITLES[key][1]}" if not key.startswith("core") else f"Widerstand am {LOAD_TITLES[key][1]}"}
    for p in asset["poses"]:
        if p["id"] == "fruit-laden":
            for b, off in p["bones"].items():
                if b.startswith("t") or b.startswith("fol-"):
                    off["angleDeg"] = round(off.get("angleDeg", 0) - (0.6 if "w" in b.split("-")[-1] else -0.6), 3)


def provenance(asset):
    pv = asset["provenance"]
    pv["author"] = "Semio staffage vegetation content worker PW-09"
    pv["method"] = "handcrafted"
    pv["created"] = "2026-11-22"
    refs = [x for x in pv["references"] if x["id"] not in ("pretzsch-2015", "en-1991-1-3")]
    refs += [
        {"id": "flaishman-2008", "citation": "Flaishman, M. A., Rodov, V. and Stover, E. (2008). The fig: botany, horticulture, and breeding. Horticultural Reviews 34, 113-196. Habit, leaves, crops and growth of Ficus carica; used as a guide, values here are authored estimates."},
        {"id": "condit-1947", "citation": "Condit, I. J. (1947). The Fig. Chronica Botanica, Waltham, Massachusetts. Tree form, multi-stemmed habit and leaf size of the common fig."},
        {"id": "fig-yield", "citation": "General Mediterranean practice: a mature garden fig tree of 4 to 6 m bears roughly 10 to 30 kg of figs (40 to 60 g each) in its main late-summer crop; the value used here is an authored estimate, not a cited measurement."},
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
    by = {b["id"]: b for b in asset["rig"]["bones"]}
    ds = {k: math.sqrt(4 * by[k]["section"]["areaM2"] / math.pi) for k in ("stem-c", "stem-w", "stem-e")}
    H, W, D = d["heightM"], d["widthM"], d["depthM"]
    replace_words(asset, [("layered", "rounded"), ("tier limbs", "main limbs"), ("tiers", "levels"), ("tier", "level"), ("maple", "fig"), ("Maple", "Fig")],
                  [("geschichtet", "gerundet"), ("Etagenäste", "Hauptäste"), ("Etagen", "Ebenen"), ("Etage", "Ebene"), ("Ahorn", "Feigen")])
    stems = f"{ds['stem-c']:.2f}, {ds['stem-w']:.2f} and {ds['stem-e']:.2f} m"
    stems_de = f"{ds['stem-c']:.2f}, {ds['stem-w']:.2f} und {ds['stem-e']:.2f} m".replace(".", ",")
    T = {
        ".title": ("Fig Tree, Multi-Stem", "Feigenbaum, mehrstämmig"),
        ".summary": (
            f"A mature common fig (Ficus carica) of {n_en(H)} m, three smooth silver-grey stems from one low collar and a broad, dense, rounded crown {n_en(W)} m across of large, lobed leaves; the late-summer crop of figs is carried as a pose and load, in a hot, dry summer climate.",
            f"Ein ausgewachsener Echter Feigenbaum (Ficus carica) von {n_de(H)} m, drei glatte silbergraue Stämme aus einem niedrigen Wurzelhals und eine breite, dichte, runde Krone von {n_de(W)} m Durchmesser aus großen, gelappten Blättern; der spätsommerliche Feigenbehang wird als Pose und Last geführt, in heißem, trockenem Sommerklima.",
        ),
        ".provenance.notes": (
            f"Rig and drawings are derived from the handcrafted multi-stem Japanese maple of this archive by an exact per-bone and world-diagonal morph (leaf masses enlarged 1.22 times about their bones, the whole tree 1.12 and 1.1 times wider and 0.98 times as high, every leaf mass thickened {geometry.DOME} times about its own height into a dome) and re-authored for a garden fig of about 30 years: three stems of {stems} from one collar, a crown {n_en(W)} m wide and {n_en(H)} m high. Figs of this kind commonly reach 3 to 6 m with crowns wider than high. The {FRUIT_KG} kg of figs of the fruit pose are an authored estimate within the 10 to 30 kg of a main crop, not a measured yield.",
            f"Rig und Zeichnungen sind aus dem handgezeichneten mehrstämmigen Fächerahorn dieses Archivs durch eine exakte Abbildung je Knochen und eine weltdiagonale Abbildung abgeleitet (Laubmassen um ihre Knochen 1,22-fach vergrößert, der ganze Baum 1,12- und 1,1-fach breiter und 0,98-fach so hoch, jede Laubmasse um ihre eigene Höhe {n_de(geometry.DOME)}-fach zu einer Kuppel verdickt) und für einen etwa 30-jährigen Gartenfeigenbaum neu bearbeitet: drei Stämme von {stems_de} aus einem Wurzelhals, eine Krone von {n_de(W)} m Breite und {n_de(H)} m Höhe. Feigenbäume dieser Art erreichen meist 3 bis 6 m mit Kronen, die breiter als hoch sind. Die {FRUIT_KG} kg Feigen der Fruchtpose sind eine Autorenschätzung innerhalb der 10 bis 30 kg einer Haupternte, kein gemessener Ertrag.",
        ),
        ".symbols[fig-tree-symbol].description": (
            "A conventional plan sign for garden and courtyard plans: the true plan outline of the crown, the three stem sections at breast height and the directions of the six main limbs. It is a separate symbol and is never used silently for a pictorial scale.",
            "Ein konventionelles Grundrisszeichen für Garten- und Hofpläne: der wahre Grundrissumriss der Krone, die drei Stammquerschnitte in Brusthöhe und die Richtungen der sechs Hauptäste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.",
        ),
        ".symbols[fig-tree-symbol].dimensionalNote": (
            f"The crown outline and the three stems are drawn at their true extent of {W:.2f} × {D:.2f} × {H:.2f} m in width, depth and height.",
            f"Der Kronenumriss und die drei Stämme sind in ihrer wahren Ausdehnung von {W:.2f} × {D:.2f} × {H:.2f} m".replace(".", ",") + " in Breite, Tiefe und Höhe gezeichnet.",
        ),
        ".symbols[tree-sign].dimensionalNote": (
            "A conventional symbol in paper millimetres, not to scale: a circle of fixed paper size with a dot for the stem group, drawn as plan circle and as side and front sign. Its size carries no dimension.",
            "Ein konventionelles Symbol in Papiermillimetern, nicht maßstäblich: ein Kreis fester Papiergröße mit einem Punkt für die Stammgruppe, gezeichnet als Grundrisskreis und als Seiten- und Vorderansichtszeichen. Seine Größe trägt keine Abmessung.",
        ),
        ".poses[calm].title": ("Calm hot summer day", "Windstiller heißer Sommertag"),
        ".poses[calm].description": (
            "Light air of 1.5 m/s at 10 m on a hot, dry summer day: the dense dome stands at rest, the outer leaves are lit and shade the inner ones; transpiration at 80 % of the daytime maximum.",
            "Leiser Zug von 1,5 m/s in 10 m Höhe an einem heißen, trockenen Sommertag: die dichte Kuppel steht still, die äußeren Blätter sind besonnt und beschatten die inneren; Transpiration mit 80 % des Tagesmaximums.",
        ),
        ".poses[windy].title": ("Summer breeze", "Sommerbrise"),
        ".poses[windy].description": (
            "Moderate breeze of 5.5 m/s at 10 m (Beaufort 4) in an open courtyard: the limbs lean downwind and the large leaves flutter; drag by the reconfigured law with cd 0.35 on the dense crown.",
            "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4) in einem offenen Hof: die Äste neigen sich in Windrichtung, die großen Blätter flattern; Widerstand nach dem Umformungsgesetz mit cd 0,35 an der dichten Krone.",
        ),
        ".poses[fruit-laden].title": ("Late summer, ripe figs", "Spätsommer, reife Feigen"),
        ".poses[fruit-laden].description": (
            f"Calm, hot late-summer day with the main crop of about {FRUIT_KG} kg of ripe figs (some 300 of 50 g) on the young shoots of the outer crown: the limbs sag slightly; transpiration at 70 % in the drier soil of the season.",
            f"Windstiller, heißer Spätsommertag mit der Haupternte von etwa {FRUIT_KG} kg reifer Feigen (rund 300 zu 50 g) an den jungen Trieben der Außenkrone: die Äste hängen leicht durch; Transpiration mit 70 % im trockeneren Boden der Jahreszeit.",
        ),
        ".poses[storm].title": ("Storm with rain", "Sturm mit Regen"),
        ".poses[storm].description": (
            "Beaufort 9, 22 m/s at 10 m with rain in an exposed garden: strong downwind lean, drag with the reconfiguration reduction (cd 0.35, (v/8)^-0.6 above 8 m/s) on the large leaves, wet bark and leaves, overcast sky; the brittle fig wood is the weak point.",
            "Beaufort 9, 22 m/s in 10 m Höhe mit Regen in einem offenen Garten: starke Neigung in Windrichtung, Widerstand mit Umformungsminderung (cd 0,35, (v/8)^-0,6 über 8 m/s) an den großen Blättern, nasse Rinde und Blätter, bedeckter Himmel; das spröde Feigenholz ist die Schwachstelle.",
        ),
        ".clips[wind-sway].title": ("Steady wind sway", "Gleichmäßiges Schwanken im Wind"),
        ".clips[wind-sway].description": (
            f"A deterministic 1.8 s sway cycle about the breeze lean (5.5 m/s at 10 m; estimated first mode of the {n_en(H)} m multi-stem tree about 0.55 Hz). The limbs bob with a phase lag behind the stems; the leaf masses swing about 3 degrees.",
            f"Ein deterministischer Schwingungszyklus von 1,8 s um die Neigung bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des {n_de(H)} m hohen mehrstämmigen Baums etwa 0,55 Hz). Die Äste wippen phasenverzögert hinter den Stämmen; die Laubmassen schwingen etwa 3 Grad.",
        ),
        ".clips[gust].description": (
            "A single gust over 8 s from 5.5 to 14.0 m/s at 10 m: the wind-load tracks rise with the reconfigured drag law and the limbs lean downwind with a level-wise lag, the stems last.",
            "Eine einzelne Böe über 8 s von 5,5 auf 14,0 m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Äste neigen sich ebenenweise verzögert in Windrichtung, die Stämme zuletzt.",
        ),
        ".clips[diurnal-transpiration].description": (
            "A 24 h cycle of a hot summer day compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, with stomatal closure at night and a mid-afternoon depression; leaf and bark temperatures are not authored but follow from the thermal solver.",
            "Ein 24-h-Zyklus eines heißen Sommertags auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei Windstille. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata geschlossen, am frühen Nachmittag gibt es eine Mittagsdepression; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser.",
        ),
        ".representations.50.decisions.silhouette": (
            "At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: the flared collar with its surface roots, the three smooth, sinuous stems, the limbs that fan out under the crown and seventy-three dome-shaped leaf masses with lobed rims, each drawn as the outline of its own solid and stacked by depth.",
            "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der ausgestellte Wurzelhals mit seinen Oberflächenwurzeln, die drei glatten, geschwungenen Stämme, die Äste, die sich unter der Krone auffächern, und dreiundsiebzig kuppelförmige Laubmassen mit gelappten Rändern, jede als Umriss ihres eigenen Körpers gezeichnet und nach Tiefe gestaffelt.",
        ),
        ".representations.50.decisions.retained": (
            "Retained: every stem, the central top, every main limb as a filled outline under its foliage (one outline per limb bone with its secondary limbs), the shoots as texture strokes, every leaf mass of the three views and the visible leaf marks of the large lobed leaves.",
            "Beibehalten: jeder Stamm, die mittlere Spitze, jeder Hauptast als gefüllter Umriss unter seinem Laub (ein Umriss je Astknochen mit seinen Nebenästen), die Triebe als Texturstriche, jede Laubmasse der drei Ansichten und die sichtbaren Blattzeichen der großen gelappten Blätter.",
        ),
        ".representations.50.decisions.merged": (
            "Front and plan draw the crown by sectors (four or eight per level, as many as the drawing names), each the union of the leaf clusters of that sector; the elevation draws one outline per foliage bone.",
            "Vorderansicht und Grundriss zeichnen die Krone nach Sektoren (vier oder acht je Ebene, so viele, wie die Zeichnung nennt), jeder die Vereinigung der Blattbüschel dieses Sektors; die Seitenansicht zeichnet einen Umriss je Laubknochen.",
        ),
        ".representations.50.decisions.omitted": (
            "Omitted: the single leaves (10 to 25 cm, 2 to 5 mm on paper, typified by the leaf marks), the figs (a load, not a drawn region), buds and the latex; arcs and leaf marks on clusters hidden behind nearer ones.",
            "Weggelassen: die einzelnen Blätter (10 bis 25 cm, 2 bis 5 mm auf dem Papier, durch die Blattzeichen typisiert), die Feigen (eine Last, keine gezeichnete Region), Knospen und der Milchsaft; Bögen und Blattzeichen auf Büscheln, die hinter näheren liegen.",
        ),
        ".representations.50.decisions.thinElements": (
            "Limbs of 1.5 to 9 cm are 0.3 to 1.8 mm wide and drawn as filled outlines at their real width; shoots (2 cm, 0.4 mm) are 0.13 mm texture strokes; arcs and leaf marks are 0.13 to 0.18 mm strokes, all above the pen minimum.",
            "Äste von 1,5 bis 9 cm sind 0,3 bis 1,8 mm breit und als gefüllte Umrisse in realer Breite gezeichnet; Triebe (2 cm, 0,4 mm) sind 0,13-mm-Texturstriche; Bögen und Blattzeichen sind 0,13- bis 0,18-mm-Striche, alle über dem Stiftminimum.",
        ),
        ".representations.50.decisions.overlays": (
            "Each thermal region has its own shapes: stem bark on collar and stems, limb bark on the limb outlines, sun and shade leaves on their leaf masses; arcs, leaf marks and shoots are strokes without a region and follow their bone.",
            "Jede thermische Region hat eigene Formen: Stammrinde an Wurzelhals und Stämmen, Astrinde an den Astumrissen, Sonnen- und Schattenblätter an ihren Laubmassen; Bögen, Blattzeichen und Triebe sind Striche ohne Region und folgen ihrem Knochen.",
        ),
        ".representations.100.decisions.silhouette": (
            "At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: collar, stems, limbs and fifty-seven dome-shaped leaf masses with rims scalloped at the size of a leaf cluster.",
            "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: Wurzelhals, Stämme, Äste und siebenundfünfzig kuppelförmige Laubmassen mit Rändern, die in der Größe eines Blattbüschels gewellt sind.",
        ),
        ".representations.100.decisions.retained": (
            "Retained: three stems with collar and central top, the six main limbs with their secondary limbs as filled outlines, every leaf mass of the three views, one bark line.",
            "Beibehalten: drei Stämme mit Wurzelhals und mittlerer Spitze, die sechs Hauptäste mit ihren Nebenästen als gefüllte Umrisse, jede Laubmasse der drei Ansichten, eine Rindenlinie.",
        ),
        ".representations.100.decisions.merged": (
            "The hanging fringe and the small clusters of 1:50 are merged into broader masses; the diagonal sectors of the lowest level are merged into the neighbouring sectors in front and plan.",
            "Der hängende Saum und die kleinen Büschel von 1:50 sind zu breiteren Massen zusammengefasst; die diagonalen Sektoren der untersten Ebene gehen in Vorderansicht und Grundriss in den benachbarten Sektoren auf.",
        ),
        ".representations.100.decisions.omitted": (
            "Omitted: single leaves, shoots and the leaf marks of 1:50; arcs on hidden clusters; the figs.",
            "Weggelassen: einzelne Blätter, Triebe und die Blattzeichen von 1:50; Bögen auf verdeckten Büscheln; die Feigen.",
        ),
        ".representations.100.decisions.thinElements": (
            "Limbs of 2 to 9 cm are 0.2 to 0.9 mm wide and drawn as filled outlines; the stems keep their real width.",
            "Äste von 2 bis 9 cm sind 0,2 bis 0,9 mm breit und als gefüllte Umrisse gezeichnet; die Stämme behalten ihre reale Breite.",
        ),
        ".representations.200.decisions.silhouette": (
            "At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: collar, stems, main limbs and forty dome-shaped leaf masses on three levels and a top.",
            "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: Wurzelhals, Stämme, Hauptäste und vierzig kuppelförmige Laubmassen auf drei Ebenen und einer Spitze.",
        ),
        ".representations.200.decisions.retained": (
            "Retained: the three stems, collar and central top as filled tapered outlines, the six main limbs as filled outlines under the foliage, every leaf mass.",
            "Beibehalten: die drei Stämme, Wurzelhals und mittlere Spitze als gefüllte verjüngte Umrisse, die sechs Hauptäste als gefüllte Umrisse unter dem Laub, jede Laubmasse.",
        ),
        ".representations.200.decisions.merged": (
            "Merged into the crown outlines: the secondary limbs, shoots and single clusters; the crown is drawn by four to six sectors in front and two to four in plan.",
            "In den Kronenumrissen zusammengefasst: die Nebenäste, Triebe und einzelnen Büschel; die Krone ist in der Vorderansicht nach vier bis sechs und im Grundriss nach zwei bis vier Sektoren gezeichnet.",
        ),
        ".representations.200.decisions.thinElements": (
            "The main limbs (3 to 9 cm, 0.15 to 0.45 mm) are drawn as filled outlines under the foliage; the stems keep their real width (0.6 to 1.2 mm).",
            "Die Hauptäste (3 bis 9 cm, 0,15 bis 0,45 mm) sind als gefüllte Umrisse unter dem Laub gezeichnet; die Stämme behalten ihre reale Breite (0,6 bis 1,2 mm).",
        ),
        ".representations.500.decisions.silhouette": (
            "At 1:500 (1 m is 2 mm) the drawings are the true views of the 1:500 body: the collar, the three splayed stems and eighteen deep masses that close into the broad, rounded dome of the fig.",
            "Bei 1:500 (1 m sind 2 mm) sind die Zeichnungen die wahren Ansichten des 1:500-Körpers: der Wurzelhals, die drei gespreizten Stämme und achtzehn tiefe Massen, die sich zur breiten, runden Kuppel der Feige schließen.",
        ),
        ".representations.500.decisions.retained": (
            "Retained: the scalloped dome outline and the three stems with collar and central top as one shape.",
            "Beibehalten: der gewellte Kuppelumriss und die drei Stämme mit Wurzelhals und mittlerer Spitze als eine Form.",
        ),
        ".representations.500.decisions.merged": (
            "Merged into the crown path: all sun and shade foliage and all main limbs; collar, stems and central top are one stem shape.",
            "Im Kronenpfad zusammengefasst: alles Sonnen- und Schattenlaub und alle Hauptäste; Wurzelhals, Stämme und mittlere Spitze sind eine Stammform.",
        ),
        ".representations.500.decisions.thinElements": (
            "Main limbs of 3 to 9 cm (0.06 to 0.18 mm) are not drawn; they are merged into the filled crown. The stems are 0.3 to 0.5 mm wide and drawn at that width.",
            "Hauptäste von 3 bis 9 cm (0,06 bis 0,18 mm) werden nicht gezeichnet; sie gehen in der gefüllten Krone auf. Die Stämme sind 0,3 bis 0,5 mm breit und in dieser Breite gezeichnet.",
        ),
        ".volumes.50.decisions[0]": (
            f"Body at 1:50: a flared root collar with four shallow surface roots; three smooth, slightly sinuous stems of {stems} that splay from it in elevation and in plan (west stem towards north-west, east stem towards south-east, centre stem nearly upright); the central top continues the centre stem; six main limbs with secondary limbs carry nine leaf masses that close into a dense dome. Derived from the multi-stem maple of this archive (stems thickened, leaf masses enlarged and thickened into domes).",
            f"Körper bei 1:50: ein ausgestellter Wurzelhals mit vier flachen Oberflächenwurzeln; drei glatte, leicht geschwungene Stämme von {stems_de}, die sich in Ansicht und Grundriss aus ihm spreizen (Weststamm nach Nordwesten, Oststamm nach Südosten, Mittelstamm fast aufrecht); die mittlere Spitze setzt den Mittelstamm fort; sechs Hauptäste mit Nebenästen tragen neun Laubmassen, die sich zu einer dichten Kuppel schließen. Abgeleitet aus dem mehrstämmigen Ahorn dieses Archivs (Stämme verdickt, Laubmassen vergrößert und zu Kuppeln verdickt).",
        ),
        ".volumes.50.decisions[1]": (
            "Thermal at 1:50: stem bark is the collar, the surface roots, the three stems and the central top; limb bark the six main and twelve secondary limbs; sun leaves are the outer clusters and the top, shade leaves the ten inner clusters, all by reference to the body. The shoots are counted with the leaf clusters they carry (exchange area).",
            "Thermik bei 1:50: Stammrinde sind Wurzelhals, Oberflächenwurzeln, die drei Stämme und die mittlere Spitze; Astrinde die sechs Haupt- und zwölf Nebenäste; Sonnenblätter sind die äußeren Büschel und die Spitze, Schattenblätter die zehn inneren Büschel, alle als Verweis auf den Körper. Die Triebe zählen zu den Blattbüscheln, die sie tragen (Austauschfläche).",
        ),
        ".volumes.50.decisions[3]": (
            "The foliage follows nine branch groups. Each leaf mass has three asymmetric, drooping lobes and a distinct lower surface; the lobed leaf blades at their edges follow the same group and distinguish the sunlit and shaded surfaces.",
            "Das Laub folgt neun Astgruppen. Jede Laubmasse hat drei asymmetrische, hängende Lappen und eine ausgeprägte Unterseite; die gelappten Blattspreiten an ihren Rändern folgen derselben Gruppe und unterscheiden besonnte und beschattete Flächen.",
        ),
        ".volumes.100.decisions[0]": (
            "The flared root collar carries the curved stems and the secondary limbs retain two stations. Fifty-three broad, dome-shaped leaf masses describe the crown; hanging cluster tips mark its lower, middle and upper levels.",
            "Der ausgestellte Wurzelhals trägt die geschwungenen Stämme, die Nebenäste behalten zwei Stationen. Dreiundfünfzig breite, kuppelförmige Laubmassen beschreiben die Krone; hängende Büschelspitzen markieren ihre untere, mittlere und obere Ebene.",
        ),
        ".volumes.200.decisions[0]": (
            "Body at 1:200: collar, stems and central top with three or four sections, the six main limbs with three; the secondary limbs (below one smallest feature at the tip) are merged into the crown. The crown is thirty-four broad, dome-shaped clusters on three levels with inner clusters and a top.",
            "Körper bei 1:200: Wurzelhals, Stämme und mittlere Spitze mit drei oder vier Schnitten, die sechs Hauptäste mit drei; die Nebenäste (an der Spitze unter einem kleinsten Merkmal) gehen in der Krone auf. Die Krone besteht aus vierunddreißig breiten, kuppelförmigen Büscheln auf drei Ebenen mit inneren Büscheln und einer Spitze.",
        ),
        ".volumes.500.decisions[0]": (
            "Body at 1:500: collar, three stems and central top as plain lofts; the crown is twenty-six masses bound to the centre stem as the drawing binds it, thickened into the closed dome of the fig; the main limbs are thickened to the smallest feature (75 mm).",
            "Körper bei 1:500: Wurzelhals, drei Stämme und mittlere Spitze als einfache Lofts; die Krone besteht aus sechsundzwanzig Massen, an den Mittelstamm gebunden wie in der Zeichnung, zur geschlossenen Kuppel der Feige verdickt; die Hauptäste sind auf das kleinste Merkmal (75 mm) verdickt.",
        ),
        ".volumes.500.decisions[3]": (
            "The leaf masses follow the eastern and western fans of each stem.",
            "Die Laubmassen folgen den östlichen und westlichen Fächern jedes Stamms.",
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
    a["tags"] = ["tree", "deciduous", "small-tree", "multi-stem", "mediterranean", "fruit-tree", "fruit-load", "wind", "transpiration"]
    materials(a)
    retone(a, tone)
    thermal(a, old_lateral)
    poses(a)
    provenance(a)
    bone_titles(a)
    textual(a)
    return doc


DISTINCTION = (
    "A multi-stemmed common fig with smooth silver-grey stems and a broad, dense, rounded crown of large lobed leaves; the late-summer crop of figs is carried as a pose and load.",
    "Ein mehrstämmiger Echter Feigenbaum mit glatten silbergrauen Stämmen und einer breiten, dichten, runden Krone aus großen gelappten Blättern; der spätsommerliche Feigenbehang wird als Pose und Last geführt.",
)
