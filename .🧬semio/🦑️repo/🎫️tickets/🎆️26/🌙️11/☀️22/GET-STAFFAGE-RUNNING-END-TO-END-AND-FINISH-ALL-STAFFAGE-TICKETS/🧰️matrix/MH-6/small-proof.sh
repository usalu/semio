#!/bin/zsh
# 🧾️ Gate, official scan, silhouette with validate and sweep, and check CLI of one asset on the catalog file as it is, for use inside a grant that is already held (lane.sh); the lines are appended to $GEN/proof/<id>.out: small-proof.sh <id>
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"; a="$1"; d="$GEN/proof"
f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"; h="$(shasum "$f" | cut -c1-12)"
{
echo "## small proof of $a, file $h, $(date '+%d %H:%M')"
(cd "$T" && "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1); echo "gate exit $? $(tail -1 "$d/$a-gate.log")"
(cd "$T" && "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1); echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"
(cd "$S/📦️packages/🟦️typescript" && "$B" "$M/measure.ts" "$a" > "$d/$a-measure.log" 2>&1); grep -E "silhouette:|validate:|sweep:| 1:" "$d/$a-measure.log"
(cd "$S" && "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1); echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
echo "file unchanged during the proof: $([[ "$h" == "$(shasum "$f" | cut -c1-12)" ]] && echo yes || echo NO)"
} >> "$d/$a.out" 2>&1
tail -12 "$d/$a.out"
