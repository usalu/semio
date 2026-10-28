#!/bin/sh
# usage: run1.sh <build-relative-test-file> [name-pattern] [wallSeconds] — one file under the official-default census envelope; prints the verdict and the failing assertions
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/route-g/r1.$$"; mkdir -p "$out"; printf '%s\n' "$1" > "$out/list.txt"
CENSUS_TAG="r$$" CENSUS_PATTERN="$2" nice -n 10 bun "$C/route-g-tools/census-run.ts" "$out/list.txt" "$out/o" "${3:-200}" >/dev/null 2>&1
head -1 "$out/o/summary.txt"
/usr/bin/grep -E '^\(fail\)|^error:|Expected|Received|^ *[0-9]+ (pass|fail)' "$out/o/f000.log" | cut -c1-300 | head -${LINES_MAX:-40}
echo "log: $out/o/f000.log"
