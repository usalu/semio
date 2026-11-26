#!/bin/zsh
# 🌳️ What the lane still owes after the matrix chain of the plants, in one grant of 2500 MB (the figure of the `🧊️three-views` suite, the largest run in it): the authors' gate over the 22 plants (rows $GEN/scan/trees.jsonl), the six suites with the name filter of the repaired assets ($GEN/proof/vt-*.log, verdicts $GEN/proof/suites.txt), the `🕳️fill-rule` suite for `tram`, and the matrix of every asset of the lane whose record is missing or older than its catalog file (the components above 700 MB included): chain/trees-rest.sh <repaired id>…
source "${0:A:h}/../env.sh"
mkdir -p "$GEN/scan" "$GEN/proof"
ids=($(awk -F'\t' '$1=="vegetation"{print $2}' "$U/🧰️matrix/batches/MU.tsv"))
(cd "$T" && timeout 20000 "$B" 🔎️scan.ts --language en --quiet --out "$GEN/scan/trees.jsonl" --report "$GEN/scan/trees.json" "${ids[@]}") > "$GEN/scan/trees.log" 2>&1
echo "scan trees exit $? | $(tail -1 "$GEN/scan/trees.log" | cut -c1-200)"
while pgrep -f "MU/worker.sh" > /dev/null; do sleep 30; done
"$W/steps.sh" suites "$@" > "$GEN/proof/suites.txt" 2>&1
(cd "$P" && timeout 3000 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🕳️fill-rule/🟦️.ts") > "$GEN/proof/vt-fill-rule.log" 2>&1
echo "fill-rule exit $? | $(grep -E '^ *(Test Files|Tests) ' "$GEN/proof/vt-fill-rule.log" | tr -s ' ' | tr '\n' ';')" >> "$GEN/proof/suites.txt"
CAP=2300 "$W/matrix1.sh" "${IDS[@]}"
echo "[DEBUG] chain ended $(date +%H:%M)"
