#!/bin/sh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; L="🧰️framework/🔨️modules/🏃️process/🔒️leases"; SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/lg-reg"
for f in "⚖️fairness/🧪️tests/🟦️.ts" "⏳️wait/🧪️tests/🟦️.ts" "🧭️order/🧪️tests/🟦️.ts" "🧭️order/🧪️tests/📥️processes/🟦️.ts" "♻️retirement/🧪️tests/📋️frontier/🟦️.ts" "♻️retirement/🧪️tests/📋️claims/🟦️.ts" "♻️retirement/🧹️sweep/🧪️tests/🟦️.ts" "📦️bounded/🧪️tests/🟦️.ts" "🚪️admission/🧪️tests/🟦️.ts" "🧪️tests/📋️status/🟦️.ts"; do
  echo "=== $f"
  nice -n 10 bun "$C/lease-gc-b-tools/run-test.ts" "./$L/$f" "" "$SC" 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)|^\(fail\)|error:|No such file|Cannot find" | head -12
done
echo ALLDONE
