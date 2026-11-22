#!/bin/sh
# 🩺 Runs the staffage check CLI (validation, suitability, every pose and clip instant, thermal, structure, all seven views at every scale) for PW-15 assets; output in 🗑️generated/PW-15/<id>-check.txt. Usage: sh pw15-check.sh <id> [<id> ...]
W="$(cd "$(dirname "$0")/../.." && pwd)"
S="$(cd "$W/../../../../../../.." && pwd)/🎓️teaching/🏛️architecture/🧍️staffage"
cd "$S" || exit 1
for ID in "$@"; do
  repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$ID" 2>&1 | grep -v "capacity\|still working" > "$W/🗑️generated/PW-15/$ID-check.txt"
  echo "== $ID"; cut -c1-400 "$W/🗑️generated/PW-15/$ID-check.txt" | head -40
done
