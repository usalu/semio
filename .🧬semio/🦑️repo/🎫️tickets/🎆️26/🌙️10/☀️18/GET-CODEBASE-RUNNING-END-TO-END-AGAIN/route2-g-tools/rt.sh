#!/bin/sh
# 🧪 route2-g: runs one REAL-tree build-library test file under the census envelope with a short workspace-internal artifact root and prints the compact failing assertions. usage: rt.sh <build-relative-file> [pattern] [wall]
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/route2-g/rt.$$"; mkdir -p "$out"; printf '%s\n' "$1" > "$out/list.txt"
CENSUS_ARTROOT="${CENSUS_ARTROOT:-/private/tmp/claude-501/rg2o}" CENSUS_TAG="r$$" CENSUS_PATTERN="$2" nice -n 10 bun "$C/route-g-tools/census-run.ts" "$out/list.txt" "$out/o" "${3:-200}" >/dev/null 2>&1
head -1 "$out/o/summary.txt" | cut -c1-200
grep -v -E '^ *[0-9]+ \|' "$out/o/f000.log" | grep -E '^error|^\(fail\)|Expected|Received|^[-+@] |Error:|^ *at <anonymous>' | cut -c1-330 | head -${LINES_MAX:-40}
echo "log: $out/o/f000.log"
