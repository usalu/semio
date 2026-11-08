#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools/flow/tst.sh"
"$T" flow-101 Cargo.toml -p semio-os-flow-host-laws --test original-evaluation-pipeline -- --test-threads=1
"$T" flow-102 Cargo.toml -p semio-os-flow-host-read --lib
"$T" flow-103 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-editor-retained --lib -- --test-threads=1
echo done > "$C/🗑️generated/1105b/flow-run-1107.rc"
