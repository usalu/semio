#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
while [ ! -f "$C/🗑️generated/1105b/reviewfix3-run2.rc" ]; do sleep 20; done
"$T/runtime/tst.sh" reviewfix3-121 "🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-3d-mesh-document --lib
"$T/chk.sh" reviewfix3-122 "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-os-kernel-neural-engine-model
"$T/chk.sh" reviewfix3-123 "🧰️framework/🔨️modules/🎠️kernel/🖼️ui/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-framework-kernel-ui
"$T/chk.sh" reviewfix3-124 "🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-framework-3d-mesh-document
echo done > "$C/🗑️generated/1105b/reviewfix3-run3.rc"
