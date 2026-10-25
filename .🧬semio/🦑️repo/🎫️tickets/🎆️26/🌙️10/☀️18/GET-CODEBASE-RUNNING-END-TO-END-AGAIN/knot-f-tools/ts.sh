#!/bin/sh
# usage: ts.sh <name> <scope...>  (run from the repo root)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
name="$1"; shift
args=""
for scope in "$@"; do args="$args --scope $scope"; done
s=$(date +%s)
SEMIO_LANGUAGE=en nice -n 10 bun ./📜️script.ts verify typescript check $args --scope "🧰️framework/📦️packages/🟦️typescript/🌿️ambient" --timeout-minutes 25 > "$C/🗑️generated/knot-f/$name.log" 2>&1
rc=$?
echo "$name rc=$rc $(( $(date +%s)-s ))s" >> "$C/🗑️generated/knot-f/verdicts.txt"
