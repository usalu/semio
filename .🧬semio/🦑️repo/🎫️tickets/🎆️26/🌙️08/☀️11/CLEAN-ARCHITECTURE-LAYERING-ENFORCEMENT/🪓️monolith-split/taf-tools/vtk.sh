#!/bin/zsh
# usage: vtk.sh <phase> ; kernel vitest with the hand-built policy (as ikt-tools/vitest.sh)
R=/Users/ueli/Documents/semio
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
KD="$R/🧰️framework/🔨️modules/🎠️kernel"
P="$KD/📦️packages/🟦️typescript"
mkdir -p "$G/taf-vitest-cache"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/taf-vitest-cache\",\"coverageDirectory\":\"$G/taf-vitest-cov\",\"budgetMs\":1500000}"
cd "$P"
perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --testTimeout 120000 --reporter=json --outputFile="$G/taf-vt-$1-kernel.json" > "$G/taf-vt-$1-kernel.log" 2>&1
echo "kernel exit=$?" >> "$G/taf-vt-$1-status.txt"
