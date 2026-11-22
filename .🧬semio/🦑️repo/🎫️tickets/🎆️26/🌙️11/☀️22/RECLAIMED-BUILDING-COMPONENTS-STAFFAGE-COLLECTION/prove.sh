#!/bin/sh
# 🔬️ Finisher proof: for every candidate (or the slugs given) runs the full proof with the measured volume rules and the product check CLI, one process at a time, and writes both outputs per asset.
cd "$(dirname "$0")" || exit 1
K="$PWD"; OUT="$K/🗑️generated/prove"; mkdir -p "$OUT"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
S="$K/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
SLUGS=${*:-$(ls assets)}
cd "../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE" || exit 1
for slug in $SLUGS; do
  C="$K/assets/$slug/🔣️.json"
  "$BUN" r3-validate.ts --file "$C" --volumes > "$OUT/$slug.proof.txt" 2>&1
  echo "exit $?" >> "$OUT/$slug.proof.txt"
  "$BUN" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$C" > "$OUT/$slug.check.txt" 2>&1
  echo "exit $?" >> "$OUT/$slug.check.txt"
done
echo DONE > "$OUT/DONE"
