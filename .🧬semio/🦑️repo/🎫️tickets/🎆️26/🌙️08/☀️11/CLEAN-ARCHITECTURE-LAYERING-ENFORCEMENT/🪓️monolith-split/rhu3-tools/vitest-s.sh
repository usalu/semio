#!/bin/zsh
# usage: vitest-s.sh <label> — the ✏️s dev catalog vitest project (extends the renderer config)
. "$(dirname "$0")/env.sh"
mkdir -p "$G/rhu3-vitest-cache"
P="$R/✏️s/🧑‍💻dev/🧩️catalog/📦️packages/🟦️typescript"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/rhu3-vitest-cache\",\"coverageDirectory\":\"$G/rhu3-vitest-cov\",\"budgetMs\":1500000}"
cd "$P"
perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "../../🧪️tests/🎚️config/🟦️.ts" --testTimeout 120000 --reporter=default
