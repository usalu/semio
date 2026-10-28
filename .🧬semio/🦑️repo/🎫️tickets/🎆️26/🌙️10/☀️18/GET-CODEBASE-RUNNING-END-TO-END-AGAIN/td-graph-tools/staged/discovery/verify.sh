#!/bin/sh
# ✅ laws of batch `discovery` (lane td-graph): sh verify.sh <root> (after apply; bare, short in-repo artifacts)
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
law 6/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🌐️source-discovery/🟦️.ts"
[ $FAILED = 0 ] && echo "VERIFY OK discovery" || echo "VERIFY FAIL discovery"; exit $FAILED
