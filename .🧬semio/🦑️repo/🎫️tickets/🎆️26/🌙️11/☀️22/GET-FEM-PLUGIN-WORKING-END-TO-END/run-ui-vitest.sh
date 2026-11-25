#!/usr/bin/env bash
# 🧪 Runs ui-react vitest files: ./run-ui-vitest.sh <filter>...
PKG="C:/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"
OUT="C:/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END/🗑️generated/ui3"
cd "$PKG"
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PKG\",\"toolPath\":\"C:/semio/node_modules/vitest/vitest.mjs\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"$OUT/cache\",\"coverageDirectory\":\"$OUT/cov\",\"budgetMs\":240000}"
node C:/semio/node_modules/vitest/vitest.mjs run --configLoader native --config "../../🧪️tests/🎚️config/🟦️.ts" "$@"
