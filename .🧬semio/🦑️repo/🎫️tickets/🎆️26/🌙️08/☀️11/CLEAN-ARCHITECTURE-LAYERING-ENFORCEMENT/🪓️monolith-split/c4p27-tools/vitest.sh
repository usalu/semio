#!/bin/zsh
# usage: vitest.sh <label> [vitest args...] -> G/c4p27-vt-<label>.json/.log (renderer react package, long level)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p27-tools/env.sh
LABEL="$1"; shift
P="$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
mkdir -p "$G/c4p27-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/c4p27-vitest-cache\",\"coverageDirectory\":\"$G/c4p27-vitest-cov\",\"budgetMs\":580000}"
cd "$P"
perl -e 'alarm 585; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "${VITEST_CONFIG:-../../🧪️tests/🎚️config/🟦️.ts}" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/c4p27-vt-$LABEL.json" "$@" > "$G/c4p27-vt-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/c4p27-vt-$LABEL.log"
