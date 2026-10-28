#!/bin/sh
# [gram2-g] verify of batch index-chunks AFTER apply: the chunked-index law (4 pass; the memory law is red on the expanding builder: 79 MB of typed arrays against a budget of 8 MB) and every generations suite (the pass counts of the baseline run recorded in gen-baseline.txt, taken before the batch).
cd /Users/ueli/Documents/semio || exit 1
HERE='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram2-g-tools/staged/index-chunks'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations'
$BUN test "./$G/🧪️tests/🧱️index-chunks/🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)"
python3 - "$G" > /private/tmp/gram2-g-gen-tests.txt <<'PY'
import os,sys
out=[os.path.join(b,n) for b,d,ns in os.walk(sys.argv[1]) if b.endswith("🧪️tests") or "🧪️tests" in b.split(os.sep) for n in ns if n=="🟦️.ts"]
print("\n".join(sorted(out)))
PY
$BUN test $(sed 's#^#./#' /private/tmp/gram2-g-gen-tests.txt) 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"
echo "baseline before the batch:"; cat "$HERE/gen-baseline.txt" 2>/dev/null
