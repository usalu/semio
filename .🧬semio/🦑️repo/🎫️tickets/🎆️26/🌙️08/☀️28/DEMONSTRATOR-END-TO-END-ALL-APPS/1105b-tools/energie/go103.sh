#!/bin/bash
# usage: go103.sh <n> — wait until runtime splits semio-framework-job-context (park file changes), then run the energy mounted smoke as W-energie-t<n>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
F="🧰️framework/🔨️modules/🧵️job/⏱️context/⏸️park/🦀️.rs"
B=$(stat -f %z "$F")
while [ "$(stat -f %z "$F" 2>/dev/null)" = "$B" ]; do sleep 20; done
sleep 120
exec "$C/1105b-tools/runtime/tst.sh" W-energie-t$1 "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" native -p semio-s-artifact-energy-model-editor-mounted-tests -- --nocapture --test-threads=1
