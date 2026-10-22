#!/bin/bash
# Runs one React vitest suite live (no overlay), nice, one process: vt-live.sh <path-substring>
set -eu
ROOT=/Users/ueli/Documents/semio
E="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
CD="$ROOT/$E/🎯️targets/⚛️react/🧪️tests/🎚️config"; PKG="$ROOT/$E/🎯️targets/⚛️react/📦️packages/🟦️typescript"
mkdir -p /private/tmp/triage-r4/vitest-cache
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PKG\",\"toolPath\":\"$ROOT/node_modules/vitest/vitest.mjs\",\"runtime\":\"bun\",\"coverageRuntime\":\"bun\",\"cacheRoot\":\"/private/tmp/triage-r4/vitest-cache\",\"coverageDirectory\":\"/private/tmp/triage-r4/vitest-cov\",\"budgetMs\":600000}"
cd "$PKG"
SEMIO_TEST_LEVEL=long nice -n 10 bun "$ROOT/node_modules/vitest/vitest.mjs" run --config "$CD/🟦️.ts" --configLoader native --testTimeout 120000 --hookTimeout 120000 "$@"
