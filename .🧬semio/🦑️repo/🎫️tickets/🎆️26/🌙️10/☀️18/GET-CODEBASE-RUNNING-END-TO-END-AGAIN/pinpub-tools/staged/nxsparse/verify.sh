#!/bin/sh
# ✅ laws of batch `nxsparse` (lane pinpub): sh verify.sh <root>  (after apply). The sparse law is RED on base (export missing + negative control of the former rule); the Nx graph store suites keep their base failure sets under bare `bun test` (they need the runner envelope): yield 11/0, ambient-scope 2/0, pool-order 7/0.
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
N="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests"
law 1/0 "$N/🪶️sparse/🟦️.ts"
law 11/0 "$N/🤝️yield/🟦️.ts"
law 2/0 "$N/🧭️ambient-scope/🟦️.ts"
law 7/0 "$N/🔓️pool-order/🟦️.ts"
[ $FAILED = 0 ] && echo "VERIFY OK nxsparse" || echo "VERIFY FAIL nxsparse"; exit $FAILED
