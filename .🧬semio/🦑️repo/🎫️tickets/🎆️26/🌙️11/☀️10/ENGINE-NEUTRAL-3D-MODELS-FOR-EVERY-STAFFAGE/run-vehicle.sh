#!/bin/bash
# usage: run-vehicle.sh <slug> [check|splice|all] ; author script -> json -> check (draft file) / splice + validate
T="$(cd "$(dirname "$0")" && pwd)"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$T" || exit 1
slug="$1"; mode="${2:-check}"
asset=$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*vehicles/"$slug")/🔣️.json
python3 volumes/vehicles/"$slug".py || exit 1
if [ "$mode" = "check" ]; then "$BUN" volume-check.ts "$slug" --volume volumes/vehicles/"$slug".json; exit $?; fi
python3 volume-insert.py volumes/vehicles/"$slug".json "$asset" || exit 1
"$BUN" volume-check.ts "$slug"
"$BUN" authoring-vehicles-validate.ts "$slug" "${@:3}"
