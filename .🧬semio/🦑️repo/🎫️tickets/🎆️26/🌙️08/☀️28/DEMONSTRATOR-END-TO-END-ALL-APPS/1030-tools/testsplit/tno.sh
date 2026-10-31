#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
rm -f "$C/🗑️generated/1030/testsplit-tno.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "🧰️framework/Cargo.toml" -p semio-framework-value-native-owned --no-fail-fast -- unit_and_struct_variants_derive_a_closable_receiver > "$C/🗑️generated/1030/testsplit-tno.txt" 2>&1
echo $? > "$C/🗑️generated/1030/testsplit-tno.rc"
