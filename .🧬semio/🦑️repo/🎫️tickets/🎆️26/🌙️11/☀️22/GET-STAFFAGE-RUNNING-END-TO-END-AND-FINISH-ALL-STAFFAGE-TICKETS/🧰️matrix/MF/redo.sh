#!/bin/zsh
# ♻️ Rebuilds the catalog document of an asset of batch MF from the document it had before the batch (`$GEN/backup/<id>.json`) with the current spec, when a spec was corrected after its first application: refuses when the catalog document is not the one this batch wrote last (`$GEN/after/<id>.json`). redo.sh <id>…
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
for a in "$@"; do
  f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
  (cd "$T" && timeout 1800 "$B" "$W/fix.ts" "$a" --in "$GEN/backup/$a.json" --out "$GEN/redo-$a.json" --quiet) | tail -1
  cmp -s "$f" "$GEN/after/$a.json" || { echo "$a: the catalog document is not the one written last, nothing done"; continue; }
  cp "$GEN/redo-$a.json" "$f" && cp "$GEN/redo-$a.json" "$GEN/after/$a.json" && rm "$GEN/redo-$a.json" && echo "$a: rebuilt"
done
