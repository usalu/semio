#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
M="🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️packages/🦀️rust/Cargo.toml"
true
"$T/runtime/tst.sh" reviewfix3-401 "$M" native -p semio-framework-3d-brep-engine --lib -- finished_mesh_import original_brep_retention
"$T/runtime/tst.sh" reviewfix3-402 "🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-3d-mesh-document --lib -- --skip source_collection_requires_original_buffer_reservation
"$T/chk.sh" reviewfix3-403 "$M" wasm -p semio-framework-3d-brep-engine -p semio-framework-3d-brep-mesh-io
echo done > "$C/🗑️generated/1105b/reviewfix3-run16.rc"
