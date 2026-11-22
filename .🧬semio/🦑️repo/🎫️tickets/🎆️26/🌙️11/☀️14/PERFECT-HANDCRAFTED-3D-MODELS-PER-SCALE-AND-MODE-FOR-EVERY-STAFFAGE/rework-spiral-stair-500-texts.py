"""🪜 Spiral stair 1:500 volume decisions: the infill as eighteen posts and the thermal posts with the true infill surface."""
import json

PATH = "volumes/components/spiral-stair.json"
SWAPS = (
    ("cannot be modelled rod by rod, because 52 rods of the 75 mm feature leave 26 mm between them and close into a wall; it is modelled as a guard of posts a model maker would set at this scale: one 75 mm post per tread at the newel's place on the tread (thirteen, 0.30 m apart, 0.23 m clear = three features) and two under each landing rail at the third points, and the drawing shows rail and the same posts as 0.15 mm lines.",
     "cannot be modelled rod by rod, because 52 rods of the 75 mm feature leave 26 mm between them: gaps below the feature merge, so the infill is the surface it forms, one 75 mm guard panel per tread that stands on the tread and ends in the sloping rail (fourteen, the first cut at the newel) and one panel under each landing rail, and the drawing shows the same panels as light fields under the rail line."),
    ("lässt sich nicht Stab für Stab modellieren, weil 52 Stäbe im Merkmal 75 mm nur 26 mm zwischen sich lassen und zu einer Wand zulaufen; sie ist als Pfostengeländer modelliert, wie es ein Modellbauer in diesem Maßstab setzt: ein 75-mm-Pfosten je Stufe an der Stelle des Antrittspfostens auf der Stufe (dreizehn, 0,30 m auseinander, 0,23 m licht = drei Merkmale) und zwei unter jedem Podesthandlauf in den Drittelspunkten, die Zeichnung zeigt Handlauf und dieselben Pfosten als 0,15-mm-Linien.",
     "lässt sich nicht Stab für Stab modellieren, weil 52 Stäbe im Merkmal 75 mm nur 26 mm zwischen sich lassen: Lücken unter dem Merkmal verschmelzen, die Füllung ist deshalb die Fläche, die sie bildet, ein 75-mm-Geländerfeld je Stufe, das auf der Stufe steht und im geneigten Handlauf endet (vierzehn, das erste am Antrittspfosten geschnitten), und ein Feld unter jedem Podesthandlauf, die Zeichnung zeigt dieselben Felder als helle Flächen unter der Handlauflinie."),
    ("the seventeen infill posts stand at the places of the body posts as their own slender rods of 38.7 mm, the diameter at which they carry exactly the surface of the 52 real 12 mm rods (1.92 m2), because the 75 mm posts of the body would count twice that; only the total area is compared at this scale.",
     "the infill is the 52 real 12 mm rods as slender solids inside the guard panels of the body (1.92 m2), because the closed panels would count more than four times the surface that exchanges heat; only the total area is compared at this scale."),
    ("die siebzehn Füllpfosten stehen an den Stellen der Körperpfosten als eigene schlanke Stäbe von 38,7 mm, dem Durchmesser, bei dem sie genau die Oberfläche der 52 wahren 12-mm-Stäbe tragen (1,92 m2), weil die 75-mm-Pfosten des Körpers das Doppelte zählen würden; verglichen wird in diesem Maßstab nur die Gesamtfläche.",
     "die Füllung sind die 52 wahren 12-mm-Stäbe als schlanke Körper innerhalb der Geländerfelder des Körpers (1,92 m2), weil die geschlossenen Felder mehr als das Vierfache der wärmetauschenden Oberfläche zählen würden; verglichen wird in diesem Maßstab nur die Gesamtfläche."),
    ("handrail, posts and infill posts, a 75 mm fixed disc", "handrail, posts and guard panels, a 75 mm fixed disc"),
    ("Handlauf, Pfosten und Füllpfosten, eine feste 75-mm-Scheibe", "Handlauf, Pfosten und Geländerfelder, eine feste 75-mm-Scheibe"),
)
source = json.load(open(PATH))
text = json.dumps(source["volumes"]["500"]["decisions"], ensure_ascii=False)
for old, new in SWAPS:
    assert old in text, old[:60]
    text = text.replace(old, new)
source["volumes"]["500"]["decisions"] = json.loads(text)
json.dump(source, open(PATH, "w"), ensure_ascii=False, indent=1)
