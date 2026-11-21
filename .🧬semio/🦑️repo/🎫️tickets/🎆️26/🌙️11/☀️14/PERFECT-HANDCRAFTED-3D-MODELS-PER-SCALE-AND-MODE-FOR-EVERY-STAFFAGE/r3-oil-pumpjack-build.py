"""🏗️ Round 3 build of `machinery/oil-pumpjack`: applies the root fixes (rig, provenance notes, drawings and their decisions of every scale, thermal areas), adds the volumes of every scale, writes the candidate document to `🗑️generated/r3-oil-pumpjack/🔣️.json`; `--source` also writes `T/volumes/machinery/oil-pumpjack.json`; `--apply` writes the changed members into the catalog file (re-read right before, every other member byte-identical) and splices the volumes with `volumes-insert.py`.

Usage: python3 r3-oil-pumpjack-build.py [--source] [--apply]
"""

import copy
import importlib.util
import json
import os
import re
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/oil-pumpjack/🔣️.json"
SOURCE = os.path.join(T, "volumes", "machinery", "oil-pumpjack.json")
OUT = os.path.join(T, "🗑️generated", "r3-oil-pumpjack")
CANDIDATE = os.path.join(OUT, "🔣️.json")
ROOTS = ["provenance", "dimensions", "materials", "rig", "representations", "thermal", "poses", "clips"]


def _load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


DR = _load("pjdraw", "r3-oil-pumpjack-draw.py")
SO = _load("pjsolids", "r3-oil-pumpjack-solids.py")

AREAS = {"gearbox": 13.2, "crank": 6.65, "motor": 4.7}

NOTES = {
    "en": "A conventional crank-balanced beam pumping unit with a 1.9 m (75 in) stroke in the class of a Lufkin C-228D-173-74, seen from the side: concrete foundation 7.8 m by 0.3 m, a 0.25 m steel skid of two rolled I-beams with cross members and deck, a four-legged Samson post with the saddle bearing at H = 4.4 m, a W24 walking beam on the centre bearing under its bottom flange, a horsehead of radius A = 3.0 m, the equalizer bearing at C = 2.7 m behind the saddle, a gear reducer with its low-speed shaft at G = 1.75 m and I = 2.7 m behind the saddle, and cranks of radius R = 0.85 m with the counterweights in phase with the wrist pin. The two pitman arms of P = 2.65 m hang from the equalizer outboard of the crank arms; the stroke is 2 R A / C = 1.9 m at the polished rod. API 11E proportions of the Lufkin geometry table (A/C 1.0 to 1.14, P about 3.16 R, I = C, H - G about P) hold; A and C are about a quarter longer than those of the 74 in Lufkin unit (2.44 and 2.13 m) and within the family of longer-stroke units. The origin is the ground below the saddle bearing; the wellhead stands at x = -3.0 m under the horsehead tangent. The rope leaves the horsehead tangentially at its leftmost point, level with the saddle, and hangs vertical, so the polished rod and the rope are drawn as one line and only the carrier bar travels along it. Every moving pose follows the closed four-bar linkage: for each crank angle the beam angle is solved so that the pitman keeps its length. A 30 kW (40 hp) TEFC motor of frame 324T lies across the unit on its slide base behind the reducer and drives the reducer input shaft through V-belts in the guard. All masses (18 t in all) and the 0.72 kW and 1.8 kW losses of gear reducer and motor are authored estimates for this class (a 30 kW prime mover at 93 percent efficiency carrying a mean shaft load of 24 kW loses 24 / 0.93 - 24 = 1.8 kW; the gears at 97 percent lose 0.72 kW). The unit is 5.57 m high at mid-stroke (the default pose, heightM) and 6.27 m with the horsehead up (the stopped pose).",
    "de": "Eine klassische kurbelausgeglichene Balancier-Pumpeneinheit mit 1,9 m (75 in) Hub in der Klasse einer Lufkin C-228D-173-74, von der Seite gesehen: Betonfundament 7,8 m mal 0,3 m, ein 0,25 m hoher Stahlrahmen aus zwei gewalzten Doppel-T-Trägern mit Querträgern und Deck, ein vierbeiniger Lagerbock mit dem Sattellager auf H = 4,4 m, ein W24-Balancier auf dem Mittellager unter seinem Untergurt, ein Pferdekopf vom Radius A = 3,0 m, das Ausgleichslager C = 2,7 m hinter dem Sattel, ein Getriebe mit seiner langsamen Welle auf G = 1,75 m und I = 2,7 m hinter dem Sattel und Kurbeln vom Radius R = 0,85 m mit den Gegengewichten in Phase mit dem Kurbelzapfen. Die zwei Schubstangen von P = 2,65 m hängen außen an den Kurbelarmen vom Ausgleichsbalken; der Hub an der Polierstange beträgt 2 R A / C = 1,9 m. Die API-11E-Verhältnisse der Lufkin-Geometrietabelle (A/C 1,0 bis 1,14, P etwa 3,16 R, I = C, H - G etwa P) gelten; A und C sind etwa ein Viertel länger als bei der 74-in-Einheit von Lufkin (2,44 und 2,13 m) und liegen in der Familie der Einheiten mit längerem Hub. Der Ursprung ist der Boden unter dem Sattellager; der Bohrlochkopf steht bei x = -3,0 m unter der Tangente des Pferdekopfs. Das Seil verlässt den Pferdekopf tangential an dessen linkem Punkt auf Sattelhöhe und hängt senkrecht, daher sind Polierstange und Seil als eine Linie gezeichnet, und nur der Gestängeträger wandert an ihr entlang. Jede bewegte Pose folgt dem geschlossenen Viergelenk: zu jedem Kurbelwinkel wird der Balancierwinkel so gelöst, dass die Schubstange ihre Länge behält. Ein 30-kW-Motor (40 hp) IC 411 der Baugröße 324T liegt quer zur Anlage auf seiner Spannschiene hinter dem Getriebe und treibt die Antriebswelle des Getriebes über Keilriemen im Schutz. Alle Massen (zusammen 18 t) und die Verluste von 0,72 kW und 1,8 kW von Getriebe und Motor sind Schätzwerte für diese Klasse (eine 30-kW-Antriebsmaschine mit 93 Prozent Wirkungsgrad bei einer mittleren Wellenlast von 24 kW verliert 24 / 0,93 - 24 = 1,8 kW; die Zahnräder mit 97 Prozent verlieren 0,72 kW). Die Einheit ist bei Mittelhub 5,57 m hoch (Standardpose, heightM) und 6,27 m mit hochgestelltem Pferdekopf (Stillstandspose).",
}

REFS = [
    {"id": "lufkin-catalog", "citation": "Lufkin Industries. Pumping Units catalog: conventional units, API geometry dimensions A, C, I, P, H, G and crank radii (C-114D to C-320D, 54 to 144 in strokes)."},
    {"id": "nema-mg1", "citation": "NEMA MG 1 Motors and Generators: frame 324T dimensions (shaft height D = 8 in) of a 40 hp four-pole TEFC motor."},
]


def bones(old):
    by = {b["id"]: copy.deepcopy(b) for b in old}
    by["base"]["lengthM"] = 5.9
    by["post"].update({"angleDeg": 90})
    by["post"]["title"] = {"en": "Samson post: four braced legs, cap and saddle blocks", "de": "Lagerbock: vier ausgesteifte Beine, Kopfplatte und Sattelböcke"}
    by["post"]["mass"].update({"comX": 1.75, "comY": 0})
    by["beam"].update({"x": 3.85, "y": 0, "angleDeg": -90, "lengthM": 2.7})
    by["beam"]["title"] = {"en": "Walking beam, tail arm from the saddle bearing to the equalizer bearing", "de": "Balancier, hinterer Arm vom Sattellager zum Ausgleichslager"}
    by["beam"]["mass"].update({"kg": 1867, "comX": 0.81, "comY": 0.4})
    head = {"id": "horsehead", "parent": "beam", "x": 0, "y": 0, "angleDeg": 180, "title": {"en": "Walking beam, front arm with the horsehead", "de": "Balancier, vorderer Arm mit Pferdekopf"}, "lengthM": 3.0, "joint": {"type": "fixed"}, "mass": {"kg": 1033, "comX": 2.3, "comY": -0.2, "distribution": "point", "source": "authored-estimate"}}
    by["pitman"].update({"angleDeg": -90})
    by["pitman"]["title"] = {"en": "Pitman arms with the equalizer", "de": "Schubstangen mit Ausgleichsbalken"}
    by["pitman"]["mass"].update({"comX": 1.4, "comY": 0})
    by["gearbox"]["lengthM"] = 1.65
    by["gearbox"]["mass"].update({"comX": 0.5, "comY": -0.4})
    by["crank"]["title"] = {"en": "Crank arms with counterweights in phase with the wrist pin", "de": "Kurbelarme mit Gegengewichten in Phase mit dem Kurbelzapfen"}
    by["crank"]["mass"].update({"comX": 0.6, "comY": 0})
    by["rod"].update({"angleDeg": 90, "lengthM": 0.16})
    by["rod"]["title"] = {"en": "Carrier bar with the polished rod clamp", "de": "Gestängeträger mit Polierstangenklemme"}
    by["motor"]["lengthM"] = 0.27
    by["motor"]["title"] = {"en": "Electric motor across the unit on its slide base", "de": "Elektromotor quer zur Anlage auf seiner Spannschiene"}
    by["motor"]["mass"].update({"comY": 0})
    by["horsehead"] = head
    out = []
    for b in old:
        if b["id"] == "horsehead":
            continue
        out.append(by[b["id"]])
        if b["id"] == "beam":
            out.append(head)
    return out


DEC2D = {
    50: {
        "silhouette": ("A 9 m wide, 5.6 m tall conventional oil pumpjack (6.3 m with the horsehead up), 18 cm by 11 cm on paper: foundation and skid, a four-legged Samson post with brace and saddle, a straight W24 walking beam on its centre bearing with the curved horsehead on the left, the pitman arms on the equalizer down to the cranks, whose counterweights stand in phase with the wrist pin, the long reducer housing with the belt guard over both sheaves, the transverse motor behind it and the wellhead with its polished rod and carrier bar. Front and top views are the true projections of the same parts: slab, skid with its I-beam ends, wellhead, legs, brace, saddle, reducer, guard, motor with cowl and shafts stand still; beam, horsehead, bearings, equalizer, both pitman arms, both cranks and the carrier bar are solids extruded across their real lateral spans from their elevation profiles, so the stroke moves them as in the side view.",
                       "Eine 9 m breite, 5,6 m hohe klassische Erdölpumpe (6,3 m mit hochgestelltem Pferdekopf), 18 cm mal 11 cm auf dem Papier: Fundament und Rahmen, ein vierbeiniger Lagerbock mit Strebe und Sattel, ein gerader W24-Balancier auf seinem Mittellager mit dem gebogenen Pferdekopf links, die Schubstangen am Ausgleichsbalken hinab zu den Kurbeln, deren Gegengewichte in Phase mit dem Kurbelzapfen stehen, das lange Getriebegehäuse mit dem Riemenschutz über beiden Scheiben, der quer liegende Motor dahinter und der Bohrlochkopf mit Polierstange und Gestängeträger. Vorder- und Draufsicht sind die wahren Projektionen derselben Teile: Platte, Rahmen mit seinen Trägerenden, Bohrlochkopf, Beine, Strebe, Sattel, Getriebe, Schutz, Motor mit Haube und Wellen stehen still; Balancier, Pferdekopf, Lager, Ausgleichsbalken, beide Schubstangen, beide Kurbeln und der Gestängeträger sind über ihre wahre Querbreite aus ihren Aufrissprofilen extrudierte Körper, sodass der Hub sie wie in der Seitenansicht bewegt."),
        "retained": ("All moving parts with their bearings: beam, horsehead with its face plate, centre bearing housing, equalizer hanger, equalizer, pitman arms with fittings and eyes, crank arms, counterweights, hubs, carrier bar and rod clamp; wellhead casing head, flange, tee, stuffing box and flowline; reducer with sub-base, input shaft; motor fins, feet, terminal box and shaft.",
                     "Alle bewegten Teile mit ihren Lagern: Balancier, Pferdekopf mit Bogenblech, Mittellagergehäuse, Ausgleichslagerbock, Ausgleichsbalken, Schubstangen mit Kopfstücken und Augen, Kurbelarme, Gegengewichte, Naben, Gestängeträger und Stangenklemme; Futterrohrkopf, Flansch, T-Stück, Stopfbuchse und Flowline; Getriebe mit Untersatz und Antriebswelle; Motorrippen, Füße, Klemmenkasten und Welle."),
        "merged": ("The polished rod, the bridle ropes and the vertical rope run are one 50 mm line from the stuffing box to the tangent point of the horsehead: the rope always leaves the head level with the saddle, only the carrier bar travels along it.",
                   "Polierstange, Seilgehänge und senkrechter Seilstrang sind eine 50-mm-Linie von der Stopfbuchse zum Tangentenpunkt des Pferdekopfs: das Seil verlässt den Kopf immer auf Sattelhöhe, nur der Gestängeträger wandert an ihr entlang."),
        "omitted": ("Hidden parts: the wrist pins behind the pitman eyes, the sheaves and belts inside the guard, the gears; and bolts, ladder, brake and lubrication fittings.",
                    "Verdeckte Teile: die Kurbelzapfen hinter den Schubstangenaugen, Scheiben und Riemen im Schutz, die Zahnräder; dazu Schrauben, Leiter, Bremse und Schmierstellen."),
        "thinElements": ("The polished rod is 50 mm (1.0 mm on paper), the pitman arm 0.16 m (3.2 mm), the carrier bar 0.12 m high (2.4 mm), the horsehead face plate 30 mm (0.6 mm).",
                         "Die Polierstange ist 50 mm (1,0 mm auf dem Papier), die Schubstange 0,16 m (3,2 mm), der Gestängeträger 0,12 m hoch (2,4 mm), das Bogenblech des Pferdekopfs 30 mm (0,6 mm)."),
    },
    100: {
        "silhouette": ("A 9 cm wide pumpjack: foundation and skid, four-legged post with brace and saddle, straight beam with horsehead and bearings, pitman arms on the equalizer, cranks with in-phase counterweights, reducer with belt guard, transverse motor and wellhead. Front and top views: as at 1:50 with the wellhead in two blocks and without the face plate, motor fins and guard brackets; the moving parts are the same extruded solids.",
                       "Eine 9 cm breite Erdölpumpe: Fundament und Rahmen, vierbeiniger Bock mit Strebe und Sattel, gerader Balancier mit Pferdekopf und Lagern, Schubstangen am Ausgleichsbalken, Kurbeln mit gleichphasigen Gegengewichten, Getriebe mit Riemenschutz, quer liegender Motor und Bohrlochkopf. Vorder- und Draufsicht: wie bei 1:50 mit dem Bohrlochkopf in zwei Blöcken und ohne Bogenblech, Motorrippen und Schutzkonsolen; die bewegten Teile sind dieselben extrudierten Körper."),
        "retained": ("Every moving part and the main masses: post with brace and saddle, beam with both bearing housings, horsehead, equalizer, pitman arms with eyes, crank arms with hubs and counterweights, reducer with sub-base and input shaft, belt guard, motor with feet, cowl, terminal box and shaft.",
                     "Jedes bewegte Teil und die Hauptmassen: Bock mit Strebe und Sattel, Balancier mit beiden Lagergehäusen, Pferdekopf, Ausgleichsbalken, Schubstangen mit Augen, Kurbelarme mit Naben und Gegengewichten, Getriebe mit Untersatz und Antriebswelle, Riemenschutz, Motor mit Füßen, Haube, Klemmenkasten und Welle."),
        "merged": ("The wellhead flange, tee, stuffing box and flowline are two blocks; the rope and the polished rod are one 50 mm line.",
                   "Flansch, T-Stück, Stopfbuchse und Flowline des Bohrlochkopfs sind zwei Blöcke; Seil und Polierstange sind eine 50-mm-Linie."),
        "omitted": ("Horsehead face plate, motor fins, guard brackets and the hidden wrist pins.",
                    "Bogenblech des Pferdekopfs, Motorrippen, Schutzkonsolen und die verdeckten Kurbelzapfen."),
        "thinElements": ("The polished rod is 50 mm (0.5 mm on paper), the pitman arm 0.16 m (1.6 mm).",
                         "Die Polierstange ist 50 mm (0,5 mm auf dem Papier), die Schubstange 0,16 m (1,6 mm)."),
    },
    200: {
        "silhouette": ("A 4.5 cm wide pumpjack silhouette: the four-legged post, the straight nodding beam with its head, the pitman arms down to the cranks with their counterweights, the long reducer with the belt guard and the motor on one skid and slab. Front and top views: main masses only: slab, skid, one wellhead block, rod, legs, brace, saddle, reducer, guard, motor and the extruded beam, housings, horsehead, equalizer, pitman arms, cranks and carrier bar.",
                       "Die Silhouette einer 4,5 cm breiten Erdölpumpe: der vierbeinige Bock, der gerade nickende Balancier mit Kopf, die Schubstangen hinab zu den Kurbeln mit ihren Gegengewichten, das lange Getriebe mit Riemenschutz und der Motor auf einem Rahmen und einer Platte. Vorder- und Draufsicht: nur Hauptmassen: Platte, Rahmen, ein Bohrlochkopfblock, Stange, Beine, Strebe, Sattel, Getriebe, Schutz, Motor und die extrudierten Teile Balancier, Lagergehäuse, Pferdekopf, Ausgleichsbalken, Schubstangen, Kurbeln und Gestängeträger."),
        "retained": ("Slab with skid, the post legs with brace and saddle, beam with its housings, horsehead, equalizer, pitman arms, crank arms with hubs and counterweights, reducer with sub-base, belt guard and motor with feet.",
                     "Platte mit Rahmen, die Bockbeine mit Strebe und Sattel, Balancier mit Lagergehäusen, Pferdekopf, Ausgleichsbalken, Schubstangen, Kurbelarme mit Naben und Gegengewichten, Getriebe mit Untersatz, Riemenschutz und Motor mit Füßen."),
        "merged": ("The wellhead is one stepped block; the rope and the rod are one line; the pitman eyes are part of the arms; motor frame and cowl are one body.",
                   "Der Bohrlochkopf ist ein gestufter Block; Seil und Stange sind eine Linie; die Schubstangenaugen gehören zu den Armen; Motorgehäuse und Haube sind ein Körper."),
        "omitted": ("All details of 1:100: rod clamp, terminal box, shafts.",
                    "Alle Details von 1:100: Stangenklemme, Klemmenkasten, Wellen."),
        "thinElements": ("The rod is 50 mm (0.25 mm on paper) and the pitman arm 0.16 m (0.8 mm).",
                         "Die Stange ist 50 mm (0,25 mm auf dem Papier) und die Schubstange 0,16 m (0,8 mm)."),
    },
    500: {
        "silhouette": ("An 18 mm wide pumpjack mark: slab and skid, the four legs, the beam with its head and housings, the pitman arms on the equalizer, the cranks with counterweights, the reducer with guard and the motor. Front and top views: composed silhouette of the same masses as 1:200 without the brace, with the polished rod widened; the moving solids keep their elevation profiles.",
                       "Ein 18 mm breites Zeichen einer Erdölpumpe: Platte und Rahmen, die vier Beine, der Balancier mit Kopf und Lagergehäusen, die Schubstangen am Ausgleichsbalken, die Kurbeln mit Gegengewichten, das Getriebe mit Schutz und der Motor. Vorder- und Draufsicht: zusammengesetzte Silhouette derselben Massen wie bei 1:200 ohne Strebe, mit verbreiterter Polierstange; die bewegten Körper behalten ihre Aufrissprofile."),
        "retained": ("Slab, skid, four legs, saddle, beam with housings, horsehead, equalizer, pitman arms, crank arms and counterweights, reducer, belt guard, motor, wellhead block and rod.",
                     "Platte, Rahmen, vier Beine, Sattel, Balancier mit Lagergehäusen, Pferdekopf, Ausgleichsbalken, Schubstangen, Kurbelarme und Gegengewichte, Getriebe, Riemenschutz, Motor, Bohrlochkopfblock und Stange."),
        "merged": ("The reducer outline is a polygon over its arch; motor frame and cowl are one cylinder.",
                   "Der Getriebeumriss ist ein Polygon über seinem Bogen; Motorgehäuse und Haube sind ein Zylinder."),
        "omitted": ("The brace, the hubs, the pitman eyes as separate shapes, and every detail of 1:200.",
                    "Die Strebe, die Naben, die Schubstangenaugen als eigene Formen und jedes Detail von 1:200."),
        "thinElements": ("The rod is widened to 0.1 m (0.2 mm on paper); the pitman arm is 0.16 m (0.32 mm).",
                         "Die Stange ist auf 0,1 m verbreitert (0,2 mm auf dem Papier); die Schubstange ist 0,16 m (0,32 mm)."),
    },
}

STYLES = {
    "styles": ("Slab 0.82, skid 0.5, steel 0.55 to 0.78, bearings 0.3, reducer 0.62, counterweights 0.4, crank arms 0.58, motor 0.68, guard 0.45.",
               "Platte 0,82, Rahmen 0,5, Stahl 0,55 bis 0,78, Lager 0,3, Getriebe 0,62, Gegengewichte 0,4, Kurbelarme 0,58, Motor 0,68, Schutz 0,45."),
    "overlays": ("Slab and skid, wellhead, steel members, reducer, cranks and motor are six regions: in the thermal view the motor and the reducer warm while the pump runs and the wellhead carries the warm produced fluid; beam, horsehead, pitman arms, cranks and carrier bar move with their bones.",
                 "Platte und Rahmen, Bohrlochkopf, Stahlteile, Getriebe, Kurbeln und Motor sind sechs Regionen: in der Thermoansicht erwärmen sich Motor und Getriebe beim Pumpen, und der Bohrlochkopf führt das warme Fördermedium; Balancier, Pferdekopf, Schubstangen, Kurbeln und Gestängeträger bewegen sich mit ihren Knochen."),
}


def decisions2d(old, scale):
    out = {}
    for k, v in old.items():
        if k in DEC2D[scale]:
            e, g = DEC2D[scale][k]
            out[k] = {"en": e, "de": g}
        elif k == "styles" and scale == 50:
            out[k] = {"en": STYLES[k][0] + " Body pen 0.25 mm, detail 0.13 mm, ground 0.35 mm.", "de": STYLES[k][1] + " Körperstift 0,25 mm, Detail 0,13 mm, Boden 0,35 mm."}
        elif k == "overlays":
            out[k] = {"en": STYLES[k][0], "de": STYLES[k][1]}
        else:
            out[k] = v
    return out


def build(document):
    a = document["asset"]
    a["provenance"]["notes"] = dict(NOTES)
    have = {r["id"] for r in a["provenance"]["references"]}
    a["provenance"]["references"] += [r for r in REFS if r["id"] not in have]
    a["rig"]["bones"] = bones(a["rig"]["bones"])
    for s in (50, 100, 200, 500):
        rep = a["representations"][str(s)]
        rep["geometry"] = DR.geometry(s)
        rep["decisions"] = decisions2d(rep["decisions"], s)
    for reg in a["thermal"]["regions"]:
        if reg["id"] in AREAS:
            reg["areaM2"] = AREAS[reg["id"]]
    a["volumes"] = {str(s): SO.volume(s) for s in (50, 100, 200, 500)}
    import runpy
    runpy.run_path(os.path.join(T, "pumpjack-rod.py"))["apply"](a)
    return document


def scalar(v):
    return not isinstance(v, (dict, list))


def inline(v):
    if isinstance(v, list):
        return all(scalar(i) for i in v) or (len(v) > 0 and all(isinstance(i, list) and all(scalar(x) for x in i) for i in v))
    return False


def dump(v, depth):
    pad, inner = " " * depth, " " * (depth + 1)
    if isinstance(v, dict):
        if not v:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(i, depth + 1) for k, i in v.items()) + "\n" + pad + "}"
    if isinstance(v, list):
        if not v:
            return "[]"
        if inline(v) and not all(isinstance(i, str) for i in v):
            return json.dumps(v, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(i, depth + 1) for i in v) + "\n" + pad + "]"
    return json.dumps(v, ensure_ascii=False)


def span(text, key):
    m = re.search(r'\n  "' + re.escape(key) + r'": ', text)
    i, depth, quoted = m.end(), 0, False
    while True:
        c = text[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return m.end(), i + 1
        i += 1


def apply(candidate):
    text = open(CATALOG, encoding="utf-8").read()
    asset = candidate["asset"]
    for key in ROOTS:
        a, b = span(text, key)
        text = text[:a] + dump(asset[key], 2) + text[b:]
    after = json.loads(text)["asset"]
    before = json.loads(open(CATALOG, encoding="utf-8").read())["asset"]
    for k in before:
        if k not in ROOTS and k != "volumes":
            assert before[k] == after[k], k
    for k in ROOTS:
        assert after[k] == asset[k], k
    with open(CATALOG, "w", encoding="utf-8") as f:
        f.write(text)
    print("catalog roots written:", ", ".join(ROOTS))


if __name__ == "__main__":
    doc = build(json.loads(open(CATALOG, encoding="utf-8").read()))
    os.makedirs(OUT, exist_ok=True)
    with open(CANDIDATE, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print("candidate", CANDIDATE, {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in doc["asset"]["volumes"].items()})
    if "--source" in sys.argv or "--apply" in sys.argv:
        with open(SOURCE, "w", encoding="utf-8") as f:
            json.dump({"volumes": doc["asset"]["volumes"]}, f, ensure_ascii=False, indent=1)
        print("source", SOURCE)
    if "--apply" in sys.argv:
        apply(doc)
        subprocess.run(["python3", os.path.join(T, "volumes-insert.py"), SOURCE, CATALOG], check=True)
