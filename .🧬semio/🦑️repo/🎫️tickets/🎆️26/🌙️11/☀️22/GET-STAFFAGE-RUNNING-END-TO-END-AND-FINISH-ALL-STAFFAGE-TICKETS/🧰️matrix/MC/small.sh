#!/bin/zsh
# 🔬️ The small proofs of single assets on their catalog documents, one process after the other: the fixer's gate over every subject, the authors' gate `T/🔎️scan.ts`, `validateAsset` and the print check of every band and its range ends; one summary line per asset, logs in $GEN/log/small-*: small.sh <id>…   (run it through q.sh with 800 MB or inside a larger grant)
source "${0:h}/env.sh"
mkdir -p "$GEN/log"; cd "$T"
for a in "$@"; do
  timeout 1500 "$B" "$M/gate.ts" "$a" --soft > "$GEN/log/small-gate-$a.txt" 2>&1; echo "[DEBUG] gate $a exit $?" >> "$GEN/log/small-gate-$a.txt"
  timeout 1500 "$B" 🔎️scan.ts --language en "$a" > "$GEN/log/small-scan-$a.txt" 2>&1; scan=$?
  timeout 600 "$B" "$W/prints.ts" "$a" > "$GEN/log/small-prints-$a.txt" 2>&1
  (cd "$P" && timeout 600 "$B" "$W/validate.ts" "$a") > "$GEN/log/small-validate-$a.txt" 2>&1
  echo "$a | gate $(python3 "$W/gate-table.py" "$GEN/log/small-gate-$a.txt" "$a") | scan exit $scan: $(grep -E 'clean|hard finding' "$GEN/log/small-scan-$a.txt" | tail -1 | cut -c1-120) | validate $(cut -f2 "$GEN/log/small-validate-$a.txt" | head -1) | print: $(head -1 "$GEN/log/small-prints-$a.txt" | cut -f2)"
done
