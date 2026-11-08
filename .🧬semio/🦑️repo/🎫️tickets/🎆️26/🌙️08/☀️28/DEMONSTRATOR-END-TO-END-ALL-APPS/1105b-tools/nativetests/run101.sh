#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
while [ ! -f "$C/🗑️generated/1105b/lock-sync-1107-ok" ]; do sleep 20; done
exec "$C/1105b-tools/nativetests/tst.sh" nativetests-101 🧰️framework/Cargo.toml -p semio-framework-value-native-owned -p semio-framework-value-persistent -p semio-framework-value-snapshot-read -p semio-framework-value-tests -p semio-framework-value-tests-2 --lib --tests --keep-going
