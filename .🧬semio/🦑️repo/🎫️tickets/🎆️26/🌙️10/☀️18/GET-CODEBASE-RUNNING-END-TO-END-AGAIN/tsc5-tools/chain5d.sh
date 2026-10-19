#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc5-tools/runs5.sh" "$1" "$2"; }
r "✏️s/tsconfig.json" plugins3
r "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo2
r "🌎️hub/📦️packages/🟦️typescript/tsconfig.json" hub2
r "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" uireact2
echo finished > "$C/🗑️generated/tsc5-logs/chaind.done"
