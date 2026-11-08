#!/bin/bash
# usage: seq4.sh : decode all targets then clone-source lib (one run at a time)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
"$T/runtime/tst.sh" testdebt-h-10 🧰️framework/Cargo.toml native -p semio-framework-value-native-decoding --lib --tests
"$T/runtime/tst.sh" testdebt-h-11 🧰️framework/Cargo.toml native -p semio-framework-value-clone-source --lib
