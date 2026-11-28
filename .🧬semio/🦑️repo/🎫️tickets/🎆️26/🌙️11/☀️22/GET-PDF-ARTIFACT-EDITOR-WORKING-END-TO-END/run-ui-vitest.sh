#!/bin/sh
# usage: run-ui-vitest.sh <suite path relative to the ui react package root> <log>  (the framework ui react package's vitest, one fork, the repo vitest policy)
S=/Users/ueli/Documents/semio
R="$S/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"
G="$S/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END/🗑️generated"
cd "$R" || exit 2
export SEMIO_TEST_LEVEL=long NODE_OPTIONS=--max-old-space-size=3072
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$R\",\"toolPath\":\"$S/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(command -v node)\",\"coverageRuntime\":\"$(command -v node)\",\"cacheRoot\":\"$G/vitest-cache\",\"coverageDirectory\":\"$G/vitest-coverage\",\"budgetMs\":1500000,\"maxWorkers\":1}"
exec "$(command -v node)" "$S/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native --pool=forks --maxWorkers=1 --no-file-parallelism --reporter=verbose "$1" > "$2" 2>&1
