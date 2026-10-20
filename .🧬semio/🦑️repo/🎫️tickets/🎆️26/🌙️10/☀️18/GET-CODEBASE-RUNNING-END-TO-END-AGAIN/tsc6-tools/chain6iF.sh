#!/bin/zsh
setopt NO_BG_NICE
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r(){ "$C/tsc6-tools/runs6.sh" "$1" "$2"; }
r "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" engineF
r "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json" uireactF
r "🧰️framework/🔨️modules/🖱️ui/🌐️locale/tsconfig.json" localeF
r "🧰️framework/🔨️modules/🖱️ui/🎓️introduction/tsconfig.json" introF
r "🧰️framework/🔨️modules/◻️2d/🧮️compute/tsconfig.json" computeF
r "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/tsconfig.json" uirustF
r "✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/tsconfig.json" devcadF
r "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/tsconfig.json" cadspatialF
r "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/tsconfig.json" cadstructureF
r "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/tsconfig.json" cadenergyF
r "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/tsconfig.json" cadbuildingF
r "🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/tsconfig.json" 3dF
echo finished > "$C/🗑️generated/tsc6-logs/chain6iF.done"
