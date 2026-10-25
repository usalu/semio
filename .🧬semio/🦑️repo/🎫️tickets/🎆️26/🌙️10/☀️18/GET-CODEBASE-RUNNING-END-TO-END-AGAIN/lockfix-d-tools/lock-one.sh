#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/lockfix-d"
export SEMIO_LANGUAGE=en
unset SEMIO_SCRIPT_STALL_MS
for s in "$@"; do
  echo "== $s $(date +%T)" >> "$G/chain.log"
  nice -n 10 bun repo dependencies lock --scope "$s" > "$G/lock-$s.log" 2>&1
  echo "rc=$? $s $(date +%T)" >> "$G/chain.log"
done
echo done >> "$G/chain.log"
