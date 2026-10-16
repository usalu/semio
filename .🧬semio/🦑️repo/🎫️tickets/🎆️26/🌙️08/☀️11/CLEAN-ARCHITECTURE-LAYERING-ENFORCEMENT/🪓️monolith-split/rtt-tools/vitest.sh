#!/bin/zsh
# usage: vitest.sh <level> <label> <filter...>   -> JSON report at $G/rtt-<label>.json, log at $G/rtt-<label>.log
R=/Users/ueli/Documents/semio
G=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated
P=$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript
LEVEL=$1; LABEL=$2; shift 2
mkdir -p "$G/rtt-vitest-cache"
export SEMIO_TEST_LEVEL=$LEVEL
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/rtt-vitest-cache\",\"coverageDirectory\":\"$G/rtt-vitest-cov\",\"budgetMs\":1800000}"
cd "$P"
perl -e 'alarm 1750; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "../../🧪️tests/🎚️config/🟦️.ts" --testTimeout 120000 --reporter=json --outputFile="$G/rtt-$LABEL.json" "$@" > "$G/rtt-$LABEL.log" 2>&1
echo "exit $?" >> "$G/rtt-$LABEL.log"
