#!/bin/bash
# usage: seq6.sh : clone-source lib then decode all targets
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
"$T/runtime/tst.sh" testdebt-h-13 🧰️framework/Cargo.toml native -p semio-framework-value-clone-source --lib
"$T/runtime/tst.sh" testdebt-h-14 🧰️framework/Cargo.toml native -p semio-framework-value-native-decoding --lib --tests
