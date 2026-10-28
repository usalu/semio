#!/bin/sh
# [gram3-g] verify of batch `churn` AFTER apply (or with BATCH_DIRS=<this dir> before: the laws then run on the in-memory edits): (1) the five new laws, (2) the suites it touches keep their pass/fail sets against the recorded baselines (any difference is printed). Needs the repo test envelope for the file-record law (SEMIO_SCRIPT_PROCESS_INVOCATION / SEMIO_TEST_ARTIFACT_DIR; gram3-g-tools/law.py shows one way).
cd /Users/ueli/Documents/semio || exit 1
HERE='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram3-g-tools/staged/churn'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'
$BUN test "./$L/🏗️build/🧂️text-bytes/🧪️tests/🟦️.ts" "./$L/⚡️caching/🗃️records/🧪️tests/🔑️key-parts/🟦️.ts" "./$L/🏗️build/📥️inputs/🧪️tests/🧂️chunk-memo/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)"
$BUN test "./$L/🏗️build/👁️observation/🧪️tests/🧠️file-record-image/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)"
for pair in "obs:👁️observation" "inputs:📥️inputs 📤️arguments" "infer:🕸️graph/📥️inference"; do
  name=${pair%%:*}; dirs=${pair#*:}
  sh "$HERE/../../suites.sh" "verify-$name" - $dirs
  diff "$HERE/baselines/$name.txt" "/private/tmp/gram3-g/suites-verify-$name.txt" > /dev/null && echo "$name suites: same pass/fail set as the baseline" || { echo "$name suites DIFFER from the baseline:"; diff "$HERE/baselines/$name.txt" "/private/tmp/gram3-g/suites-verify-$name.txt"; }
done
