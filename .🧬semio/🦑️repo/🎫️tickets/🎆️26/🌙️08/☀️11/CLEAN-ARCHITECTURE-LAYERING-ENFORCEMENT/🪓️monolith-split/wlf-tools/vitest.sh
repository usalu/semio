#!/bin/zsh
# usage: vitest.sh <label> [vitest args...] -> G/wlf-vt-<label>.json/.log (renderer react package, long level)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tsr-tools/env.sh
LABEL="$1"; shift
P="$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/${TARGET:-⚛️react}/📦️packages/🟦️typescript"
mkdir -p "$G/wlf-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/wlf-vitest-cache\",\"coverageDirectory\":\"$G/wlf-vitest-cov\",\"budgetMs\":2350000}"
cd "$P"
nice -n 10 perl -e 'alarm 2400; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "${VITEST_CONFIG:-../../🧪️tests/🎚️config/🟦️.ts}" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/wlf-vt-$LABEL.json" "$@" > "$G/wlf-vt-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/wlf-vt-$LABEL.log"
