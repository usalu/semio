#!/bin/sh
# [gram2-g] recreates the three symlinks that let the law files of the staged mirrors (tree/) run BEFORE a batch lands (they resolve the real modules next to the mirrored tests): sh mirror-links.sh [remove]. Run the laws with: bun test --preload staged/preload_generic.ts (BATCH_DIRS=<batch dir>[,<batch dir>]) <mirror test file>.
cd /Users/ueli/Documents/semio || exit 1
S=$(python3 -I '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/flatten-g-tools/flat.py' scratch '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram2-g-tools/staged/row-table/tree') || exit 1
S2=$(python3 -I '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/flatten-g-tools/flat.py' scratch '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram2-g-tools/staged/index-chunks/tree') || exit 1
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'
if [ "$1" = remove ]; then rm -f "$S/🔨️modules" "$S/$L/👁️observation/🟨️.mjs" "$S2/$L/🕸️graph/🕰️generations/🟨️.mjs"; exit 0; fi
ln -sfn "$PWD/🧰️framework/🔨️modules" "$S/🔨️modules"
ln -sfn "$PWD/$L/👁️observation/🟨️.mjs" "$S/$L/👁️observation/🟨️.mjs"
ln -sfn "$PWD/$L/🕸️graph/🕰️generations/🟨️.mjs" "$S2/$L/🕸️graph/🕰️generations/🟨️.mjs"
