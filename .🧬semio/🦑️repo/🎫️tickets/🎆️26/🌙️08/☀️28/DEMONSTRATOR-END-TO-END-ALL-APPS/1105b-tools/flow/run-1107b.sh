#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while [ ! -f "$C/🗑️generated/1105b/flow-run-1107.rc" ]; do sleep 20; done
"$C/1105b-tools/chk.sh" flow-104 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" wasm -p semio-s-artifact-flow-flow-editor -p semio-s-artifact-flow-flow
"$C/1105b-tools/flow/tst.sh" flow-105 "🧰️framework/Cargo.toml" -p semio-framework-pack-json --tests
echo done > "$C/🗑️generated/1105b/flow-run-1107b.rc"
