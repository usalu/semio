#!/bin/sh
# 🔁 Geometry iteration for one species module (PW-09): draft, measure, dimensions, check, multiview preview at 1:100.
# usage: sh geometry-loop.sh <module> <name> [scale]
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/../../🗑️generated/PW-09"
S="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
SCALE="${3:-100}"
mkdir -p "$G"
python3 "$HERE/draft.py" "$1" "$G/$2-draft.json"
(cd "$S" && repo tools exec bun -- "$HERE/measure.ts" --file "$G/$2-draft.json" --out "$G/$2-measure.json" --no-validate 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]")
python3 "$HERE/draft.py" "$1" "$G/$2-draft.json" "$G/$2-measure.json"
(cd "$S" && repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$G/$2-draft.json" --samples 3 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]" | cut -c1-500 > "$G/$2-check.txt") || true
cat "$G/$2-check.txt"
ID=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['asset']['id'])" "$G/$2-draft.json")
rm -rf "$G/$2-prev"
(cd "$S" && repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$ID" --file "$G/$2-draft.json" --scale "$SCALE" --views elevation,front,plan,axonometric --projection third --view light-solid --language en --no-png --out "$G/$2-prev" 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]" | tail -1)
(cd "$S" && repo tools exec bun -- "$HERE/raster.ts" 60 "$G/$2-prev"/*.svg 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]")
