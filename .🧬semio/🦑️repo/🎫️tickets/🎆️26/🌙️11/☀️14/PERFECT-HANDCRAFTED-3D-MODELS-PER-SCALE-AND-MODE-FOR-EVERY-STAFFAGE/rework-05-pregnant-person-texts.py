"""📝 pregnant-person: the structure decisions of every scale say what the rig and the load path are now.

Usage: python3 rework-05-pregnant-person-texts.py    edits `volumes/humans/pregnant-person.json` (run once)
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "pregnant-person.json")
spec = importlib.util.spec_from_file_location("rig", os.path.join(T, "rework-05-pregnant-person-rig.py"))
RIG = importlib.util.module_from_spec(spec)
spec.loader.exec_module(RIG)

RIG_EN = " The rig follows the anatomy: lumbar spine (L5/S1 to T12/L1, 0.17 m) below the thorax, clavicles of 0.148 m running laterally from the sternoclavicular to the acromioclavicular joint and carrying scapula, glenoid and arm, hip bones of 0.090 m from the sacroiliac joint to the hip joint carrying the thighs, and a foot bone from the ankle to the ball."
RIG_DE = " Das Rig folgt der Anatomie: Lendenwirbelsäule (L5/S1 bis T12/L1, 0,17 m) unter dem Brustkorb, Schlüsselbeine von 0,148 m, die seitlich vom Brustbein-Schlüsselbein-Gelenk zum Schultereckgelenk laufen und Schulterblatt, Gelenkpfanne und Arm tragen, Hüftbeine von 0,090 m vom Kreuzbein-Darmbein-Gelenk zum Hüftgelenk, an denen die Oberschenkel hängen, und ein Fußknochen vom Sprunggelenk zum Ballen."
PATH = {
    "50": (" The load path is closed: iliac body from the sacroiliac joint to the acetabulum under the arched iliac crest, acromion on the clavicle, scapular neck under the glenoid, every rib head on its vertebral body, patella on the femoral trochlea with its ligament to the tibia; the fetal skeleton is one piece (spine into the skull base, shoulders on the rib cage) and lies in the uterine wall, a muscular band of 8 mm in the median plane that carries it on the lumbar spine.", " Der Lastpfad ist geschlossen: Darmbeinkörper vom Kreuzbein-Darmbein-Gelenk zur Hüftpfanne unter dem gewölbten Darmbeinkamm, Schulterhöhe am Schlüsselbein, Schulterblatthals unter der Gelenkpfanne, jeder Rippenkopf an seinem Wirbelkörper, Kniescheibe auf der Gleitrinne des Oberschenkels mit ihrem Band zum Schienbein; das Skelett des Kindes ist ein Stück (Wirbelsäule in die Schädelbasis, Schultern am Brustkorb) und liegt in der Gebärmutterwand, einem Muskelband von 8 mm in der Mittelebene, das es an der Lendenwirbelsäule trägt."),
    "100": (" The load path is closed: iliac body from sacrum to hip joint, the girdle rod runs clavicle, acromion, glenoid, shoulder blade, the six typified ribs start in the spine, the femoral condyles reach the knee centre, the patella lies on them with its ligament, and the uterine wall (a band of 16 mm) carries the fetus on the lumbar spine.", " Der Lastpfad ist geschlossen: Darmbeinkörper vom Kreuzbein zum Hüftgelenk, der Schultergürtelstab läuft Schlüsselbein, Schulterhöhe, Gelenkpfanne, Schulterblatt, die sechs typisierten Rippen beginnen in der Wirbelsäule, die Oberschenkelrollen reichen bis zur Kniemitte, die Kniescheibe liegt mit ihrem Band darauf, und die Gebärmutterwand (ein Band von 16 mm) trägt das Kind an der Lendenwirbelsäule."),
    "200": (" The load path is closed: iliac body from sacrum to hip joint, the girdle rod runs clavicle, acromion, glenoid, shoulder blade, the three typified ribs start in the spine, the femur reaches the knee centre and the uterine wall (a band of 31 mm) carries the fetus on the lumbar spine.", " Der Lastpfad ist geschlossen: Darmbeinkörper vom Kreuzbein zum Hüftgelenk, der Schultergürtelstab läuft Schlüsselbein, Schulterhöhe, Gelenkpfanne, Schulterblatt, die drei typisierten Rippen beginnen in der Wirbelsäule, der Oberschenkelknochen reicht bis zur Kniemitte und die Gebärmutterwand (ein Band von 31 mm) trägt das Kind an der Lendenwirbelsäule."),
    "500": (" The spine is two rods (lumbar, thoracic) and each clavicle a bar of the smallest feature from the sternum to the shoulder.", " Die Wirbelsäule besteht aus zwei Stäben (Lende, Brust) und jedes Schlüsselbein ist ein Stab der kleinsten Darstellungsgröße vom Brustbein zur Schulter."),
}


def main():
    source = json.load(open(SOURCE, encoding="utf-8"))
    for scale, (en, de) in PATH.items():
        decisions = source["volumes"][scale]["decisions"]
        target = next(d for d in decisions if d["en"].lstrip().lower().startswith(("structure", "skeleton", "forces")))
        if "sacroiliac" in target["en"] or "two rods" in target["en"]:
            raise SystemExit("already edited")
        target["en"] += RIG_EN + en
        target["de"] += RIG_DE + de
        print(scale, target["en"][:90])
    open(SOURCE, "w", encoding="utf-8").write(RIG.DUMP({"volumes": source["volumes"]}, 0) + "\n")


main()
