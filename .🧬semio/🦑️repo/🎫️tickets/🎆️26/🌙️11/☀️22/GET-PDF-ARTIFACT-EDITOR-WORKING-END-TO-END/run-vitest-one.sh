#!/bin/sh
# usage: run-vitest-one.sh <suite path relative to the react package root> <log> [extra vitest args]
S=/Users/ueli/Documents/semio
R="$S/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
G="$S/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END/🗑️generated"
cd "$R" || exit 2
SUITE="$1"; LOG="$2"; shift 2
export SEMIO_TEST_LEVEL=long NODE_OPTIONS=--max-old-space-size=3072
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$R\",\"toolPath\":\"$S/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(command -v node)\",\"coverageRuntime\":\"$(command -v node)\",\"cacheRoot\":\"$G/vitest-cache\",\"coverageDirectory\":\"$G/vitest-coverage\",\"budgetMs\":1500000,\"maxWorkers\":1}"
exec "$(command -v node)" "$S/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native --testTimeout 1500000 --hookTimeout 1500000 --teardownTimeout 1500000 --pool=forks --maxWorkers=1 --no-file-parallelism --silent=false --reporter=verbose "$SUITE" "$@" > "$LOG" 2>&1
