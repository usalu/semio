#!/bin/sh
# 🧪️ Proves a candidate document of one person before it is shipped, one process after the other: the three-views sweep, the validator with the measured volume rules, and the silhouette measurement of the candidate against the untouched copy. `cand-3.sh <id>` reads `🗑️generated/tv3/<id>/before.json` and `cand.json`.
HERE="$(cd "$(dirname "$0")" && pwd)"; U="$(dirname "$HERE")"
S="$U/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for id in "$@"; do
  G="$U/🗑️generated/tv3/$id"
  cd "$S" && timeout 900 "$BUN" "$HERE/sweep-2.ts" "$id" --file "$G/cand.json" | tail -2 | cut -c1-300
  cd "$T14" && timeout 2400 "$BUN" r3-validate.ts --file "$G/cand.json" --volumes 2>&1 > "$G/val-after.txt"; grep -v "^PRINT ok" "$G/val-after.txt" | tail -${TAIL:-6} | cut -c1-400
  [ -f "$G/val-before.txt" ] || (cd "$T14" && timeout 2400 "$BUN" r3-validate.ts --file "$G/before.json" --volumes > "$G/val-before.txt" 2>&1)
  echo "validator before: $(tail -1 "$G/val-before.txt" | cut -c1-200)"
  [ -f "$G/sil-before.txt" ] || (cd "$S" && timeout 900 "$BUN" "$HERE/silhouette-2.ts" "$id" --file "$G/before.json" > "$G/sil-before.txt" 2>&1)
  cd "$S" && timeout 900 "$BUN" "$HERE/silhouette-2.ts" "$id" --file "$G/cand.json" > "$G/sil-after.txt" 2>&1
  echo "silhouette before: $(grep -c FAIL "$G/sil-before.txt") failing; after: $(grep -c FAIL "$G/sil-after.txt") failing"
  diff "$G/sil-before.txt" "$G/sil-after.txt" | grep "^[<>]" | cut -c1-200
done
