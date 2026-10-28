#!/bin/sh
# [gram2-g] verify of batch row-table AFTER apply: (1) row-table law 5 pass, (2) row-cost law 2 pass and row-sequence law 2 pass (rows and order equal to the module before the table; 9 pass in all) (red before the batch: 1113 > 900 bytes per row), (3) the observation suites keep their pass/fail set (baseline recorded before the batch: 42 pass, 19 fail that fail in this sandbox for lack of the repo test envelope; any difference is printed).
cd /Users/ueli/Documents/semio || exit 1
HERE='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram2-g-tools/staged/row-table'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
O='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation'
$BUN test "./$O/🔢️row-table/🧪️tests/🟦️.ts" "./$O/🧪️tests/🔢️row-cost/🟦️.ts" "./$O/🧪️tests/🧾️row-sequence/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)"
$BUN test $(ls ./$O/🧪️tests/*/🟦️.ts ./$O/🧪️tests/🟦️.ts) 2>&1 | grep -E "^\((pass|fail)\)" | sed 's/ \[[0-9.]*ms\]//' | sort > /private/tmp/gram2-g-obs-now.txt
diff "$HERE/obs-baseline.txt" /private/tmp/gram2-g-obs-now.txt > /dev/null && echo "observation suites: same pass/fail set as the baseline" || { echo "observation suites DIFFER from the baseline:"; diff "$HERE/obs-baseline.txt" /private/tmp/gram2-g-obs-now.txt; }
