#!/bin/bash
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd /Users/ueli/Documents/semio || exit 1
"$C/1105b-tools/runtime/tst.sh" W-reviewfix4-5 "Cargo.toml" native -p semio-os-flow-drawing --lib
"$C/1105b-tools/runtime/tst.sh" W-reviewfix4-6 "Cargo.toml" native -p semio-os-dsl-module-tests --no-run
echo done > "$C/🗑️generated/1105b/reviewfix4-chain2.rc"
