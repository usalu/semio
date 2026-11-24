#!/bin/zsh
# 🔁️ Reruns one private-build job while another agent's live edit breaks a crate this job does not own.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
L=$1
for i in 1 2 3 4 5 6 7 8 9 10; do
  $T/brep-cargo2.sh "$@" >/dev/null 2>&1
  if grep -q "^error: could not compile\|can't find crate" $T/🗑️generated/brep-$L.txt && ! grep -q "could not compile .semio-s-artifact-process-process3d" $T/🗑️generated/brep-$L.txt; then sleep 180; else break; fi
done
