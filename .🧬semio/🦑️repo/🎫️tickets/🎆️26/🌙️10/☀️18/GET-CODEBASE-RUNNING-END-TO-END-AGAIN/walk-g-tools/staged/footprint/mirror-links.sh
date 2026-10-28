#!/bin/sh
# [walk-g] recreates the symlinks that let the law files of the staged mirror (tree/) run BEFORE the batch lands: sh mirror-links.sh [remove]. Run the laws with: BATCH_DIRS=<batch dir> bun test --preload walk-g-tools/staged/preload_generic.ts <mirror test file>.
cd /Users/ueli/Documents/semio || exit 1
S=$(python3 -I '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/flatten-g-tools/flat.py' scratch '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/walk-g-tools/staged/footprint/tree') || exit 1
H='🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity'
LINKS="$H/📡️sampling/🟦️.ts $H/📐️estimates/🟦️.ts $H/📏️decision/🟦️.ts $H/🧬️schema/🔣️.json $H/🧪️tests/🧫️fixtures/🔣️.json"
OTHER='🧰️framework/🔨️modules/🏃️process/📦️packages'
if [ "$1" = remove ]; then rm -f "$S/$OTHER" "$S/$H/🟦️.ts"; for f in $LINKS; do rm -f "$S/$f"; done; exit 0; fi
mkdir -p "$S/$(dirname "$OTHER")"; ln -sfn "$PWD/$OTHER" "$S/$OTHER"
for f in $LINKS; do mkdir -p "$S/$(dirname "$f")"; ln -sfn "$PWD/$f" "$S/$f"; done
