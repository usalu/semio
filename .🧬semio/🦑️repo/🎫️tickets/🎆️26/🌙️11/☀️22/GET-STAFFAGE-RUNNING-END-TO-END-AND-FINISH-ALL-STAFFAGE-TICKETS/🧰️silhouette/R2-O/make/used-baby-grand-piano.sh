#!/bin/zsh
# 🎹️ Writes the repair ops of `used-baby-grand-piano` from the document of its generator before this repair.
source "${0:A:h}/../env.sh"
export ASSET_FILE=$("$W/pristine.sh" used-baby-grand-piano "$U/../USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/assets/used-baby-grand-piano/author-piano.py" | tail -1)
python3 "$W/make/used-baby-grand-piano.py" || exit 1
python3 "$W/texts.py" used-baby-grand-piano '[
["50","thinElements","Straps (3 mm) and the lyre braces are slender solids drawn at their true width","The straps are declared sheets of 2.5 mm webbing drawn in their true outline, the lyre braces slender solids drawn at their true width","Gurte (3 mm) und Lyrastreben sind schlanke Körper in wahrer Breite","Die Gurte sind erklärte Platten aus 2,5-mm-Gurtband in wahrem Umriss, die Lyrastreben schlanke Körper in wahrer Breite"],
["50","retained","skid with slots and battens","skid with its open hand slots and battens","Brett mit Schlitzen und Leisten","Brett mit offenen Griffschlitzen und Leisten"],
["100","thinElements","Straps and braces stay slender at their true width","The straps are declared sheets of 2.5 mm webbing in their true outline and the braces stay slender at their true width","Gurte und Streben bleiben schlank in wahrer Breite","Die Gurte sind erklärte Platten aus 2,5-mm-Gurtband in wahrem Umriss, die Streben bleiben schlank in wahrer Breite"],
["500","thinElements","the straps and the long prop are slender bars drawn at their true width with the 0.2 mm pen","the straps are declared sheets of 2.5 mm webbing and the long prop is a slender bar drawn at its true width with the 0.2 mm pen","Gurte und langer Stab sind schlanke Stäbe in wahrer Breite mit dem 0,2-mm-Stift","die Gurte sind erklärte Platten aus 2,5-mm-Gurtband und der lange Stab ist ein schlanker Stab in wahrer Breite mit dem 0,2-mm-Stift"],
["500",0,"straps stay slender at their true section","the straps are declared sheets of 2.5 mm webbing","die Gurte bleiben schlank in wahrem Querschnitt","die Gurte sind erklärte Platten aus 2,5-mm-Gurtband"]
]'
