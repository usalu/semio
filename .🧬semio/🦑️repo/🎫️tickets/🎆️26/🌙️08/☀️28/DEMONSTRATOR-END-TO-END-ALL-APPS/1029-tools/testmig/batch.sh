#!/bin/bash
# usage: batch.sh <batchname> name=dir ...   sequentially: tc.sh <name> <dir> --workspace --tests  (check)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
B="$1"; shift
for spec in "$@"; do
  n="${spec%%=*}"; d="${spec#*=}"
  bash "$C/1029-tools/testmig/tc.sh" "$n" "$d" --workspace
  echo "$n $(cat "$C/🗑️generated/1029/testmig-$n.rc")" >> "$C/🗑️generated/1029/testmig-batch-$B.log"
done
echo done > "$C/🗑️generated/1029/testmig-batch-$B.rc"
