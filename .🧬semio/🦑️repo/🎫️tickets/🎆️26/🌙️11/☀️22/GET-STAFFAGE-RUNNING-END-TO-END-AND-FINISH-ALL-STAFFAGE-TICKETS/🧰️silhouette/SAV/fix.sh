#!/bin/zsh
# 🤖️ Candidate repair of one asset of batch SAV, one process after the other: the greedy silhouette repair (`auto.ts`) at the given scales on the catalog file, continuing the ops of 🗑️generated/silhouette/SAV/ops/<id>.json; then the candidate document 🗑️generated/silhouette/SAV/cand/<id>.json (catalog file plus ops), every check of the guard on it, the three-views sweep of it and the product check CLI on it. Nothing is written to the catalog: fix.sh <id> <scale>…   (MAXSEC seconds per scale, default 240; `promote.sh` ships a candidate)
source "${0:A:h}/env.sh"
a="$1"; shift
f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
mkdir -p "$GEN/ops" "$GEN/auto" "$GEN/cand"
[[ -f "$GEN/ops/$a.json" ]] || { [[ -f "$W/repair/$a.json" ]] && cp "$W/repair/$a.json" "$GEN/ops/$a.json"; }
cd "$S/📦️packages/🟦️typescript"
for s in "$@"; do
  MAXSEC=${MAXSEC:-240} timeout 900 "$B" "$W/auto.ts" "$f" $s "$GEN/ops/$a.json" ${MAXITER:-60} "$GEN/ops/$a.json" > "$GEN/auto/$a-$s.log" 2>&1
  echo "$a 1:$s $(grep 'issues ' "$GEN/auto/$a-$s.log" | tail -1 | cut -c9-); $(tail -1 "$GEN/auto/$a-$s.log" | cut -c9-200)"
done
timeout 300 "$B" "$W/apply.ts" "$a" --ops "$GEN/ops/$a.json" --out "$GEN/cand/$a.json"
timeout 900 "$B" "$W/measure.ts" "$GEN/cand/$a.json"
(cd "$U/🧰️three-views" && timeout 900 "$B" tv1-sweep.ts --file "$GEN/cand/$a.json" | tail -4 | cut -c1-300)
(cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$GEN/cand/$a.json" 2>&1 | grep -E "^(✓|✗)|UNSUITABLE|^  [a-z]" | cut -c1-260 | head -8)
