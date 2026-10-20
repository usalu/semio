#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc6-tools/runs6.sh" "$1" "$2"; }
r "🧰️framework/🛍️products/💻️os/tsconfig.json" os
r "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json" repolib
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/tsconfig.json" repotest
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json" coordinator
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/tsconfig.json" vscode
r "🌎️hub/📦️packages/🟦️typescript/tsconfig.json" hub
r "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/tsconfig.json" hubadmin
r "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/tsconfig.json" pets
r "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" petsreact
echo finished > "$C/🗑️generated/tsc6-logs/chain6a.done"
