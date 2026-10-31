import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ivy import *

CROWNS = ["crown-L", "crown-R"]


def anchors_emitters():
    A = [anchors_support("post-L", T("Left post shoe on the footing", "Linker Pfostenschuh auf dem Fundament"), 0.11, 0.11, id="base-support-L"), anchors_support("post-R", T("Right post shoe on the footing", "Rechter Pfostenschuh auf dem Fundament"), 0.11, 0.11, id="base-support-R")]
    up_, lo_ = [], []
    for k in range(1, 5):
        for z, lst, y in (("hi", up_, 0.45), ("lo", lo_, 0.35)):
            bone = f"fol-{k}-{z}"
            aid = f"tp-{'upper' if z == 'hi' else 'lower'}-{k}"
            A.append(emitter_anchor(aid, bone, y, 0.0, T(f"Transpiration of the {'upper' if z == 'hi' else 'lower'} leaf mat {k}", f"Transpiration des {'oberen' if z == 'hi' else 'unteren'} Blattteppichs {k}")))
            lst.append(aid)
    for c, lst in (("crown-L", up_), ("crown-R", up_)):
        aid = f"tp-upper-{c}"
        A.append(emitter_anchor(aid, c, 0.15, 0.0, T(f"Transpiration of the overhanging shoots ({c[-1]})", f"Transpiration der überhängenden Triebe ({c[-1]})")))
        lst.append(aid)
    return A, up_, lo_


def make():
    A, up_, lo_ = anchors_emitters()
    em = [transpiration("transpiration-upper", T("Transpiration of the upper leaf mats and shoots (distributed over six anchors)", "Transpiration der oberen Blattteppiche und Triebe (über sechs Anker verteilt)"), up_, "leaves-upper", up_in_bone(B, "fol-1-hi"), 1.6),
          transpiration("transpiration-lower", T("Transpiration of the lower leaf mats (distributed over four anchors)", "Transpiration der unteren Blattteppiche (über vier Anker verteilt)"), lo_, "leaves-lower", up_in_bone(B, "fol-1-lo"), 1.6)]
    regions = [
        leaf_region("leaves-upper", T("Upper ivy leaves", "Obere Efeublätter"), "ivy-leaf", 8.0, 0.06, "vertical-plate", 0.5, 1200, 0.003, 5.3, "c3", "metcalfe-2005"),
        leaf_region("leaves-lower", T("Lower ivy leaves", "Untere Efeublätter"), "ivy-leaf", 7.0, 0.06, "vertical-plate", 0.4, 1200, 0.0025, 4.7, "c3", "metcalfe-2005"),
        passive_region("stems", T("Ivy stems with adhesive roots", "Efeutriebe mit Haftwurzeln"), "stem", "ivy-stem", 0.45, 0.012, "cylinder", 0.4, 8000, "metcalfe-2005"),
        passive_region("trellis-timber", T("Timber posts, rails and battens", "Holzpfosten, Riegel und Latten"), "surface", "timber-c24", 6.3, 0.03, "vertical-plate", 0.5, 7000, "en-338", source="authored-estimate"),
        passive_region("post-shoes", T("Galvanised steel post shoes", "Verzinkte Stahl-Pfostenschuhe"), "surface", "steel-s235", 0.5, 0.1, "vertical-plate", 0.4, 10500, "en-10025-2"),
    ]
    regions[2]["hidden"] = T("The stems run behind the leaf mat; in elevation they show only at 1:50 as strokes through the leaf gaps.", "Die Triebe laufen hinter dem Blattteppich; in der Ansicht zeigen sie sich nur bei 1:50 als Striche durch die Blattlücken.")
    mats = [{"id": "ivy-leaf", "title": T("Ivy leaf (leathery, dark green)", "Efeublatt (ledrig, dunkelgrün)"), "emissivity": 0.97, "solarAbsorptivity": 0.55, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "ivy-stem", "title": T("Woody ivy stem", "Verholzter Efeutrieb"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": 650, "conductivityWmK": 0.15, "specificHeatJKgK": 2400, "source": "authored-estimate", "reference": "metcalfe-2005"},
            {"id": "timber-c24", "title": T("Structural timber C24", "Bauholz C24"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 420, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
            {"id": "steel-s235", "title": T("Galvanised steel S235", "Verzinkter Stahl S235"), "emissivity": 0.35, "solarAbsorptivity": 0.45, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "en-10025-2"}]
    def tb(up, lo, emit, wet=None, shade=1.0):
        return bind({"leaves-upper": (1.0, up, wet), "leaves-lower": (1.0, lo, wet), "stems": (0.2, 0.05, wet), "trellis-timber": (1.0, up * 0.7, wet), "post-shoes": (1.0, up * 0.6, wet)}, {"transpiration-upper": emit[0], "transpiration-lower": emit[1]})
    def winds(v, cf):
        out = [wind_load(f"wind-{b}", T(f"Wind drag on {b}", f"Winddruck auf {b}"), b, 0.0, 0.0, cf, 0.35, v) for b in FOLS]
        out += [wind_load(f"wind-{c}", T(f"Wind drag on {c}", f"Winddruck auf {c}"), c, 0.1, 0.0, cf, 0.1, v) for c in CROWNS]
        out.append(wind_load("wind-panel", T("Wind drag on the lattice panel", "Winddruck auf das Gitterfeld"), "panel", 0.9, 0.75, 1.3, 0.45, v))
        return out
    def mats_off(a, c):
        o = {b: {"angleDeg": r4(a)} for b in FOLS}
        o.update({"crown-L": {"angleDeg": r4(c)}, "crown-R": {"angleDeg": r4(-c)}})
        return o
    snowl = [vertical_load(f"snow-{b}", "snow", T(f"Snow on the leaf mat {b}", f"Schnee auf dem Blattteppich {b}"), b, 0.3, 0.0, -9.81 * 4.0 * 0.12, "en-1991-1-3") for b in FOLS]
    poses = [
        pose("summer-calm", T("Summer, calm", "Sommer, windstill"), T("Light air of 1.5 m/s on a sunny day: the leaf mats hang flat against the lattice and the shoots overhang the top rail; the upper leaves are sunlit and transpire at the full rate.", "Leiser Zug von 1,5 m/s an einem sonnigen Tag: Die Blattteppiche liegen flach am Gitter und die Triebe hängen über den oberen Riegel; die oberen Blätter sind besonnt und transpirieren mit voller Rate."), {}, tb(0.85, 0.5, (0.9, 0.7)), winds(1.5, 1.2)),
        pose("summer-night", T("Summer night", "Sommernacht"), T("The same dense mat on a clear night: no sun, stomata nearly closed (C3 plant), a cold sky draws radiative heat from the leaves.", "Derselbe dichte Teppich in einer klaren Nacht: keine Sonne, Spaltöffnungen fast geschlossen (C3-Pflanze), ein kalter Himmel entzieht den Blättern Strahlungswärme."), {}, tb(0.0, 0.0, (0.05, 0.05)), []),
        pose("breezy", T("Fresh breeze", "Frische Brise"), T("A breeze of 8 m/s from the left lifts the leaf mats a few degrees off the lattice and tosses the shoots; leaf undersides turn to the wind and transpiration rises.", "Eine Brise von 8 m/s von links hebt die Blattteppiche einige Grad vom Gitter und wirft die Triebe; die Blattunterseiten drehen sich in den Wind und die Transpiration steigt."), mats_off(-4, -10), tb(0.8, 0.5, (1.0, 0.9)), winds(8, 1.2)),
        pose("storm", T("Storm", "Sturm"), T("A gale of 24 m/s: the whole screen takes the wind load of a closed surface through the posts; the leaf mats are pressed back and the shoots stream flat.", "Ein Sturm von 24 m/s: Die ganze Wand nimmt die Windlast einer geschlossenen Fläche über die Pfosten auf; die Blattteppiche werden zurückgedrückt und die Triebe streichen flach."), mats_off(-8, -22), tb(0.5, 0.3, (0.4, 0.3)), winds(24, 1.3)),
        pose("winter-snow", T("Winter, snow and frost", "Winter, Schnee und Frost"), T("Evergreen ivy under wet snow in frost: the mats sag by the snow load, the leaves are curled, and the plant transpires hardly at all.", "Immergrüner Efeu unter Nassschnee bei Frost: Die Teppiche sacken unter der Schneelast ab, die Blätter sind eingerollt und die Pflanze transpiriert kaum."), mats_off(4, 14), tb(0.25, 0.15, (0.02, 0.02), wet=0.4), snowl),
    ]
    return A, em, regions, mats, poses


def clips(poses):
    def bo(pid):
        return [p for p in poses if p["id"] == pid][0]["bones"]
    base = {k: v["angleDeg"] for k, v in bo("breezy").items()}
    specs = {}
    for i, b in enumerate(FOLS):
        specs[b] = (2.2, 45 * i, 2)
    specs["crown-L"] = (9.0, 0, 1)
    specs["crown-R"] = (9.0, 140, 1)
    sway = sway_clip(B.list, "wind-sway", T("Wind sway", "Windschwingung"), T("A deterministic 1.6 s cycle: the leaf mats flutter at 1.25 Hz about the breeze lean with a phase that runs along the lattice; the overhanging shoots toss at 0.625 Hz.", "Ein deterministischer Zyklus von 1,6 s: Die Blattteppiche flattern mit 1,25 Hz um die Neigung der Brise mit einer am Gitter entlanglaufenden Phase; die überhängenden Triebe schwingen mit 0,625 Hz."), 1.6, base, specs)
    for k in sway["keys"]:
        k["pose"] = "breezy"
    keys = []
    for t, pid, amp in ((0, "summer-calm", 0.5), (1.4, "breezy", 2.5), (2.8, "storm", 4.0), (4.0, "breezy", 2.5), (5.6, "summer-calm", 1.0), (7.0, "summer-calm", 0.5)):
        ov = {}
        for i, b in enumerate(FOLS):
            ov[b] = {"angleDeg": r4(clamp(bo(pid).get(b, {}).get("angleDeg", 0) + amp * sin(radians(50 * i + 160 * t)), -8 if b.endswith("lo") else -10, 8 if b.endswith("lo") else 10))}
        for c in CROWNS:
            ov[c] = {"angleDeg": r4(clamp(bo(pid).get(c, {}).get("angleDeg", 0) + 2 * amp * sin(radians(120 * t + (60 if c == "crown-R" else 0))), -25, 25))}
        keys.append({"t": t, "pose": pid, "ease": "smooth", "bones": ov})
    gust = {"id": "gust", "title": T("Gust of wind", "Windböe"), "description": T("A 7 s gust: calm, breeze, storm peak at 2.8 s with the mats pressed back and the shoots streaming, decay to calm. Transpiration follows with stomatal closure at the peak.", "Eine Böe von 7 s: Stille, Brise, Sturmspitze bei 2,8 s mit zurückgedrückten Teppichen und wehenden Trieben, Abklingen zur Stille. Die Transpiration folgt, am Höhepunkt schließen sich die Spaltöffnungen."), "durationS": 7.0, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "rise", "fromS": 0, "toS": 2.8, "title": T("Gust builds up", "Böe baut sich auf")}, {"id": "decay", "fromS": 2.8, "toS": 7.0, "title": T("Gust decays", "Böe klingt ab")}], "events": [{"t": 2.8, "kind": "motion", "title": T("Gust peak", "Böenspitze")}],
            "tracks": [{"target": "emitter:transpiration-upper", "ease": "smooth", "keys": [{"t": 0, "v": 0.9}, {"t": 1.4, "v": 1.0}, {"t": 2.8, "v": 0.4}, {"t": 5.6, "v": 0.9}, {"t": 7.0, "v": 0.9}]}]}
    return [sway, gust]


def representations():
    r = {}
    r["500"] = rep(500, 351, 1000, pens(0.15, None, None, None, 0.25), [], build_500(), decisions(
        T("At 1:500 (1 m is 2 mm) the ivy-clad trellis is one upright screen of 3.8 by 4.1 mm with two small overhanging crowns of shoots above its top edge.", "Bei 1:500 (1 m sind 2 mm) ist das berankte Spalier ein aufrechter Schirm von 3,8 mal 4,1 mm mit zwei kleinen überhängenden Triebkronen über der Oberkante."),
        T("Retained: the real width of 1.9 m, height of 2.15 m and the two overhanging crowns that break the top edge.", "Beibehalten: die wahre Breite von 1,9 m, die Höhe von 2,15 m und die beiden überhängenden Kronen, die die Oberkante brechen."),
        T("Merged: posts, steel shoes, rails, lattice, stems and both leaf zones are one silhouette (all regions listed); the crowns stay separate because they move.", "Zusammengefasst: Pfosten, Stahlschuhe, Riegel, Gitter, Triebe und beide Blattzonen sind eine Silhouette (alle Regionen aufgeführt); die Kronen bleiben getrennt, weil sie sich bewegen."),
        T("Omitted: the lattice, the open strip below the leaves, single leaves.", "Weggelassen: das Gitter, der offene Streifen unter den Blättern, Einzelblätter."),
        T("The posts (90 mm) would be 0.18 mm wide, below the minimum feature; they are part of the silhouette and not drawn separately.", "Die Pfosten (90 mm) wären 0,18 mm breit, unter dem Mindestmerkmal; sie sind Teil der Silhouette und nicht einzeln gezeichnet."),
        T("None.", "Keine."),
        T("One mid-grey filled screen with a 0.15 mm outline; the crowns are slightly lighter.", "Ein mittelgrauer gefüllter Schirm mit Kontur von 0,15 mm; die Kronen sind etwas heller."),
        T("The thermal view paints the screen with the area-weighted value of all regions; the crowns with the upper-leaf value.", "Die Thermalansicht färbt den Schirm mit dem flächengewichteten Wert aller Regionen; die Kronen mit dem Wert der oberen Blätter.")))
    r["200"] = rep(200, 151, 350, pens(0.18, 0.13, None, None, 0.25), [], build_200(), decisions(
        T("At 1:200 (1 m is 5 mm) two filled posts with steel shoes and the two rails frame eight scalloped leaf mats: two rows (lower dark, upper lighter) of four mats around the four stems.", "Bei 1:200 (1 m sind 5 mm) rahmen zwei gefüllte Pfosten mit Stahlschuhen und die zwei Riegel acht gezackte Blattteppiche: zwei Reihen (unten dunkler, oben heller) zu je vier Teppichen um die vier Triebe."),
        T("Retained: posts (0.45 mm), shoes, rails, the rows of leaf mats with their overlaps and the top shoots that break the top rail.", "Beibehalten: Pfosten (0,45 mm), Schuhe, Riegel, die Reihen der Blattteppiche mit ihren Überlagerungen und die oberen Triebe, die den oberen Riegel durchbrechen."),
        T("Merged: each leaf mat is one lobed mass; the lattice behind it is merged into the mat.", "Zusammengefasst: Jeder Blattteppich ist eine gelappte Masse; das Gitter dahinter geht im Teppich auf."),
        T("Omitted: the lattice battens (20 mm, 0.1 mm), the stems, single leaves.", "Weggelassen: die Gitterlatten (20 mm, 0,1 mm), die Triebe, Einzelblätter."),
        T("Battens of 20 mm would be 0.1 mm wide and are dropped rather than thickened.", "Latten von 20 mm wären 0,1 mm breit und entfallen statt verdickt zu werden."),
        T("None.", "Keine."),
        T("Posts and rails mid-grey, leaf mats dark below and lighter above.", "Pfosten und Riegel mittelgrau, Blattteppiche unten dunkel und oben heller."),
        T("Each mat is one leaf region (upper or lower); posts, rails and shoes carry their own regions.", "Jeder Teppich ist eine Blattregion (oben oder unten); Pfosten, Riegel und Schuhe tragen ihre eigenen Regionen.")))
    r["100"] = rep(100, 76, 150, pens(0.2, 0.13, 0.1, None, 0.3), [], build_100(), decisions(
        T("At 1:100 (1 m is 10 mm) the posts, rails and steel shoes are filled and the lattice shows as a grid of 0.13 mm lines in the open lower strip; the leaf mats are merged outlines with a scalloped, leaf-lobed edge.", "Bei 1:100 (1 m sind 10 mm) sind Pfosten, Riegel und Stahlschuhe gefüllt und das Gitter zeigt sich im offenen unteren Streifen als Raster aus Linien von 0,13 mm; die Blattteppiche sind verschmolzene Konturen mit gezacktem Blattlappenrand."),
        T("Retained: posts, rails, shoes, the grid of five by five battens, mat outlines with leaf lobes.", "Beibehalten: Pfosten, Riegel, Schuhe, das Raster aus fünf mal fünf Latten, Teppichkonturen mit Blattlappen."),
        T("Merged: 60 to 70 leaves per mat form one outline (leaves 85 mm radius).", "Zusammengefasst: 60 bis 70 Blätter je Teppich bilden eine Kontur (Blätter von 85 mm Radius)."),
        T("Omitted: stems and single leaf outlines.", "Weggelassen: Triebe und einzelne Blattumrisse."),
        T("Battens (20 mm = 0.2 mm) are drawn as 0.13 mm lines, a reduction of 7 mm, every second batten omitted.", "Latten (20 mm = 0,2 mm) sind als Linien von 0,13 mm gezeichnet, eine Verringerung um 7 mm, jede zweite Latte entfällt."),
        T("None.", "Keine."),
        T("Grid lines light; mats dark below, lighter above, one 0.2 mm body pen.", "Rasterlinien hell; Teppiche unten dunkel, oben heller, ein Körperstift von 0,2 mm."),
        T("Each mat maps to one leaf region; grid lines carry the timber region.", "Jeder Teppich entspricht einer Blattregion; Rasterlinien tragen die Holzregion.")))
    r["50"] = rep(50, 20, 75, pens(0.25, 0.18, 0.13, None, 0.35), [], build_50(), decisions(
        T("At 1:50 (1 m is 20 mm) the timber trellis is built from filled posts (90 mm), rails, eight vertical and seven horizontal 20 mm battens and the steel shoes; the ivy stems climb in front of the open lower strip and the leaf mats have scalloped lobed edges.", "Bei 1:50 (1 m sind 20 mm) ist das Holzspalier aus gefüllten Pfosten (90 mm), Riegeln, acht senkrechten und sieben waagrechten Latten von 20 mm und den Stahlschuhen gebaut; die Efeutriebe klettern vor dem offenen unteren Streifen und die Blattteppiche haben gezackte Lappenränder."),
        T("Retained: all battens as filled strips, posts, rails, shoes, the four stems as strokes and the leaf mat outlines.", "Beibehalten: alle Latten als gefüllte Streifen, Pfosten, Riegel, Schuhe, die vier Triebe als Striche und die Umrisse der Blattteppiche."),
        T("Merged: 70 to 90 leaves per mat as one outline with leaf-lobe scallops.", "Zusammengefasst: 70 bis 90 Blätter je Teppich als eine Kontur mit Blattlappenzacken."),
        T("Omitted: individual leaf outlines and veins inside the mats, adhesive roots.", "Weggelassen: einzelne Blattumrisse und Adern in den Teppichen, Haftwurzeln."),
        T("Stems (12 mm) are strokes of 0.18 mm; leaf lobes below 12 mm are not separated.", "Triebe (12 mm) sind Striche von 0,18 mm; Blattlappen unter 12 mm werden nicht getrennt."),
        T("None; the battens are filled strips.", "Keine; die Latten sind gefüllte Streifen."),
        T("Timber mid-grey, steel shoes light, leaf mats dark below and lighter above.", "Holz mittelgrau, Stahlschuhe hell, Blattteppiche unten dunkel und oben heller."),
        T("Mats carry their leaf regions; stems carry the stem region; battens, posts and rails the timber region.", "Teppiche tragen ihre Blattregionen; Triebe die Triebregion; Latten, Pfosten und Riegel die Holzregion.")))
    return r


def asset():
    A, em, regions, mats, poses = make()
    sym = {"id": "plan-extent", "title": T("Plan symbol of the screen", "Grundrisssymbol der Wand"), "description": T("A plan sign for landscape plans: the screen footprint of 2.0 m by 0.25 m with the two posts; a separate symbol, never used silently for a pictorial scale.", "Ein Grundrisszeichen für Freiraumpläne: der Wandgrundriss von 2,0 m mal 0,25 m mit den zwei Pfosten; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."), "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": {"elevation": [], "plan": plan_symbol()}, "dimensionalNote": T("The rectangle is the true footprint of the screen with ivy; the squares are the 90 mm posts.", "Das Rechteck ist der wahre Grundriss der berankten Wand; die Quadrate sind die Pfosten von 90 mm.")}
    a = {"id": ID, "category": "vegetation", "rig": {"kind": "botanical", "root": "root-plate", "bones": B.list}, "anchors": A,
         "title": T("Ivy on Trellis", "Efeu am Spalier"),
         "summary": T("English ivy (Hedera helix) climbing a freestanding timber trellis, 1.9 m wide and 2.15 m high, on two timber posts in galvanised steel shoes: four main stems carry leaf mats that cover the upper lattice and shoots that overhang the top rail. Trellis and plant are one asset.", "Gemeiner Efeu (Hedera helix) klettert an einem freistehenden Holzspalier von 1,9 m Breite und 2,15 m Höhe auf zwei Holzpfosten in verzinkten Stahlschuhen: vier Haupttriebe tragen Blattteppiche, die das obere Gitter decken, und Triebe, die über den oberen Riegel hängen. Spalier und Pflanze sind ein Asset."),
         "tags": ["ivy", "climber", "trellis", "timber", "steel", "evergreen", "wind", "transpiration"],
         "provenance": provenance(["metcalfe-2005", "jones-2014", "monteith-unsworth-2013", "en-338", "en-10025-2", "en-1991-1-4", "en-1991-1-3"], T("Four stems with a lower and an upper leaf-mat bone each and two overhanging shoot bones sit on a fixed trellis (two posts, panel). Leaf masses, areas and conductances are authored estimates guided by Metcalfe (2005) and Jones (2014); the timber and steel properties are from EN 338 and EN 10025-2.", "Vier Triebe mit je einem unteren und einem oberen Blattteppich-Knochen und zwei überhängende Triebknochen sitzen auf einem festen Spalier (zwei Pfosten, Gitterfeld). Blattmassen, Flächen und Leitfähigkeiten sind Autorenschätzungen angelehnt an Metcalfe (2005) und Jones (2014); die Holz- und Stahleigenschaften stammen aus EN 338 und EN 10025-2.")),
         "dimensions": {"heightM": 2.11, "widthM": 1.91, "depthM": 0.25, "massKg": MASS},
         "materials": mats, "representations": representations(), "symbols": [sym], "poses": poses, "clips": clips(poses),
         "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 23, "relativeHumidity": 0.6, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 650, "diffuseFraction": 0.18, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": regions, "emitters": em,
                     "assumptions": [T("Leaf area index is referred to the trellis face (3.06 m2), not the ground: 5.3 upper and 4.7 lower. Maximum stomatal conductances (0.003 and 0.0025 m/s) are authored estimates for an evergreen C3 climber (Jones 2014). Leaves are leathery, 0.4 mm; areal heat capacity 1.2 kJ/(m2 K).", "Der Blattflächenindex bezieht sich auf die Spalierfläche (3,06 m2), nicht auf den Boden: 5,3 oben und 4,7 unten. Maximale Spaltöffnungsleitfähigkeiten (0,003 und 0,0025 m/s) sind Autorenschätzungen für einen immergrünen C3-Kletterer (Jones 2014). Die Blätter sind ledrig, 0,4 mm; flächenbezogene Wärmekapazität 1,2 kJ/(m2 K)."),
                                     T("Timber and steel are passive regions: timber 7 kJ/(m2 K) (10 mm effective depth), steel 10.5 kJ/(m2 K) (3 mm). Emitters release the latent flux of their leaf region over six and four anchors on the leaf mats and shoots.", "Holz und Stahl sind passive Regionen: Holz 7 kJ/(m2 K) (10 mm wirksame Tiefe), Stahl 10,5 kJ/(m2 K) (3 mm). Die Emitter geben den latenten Strom ihrer Blattregion über sechs und vier Anker auf den Blattteppichen und Trieben ab.")]},
         "structure": structure(["base-support-L", "base-support-R"], "One rigid body per evaluated pose, both posts fixed at their steel shoes; wind drag acts on every leaf mat (0.35 m2), the shoots and the lattice panel (cf 1.2 to 1.3 for a nearly closed porous screen, EN 1991-1-4 form). Posts are C24 timber 90 x 90 mm. Dynamic amplification is not modelled.", "Ein starrer Körper je ausgewerteter Pose, beide Pfosten an den Stahlschuhen eingespannt; Winddruck wirkt auf jeden Blattteppich (0,35 m2), die Triebe und das Gitterfeld (cf 1,2 bis 1,3 für eine fast geschlossene durchlässige Wand, Form nach EN 1991-1-4). Die Pfosten sind Holz C24 90 x 90 mm. Dynamische Überhöhung wird nicht modelliert."),
         "presentation": presentation("summer-calm", "wind-sway", 100), "exports": EXPORTS, "completion": "authored"}
    return a


if __name__ == "__main__":
    doc = document(asset())
    validate_text(doc)
    print(dump(doc, ID), MASS)
