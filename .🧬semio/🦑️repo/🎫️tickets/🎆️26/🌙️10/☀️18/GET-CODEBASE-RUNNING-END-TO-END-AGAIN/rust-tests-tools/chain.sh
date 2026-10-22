#!/bin/sh
# usage: chain.sh <step-id>... ; runs steps sequentially, logs to 🗑️generated/rust-tests-logs/<n>.log
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for id in "$@"; do
  n=${id%-test}
  sh "$C/rust-tests-tools/step.sh" "$id" > "$C/🗑️generated/rust-tests-logs/$n.log" 2>&1
  echo "$id rc=$?" >> "$C/🗑️generated/rust-tests-logs/chain.status"
done
