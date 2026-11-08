#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
"$T/runtime/tst.sh" reviewfix3-101 "🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-3d-mesh-document
"$T/chk.sh" reviewfix3-102 "🧰️framework/🔨️modules/🎠️kernel/🖼️ui/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-kernel-ui
echo done > "$C/🗑️generated/1105b/reviewfix3-all.rc"
