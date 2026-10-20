#!/bin/sh
# usage: run-tests.sh [ROOT]   (ROOT = a checkout or an overlay copy; default: the live repository)  Runs the three affected suites one at a time, each with an explicit ./ path.
set -eu
ROOT="${1:-$(cd "$(dirname "$0")/../../../../../../../.." && pwd)}"
cd "$ROOT"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BIN="${SEMIO_TEST_BAZEL_BINARY:-}"
for t in "$L/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️platform/🟦️.ts" "$L/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts" "$L/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🧪️tests/📥️pins/🟦️.ts"; do
  echo "== $t"
  SEMIO_TEST_BAZEL_BINARY="$BIN" perl -e 'alarm 900; exec @ARGV' -- bun test "./$t"
done
