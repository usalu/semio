#!/bin/sh
# [gram3-g] usage: suites.sh <label> <batch dirs comma list or -> <dir under the build library>... : runs every test file below the dirs (one bun test process, preload applies the batches in memory) and writes the sorted pass/fail lines to /private/tmp/gram3-g/suites-<label>.txt plus the totals.
cd /Users/ueli/Documents/semio || exit 1
C="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L='./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'
label=$1; batches=$2; shift 2
FILES=""; for d in "$@"; do FILES="$FILES $(find "$L/$d" -path '*/🧪️tests/*' -name '🟦️.ts' -not -path '*/node_modules/*' | sort)"; done
PRE=""; [ "$batches" != "-" ] && PRE="--preload $C/gram3-g-tools/preload_tree.ts"
[ "$batches" = "-" ] && batches=""
env BATCH_DIRS="$batches" nice -n 10 $BUN test --timeout 120000 $PRE $FILES > /private/tmp/gram3-g/suites-$label.log 2>&1
rg '^\((pass|fail)\)' /private/tmp/gram3-g/suites-$label.log | sed -E 's/ \[[0-9.]+ms\]//' | sort > /private/tmp/gram3-g/suites-$label.txt
rg '^ *[0-9]+ (pass|fail)$' /private/tmp/gram3-g/suites-$label.log | tr '\n' ' '; echo
