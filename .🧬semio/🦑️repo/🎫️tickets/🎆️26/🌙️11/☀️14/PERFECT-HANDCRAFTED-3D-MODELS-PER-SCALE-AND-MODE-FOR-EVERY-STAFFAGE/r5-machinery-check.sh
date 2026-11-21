#!/bin/sh
# 🧪 Round 5 checks of the machinery assets, one after the other: r3-validate with volumes and the product volume check, output under 🗑️generated/r5-<slug>/.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for slug in "$@"; do
  G="🗑️generated/r5-$slug"; mkdir -p "$G"
  $BUN r3-validate.ts "machinery/$slug" --volumes > "$G/validate.txt" 2>&1
  echo "$slug validate: $(tail -1 "$G/validate.txt" | cut -c1-200)"
  $BUN volumes-check.ts "machinery/$slug" --product > "$G/check.txt" 2>&1
  echo "$slug check: $(tail -1 "$G/check.txt" | cut -c1-200)"
done
