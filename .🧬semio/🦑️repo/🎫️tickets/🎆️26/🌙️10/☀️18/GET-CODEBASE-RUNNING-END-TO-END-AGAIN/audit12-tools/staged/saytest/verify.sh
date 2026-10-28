#!/bin/sh
# 🗣 laws of batch `saytest` (lane audit12): sh verify.sh <root>  (after apply; the root path must leave the fixture output under 129 units: live or a short mirror). Red on live base: the diagnostics budget law 0/1 (a German hidden-lines row ahead of the newest lines); green 1/0, in German, English and both-language environments. The rest of the file needs the runner envelope (pre-existing).
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"; applied
T="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts"
for env in "SEMIO_LANGUAGE=de" "SEMIO_LANGUAGE=en" "-u SEMIO_LANGUAGE LC_ALL=C"; do LAWENV="$env SEMIO_TEST_ARTIFACT_DIR=$ART" law 1/0 "$T" -t "compiler diagnostics stay inside their line budget"; done
parse; mode
[ $FAILED = 0 ] && echo "VERIFY OK saytest" || echo "VERIFY FAIL saytest"; exit $FAILED
