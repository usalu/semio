#!/bin/bash
# usage: authoring-components-run.sh <slug>   splices volumes/components/<slug>.json, checks and validates
T="$(cd "$(dirname "$0")" && pwd)"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$T" || exit 1
slug="$1"
asset=$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*components/"$slug")/🔣️.json
python3 volumes/components/"$slug".py 2>&1 || exit 1
python3 volume-insert.py volumes/components/"$slug".json "$asset" || exit 1
"$BUN" volume-check.ts "$slug"
"$BUN" authoring-components-validate.ts "$slug" "${@:2}"
