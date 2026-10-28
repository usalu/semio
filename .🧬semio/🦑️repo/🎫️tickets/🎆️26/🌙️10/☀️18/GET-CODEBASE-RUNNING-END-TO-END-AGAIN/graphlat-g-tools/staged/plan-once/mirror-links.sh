#!/bin/sh
# [graphlat-g] recreates the symlinks that let the staged law of tree/ run BEFORE the batch lands (it resolves the real modules next to the mirrored test): sh mirror-links.sh [remove]
cd /Users/ueli/Documents/semio || exit 1
S='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graphlat-g-tools/staged/plan-once/tree'
G='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations'
if [ "$1" = remove ]; then rm -f "$S/$G/🟨️.mjs" "$S/$G/🧪️tests/🏃️world" "$S/$G/🧪️tests/🐍️.py"; exit 0; fi
ln -sfn "$PWD/$G/🟨️.mjs" "$S/$G/🟨️.mjs"
ln -sfn "$PWD/$G/🧪️tests/🏃️world" "$S/$G/🧪️tests/🏃️world"
ln -sfn "$PWD/$G/🧪️tests/🐍️.py" "$S/$G/🧪️tests/🐍️.py"
