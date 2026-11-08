#!/bin/bash
# usage: seq3.sh : waits for seq2, then reruns the decode backing and continuation targets
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
while [ ! -f "$C/🗑️generated/1105b/testdebt-h-7.rc" ]; do sleep 20; done
"$T/runtime/tst.sh" testdebt-h-8 🧰️framework/Cargo.toml native -p semio-framework-value-native-decoding --lib --tests
