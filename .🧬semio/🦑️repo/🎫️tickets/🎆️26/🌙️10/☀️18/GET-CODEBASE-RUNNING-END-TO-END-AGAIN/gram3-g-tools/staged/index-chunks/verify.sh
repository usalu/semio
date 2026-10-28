#!/bin/sh
# [gram3-g] verify of batch index-chunks (REPLACES gram2-g's) AFTER apply: (1) the chunk-classes law (policy = schema, runs = lists, classes = model partition) and the index-chunks law (equal to the legacy builder on 300 seeded corpora, bulk patch = per-owner patch on 60 seeds, the memory law, fixture schema): 6 pass; (2) every generations suite keeps the pass/fail set of baselines/generations.txt (taken without the batch, 718 pass 7 fail); (3) no graph consumer file is over its 65536 byte bound (generations/🟨️.mjs 62326 with rev3fix, 61967 alone; gram2's version was 65887).
cd /Users/ueli/Documents/semio || exit 1
HERE='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram3-g-tools/staged/index-chunks'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations'
$BUN test "./$G/🧱️chunk-classes/🧪️tests/🟦️.ts" "./$G/🧪️tests/🧱️index-chunks/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)"
sh "$HERE/../../suites.sh" verify-generations - 🕸️graph/🕰️generations
diff "$HERE/baselines/generations.txt" /private/tmp/gram3-g/suites-verify-generations.txt > /dev/null && echo "generations suites: same pass/fail set as the baseline" || { echo "generations suites DIFFER from the baseline:"; diff "$HERE/baselines/generations.txt" /private/tmp/gram3-g/suites-verify-generations.txt; }
python3 ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rebase-g-tools/consumer-cap.py"
