#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
while [ ! -f "$C/🗑️generated/1105b/reviewfix3-run4.rc" ]; do sleep 20; done
"$T/runtime/tst.sh" reviewfix3-141 "🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-3d-mesh-document --lib
echo done > "$C/🗑️generated/1105b/reviewfix3-run5.rc"
