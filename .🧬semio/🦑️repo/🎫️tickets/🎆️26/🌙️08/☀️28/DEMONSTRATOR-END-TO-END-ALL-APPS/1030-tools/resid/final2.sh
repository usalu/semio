#!/bin/bash
D="$(dirname "$0")"; cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
blocked(){ /usr/bin/grep -q 'Source refused\|Source extent or boundary exceeded\|ENOENT\|could not compile `semio-framework' "$1"; }
n=310
for round in $(seq 1 60); do
  "$D/hub.sh" $n
  if blocked "$G/resid-chk$n.txt"; then n=$((n+1)); sleep 600; continue; fi
  break
done
m=$((n+500))
"$D/gis.sh" $m $((m+1))
"$D/chk.sh" $((m+2)) "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml" native -p semio-s-artifact-cad-cad-editor
bash "$D/run-retry.sh" x4 flow-list flow-logic flow-math flow-primitive flow-text cad-aec-building cad-aec-energy cad-aec-structure process-concrete process-metal sourcing-beams sourcing-slabs sourcing-windows
