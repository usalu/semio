#!/bin/bash
# usage: retry2.sh <first-n> — rerun the energy mounted smoke as W-energie-t<n>, n++ every 6 min, until the log shows a test result or a compile error in an energy crate
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"; n=$1
while :; do
 "$C/1105b-tools/runtime/tst.sh" W-energie-t$n "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" native -p semio-s-artifact-energy-model-editor-mounted-tests -- --nocapture --test-threads=1
 /usr/bin/grep -q "test result\|could not compile .semio-s-.*energy\|CargoLockRefused" "$G/W-energie-t$n.txt" && break
 n=$((n+1)); sleep 360
done
echo $n > "$G/W-energie-final-n"
