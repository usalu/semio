#!/bin/bash
# usage: seq.sh : decode tests, clone-source test check, encode effect test (one run at a time)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
"$T/runtime/tst.sh" testdebt-h-2 🧰️framework/Cargo.toml native -p semio-framework-value-native-decoding --lib --tests
"$T/chk.sh" testdebt-h-3 🧰️framework/Cargo.toml native -p semio-framework-value-clone-source --lib --tests
"$T/runtime/tst.sh" testdebt-h-4 🧰️framework/Cargo.toml native -p semio-framework-value-native-encoding --lib -- denied_and_unperformed
