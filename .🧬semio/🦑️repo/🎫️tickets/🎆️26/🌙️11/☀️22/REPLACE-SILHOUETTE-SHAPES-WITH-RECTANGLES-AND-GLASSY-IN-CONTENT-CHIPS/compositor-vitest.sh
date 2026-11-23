#!/bin/sh
# usage: compositor-vitest.sh <ui-react test filter...>; native fallback runner, output goes to 🗑️generated
R=/Users/ueli/Documents/semio
UI="$R/🧰️framework/🔨️modules/🖱️ui"
CWD="$UI/🎯️targets/⚛️react/📦️packages/🟦️typescript"
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS/🗑️generated"
mkdir -p "$G/cache"
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$CWD\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"node\",\"coverageRuntime\":\"node\",\"cacheRoot\":\"$G/cache\",\"coverageDirectory\":\"$G/cov\",\"budgetMs\":60000}"
cd "$CWD" && exec node "$R/node_modules/vitest/vitest.mjs" run --config "${COMPOSITOR_CONFIG:-$UI/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts}" --configLoader native --testTimeout 60000 "$@"
