#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
run(){ rm -f "$C/🗑️generated/tsc3/$2.done"; "$C/tsc-tools/runs.sh" "$1" "$2"; }
run "🧰️framework/🛍️products/💻️os/tsconfig.json" os4
run "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo4
run "🧰️framework/📦️packages/🟦️typescript/tsconfig.json" fw4
run "✏️s/tsconfig.json" plugins4
run "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json" course4
run "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" coursereact4
run "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" uireact4
run "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" rendererreact4
echo finished > "$C/🗑️generated/tsc3/chain6.done"
