#!/bin/sh
# 🧪 Builds one PW-09 plant as a candidate only (build, measure, feature floors, build with measured dimensions; nothing is shipped): 🗑️generated/PW-09/<asset id>/🔣️.json. Tools run directly with the pinned bun, one process at a time.
# usage: sh candidate.sh <species module> <asset id>
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/../../🗑️generated/PW-09"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
mkdir -p "$G/$2"
python3 "$HERE/build.py" "$1" "$G/$2/🔣️.json"
for round in 1 2 3; do
  "$BUN" "$HERE/measure.ts" --file "$G/$2/🔣️.json" --out "$G/$2-measure.json"
  changed=$(cd "$HERE" && python3 floors.py "$1" "$G/$2-measure.json")
  python3 "$HERE/build.py" "$1" "$G/$2/🔣️.json" "$G/$2-measure.json"
  echo "[DEBUG] round $round: $changed floors changed"
  [ "$changed" = "0" ] && [ "$round" != "1" ] && break
done
"$BUN" "$HERE/measure.ts" --file "$G/$2/🔣️.json" --out "$G/$2-measure.json"
