#!/bin/sh
# Proves candidates of this ticket one after the other: full proof (--volumes) and the product check CLI; raw output goes to 🗑️generated/prove/<slug>.{proof,check}.txt, one summary line per asset to stdout.
R=/Users/ueli/Documents/semio
K="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION"
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
O="$K/🗑️generated/prove"
mkdir -p "$O"
cd "$T" || exit 1
for slug in "$@"; do
  C="$K/assets/$slug/🔣️.json"
  "$BUN" r3-validate.ts --file "$C" --volumes > "$O/$slug.proof.txt" 2>&1
  p=$?
  "$BUN" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$C" > "$O/$slug.check.txt" 2>&1
  c=$?
  echo "$slug | proof($p): $(grep -E '^(VALID|INVALID)' "$O/$slug.proof.txt" | tail -1) | check($c): $(grep -E '^[✓✗]' "$O/$slug.check.txt" | head -1)"
done
