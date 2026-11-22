#!/bin/sh
# 🚚 PW-06: generate one species candidate, validate it with its volumes (r3-validate --file --volumes), and with "catalog" write the catalog file and run the staffage check CLI on it.
# Usage: sh ship.sh <rowan|hawthorn|black-alder> [candidate|catalog]
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
WAVE="$HERE/../.."
P="$WAVE/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
S="$WAVE/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
G="$WAVE/🗑️generated/PW-06"
ID="$1"
MODE="${2:-candidate}"
mkdir -p "$G"
quiet() { grep -v "capacity\|still working" || true; }
cd "$HERE" && repo tools exec bun -- "$ID.ts" $([ "$MODE" = catalog ] && echo --catalog) 2>&1 | quiet > "$G/gen-$ID.txt"
if [ "$MODE" = catalog ]; then
  cd "$S" && repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$ID" 2>&1 | quiet > "$G/check-$ID.txt"
else
  cd "$P" && repo tools exec bun -- r3-validate.ts --file "$G/$ID/🔣️.json" --volumes 2>&1 | quiet | cut -c1-600 > "$G/validate-$ID.txt"
fi
echo "[DEBUG] $ID $MODE done"
