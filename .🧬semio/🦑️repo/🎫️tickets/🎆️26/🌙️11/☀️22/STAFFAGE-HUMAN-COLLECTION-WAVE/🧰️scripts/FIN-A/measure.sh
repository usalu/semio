#!/bin/zsh
# 🩺️ Measures assets one after the other: validateAsset issue count and the staffage check CLI verdict. measure.sh <slug|path.json>...
source "${0:A:h}/env.sh"
for a in "$@"; do
  if [[ "$a" == *.json ]]; then f="$a"; else f="$CAT/$a/🔣️.json"; fi
  n=$(basename "$(dirname "$f")"); [[ "$a" == *.json ]] && n=$(basename "$f" .json)
  (cd "$T/🧰️scripts/HW-01" && $B validate.ts --file "$f") > "$GEN/[DEBUG]-validate-$n.log" 2>&1
  (cd "$S" && $B 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$f" --samples 9) > "$GEN/[DEBUG]-check-$n.log" 2>&1
  echo "$n validate: $(head -1 "$GEN/[DEBUG]-validate-$n.log" | cut -f2) check: $(grep -m1 -E '^(✓|✗)' "$GEN/[DEBUG]-check-$n.log" | cut -c1-120) issues: $(grep -c -E '^\s+(✗|!|error|issue)' "$GEN/[DEBUG]-check-$n.log")"
done
