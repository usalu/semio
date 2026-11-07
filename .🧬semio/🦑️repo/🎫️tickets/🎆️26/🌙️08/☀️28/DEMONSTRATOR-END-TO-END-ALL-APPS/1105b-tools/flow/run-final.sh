#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools/flow/tst.sh"
while [ ! -f "$C/🗑️generated/1105b/flow-run-json.rc" ]; do sleep 20; done
"$T" flow-r11 Cargo.toml -p semio-os-flow-host-laws --test original-evaluation-pipeline -- --test-threads=1
"$T" flow-r12 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-editor-retained --lib
echo done > "$C/🗑️generated/1105b/flow-run-final.rc"
