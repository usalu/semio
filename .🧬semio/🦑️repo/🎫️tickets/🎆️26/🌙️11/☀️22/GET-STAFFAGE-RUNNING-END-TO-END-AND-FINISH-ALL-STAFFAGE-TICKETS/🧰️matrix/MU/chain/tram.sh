#!/bin/zsh
# 🚋️ The repair of `tram` in the chain of the lane: the fill-rule repair that ML-4 prepared (`../../ML-4/fill-rule.py tram`: the bogie legs of the 1:50 and 1:100 side elevation become one ring), run twice to show the second run changes nothing, then the cheap proofs: chain/tram.sh   (run it inside a grant)
source "${0:A:h}/../env.sh"
mkdir -p "$GEN/backup" "$GEN/apply"
[[ -f "$GEN/backup/tram.json" ]] || cp "$(asset tram)" "$GEN/backup/tram.json"
python3 "$U/🧰️matrix/ML-4/fill-rule.py" tram > "$GEN/apply/tram.txt" 2>&1; shasum "$(asset tram)" | cut -c1-12 >> "$GEN/apply/tram.txt"
python3 "$U/🧰️matrix/ML-4/fill-rule.py" tram > "$GEN/apply/tram.again.txt" 2>&1; shasum "$(asset tram)" | cut -c1-12 >> "$GEN/apply/tram.again.txt"
cat "$GEN/apply/tram.txt" "$GEN/apply/tram.again.txt"
"$W/steps.sh" scan,validate,print,check tram
(cd "$W" && "$B" sheet.ts --panel 700x300 "$GEN/look/tram.png" "$GEN/backup/tram.json|50|pose:running" "tram|50|pose:running" "tram|100|pose:running")
