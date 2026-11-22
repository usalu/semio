#!/bin/sh
# 🚢 Builds, measures, floors, checks and (on ✓) ships one PW-09 plant, then renders its evidence sheets (PW-09).
# usage: sh ship.sh <species module> <asset id>
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/../../🗑️generated/PW-09"
S="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
mkdir -p "$G"
measure() { (cd "$S" && repo tools exec bun -- "$HERE/measure.ts" --file "$G/$2.json" --out "$G/$2-measure.json" 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]"); }
python3 "$HERE/build.py" "$1" "$G/$2.json"
for round in 1 2 3; do
  measure "$1" "$2"
  changed=$(cd "$HERE" && python3 floors.py "$1" "$G/$2-measure.json")
  python3 "$HERE/build.py" "$1" "$G/$2.json" "$G/$2-measure.json"
  echo "[DEBUG] round $round: $changed floors changed"
  [ "$changed" = "0" ] && [ "$round" != "1" ] && break
done
(cd "$S" && repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$G/$2.json" --samples 9 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]" > "$G/$2-check.txt") || true
cat "$G/$2-check.txt"
if head -1 "$G/$2-check.txt" | grep -q "^✓"; then
  python3 "$HERE/build.py" "$1" "$G/$2.json" "$G/$2-measure.json" --ship
  rm -rf "$G/$2-sheets"
  for d in 50 100 200 500; do
    (cd "$S" && repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$2" --scale $d --views elevation,front,plan,axonometric --projection third --view light-solid --language en --no-png --out "$G/$2-sheets" 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]" | tail -1)
  done
  (cd "$S" && repo tools exec bun -- "$HERE/raster.ts" 50 "$G/$2-sheets"/*.svg 2>&1 | grep -v "^\[capacity\]\|^\[repo\]\|^\[tools\]")
fi
