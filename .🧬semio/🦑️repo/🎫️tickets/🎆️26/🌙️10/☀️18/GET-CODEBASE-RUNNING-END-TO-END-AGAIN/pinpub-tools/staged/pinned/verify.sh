#!/bin/sh
# ✅ laws of batch `pinned` (lane pinpub): sh verify.sh <root>  (after apply; td-graph `demand` + `nxoracle` landed first). MIRROR=1 skips the Nx cache law: a mirror under $TMPDIR exceeds the 104-byte Unix socket path of the Nx namespace ("Funded Nx socket endpoint byte limit exceeded"), the live root does not.
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph"
if [ "$MIRROR" = 1 ]; then law 11/0 "$G/📌️pinned-publication/🟦️.ts" -t "^(?!each pinned publication receives)"; else law 12/0 "$G/📌️pinned-publication/🟦️.ts"; fi
law 20/0 "$G/🛂️admission/🟦️.ts"
[ $FAILED = 0 ] && echo "VERIFY OK pinned" || echo "VERIFY FAIL pinned"; exit $FAILED
