#!/bin/sh
# Proves candidates of this ticket one after the other: full proof (--volumes) and the product check CLI.
# Usage: sh finish-prove.sh [slug ...]   (no slug: all candidates). Output: 🗑️generated/prove/<slug>.txt and one summary line each.
K=$(cd "$(dirname "$0")" && pwd)
R=$(cd "$K/../../../../../../.." && pwd)
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
T="$K/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "$K/🗑️generated/prove"
[ $# -eq 0 ] && set -- $(ls "$K/assets")
for slug in "$@"; do
  C="$K/assets/$slug/🔣️.json"
  O="$K/🗑️generated/prove/$slug.txt"
  (cd "$T" && "$BUN" r3-validate.ts --file "$C" --volumes) > "$O" 2>&1
  proof=$(grep -E "^(VALID|INVALID)" "$O" | tail -1)
  echo "--- check ---" >> "$O"
  "$BUN" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$C" > "$O.check" 2>&1
  code=$?
  cat "$O.check" >> "$O"
  check=$(grep -E "^(✓|✗|✘)" "$O.check" | tail -1)
  rm "$O.check"
  echo "$slug | ${proof:-NO VERDICT} | check exit $code: $check"
done
