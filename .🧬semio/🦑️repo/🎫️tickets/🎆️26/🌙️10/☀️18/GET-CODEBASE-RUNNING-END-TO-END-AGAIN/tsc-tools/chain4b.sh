#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc-tools/runs4.sh" "$1" "$2"; }
r "🧰️framework/📦️packages/🟦️typescript/tsconfig.json" framework
r "✏️s/tsconfig.json" plugins
r "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json" coursepkg
r "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" coursereact
r "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" uireact
r "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" engine
r "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/tsconfig.json" windowkits
r "🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/tsconfig.json" machine
r "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/tsconfig.json" assets
r "🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/tsconfig.json" 3d
r "🧰️framework/🔨️modules/🖱️ui/🌐️locale/tsconfig.json" locale
r "🧰️framework/🔨️modules/🖱️ui/🎓️introduction/tsconfig.json" intro
r "🧰️framework/🔨️modules/◻️2d/🧮️compute/tsconfig.json" compute
r "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/tsconfig.json" uirust
r "✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/tsconfig.json" devcad
r "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/tsconfig.json" cadspatial
r "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/tsconfig.json" cadstructure
r "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/tsconfig.json" cadenergy
r "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/tsconfig.json" cadbuilding
r "tsconfig.json" root
echo finished > "$C/🗑️generated/tsc4-logs/chainb.done"
