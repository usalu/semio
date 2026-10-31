#!/bin/bash
D="$(dirname "$0")"; C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
N=$1
for a in $(seq 1 12); do
  "$D/chk.sh" "$N" "🌎️hub/Cargo.toml" wasm --lib -p semio-hub-playbook-component
  grep "^error: could not compile" "$C/playbook-chk$N.txt" | grep -qv "playbook\|playground" || break
  cp "$C/playbook-chk$N.txt" "$C/playbook-chk$N.try$a.txt"; sleep 900
done
echo done > "$C/playbook-hub$N.done"
