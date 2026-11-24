#!/bin/zsh
# 🎯️ Greedy silhouette repair of one person at the given scales: continues the repair ops of the person (`ops/<slug>.json` here, or the repair file of its generator chain) and applies them to the catalog file; logs in 🗑️generated/silhouette/SH-2/auto. fix.sh <slug> <scale>…   (LIMIT=<m> a tighter largest distance, MAXSEC=<s> the time budget per scale, PRUNE=0 PARAM=0 POLISH=0 switch steps off, NOAPPLY=1 keeps the catalog file, CMD=<directed steps> with GROUP=1 instead of the greedy pass, FITSEC=<s> the budget of the fit, TIMEOUT=<s> the hard limit of one scale)
source "${0:A:h}/env.sh"
a="$1"; shift
ops="$("$B" -e "import { opsFile } from '$W/people.ts'; console.log(opsFile('$a'))")"
mkdir -p "$GEN/auto" "$W/ops"
cd "$S/📦️packages/🟦️typescript"
for s in "$@"; do
  log="$GEN/auto/$a-$s.log"
  MAXSEC=${MAXSEC:-150} timeout ${TIMEOUT:-900} "$B" "$W/auto.ts" "$a" "$s" "$ops" ${MAXITER:-60} "$ops" > "$log" 2>&1
  echo "$a 1:$s $(grep 'issues ' "$log" | tail -1 | cut -c9-); $(tail -1 "$log" | cut -c9-200)"
done
[[ -n "$NOAPPLY" ]] && exit 0
cd "$W" && ./apply.sh "$a"
