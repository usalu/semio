"""📝 Office swivel chair: decisions of the second-round pass (castor swivel stems, armrest straps), English first, German second."""
import json, pathlib

SOURCE = pathlib.Path(__file__).parent / "volumes/furniture/office-swivel-chair.json"
TEXT = {
    "50": ("Rig and joinery: every castor has its two real pivots, a swivel stem bone that hangs from the arm boss (axial twist is the castor swivel) and below it the wheel bone on the axle, 20 mm behind and 20 mm under the stem joint. Neck, swivel head, centre web, hood and the stem joint of the structure ride on the stem bone, tyres, rims, axle and floor contact on the wheel bone, so a swivelled castor keeps its housing over its wheels. The armrest straps run 40 mm further under the seat and lie against the underside of the seat pan, where they are screwed; before they hung 3 mm below it.",
           "Rig und Fügung: Jede Rolle hat ihre zwei wirklichen Drehpunkte, einen Schwenkzapfen-Knochen, der im Armauge hängt (die Achsdrehung ist das Schwenken der Rolle), und darunter den Rad-Knochen auf der Achse, 20 mm hinter und 20 mm unter dem Zapfengelenk. Hals, Drehkopf, Mittelsteg, Haube und das Zapfengelenk des Tragwerks sitzen auf dem Zapfen-Knochen, Reifen, Felgen, Achse und Bodenkontakt auf dem Rad-Knochen, sodass eine geschwenkte Rolle ihr Gehäuse über den Rädern behält. Die Armlehnenlaschen laufen 40 mm weiter unter den Sitz und liegen an der Unterseite der Sitzschale an, wo sie verschraubt sind; zuvor hingen sie 3 mm darunter."),
    "100": ("Rig: the swivel head and the centre body of every castor ride on its swivel stem bone, the two wheels on the wheel bone below it, so the castor swivels about its stem as at 1:50.",
            "Rig: Drehkopf und Mittelkörper jeder Rolle sitzen auf ihrem Schwenkzapfen-Knochen, die zwei Räder auf dem Rad-Knochen darunter, sodass die Rolle wie in 1:50 um ihren Zapfen schwenkt."),
    "200": ("Rig: the one wheel body of every castor hangs on the wheel bone under the swivel stem bone and trails 20 mm behind the stem, so it swings about the boss when the castor swivels. The armrest straps reach under the seat mass and touch it.",
            "Rig: Der eine Radkörper jeder Rolle hängt am Rad-Knochen unter dem Schwenkzapfen-Knochen und läuft 20 mm hinter dem Zapfen nach, sodass er beim Schwenken um das Auge wandert. Die Armlehnenlaschen reichen unter die Sitzmasse und berühren sie."),
    "500": ("Rig: the wheel body of every castor hangs on the wheel bone under the swivel stem bone, as at the finer scales; the armrest straps touch the seat mass.",
            "Rig: Der Radkörper jeder Rolle hängt am Rad-Knochen unter dem Schwenkzapfen-Knochen wie in den feineren Massstäben; die Armlehnenlaschen berühren die Sitzmasse."),
}
document = json.loads(SOURCE.read_text())
for scale, (en, de) in TEXT.items():
    decisions = [d for d in document["volumes"][scale]["decisions"] if not d["en"].startswith("Rig")]
    document["volumes"][scale]["decisions"] = decisions + [{"en": en, "de": de}]
SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
