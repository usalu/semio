#!/bin/zsh
# 🧪️ A candidate file of a plant against its state before this round (🗑️generated/silhouette/R4-P/backup/<id>.json), one process after the other: every silhouette check, `validateAsset` (issues new in the candidate), the three-views sweep (candidate and before), the print suitability and the sheets of the given scales; the catalog file stays as it is. trial.sh <id> <candidate.json> [scales for sheets…]
source "${0:A:h}/env.sh"; mkdir -p "$GEN/trial"
a="$1"; f="$2"; shift 2
"$W/t.sh" measure.ts "$f" | cut -c1-330
[[ -s "$GEN/trial/$a-validate-before.txt" ]] || "$W/t.sh" validate.ts "$GEN/backup/$a.json" "$GEN/trial/$a-validate-before.txt" | head -1
N=6 "$W/t.sh" validate.ts "$f" "$GEN/trial/$a-validate.txt"
echo "validateAsset issues new in the candidate: $(diff "$GEN/trial/$a-validate-before.txt" "$GEN/trial/$a-validate.txt" | grep -c '^>')"
[[ -s "$GEN/trial/$a-sweep-before.txt" ]] || N=3 "$W/t.sh" sweep.ts "$GEN/backup/$a.json" > "$GEN/trial/$a-sweep-before.txt" 2>&1
echo "sweep before: $(head -1 "$GEN/trial/$a-sweep-before.txt" | cut -c1-200)"
N=8 "$W/t.sh" sweep.ts "$f" | cut -c1-330
N=4 "$W/t.sh" suit.ts "$f" 2>&1 | cut -c1-300
for s in "$@"; do "$W/look.sh" cand "$a" "$s" "$f" | sed 's|.*/sheets/||'; done
