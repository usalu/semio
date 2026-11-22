"""📝 worker-carrying-box, second round pass 1: the decisions of every scale state the girdles on their own bones and the joined skeleton (English first, German second).

Usage: python3 rework-wcb-texts.py    rewrites `decisions` in the volumes source; publish with `volumes-insert.py`

@see ./rework-wcb-girdles.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "worker-carrying-box.json")
MARK = "Rework 2026-11-22, girdles:"
TEXTS = {
    "50": (
        "the rig carries a clavicle on each side from the sternoclavicular joint 22 mm beside the midline to the acromioclavicular joint 0.176 m out, 0.166 m long, running 19 degrees backwards and 10 degrees upwards, and a hip bone on each side from the sacroiliac joint to the hip joint, 0.106 m; upper arms and thighs hang from these bones at unchanged places. The clavicle is an S-curved loft, convex forwards in its medial two thirds and backwards in its lateral third, with a thick sternal and a flat acromial end; scapula, scapular spine and glenoid ride on the clavicle bone, the acromion ends 12 mm beyond the joint so that the breadth over the acromia is 0.39 m. The iliac body runs from the auricular surface on the sacrum into the roof of the socket and closes the pelvic ring; ilium plates, rami, ischium and socket ride on the hip bone. Sternoclavicular, sacroiliac and jaw joints are drawn, the last one seating the condyle of the mandible under the cranium.",
        "Das Rig führt je Seite ein Schlüsselbein vom Brustbein-Schlüsselbein-Gelenk 22 mm neben der Mitte zum Schultereckgelenk 0,176 m außen, 0,166 m lang, 19 Grad nach hinten und 10 Grad nach oben laufend, und je Seite ein Hüftbein vom Kreuz-Darmbein-Gelenk zum Hüftgelenk, 0,106 m; Oberarme und Oberschenkel hängen an diesen Knochen an unveränderter Stelle. Das Schlüsselbein ist ein S-förmig geschwungener Loft, in den inneren zwei Dritteln nach vorn und im äußeren Drittel nach hinten gewölbt, mit dickem Brustbeinende und flachem Schulterende; Schulterblatt, Schulterblattgräte und Gelenkpfanne sitzen auf dem Schlüsselbeinknochen, das Akromion endet 12 mm jenseits des Gelenks, sodass die Breite über die Schulterhöhen 0,39 m beträgt. Der Darmbeinkörper läuft von der Gelenkfläche am Kreuzbein in das Pfannendach und schließt den Beckenring; Darmbeinplatten, Äste, Sitzbein und Pfanne sitzen auf dem Hüftbein. Brustbein-Schlüsselbein-, Kreuz-Darmbein- und Kiefergelenk sind gezeichnet, das letzte setzt den Gelenkfortsatz des Unterkiefers unter den Hirnschädel.",
    ),
    "100": (
        "clavicles and hip bones are bones of the rig as at 1:50. The clavicle is an S-curved loft of at least 15 mm, the iliac body joins sacrum and hip joint, sternoclavicular, sacroiliac and jaw joints are drawn, and the tarsus stands between talus and metatarsal fan so that the load path reaches the forefoot without a gap.",
        "Schlüsselbeine und Hüftbeine sind wie in 1:50 Knochen des Rigs. Das Schlüsselbein ist ein S-förmiger Loft von mindestens 15 mm, der Darmbeinkörper verbindet Kreuzbein und Hüftgelenk, Brustbein-Schlüsselbein-, Kreuz-Darmbein- und Kiefergelenk sind gezeichnet, und die Fußwurzel steht zwischen Sprungbein und Mittelfußfächer, sodass der Lastpfad den Vorfuß ohne Lücke erreicht.",
    ),
    "200": (
        "the single bar across the shoulders is replaced by two clavicles of 30 mm on their own bones from the sternum to the acromion, and an iliac body of 34 mm on each hip bone joins sacrum and hip joint. The three rib hoops per side end on the sternum through their cartilage. Fibula, ulna and radius keep their true shafts as slender members; knee, elbow and C7 joints and the humeral condyle are 30 mm masses, one atlanto-occipital joint stands for both condyles and the patella is merged into the knee.",
        "Der eine Stab über die Schultern ist durch zwei 30 mm starke Schlüsselbeine auf eigenen Knochen vom Brustbein zum Akromion ersetzt, und je Hüftbein verbindet ein 34 mm starker Darmbeinkörper Kreuzbein und Hüftgelenk. Die drei Rippenbögen je Seite enden über ihren Knorpel am Brustbein. Wadenbein, Elle und Speiche behalten ihre wahren Schäfte als schlanke Stäbe; Knie-, Ellbogen- und C7-Gelenk sowie die Oberarmrolle sind 30-mm-Massen, ein Kopfgelenk steht für beide Gelenkhöcker und die Kniescheibe ist im Knie aufgegangen.",
    ),
    "500": (
        "the shoulder girdle is two clavicle bars of 75 mm on their own bones and each hip bone a 75 mm bar from the sacrum to the hip joint. Femur, tibia, humerus, ulna and radius keep their true shafts as slender members; heads, condyles, trochanters, patellae and the fibula are merged into joint masses of 75 mm at hip and knee, 60 mm at the shoulder and 48 mm at the elbow, and the cervical spine is one 75 mm bar.",
        "Der Schultergürtel besteht aus zwei 75 mm starken Schlüsselbeinstäben auf eigenen Knochen, jedes Hüftbein ist ein 75-mm-Stab vom Kreuzbein zum Hüftgelenk. Oberschenkelknochen, Schienbein, Oberarmknochen, Elle und Speiche behalten ihre wahren Schäfte als schlanke Stäbe; Köpfe, Kondylen, Rollhügel, Kniescheiben und das Wadenbein sind in Gelenkmassen von 75 mm an Hüfte und Knie, 60 mm an der Schulter und 48 mm am Ellbogen aufgegangen, und die Halswirbelsäule ist ein 75-mm-Stab.",
    ),
}
STALE = {
    "200": (
        ("one bar for the shoulder girdle with the scapulae, sacrum with the hip bones and one bar for the pubic arch, femur, tibia with the fibula merged,", "two clavicles with the scapulae, sacrum with the hip bones and one bar for the pubic arch, femur, tibia and fibula,"),
        ("ein Stab für den Schultergürtel mit den Schulterblättern, Kreuzbein mit den Hüftbeinen und ein Stab für den Schambogen, Oberschenkelknochen, Schienbein mit eingeschlossenem Wadenbein,", "zwei Schlüsselbeine mit den Schulterblättern, Kreuzbein mit den Hüftbeinen und ein Stab für den Schambogen, Oberschenkelknochen, Schien- und Wadenbein,"),
    ),
}

source = json.load(open(SOURCE, encoding="utf8"))
for scale, (en, de) in TEXTS.items():
    decisions = [d for d in source["volumes"][scale]["decisions"] if not d["en"].startswith(MARK)]
    for old_en, new_en in STALE.get(scale, ())[:1]:
        (old_de, new_de) = STALE[scale][1]
        for d in decisions:
            if old_en in d["en"]:
                if old_de not in d["de"]:
                    raise SystemExit("German sentence not found")
                d["en"], d["de"] = d["en"].replace(old_en, new_en), d["de"].replace(old_de, new_de)
                print("1:%s stale sentence rewritten" % scale)
    decisions.append({"en": f"{MARK} {en}", "de": "Überarbeitung 2026-11-22, Gürtel: " + de})
    source["volumes"][scale]["decisions"] = decisions
open(SOURCE, "w", encoding="utf8").write(json.dumps(source, ensure_ascii=False, separators=(",", ":")) + "\n")
print("decisions written")
