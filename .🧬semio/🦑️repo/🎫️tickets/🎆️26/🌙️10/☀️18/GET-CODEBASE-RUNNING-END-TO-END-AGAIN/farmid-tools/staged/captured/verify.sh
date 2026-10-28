#!/bin/sh
# 🪆 Laws of batch `captured` after apply: sh verify.sh <repository or mirror root>  (default: the root above this batch). BUN=<bun> overrides the runtime.
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${1:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
BUN=${BUN:-bun}
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
B=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
I=$B/🕸️graph/📥️inference
F=$I/📦️project-nodes/🏊️project-farm
"$BUN" test "$F/🧪️tests/🟦️.ts" "$F/🧪️tests/🔒️farm-close/🟦️.ts" "$F/🧪️tests/🪆️captured/🟦️.ts" "$I/🧱️project-assembly/🧪️tests/🟦️.ts" "$I/🔖️stamps/🧪️tests/🟦️.ts" "$I/🧪️tests/🔖️output-stamps/🟦️.ts"
echo "[verify] the next suites need the test-runner envelope for 11 tests (same failure set before and after the batch in a bare mirror); run them through the runner:"
echo "  $B/📥️source/🔏️closure/🚪️command/🧪️tests/🟦️.ts $B/🧭️invocation/🧪️tests/🔗️process-chain/🟦️.ts $B/⌨️command/🧪️tests/🟦️.ts $B/🏛️sealed/🧪️tests/{👥️processes,🗂️store,🔀️interleavings,🚦️trampoline}/🟦️.ts"
