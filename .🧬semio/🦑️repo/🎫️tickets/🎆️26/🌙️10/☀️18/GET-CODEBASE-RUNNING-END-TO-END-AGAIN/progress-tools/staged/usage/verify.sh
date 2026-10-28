#!/bin/sh
# usage: verify.sh <mirror-root>   (applies this batch to the mirror, then runs the laws)
set -eu
ROOT="$1"; HERE="$(cd "$(dirname "$0")" && pwd)"
PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests"
SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:?set a short artifact dir under the ticket generated folder}" bun test --timeout 60000 ./🟦️.ts
echo "usage batch: GREEN"
