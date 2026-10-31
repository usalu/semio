#!/bin/bash
# retries the puzzle3d mounted run until it gets past dependency/lock errors (max 6 tries, 8 min apart)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
for n in 31 32 33 34 35 36; do
  $C/1030-tools/runtime/t.sh p$n "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust" --test editor-task-resume --test editor-action-producer -- --test-threads=1
  if ! grep -q -E "CargoLockRefused|could not compile|Source refused" $C/🗑️generated/1030/runtime-p$n.txt; then echo p$n > $C/🗑️generated/1030/runtime-final.tag; exit 0; fi
  sleep 480
done
echo none > $C/🗑️generated/1030/runtime-final.tag
