#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc-d-tools/run.sh" "$1" "$2"; }
r "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo2
r "🧰️framework/📦️packages/🟦️typescript/tsconfig.json" framework2
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json" repolib
echo finished > "$C/🗑️generated/tsc-d/chain2.done"
