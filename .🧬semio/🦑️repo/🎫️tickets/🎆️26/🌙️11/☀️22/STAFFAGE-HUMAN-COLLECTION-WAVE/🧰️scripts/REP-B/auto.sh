#!/bin/zsh
# 🤖️ Greedy silhouette repair of people at 1:50, 1:100, 1:200 and 1:500, one process after the other; the ops accumulate in 🗑️generated/REP-B/ops/<slug>.json (a start file there is continued), one log per scale: auto.sh <slug>…  (FILE_DIR=<dir> reads <dir>/<slug>.json instead of the catalog; SCALES="50 100" limits the scales)
source "${0:A:h}/env.sh"
mkdir -p "$GEN/ops" "$GEN/auto"
cd "$S/📦️packages/🟦️typescript"
for a in "$@"; do
  f="$a"; [[ -n "$FILE_DIR" ]] && f="$FILE_DIR/$a.json"
  for s in ${=SCALES:-50 100 200 500}; do
    "$B" "$W/auto.ts" "$f" $s "$GEN/ops/$a.json" ${MAXITER:-60} "$GEN/ops/$a.json" > "$GEN/auto/$a-$s.log" 2>&1
    echo "$a 1:$s $(grep -c . "$GEN/auto/$a-$s.log") lines; $(grep 'issues ' "$GEN/auto/$a-$s.log" | tail -1 | cut -c9-); $(tail -1 "$GEN/auto/$a-$s.log" | cut -c9-200)"
  done
done
