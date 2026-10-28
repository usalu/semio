#!/bin/sh
# ✅ laws of batch `walks` (lane td-graph): sh verify.sh <root>  (after apply; bare `bun test` from the root, artifacts under <root>/.cache/test-sandbox/tgv)
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
law 4/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🟦️.ts"
law 4/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🔖️output-stamps/🟦️.ts"
[ $FAILED = 0 ] && echo "VERIFY OK walks" || echo "VERIFY FAIL walks"; exit $FAILED
