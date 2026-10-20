#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc6-tools/runs6.sh" "$1" "$2"; }
r "🧰️framework/📦️packages/🟦️typescript/tsconfig.json" frameworkF
r "🧰️framework/🛍️products/💻️os/tsconfig.json" osF
r "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repoF
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json" repolibF
r "✏️s/tsconfig.json" pluginsF
echo finished > "$C/🗑️generated/tsc6-logs/chain6jF.done"
