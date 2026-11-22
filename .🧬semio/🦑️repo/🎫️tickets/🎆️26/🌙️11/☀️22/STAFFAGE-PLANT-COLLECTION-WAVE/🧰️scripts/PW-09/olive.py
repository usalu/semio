"""🫒 Olive tree (Olea europaea), old gnarled orchard tree of about 7 m (PW-09): species re-authoring of the morphed `apple-tree`.

Species data and sources: `../../📝️PW-09.md`. Geometry: `olive_geometry.py`.
"""
import math

import olive_geometry as geometry
from morph import each_shape, map_path, r
from species import count, count_solids, de_words, en_words, rename_ids, replace_words, retone, scale_loads, scale_pose_angles, section_titles, set_texts

ID = "olive-tree"
TEMPLATE = geometry.TEMPLATE
WORDS = [r"[Aa]pple", r"Apfel", r"Äpf", r"Obstwiese", r"June drop", r"Junifall", r"orchard(?! standard of this archive)", r"(?<!dem )Hochstamm(?! des Archivs)(?!-Obstbaum dieses)"]
IDS = {
    "apple-tree": ID, "apple-tree-symbol": "olive-tree-symbol",
    "apple-bark": "olive-bark", "apple-bark-young": "olive-bark-young", "apple-twig": "olive-twig",
    "apple-leaf-sun": "olive-leaf-sun", "apple-leaf-shade": "olive-leaf-shade", "apple-fruit": "olive-fruit", "apple-wood": "olive-wood",
    "apple-yield": "olive-yield",
    "summer-fruit-set": "calm", "breeze-with-fruit": "windy", "harvest-laden": "fruit-laden", "gale-windfall": "storm",
    "breeze-sway": "wind-sway",
}
ENVIRONMENT = {"airTempC": 32, "meanRadiantTempC": 36, "skyTempC": 21, "groundTempC": 50, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 850, "diffuseFraction": 0.12, "pressurePa": 101325}
FRUIT_KG = {"calm": 10, "windy": 25, "fruit-laden": 40, "storm": 30}
APPLE_KG = {"calm": 25, "windy": 90, "fruit-laden": 150, "storm": 120}
REGION_BONES = {"trunk-bark": ["root-plate", "trunk", "ld"], "limb-bark": ["ld-b", "sc-w1a", "sc-w1b", "sc-e1a", "sc-e1b", "sc-w2a", "sc-w2b", "sc-e2a", "sc-e2b"], "spur-twig": ["spur-w1", "spur-e1", "spur-w2", "spur-e2"]}


def lateral(asset, ids):
    by = {b["id"]: b for b in asset["rig"]["bones"]}
    return sum(math.pi * math.sqrt(4 * by[i]["section"]["areaM2"] / math.pi) * by[i]["lengthM"] for i in ids)


def materials(asset):
    m = {x["id"]: x for x in asset["materials"]}
    m["olive-bark"].update({"title": {"en": "Olive bark, grey, deeply fissured and twisted", "de": "Olivenrinde, grau, tief rissig und gedreht"}, "solarAbsorptivity": 0.6, "densityKgM3": 750})
    m["olive-bark-young"].update({"title": {"en": "Smooth grey bark of the scaffold limbs", "de": "Glatte graue Rinde der Leitäste"}, "solarAbsorptivity": 0.55, "densityKgM3": 800})
    m["olive-twig"].update({"title": {"en": "Grey-green bark of the fruiting shoots", "de": "Graugrüne Rinde der Fruchttriebe"}, "solarAbsorptivity": 0.55})
    m["olive-leaf-sun"].update({"title": {"en": "Olive sun leaves (sclerophyllous, silvery underside)", "de": "Oliven-Sonnenblätter (hartlaubig, silbrige Unterseite)"}, "solarAbsorptivity": 0.42, "source": "authored-estimate", "reference": "connor-fereres-2005"})
    m["olive-leaf-shade"].update({"title": {"en": "Olive shade leaves (effective)", "de": "Oliven-Schattenblätter (wirksam)"}, "solarAbsorptivity": 0.22})
    m["olive-fruit"].update({"title": {"en": "Olive drupe skin (waxy, oil-rich fruit)", "de": "Haut der Olivensteinfrucht (wachsig, ölreich)"}, "solarAbsorptivity": 0.75, "densityKgM3": 1050, "conductivityWmK": 0.35, "specificHeatJKgK": 3000})
    m["olive-wood"].update({"title": {"en": "Green olive wood", "de": "Frisches Olivenholz"}, "densityKgM3": 1100, "conductivityWmK": 0.2, "elasticModulusPa": 9000000000, "strengthPa": 60000000})


def tone(item, t):
    mat = str(item.get("material", "")) + str(item.get("region", ""))
    if "fruit" in mat:
        return 0.2
    if "leaf" in mat or (geometry.is_crown(item) and item.get("role") == "body" and t >= 0.4):
        return min(0.86, t + 0.13)
    if "bark" in mat or "wood" in mat:
        return min(0.6, t + 0.08)
    return t


def ridges(asset):
    """🌀 The spiral ridges of an old olive stem replace the bark scales of the template at 1:50 and 1:100 (texture strokes inside the stem)."""
    by = {b["id"]: b for b in asset["rig"]["bones"]}
    L = by["trunk"]["lengthM"]
    R = math.sqrt(by["trunk"]["section"]["areaM2"] / math.pi)
    for key, n in (("50", 6), ("100", 3)):
        g = asset["representations"][key]["geometry"]
        for view in ("elevation", "front"):
            keep = [s for s in g[view] if not (s["bone"] == "trunk" and s["role"] == "texture")]
            strokes = []
            for i in range(n):
                u0 = L * (0.08 + 0.84 * i / n)
                du = L * 0.55 / n
                if view == "elevation":
                    pts = [(u0 + du * t / 6, R * 0.72 * math.cos(math.pi * (0.15 + 0.7 * t / 6)) * (-1 if i % 2 else 1)) for t in range(7)]
                    d = "M " + " L ".join(f"{r(x, 4):g} {r(y, 4):g}" for x, y in pts)
                    strokes.append({"id": f"bark-ridge-{i + 1}", "bone": "trunk", "role": "texture", "z": 1.2, "filled": False, "tone": 0.15, "kind": "path", "d": d})
                else:
                    y0 = by["root-plate"]["lengthM"] + u0
                    pts = [[r(R * 0.72 * math.cos(math.pi * (0.15 + 0.7 * t / 6)) * (-1 if i % 2 else 1), 4), r(y0 + du * t / 6, 4)] for t in range(7)]
                    strokes.append({"id": f"bark-ridge-{i + 1}", "bone": "trunk", "role": "texture", "z": 1.2, "filled": False, "tone": 0.15, "kind": "poly", "points": pts, "closed": False})
            body = [i for i, s in enumerate(keep) if s["bone"] == "trunk" and s["role"] == "body"]
            at = (body[-1] + 1) if body else len(keep)
            g[view] = keep[:at] + strokes + keep[at:]


def thermal(asset, old_lateral):
    t = asset["thermal"]
    t["environment"] = dict(ENVIRONMENT)
    t["referenceEnvironment"] = "summer-hot-sun"
    regions = {x["id"]: x for x in t["regions"]}
    for rid, ids in REGION_BONES.items():
        regions[rid]["areaM2"] = round(regions[rid]["areaM2"] * lateral(asset, ids) / old_lateral[rid], 2)
    regions["trunk-bark"].update({"title": {"en": "Grey, fissured and twisted bark of the stem and the fork", "de": "Graue, rissige und gedrehte Rinde von Stamm und Gabelung"}, "characteristicLengthM": 0.7, "heatCapacityJM2K": 80000})
    regions["limb-bark"].update({"title": {"en": "Bark of the scaffold limbs and the upper fork", "de": "Rinde der Leitäste und der oberen Stammgabel"}, "characteristicLengthM": 0.2, "heatCapacityJM2K": 50000})
    regions["spur-twig"].update({"title": {"en": "Fruiting shoots and twigs", "de": "Fruchttriebe und Zweige"}})
    regions["leaf-sun"].update({"title": {"en": "Sun leaves of the outer crown", "de": "Sonnenblätter der Außenkrone"}, "areaM2": 96, "characteristicLengthM": 0.012, "heatCapacityJM2K": 1400, "leaf": {"stomatalConductanceMaxMS": 0.004, "leafAreaIndex": 2.0, "photosynthesis": "c3"}, "reference": "connor-fereres-2005"})
    regions["leaf-shade"].update({"title": {"en": "Shade leaves of the inner crown", "de": "Schattenblätter der Innenkrone"}, "areaM2": 64, "characteristicLengthM": 0.012, "heatCapacityJM2K": 1100, "leaf": {"stomatalConductanceMaxMS": 0.0025, "leafAreaIndex": 1.3, "photosynthesis": "c3"}, "reference": "connor-fereres-2005"})
    regions["fruit"].update({"title": {"en": "Skin of the olives", "de": "Haut der Oliven"}, "areaM2": 9, "characteristicLengthM": 0.02, "heatCapacityJM2K": 10000, "reference": "olive-yield"})
    t["assumptions"][0] = {
        "en": "Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection (about 50 m2 for the 8 m crown, 160 m2 of leaves). Olive leaves are small (1 to 1.5 cm wide), thick and sclerophyllous with a scaly, silvery underside; the maximum stomatal conductance (0.004 m/s sun, 0.0025 m/s shade) is an authored estimate at the low end of the broadleaf range, guided by Connor and Fereres (2005) and Jones (2014), and the stomata close early under drought. Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.",
        "de": "Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden (etwa 50 m2 für die 8 m breite Krone, 160 m2 Blätter). Olivenblätter sind klein (1 bis 1,5 cm breit), dick und hartlaubig mit schuppiger, silbriger Unterseite; die maximale Spaltöffnungsleitfähigkeit (0,004 m/s Sonne, 0,0025 m/s Schatten) ist eine Autorenschätzung am unteren Ende des Laubholzbereichs, angelehnt an Connor und Fereres (2005) und Jones (2014), und die Spaltöffnungen schließen bei Trockenheit früh. Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben.",
    }
    t["assumptions"][4] = {
        "en": "The olive skin is a passive surface of the whole crop (about 40 kg, some 10 000 drupes of 2 to 3 cm, 9 m2); its areal heat capacity is that of a water- and oil-rich ellipsoid (1050 kg/m3, 3000 J/(kg K), radius/3), an authored estimate.",
        "de": "Die Olivenhaut ist eine passive Oberfläche der ganzen Ernte (etwa 40 kg, rund 10 000 Steinfrüchte von 2 bis 3 cm, 9 m2); ihre flächenbezogene Wärmekapazität ist die eines wasser- und ölreichen Ellipsoids (1050 kg/m3, 3000 J/(kg K), Radius/3), eine Autorenschätzung.",
    }


def poses(asset):
    for p in asset["poses"]:
        f = FRUIT_KG[p["id"]] / APPLE_KG[p["id"]]
        scale_loads(asset, "carried", f, poses=[p["id"]])
    scale_loads(asset, "wind", geometry.CROWN[0] * geometry.CROWN[1])
    for p in asset["poses"]:
        for ld in p["loads"]:
            if ld["id"] == "wind-trunk":
                ld["distributed"]["wxNPerM"] = round(ld["distributed"]["wxNPerM"] * 1.85 / geometry.CROWN[0], 3)
    scale_pose_angles(asset, "fruit-laden", 0.45)
    scale_pose_angles(asset, "calm", 0.6)
    for p in asset["poses"]:
        for ld in p["loads"]:
            if ld["kind"] == "carried":
                ld["reference"] = "olive-yield"


def provenance(asset):
    pv = asset["provenance"]
    pv["author"] = "Semio staffage vegetation content worker PW-09"
    pv["method"] = "handcrafted"
    pv["created"] = "2026-11-22"
    refs = [x for x in pv["references"] if x["id"] not in ("olive-yield", "pretzsch-2015")]
    refs += [
        {"id": "connor-fereres-2005", "citation": "Connor, D. J. and Fereres, E. (2005). The physiology of adaptation and yield expression in olive. Horticultural Reviews 31, 155-229. Canopy structure, leaf traits, stomatal behaviour and yield of olive; used as a guide, values here are authored estimates."},
        {"id": "fernandez-moreno-1999", "citation": "Fernández, J. E. and Moreno, F. (1999). Water use by the olive tree. Journal of Crop Production 2(2), 101-162. Transpiration and conductance of olive under Mediterranean conditions."},
        {"id": "olive-yield", "citation": "General Mediterranean practice: an old, unirrigated traditional olive tree bears roughly 15 to 50 kg of olives in an on-year; the values used here are authored estimates, not cited measurements."},
        {"id": "rhizopoulou-1991", "citation": "Rhizopoulou, S., Meletiou-Christou, M. S. and Diamantoglou, S. (1991). Water relations for sun and shade leaves of four Mediterranean evergreen sclerophylls. Journal of Experimental Botany 42(5), 627-635. Sun and shade leaves of Olea europaea among other sclerophylls."},
    ]
    pv["references"] = refs
    for b in asset["rig"]["bones"]:
        if "mass" in b and "section" in b:
            b["mass"]["reference"] = "wood-handbook-2021"


TITLES = {
    "root-plate": ("Root plate and swollen butt", "Wurzelteller und verdickter Stammfuß"),
    "trunk": ("Gnarled stem", "Knorriger Stamm"),
    "ld": ("Central fork, lower part", "Stammgabel, unterer Teil"),
    "ld-b": ("Central fork, upper part", "Stammgabel, oberer Teil"),
    "spur-w1": ("Fruiting shoots, west lower", "Fruchttriebe, West unten"),
    "spur-e1": ("Fruiting shoots, east lower", "Fruchttriebe, Ost unten"),
    "spur-w2": ("Fruiting shoots, west upper", "Fruchttriebe, West oben"),
    "spur-e2": ("Fruiting shoots, east upper", "Fruchttriebe, Ost oben"),
}


def bone_titles(asset):
    for b in asset["rig"]["bones"]:
        if b["id"] in TITLES:
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
    stem = by["root-plate"]["lengthM"] + by["trunk"]["lengthM"]
    dstem = math.sqrt(4 * by["trunk"]["section"]["areaM2"] / math.pi)
    H, W, D = d["heightM"], d["widthM"], d["depthM"]
    replace_words(asset, [
        ("fruit tree", "olive tree"), ("apple tree", "olive tree"), ("the apples", "the olives"), ("apples", "olives"), ("Apples", "Olives"), ("apple", "olive"),
        ("fruit branches", "fruiting branches"), ("leader", "central fork"), ("Leader", "Central fork"),
        ("fruiting spurs", "fruiting shoots"), ("forks and spurs", "forks and short shoots"), ("Spur ends", "Shoot ends"),
    ], [
        ("Widerstand an beschatteter Kern", "Widerstand am beschatteten Kern"), ("das Fruchtholz", "die Fruchttriebe"), ("Fruchtspieße", "Kurztriebe"),
        ("Obstbaum", "Ölbaum"), ("Apfelbaum", "Ölbaum"), ("die Äpfel", "die Oliven"), ("Äpfeln", "Oliven"), ("Äpfel", "Oliven"), ("Apfel", "Oliven"),
        ("Stammverlängerung", "Stammgabel"),
    ])
    T = {
        ".title": ("Olive Tree, Old and Gnarled", "Ölbaum, alt und knorrig"),
        ".summary": (
            f"An old, gnarled olive tree (Olea europaea) of {n_en(H)} m with a massive, fissured stem of {dstem:.2f} m that forks at {n_en(stem)} m into heavy scaffold limbs, and a broad, silvery-green evergreen crown {n_en(W)} m across; the olive crop is carried as a seasonal pose and load, from the green fruit set to the full autumn harvest, in a hot, dry summer climate.",
            f"Ein alter, knorriger Ölbaum (Olea europaea) von {n_de(H)} m mit einem mächtigen, rissigen Stamm von {dstem:.2f} m".replace(".", ",", 1) + f", der sich in {n_de(stem)} m in schwere Leitäste gabelt, und einer breiten, silbergrünen immergrünen Krone von {n_de(W)} m Durchmesser; der Olivenbehang wird als jahreszeitliche Pose und Last geführt, vom grünen Fruchtansatz bis zur vollen Herbsternte, in heißem, trockenem Sommerklima.",
        ),
        ".provenance.notes": (
            f"Rig and drawings are derived from the handcrafted orchard standard of this archive by an exact per-bone affine morph and re-authored for an old traditional olive of about 150 years: a short, massive stem of {dstem:.2f} m forking at {n_en(stem)} m, two tiers of heavy scaffolds, an evergreen crown {n_en(W)} m wide and {n_en(H)} m high. Olive trees of this age commonly reach 5 to 10 m with crowns as wide as they are high. An old unirrigated tree bears roughly 15 to 50 kg of olives in an on-year; the 40 kg of the harvest pose and the drawn fruit clusters are authored estimates, not measured yields.",
            f"Rig und Zeichnungen sind aus dem handgezeichneten Hochstamm-Obstbaum dieses Archivs durch eine exakte affine Abbildung je Knochen abgeleitet und für einen alten traditionellen Ölbaum von etwa 150 Jahren neu bearbeitet: ein kurzer, mächtiger Stamm von {dstem:.2f} m".replace(".", ",", 1) + f", der sich in {n_de(stem)} m gabelt, zwei Etagen schwerer Leitäste, eine immergrüne Krone von {n_de(W)} m Breite und {n_de(H)} m Höhe. Ölbäume dieses Alters erreichen meist 5 bis 10 m mit Kronen, die so breit wie hoch sind. Ein alter, unbewässerter Baum trägt im Vollertragsjahr grob 15 bis 50 kg Oliven; die 40 kg der Erntepose und die gezeichneten Fruchtbüschel sind Autorenschätzungen, keine gemessenen Erträge.",
        ),
        ".symbols[olive-tree-symbol].description": (
            "A conventional plan sign for gardens, groves and site plans: the true plan outline of the crown, four scaffold directions and the stem section. It is a separate symbol and is never used silently for a pictorial scale.",
            "Ein konventionelles Grundrisszeichen für Gärten, Olivenhaine und Lagepläne: der wahre Grundrissumriss der Krone, vier Leitastrichtungen und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.",
        ),
        ".symbols[olive-tree-symbol].dimensionalNote": (
            f"A conventional simplification of the plant drawn at its true extent in model metres in all three views ({W:.2f} m wide, {D:.2f} m deep and {H:.2f} m high): the outline is the crown, the stem is drawn at its real position and thickness. It is a symbol, never used silently for a pictorial scale.",
            f"Eine konventionelle Vereinfachung der Pflanze in wahrer Ausdehnung in Modellmetern in allen drei Ansichten ({W:.2f} m breit, {D:.2f} m tief und {H:.2f} m hoch)".replace(".", ",") + ": der Umriss ist die Krone, der Stamm ist an seiner wirklichen Lage und in seiner wirklichen Dicke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.",
        ),
        ".symbols[tree-sign].title": ("Nominal tree sign with stem centre", "Nominelles Baumzeichen mit Stammmitte"),
        ".symbols[tree-sign].description": (
            "A circle with a centre dot and an inner ring as the conventional sign of a fruit-bearing tree in small-scale plans; the circle is a fixed paper size.",
            "Ein Kreis mit Mittelpunkt und Innenring als übliches Zeichen eines fruchttragenden Baums in kleinmaßstäblichen Plänen; der Kreis hat eine feste Papiergröße.",
        ),
        ".poses[calm].title": ("Calm hot day, green fruit set", "Windstiller heißer Tag, grüner Fruchtansatz"),
        ".poses[calm].description": (
            "Light air of 1.5 m/s at 10 m on a hot, dry summer day (32 °C, 40 % relative humidity, strong sun): about 10 kg of small green olives have set, the scaffolds are nearly straight; the stomata are partly closed at midday (80 % of the daytime maximum).",
            "Leiser Zug von 1,5 m/s in 10 m Höhe an einem heißen, trockenen Sommertag (32 °C, 40 % relative Feuchte, kräftige Sonne): etwa 10 kg kleine grüne Oliven haben angesetzt, die Leitäste sind fast gerade; die Spaltöffnungen sind mittags teilweise geschlossen (80 % des Tagesmaximums).",
        ),
        ".poses[windy].title": ("Breeze with half the crop", "Brise mit halbem Behang"),
        ".poses[windy].description": (
            "Moderate breeze of 5.5 m/s at 10 m (Beaufort 4) in late summer with about 25 kg of olives: the scaffolds lean downwind; drag by the reconfigured law (cd 0.35) on the dense evergreen crown.",
            "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4) im Spätsommer mit etwa 25 kg Oliven: die Leitäste neigen sich in Windrichtung; Widerstand nach dem Umformungsgesetz (cd 0,35) an der dichten immergrünen Krone.",
        ),
        ".poses[fruit-laden].title": ("Harvest, full olive crop", "Ernte, voller Olivenbehang"),
        ".poses[fruit-laden].description": (
            "Calm late-autumn day with the full crop of about 40 kg (some 10 000 ripening, purple to black olives): the outer shoots and scaffolds bow down slightly; the evergreen leaves transpire at 50 % under the lower autumn sun.",
            "Windstiller Spätherbsttag mit dem vollen Behang von etwa 40 kg (rund 10 000 reifende, violette bis schwarze Oliven): die äußeren Triebe und Leitäste biegen sich leicht nach unten; die immergrünen Blätter transpirieren bei tieferer Herbstsonne mit 50 %.",
        ),
        ".poses[storm].title": ("Storm with rain and olives", "Sturm mit Regen und Oliven"),
        ".poses[storm].description": (
            "Beaufort 9, 22 m/s at 10 m with rain, 30 kg of olives still on the tree: strong lean, the dense evergreen crown catches the full winter wind; wet bark and leaves, the critical case for the scaffold forks of the old stem.",
            "Beaufort 9, 22 m/s in 10 m Höhe mit Regen, noch 30 kg Oliven am Baum: starke Neigung, die dichte immergrüne Krone fängt den vollen Winterwind; nasse Rinde und Blätter, der maßgebende Fall für die Leitastgabeln des alten Stamms.",
        ),
        ".clips[wind-sway].title": ("Steady wind sway", "Gleichmäßiges Schwanken im Wind"),
        ".clips[wind-sway].description": (
            f"A deterministic 2.4 s sway cycle about the breeze lean with half the crop (5.5 m/s at 10 m; estimated first mode of the {n_en(H)} m tree with its stiff, heavy stem about 0.4 Hz). The scaffolds bob in anti-phase to the stem; the foliage masses swing about 3 degrees.",
            f"Ein deterministischer Schwingungszyklus von 2,4 s um die Neigung bei Brise mit halbem Behang (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des {n_de(H)} m hohen Baums mit seinem steifen, schweren Stamm etwa 0,4 Hz). Die Leitäste wippen gegenphasig zum Stamm; die Laubmassen schwingen etwa 3 Grad.",
        ),
        ".clips[gust].description": (
            "A single gust over 8 s from 5.5 to 14.0 m/s at 10 m with the crop of the breeze pose: the wind-load tracks rise with the reconfigured drag law, the scaffolds lean downwind with a level-wise lag.",
            "Eine einzelne Böe über 8 s von 5,5 auf 14,0 m/s in 10 m Höhe mit dem Behang der Brisenpose: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Leitäste neigen sich ebenenweise verzögert in Windrichtung.",
        ),
        ".clips[diurnal-transpiration].description": (
            "A 24 h cycle of a hot summer day compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, with stomatal closure at night and a deep midday depression typical of olive under high evaporative demand; leaf and bark temperatures are not authored but follow from the thermal solver.",
            "Ein 24-h-Zyklus eines heißen Sommertags auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei Windstille. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata geschlossen, mittags gibt es die für den Ölbaum bei hohem Verdunstungsanspruch typische tiefe Mittagsdepression; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser.",
        ),
        ".representations.50.decisions.silhouette": (
            f"At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: the swollen butt with its buttress roots, the massive stem of {dstem:.2f} m with spiral ridges, the central fork and the heavy scaffold limbs running into the crown, the fruiting branches where they show under the crown, the fruiting shoots, and the broad rounded evergreen crown as seven foliage masses whose outlines follow their lobes, with leaf-cluster arcs, leaf marks and the olive clusters (0.8 mm).",
            f"Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der verdickte Stammfuß mit seinen Wurzelanläufen, der mächtige Stamm von {dstem:.2f} m".replace(".", ",", 1) + " mit spiraligen Rippen, die Stammgabel und die schweren Leitäste, die in die Krone laufen, die Fruchtäste, wo sie unter der Krone sichtbar sind, die Fruchttriebe und die breite, runde immergrüne Krone als sieben Laubmassen, deren Umrisse ihren Lappen folgen, mit Blattbüschelbögen, Blattzeichen und den Olivenbüscheln (0,8 mm).",
        ),
        ".representations.50.decisions.retained": ("Retained: every woody member of the rig, every foliage mass; the olive clusters are a sample of the crop, not a count.", "Beibehalten: jedes Holzglied des Rigs, jede Laubmasse; die Olivenbüschel sind eine Auswahl des Behangs, keine Zählung."),
        ".representations.50.decisions.merged": ("Only the plug overlaps at the bone joints close the seams between tapered woody parts; the deep fissures of the stem are typified as six spiral ridge strokes.", "Nur die Überlappungen an den Knochengelenken schließen die Fugen zwischen den verjüngten Holzteilen; die tiefen Risse des Stamms sind als sechs spiralige Rippenstriche typisiert."),
        ".representations.50.decisions.omitted": ("Omitted: the tens of thousands of single leaves (0.2 to 0.3 mm wide on paper), flowers and the single drupes (1 to 1.5 mm); the olives are drawn as clusters of about 4 cm.", "Weggelassen: die zehntausenden Einzelblätter (0,2 bis 0,3 mm breit auf dem Papier), Blüten und die einzelnen Steinfrüchte (1 bis 1,5 mm); die Oliven sind als Büschel von etwa 4 cm gezeichnet."),
        ".representations.50.decisions.overlays": ("Each thermal region has its own shapes (the olive clusters carry the fruit region); ridges, arcs and leaf marks are strokes without a region and follow their bone.", "Jede thermische Region hat eigene Formen (die Olivenbüschel tragen die Fruchtregion); Rippen, Bögen und Blattzeichen sind Striche ohne Region und folgen ihrem Knochen."),
        ".representations.100.decisions.silhouette": (
            "At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: swollen butt, massive stem with three spiral ridges, central fork and scaffold limbs under the crown, four fruiting-shoot strokes, the crown as seven masses with broader lobes and leaf-cluster arcs, and the visible olive clusters (0.45 mm): eighteen in the side elevation, seventeen in the front elevation, twenty-two in the plan. Plan: lobes in sun and shade, leaf arcs and clusters; the stem is a dot under the crown.",
            "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: verdickter Stammfuß, mächtiger Stamm mit drei spiraligen Rippen, Stammgabel und Leitäste unter der Krone, vier Fruchttrieb-Striche, die Krone als sieben Massen mit breiteren Lappen und Blattbüschelbögen und die sichtbaren Olivenbüschel (0,45 mm): achtzehn in der Seitenansicht, siebzehn in der Vorderansicht, zweiundzwanzig im Grundriss. Grundriss: Lappen in Sonne und Schatten, Blattbögen und Büschel; der Stamm ist ein Punkt unter der Krone.",
        ),
        ".representations.100.decisions.retained": ("Retained: seven foliage masses, swollen butt, stem, central fork, all scaffold sections, the fruiting shoots and the visible olive clusters as filled dots.", "Beibehalten: sieben Laubmassen, verdickter Stammfuß, Stamm, Stammgabel, alle Leitastabschnitte, die Fruchttriebe und die sichtbaren Olivenbüschel als gefüllte Punkte."),
        ".representations.100.decisions.merged": ("Single leaves and twig ends are merged into lobed mass outlines; the stem fissures into three spiral ridges.", "Einzelblätter und Zweigenden sind zu gelappten Massenumrissen zusammengefasst; die Stammrisse zu drei spiraligen Rippen."),
        ".representations.100.decisions.omitted": ("Omitted: individual leaves, fine twigs, forks and knots; most olives (the body carries a sample of twenty clusters); the fine 1:50 leaf marks are replaced by arcs.", "Weggelassen: einzelne Blätter, feine Zweige, Gabelungen und Astknoten; die meisten Oliven (der Körper trägt eine Auswahl von zwanzig Büscheln); die feinen Blattzeichen von 1:50 sind durch Bögen ersetzt."),
        ".representations.100.decisions.overlays": ("Every region keeps its shapes; the shade hatch is a pure drawing hatch and does not stand for temperature; the olive dots carry the fruit region.", "Jede Region behält ihre Formen; die Schattenschraffur ist eine reine Zeichenschraffur und steht nicht für Temperatur; die Olivenpunkte tragen die Fruchtregion."),
        ".representations.200.decisions.silhouette": (
            "At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: massive stem with its swollen butt, central fork and the scaffold limbs of two tiers under a broad rounded crown of seven foliage masses (five sun masses, two shade cores) with scalloped outlines; the visible olive clusters are single dots (0.26 mm; six in each elevation, seven in the plan). Plan: the lobes around the top mass and the stem as a dot.",
            "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: mächtiger Stamm mit verdicktem Fuß, Stammgabel und die Leitäste zweier Etagen unter einer breiten, runden Krone aus sieben Laubmassen (fünf Sonnenmassen, zwei Schattenkerne) mit gewellten Umrissen; die sichtbaren Olivenbüschel sind einzelne Punkte (0,26 mm; sechs in jeder Ansicht, sieben im Grundriss). Grundriss: die Lappen um die Gipfelmasse und der Stamm als Punkt.",
        ),
        ".representations.200.decisions.retained": ("Retained: swollen butt, stem, central fork and the scaffold limbs thicker than 0.5 mm on paper, one fissure line, and the visible sample olive clusters (5 cm, 0.26 mm).", "Beibehalten: verdickter Stammfuß, Stamm, Stammgabel und die Leitäste dicker als 0,5 mm auf dem Papier, eine Risslinie und die sichtbaren Olivenbüschel der Auswahl (5 cm, 0,26 mm)."),
        ".representations.200.decisions.omitted": ("Omitted: twigs, bark ridges, knots, leaf marks and all other olives.", "Weggelassen: Zweige, Rindenrippen, Astknoten, Blattzeichen und alle übrigen Oliven."),
        ".representations.200.decisions.hatching": ("No hatching. Internal edges: one fissure line on the stem.", "Keine Schraffur. Innenkanten: eine Risslinie am Stamm."),
        ".representations.500.decisions.silhouette": (
            f"At 1:500 (1 m is 2 mm) the olive tree is one closed, broad and rounded crown, {W * 2:.0f} mm wide, with a scalloped underside, four low side lobes and four shoulder lobes under a rounded top, above a short, massive stem of {n_en(stem)} m with its swollen butt; the drawings are the true views of the 1:500 body. Front: stem, lower fork and the crown. Plan: one scalloped crown outline from above; the stem is hidden.",
            f"Bei 1:500 (1 m sind 2 mm) ist der Ölbaum eine geschlossene, breite und runde Krone von {W * 2:.0f} mm Breite mit gewellter Unterseite, vier tiefen Seitenlappen und vier Schulterlappen unter einem runden Gipfel, über einem kurzen, mächtigen Stamm von {n_de(stem)} m mit verdicktem Fuß; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers. Vorderansicht: Stamm, untere Gabelung und Krone. Grundriss: ein gewellter Kronenumriss von oben; der Stamm ist verdeckt.",
        ),
        ".representations.500.decisions.retained": ("Retained: the stem with its swollen butt and the crown outline with its low side lobes; the stem runs on into the crown for a clean join.", "Beibehalten: der Stamm mit verdicktem Fuß und der Kronenumriss mit seinen tiefen Seitenlappen; der Stamm läuft für einen sauberen Anschluss in die Krone weiter."),
        ".representations.500.decisions.merged": ("Merged into the crown path: five sun foliage masses, both shade cores, the scaffold limbs, the fruiting shoots and the fruit.", "Im Kronenpfad zusammengefasst: fünf Sonnenlaubmassen, beide Schattenkerne, die Leitäste, die Fruchttriebe und die Früchte."),
        ".representations.500.decisions.omitted": ("Omitted: single scaffolds, shoots, leaves and every olive (2 cm is 0.04 mm); no internal edge.", "Weggelassen: einzelne Leitäste, Triebe, Blätter und jede Olive (2 cm sind 0,04 mm); keine Innenkante."),
        ".representations.500.decisions.thinElements": ("Scaffolds and shoots are not thickened; they are merged into the filled crown, so the outline stays within the real outline tolerance of the 1:50 drawing.", "Leitäste und Triebe werden nicht verdickt; sie sind in der gefüllten Krone aufgegangen, sodass der Umriss innerhalb der Toleranz des wahren Umrisses der 1:50-Zeichnung bleibt."),
        ".representations.500.decisions.overlays": ("Overlays map onto: trunk-bark (stem) and, merged in the crown path, leaf-sun, leaf-shade, limb-bark, spur-twig and fruit; merged shapes show the area-weighted value of their regions, labelled as merged.", "Overlays bilden ab auf: trunk-bark (Stamm) und, im Kronenpfad zusammengefasst, leaf-sun, leaf-shade, limb-bark, spur-twig und fruit; zusammengefasste Formen zeigen den flächengewichteten Wert ihrer Regionen, als zusammengefasst beschriftet."),
        ".volumes.50.decisions[0]": (
            f"Body at 1:50: the swollen butt with five buttress roots, the massive stem of {dstem:.2f} m with its swelling below the fork and a knot, the central fork in two parts, four heavy scaffold limbs in two tiers (swept lofts with branch collars, tapered and bowed, running out of the rig plane at 189, 9, 143 and 323 degrees in plan), fourteen second-order fruiting branches with eight forks, four fruiting shoots, and one broad rounded crown of eighty-three leaf lobes (a drooping skirt of twelve, the widest ring of twelve, ten shoulder lobes, nine and eight lobes on the two upper rings, seven at the top, fourteen small lobes between the rings and eleven shade lobes of the inner crown); fifty-three olive clusters of about 4 cm hang all round the crown. Derived from the orchard standard of this archive by an exact per-bone morph (stem and butt thickened 1.85 and 1.75 times, crown widened 1.2 and raised 1.15 times).",
            f"Körper bei 1:50: der verdickte Stammfuß mit fünf Wurzelanläufen, der mächtige Stamm von {dstem:.2f} m".replace(".", ",", 1) + " mit Verdickung unter der Gabelung und einem Astknoten, die Stammgabel in zwei Teilen, vier schwere Leitäste in zwei Etagen (geführte Lofts mit Astringen, verjüngt und gebogen, im Grundriss unter 189, 9, 143 und 323 Grad aus der Rig-Ebene laufend), vierzehn Fruchtäste zweiter Ordnung mit acht Gabelungen, vier Fruchttriebe und eine breite, runde Krone aus dreiundachtzig Laublappen (ein hängender Saum aus zwölf, der breiteste Ring aus zwölf, zehn Schulterlappen, neun und acht Lappen auf den beiden oberen Ringen, sieben am Gipfel, vierzehn kleine Lappen zwischen den Ringen und elf Schattenlappen der Innenkrone); dreiundfünfzig Olivenbüschel von etwa 4 cm hängen rund um die Krone. Abgeleitet aus dem Hochstamm des Archivs durch eine exakte Abbildung je Knochen (Stamm und Fuß 1,85- und 1,75-fach verdickt, Krone 1,2-fach verbreitert und 1,15-fach erhöht).",
        ),
        ".volumes.50.decisions[1]": (
            "Thermal at 1:50: stem bark is the butt, buttresses, stem and lower fork; limb bark the upper fork, scaffolds, fruiting branches and forks; the fruiting shoots are the twig region; sun leaves are the outer lobes, shade leaves the inner lobes, the olive clusters the fruit region, all by reference to the body. The twelve transpiration outlets are lens patches lying in the free outer face of the lobe that carries each emitter anchor (in the top of the shade lobes), 2 cm proud.",
            "Thermik bei 1:50: Stammrinde sind Stammfuß, Wurzelanläufe, Stamm und untere Stammgabel; Astrinde obere Stammgabel, Leitäste, Fruchtäste und Gabelungen; die Fruchttriebe sind die Zweigregion; Sonnenblätter sind die äußeren Lappen, Schattenblätter die inneren, die Olivenbüschel die Fruchtregion, alle als Verweis auf den Körper. Die zwölf Transpirationsauslässe sind Linsenflächen in der freien Außenseite des Lappens, der den jeweiligen Emitteranker trägt (bei den Schattenlappen in der Oberseite), 2 cm vorstehend.",
        ),
    }
    fr = lambda x: x.get("region") == "fruit"
    c100 = [count(asset, "100", v, fr) for v in ("elevation", "front", "plan")]
    c200 = [count(asset, "200", v, fr) for v in ("elevation", "front", "plan")]
    v100, v200, v500 = (count_solids(asset, k, "body", lambda x: x.get("material") == "olive-fruit") for k in ("100", "200", "500"))
    t500 = count_solids(asset, "500", "thermal", lambda x: x.get("region") == "fruit")
    T.update({
        ".representations.50.decisions.thinElements": ("Shoot ends of 0.014 m stay as 0.13 mm strokes; ridges and arcs are 0.13 mm strokes and the leaf marks 0.18 mm outlines, all above the pen minimum.", "Triebenden von 0,014 m bleiben 0,13-mm-Striche; Rippen und Bögen sind 0,13-mm-Striche und die Blattzeichen 0,18-mm-Umrisse, alle über dem Stiftminimum."),
        ".representations.100.decisions.thinElements": ("Thin strokes are at least 0.1 mm: ridges and shoots use the texture and detail pens, scaffolds keep their real section width.", "Dünne Striche sind mindestens 0,1 mm: Rippen und Triebe mit Textur- und Detailstift, Leitäste in realer Querschnittsbreite."),
        ".representations.100.decisions.silhouette": (
            f"At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: swollen butt, massive stem with three spiral ridges, central fork and scaffold limbs under the crown, four fruiting-shoot strokes, the crown as seven masses with broader lobes and leaf-cluster arcs, and the visible olive clusters (0.45 mm): {en_words(c100[0])} in the side elevation, {en_words(c100[1])} in the front elevation, {en_words(c100[2])} in the plan. Plan: lobes in sun and shade, leaf arcs and clusters; the stem is a dot under the crown.",
            f"Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: verdickter Stammfuß, mächtiger Stamm mit drei spiraligen Rippen, Stammgabel und Leitäste unter der Krone, vier Fruchttrieb-Striche, die Krone als sieben Massen mit breiteren Lappen und Blattbüschelbögen und die sichtbaren Olivenbüschel (0,45 mm): {de_words(c100[0])} in der Seitenansicht, {de_words(c100[1])} in der Vorderansicht, {de_words(c100[2])} im Grundriss. Grundriss: Lappen in Sonne und Schatten, Blattbögen und Büschel; der Stamm ist ein Punkt unter der Krone.",
        ),
        ".representations.200.decisions.silhouette": (
            f"At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: massive stem with its swollen butt, central fork and the scaffold limbs of two tiers under a broad rounded crown of seven foliage masses (five sun masses, two shade cores) with scalloped outlines; the visible olive clusters are single dots (0.26 mm; {en_words(c200[0])} in the side elevation, {en_words(c200[1])} in the front elevation, {en_words(c200[2])} in the plan). Plan: the lobes around the top mass and the stem as a dot.",
            f"Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: mächtiger Stamm mit verdicktem Fuß, Stammgabel und die Leitäste zweier Etagen unter einer breiten, runden Krone aus sieben Laubmassen (fünf Sonnenmassen, zwei Schattenkerne) mit gewellten Umrissen; die sichtbaren Olivenbüschel sind einzelne Punkte (0,26 mm; {de_words(c200[0])} in der Seitenansicht, {de_words(c200[1])} in der Vorderansicht, {de_words(c200[2])} im Grundriss). Grundriss: die Lappen um die Gipfelmasse und der Stamm als Punkt.",
        ),
        ".representations.100.decisions.omitted": (f"Omitted: individual leaves, fine twigs, forks and knots; most olives (the body carries a sample of {en_words(v100)} clusters); the fine 1:50 leaf marks are replaced by arcs.", f"Weggelassen: einzelne Blätter, feine Zweige, Gabelungen und Astknoten; die meisten Oliven (der Körper trägt eine Auswahl von {de_words(v100)} Büscheln); die feinen Blattzeichen von 1:50 sind durch Bögen ersetzt."),
        ".volumes.100.decisions[0]": (
            f"Body at 1:100: the buttress roots merge into one flared root loft, every limb keeps its taper with four or five sections, eight fruiting branches remain and the forks and short shoots are dropped, the fruiting shoots are single twigs; the crown is fifty-nine broader lobes (skirt and widest ring of twelve each, eight shoulder lobes, eight on the upper ring, six small ones between, seven at the top, six shade lobes) and {en_words(v100)} olive clusters of about 4.4 cm.",
            f"Körper bei 1:100: Die Wurzelanläufe gehen in einem ausgestellten Wurzelloft auf, jeder Ast behält seine Verjüngung mit vier bis fünf Schnitten, acht Fruchtäste bleiben, Gabelungen und Kurztriebe entfallen, die Fruchttriebe sind einzelne Zweige; die Krone besteht aus neunundfünfzig breiteren Lappen (Saum und breitester Ring aus je zwölf, acht Schulterlappen, acht auf dem oberen Ring, sechs kleine dazwischen, sieben am Gipfel, sechs Schattenlappen) und {de_words(v100)} Olivenbüscheln von etwa 4,4 cm.",
        ),
        ".volumes.200.decisions[0]": (
            f"Body at 1:200: swollen butt, stem, central fork and the eight scaffold parts with three sections each, no fruiting branches and no shoots; the crown is twenty-one masses (a widest ring of eight, six shoulder masses, four upper masses over the ends of the upper scaffolds, one top mass, two shade cores) and {en_words(v200)} olive clusters of about 5 cm.",
            f"Körper bei 1:200: verdickter Stammfuß, Stamm, Stammgabel und die acht Leitastteile mit je drei Schnitten, keine Fruchtäste und keine Triebe; die Krone besteht aus einundzwanzig Massen (ein breitester Ring aus acht, sechs Schultermassen, vier obere Massen über den Enden der oberen Leitäste, eine Gipfelmasse, zwei Schattenkerne) und {de_words(v200)} Olivenbüscheln von etwa 5 cm.",
        ),
        ".volumes.500.decisions[1]": (
            f"Thermal at 1:500: stem bark and leaves by reference; the upper central fork, the scaffold limbs, the fruiting shoots and {en_words(t500)} olive clusters (9 cm, the smallest feature of the scale) are thermal solids of their own inside the crown; two outlets.",
            f"Thermik bei 1:500: Stammrinde und Laub als Verweis; obere Stammgabel, Leitäste, Fruchttriebe und {de_words(t500)} Olivenbüschel (9 cm, das kleinste Merkmal des Maßstabs) sind eigene Thermikkörper in der Krone; zwei Auslässe.",
        ),
    })
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
    a["tags"] = ["tree", "evergreen", "mediterranean", "fruit-tree", "fruit-load", "drought-tolerant", "wind", "transpiration"]
    materials(a)
    retone(a, tone)
    ridges(a)
    thermal(a, old_lateral)
    poses(a)
    provenance(a)
    bone_titles(a)
    textual(a)
    return doc


DISTINCTION = (
    "An old, gnarled olive tree with a massive, fissured stem and a broad, silvery evergreen crown; the olive crop is carried as a seasonal pose and load in a hot, dry summer climate.",
    "Ein alter, knorriger Ölbaum mit mächtigem, rissigem Stamm und breiter, silbriger immergrüner Krone; der Olivenbehang wird als jahreszeitliche Pose und Last in heißem, trockenem Sommerklima geführt.",
)
