#!/bin/sh
R=/Users/ueli/Documents/semio
CWD="$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS/🗑️generated"
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$CWD\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"$G/cache\",\"coverageDirectory\":\"$G/cov\",\"budgetMs\":60000}"
cd "$CWD" && SEMIO_TEST_LEVEL=long exec node $R/node_modules/vitest/vitest.mjs run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native --testTimeout 60000 "$@"
