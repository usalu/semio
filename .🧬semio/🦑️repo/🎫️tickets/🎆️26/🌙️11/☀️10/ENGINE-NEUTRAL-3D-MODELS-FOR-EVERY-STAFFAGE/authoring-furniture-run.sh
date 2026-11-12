#!/bin/bash
# usage: authoring-furniture-run.sh <slug> [poseId ...]   builds volumes/furniture/<slug>.json from <slug>.py, splices, checks and validates
T="$(cd "$(dirname "$0")" && pwd)"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$T" || exit 1
slug="$1"
asset=$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*furniture/"$slug")/🔣️.json
python3 volumes/furniture/"$slug".py 2>&1 || exit 1
python3 volume-insert.py volumes/furniture/"$slug".json "$asset" || exit 1
mkdir -p 🗑️generated/authoring-furniture
patched=$(python3 authoring-furniture-patch.py "$slug")
echo "## live catalog asset (extruded hints without sweep are evaluated as sweep v)"
"$BUN" volume-check.ts "$slug" 2>&1 | tee 🗑️generated/authoring-furniture/"$slug"-check-live.txt | grep -E "IoU|x \(|y \(|z \(|PASS|FAIL"
echo "## scratch copy with sweep w for every extruded hint (intended 2D views)"
"$BUN" volume-check.ts "$patched" 2>&1 | tee 🗑️generated/authoring-furniture/"$slug"-check.txt
"$BUN" authoring-furniture-validate.ts "$slug" "${@:2}" 2>&1 | tee 🗑️generated/authoring-furniture/"$slug"-validate.txt
