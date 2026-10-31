#!/bin/bash
D="$(dirname "$0")"; C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
N=$1
for a in 1 2 3 4 5 6; do
  "$D/chk.sh" "$N" "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/Cargo.toml" wasm -p semio-s-artifact-playbook-playbook-inferences
  grep -q "Source refused before compiler\|CargoLockRefused\|ENOENT: no such" "$C/playbook-chk$N.txt" || break
  cp "$C/playbook-chk$N.txt" "$C/playbook-chk$N.try$a.txt"; sleep 300
done
