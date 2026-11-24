#!/bin/sh
# 🔁️ Builds and proves the candidates of the people the sweep reports, one process after the other; the output of each goes to `🗑️generated/tv3/<id>/cand-proof.txt`. `all-3.sh <id> …`
HERE="$(cd "$(dirname "$0")" && pwd)"; U="$(dirname "$HERE")"
for id in "$@"; do
  { sh "$HERE/build-3.sh" "$id"; sh "$HERE/cand-3.sh" "$id"; } > "$U/🗑️generated/tv3/$id/cand-proof.txt" 2>&1
  echo "== $id"; tail -12 "$U/🗑️generated/tv3/$id/cand-proof.txt" | cut -c1-300
done
