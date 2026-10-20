#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc6-tools/runs6.sh" "$1" "$2"; }
r "tsconfig.json" rootF
r "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json" coursepkgF
r "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" coursereactF
r "🌎️hub/📦️packages/🟦️typescript/tsconfig.json" hubF
r "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/tsconfig.json" hubadminF
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json" coordinatorF
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/tsconfig.json" vscodeF
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/tsconfig.json" repotestF
r "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/tsconfig.json" windowkitsF
r "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/tsconfig.json" petsF
r "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" petsreactF
r "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/tsconfig.json" assetsF
r "🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/tsconfig.json" machineF
echo finished > "$C/🗑️generated/tsc6-logs/chain6hF.done"
