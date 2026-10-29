#!/bin/bash
# usage: retry.sh <t|c> <name> <ws> <args...>  : waits for >=10GiB on the rust image, runs t.sh/c.sh, retries (max 4) while the log shows ENOSPC
D=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
K=$1; N=$2; shift 2
HERE=$(dirname "$0")
for try in 1 2 3 4; do
  while [ "$(df -k "$D" | tail -1 | awk '{print $4}')" -lt 10000000 ]; do sleep 20; done
  "$HERE/$K.sh" "$N" "$@"
  if rg -q 'failed to write|ENOSPC|No space left|compiler image ran out' "$C/🗑️generated/1029/rtb-$N.txt"; then cp "$C/🗑️generated/1029/rtb-$N.txt" "$C/🗑️generated/1029/rtb-$N.enospc$try.txt"; sleep 60; continue; fi
  break
done
