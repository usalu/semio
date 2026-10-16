#!/bin/zsh
# usage: vitest.sh <label> <filter...>   -> JSON report at $G/qsd-<label>.json, log at $G/qsd-<label>.log
R=/Users/ueli/Documents/semio
G=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated
P=$R/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript
LABEL=$1; shift 1
mkdir -p "$G/qsd-vitest-cache"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/qsd-vitest-cache\",\"coverageDirectory\":\"$G/qsd-vitest-cov\",\"budgetMs\":1800000}"
cd "$P"
perl -e 'alarm 1750; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "../../🧪️tests/🎚️config/🟦️.ts" --testTimeout 120000 --reporter=json --outputFile="$G/qsd-$LABEL.json" "$@" > "$G/qsd-$LABEL.log" 2>&1
echo "exit $?" >> "$G/qsd-$LABEL.log"
