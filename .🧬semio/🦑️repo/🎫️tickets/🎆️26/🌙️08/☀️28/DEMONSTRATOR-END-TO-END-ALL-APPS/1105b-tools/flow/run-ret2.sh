#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
sleep 90
while pgrep -f "run-pipe3.sh|run-unit.sh|run-ret.sh" >/dev/null; do sleep 20; done
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/flow/tst.sh flow-r19 "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml" -p semio-s-artifact-flow-flow-editor-retained -p semio-s-artifact-flow-flow-editor-publication --lib
