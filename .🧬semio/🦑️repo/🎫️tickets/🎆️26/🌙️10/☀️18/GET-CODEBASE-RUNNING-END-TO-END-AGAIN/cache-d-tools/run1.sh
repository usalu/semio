#!/bin/sh
# usage: run1.sh <caching-relative-test-file> [name-pattern] [wallSeconds] — one file under the authored census envelope; prints the tail of the log
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/cache-d/r1.$$"; mkdir -p "$out"; printf '%s\n' "$1" > "$out/list.txt"
CENSUS_PATTERN="$2" nice -n 10 bun "$C/cache-d-tools/census-run.ts" "$out/list.txt" "$out/o" "${3:-200}" >/dev/null 2>&1
head -1 "$out/o/summary.txt"
/usr/bin/grep -E '^\(fail\)|^error:|Expected|Received|^ *[0-9]+ (pass|fail)' "$out/o/f000.log" | cut -c1-300 | head -${LINES_MAX:-40}
echo "log: $out/o/f000.log"
