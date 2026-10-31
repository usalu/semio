"""Builds S/🗂️catalog/👤️humans/seated-guitarist/🔣️.json"""
from lib import *
from common_build import *
from guit_rig import *
import guit_art

anchors = build_anchors(L)
A_SKIN = dubois(H, MASS)
P = poses()
for n in P:
    P[n].aim("hand-l", 40)
P["strum-up"] = None
base = P["strumming"]
g = base.copy()
g.rel("forearm-r", g.off["forearm-r"]["angleDeg"] + 14)
g.rel("hand-r", g.off["hand-r"]["angleDeg"] + 8)
g.rel("upper-arm-r", g.off["upper-arm-r"]["angleDeg"] - 5)
P["strum-up"] = g
for n, f in P.items():
    print(n, joint_report(f))

MAT = [
    {"id": "skin", "title": tx("Skin", "Haut"), "emissivity": 0.98, "solarAbsorptivity": 0.6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "hair", "title": tx("Short hair and beard", "Kurzes Haar und Bart"), "emissivity": 0.95, "solarAbsorptivity": 0.85, "source": "authored-estimate"},
    {"id": "flannel", "title": tx("Cotton flannel shirt", "Baumwoll-Flanellhemd"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "denim", "title": tx("Denim jeans", "Jeansstoff"), "emissivity": 0.92, "solarAbsorptivity": 0.7, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "leather", "title": tx("Leather work boot", "Lederstiefel"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "source": "authored-estimate"},
    {"id": "spruce", "title": tx("Spruce top with satin lacquer", "Fichtendecke mit Seidenlack"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 420, "specificHeatJKgK": 1600, "source": "authored-estimate"},
    {"id": "mahogany", "title": tx("Mahogany neck and headstock", "Hals und Kopfplatte aus Mahagoni"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 550, "specificHeatJKgK": 1600, "source": "authored-estimate"},
    {"id": "rosewood", "title": tx("Rosewood bridge", "Steg aus Palisander"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": 850, "source": "authored-estimate"},
    {"id": "pine", "title": tx("Pine bar stool", "Barhocker aus Kiefer"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 500, "specificHeatJKgK": 1600, "source": "authored-estimate"},
    {"id": "denim-sock", "title": tx("Cotton sock", "Baumwollsocke"), "emissivity": 0.95, "solarAbsorptivity": 0.5, "source": "authored-estimate", "reference": "iso-9920-2007"},
]
fc = dict(shirt=1.22, jeans=1.15)


def regions():
    R = []
    R.append(region("face", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.04 * A_SKIN, 0.15, "sphere", 0.5, [], 15000, "head", 0.06, 0.04, ref="dubois-1916"))
    R.append(region("hair", "Short hair and beard", "Kurzes Haar und Bart", "hair", "hair", 0.04 * A_SKIN * 1.2, 0.16, "sphere", 0.5, [layer("hair-pack", "Hair and trapped air", "Haar und eingeschlossene Luft", "hair", "hair", 0.012, 0.05, 250, src="authored-estimate")], 4000, "head", 0.04, 0.04))
    R.append(region("shirt-torso", "Flannel shirt over T-shirt, trunk", "Flanellhemd über T-Shirt, Rumpf", "clothing", "flannel", 0.37 * A_SKIN * fc["shirt"], 0.32, "cylinder", 0.45, [
        layer("torso-air", "Air between skin and T-shirt", "Luft zwischen Haut und T-Shirt", "air-gap", "flannel", 0.004, 0.025),
        layer("tshirt", "Cotton T-shirt (0.09 clo)", "Baumwoll-T-Shirt (0,09 clo)", "clothing", "flannel", 0.0008, clo_r(0.09, fc["shirt"]), 250),
        layer("flannel", "Flannel shirt (0.30 clo)", "Flanellhemd (0,30 clo)", "clothing", "flannel", 0.002, clo_r(0.30, fc["shirt"]), 500)], 19000, "trunk", 0.40, 0.37))
    R.append(region("shirt-sleeves", "Flannel sleeves", "Flanellärmel", "clothing", "flannel", 0.14 * A_SKIN * fc["shirt"], 0.09, "cylinder", 0.45, [
        layer("sleeve-air", "Air in the sleeve", "Luft im Ärmel", "air-gap", "flannel", 0.005, 0.03),
        layer("sleeve", "Flannel sleeve (0.30 clo)", "Flanellärmel (0,30 clo)", "clothing", "flannel", 0.002, clo_r(0.30, fc["shirt"]), 450)], 15000, "arm", 0.08, 0.14))
    R.append(region("hands", "Bare hands and fingers", "Nackte Hände und Finger", "skin", "skin", 0.05 * A_SKIN, 0.06, "cylinder", 0.45, [], 12000, "hand", 0.02, 0.05, ref="dubois-1916"))
    R.append(region("jeans", "Denim jeans", "Jeans", "clothing", "denim", 0.31 * A_SKIN * fc["jeans"], 0.12, "cylinder", 0.35, [
        layer("leg-air", "Air between skin and jeans", "Luft zwischen Haut und Jeans", "air-gap", "denim", 0.005, 0.03),
        layer("denim", "Denim (0.26 clo)", "Jeansstoff (0,26 clo)", "clothing", "denim", 0.001, clo_r(0.26, fc["jeans"]), 550)], 18000, "leg", 0.35, 0.31))
    R.append(region("boots", "Leather boots with socks", "Lederstiefel mit Socken", "clothing", "leather", 0.05 * A_SKIN, 0.12, "horizontal-plate-up", 0.25, [
        layer("sock", "Cotton sock (0.04 clo)", "Baumwollsocke (0,04 clo)", "clothing", "denim-sock", 0.002, 0.0062, 300),
        layer("boot-air", "Air in the boot", "Luft im Stiefel", "air-gap", "leather", 0.008, 0.035),
        layer("boot", "Leather upper (0.08 clo)", "Lederschaft (0,08 clo)", "shell", "leather", 0.004, 0.03, 1100)], 8000, "foot", 0.05, 0.05))
    R.append(region("guitar", "Acoustic guitar", "Akustikgitarre", "surface", "spruce", 0.62, 0.45, "vertical-plate", 0.5, [], 5000, None, None, None, model="passive", ref="incropera-2011"))
    R.append(region("stool", "Pine bar stool", "Barhocker aus Kiefer", "surface", "pine", 0.55, 0.4, "horizontal-plate-up", 0.4, [], 9000, None, None, None, model="passive", ref="incropera-2011"))
    return R


REG = regions()
RIDS = [r["id"] for r in REG]


def bind(act, br, plume=0.5, seat_contact=0.35):
    reg = {}
    for rid in RIDS:
        if rid == "jeans":
            reg[rid] = rbind(1.0 - seat_contact, 0.0, contact=(seat_contact, 23))
        elif rid == "boots":
            reg[rid] = rbind(0.8, 0.0, contact=(0.2, 20))
        elif rid == "guitar":
            reg[rid] = rbind(0.8, 0.0, contact=(0.2, 31))
        elif rid == "stool":
            reg[rid] = rbind(0.7, 0.0, contact=(0.3, 31))
        elif rid == "hands":
            reg[rid] = rbind(0.7, 0.0, contact=(0.3, 22))
        else:
            reg[rid] = rbind(1.0, 0.0)
    return {"activityMet": act, "breathing": br, "regions": reg, "emitters": {"plume": plume}}


TB = {"strumming": bind(1.7, "play"), "fretting-high": bind(1.8, "sing"), "resting": bind(1.2, "rest", 0.4), "tuning": bind(1.5, "play", 0.5), "strum-up": bind(1.7, "play")}
CONT = ["stool-ground", "seat", "rung-support"]


def pose_list():
    spec = [
        ("strumming", "Strumming", "Anschlagen", "Seated on the stool, the near foot on the rung, the guitar on the far thigh, the right forearm over the lower bout and the left hand around the neck at the fifth fret.", "Auf dem Hocker sitzend, der nahe Fuß auf der Fußraste, die Gitarre auf dem fernen Oberschenkel, der rechte Unterarm über dem unteren Korpus und die linke Hand am Hals im fünften Bund.", False),
        ("fretting-high", "Fretting high on the neck", "Greifen weit oben am Hals", "The left hand slides towards the body for a high chord, the head bends over the fretboard and the player sings.", "Die linke Hand rutscht zum Korpus für einen hohen Griff, der Kopf neigt sich über das Griffbrett und der Spieler singt.", False),
        ("resting", "Resting the guitar", "Gitarre ablegen", "Both hands rest on the guitar, back straight, looking up.", "Beide Hände ruhen auf der Gitarre, Rücken gerade, Blick nach oben.", False),
        ("tuning", "Tuning", "Stimmen", "Leaning towards the headstock to turn a tuning peg with the right hand.", "Zur Kopfplatte vorgebeugt, um mit der rechten Hand einen Wirbel zu drehen.", True),
        ("strum-up", "Up-stroke", "Aufschlag", "Key pose of the strumming loop: forearm lifted for the up-stroke.", "Schlüsselpose der Anschlagschleife: Unterarm für den Aufschlag gehoben.", True),
    ]
    return [pose_record(pid, en, de, den, dee, P[pid], CONT, TB[pid], key=key) for pid, en, de, den, dee, key in spec]


def clips():
    ph = lambda i, fr, t, en, de: (i, fr, t, (en, de), False)
    c = []
    c.append(clip_record("strum-loop", "Strumming a folk rhythm", "Folk-Rhythmus schlagen",
        "Loop of 1.0 s (120 beats per minute, down-up): the right forearm swings over the strings while the left hand holds the chord; breathing follows the playing pattern.",
        "Schleife von 1,0 s (120 Schläge pro Minute, ab-auf): Der rechte Unterarm schwingt über die Saiten, während die linke Hand den Akkord hält; die Atmung folgt dem Spielmuster.",
        1.0, True, [(0, "strumming", "smooth"), (0.25, "strum-up", "smooth"), (0.5, "strumming", "smooth"), (0.75, "strum-up", "smooth")],
        [(a, 0, 1.0) for a in CONT], [ph("down", 0, 0.25, "Down-stroke", "Abschlag"), ph("up", 0.25, 0.5, "Up-stroke", "Aufschlag"), ph("down2", 0.5, 0.75, "Down-stroke", "Abschlag"), ph("up2", 0.75, 1.0, "Up-stroke", "Aufschlag")],
        [(0, "motion", ("Down-stroke on the beat", "Abschlag auf dem Schlag"), None)], [("breathing", "step", [(0, 1), (1.0, 1)])]))
    c.append(clip_record("chord-change", "Chord change and verse", "Akkordwechsel und Strophe",
        "Non-looping 4 s phrase: strumming, a slide of the left hand to the high chord while singing, and back to the first chord.",
        "Nicht wiederholte Phrase von 4 s: Anschlagen, Gleiten der linken Hand zum hohen Akkord beim Singen und zurück zum ersten Akkord.",
        4.0, False, [(0, "strumming", "smooth"), (1.0, "strumming", "smooth"), (1.8, "fretting-high", "smooth"), (3.0, "fretting-high", "smooth"), (3.8, "strumming", "smooth")],
        [(a, 0, 4.0) for a in CONT], [ph("play", 0, 1.0, "Playing", "Spielen"), ph("sing", 1.0, 3.0, "Singing the verse", "Strophe singen"), ph("back", 3.0, 4.0, "Back to the first chord", "Zurück zum ersten Akkord")],
        [(1.8, "motion", ("Left hand arrives at the high chord", "Linke Hand erreicht den hohen Akkord"), None)], [("breathing", "step", [(0, 1), (1.0, 2), (3.0, 1)])]))
    c.append(clip_record("tune-up", "Tuning a string", "Eine Saite stimmen",
        "Non-looping 3.5 s: the player resting the guitar leans to the headstock, turns a peg and sits back.",
        "Nicht wiederholte Folge von 3,5 s: Der Spieler mit abgelegter Gitarre beugt sich zur Kopfplatte, dreht an einem Wirbel und lehnt sich zurück.",
        3.5, False, [(0, "resting", "smooth"), (1.0, "tuning", "smooth"), (2.4, "tuning", "smooth"), (3.4, "resting", "smooth")],
        [(a, 0, 3.5) for a in CONT], [ph("lean", 0, 1.0, "Leaning to the headstock", "Zur Kopfplatte beugen"), ph("turn", 1.0, 2.4, "Turning the peg", "Wirbel drehen"), ph("sit", 2.4, 3.5, "Sitting back", "Zurücklehnen")],
        [], [("breathing", "step", [(0, 0), (1.0, 1), (3.0, 0)])]))
    return c


def thermal():
    pats = [
        breath_pattern("rest", "Resting breathing", "Ruheatmung", 14, 0.5, 0.38, 0.04, 0.45, 0.13, "nose", "gupta-2010"),
        breath_pattern("play", "Playing, light activity", "Spielen, leichte Aktivität", 15, 0.7, 0.36, 0.03, 0.50, 0.11, "nose-and-mouth", "ainsworth-2011"),
        breath_pattern("sing", "Singing a long phrase", "Lange Gesangsphrase", 12, 1.1, 0.20, 0.03, 0.67, 0.10, "mouth", "ainsworth-2011"),
    ]
    env = {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1.0, "pressurePa": 101325, "groundTempC": 20}
    return {"environment": env, "baseMet": 1.4, "body": physiology(MASS, H, True, 42),
        "respiration": {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95, "patterns": pats, "motion": [{"bone": "torso", "angleDeg": 1.2, "dy": 0.004}, {"bone": "pelvis", "angleDeg": 0.5, "dy": 0.003}], "source": "authored-estimate", "reference": "hoeppe-1981"},
        "passengers": [], "operating": [], "regions": REG,
        "emitters": nose_mouth_emitters(-150, -100) + [plume_emitter("shirt-torso", 0.35)],
        "assumptions": [
            tx("DuBois skin area for 1.76 m and 74 kg; a flannel shirt over a T-shirt (0.39 clo trunk, 0.30 clo sleeves), denim jeans (0.26 clo), leather boots with socks, short hair and beard. Garment values are authored estimates guided by ISO 9920 (1 clo = 0.155 m2K/W); layer resistances include the enclosed air.", "DuBois-Hautfläche für 1,76 m und 74 kg; Flanellhemd über T-Shirt (0,39 clo Rumpf, 0,30 clo Ärmel), Jeans (0,26 clo), Lederstiefel mit Socken, kurzes Haar und Bart. Die Kleidungswerte sind Autorenschätzungen nach ISO 9920 (1 clo = 0,155 m2K/W); die Schichtwiderstände enthalten die eingeschlossene Luft."),
            tx("Metabolic rate 1.2 met resting, 1.7 met playing, 1.8 met singing while playing (Compendium of Physical Activities: playing guitar about 2 METs; mapped to thermal met as an estimate). Basal metabolism by Mifflin-St Jeor for a 42-year-old man.", "Stoffwechsel 1,2 met in Ruhe, 1,7 met beim Spielen, 1,8 met beim Singen und Spielen (Compendium of Physical Activities: Gitarre spielen etwa 2 MET; als Schätzung auf thermisches met übertragen). Grundumsatz nach Mifflin-St Jeor für einen 42-jährigen Mann."),
            tx("Indoor reference room at 21 degrees Celsius without sun; the seat conducts through 35 % of the jeans area, the guitar and stool surfaces warm by contact at about 31 degrees Celsius. Singing uses long mouth exhalations (12 per minute, 1.1 L); all respiration numbers are authored estimates and the first pattern (rest) is the reference of the chest motion.", "Innenraum als Referenz mit 21 Grad Celsius ohne Sonne; der Sitz leitet über 35 % der Jeansfläche, Gitarre und Hocker erwärmen sich durch Kontakt auf etwa 31 Grad Celsius. Singen nutzt lange Mundausatmungen (12 pro Minute, 1,1 L); alle Atemwerte sind Autorenschätzungen und das erste Muster (Ruhe) ist die Referenz der Brustbewegung.")]}


PENS = {500: dict(body=0.15, detail=None, texture=None, glazing=None, ground=0.2), 200: dict(body=0.18, detail=0.13, texture=None, glazing=None, ground=0.25), 100: dict(body=0.2, detail=0.13, texture=0.13, glazing=0.13, ground=0.3), 50: dict(body=0.25, detail=0.18, texture=0.13, glazing=0.13, ground=0.35)}
VR = {500: (350, 1000), 200: (150, 350), 100: (75, 150), 50: (20, 75)}


def build():
    import guit_decisions
    shapes = guit_art.all_shapes()
    d = guit_decisions.decisions()
    reps = {str(sc): rep_authored(sc, VR[sc], PENS[sc], guit_decisions.HATCHES.get(sc, []), shapes[sc], d[sc]) for sc in (500, 200, 100, 50)}
    for sc in (50, 100, 200, 500):
        if sc == 50:
            x0, y0, x1, y1 = bounds(P["strumming"], shapes[50])
            dims = {"heightM": round(y1 - y0, 3), "widthM": round(x1 - x0, 3), "depthM": 0.55, "massKg": round(MASS + 3.4 + 2.1, 2)}
            print("dims", dims)
        else:
            b = bounds(P["strumming"], shapes[sc])
            print(sc, "h", round(b[3] - b[1], 3), "w", round(b[2] - b[0], 3), "y0", round(b[1], 3))
    return {
        "id": "seated-guitarist", "category": "humans", "title": tx("Seated guitarist", "Sitzender Gitarrist"),
        "summary": tx("A 42-year-old man of 1.76 m in flannel shirt, jeans and leather boots on a wooden bar stool with one foot on the rung, playing an acoustic guitar: strumming, fretting high on the neck while singing, resting and tuning, with a strum loop, a chord-change phrase and a tuning sequence; four individually drawn scales from 1:500 to 1:50.", "Ein 42-jähriger, 1,76 m großer Mann in Flanellhemd, Jeans und Lederstiefeln sitzt auf einem hölzernen Barhocker mit einem Fuß auf der Fußraste und spielt Akustikgitarre: Anschlagen, hohes Greifen am Hals beim Singen, Ablegen und Stimmen, mit einer Anschlagschleife, einer Akkordwechsel-Phrase und einer Stimmfolge; vier einzeln gezeichnete Maßstäbe von 1:500 bis 1:50."),
        "tags": ["human", "musician", "guitar", "seated", "stool", "instrument", "adult"],
        "provenance": {"author": "Semio staffage content worker NW-H2", "method": "handcrafted", "created": "2026-10-29",
            "references": refs("de-leva-1996", "dubois-1916", "iso-9920-2007", "ainsworth-2011", "ashrae-2021", "gagge-1971", "gupta-2010", "hoeppe-1981", "mifflin-1990", "incropera-2011", extra=[{"id": "guitar-spec", "citation": "Typical dreadnought acoustic guitar: body about 0.5 m long and 0.38 m wide at the lower bout, scale length 0.645 m, mass about 2.1 kg; wooden bar stool with 0.56 m seat height and a foot rung at 0.25 m, mass about 3.4 kg. Dimensions are authored from catalogue ranges."}]),
            "notes": tx("Newly authored for semio.staffage/v2: a seated figure drawn in three-quarter projection, so the guitar neck is foreshortened to 0.78 m along its axis; poses are solved by inverse kinematics with the hands on anchors of the guitar.", "Neu erstellt für semio.staffage/v2: eine sitzende Figur in Dreiviertel-Projektion, daher ist der Gitarrenhals entlang seiner Achse auf 0,78 m verkürzt; die Posen sind per inverser Kinematik mit den Händen an Ankern der Gitarre gelöst.")},
        "dimensions": dims, "materials": MAT,
        "rig": {"kind": "anatomical", "root": "stool", "bones": bones}, "anchors": anchors,
        "representations": reps, "symbols": [], "poses": pose_list(), "clips": clips(), "thermal": thermal(),
        "structure": structure([("Inertial model: segment masses as point masses at de Leva centres, guitar 2.1 kg on the trunk and stool 3.4 kg; the stool legs on the floor are the permanent unilateral support, the seat and the rung foot rest on the stool.", "Trägheitsmodell: Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva, Gitarre 2,1 kg am Rumpf und Hocker 3,4 kg; die Hockerbeine auf dem Boden sind das einseitige Auflager, Sitz und Fuß auf der Fußraste ruhen auf dem Hocker.")]),
        "presentation": {"defaultPose": "strumming", "defaultClip": "strum-loop", "defaultScale": 100, "viewpoints": ["elevation"]},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}


if __name__ == "__main__":
    dump(asset_doc(build()), out_path("seated-guitarist"), depth=5)
    print("written")
