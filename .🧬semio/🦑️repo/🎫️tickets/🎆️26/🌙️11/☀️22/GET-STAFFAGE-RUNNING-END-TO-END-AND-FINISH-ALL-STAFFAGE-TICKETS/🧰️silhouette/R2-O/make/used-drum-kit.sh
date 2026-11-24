#!/bin/zsh
# 🥁️ Writes the repair ops of `used-drum-kit` from the document of its generator before this repair: the skirts by make/used-drum-kit.py, then the cymbals redrawn from the body (outline.ts).
source "${0:A:h}/../env.sh"
A=used-drum-kit; O="$W/ops/$A.json"
export ASSET_FILE=$("$W/pristine.sh" $A "$U/../USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/assets/$A/author-drum-kit.py" | tail -1)
python3 "$W/make/$A.py" || exit 1
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 900 "$B" "$W/outline.ts" "$ASSET_FILE" "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
for d in 50 100; do for v in elevation front; do ol $d $v "crash-cymbal=crash-cymbal" "ride-cymbal=ride-cymbal" "hihat-top=hihat-top" "hihat-bottom=hihat-bottom"; done; done
python3 "$W/texts.py" used-drum-kit '[
["50","retained","the cymbals in their true bowed section with the bell","the cymbals as the outline of their bowed body with the bell","die Becken in ihrem wahren gewölbten Schnitt mit der Kuppe","die Becken als Umriss ihres gewölbten Körpers mit der Kuppe"],
["50","thinElements","cymbals and heads are true-thickness sections inside the pen.","heads are true-thickness sections inside the pen; the cymbals are drawn by the outline of their bowed body, as the side shows them.","Becken und Felle sind Schnitte in wahrer Dicke innerhalb der Linie.","Felle sind Schnitte in wahrer Dicke innerhalb der Linie; die Becken sind mit dem Umriss ihres gewölbten Körpers gezeichnet, wie die Seite sie zeigt."],
["100","retained","cymbals in section","cymbals as the outline of their bowed body","Becken im Schnitt","Becken als Umriss ihres gewölbten Körpers"],
["50",0,"4 mm lid skirt)","4 mm lid skirt, modelled at the 7.5 mm feature and grown inwards into the wall)","4-mm-Deckelkrempe)","4-mm-Deckelkrempe, am 7,5-mm-Merkmal modelliert und nach innen in die Wand gewachsen)"],
["100",0,"Body: as 1:50 without","Body: as 1:50, the lid skirts at the 15 mm feature, without","Körper: wie 1:50 ohne","Körper: wie 1:50, die Deckelkrempen am 15-mm-Merkmal, ohne"]
]'
