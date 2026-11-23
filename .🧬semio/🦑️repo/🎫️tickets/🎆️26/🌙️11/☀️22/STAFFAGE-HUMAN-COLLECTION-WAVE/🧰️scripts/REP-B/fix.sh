#!/bin/zsh
# 🎯️ Continues the greedy silhouette repair of one person at the given scales directly on its repair file (`HW-1x/repair/<slug>.json`, whose ops are idempotent on the catalog file that already holds them) and applies the result to the catalog; logs in 🗑️generated/REP-B/auto-d. fix.sh <slug> <scale>…   (LIMIT=<m> a tighter largest distance, MAXSEC=<s> the time budget per scale)
source "${0:A:h}/env.sh"
a="$1"; shift
folder="$(grep -o "\"$a\": \"HW-1[0-9]\"" "$W/people.ts" | grep -o "HW-1[0-9]")"
[[ -z "$folder" ]] && { echo "$a: not a person of HW-10 … HW-18"; exit 1; }
ops="$K/🧰️scripts/$folder/repair/$a.json"
mkdir -p "$GEN/auto-d"
cd "$S/📦️packages/🟦️typescript"
for s in "$@"; do
  log="$GEN/auto-d/$a-$s.log"
  MAXSEC=${MAXSEC:-150} "$B" "$W/auto.ts" "$a" "$s" "$ops" ${MAXITER:-60} "$ops" > "$log" 2>&1
  echo "$a 1:$s $(grep 'issues ' "$log" | tail -1 | cut -c9-); $(tail -1 "$log" | cut -c9-200)"
done
(cd "$W" && "$B" apply.ts "$a") | tail -1
