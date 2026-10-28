#!/bin/sh
# ✅ laws of batch `envelope` (lane td-graph; AFTER td-leases `envelope`, `candidate` and `nxoracle`): sh verify.sh <root>; BARE bun test (no parent envelope), short in-repo artifacts
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
law 2/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🔄️candidate/🟦️.ts"
law 1/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🔄️settlement/🟦️.ts"
law 15/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts"
[ $FAILED = 0 ] && echo "VERIFY OK envelope" || echo "VERIFY FAIL envelope"; exit $FAILED
