#!/bin/zsh
# 🏁️ The last chain of the lane in one grant: `sunbather-on-towel` (trial with the longer hand sweep, and with `--palm` instead when the silhouette guard of the trial is not OK at all four bands; then landed with its sheet), `tram` (chain/tram.sh), the six suites for both and the whole `🕳️fill-rule` suite ($GEN/proof/suites-last.txt), and the matrix of every asset whose record is missing or older than its catalog file: chain/last.sh
source "${0:A:h}/../env.sh"
a=sunbather-on-towel; out="$GEN/proof/last.txt"; : > "$out"
attempt() {
  python3 "$W/make/$a.py" $1 >> "$out" 2>&1
  MEASURE=1 TAG=last$1 "$W/try.sh" '' $a > "$GEN/try/$a.last$1.out" 2>&1
  echo "trial $1: $(grep -c 'silhouette: OK' "$GEN/measure/$a.now") of 4 bands silhouette OK | $(grep '^hard findings' "$GEN/try/$a.last$1.txt")" >> "$out"
  [[ $(grep -c 'silhouette: OK' "$GEN/measure/$a.now") == 4 ]] && cmp -s <(grep validate "$GEN/measure/$a.was") <(grep validate "$GEN/measure/$a.now")
}
if attempt "" || attempt --palm; then
  "$W/land.sh" $a '50|trans:prone>supine@0.5' '50|pose:supine' '100|pose:prone' '200|trans:prone>supine@0.25' '500|pose:supine' >> "$out" 2>&1
else echo "$a NOT LANDED" >> "$out"; fi
"$W/chain/tram.sh" >> "$out" 2>&1
"$W/steps.sh" suites $a tram > "$GEN/proof/suites-last.txt" 2>&1
(cd "$P" && timeout 3000 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🕳️fill-rule/🟦️.ts") > "$GEN/proof/vt-fill-rule.log" 2>&1
echo "fill-rule exit $? | $(grep -E '^ *(Test Files|Tests) ' "$GEN/proof/vt-fill-rule.log" | tr -s ' ' | tr '\n' ';')" >> "$GEN/proof/suites-last.txt"
CAP=2300 "$W/matrix1.sh" "${IDS[@]}"
echo "[DEBUG] chain ended $(date +%H:%M)" >> "$out"
