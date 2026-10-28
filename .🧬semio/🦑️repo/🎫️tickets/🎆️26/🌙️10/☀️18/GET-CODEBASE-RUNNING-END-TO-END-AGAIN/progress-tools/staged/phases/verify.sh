#!/bin/sh
# usage: verify.sh <mirror-root>   (applies this batch to the mirror, then runs the laws)
set -eu
ROOT="$1"; HERE="$(cd "$(dirname "$0")" && pwd)"
PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
export SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:?set a short artifact dir under the ticket generated folder}"
B="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
for d in "$ROOT/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress" "$B/🚪️door/🧪️tests" "$B/⏱️profile/🧪️tests"; do (cd "$d" && bun test --timeout 60000 ./🟦️.ts) || true; done
echo "phases batch applied; compare the (fail) lines with the baseline in C/result-progress.md"
