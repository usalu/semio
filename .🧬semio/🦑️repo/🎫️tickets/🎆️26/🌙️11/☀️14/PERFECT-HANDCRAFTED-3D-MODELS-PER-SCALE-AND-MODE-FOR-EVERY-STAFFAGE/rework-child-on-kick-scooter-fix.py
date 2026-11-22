"""🦴 Joint anatomy of `humans/child-on-kick-scooter` that closes the load path: manubrium, temporomandibular condyles, arcuate line and acetabulum, metatarsal heads, patella in its groove, hands on the grips. Called by `rework-child-on-kick-scooter-build.py`."""
import copy
import re


def after(structure, solid_id, solids):
    at = next(i for i, s in enumerate(structure) if s["id"] == solid_id) + 1
    structure[at:at] = solids


def fix(asset, volumes, old, new, kit):
    for scale, rod, thin in (("50", 0.008, 0.005), ("100", 0.008, 0.0075), ("200", 0.015, 0.015)):
        structure = volumes[scale]["structure"]
        ids = kit.by_id(structure)
        length = next(b for b in asset["rig"]["bones"] if b["id"] == "ilium-l")["lengthM"]
        for side, (sign, _) in kit.SIDES.items():
            bone = f"ilium-{side}"
            after(structure, bone, [
                kit.capsule(f"arcuate-line-{side}", bone, [-0.008, 0, 0], [length, 0, 0], rod),
                kit.ball(f"acetabulum-{side}", bone, [length, 0, 0], [0.021, 0.021, 0.021]),
            ])
        manubrium = kit.ball("manubrium", "torso", [0.31, -0.042, 0], [0.016, 0.008 if scale != "200" else 0.015, 0.022])
        if scale == "200":
            sternum = ids["open-sternum"]
            top = copy.deepcopy(sternum["sections"][0])
            top["at"] = [0.3, -0.04, 0]
            sternum["sections"].insert(0, top)
            after(structure, "open-sternum", [manubrium])
            continue
        after(structure, "sternum", [manubrium])
        for side, (sign, _) in kit.SIDES.items():
            after(structure, f"mandible-ramus-{side}", [kit.capsule(f"mandible-condyle-{side}", "head", [0, -0.01, sign * 0.036], [0.012, -0.008, sign * 0.04], thin)])
            ids[f"patella-{side}"]["transform"]["translation"] = [0.292, 0.026, 0]
            tarsus = ids[f"tarsus-{side}"]
            head = copy.deepcopy(tarsus["sections"][-1])
            head["at"] = [0.113, 0.0125, 0]
            tarsus["sections"].append(head)
    structure = volumes["50"]["structure"]
    after(structure, "st-handlebar", [{"id": f"st-grip-{side}", "of": f"grip-{side}", "member": "frame", "part": "member"} for side in kit.SIDES])
    texts(volumes)


GIRDLE_EN = "The clavicles and hip bones are bones of the rig on a lateral rest placement: each clavicle runs from the manubrium to the acromion and carries scapula, scapular spine, acromion and glenoid; each hip bone runs from the sacroiliac joint along the arcuate line to the acetabulum and carries ilium, pubis and ischium, so a shrug or a pelvic tilt moves the limb with its girdle. The mandibular condyles seat in the skull base, the patella lies in the femoral groove and the metatarsal heads meet the toes."
GIRDLE_DE = "Schlüsselbeine und Hüftbeine sind Knochen des Rigs in seitlicher Ruhelage: Jedes Schlüsselbein läuft vom Manubrium zum Akromion und trägt Schulterblatt, Schulterblattgräte, Akromion und Gelenkpfanne; jedes Hüftbein läuft vom Iliosakralgelenk entlang der Linea arcuata zur Hüftpfanne und trägt Darmbein, Schambein und Sitzbein, sodass ein Schulterzucken oder eine Beckenneigung die Gliedmaße mit ihrem Gürtel bewegt. Die Kiefergelenkköpfe sitzen in der Schädelbasis, die Kniescheibe liegt in der Furche des Oberschenkelknochens und die Mittelfußköpfe treffen die Zehen."
DECISIONS = {
    ("50", 2, "en"): ("1:50 structure (122 solids)", "1:50 structure (139 solids)"),
    ("50", 2, "de"): ("(122 Körper)", "(139 Körper)"),
    ("100", 2, "en"): ("1:100 structure (101 solids)", "1:100 structure (114 solids)"),
    ("100", 2, "de"): ("(101 Körper)", "(114 Körper)"),
    ("100", 0, "en"): ("tyre, rim, spokes and hub", "tyre, rim, one web disc for the six spoke arms and hub"),
    ("100", 0, "de"): ("Reifen, Felge, Speichen und Nabe", "Reifen, Felge, einer Radscheibe für die sechs Speichenarme und Nabe"),
    ("100", 3, "en"): "The 145 × 30 mm wheels retain tyre, rim and hub; the six 9 mm spoke arms are one 15 mm web disc, because a single arm is below the smallest feature of this scale. The front crown spans the blades above the tread and both axles seat in the dropouts.",
    ("100", 3, "de"): "Die Räder 145 × 30 mm behalten Reifen, Felge und Nabe; die sechs 9 mm starken Speichenarme sind eine 15 mm starke Radscheibe, weil ein einzelner Arm unter dem kleinsten Merkmal dieses Maßstabs liegt. Die Gabelkrone überspannt die Gabelblätter über der Lauffläche, beide Achsen sitzen in den Ausfallenden.",
    ("200", 0, "en"): "1:200 body (34 solids), authored anew for 30 mm: one loft per limb segment, the hand a closed fist over the grip, shoe and toe as two plain masses, hip and seat merged under the jacket hem, neck with collar, head without nose or ears, helmet, ponytail, one backpack mass. Scooter: slotted deck, brake spoon, one stem tube with the head tube, gooseneck, fork crown, one fork solid across both blades with the wheel running in it, handlebar with grips, each wheel a tyre on one centre disc.",
    ("200", 0, "de"): "1:200-Körper (34 Körper), neu für 30 mm modelliert: ein Loft je Gliedmaßensegment, die Hand als geschlossene Faust um den Griff, Schuh und Zehen als zwei schlichte Massen, Hüfte und Gesäß unter dem Jackensaum zusammengefasst, Hals mit Kragen, Kopf ohne Nase und Ohren, Helm, Pferdeschwanz, eine Rucksackmasse. Roller: geschlitztes Trittbrett, Löffelbremse, ein Lenkrohr mit Steuerrohr, Rahmenhals, Gabelkrone, ein Gabelkörper über beide Gabelblätter, in dem das Rad läuft, Lenker mit Griffen, jedes Rad ein Reifen auf einer Radscheibe.",
    ("200", 1, "en"): ("(44 solids)", "(37 solids)"),
    ("200", 1, "de"): ("(44 Körper)", "(37 Körper)"),
    ("200", 2, "en"): "1:200 structure (69 solids): skull, facial skeleton with jaw, spine, open ribs with sternum and manubrium, a clavicle on each clavicle bone, pelvis with ilium, arcuate line and acetabulum on each hip bone, and one bone per limb segment at its true shaft section (declared slender), joints at the large joints, masses, deck, gooseneck, stem, fork, handlebar and wheels as members, two roller supports. The patella is below the feature and left out.",
    ("200", 2, "de"): "1:200-Tragwerk (69 Körper): Schädel, Gesichtsschädel mit Kiefer, Wirbelsäule, offene Rippen mit Brustbein und Manubrium, ein Schlüsselbein je Schlüsselbeinknochen, Becken mit Darmbein, Linea arcuata und Hüftpfanne je Hüftbein und ein Knochen je Gliedmaßensegment mit seinem wahren Schaftquerschnitt (als schlank erklärt), Gelenke an den großen Gelenken, Massen, Trittbrett, Rahmenhals, Lenkstange, Gabel, Lenker und Räder als Stäbe, zwei Rollenlager. Die Kniescheibe liegt unter dem kleinsten Merkmal und entfällt.",
    ("200", 3, "en"): "The 145 × 30 mm wheels are a tyre on one centre disc; rim, hub, spokes and axle are below 30 mm and merged into it. The fork is one solid 55 mm wide, the outer width of the two blades.",
    ("200", 3, "de"): "Die Räder 145 × 30 mm sind ein Reifen auf einer Radscheibe; Felge, Nabe, Speichen und Achse liegen unter 30 mm und gehen darin auf. Die Gabel ist ein 55 mm breiter Körper, die Außenbreite der beiden Gabelblätter.",
    ("500", 0, "en"): "1:500 body (25 solids), authored anew for 75 mm: three or four sections per mass; thigh, shank, shoe with toe, upper arm, forearm and fist per side, hip, trunk, shoulders with neck, head, helmet and backpack. The scooter has a slotted deck, a gooseneck, a stem, a handlebar, one fork solid and two disc wheels.",
    ("500", 0, "de"): "1:500-Körper (25 Körper), neu für 75 mm modelliert: drei bis vier Querschnitte je Masse; Oberschenkel, Unterschenkel, Schuh mit Zehen, Oberarm, Unterarm und Faust je Seite, Hüfte, Rumpf, Schultern mit Hals, Kopf, Helm und Rucksack. Der Roller hat ein geschlitztes Trittbrett, einen Rahmenhals, eine Lenkstange, einen Lenker, einen Gabelkörper und zwei Scheibenräder.",
    ("500", 1, "en"): ("(39 solids)", "(29 solids)"),
    ("500", 1, "de"): ("(39 Körper)", "(29 Körper)"),
    ("500", 2, "en"): "1:500 structure (41 solids): skull with facial skeleton, one spine rod at its true section from the sacrum to the atlas (it spans the neck bone), open ribs and sternum, one bar across both clavicles, one pelvis mass that spans both hip bones, humerus, radius, ulna, femur, tibia and fibula at their true shaft sections (declared slender), hand and foot masses, masses, deck, stem, fork, handlebar and wheels as members, two roller supports; joints and patella are below what the scale shows.",
    ("500", 2, "de"): "1:500-Tragwerk (41 Körper): Schädel mit Gesichtsschädel, ein Wirbelsäulenstab mit wahrem Querschnitt vom Kreuzbein bis zum Atlas (er überspannt den Halsknochen), offene Rippen und Brustbein, ein Stab über beide Schlüsselbeine, eine Beckenmasse über beide Hüftbeine, Oberarmknochen, Speiche, Elle, Oberschenkelknochen, Schienbein und Wadenbein mit wahren Schaftquerschnitten (als schlank erklärt), Hand- und Fußmassen, Massen, Trittbrett, Lenkstange, Gabel, Lenker und Räder als Stäbe, zwei Rollenlager; Gelenke und Kniescheibe liegen unter dem, was der Maßstab zeigt.",
    ("500", 3, "en"): "Each 145 mm wheel is one disc; rim, hub, spokes, axles and fork crown are merged into wheel and fork.",
    ("500", 3, "de"): "Jedes 145-mm-Rad ist eine Scheibe; Felge, Nabe, Speichen, Achsen und Gabelkrone gehen in Rad und Gabel auf.",
    ("500", 4, "en"): "At 1:500 the wheel disc is 72 mm wide, the width the drawings of this scale give it (the physical tread is 30 mm), and the fork is one 55 mm solid inside that width.",
    ("500", 4, "de"): "Bei 1:500 ist die Radscheibe 72 mm breit, so breit wie die Zeichnungen dieses Maßstabs sie zeigen (die wirkliche Lauffläche misst 30 mm), und die Gabel ist ein 55 mm breiter Körper innerhalb dieser Breite.",
}


def texts(volumes):
    for (scale, index, lang), change in DECISIONS.items():
        if change is None:
            continue
        entry = volumes[scale]["decisions"][index]
        if isinstance(change, tuple):
            if change[0] not in entry[lang]:
                raise SystemExit(f"decision {scale}/{index}/{lang} lacks {change[0]!r}")
            entry[lang] = entry[lang].replace(*change)
        else:
            entry[lang] = change
    for scale in ("50", "100"):
        volumes[scale]["decisions"].insert(3, {"en": GIRDLE_EN, "de": GIRDLE_DE})
    volumes["50"]["decisions"][3]["en"] += " The finger bones bear on the rubber grips, which are members of the frame."
    volumes["50"]["decisions"][3]["de"] += " Die Fingerknochen liegen auf den Gummigriffen auf, die Stäbe des Rahmens sind."
    names = {"body": ("body", "Körper"), "thermal": ("thermal", "Thermik"), "structure": ("structure", "Tragwerk")}
    for scale, volume in volumes.items():
        for name, (en, de) in names.items():
            count = len(volume[name])
            for entry in volume["decisions"]:
                entry["en"] = re.sub(rf"^(1:{scale} {en} \()\d+( solids\))", rf"\g<1>{count}\g<2>", entry["en"])
                entry["de"] = re.sub(rf"^(1:{scale}-{de} \()\d+( Körper\))", rf"\g<1>{count}\g<2>", entry["de"])
