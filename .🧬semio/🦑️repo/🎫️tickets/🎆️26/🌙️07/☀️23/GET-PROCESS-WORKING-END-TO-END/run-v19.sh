#!/bin/zsh
# Runs the acceptance browser scenarios against the published hub; a run killed from outside (fin 2) is retried.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; cd $T
V=${V:-v19}; rm -f 🗑️generated/$V-done.txt
for s in ${=SCENARIOS:-cancel-load interactions load-trace role-cycle}; do
  for i in 1 2 3; do
    LIGHT=1 TAG=$V-$s SCENARIO=process-$s.json ./run-forest-render.sh
    grep -q "^fin 2" 🗑️generated/$V-$s-out.txt || break
  done
  echo "$s $(tail -1 🗑️generated/$V-$s-out.txt)" >> 🗑️generated/$V-done.txt
  cp 🗑️generated/$V-$s.png e2e-process-$V-$s.png
done
echo end >> 🗑️generated/$V-done.txt
