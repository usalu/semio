#!/bin/bash
D="$(dirname "$0")"; cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
n=31
for round in $(seq 1 40); do
  "$D/hub.sh" $n
  if ! /usr/bin/grep -q 'could not compile `semio-framework' "$G/resid-chk$n.txt"; then break; fi
  n=$((n+1)); sleep 600
done
"$D/gis.sh" $((n+100)) $((n+101))
"$D/chk.sh" $((n+102)) "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml" native -p semio-s-artifact-cad-cad-editor
bash "$D/run-retry.sh" x4 flow-list flow-logic flow-math flow-primitive flow-text cad-aec-building cad-aec-energy cad-aec-structure process-concrete process-metal sourcing-beams sourcing-slabs sourcing-windows
