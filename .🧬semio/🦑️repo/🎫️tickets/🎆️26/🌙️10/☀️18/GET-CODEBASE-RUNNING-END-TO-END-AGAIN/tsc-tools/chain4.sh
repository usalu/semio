#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
run(){ rm -f "$C/🗑️generated/tsc3/$2.done"; "$C/tsc-tools/runs.sh" "$1" "$2"; }
run "$C/🗑️generated/tsc3/s-repo.json" srepo
run "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json" course2
run "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" coursereact2
run "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" uireact
run "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" rendererreact
run "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/tsconfig.json" winkits
run "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/tsconfig.json" pets
run "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" petsreact
run "🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/tsconfig.json" machine
run "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/tsconfig.json" assets
run "🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/tsconfig.json" c3d
run "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/tsconfig.json" uirust
run "🧰️framework/🔨️modules/🖱️ui/🌐️locale/tsconfig.json" locale
run "🧰️framework/🔨️modules/🖱️ui/🎓️introduction/tsconfig.json" intro
run "🧰️framework/🔨️modules/◻️2d/🧮️compute/tsconfig.json" compute
run "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/tsconfig.json" vscode
run "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json" repolib
run "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json" coordinator
run "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/tsconfig.json" repotest
run "✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/tsconfig.json" devcad
run "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/tsconfig.json" aecbuilding
run "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/tsconfig.json" aecstructure
run "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/tsconfig.json" aecenergy
run "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/tsconfig.json" spatial
echo finished > "$C/🗑️generated/tsc3/chain.done"
