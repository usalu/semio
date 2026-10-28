#!/bin/sh
# ✅ laws of batch `nxoracle` (lane td-graph): sh verify.sh <root>  (after apply; bare `bun test` from the root, artifacts under <root>/.cache/test-sandbox/tgv)
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
law 7/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/📐️portable-model/🟦️.ts"
law 19/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts"
law 20/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🛂️admission/🟦️.ts"
[ $FAILED = 0 ] && echo "VERIFY OK nxoracle" || echo "VERIFY FAIL nxoracle"; exit $FAILED
