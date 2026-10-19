#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
run(){ rm -f "$C/🗑️generated/tsc3/$2.done"; "$C/tsc-tools/runs.sh" "$1" "$2"; }
run "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo5
run "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json" course5
run "🧰️framework/🛍️products/💻️os/tsconfig.json" os5
run "🌎️hub/📦️packages/🟦️typescript/tsconfig.json" hub5
run "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" petsreact5
echo finished > "$C/🗑️generated/tsc3/chain7.done"
