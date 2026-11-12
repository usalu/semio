#!/bin/sh
# usage: run.sh <slug> [insert]   builds the volume, optionally splices it, then checks it
slug="$1"
here="$(cd "$(dirname "$0")" && pwd)"
bun="$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
asset="$(ls -d "$S"/🗂️catalog/*humans/"$slug")/🔣️.json"
cd "$here" || exit 1
python3 "$slug.py" >/dev/null || exit 1
if [ "$2" = "insert" ]; then python3 "$here/../volume-insert.py" "$here/../volumes/humans/$slug.json" "$asset" || exit 1; fi
"$bun" "$here/../volume-check.ts" "$slug" --volume "$here/../volumes/humans/$slug.json" | grep -E "IoU|^  [xyz] |PASS|FAIL|outside the drawing"
"$bun" render.ts "$slug" --volume "$here/../volumes/humans/$slug.json" >/dev/null
