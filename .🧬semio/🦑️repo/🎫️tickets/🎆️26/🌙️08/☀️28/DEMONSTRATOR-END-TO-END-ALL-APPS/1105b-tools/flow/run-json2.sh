#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while [ ! -f "$C/🗑️generated/1105b/flow-run-final.rc" ]; do sleep 20; done
"$C/1105b-tools/flow/tst.sh" flow-r13 "🧰️framework/Cargo.toml" -p semio-framework-pack-json --tests
echo done > "$C/🗑️generated/1105b/flow-run-json2.rc"
