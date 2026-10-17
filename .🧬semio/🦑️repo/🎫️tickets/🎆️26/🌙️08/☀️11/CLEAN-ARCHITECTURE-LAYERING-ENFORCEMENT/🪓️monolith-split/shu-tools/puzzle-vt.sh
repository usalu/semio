#!/bin/zsh
# usage: puzzle-vt.sh <label> -> G/shu-puzzle-<label>.log (puzzle renderer-contract vitest; contains the brush-mesh-upload importer)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/shu-tools/env.sh
P="$R/✏️s/🔌️plugins/🧩️puzzle"
mkdir -p "$G/shu-vitest-cache"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/shu-vitest-cache\",\"coverageDirectory\":\"$G/shu-vitest-cov\",\"budgetMs\":580000}"
cd "$P"
nice -n 10 perl -e 'alarm 585; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "🧪️tests/🎚️renderer-contract/🟦️.ts" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/shu-puzzle-$1.json" > "$G/shu-puzzle-$1.log" 2>&1
echo "exit=$?" >> "$G/shu-puzzle-$1.log"
