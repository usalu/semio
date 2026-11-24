#!/bin/zsh
# 🎯️ Greedy silhouette repair of one person at the given scales on its repair file `ops/<slug>.json` (idempotent on the catalog file that already holds them), then applies the result to the catalog; logs in 🗑️generated/silhouette/SH-1/auto. fix.sh <slug> <scale>…   (LIMIT=<m> a tighter largest distance, MAXSEC=<s> the time budget per scale, NOAPPLY=1 keeps the catalog file)
source "${0:A:h}/env.sh"
export WHOLE=1
a="$1"; shift
ops="$W/ops/$a.json"
mkdir -p "$GEN/auto" "$GEN/backup"
[[ -f "$GEN/backup/$a.json" ]] || cp "$CAT/$a/🔣️.json" "$GEN/backup/$a.json"
cd "$S/📦️packages/🟦️typescript"
for s in "$@"; do
  log="$GEN/auto/$a-$s.log"
  MAXSEC=${MAXSEC:-240} timeout 900 "$B" "$W/auto.ts" "$GEN/backup/$a.json" "$s" "$ops" ${MAXITER:-60} "$ops" > "$log" 2>&1
  echo "$a 1:$s $(grep 'issues ' "$log" | tail -1 | cut -c9-); $(tail -1 "$log" | cut -c9-200)"
done
[[ -n "$NOAPPLY" ]] || "$W/keep.sh" "$a"
