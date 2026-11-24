#!/bin/zsh
# 🛁️ Writes the repair ops of `used-bathtub` from the document of its generator before this repair: the shapes by make/used-bathtub.py, then the decision texts of 1:50 that describe them.
source "${0:A:h}/../env.sh"
export ASSET_FILE=$("$W/pristine.sh" used-bathtub "$U/../USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION/assets/used-bathtub/author-used-bathtub.py" | tail -1)
python3 "$W/make/used-bathtub.py" || exit 1
python3 "$W/texts.py" used-bathtub '[
["50","styles","waste and overflow holes 0.05","overflow hole 0.05 and in the plan the waste hole 0.05; in the front view the waste hole is an opening","Ablauf- und Überlaufloch 0,05","Überlaufloch 0,05 und im Grundriss das Ablaufloch 0,05; in der Vorderansicht ist das Ablaufloch eine Öffnung"],
["50","overlays","are both drawn from one definition;","are both drawn from one definition; in the front view the bands of wall and inside, the floor and the rim that the line of sight through the waste hole passes are prisms of their outline in the plane of the bath swept through their depth, the outline with a slot along the sight line, so the stored bath shows the hole as an opening and the rim with its round corners;","aus einer Definition gezeichnet;","aus einer Definition gezeichnet; in der Vorderansicht sind die Bänder von Wand und Innerem, der Boden und der Rand, durch die die Sichtlinie durch das Ablaufloch geht, Prismen ihres Umrisses in der Wannenebene über ihre Tiefe, der Umriss mit einem Schlitz entlang der Sichtlinie, so zeigt die gelagerte Wanne das Loch als Öffnung und den Rand mit seinen runden Ecken;"]
]'
