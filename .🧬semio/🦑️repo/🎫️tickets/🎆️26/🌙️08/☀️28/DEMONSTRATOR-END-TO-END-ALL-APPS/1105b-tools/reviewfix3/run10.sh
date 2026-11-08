#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
while [ ! -f "$C/🗑️generated/1105b/reviewfix3-run9.rc" ]; do sleep 20; done
M="🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️packages/🦀️rust/Cargo.toml"
"$T/runtime/tst.sh" reviewfix3-191 "$M" native -p semio-framework-3d-brep-engine --lib -- --skip dispose_reclaims_unreferenced_topology --skip original_brep_retention_borrows
"$T/chk.sh" reviewfix3-192 "$M" wasm -p semio-framework-3d-brep-engine -p semio-framework-3d-brep-mesh-io
echo done > "$C/🗑️generated/1105b/reviewfix3-run10.rc"
