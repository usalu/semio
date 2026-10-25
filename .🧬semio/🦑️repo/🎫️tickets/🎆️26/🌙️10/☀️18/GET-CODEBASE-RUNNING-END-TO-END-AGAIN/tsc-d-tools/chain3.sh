#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc-d-tools/run.sh" "$1" "$2"; }
r "🧰️framework/📦️packages/🟦️typescript/tsconfig.json" framework2
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json" repolib
r "🧰️framework/🛍️products/🦑️repo/tsconfig.json" repo2
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json" coordinator
r "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/tsconfig.json" repotest
r "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" uireact
r "🧰️framework/🔨️modules/🖱️ui/🌐️locale/tsconfig.json" locale
r "🧰️framework/🔨️modules/🖱️ui/🎓️introduction/tsconfig.json" intro
r "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/tsconfig.json" uirust
r "🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/tsconfig.json" threed
r "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/tsconfig.json" assets
r "🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/tsconfig.json" machine
r "🧰️framework/🔨️modules/◻️2d/🧮️compute/tsconfig.json" compute
echo finished > "$C/🗑️generated/tsc-d/chain3.done"
