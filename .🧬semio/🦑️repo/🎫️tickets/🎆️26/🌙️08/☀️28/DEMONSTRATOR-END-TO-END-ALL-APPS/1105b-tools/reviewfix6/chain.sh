#!/bin/bash
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools
cd /Users/ueli/Documents/semio || exit 1
RT="🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/📤️receipt/📦️packages/🦀️rust/Cargo.toml"
FEM="✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/📦️packages/🦀️rust/Cargo.toml"
DRAW="🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📦️packages/🦀️rust/Cargo.toml"
RP="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️/📦️packages/🦀️rust/Cargo.toml"
"$T/runtime/tst.sh" reviewfix-611 "$RT" native -p semio-framework-actor-receipt
"$T/chk.sh" reviewfix-612 "$FEM" native -p semio-s-fem-sparse-ldlt -p semio-s-fem-analyses-assembly -p semio-s-fem-sparse-subspace -p semio-s-fem-sparse-pcg
"$T/chk.sh" reviewfix-613 "$FEM" wasm -p semio-s-fem-sparse-ldlt -p semio-s-fem-analyses-assembly
"$T/chk.sh" reviewfix-614 "$DRAW" native -p semio-ui-draw-contract
"$T/chk.sh" reviewfix-615 "$RP" native -p semio-plugin-result-page
echo done > "$T/../🗑️generated/1105b/reviewfix-chain.rc"
