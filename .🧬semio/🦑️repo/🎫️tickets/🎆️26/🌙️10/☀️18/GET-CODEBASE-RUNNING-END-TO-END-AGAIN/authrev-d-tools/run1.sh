#!/bin/sh
# usage: run1.sh <module-relative-test-file> [name-pattern] [wallSeconds] — one file under proc-d's authored census envelope (reused read-only); output under generated/authrev-d
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/authrev-d/r1.$$"; mkdir -p "$out"; printf '%s\n' "$1" > "$out/list.txt"
CENSUS_TAG="ar$$" CENSUS_PATTERN="$2" nice -n 10 bun "$C/proc-d-tools/census-run.ts" "$out/list.txt" "$out/o" "${3:-200}" >/dev/null 2>&1
head -1 "$out/o/summary.txt"
grep -E '^\(fail\)|^\(pass\)|^error:|Expected|Received|^ *[0-9]+ (pass|fail)|^EVT' "$out/o/f000.log" | cut -c1-300 | head -${LINES_MAX:-60}
echo "log: $out/o/f000.log"
