#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while [ ! -f "$C/🗑️generated/1105b/flow-run-json2.rc" ]; do sleep 20; done
"$C/1105b-tools/flow/tst.sh" flow-r14 Cargo.toml -p semio-os-flow-host-laws --test original-evaluation-pipeline -- --test-threads=1
echo done > "$C/🗑️generated/1105b/flow-run-pipe2.rc"
