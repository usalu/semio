#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
M="🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️packages/🦀️rust/Cargo.toml"
"$T/chk.sh" reviewfix3-111 "$M" native -p semio-framework-3d-brep-engine -p semio-framework-3d-brep-mesh-io
"$T/chk.sh" reviewfix3-112 "$M" wasm -p semio-framework-3d-brep-engine -p semio-framework-3d-brep-mesh-io
"$T/runtime/tst.sh" reviewfix3-113 "$M" native -p semio-framework-3d-brep-engine --lib
echo done > "$C/🗑️generated/1105b/reviewfix3-run2.rc"
