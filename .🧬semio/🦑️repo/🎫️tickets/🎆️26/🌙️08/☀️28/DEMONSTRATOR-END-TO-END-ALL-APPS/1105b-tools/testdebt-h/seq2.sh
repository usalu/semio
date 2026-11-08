#!/bin/bash
# usage: seq2.sh : decode tests, clone-source lib tests, encode effect-settlement repro (one run at a time)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
T="$C/1105b-tools"
"$T/runtime/tst.sh" testdebt-h-5 🧰️framework/Cargo.toml native -p semio-framework-value-native-decoding --lib --tests
"$T/runtime/tst.sh" testdebt-h-6 🧰️framework/Cargo.toml native -p semio-framework-value-clone-source --lib
"$T/runtime/tst.sh" testdebt-h-7 🧰️framework/Cargo.toml native -p semio-framework-value-native-encoding --test effect-settlement -- denied_and_unperformed
