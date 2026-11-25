#!/bin/zsh
# 🩺️ The proof of assets of batch MC on their catalog documents, one process after the other (run it in a queue grant): the boxes of every shape (boxes.ts into $GEN/boxes-after), the fixer's gate over every subject ($GEN/log/after-gate.txt), the authors' gate of the suite (`T/🔎️scan.ts`, $GEN/log/after-scan.*), `validateAsset`, the print check of every band and its range ends, the silhouette, validate and three-views measure of `../MR/measure.ts`, and with CHECK=1 the product check CLI per asset ($GEN/log/check-<id>.txt). One summary line per asset at the end: prove.sh <id>…   (2500 MB; with CHECK=1 5000 MB)
source "${0:h}/env.sh"
ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS)
mkdir -p "$GEN/log"; cd "$T"
timeout 1500 "$B" "$W/boxes.ts" "$GEN/boxes-after" $ids > "$GEN/log/after-boxes.txt" 2>&1
for a in $ids; do
  timeout 1500 "$B" "$M/gate.ts" "$a" --soft > "$GEN/log/after-gate-$a.txt" 2>&1
  echo "[DEBUG] gate $a exit $?" >> "$GEN/log/after-gate-$a.txt"
done
cat $GEN/log/after-gate-${^ids}.txt > "$GEN/log/after-gate.txt" 2>/dev/null
timeout 3000 "$B" 🔎️scan.ts --language en --fresh --out "$GEN/log/after-scan.jsonl" --report "$GEN/log/after-scan.json" $ids > "$GEN/log/after-scan.txt" 2>&1
echo "[DEBUG] scan exit $?" >> "$GEN/log/after-scan.txt"
timeout 900 "$B" "$W/prints.ts" $ids > "$GEN/log/after-prints.txt" 2>&1
(cd "$P" && timeout 900 "$B" "$W/validate.ts" $ids) > "$GEN/log/after-validate.txt" 2>&1
(cd "$P" && timeout 2400 "$B" "$M/measure.ts" $ids) > "$GEN/log/after-measure.txt" 2>&1
if [[ -n "$CHECK" ]]; then
  for a in $ids; do (cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/log/check-$a.txt" 2>&1; done
fi
for a in $ids; do
  echo "$a | gate $(python3 "$W/gate-table.py" "$GEN/log/after-gate.txt" "$a") | scan $(grep -m1 "\] $a " "$GEN/log/after-scan.txt" | sed -E 's/.*s · //; s/ · remaining.*//') | validate $(grep "^$a" "$GEN/log/after-validate.txt" | cut -f2) | print: $(grep "^$a" "$GEN/log/after-prints.txt" | cut -f2) | $(grep "^$a silhouette" "$GEN/log/after-measure.txt" | cut -d' ' -f2-) | check $(grep -m1 -E '^(✓|✗)' "$GEN/log/check-$a.txt" 2>/dev/null | cut -c1-80)"
done
