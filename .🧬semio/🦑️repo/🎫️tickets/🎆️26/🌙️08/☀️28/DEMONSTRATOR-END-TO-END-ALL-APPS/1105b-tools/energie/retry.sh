#!/bin/bash
# usage: retry.sh <first-n> — rerun the energy mounted smoke as W-energie-t<n>, n++, until the run is not a source-boundary refusal
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"; n=$1
while :; do
 "$C/1105b-tools/runtime/tst.sh" W-energie-t$n "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" native -p semio-s-artifact-energy-model-editor-mounted-tests -- --nocapture --test-threads=1
 /usr/bin/grep -q "Source refused before compiler\|CargoLockRefused" "$G/W-energie-t$n.txt" || break
 n=$((n+1)); sleep 240
done
echo $n > "$G/W-energie-final-n"
