#!/bin/sh
# [walk-g] recreates the symlinks that let the law files of the staged mirror (tree/) run BEFORE the batch lands (they resolve the real modules next to the mirrored tests): sh mirror-links.sh [remove]. Run the laws with: BATCH_DIRS=<batch dir> bun test --preload walk-g-tools/staged/preload_generic.ts <mirror test file>.
cd /Users/ueli/Documents/semio || exit 1
S=$(python3 -I '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/flatten-g-tools/flat.py' scratch '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/walk-g-tools/staged/walks/tree') || exit 1
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'
LINKS="$L/🕸️graph/📥️inference/🟨️.mjs $L/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs $L/👁️observation/🟨️.mjs $L/📥️inputs/🎯️precision/🟦️.ts"
if [ "$1" = remove ]; then rm -f "$S/🧰️framework/🔨️modules"; for f in $LINKS; do rm -f "$S/$f"; done; exit 0; fi
ln -sfn "$PWD/🧰️framework/🔨️modules" "$S/🧰️framework/🔨️modules"
for f in $LINKS; do mkdir -p "$S/$(dirname "$f")"; ln -sfn "$PWD/$f" "$S/$f"; done
