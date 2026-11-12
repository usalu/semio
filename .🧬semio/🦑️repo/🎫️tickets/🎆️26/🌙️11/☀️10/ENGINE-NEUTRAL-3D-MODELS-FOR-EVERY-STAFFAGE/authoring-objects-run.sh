#!/bin/bash
# usage: authoring-objects-run.sh <slug> [nosplice]   author -> splice -> check -> validate -> posecheck
T="$(cd "$(dirname "$0")" && pwd)"
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug="$1"
cd "$T"
python3 "authoring-objects-$slug.py" || exit 1
[ "$2" = nosplice ] && { $BUN volume-check.ts "$slug" --volume "volumes/objects/$slug.json"; exit; }
python3 volume-insert.py "volumes/objects/$slug.json" "$S/$slug/🔣️.json" || exit 1
$BUN volume-check.ts "$slug"
$BUN authoring-objects-validate.ts "$slug"
$BUN authoring-objects-posecheck.ts "$slug"
