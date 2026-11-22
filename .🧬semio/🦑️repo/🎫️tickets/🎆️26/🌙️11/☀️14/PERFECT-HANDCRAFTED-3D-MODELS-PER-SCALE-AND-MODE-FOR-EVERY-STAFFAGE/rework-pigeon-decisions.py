"""📝 Pigeon second round: one decision per scale on the remiges, the hip bones and the solved wing poses (English first, German second)."""
import json, os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/animals/pigeon.json")
TEXT = {
    "50": (
        "Second round at 1:50: every remex is a true vane, a lens 0.9 mm thick and 19 to 27 mm wide from the quill to the rounded tip, declared as a sheet of its own material (flight feather vane) because a flight feather is not the 8 mm down layer of the body plumage; each feather bone rests turned 10 degrees about its shaft so that neighbouring vanes lie shingled instead of in one plane. The rachis members are rods of 1.6 to 1 mm. The hip joints hang from two hip bones that run from the synsacrum 5 mm beside the midline out and down to the acetabulum 20 mm beside it, each with its own member solid. The opened wing poses are solved from the direction of the humerus, the dorsal normal of the wing, the elbow and wrist angles and the spread of the two tracts, so the leading edge leads and the secondaries trail in every pose.",
        "Zweite Runde bei 1:50: Jede Schwungfeder ist eine echte Fahne, eine Linse von 0,9 mm Dicke und 19 bis 27 mm Breite von der Spule bis zur runden Spitze, als Platte aus eigenem Material (Fahne der Schwungfeder) erklärt, weil eine Schwungfeder nicht die 8 mm dicke Daunenschicht des Körpergefieders ist; jeder Federknochen ruht 10 Grad um seinen Schaft gedreht, sodass benachbarte Fahnen geschindelt statt in einer Ebene liegen. Die Federschäfte sind Stäbe von 1,6 bis 1 mm. Die Hüftgelenke hängen an zwei Hüftbeinen, die vom Synsacrum 5 mm neben der Mitte nach aussen und unten zur Gelenkpfanne 20 mm neben der Mitte laufen, jedes mit eigenem Tragglied. Die geöffneten Flügelstellungen sind aus der Richtung des Oberarms, der Rückennormale des Flügels, den Winkeln von Ellbogen und Handgelenk und der Spreizung der beiden Federfluren gelöst, sodass in jeder Stellung die Vorderkante führt und die Armschwingen nach hinten zeigen.",
    ),
    "100": (
        "Second round at 1:100: the twenty remiges per wing stay single vanes of 0.9 mm (19 to 27 mm wide, above the 15 mm feature) as sheets of the flight feather material; hand, shoulder lobe and scapulars are as thick as the 15 mm feature or the thinnest drawing of their bone. The rachis members keep their true section as slender rods, and the synsacrum member spans the two hip bones that carry the hip joints.",
        "Zweite Runde bei 1:100: Die zwanzig Schwungfedern je Flügel bleiben einzelne Fahnen von 0,9 mm (19 bis 27 mm breit, über dem Merkmal von 15 mm) als Platten aus dem Schwungfedermaterial; Hand, Schulterlappen und Schulterfedern sind so dick wie das Merkmal von 15 mm oder die dünnste Zeichnung ihres Knochens. Die Federschäfte behalten als schlanke Stäbe ihren wahren Querschnitt, und das Tragglied des Synsacrums überspannt die beiden Hüftbeine, an denen die Hüftgelenke hängen.",
    ),
    "200": (
        "Second round at 1:200: a 23 mm vane is below the 30 mm feature, so each tract is typified to five vanes of 32 mm on every second feather bone (primaries 2, 4, 6, 8, 10 and secondaries 1, 3, 5, 7, 9), 1.5 mm thick; they fan and fold on the same bones as the twenty feathers of 1:50. The scapulars are merged into the trunk. All twenty rachis members remain as slender rods, and the synsacrum member spans the two hip bones.",
        "Zweite Runde bei 1:200: Eine Fahne von 23 mm liegt unter dem Merkmal von 30 mm, daher ist jede Federflur auf fünf Fahnen von 32 mm auf jedem zweiten Federknochen typisiert (Handschwingen 2, 4, 6, 8, 10 und Armschwingen 1, 3, 5, 7, 9), 1,5 mm dick; sie fächern und falten auf denselben Knochen wie die zwanzig Federn von 1:50. Die Schulterfedern sind in den Rumpf eingegangen. Alle zwanzig Federschäfte bleiben als schlanke Stäbe, und das Tragglied des Synsacrums überspannt die beiden Hüftbeine.",
    ),
    "500": (
        "Second round at 1:500: each tract is typified to two vanes of 76 mm (the 75 mm feature) on primaries 3 and 8 and secondaries 3 and 8, 3.6 mm thick, so the wing still opens into a surface and folds onto the flank; rachis members stay slender rods and the synsacrum member spans the two hip bones.",
        "Zweite Runde bei 1:500: Jede Federflur ist auf zwei Fahnen von 76 mm (Merkmal 75 mm) auf den Handschwingen 3 und 8 und den Armschwingen 3 und 8 typisiert, 3,6 mm dick, sodass der Flügel weiterhin zu einer Fläche öffnet und an die Flanke faltet; die Federschäfte bleiben schlanke Stäbe und das Tragglied des Synsacrums überspannt die beiden Hüftbeine.",
    ),
}
text = open(SOURCE, encoding="utf-8").read()
source = json.loads(text)
for scale, (en, de) in TEXT.items():
    decisions = [d for d in source["volumes"][scale]["decisions"] if not d["en"].startswith("Second round")]
    decisions.append({"en": en, "de": de})
    source["volumes"][scale]["decisions"] = decisions
open(SOURCE, "w", encoding="utf-8").write(json.dumps(source, ensure_ascii=False) + ("\n" if text.endswith("\n") else ""))
print("decisions written")
