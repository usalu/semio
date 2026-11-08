#!/bin/bash
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd /Users/ueli/Documents/semio || exit 1
"$C/1105b-tools/runtime/tst.sh" W-reviewfix4-3 "Cargo.toml" native -p semio-os-flow-drawing --lib
"$C/1105b-tools/chk.sh" W-reviewfix4-4 "Cargo.toml" wasm -p semio-os-history -p semio-os-dsl
echo done > "$C/🗑️generated/1105b/reviewfix4-chain.rc"
