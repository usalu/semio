#!/bin/sh
# 📇 laws of batch `farmexports` (lane audit12): sh verify.sh <root>  (after apply). Red on live base: observation 51/1 (declaration roster vs runtime exports) and Rust Source 3/1 (declared exports); green: 52/0 and 4/0 (the Rust Source file runs bare on a short root and with the checkout scratch artifact directory on a long mirror root, as the rusttests envelope verify does).
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"; applied
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
LAWENV="-u SEMIO_TEST_ARTIFACT_DIR" law 52/0 "$B/👁️observation/🧪️tests/🟦️.ts"
CHECKOUT=$([ ${#ROOT} -gt 40 ] && echo "SEMIO_TEST_ARTIFACT_DIR=$ART" || echo "-u SEMIO_TEST_ARTIFACT_DIR")
LAWENV="$CHECKOUT" law 4/0 "$B/🕸️graph/🦀️rust/📥️source/🧪️tests/🟦️.ts"
python3 -I -c 'import json,sys;json.load(open(sys.argv[1],encoding="utf8"))' "$ROOT/$B/👁️observation/🧪️tests/🧫️fixtures/🔣️.json" && echo "ok   fixture JSON" || FAILED=1
mode
[ $FAILED = 0 ] && echo "VERIFY OK farmexports" || echo "VERIFY FAIL farmexports"; exit $FAILED
