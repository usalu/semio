#!/bin/bash
# usage: vt.sh <tag> <vitest filters...>  (direct vitest over the react package config; no repo graph)
R=/Users/ueli/Documents/semio; C="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"
T=$1; shift
P="$R/♻️mit-bestand/🧺️demonstrator"
CFG="$P/🧪️tests/🎚️config/🟦️.ts"
VT=/Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs
export PATH="/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH"
export PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright}"
export SEMIO_LANGUAGE=en SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$VT\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"$G/vt-cache\",\"coverageDirectory\":\"$G/vt-cov\",\"budgetMs\":120000}"
cd "$P" && node "$VT" run --config "$CFG" --configLoader native --testTimeout ${TEST_TIMEOUT_MS:-120000} "$@" > "$G/demo-$T.txt" 2>&1; echo $? > "$G/demo-$T.rc"
