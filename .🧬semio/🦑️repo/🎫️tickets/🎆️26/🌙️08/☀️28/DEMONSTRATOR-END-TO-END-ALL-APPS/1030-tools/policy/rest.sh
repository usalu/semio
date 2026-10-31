#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
S=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$S/🗑️generated/1030"
"$S/1030-tools/policy/pubtoken.sh"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml --lib -p semio-framework-value-native-owned native_receive_slot > "$G/policy-slot.txt" 2>&1
echo $? > "$G/policy-slot.rc"
repo run workspace:verify-taxonomy-report > "$G/policy-taxonomy.txt" 2>&1
echo $? > "$G/policy-taxonomy.rc"
