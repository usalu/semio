#!/bin/sh
# 🧪️ Proves candidates one after the other: full proof (--volumes) and the product check CLI, last lines into 🗑️generated/prove/<slug>.txt.
# Usage: sh prove.sh [slug …]   (no slug: every candidate under assets/)
K=$(cd "$(dirname "$0")" && pwd)
R=$(cd "$K/../../../../../../.." && pwd)
T="$K/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
OUT="$K/🗑️generated/prove"
mkdir -p "$OUT"
[ $# -eq 0 ] && set -- $(ls "$K/assets")
cd "$T" || exit 1
for slug in "$@"; do
  C="$K/assets/$slug/🔣️.json"
  "$BUN" r3-validate.ts --file "$C" --volumes > "$OUT/$slug.proof.txt" 2>&1
  proof=$(tail -1 "$OUT/$slug.proof.txt")
  "$BUN" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$C" > "$OUT/$slug.check.txt" 2>&1
  check=$(tail -1 "$OUT/$slug.check.txt")
  echo "$slug | $proof | $check"
done
