import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from agave import *

LEAFB = [b for n in LEAVES for b in LEAVES[n]["bones"]]
FIRST = {n: LEAVES[n]["bones"][0] for n in LEAVES}
RIGHT = [n for n in LEAVES if n.startswith("R")]
LEFT = [n for n in LEAVES if n.startswith("L")]


def side_sign(bone):
    n = bone.split("-")[1]
    return 1 if n.startswith("R") or n == "S" else -1


def offs(a, b, e=None):
    out = {}
    for n, l in LEAVES.items():
        s = 1 if n.startswith("R") else -1
        for i, bn in enumerate(l["bones"]):
            v = (a if i == 0 else b)
            if n == "S":
                v = 0
            out[bn] = {"angleDeg": r4(v * s)}
    return out


def lean(deg):
    return {bn: {"angleDeg": r4(deg)} for bn in LEAFB if bn != "leaf-S"} | {"leaf-S": {"angleDeg": r4(deg)}}


def anchors_emitters():
    A = [anchors_support("root-plate", T("Root plate in the planting bed", "Wurzelplatte im Pflanzbeet"), 1.0, 1.0)]
    sp, er = [], []
    plan = [("R1", "leaf-R1-b", 0.3, sp), ("R2", "leaf-R2-a", 0.5, sp), ("L1", "leaf-L1-b", 0.3, sp), ("L2", "leaf-L2-a", 0.5, sp), ("R1", "leaf-R1-a", 0.5, sp), ("L1", "leaf-L1-a", 0.5, sp),
            ("R3", "leaf-R3-a", 0.5, er), ("R4", "leaf-R4-b", 0.2, er), ("R5", "leaf-R5", 0.6, er), ("L3", "leaf-L3-b", 0.2, er), ("L4", "leaf-L4-a", 0.5, er), ("L5", "leaf-L5", 0.6, er), ("S", "leaf-S", 0.8, er)]
    for i, (n, bone, x, lst) in enumerate(plan):
        aid = f"tp-{'spreading' if lst is sp else 'erect'}-{len(lst) + 1}"
        A.append(emitter_anchor(aid, bone, x, 0.0, T(f"Transpiration of leaf {n}, anchor {len(lst)+1}", f"Transpiration von Blatt {n}, Anker {len(lst)+1}")))
        lst.append(aid)
    return A, sp, er


def make():
    A, sp, er = anchors_emitters()
    up = up_in_bone(B, "leaf-R1-b")
    em = [transpiration("transpiration-spreading", T("Transpiration of the spreading lower leaves (distributed over six anchors)", "Transpiration der ausgebreiteten unteren Blätter (über sechs Anker verteilt)"), sp, "leaves-spreading", up, 1.4),
          transpiration("transpiration-erect", T("Transpiration of the erect inner leaves (distributed over seven anchors)", "Transpiration der aufrechten inneren Blätter (über sieben Anker verteilt)"), er, "leaves-erect", up_in_bone(B, "leaf-R3-a"), 0.8)]
    regions = [
        leaf_region("leaves-spreading", T("Spreading lower leaves", "Ausgebreitete untere Blätter"), "agave-leaf", 2.0, 0.15, "horizontal-plate-up", 0.85, 90000, 0.0012, 0.4, "cam", "nobel-1988"),
        leaf_region("leaves-erect", T("Erect inner leaves and spear", "Aufrechte innere Blätter und Herzblatt"), "agave-leaf", 3.0, 0.15, "vertical-plate", 0.6, 90000, 0.0012, 0.6, "cam", "nobel-1988"),
        passive_region("caudex", T("Caudex", "Caudex"), "stem", "agave-caudex", 0.21, 0.22, "cylinder", 0.5, 60000, "nobel-1988"),
    ]
    mats = [{"id": "agave-leaf", "title": T("Succulent glaucous agave leaf (about 25 mm thick, water-rich)", "Sukkulentes blaugraues Agavenblatt (etwa 25 mm dick, wasserreich)"), "emissivity": 0.96, "solarAbsorptivity": 0.5, "densityKgM3": 950, "specificHeatJKgK": 3800, "source": "authored-estimate", "reference": "nobel-1988"},
            {"id": "agave-caudex", "title": T("Fibrous caudex with dry leaf bases", "Faseriger Caudex mit trockenen Blattbasen"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": 600, "conductivityWmK": 0.15, "specificHeatJKgK": 2500, "elasticModulusPa": 500000000, "strengthPa": 5000000, "source": "authored-estimate", "reference": "nobel-1988"}]
    def tb(spr, ere, cau, emit, wet=None):
        return bind({"leaves-spreading": (1.0, spr, wet), "leaves-erect": (1.0, ere, wet), "caudex": (0.6, cau, wet)}, {"transpiration-spreading": emit[0], "transpiration-erect": emit[1]})
    def winds(v, cf):
        return [wind_load(f"wind-{n}", T(f"Wind drag on leaf group {n}", f"Winddruck auf Blattgruppe {n}"), FIRST[n], 0.45, 0.0, cf, 0.35, v) for n in LEAVES]
    poses = [
        pose("summer-turgid", T("Summer, turgid", "Sommer, prall"), T("Full water balance: the leaves are rigid and fully spread, the lowest leaves arch down towards the bed. Stomata open at night (CAM), so transpiration is bound by the night rate.", "Voller Wasserhaushalt: Die Blätter sind steif und weit gespreizt, die untersten Blätter biegen sich zum Beet hinunter. Die Spaltöffnungen öffnen sich nachts (CAM), die Transpiration folgt daher der Nachtrate."), {}, tb(0.9, 0.8, 0.2, (1.0, 1.0)), winds(1.5, 1.0)),
        pose("night-stomata-open", T("Summer night, stomata open", "Sommernacht, Spaltöffnungen offen"), T("The same turgid rosette on a clear night: no sun, a cold sky and open stomata taking up CO2 by the CAM pathway; the plant transpires at the maximum rate of its day.", "Dieselbe pralle Rosette in einer klaren Nacht: keine Sonne, kalter Himmel und offene Spaltöffnungen, die CO2 über den CAM-Weg aufnehmen; die Pflanze transpiriert mit der größten Rate ihres Tages."), {}, tb(0.0, 0.0, 0.0, (1.0, 1.0)), winds(1.0, 1.0)),
        pose("drought-infolded", T("Drought, leaves infolded", "Trockenheit, Blätter eingefaltet"), T("After weeks without rain the leaves lose turgor and lift and close around the heart to shade it; the plant keeps its stomata nearly shut.", "Nach Wochen ohne Regen verlieren die Blätter Turgor und heben sich, um das Herz zu beschatten; die Pflanze hält die Spaltöffnungen fast geschlossen."), offs(8, 7), tb(0.7, 0.5, 0.2, (0.15, 0.15)), []),
        pose("frost-sag", T("Winter, frost-sagged", "Winter, frostschlaff"), T("After a hard frost the water-rich leaves sag: the spreading leaves droop to the ground, the erect leaves lean outward; metabolism is almost stopped.", "Nach starkem Frost erschlaffen die wasserreichen Blätter: Die ausgebreiteten Blätter hängen zum Boden, die aufrechten neigen sich nach außen; der Stoffwechsel ruht fast."), offs(-6, -8), tb(0.4, 0.4, 0.2, (0.05, 0.05), wet=0.1), []),
        pose("fresh-breeze", T("Fresh breeze", "Frische Brise"), T("A fresh breeze of 10 m/s from the left: the stiff leaves lean about 2 degrees downwind; the rosette hardly moves.", "Eine frische Brise von 10 m/s von links: Die steifen Blätter neigen sich etwa 2 Grad in Windrichtung; die Rosette bewegt sich kaum."), lean(-2), tb(0.9, 0.8, 0.2, (1.0, 1.0)), winds(10, 0.9)),
    ]
    return A, em, regions, mats, poses


def clips(poses):
    base = {k: v["angleDeg"] for k, v in [p for p in poses if p["id"] == "fresh-breeze"][0]["bones"].items()}
    specs = {}
    for i, n in enumerate(LEAVES):
        for j, bn in enumerate(LEAVES[n]["bones"]):
            specs[bn] = (0.7 if j == 0 else 1.5, 30 * i + 20 * j, 1)
    sway = sway_clip(B.list, "wind-sway", T("Wind sway", "Windschwingung"), T("A deterministic 0.95 s cycle (about 1.05 Hz, first bending mode of a stiff succulent leaf of 1.2 m with E I about 9 N m2, estimated): each leaf nods a degree or two about the breeze lean, the phase running around the rosette. The amplitude stays small because the leaves are stiff.", "Ein deterministischer Zyklus von 0,95 s (etwa 1,05 Hz, erste Biegeeigenform eines steifen sukkulenten Blattes von 1,2 m mit E I etwa 9 N m2, geschätzt): Jedes Blatt nickt ein bis zwei Grad um die Neigung der Brise, die Phase läuft um die Rosette. Die Amplitude bleibt klein, weil die Blätter steif sind."), 0.95, base, specs)
    for k in sway["keys"]:
        k["pose"] = "fresh-breeze"
    def bo(pid):
        return [p for p in poses if p["id"] == pid][0]["bones"]
    keys = []
    for t, pid, amp in ((0, "summer-turgid", 0.2), (1.0, "fresh-breeze", 1.0), (2.0, "fresh-breeze", 2.5), (3.2, "fresh-breeze", 1.2), (5.0, "summer-turgid", 0.3)):
        ov = {}
        for i, bn in enumerate(LEAFB):
            ov[bn] = {"angleDeg": r4(clamp(bo(pid).get(bn, {}).get("angleDeg", 0) + amp * sin(radians(40 * i + 120 * t)), -8, 8))}
        keys.append({"t": t, "pose": pid, "ease": "smooth", "bones": ov})
    gust = {"id": "gust", "title": T("Gust of wind", "Windböe"), "description": T("A 5 s gust: the rosette leans 2 degrees and the stiff leaves ring at about 1 Hz with growing amplitude before settling.", "Eine Böe von 5 s: Die Rosette neigt sich 2 Grad und die steifen Blätter schwingen mit etwa 1 Hz bei wachsender Amplitude, bevor sie sich beruhigen."), "durationS": 5.0, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "gust", "fromS": 0, "toS": 3.2, "title": T("Gust", "Böe")}, {"id": "settle", "fromS": 3.2, "toS": 5.0, "title": T("Settling", "Abklingen")}], "events": [{"t": 2.0, "kind": "motion", "title": T("Peak lean", "Größte Neigung")}], "tracks": [{"target": "emitter:transpiration-erect", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 2.0, "v": 1.0}, {"t": 5.0, "v": 1.0}]}]}
    return [sway, gust]


def representations():
    r = {}
    r["500"] = rep(500, 351, 1000, pens(0.15, None, None, None, 0.25), [], build_500(), decisions(
        T("At 1:500 (1 m is 2 mm) the rosette is two spiky half-rosettes of merged leaves and the erect heart leaf on a short trunk: 5.1 mm wide and 2.9 mm high.", "Bei 1:500 (1 m sind 2 mm) ist die Rosette zwei spitze Rosettenhälften aus verschmolzenen Blättern und das aufrechte Herzblatt auf einem kurzen Stamm: 5,1 mm breit und 2,9 mm hoch."),
        T("Retained: the fan of leaf tips, the drooping lowest leaves reaching the bed, the erect spear and the exact width and height.", "Beibehalten: der Fächer der Blattspitzen, die hängenden untersten Blätter bis zum Beet, das aufrechte Herzblatt und die genaue Breite und Höhe."),
        T("Merged: the five leaves of each side are merged, leaves widened to 0.34 m so that gaps narrower than the profile gap close; spreading and erect leaves share one outline.", "Zusammengefasst: Die fünf Blätter jeder Seite sind verschmolzen, die Blätter auf 0,34 m verbreitert, damit Lücken unter dem Profilabstand schließen; ausgebreitete und aufrechte Blätter teilen eine Kontur."),
        T("Omitted: leaf joints, marginal teeth, midribs and leaf scars on the caudex.", "Weggelassen: Blattgelenke, Randzähne, Mittelrippen und Blattnarben am Caudex."),
        T("The terminal spines (4 cm) are below the pen and fall into the pointed leaf tips.", "Die Endstacheln (4 cm) liegen unter der Stiftbreite und gehen in den spitzen Blattspitzen auf."),
        T("None.", "Keine."),
        T("Mid grey filled silhouette in one 0.15 mm outline; the trunk is lighter.", "Mittelgraue gefüllte Silhouette in einer Kontur von 0,15 mm; der Stamm ist heller."),
        T("The thermal view paints each half-rosette with the area-weighted value of spreading and erect leaves; the trunk has its own value.", "Die Thermalansicht färbt jede Rosettenhälfte mit dem flächengewichteten Wert der ausgebreiteten und aufrechten Blätter; der Stamm hat seinen eigenen Wert.")))
    r["200"] = rep(200, 151, 350, pens(0.18, 0.13, None, None, 0.25), [], build_200(), decisions(
        T("At 1:200 (1 m is 5 mm) eleven leaves fan from the heart as tapered blades, the lowest ones bent by their arching tips; the trunk is a short block.", "Bei 1:200 (1 m sind 5 mm) fächern elf Blätter als verjüngte Klingen aus dem Herzen, die untersten durch ihre bogigen Spitzen geknickt; der Stamm ist ein kurzer Block."),
        T("Retained: every leaf group with its real length and angle, separated by the overlap at the heart.", "Beibehalten: jede Blattgruppe mit wahrer Länge und Winkel, getrennt durch die Überlagerung im Herzen."),
        T("Merged: 2 to 3 real leaves behind each drawn leaf.", "Zusammengefasst: 2 bis 3 wirkliche Blätter hinter jedem gezeichneten Blatt."),
        T("Omitted: marginal teeth, midribs, scars and terminal spines.", "Weggelassen: Randzähne, Mittelrippen, Narben und Endstacheln."),
        T("The terminal spine (4 cm) is lost in the leaf tip.", "Der Endstachel (4 cm) geht in der Blattspitze verloren."),
        T("None.", "Keine."),
        T("Spreading leaves mid grey, erect leaves darker; one body pen.", "Ausgebreitete Blätter mittelgrau, aufrechte dunkler; ein Körperstift."),
        T("One leaf group is one thermal region (spreading or erect).", "Eine Blattgruppe ist eine thermische Region (ausgebreitet oder aufrecht).")))
    r["100"] = rep(100, 76, 150, pens(0.2, 0.13, None, None, 0.3), [], build_100(), decisions(
        T("At 1:100 (1 m is 10 mm) the eleven leaf groups are drawn as full blades with the knee between base and arching tip and the trunk with its width; the blades are 2 mm wide.", "Bei 1:100 (1 m sind 10 mm) sind die elf Blattgruppen als volle Klingen mit dem Knick zwischen Basis und bogiger Spitze gezeichnet, dazu der Stamm in seiner Breite; die Klingen sind 2 mm breit."),
        T("Retained: leaf knee (articulation), blade taper and overlap order from the outer leaves to the spear.", "Beibehalten: Blattknick (Gelenk), Klingenverjüngung und Überlagerungsfolge von den äußeren Blättern zum Herzblatt."),
        T("Merged: neighbouring real leaves into one blade.", "Zusammengefasst: benachbarte wirkliche Blätter zu einer Klinge."),
        T("Omitted: teeth, midribs and the leaf scars.", "Weggelassen: Zähne, Mittelrippen und Blattnarben."),
        T("Terminal spines and teeth are below 0.1 mm and omitted.", "Endstacheln und Zähne liegen unter 0,1 mm und entfallen."),
        T("None.", "Keine."),
        T("Blades filled, two greys for spreading and erect leaves.", "Klingen gefüllt, zwei Grautöne für ausgebreitete und aufrechte Blätter."),
        T("Each blade maps to its region; the knee seam marks the bone joint.", "Jede Klinge entspricht ihrer Region; die Knicknaht markiert das Knochengelenk.")))
    r["50"] = rep(50, 20, 75, pens(0.25, 0.18, 0.13, None, 0.35), [], build_50(), decisions(
        T("At 1:50 (1 m is 20 mm) every leaf group has its marginal teeth, its midrib and the joint seam; the trunk shows the leaf scars as rings.", "Bei 1:50 (1 m sind 20 mm) hat jede Blattgruppe ihre Randzähne, ihre Mittelrippe und die Gelenknaht; der Stamm zeigt die Blattnarben als Ringe."),
        T("Retained: blade outline with teeth, midrib, knee seam, trunk scars.", "Beibehalten: Klingenkontur mit Zähnen, Mittelrippe, Knicknaht, Stammnarben."),
        T("Merged: 2 to 3 real leaves per drawn blade.", "Zusammengefasst: 2 bis 3 wirkliche Blätter je gezeichneter Klinge."),
        T("Omitted: the terminal spine as a separate shape (it is the sharp tip).", "Weggelassen: der Endstachel als eigene Form (er ist die scharfe Spitze)."),
        T("Teeth of 5 mm are drawn as outline waviness of 0.13 mm; the midrib is a 0.18 mm detail stroke.", "Zähne von 5 mm sind als Konturwelligkeit von 0,13 mm gezeichnet; die Mittelrippe ist ein Detailstrich von 0,18 mm."),
        T("None; seams and scars are strokes.", "Keine; Nähte und Narben sind Striche."),
        T("Blades filled mid and dark grey, midribs as strokes.", "Klingen mittel- und dunkelgrau gefüllt, Mittelrippen als Striche."),
        T("Blades carry their region; strokes stay neutral.", "Klingen tragen ihre Region; Striche bleiben neutral.")))
    return r


def asset():
    A, em, regions, mats, poses = make()
    sym = {"id": "plan-extent", "title": T("Plan symbol with true rosette extent", "Grundrisssymbol mit wahrer Rosettenausdehnung"), "description": T("A conventional plan sign for planting plans: the star outline of the true rosette (about 2.6 m across) with the heart; a separate symbol, never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Pflanzpläne: der Sternumriss der wahren Rosette (etwa 2,6 m im Durchmesser) mit dem Herzen; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."), "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": {"elevation": [], "plan": plan_symbol()}, "dimensionalNote": T("The star tips mark the leaf tips at their true radius of 1.28 m.", "Die Sternspitzen markieren die Blattspitzen im wahren Radius von 1,28 m.")}
    a = {"id": ID, "category": "vegetation", "rig": {"kind": "botanical", "root": "root-plate", "bones": B.list}, "anchors": A,
         "title": T("Agave", "Agave"),
         "summary": T("A mature century plant (Agave americana) of 1.45 m height and 2.55 m width: a rosette of stiff, succulent, glaucous leaves with marginal teeth around a short trunk, the lowest leaves arching to the ground and a spear of young leaves in the heart. A CAM plant: its stomata open at night.", "Eine ausgewachsene Amerikanische Agave (Agave americana) von 1,45 m Höhe und 2,55 m Breite: eine Rosette aus steifen, sukkulenten, blaugrauen Blättern mit Randzähnen um einen kurzen Stamm, die untersten Blätter bogig zum Boden, ein Speer junger Blätter im Herzen. Eine CAM-Pflanze: Ihre Spaltöffnungen öffnen sich nachts."),
         "tags": ["agave", "succulent", "cam", "rosette", "drought", "transpiration"],
         "provenance": provenance(["nobel-1988", "nobel-hartsock-1979", "jones-2014", "monteith-unsworth-2013", "en-1991-1-4"], T("Eleven leaf groups (20 bones), each standing for two to three of the 28 real leaves, are authored by hand; the geometry of every scale is composed separately. Leaf thickness, heat capacity and conductance are authored estimates guided by Nobel (1988); the CAM behaviour (night opening) is qualitative after Nobel and Hartsock (1979), the numbers are not claimed as measured.", "Elf Blattgruppen (20 Knochen), die je für zwei bis drei der 28 wirklichen Blätter stehen, sind von Hand entworfen; die Geometrie jedes Maßstabs ist eigens komponiert. Blattdicke, Wärmekapazität und Leitfähigkeit sind Autorenschätzungen angelehnt an Nobel (1988); das CAM-Verhalten (nächtliche Öffnung) ist qualitativ nach Nobel und Hartsock (1979), die Zahlen werden nicht als gemessen ausgegeben.")),
         "dimensions": {"heightM": 1.43, "widthM": 2.52, "depthM": 2.4, "massKg": MASS},
         "materials": mats, "representations": representations(), "symbols": [sym], "poses": poses, "clips": clips(poses),
         "thermal": {"environment": {"airTempC": 28, "meanRadiantTempC": 33, "relativeHumidity": 0.35, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 900, "diffuseFraction": 0.12, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": regions, "emitters": em,
                     "assumptions": [T("Leaf regions use one-sided leaf area of the 28 real leaves (5 m2, ground projection 4.9 m2). CAM: stomata open at night (Nobel 1988; Nobel and Hartsock 1979); the maximum conductance 0.0012 m/s (about 0.05 mol/(m2 s)) is an authored estimate. The areal heat capacity of 90 kJ/(m2 K) is a 25 mm water-rich leaf (about 3.6 MJ/(m3 K)); the engine's time constant is therefore many minutes.", "Blattregionen verwenden die einseitige Blattfläche der 28 wirklichen Blätter (5 m2, Bodenprojektion 4,9 m2). CAM: Die Spaltöffnungen öffnen sich nachts (Nobel 1988; Nobel und Hartsock 1979); die maximale Leitfähigkeit 0,0012 m/s (etwa 0,05 mol/(m2 s)) ist eine Autorenschätzung. Die flächenbezogene Wärmekapazität von 90 kJ/(m2 K) entspricht einem 25 mm dicken wasserreichen Blatt (etwa 3,6 MJ/(m3 K)); die Zeitkonstante der Engine beträgt daher viele Minuten."),
                                     T("The caudex is passive. Transpiration emitters release the latent flux of their leaf region over six and seven anchors spread across the rosette; the direction is the vertical in the frame of the first anchor's bone.", "Der Caudex ist passiv. Transpirationsemitter geben den latenten Strom ihrer Blattregion über sechs und sieben Anker ab, die über die Rosette verteilt sind; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers.")]},
         "structure": structure(["base-support"], "One rigid body per evaluated pose, fixed at the root plate; wind drag acts on each leaf group (0.35 m2, cf 1.0 in calm air, 0.9 in the breeze). Leaves are stiff succulent beams; the sections are not authored and no failure is predicted.", "Ein starrer Körper je ausgewerteter Pose, an der Wurzelplatte eingespannt; Winddruck wirkt auf jede Blattgruppe (0,35 m2, cf 1,0 in ruhiger Luft, 0,9 in der Brise). Blätter sind steife sukkulente Balken; die Querschnitte sind nicht angegeben und es wird kein Versagen vorhergesagt."),
         "presentation": presentation("summer-turgid", "wind-sway", 100), "exports": EXPORTS, "completion": "authored"}
    return a


if __name__ == "__main__":
    doc = document(asset())
    validate_text(doc)
    print(dump(doc, ID), MASS)
