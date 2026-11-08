#!/bin/bash
# usage: go102.sh <n> — wait for the lock marker, then run the energy mounted smoke as W-energie-t<n>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while [ ! -f "$C/🗑️generated/1105b/lock-sync-1107-ok" ]; do sleep 20; done
exec "$C/1105b-tools/runtime/tst.sh" W-energie-t$1 "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" native -p semio-s-artifact-energy-model-editor-mounted-tests -- --nocapture --test-threads=1
