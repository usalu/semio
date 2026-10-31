#!/bin/bash
D="$(dirname "$0")"; C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
N=$1; shift
for a in 1 2 3 4; do
  "$D/chk.sh" "$N" "$@"
  grep -q "ENOENT: no such file\|Bun v1" "$C/playbook-chk$N.txt" || break
  cp "$C/playbook-chk$N.txt" "$C/playbook-chk$N.try$a.txt"; sleep 120
done
