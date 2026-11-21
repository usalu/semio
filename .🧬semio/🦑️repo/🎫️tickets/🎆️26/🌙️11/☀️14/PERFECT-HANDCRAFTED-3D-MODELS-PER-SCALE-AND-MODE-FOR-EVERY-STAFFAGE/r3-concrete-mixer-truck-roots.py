"""🧬 Root fixes of `machinery/concrete-mixer-truck` (round 3) outside `volumes` and the drawings: dimensions, rig joints and bone lengths, anchors, poses, clip keys, thermal areas, provenance. Every value is stated here once, old values are listed in the report."""

import json
import os as _o
from importlib import util as _u

_s = _u.spec_from_file_location("kit", _o.path.join(_o.path.dirname(_o.path.abspath(__file__)), "r3-concrete-mixer-truck-kit.py"))
K = _u.module_from_spec(_s)
_s.loader.exec_module(K)

AREAS = {"chassis": 38, "engine-bay": 4.8, "drum-drive": 2.6, "tyres": 32.27, "brakes": 6.2}
NEW_REGIONS = [
    ("running-gear", {"en": "Axles, springs, rims and fenders", "de": "Achsen, Federn, Felgen und Kotflügel"}, 40, 1.0),
    ("superstructure", {"en": "Mixer subframe, pedestals, rear frame, water tank and ladder", "de": "Mischer-Hilfsrahmen, Lagerböcke, Heckrahmen, Wassertank und Leiter"}, 24, 1.2),
]
REFERENCES = [
    {"id": "liebherr-htm-905", "citation": "Liebherr-Mischtechnik GmbH, truck mixer HTM 905, technical data (product page, read 2026-11-19): nominal filling 9 m3, water capacity 10.5 m3, geometric drum capacity 16.0 m3, clearance height without chassis frame 2544 mm, hopper filling height without frame 2519 mm, assembly weight 3780 kg."},
    {"id": "man-tgs-8x4", "citation": "MAN Truck & Bus, TGS 8x4 BB chassis data sheets (dealer specification sheets of the TGS 41.400 8x4 BB, read 2026-11-19): axle spacing 1795 mm between the two front axles and 1400 mm between the two rear axles, front overhang 1475 mm; mixer chassis with 2505 mm between axle 2 and axle 3, frame top about 1.08 m, tyres 315/80 R 22.5 (single on the front axles, twin on the rear axles)."},
]
NOTES = {
    "en": "A 9 m3 truck mixer on an 8x4 heavy chassis (about 13.5 t unladen, 32 t gross, 330 kW diesel). Chassis geometry follows the MAN TGS 8x4 BB class (front overhang 1.475 m, axle spacing 1.795 + 2.505 + 1.400 m, 315/80 R 22.5 tyres, twin tyres on both rear axles), the mixer body the Liebherr HTM 905 class (drum of 16 m3 geometric capacity, diameter 2.3 m, 11 degree axis, 2.54 m above the chassis frame, so 3.65 m overall); the remaining machine numbers are authored estimates of that class, the width respects the 2.55 m limit of Directive 96/53/EC (mirrors excluded). The drum rotation is shown by a flight pattern that drifts along the drum axis (the rotation axis lies in the drawing plane); in transit the swivel chute is swung 90 degrees to the right and its folding section lies folded back on it, two loose extension chutes ride on the rear mudguards.",
    "de": "Ein 9-m3-Fahrmischer auf schwerem 8x4-Fahrgestell (etwa 13,5 t leer, 32 t Gesamtgewicht, 330-kW-Diesel). Die Fahrgestellgeometrie folgt der Klasse MAN TGS 8x4 BB (vorderer Überhang 1,475 m, Achsabstände 1,795 + 2,505 + 1,400 m, Reifen 315/80 R 22,5, Zwillingsreifen an beiden Hinterachsen), der Mischeraufbau der Klasse Liebherr HTM 905 (Trommel mit 16 m3 geometrischem Inhalt, Durchmesser 2,3 m, 11 Grad Achsneigung, 2,54 m über dem Fahrgestellrahmen, also 3,65 m Gesamthöhe); die übrigen Maschinenzahlen sind Schätzungen dieser Klasse, die Breite hält die Grenze von 2,55 m der Richtlinie 96/53/EG ein (ohne Spiegel). Die Trommeldrehung wird durch ein längs der Trommelachse wanderndes Wendelmuster gezeigt (die Drehachse liegt in der Zeichenebene); in Fahrt ist die Schwenkschurre um 90 Grad nach rechts geschwenkt, ihr Klappteil liegt zurückgeklappt darauf, zwei lose Verlängerungsschurren liegen auf den hinteren Kotflügeln.",
}


def tyre_exposure(asset):
    region=next(r for r in asset["thermal"]["regions"] if r["id"]=="tyres")
    region["areaM2"]=32.27
    for pose in asset["poses"]:
        tyre=pose["thermal"]["regions"]["tyres"]
        tyre["exposedFraction"]=round(26.71/32.27,6)
        weight=asset["dimensions"]["massKg"]*9.80665-sum(load.get("fyN",0) for load in pose["loads"] if load["kind"]=="carried")
        tyre["contact"]["fraction"]=round(weight/(850000*26.71),6)
    note={"en":"The twelve tyres have 32.27 m² of rubber surface. Their rim-covered bead seats account for 5.56 m². The road contact patch is estimated from the supported weight in each pose and a mean contact pressure of 0.85 MPa.","de":"Die zwölf Reifen haben 32,27 m² Gummioberfläche. Davon liegen 5,56 m² an den Felgensitzen. Die Aufstandsfläche auf der Fahrbahn wird aus der jeweiligen Gewichtskraft und einem mittleren Kontaktdruck von 0,85 MPa geschätzt."}
    if note not in asset["thermal"]["assumptions"]:asset["thermal"]["assumptions"].append(note)


def bone(asset, id):
    return next(b for b in asset["rig"]["bones"] if b["id"] == id)


def offsets(a, e, t):
    return {"chute": {"angleDeg": round(8 * a / 165, 3)}, "chute-ext": {"angleDeg": round(-e, 3)}, "chute-swivel": {"twistDeg": round(90 - 2 * t, 3)}}


POSES = {"discharging": offsets(165, -180, 30), "chute-swing": offsets(80, -90, 14.545)}


def apply(asset):
    asset["dimensions"]["heightM"] = K.HEIGHT
    asset["summary"] = {"en": "An 8x4 truck mixer of the 9 m3 class: heavy chassis with cab-over-engine cab and vertical exhaust stack, a mixer drum tilted 11 degrees whose spiral flight marks drift with its rotation, charging hopper, and a swivel chute with folding section that is swung to the side in transit.", "de": "Ein 8x4-Fahrmischer der 9-m3-Klasse: schweres Fahrgestell mit Frontlenker-Fahrerhaus und senkrechtem Auspuffrohr, eine um 11 Grad geneigte Mischtrommel, deren Wendelmarken mit der Drehung wandern, Einfülltrichter und eine Schwenkschurre mit Klappteil, die in Fahrt zur Seite geschwenkt ist."}
    refs = asset["provenance"]["references"]
    for r in REFERENCES:
        if all(x["id"] != r["id"] for x in refs):
            refs.insert(1 + REFERENCES.index(r), r)
    asset["provenance"]["notes"] = NOTES
    b = bone(asset, "chassis")
    b["mass"].update({"kg": 7370, "comX": 0.9, "comY": 1.0})
    b = bone(asset, "drum")
    b.update({"x": K.DRUM_O[0], "y": K.DRUM_O[1], "angleDeg": K.DRUM_A, "lengthM": K.DRUM_L})
    b = bone(asset, "chute-swivel")
    b.update({"x": K.SW[0], "y": K.SW[1]})
    b = bone(asset, "chute")
    b.update({"angleDeg": K.CHUTE_A - 90, "lengthM": K.CHUTE_L, "joint": {"type": "revolute", "minDeg": -15, "maxDeg": 25}})
    b["mass"].update({"kg": 90, "comX": 0.6})
    b = bone(asset, "chute-ext")
    b.update({"x": K.CHUTE_L, "y": 0, "angleDeg": 180, "lengthM": K.EXT_L, "joint": {"type": "revolute", "minDeg": -5, "maxDeg": 185}})
    b["mass"].update({"kg": 40, "comX": 0.42})
    for i in range(4):
        bone(asset, f"wheel-{i + 1}")["x"] = K.AX[i]
    for a in asset["anchors"]:
        if a["id"].startswith("contact-"):
            i = int(a["id"][8]) - 1
            sg = 1 if a["id"].endswith("-l") else -1
            a["x"] = K.AX[i]
            a["z"] = sg * (K.ZF if i < 2 else round((K.ZI + K.ZO) / 2, 3))
            a["footprintWidthM"] = 0.3 if i < 2 else 0.66
        if a["id"] == "stack-mouth":
            a.update({"x": K.STACK[0] - 0.02, "y": 3.3, "z": K.STACK[1]})
        if a["id"] == "radiator-outlet":
            a.update({"x": 2.9, "y": 0.75})
    for p in asset["poses"]:
        p["bones"] = POSES.get(p["id"], {"chute-swivel": {"twistDeg": 90}})
    for c in asset["clips"]:
        done = any(k["bones"].get("chute-swivel", {}).get("twistDeg", 0) > 30.001 for k in c["keys"])
        for k in c["keys"]:
            kb = k["bones"]
            if "chute" in kb and not done:
                new = offsets(kb["chute"]["angleDeg"], kb["chute-ext"]["angleDeg"], kb["chute-swivel"]["twistDeg"])
                k["bones"] = {"chute": new["chute"], "chute-ext": new["chute-ext"], "drum-spiral": kb["drum-spiral"], "chute-swivel": new["chute-swivel"]}
            elif "chute" not in kb:
                k["bones"] = {"drum-spiral": kb["drum-spiral"], "chute-swivel": {"twistDeg": 90}}
    for p in asset["poses"]:
        if p["id"] == "transit-agitating":
            p["description"] = {"en": "Driving to site with the full drum turning slowly, swivel chute swung to the right and locked, folding chute folded back, engine at part load.", "de": "Fahrt zur Baustelle mit voller, langsam drehender Trommel, Schwenkschurre nach rechts geschwenkt und verriegelt, Klappschurre zurückgeklappt, Motor in Teillast."}
        if p["id"] == "discharging":
            p["description"] = {"en": "Swivel chute turned 30 degrees off the rear and lowered, folding chute unfolded, drum turning at discharge speed, concrete half discharged.", "de": "Schwenkschurre 30 Grad neben der Längsachse nach hinten gedreht und abgesenkt, Klappschurre ausgeklappt, Trommel mit Entleerdrehzahl, Beton zur Hälfte entleert."}
        if p["id"] == "chute-swing":
            p["description"] = {"en": "Key pose of the discharge sequence: the swivel chute is half way from its transport lock to the rear, the folding chute stands upright half unfolded.", "de": "Schlüsselpose der Entleerfolge: Schwenkschurre auf halbem Weg von der Transportstellung nach hinten, Klappschurre steht halb ausgeklappt aufrecht."}
    regions = asset["thermal"]["regions"]
    chassis = next(r for r in regions if r["id"] == "chassis")
    chassis["title"] = {"en": "Frame, tanks, bumper and steps", "de": "Rahmen, Tanks, Stoßfänger und Tritte"}
    at = regions.index(chassis) + 1
    for id, title, area, length in NEW_REGIONS:
        if all(r["id"] != id for r in regions):
            regions.insert(at, {**json.loads(json.dumps(chassis)), "id": id, "title": title, "areaM2": area, "characteristicLengthM": length})
            at += 1
    for r in regions:
        if r["id"] in AREAS:
            r["areaM2"] = AREAS[r["id"]]
    for p in asset["poses"]:
        pr = p["thermal"]["regions"]
        if "running-gear" not in pr:
            p["thermal"]["regions"] = {k: v for key, val in pr.items() for k, v in ([(key, val)] + ([("running-gear", json.loads(json.dumps(val))), ("superstructure", json.loads(json.dumps(val)))] if key == "chassis" else []))}
    tyre_exposure(asset)
    for b in asset["rig"]["bones"]:
        if b["id"] == "chute-swivel":
            b["title"] = {"en": "Chute swivel bearing under the discharge funnel", "de": "Schurren-Drehlager unter dem Auslauftrichter"}
        if b["id"] == "chute":
            b["title"] = {"en": "Swivel chute", "de": "Schwenkschurre"}
        if b["id"] == "chute-ext":
            b["title"] = {"en": "Folding chute", "de": "Klappschurre"}
