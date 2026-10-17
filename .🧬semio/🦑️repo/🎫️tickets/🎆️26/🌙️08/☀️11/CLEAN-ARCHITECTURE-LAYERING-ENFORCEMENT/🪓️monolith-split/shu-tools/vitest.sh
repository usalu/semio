#!/bin/zsh
# usage: vitest.sh <label> [vitest args...] -> G/shu-vt-<label>.json/.log (renderer react package, long level)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/shu-tools/env.sh
LABEL="$1"; shift
P="$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
mkdir -p "$G/shu-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/shu-vitest-cache\",\"coverageDirectory\":\"$G/shu-vitest-cov\",\"budgetMs\":580000}"
cd "$P"
perl -e 'alarm 585; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "${VITEST_CONFIG:-../../🧪️tests/🎚️config/🟦️.ts}" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/shu-vt-$LABEL.json" "$@" > "$G/shu-vt-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/shu-vt-$LABEL.log"
