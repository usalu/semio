#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc6-tools/runs6.sh" "$1" "$2"; }
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json" repolib3
r "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo3
r "✏️s/tsconfig.json" plugins3
echo finished > "$C/🗑️generated/tsc6-logs/chain6g.done"
