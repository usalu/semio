#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
cd $T
rm -f 🗑️generated/render-attempts.txt 🗑️generated/three-done.txt
for s in interactions load-trace cancel; do
  E2E_ATTEMPTS=4 LIGHT=1 TAG=three-$s SCENARIO=process-$s.json ./run-render-retry.sh
  echo "$s $(tail -1 🗑️generated/three-$s-out.txt)" >> 🗑️generated/three-done.txt
  cp 🗑️generated/three-$s.png e2e-process-v14-$s.png
done
echo end >> 🗑️generated/three-done.txt
