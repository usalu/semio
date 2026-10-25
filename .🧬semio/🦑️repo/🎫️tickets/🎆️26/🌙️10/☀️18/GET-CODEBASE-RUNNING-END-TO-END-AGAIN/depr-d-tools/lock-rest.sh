#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/depr-d"
export SEMIO_LANGUAGE=en
unset SEMIO_SCRIPT_STALL_MS
for s in ✏️s 🏢️semio-tech 🌎️hub ♻️mit-bestand; do
  echo "== $s $(date +%T)" >> "$G/lock-rest.chain"
  nice -n 10 bun repo dependencies lock --scope "$s" > "$G/lock-rest-$s.log" 2>&1
  echo "rc=$? $s $(date +%T)" >> "$G/lock-rest.chain"
done
echo done >> "$G/lock-rest.chain"
