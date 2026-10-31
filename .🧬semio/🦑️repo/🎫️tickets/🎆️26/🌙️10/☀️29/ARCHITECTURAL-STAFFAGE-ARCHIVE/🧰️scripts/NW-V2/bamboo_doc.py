import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bamboo import *

R100 = (76, 150)


def anchors_and_emitters():
    A = [anchors_support("root-plate", T("Rhizome plate inside the root barrier", "Rhizomplatte in der Wurzelsperre"), 1.6, 1.0)]
    sun_ids, shade_ids = [], []
    for c in culms:
        n, H = c["n"], c["H"]
        for tag, z, side_off, lst in (("sun", 0.92, 0.0, sun_ids), ("shade", 0.66, 0.0, shade_ids)):
            p, a = ctl(c, z)
            sd = 1 if c["lean"] > 0.1 else -1
            q = (p[0] + sd * (0.10 if tag == "shade" else 0.05), p[1])
            lx, ly = B.anchor_local(f"fol-{n}", q[0], q[1])
            aid = f"tp-{tag}-{n}"
            A.append(emitter_anchor(aid, f"fol-{n}", lx, ly, T(f"Transpiration of the {'sunlit' if tag == 'sun' else 'shaded'} foliage of culm {n}", f"Transpiration des {'besonnten' if tag == 'sun' else 'beschatteten'} Laubs von Halm {n}")))
            lst.append(aid)
    return A, sun_ids, shade_ids


def make():
    A, sun_ids, shade_ids = anchors_and_emitters()
    up = up_in_bone(B, "fol-1")
    emitters = [
        transpiration("transpiration-sun", T("Transpiration of the sunlit foliage (distributed over the six culms)", "Transpiration des besonnten Laubs (über die sechs Halme verteilt)"), sun_ids, "leaves-sun", up, 1.2),
        transpiration("transpiration-shade", T("Transpiration of the shaded foliage (distributed over the six culms)", "Transpiration des beschatteten Laubs (über die sechs Halme verteilt)"), shade_ids, "leaves-shade", up, 1.4),
    ]
    regions = [
        passive_region("culms", T("Culms", "Halme"), "stem", "bamboo-culm", 1.6, 0.024, "cylinder", 0.45, 7000, "liese-1985"),
        leaf_region("leaves-sun", T("Sunlit foliage", "Besonntes Laub"), "bamboo-leaf", 5.4, 0.02, "vertical-plate", 0.7, 600, 0.0075, 1.6, "c3", "jones-2014"),
        leaf_region("leaves-shade", T("Shaded inner foliage", "Beschattetes Innenlaub"), "bamboo-leaf", 8.2, 0.02, "vertical-plate", 0.35, 650, 0.0055, 2.4, "c3", "jones-2014"),
    ]
    mats = [
        {"id": "bamboo-culm", "title": T("Bamboo culm (hollow, wall 3.5 to 4 mm)", "Bambushalm (hohl, Wand 3,5 bis 4 mm)"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 700, "conductivityWmK": 0.17, "specificHeatJKgK": 2000, "elasticModulusPa": 12000000000, "strengthPa": 80000000, "source": "authored-estimate", "reference": "liese-1985"},
        {"id": "bamboo-leaf", "title": T("Bamboo leaf blade", "Bambusblatt"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
    ]
    def thermal_pose(sunlit, wind_exposed, emit, wet=None):
        return bind({"culms": (1.0, sunlit[0], wet), "leaves-sun": (1.0, sunlit[1], wet), "leaves-shade": (wind_exposed, sunlit[2], wet)}, {"transpiration-sun": emit[0], "transpiration-shade": emit[1]})
    def winds(v, cf_sun, area_k, tag):
        return [wind_load(f"wind-fol-{c['n']}", T(f"Wind drag on the foliage of culm {c['n']}, {tag}", f"Winddruck auf das Laub von Halm {c['n']}, {tag}"), f"fol-{c['n']}", 0.0, 0.0, cf_sun, area_k * c["H"] ** 2, v) for c in culms]
    def lean(a, b, cc, f=0, sgn=None):
        out = {}
        for c in culms:
            s = 1 if sgn is None else (1 if c["lean"] < 0.1 else -1)
            n = c["n"]
            out[f"culm-{n}-a"] = {"angleDeg": r4(a * s)}
            out[f"culm-{n}-b"] = {"angleDeg": r4(b * s)}
            out[f"culm-{n}-c"] = {"angleDeg": r4(cc * s)}
            if f:
                out[f"fol-{n}"] = {"angleDeg": r4(f * s)}
        return out
    snow = [vertical_load(f"snow-fol-{c['n']}", "snow", T(f"Retained wet snow on the foliage of culm {c['n']}", f"Haftender Nassschnee auf dem Laub von Halm {c['n']}"), f"fol-{c['n']}", 0.0, 0.0, -9.81 * 3.0 * 0.12 * c["H"], "en-1991-1-3") for c in culms]
    poses = [
        pose("summer-calm", T("Summer, calm", "Sommer, windstill"), T("Light air of 1.5 m/s: the six culms stand in their rest curves, the foliage hangs open and the whole clump transpires at the full midday rate.", "Leiser Zug von 1,5 m/s: Die sechs Halme stehen in ihrer Ruhekrümmung, das Laub hängt offen und der ganze Horst transpiriert mit der vollen Mittagsrate."), {}, thermal_pose((0.35, 0.85, 0.12), 0.8, (0.8, 0.7)), winds(1.5, 1.0, 0.03, "calm")),
        pose("fresh-breeze", T("Fresh breeze", "Frische Brise"), T("A fresh breeze of 8 m/s from the left bends all culms downwind; the foliage streams and exposes its inner leaves to the air.", "Eine frische Brise von 8 m/s von links biegt alle Halme in Windrichtung; das Laub weht und setzt die inneren Blätter der Luft aus."), lean(-2, -3.5, -5, 0), thermal_pose((0.35, 0.85, 0.2), 1.0, (1.0, 1.0)), winds(8, 0.5, 0.03, "fresh breeze")),
        pose("storm-bent", T("Storm, culms bent", "Sturm, Halme gebogen"), T("A gale of 22 m/s: the culms bend to about a quarter of their height downwind and the foliage reconfigures into streamlined bundles that cut the drag coefficient.", "Ein Sturm von 22 m/s: Die Halme biegen sich bis etwa ein Viertel der Höhe in Windrichtung und das Laub ordnet sich zu strömungsgünstigen Bündeln, die den Widerstandsbeiwert senken."), lean(-4, -7, -11, -5), thermal_pose((0.2, 0.5, 0.05), 1.0, (0.5, 0.3)), winds(22, 0.3, 0.02, "storm")),
        pose("wet-snow", T("Winter, wet snow", "Winter, Nassschnee"), T("Wet snow on the evergreen foliage bows the culm tops outward and down; the leaves curl, stomata are shut and the clump hardly transpires.", "Nassschnee auf dem immergrünen Laub drückt die Halmspitzen nach außen und unten; die Blätter rollen sich, die Spaltöffnungen sind geschlossen und der Horst transpiriert kaum."), lean(1.5, 3, 8, 6, sgn=True), thermal_pose((0.2, 0.2, 0.05), 0.6, (0.03, 0.02), wet=0.5), snow),
        pose("winter-overcast", T("Winter, overcast and still", "Winter, bedeckt und windstill"), T("A still, overcast winter day: no direct sun, leaves slightly rolled, a few percent of the summer transpiration.", "Ein windstiller, bedeckter Wintertag: keine direkte Sonne, leicht eingerollte Blätter, wenige Prozent der sommerlichen Transpiration."), {}, thermal_pose((0.0, 0.0, 0.0), 0.7, (0.1, 0.06)), winds(1.0, 1.0, 0.03, "winter calm")),
    ]
    return A, emitters, regions, mats, poses


def clips(poses):
    names = lambda n: [f"culm-{n}-a", f"culm-{n}-b", f"culm-{n}-c", f"fol-{n}"]
    base = {}
    for p in poses:
        if p["id"] == "fresh-breeze":
            base = {k: v["angleDeg"] for k, v in p["bones"].items()}
    specs = {}
    for c in culms:
        n = c["n"]
        ph = 38 * n
        specs[f"culm-{n}-a"] = (0.9, ph, 1)
        specs[f"culm-{n}-b"] = (1.8, ph + 12, 1)
        specs[f"culm-{n}-c"] = (3.2, ph + 25, 1)
        specs[f"fol-{n}"] = (5.0, ph + 60, 2)
    sway = sway_clip(B.list, "wind-sway", T("Wind sway", "Windschwingung"), T("A deterministic 2.9 s cycle (0.34 Hz, the first bending mode of a 4.5 m culm with its foliage, estimated from E I = 300 N m2 and 2 kg tip mass): the culms swing about the fresh-breeze lean with a phase that travels through the clump; the foliage flutters at twice the frequency.", "Ein deterministischer Zyklus von 2,9 s (0,34 Hz, die erste Biegeeigenform eines 4,5 m hohen Halms mit Laub, geschätzt aus E I = 300 N m2 und 2 kg Spitzenmasse): Die Halme schwingen um die Neigung der frischen Brise mit einer durch den Horst wandernden Phase; das Laub flattert mit doppelter Frequenz."), 2.9, base, specs)
    for k in sway["keys"]:
        k["pose"] = "fresh-breeze"
    sway["tracks"] = [{"target": "emitter:transpiration-sun", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 1.45, "v": 0.9}, {"t": 2.9, "v": 1.0}]}]
    def flutter(t, amp, ph):
        return {f"fol-{c['n']}": {"angleDeg": r4(clamp(amp * sin(radians(ph + 50 * c["n"] + 360 * t / 0.9)) + base_of(pid, f"fol-{c['n']}"), -14, 14))} for c in culms}
    def base_of(pid, b):
        for p in poses:
            if p["id"] == pid:
                return p["bones"].get(b, {}).get("angleDeg", 0)
        return 0
    keys = []
    for t, pid, amp in ((0, "summer-calm", 0.5), (1.2, "fresh-breeze", 3), (2.4, "storm-bent", 6), (3.4, "fresh-breeze", 4), (5.0, "summer-calm", 1.5), (6.5, "summer-calm", 0.5)):
        keys.append({"t": t, "pose": pid, "ease": "smooth", "bones": flutter(t, amp, 0) if False else {f"fol-{c['n']}": {"angleDeg": r4(clamp(amp * sin(radians(60 * c["n"] + 90 * t)) + base_of(pid, f"fol-{c['n']}"), -14, 14))} for c in culms}})
    gust = {"id": "gust", "title": T("Gust of wind", "Windböe"), "description": T("A 6.5 s gust: calm air, a fresh breeze, a storm peak at 2.4 s with streamlined foliage and flutter growing with the speed, then the decay back to calm. Transpiration rises with the breeze and the stomata close at the peak.", "Eine Böe von 6,5 s: Windstille, frische Brise, ein Sturmspitzenwert bei 2,4 s mit strömungsgünstigem Laub und mit der Geschwindigkeit wachsendem Flattern, dann das Abklingen zur Stille. Die Transpiration steigt mit der Brise, am Höhepunkt schließen sich die Spaltöffnungen."), "durationS": 6.5, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "rise", "fromS": 0, "toS": 2.4, "title": T("Gust builds up", "Böe baut sich auf")}, {"id": "decay", "fromS": 2.4, "toS": 6.5, "title": T("Gust decays", "Böe klingt ab")}], "events": [{"t": 2.4, "kind": "motion", "title": T("Gust peak, culms bent furthest", "Böenspitze, Halme am stärksten gebogen")}],
            "tracks": [{"target": "emitter:transpiration-sun", "ease": "smooth", "keys": [{"t": 0, "v": 0.8}, {"t": 1.2, "v": 1.0}, {"t": 2.4, "v": 0.5}, {"t": 5.0, "v": 0.8}, {"t": 6.5, "v": 0.8}]}, {"target": "emitter:transpiration-shade", "ease": "smooth", "keys": [{"t": 0, "v": 0.7}, {"t": 1.2, "v": 1.0}, {"t": 2.4, "v": 0.3}, {"t": 5.0, "v": 0.7}, {"t": 6.5, "v": 0.7}]}]}
    return [sway, gust]


def dec(name_en, name_de, **kw):
    return decisions(*[T(*kw[k]) for k in ("sil", "ret", "mer", "omi", "thin", "hat", "sty", "ove")])


def representations():
    r = {}
    r["500"] = rep(500, 351, 1000, pens(0.15, None, None, None, 0.25), [], build_500(), decisions(
        T("At 1:500 (1 m is 2 mm) the clump is composed of six spindle-shaped crowns, each tapering up into its culm tip and leaning with its culm, standing on three narrow cane wedges; the tallest crown reaches the real 4.6 m.", "Bei 1:500 (1 m sind 2 mm) ist der Horst aus sechs spindelförmigen Kronen komponiert, die sich je zur Halmspitze verjüngen und mit ihrem Halm neigen, und die auf drei schmalen Halmkeilen stehen; die höchste Krone erreicht die wahren 4,6 m."),
        T("Retained: the staggered crown heights (2.4 to 4.6 m), the lean of the outer culms, the open base with cane wedges and the overall width of 3.0 m.", "Beibehalten: die gestaffelten Kronenhöhen (2,4 bis 4,6 m), die Neigung der äußeren Halme, der offene Fuß mit Halmkeilen und die Gesamtbreite von 3,0 m."),
        T("Merged: sunlit and shaded foliage of each culm form one crown (both regions listed); the six culms are merged into three wedges whose gaps are narrower than the profile gap.", "Zusammengefasst: besonntes und beschattetes Laub jedes Halms bilden eine Krone (beide Regionen aufgeführt); die sechs Halme sind zu drei Keilen zusammengefasst, deren Lücken schmaler als der Profilabstand sind."),
        T("Omitted: nodes, branches, single leaves, sheaths and the groove of the culm.", "Weggelassen: Knoten, Äste, Einzelblätter, Scheiden und die Rille des Halms."),
        T("A culm of 29 mm would be 0.06 mm wide, below the pen: culms are not drawn individually but merged into wedges of 0.3 to 0.4 m width, which is wider than the real cane spacing and says so as a mass, not as a cane.", "Ein Halm von 29 mm wäre 0,06 mm breit, unter der Stiftbreite: Halme werden nicht einzeln gezeichnet, sondern zu Keilen von 0,3 bis 0,4 m Breite zusammengefasst, die breiter sind als der wahre Halmabstand und als Masse, nicht als Halm lesen."),
        T("None: surface tone only.", "Keine: nur Flächentönung."),
        T("Crowns mid grey, cane wedges light grey, one outline pen of 0.15 mm; overlaps are intentional and read as depth.", "Kronen mittelgrau, Halmkeile hellgrau, ein Konturstift von 0,15 mm; Überlagerungen sind beabsichtigt und lesen als Tiefe."),
        T("The thermal view paints each crown with the area-weighted value of sunlit and shaded foliage and the wedges with the culm value.", "Die Thermalansicht färbt jede Krone mit dem flächengewichteten Wert von besonntem und beschattetem Laub und die Keile mit dem Halmwert.")))
    r["200"] = rep(200, 151, 350, pens(0.18, 0.13, None, None, 0.25), [], build_200(), decisions(
        T("At 1:200 (1 m is 5 mm) each culm is a single line from the rhizome plate to the tip, carrying two serrated foliage masses: a lower, wider shaded mass leaning outward and an upper, narrower sunlit mass around the tip.", "Bei 1:200 (1 m sind 5 mm) ist jeder Halm eine einzelne Linie von der Rhizomplatte zur Spitze mit zwei gezackten Laubmassen: eine untere, breitere beschattete Masse nach außen geneigt und eine obere, schmalere besonnte Masse um die Spitze."),
        T("Retained: six culms with their real lean and joints, two foliage masses per culm separating sun and shade foliage.", "Beibehalten: sechs Halme mit wahrer Neigung und Gelenken, je zwei Laubmassen pro Halm, die Sonnen- und Schattenlaub trennen."),
        T("Merged: all branches and leaves of a zone are one serrated mass.", "Zusammengefasst: alle Äste und Blätter einer Zone bilden eine gezackte Masse."),
        T("Omitted: nodes, groove, branches, single leaves.", "Weggelassen: Knoten, Rille, Äste, Einzelblätter."),
        T("The 29 mm culm is 0.15 mm wide at this scale: it is drawn as an open line with the 0.18 mm pen, a thickening of 7 mm in real size, within the outline tolerance; it is not a filled shape.", "Der Halm von 29 mm ist bei diesem Maßstab 0,15 mm breit: Er wird als offene Linie mit dem Stift 0,18 mm gezeichnet, eine Verdickung von 7 mm in Wirklichkeit, innerhalb der Konturtoleranz; er ist keine gefüllte Form."),
        T("None.", "Keine."),
        T("Culm lines in the body pen, masses filled dark (shade) and mid (sun); light foliage prints lighter.", "Halmlinien im Körperstift, Massen dunkel (Schatten) und mittel (Sonne) gefüllt; helles Laub druckt heller."),
        T("Each foliage mass maps to exactly one thermal region; the culm lines carry the culm value as stroke colour.", "Jede Laubmasse entspricht genau einer thermischen Region; die Halmlinien tragen den Halmwert als Strichfarbe.")))
    r["100"] = rep(100, 76, 150, pens(0.2, 0.13, 0.1, None, 0.3), [], build_100(), decisions(
        T("At 1:100 (1 m is 10 mm) the culms are tapered filled outlines with nodes and every culm carries four to five serrated leaf bunches hanging from its upper branches, sunlit above and shaded below.", "Bei 1:100 (1 m sind 10 mm) sind die Halme verjüngte gefüllte Konturen mit Knoten und jeder Halm trägt vier bis fünf gezackte Blattbüschel an den oberen Ästen, oben besonnt und unten beschattet."),
        T("Retained: culm taper and joints, the lowest four nodes, the hanging leaf bunches and the open crown between them.", "Beibehalten: Halmverjüngung und Gelenke, die untersten vier Knoten, die hängenden Blattbüschel und die offene Krone dazwischen."),
        T("Merged: all leaves of a branch form one serrated bunch; the twigs are not drawn.", "Zusammengefasst: alle Blätter eines Astes bilden ein gezacktes Büschel; die Zweige werden nicht gezeichnet."),
        T("Omitted: groove, sheath scars, branches and single leaves.", "Weggelassen: Rille, Scheidennarben, Äste und Einzelblätter."),
        T("The upper culm (12 to 15 mm) would be 0.12 to 0.15 mm wide; it is thickened to 22 mm (0.22 mm) so that it keeps the minimum feature, a thickening of at most 10 mm.", "Der obere Halm (12 bis 15 mm) wäre 0,12 bis 0,15 mm breit; er wird auf 22 mm (0,22 mm) verdickt, damit er das Mindestmerkmal einhält, eine Verdickung von höchstens 10 mm."),
        T("None; nodes are single texture lines.", "Keine; Knoten sind einzelne Texturlinien."),
        T("Culms light grey, shaded bunches dark, sunlit bunches mid grey, nodes in the 0.1 mm texture pen.", "Halme hellgrau, beschattete Büschel dunkel, besonnte Büschel mittelgrau, Knoten im Texturstift 0,1 mm."),
        T("Every bunch is one thermal region; the culm outlines map to the culm region.", "Jedes Büschel entspricht einer thermischen Region; die Halmkonturen entsprechen der Halmregion.")))
    r["50"] = rep(50, 20, 75, pens(0.25, 0.18, 0.13, None, 0.35), [], build_50(), decisions(
        T("At 1:50 (1 m is 20 mm) every culm is drawn with its taper, nodes and groove, carrying leaf-bearing branches in the upper half; each branch ends in drooping fans of lanceolate leaves, three fans per branch.", "Bei 1:50 (1 m sind 20 mm) wird jeder Halm mit Verjüngung, Knoten und Rille gezeichnet und trägt in der oberen Hälfte beblätterte Äste; jeder Ast endet in hängenden Fächern lanzettlicher Blätter, drei Fächer pro Ast."),
        T("Retained: culm tapers and node rings, the groove of Phyllostachys aureosulcata, two to three alternating branches per node zone, fans of four to six leaves.", "Beibehalten: Halmverjüngungen und Knotenringe, die Rille von Phyllostachys aureosulcata, zwei bis drei wechselständige Äste je Knotenzone, Fächer aus vier bis sechs Blättern."),
        T("Merged: leaves of one fan are one outline with notches between the leaf tips.", "Zusammengefasst: die Blätter eines Fächers bilden eine Kontur mit Einkerbungen zwischen den Blattspitzen."),
        T("Omitted: sheath scars, twigs below 5 mm and the leaf midribs.", "Weggelassen: Scheidennarben, Zweige unter 5 mm und die Blattmittelrippen."),
        T("Branches (about 5 mm) are open lines in the 0.18 mm pen; leaves 14 to 17 mm wide are filled and stay above 0.3 mm width.", "Äste (etwa 5 mm) sind offene Linien im Stift 0,18 mm; Blätter von 14 bis 17 mm Breite sind gefüllt und bleiben über 0,3 mm Breite."),
        T("None; the groove and node lines are texture and detail strokes.", "Keine; Rille und Knotenlinien sind Textur- und Detailstriche."),
        T("Culm outlines light, sunlit fans mid grey, shaded fans dark grey.", "Halmkonturen hell, besonnte Fächer mittelgrau, beschattete Fächer dunkelgrau."),
        T("Every fan carries its foliage region; branches and node lines stay neutral.", "Jeder Fächer trägt seine Laubregion; Äste und Knotenlinien bleiben neutral.")))
    return r


def asset():
    A, emitters, regions, mats, poses = make()
    sym = {"id": "plan-extent", "title": T("Plan symbol with true clump extent", "Grundrisssymbol mit wahrer Horstausdehnung"), "description": T("Plan sign for landscape plans: the true foliage extent (3.1 m by 1.7 m) with the six culm sections; a separate symbol, never used silently for a pictorial scale.", "Grundrisszeichen für Freiraumpläne: die wahre Laubausdehnung (3,1 m mal 1,7 m) mit den sechs Halmquerschnitten; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."), "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": {"elevation": [], "plan": plan_symbol()}, "dimensionalNote": T("The outline is the true foliage extent in plan; the dots are the true culm positions, 29 mm in diameter.", "Der Umriss ist die wahre Laubausdehnung im Grundriss; die Punkte sind die wahren Halmpositionen mit 29 mm Durchmesser.")}
    cl = clips(poses)
    a = {"id": ID, "category": "vegetation", "rig": {"kind": "botanical", "root": "root-plate", "bones": B.list}, "anchors": A,
         "title": T("Bamboo Clump", "Bambushorst"),
         "summary": T("A clump of six running-type bamboo culms (Phyllostachys aureosulcata, 29 mm culms, grown in a rhizome barrier) from 2.4 to 4.6 m tall: nodes, a groove along the culm and leaf-bearing branches in the upper half that stream in the wind.", "Ein Horst aus sechs Bambushalmen (Phyllostachys aureosulcata, Halme von 29 mm, in einer Rhizomsperre gezogen) von 2,4 bis 4,6 m Höhe: Knoten, eine Rille entlang des Halms und beblätterte Äste in der oberen Hälfte, die im Wind wehen."),
         "tags": ["bamboo", "grass", "evergreen", "culms", "nodes", "wind", "transpiration", "c3"],
         "provenance": provenance(["liese-1985", "liese-koehl-2015", "jones-2014", "monteith-unsworth-2013", "en-1991-1-4", "en-1991-1-3", "vogel-1989"], T("Six culms, each with a lower, middle and arching upper bone and a foliage bone, are authored by hand; geometry per scale is composed separately. Culm dimensions, leaf area and conductance are authored estimates guided by the cited literature; no number is claimed as measured.", "Sechs Halme mit je einem unteren, mittleren und bogigen oberen Knochen und einem Laubknochen sind von Hand entworfen; die Geometrie jedes Maßstabs ist eigens komponiert. Halmmaße, Blattfläche und Leitfähigkeit sind Autorenschätzungen angelehnt an die genannte Literatur; keine Zahl wird als gemessen ausgegeben.")),
         "dimensions": {"heightM": 4.55, "widthM": 3.02, "depthM": 1.7, "massKg": MASS},
         "materials": mats, "representations": representations(), "symbols": [sym], "poses": poses, "clips": cl,
         "thermal": {"environment": {"airTempC": 22, "meanRadiantTempC": 25, "relativeHumidity": 0.55, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 700, "diffuseFraction": 0.15, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": regions, "emitters": emitters,
                     "assumptions": [T("Leaf regions use one-sided leaf area; total leaf area index 4.0 over the 3.4 m2 crown projection, split into sunlit outer (1.6) and shaded inner foliage (2.4). Maximum stomatal conductances (0.0075 and 0.0055 m/s, about 0.3 and 0.22 mol/(m2 s)) are authored estimates for an evergreen C3 species guided by Jones (2014). Aperture and night closure are the engine's business.", "Blattregionen verwenden die einseitige Blattfläche; der gesamte Blattflächenindex 4,0 über der Kronenprojektion von 3,4 m2 teilt sich in besonntes Außenlaub (1,6) und beschattetes Innenlaub (2,4). Maximale Spaltöffnungsleitfähigkeiten (0,0075 und 0,0055 m/s, etwa 0,3 und 0,22 mol/(m2 s)) sind Autorenschätzungen für eine immergrüne C3-Art, angelehnt an Jones (2014). Öffnung und nächtliches Schließen sind Sache der Engine."),
                                     T("Culms are passive; their areal heat capacity of 7 kJ/(m2 K) is a hollow wall of 4 mm at 700 kg/m3 and 2 kJ/(kg K), an authored estimate. Transpiration emitters release the latent flux of their foliage region over six anchors, one per culm; the direction is the vertical in the frame of the first anchor's bone.", "Halme sind passiv; ihre flächenbezogene Wärmekapazität von 7 kJ/(m2 K) ist eine hohle Wand von 4 mm bei 700 kg/m3 und 2 kJ/(kg K), eine Autorenschätzung. Transpirationsemitter geben den latenten Strom ihrer Laubregion über sechs Anker ab, einen je Halm; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers.")]},
         "structure": structure(["base-support"], "One rigid body per evaluated pose, fixed at the rhizome plate; wind drag acts on each foliage bone with frontal area 0.03 H2 and cf 0.5 (breeze), 0.3 (storm, reconfiguration after Vogel 1989). Hollow culm sections have the stiffness of green bamboo (authored estimate). Dynamic amplification and rhizome pull-out are not modelled.", "Ein starrer Körper je ausgewerteter Pose, an der Rhizomplatte eingespannt; Winddruck wirkt auf jeden Laubknochen mit Stirnfläche 0,03 H2 und cf 0,5 (Brise), 0,3 (Sturm, Umordnung nach Vogel 1989). Hohle Halmquerschnitte haben die Steifigkeit von grünem Bambus (Schätzung). Dynamische Überhöhung und Ausreißen des Rhizoms werden nicht modelliert."),
         "presentation": presentation("summer-calm", "wind-sway", 100), "exports": EXPORTS, "completion": "authored"}
    return a


if __name__ == "__main__":
    doc = document(asset())
    validate_text(doc)
    print(dump(doc, ID), MASS)
