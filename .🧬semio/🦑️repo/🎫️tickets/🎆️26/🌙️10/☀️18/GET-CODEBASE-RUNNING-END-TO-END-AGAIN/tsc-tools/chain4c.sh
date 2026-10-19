#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc-tools/runs4.sh" "$1" "$2"; }
r "🧰️framework/📦️packages/🟦️typescript/tsconfig.json" framework
r "🧰️framework/🛍️products/💻️os/tsconfig.json" os
r "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo
r "✏️s/tsconfig.json" plugins
echo finished > "$C/🗑️generated/tsc4-logs/chainc.done"
