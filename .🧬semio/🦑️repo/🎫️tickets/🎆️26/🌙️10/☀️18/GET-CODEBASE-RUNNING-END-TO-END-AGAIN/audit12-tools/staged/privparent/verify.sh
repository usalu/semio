#!/bin/sh
# 📍 laws of batch `privparent` (lane audit12): sh verify.sh <root>  (after apply). Red on base: the Python/node oracle (model 248 < real 259 units for the 27-unit live checkout) and, bare on live with the default parent, the denial law ("Observation block staging path exceeds256 characters"; suite 9/4/1); green: oracle OK and the bare suite at the optcache lane's documented 10/3/1 (the 3 maintenance-worker laws), with no private repository left below the chosen parent.
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"; applied
python3 -I "$HERE/law_privparent.py" "$ROOT" && echo "ok   oracle" || { echo "FAIL oracle"; FAILED=1; }
LAWENV="-u SEMIO_NX_FIXED_STORAGE -u NX_CACHE_DIRECTORY -u SEMIO_PRIVATE_GRAPH_WORKSPACE_PARENT SEMIO_TEST_ARTIFACT_DIR=$ART" law 10/3 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts"
left=$(ls /tmp/sg 2>/dev/null | grep -c '^g[A-Za-z0-9]\{6\}$'); [ "$left" = 0 ] && echo "ok   no private repository left in /tmp/sg" || { echo "FAIL $left private repositories left in /tmp/sg"; FAILED=1; }
parse; mode
[ $FAILED = 0 ] && echo "VERIFY OK privparent" || echo "VERIFY FAIL privparent"; exit $FAILED
