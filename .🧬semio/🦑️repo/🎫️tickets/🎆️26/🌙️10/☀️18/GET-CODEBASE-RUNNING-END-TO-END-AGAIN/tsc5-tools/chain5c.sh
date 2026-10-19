#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc5-tools/runs5.sh" "$1" "$2"; }
r "🧰️framework/🛍️products/💻️os/tsconfig.json" os2
r "🧰️framework/📦️packages/🟦️typescript/tsconfig.json" framework2
r "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" petsreact2
r "✏️s/tsconfig.json" plugins2
echo finished > "$C/🗑️generated/tsc5-logs/chainc.done"
