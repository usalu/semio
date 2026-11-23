#!/bin/sh
# 🔬️ The final proof of one catalog asset of the three-views fix, one process at a time: the three-views sweep and consistency, the validator with the measured volume rules, the product check CLI and the silhouette measurement, each as its last lines. `prove-2.sh <id> [<id> …]`; the whole output also goes to `🗑️generated/tv2/<id>/proof.txt`.
HERE="$(cd "$(dirname "$0")" && pwd)"; U="$(dirname "$HERE")"
S="$U/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for id in "$@"; do
  OUT="$U/🗑️generated/tv2/$id"; mkdir -p "$OUT"
  FILE=$(ls "$S/🗂️catalog"/*/"$id"/🔣️.json)
  {
    cd "$S" && timeout 900 "$BUN" "$HERE/sweep-2.ts" "$id" | tail -3
    cd "$T14" && timeout 2400 "$BUN" r3-validate.ts --file "$FILE" --volumes 2>&1 | grep -v "^PRINT ok" | tail -4 | cut -c1-300
    cd "$S" && timeout 900 "$BUN" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$id" 2>&1 | head -1 | cut -c1-200
    cd "$S" && timeout 900 "$BUN" "$HERE/silhouette-2.ts" "$id" | grep "FAIL\|failing"
  } 2>&1 | tee "$OUT/proof.txt"
done
