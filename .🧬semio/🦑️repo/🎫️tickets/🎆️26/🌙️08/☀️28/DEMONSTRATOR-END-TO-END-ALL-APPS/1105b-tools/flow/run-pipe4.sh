#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
sleep 60
while pgrep -f "run-ret2.sh|run-unit2.sh" >/dev/null; do sleep 20; done
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/flow/tst.sh flow-r21 Cargo.toml -p semio-os-flow-host-laws --test original-evaluation-pipeline -- --test-threads=1
