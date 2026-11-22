"""✍️ person-with-rollator: second chisel pass on the volumes source after `rework-person-with-rollator-skeleton.py`: the coat shoulder is one broad low round from the neck base over the acromion into the sleeve head and the decisions say what the skeleton now is.

Usage: python3 rework-person-with-rollator-refine.py    rewrites `volumes/humans/person-with-rollator.json` (publish it with `volumes-insert.py`)
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "person-with-rollator.json")

SHOULDER = [0.038, 0.052, 0.062]
SHOULDER_CENTRE = (0.3716, 0.004, 0.12)

GIRDLES = {
    "en": "1:50 girdles: shoulder and pelvis hang on their own rig bones. Each clavicle is an S-curved bar from the sternoclavicular joint at the manubrium back and out to the acromion (0.147 m, 4 degrees up, 29 degrees back) and carries the scapula as a blade on the ribs with its spine, acromion, coracoid process and the neck to the glenoid, so the humeral head sits under the acromion and a shrug lifts the whole shoulder; each hip bone runs from the sacroiliac joint down, forward and out to the hip joint (0.100 m) and carries iliac wing, crest, acetabulum and the pubic and ischial rami, the pubic rami meeting at the symphysis. The mandible hangs in the temporomandibular joint under the cranium, the atlas reaches the occiput, the patella lies in its groove, the fibula meets the tibia below the plateau and ends as the outer malleolus. The coat has a soft round over each acromion, so the skeleton stays inside the body.",
    "de": "Gürtel 1:50: Schulter und Becken hängen an eigenen Knochen des Rigs. Jedes Schlüsselbein ist ein S-förmig geschwungener Stab vom Sternoklavikulargelenk am Brustbeingriff nach hinten außen zum Akromion (0,147 m, 4 Grad steigend, 29 Grad nach hinten) und trägt das Schulterblatt als Platte auf den Rippen mit Schultergräte, Akromion, Rabenschnabelfortsatz und dem Hals zur Gelenkpfanne, sodass der Oberarmkopf unter dem Akromion sitzt und ein Schulterzucken die ganze Schulter hebt; jedes Hüftbein läuft vom Iliosakralgelenk nach unten, vorn und außen zum Hüftgelenk (0,100 m) und trägt Darmbeinschaufel, Kamm, Hüftpfanne sowie Scham- und Sitzbeinast, die Schambeinäste treffen sich in der Symphyse. Der Unterkiefer hängt im Kiefergelenk unter dem Hirnschädel, der Atlas reicht ans Hinterhaupt, die Kniescheibe liegt in ihrer Rinne, das Wadenbein trifft das Schienbein unter dem Schienbeinkopf und endet als Außenknöchel. Der Mantel hat über jedem Akromion eine weiche Rundung, sodass das Skelett im Körper bleibt.",
}

STRUCTURE = {
    "100": {
        "en": "1:100 structure: a skeleton with merged parts on the same rig bones as 1:50 (spine, six pairs of ribs sloping down to the front, sternum; per side a straight clavicle from the sternum to the acromion, the scapula as a declared thin blade with one bar for spine and acromion and the neck to the glenoid; sacrum, per side a hip bone from the sacroiliac joint to the acetabulum with the iliac wing as a declared thin blade and the pubic and ischial rami; cranium and facial bones, cervical spine up to the occiput; femur with neck, trochanter, condyles and patella, tibia with plateau and inner malleolus, the fibula and the ulna at their true section as declared slender members, radius, a hand plate, two foot lofts), the tubes, seat, forks, axles and wheels of the rollator by reference, ball joints, support pads and fourteen masses. The torso shows six paired rib courses.",
        "de": "Tragwerk 1:100: ein Skelett mit zusammengefassten Teilen an denselben Rig-Knochen wie 1:50 (Wirbelsäule, sechs nach vorn abfallende Rippenpaare, Brustbein; je Seite ein gerades Schlüsselbein vom Brustbein zum Akromion, das Schulterblatt als erklärt dünne Platte mit einem Stab für Gräte und Akromion und dem Hals zur Pfanne; Kreuzbein, je Seite ein Hüftbein vom Iliosakralgelenk zur Hüftpfanne mit der Darmbeinschaufel als erklärt dünner Platte sowie Scham- und Sitzbeinast; Hirn- und Gesichtsschädel, Halswirbelsäule bis ans Hinterhaupt; Oberschenkelknochen mit Hals, Rollhügel, Kondylen und Kniescheibe, Schienbein mit Kopf und Innenknöchel, Wadenbein und Elle im wahren Querschnitt als erklärt schlanke Stäbe, Speiche, eine Handplatte, zwei Fußkörper), die Rohre, der Sitz, die Gabeln, Achsen und Räder des Rollators per Verweis, Kugelgelenke, Auflagerplättchen und vierzehn Massen. Der Rumpf zeigt sechs beidseitige Rippenzüge.",
    },
    "200": {
        "en": "1:200 thermal and structure: every region by reference to its body solids, the face as the front skin of the head and the beret as an open shell; the structure is skull, cervical bar up to the occiput, spine, three pairs of ribs sloping down to the front, sternum, per side the clavicle at its true 13 mm as a declared slender member and the scapula as a declared thin blade whose tip is the glenoid, sacrum, per side a 40 mm hip bone from the sacroiliac joint to the hip joint with the iliac wing as a thin blade and the two rami at the 30 mm floor; femur with neck, trochanter and condyles, tibia with plateau, and fibula, humerus, radius and ulna at their true shafts as declared slender members (patella and malleolus are below 30 mm and merged), hand and foot bars, the runs, seat, stub axles and wheels of the rollator by reference, joints, support pads and six masses. The wheels are tyre and one disc. The torso shows three paired rib courses.",
        "de": "Thermik und Tragwerk 1:200: jede Region per Verweis auf ihre Körper, das Gesicht als vordere Kopfhaut und die Mütze als offene Schale; das Tragwerk ist Schädel, Halsstab bis ans Hinterhaupt, Wirbelsäule, drei nach vorn abfallende Rippenpaare, Brustbein, je Seite das Schlüsselbein in wahren 13 mm als erklärt schlanker Stab und das Schulterblatt als erklärt dünne Platte, deren Spitze die Gelenkpfanne ist, Kreuzbein, je Seite ein 40 mm starkes Hüftbein vom Iliosakralgelenk zum Hüftgelenk mit der Darmbeinschaufel als dünner Platte und den beiden Ästen an der 30-mm-Grenze; Oberschenkelknochen mit Hals, Rollhügel und Kondylen, Schienbein mit Kopf sowie Wadenbein, Oberarmknochen, Speiche und Elle mit wahren Schäften als erklärt schlanke Stäbe (Kniescheibe und Knöchel liegen unter 30 mm und sind eingeschmolzen), Hand- und Fußstäbe, die Züge, der Sitz, die Steckachsen und die Räder des Rollators per Verweis, Gelenke, Auflagerplättchen und sechs Massen. Die Räder sind Reifen und eine Scheibe. Der Rumpf zeigt drei beidseitige Rippenzüge.",
    },
    "500": {
        "en": "1:500 thermal and structure: every region by reference, the face as the front skin of the head, the beret as an open shell; the structure is skull, neck and spine at the 75 mm floor, sternum and two rib pairs as slender bars, per side the clavicle at its true section and the scapula as a declared thin blade, a 75 mm sacrum and per side a 75 mm hip bone from the sacroiliac joint to the hip joint with the iliac wing as a thin blade; femur, tibia and humerus are one loft each from joint to joint with head, neck or plateau and condyles on the true shaft, radius and ulna keep their true shafts, all five as declared slender members (fibula, patella and malleoli are merged); hand and foot bars, the runs, seat slab and wheels of the rollator by reference, eight joints, two support pads and two masses. The wheels are tyre and one disc. The torso shows two paired rib courses and a sternal bar.",
        "de": "Thermik und Tragwerk 1:500: jede Region per Verweis, das Gesicht als vordere Kopfhaut, die Mütze als offene Schale; das Tragwerk ist Schädel, Hals und Wirbelsäule an der 75-mm-Grenze, Brustbein und zwei Rippenpaare als schlanke Stäbe, je Seite das Schlüsselbein im wahren Querschnitt und das Schulterblatt als erklärt dünne Platte, ein 75 mm starkes Kreuzbein und je Seite ein 75 mm starkes Hüftbein vom Iliosakralgelenk zum Hüftgelenk mit der Darmbeinschaufel als dünner Platte; Oberschenkelknochen, Schienbein und Oberarmknochen sind je ein Körper von Gelenk zu Gelenk mit Kopf, Hals oder Schienbeinkopf und Kondylen am wahren Schaft, Speiche und Elle behalten ihre wahren Schäfte, alle fünf als erklärt schlanke Stäbe (Wadenbein, Kniescheibe und Knöchel sind eingeschmolzen); Hand- und Fußstäbe, die Züge, die Sitzplatte und die Räder des Rollators per Verweis, acht Gelenke, zwei Auflagerplättchen und zwei Massen. Die Räder sind Reifen und eine Scheibe. Der Rumpf zeigt zwei beidseitige Rippenzüge und ein Brustbeinstab.",
    },
}

WHEELS = {
    "200": {"en": "Each of the four wheels is its polyurethane tyre, one nylon disc for rim and webs and the hub boss: the five webs are 13 mm thick, below the 30 mm this scale can show, so the openings between them are closed.", "de": "Jedes der vier Räder besteht aus seinem Polyurethanreifen, einer Nylonscheibe für Felge und Stege und dem Nabenansatz: Die fünf Stege sind 13 mm stark, unter den 30 mm, die dieser Maßstab zeigen kann, die Öffnungen dazwischen sind deshalb geschlossen."},
    "500": {"en": "Each of the four wheels is its polyurethane tyre, one nylon disc of the tyre width for rim and webs and the hub boss: the five webs are 13 mm thick, far below the 75 mm this scale can show, so the openings between them are closed.", "de": "Jedes der vier Räder besteht aus seinem Polyurethanreifen, einer Nylonscheibe in Reifenbreite für Felge und Stege und dem Nabenansatz: Die fünf Stege sind 13 mm stark, weit unter den 75 mm, die dieser Maßstab zeigen kann, die Öffnungen dazwischen sind deshalb geschlossen."},
}


def main():
    text = open(SOURCE, encoding="utf8").read()
    source = json.loads(text)
    for scale, volume in source["volumes"].items():
        for solid in volume["body"]:
            if solid["id"] in ("coat-shoulder-l", "coat-shoulder-r"):
                solid["radii"] = SHOULDER
                solid["transform"]["translation"] = [SHOULDER_CENTRE[0], SHOULDER_CENTRE[1], SHOULDER_CENTRE[2] if solid["id"].endswith("-l") else -SHOULDER_CENTRE[2]]
        decisions = volume["decisions"]
        if scale == "50":
            if not any(entry["en"].startswith("1:50 girdles") for entry in decisions):
                decisions.insert(4, GIRDLES)
        else:
            index = next(i for i, entry in enumerate(decisions) if "structure" in entry["en"][:30])
            decisions[index] = STRUCTURE[scale]
            if scale in WHEELS:
                decisions[next(i for i, entry in enumerate(decisions) if "wheels" in entry["en"][:40])] = WHEELS[scale]
    if open(SOURCE, encoding="utf8").read() != text:
        raise SystemExit("the source changed meanwhile: run again")
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("written", SOURCE)


if __name__ == "__main__":
    main()
