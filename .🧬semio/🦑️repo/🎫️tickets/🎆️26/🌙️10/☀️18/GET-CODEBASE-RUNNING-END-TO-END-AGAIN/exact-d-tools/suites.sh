#!/bin/sh
# usage: suites.sh <run-id> <suite>... : the process contract suites one after the other through the repo exec envelope, one log per suite; a run that dies in the dependency step on a peer's in-flight manifest edit (lock preflight, frozen refusal or stale receipt, no test output) is repeated after a pause, up to 40 times
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
ID="$1"; shift
for suite in "$@"; do
  log="$C/🗑️generated/exact-d/$ID-$suite.log"
  for attempt in $(seq 1 40); do
    sh "$C/exact-d-tools/test.sh" "$suite" > "$log" 2>&1
    if /usr/bin/grep -q -E "Bun refused the frozen installation|generation receipt of scope .* is stale|lock file of dependency scope .* is out of date|Too many open files" "$log" && ! /usr/bin/grep -q -E "^\(pass\)|^\(fail\)" "$log"; then echo "$suite attempt $attempt: dependency step refused, retry" >> "$C/🗑️generated/exact-d/$ID-summary.txt"; sleep 90; else break; fi
  done
  echo "$suite $(tail -1 "$log")" >> "$C/🗑️generated/exact-d/$ID-summary.txt"
done
echo done >> "$C/🗑️generated/exact-d/$ID-summary.txt"
