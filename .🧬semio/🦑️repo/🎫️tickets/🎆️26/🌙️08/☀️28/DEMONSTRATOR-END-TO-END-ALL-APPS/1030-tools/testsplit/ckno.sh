#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1030/testsplit-no.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --keep-going --tests --manifest-path "🧰️framework/Cargo.toml" -p semio-framework-value-native-owned > "$C/🗑️generated/1030/testsplit-no.txt" 2>&1
echo $? > "$C/🗑️generated/1030/testsplit-no.rc"
