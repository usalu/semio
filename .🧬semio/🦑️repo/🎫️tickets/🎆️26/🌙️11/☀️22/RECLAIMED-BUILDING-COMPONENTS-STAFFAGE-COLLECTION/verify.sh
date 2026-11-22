#!/bin/sh
# ♻️ Coordinator proof: validates every published reclaimed asset as the catalog holds it (schema, validateAsset, print suitability) and writes one verdict line per asset.
cd "$(dirname "$0")" || exit 1
OUT="$PWD/🗑️generated/verify.txt"; mkdir -p "$PWD/🗑️generated"; : > "$OUT"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
SLUGS=$(ls assets)
cd "../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE" || exit 1
for slug in $SLUGS; do
  "$BUN" r3-validate.ts "components/$slug" --catalog-only $1 2>&1 | tail -1 >> "$OUT"
done
echo DONE >> "$OUT"
