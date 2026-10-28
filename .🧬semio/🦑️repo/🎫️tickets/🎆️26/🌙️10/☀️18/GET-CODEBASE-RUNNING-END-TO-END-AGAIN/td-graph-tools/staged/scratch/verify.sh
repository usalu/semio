#!/bin/sh
# ✅ laws of batch `scratch` (lane td-graph): sh verify.sh <root>  (after apply; bare `bun test` from the root, artifacts under <root>/.cache/test-sandbox/tgv)
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
law 7/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests/🟦️.ts"
law 7/0 "🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🧪️tests/🟦️.ts"
[ $FAILED = 0 ] && echo "VERIFY OK scratch" || echo "VERIFY FAIL scratch"; exit $FAILED
