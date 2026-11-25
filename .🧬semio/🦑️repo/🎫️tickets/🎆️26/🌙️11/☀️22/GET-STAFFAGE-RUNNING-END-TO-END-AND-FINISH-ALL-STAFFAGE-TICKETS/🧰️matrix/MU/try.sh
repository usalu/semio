#!/bin/zsh
# 🧫️ Runs trial.ts (the repair on a copy, gates, moved outlines, print, silhouette guard, validate, sweep) for assets one after the other and prints each result, kept in $GEN/try/<id>.<tag>.txt: TAG=<tag> try.sh "<trial flags>" <id>…   (run it through q.sh)
source "${0:h}/env.sh"
mkdir -p "$GEN/try"; cd "$P"; flags="$1"; shift
for a in "$@"; do "$B" "$W/trial.ts" $a --no-measure ${=flags} > "$GEN/try/$a.${TAG:-t}.txt" 2>&1; echo "== $a"; cut -c1-320 "$GEN/try/$a.${TAG:-t}.txt"; [[ -n "$MEASURE" ]] && "$W/measure.sh" "$a"; done
