#!/bin/zsh
# 🏖️ `sunbather-on-towel` once more after its ops changed, in one grant: trial with the silhouette guard, landing with the cheap proofs, the six suites, the fresh matrix; and the fresh matrix of `used-bentwood-dining-chairs`: chain/sun.sh
source "${0:A:h}/../env.sh"
a=sunbather-on-towel; out="$GEN/proof/sun.txt"
MEASURE=1 TAG=plan "$W/try.sh" '' $a > "$GEN/try/$a.plan.out" 2>&1
echo "trial: $(grep -c 'silhouette: OK' "$GEN/measure/$a.now") of 4 bands silhouette OK | $(grep '^hard findings' "$GEN/try/$a.plan.txt")" > "$out"
diff "$GEN/measure/$a.was" "$GEN/measure/$a.now" >> "$out"
"$W/land.sh" $a '50|pose:prone' '100|pose:prone' >> "$out" 2>&1
"$W/steps.sh" suites $a >> "$out" 2>&1
FRESH=1 CAP=2300 "$W/matrix1.sh" $a used-bentwood-dining-chairs
echo "[DEBUG] chain ended $(date +%H:%M)" >> "$out"
