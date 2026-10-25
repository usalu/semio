#!/bin/sh
# usage: suites.sh <run-id> <suite>... : the process contract suites one after the other through the repo exec envelope, one log per suite
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
ID="$1"; shift
for suite in "$@"; do
  sh "$C/lockc-d-tools/test.sh" "$suite" > "$C/🗑️generated/lockc-d/$ID-$suite.log" 2>&1
  echo "$suite $(tail -1 "$C/🗑️generated/lockc-d/$ID-$suite.log")" >> "$C/🗑️generated/lockc-d/$ID-summary.txt"
done
echo done >> "$C/🗑️generated/lockc-d/$ID-summary.txt"
