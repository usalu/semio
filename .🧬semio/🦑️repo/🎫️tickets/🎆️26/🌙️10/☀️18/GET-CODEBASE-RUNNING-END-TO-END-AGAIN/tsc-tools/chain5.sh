#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
run(){ rm -f "$C/🗑️generated/tsc3/$2.done"; "$C/tsc-tools/runs.sh" "$1" "$2"; }
run "🧰️framework/🛍️products/💻️os/tsconfig.json" os3
run "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo3
run "🧰️framework/📦️packages/🟦️typescript/tsconfig.json" fw3
run "✏️s/tsconfig.json" plugins3
run "tsconfig.json" root3
echo finished > "$C/🗑️generated/tsc3/chain5.done"
