#!/bin/bash
D="$(dirname "$0")"; C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
N=$1
for a in 1 2 3 4 5; do
  "$D/chk.sh" "$N" "🌎️hub/Cargo.toml" wasm -p semio-hub-playbook-component
  grep -q "could not compile .semio-framework-2d\|CargoLockRefused\|Source refused" "$C/playbook-chk$N.txt" || break
  cp "$C/playbook-chk$N.txt" "$C/playbook-chk$N.try$a.txt"; sleep 600
done
"$D/chk.sh" $((N+1)) "✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.toml" wasm -p semio-s-plugin-playbook-procedural-component
