#!/bin/sh
# ✅ laws of batch `candidate` (lane td-graph): sh verify.sh <root> (after apply; the file is envelope-only until batch `envelope`: run inside the official script process envelope)
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"
python3 -I "$HERE/stage.py" check "$ROOT" | grep -v "^applied" && { echo "batch not applied at $ROOT"; exit 2; }
envlaw 2/0 "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🔄️candidate/🟦️.ts"
[ $FAILED = 0 ] && echo "VERIFY OK candidate" || echo "VERIFY FAIL candidate"; exit $FAILED
