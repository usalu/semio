#!/usr/bin/env bash
# 🧪 Runs renderer-react vitest files: ./run-renderer-vitest.sh <test path relative to the package>...
set -e
PKG="C:/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
OUT="C:/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END/🗑️generated/ui3"
cd "$PKG"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PKG\",\"toolPath\":\"C:/semio/node_modules/vitest/vitest.mjs\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"$OUT/cache\",\"coverageDirectory\":\"$OUT/cov\",\"budgetMs\":240000}"
node C:/semio/node_modules/vitest/vitest.mjs run --configLoader native --config "../../🧪️tests/🎚️config/🟦️.ts" "$@"
