#!/bin/sh
# usage: verify.sh <mirror-root>   (applies this batch to the mirror, then runs the laws)
set -eu
ROOT="$1"; HERE="$(cd "$(dirname "$0")" && pwd)"
PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
export SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:?set a short artifact dir under the ticket generated folder}"
for d in "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📣️diagnostics/🧪️tests" "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🧪️tests"; do (cd "$ROOT/$d" && bun test --timeout 60000 ./🟦️.ts) || true; done
for f in "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts" "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"; do bun build --no-bundle "$ROOT/$f" > /dev/null || exit 1; done
echo "say batch applied; compare the (fail) lines with the baseline in C/result-progress.md"
