#!/bin/zsh
# usage: vtcad.sh <phase> [vitest args...] -> G/c4p3-vt-<phase>-cad.json/.log
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p3-tools/env.sh
PH="$1"; shift
CWD="$R/✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript"
CFG="$R/✏️s/🧑‍💻dev/📐️cad/🧪️tests/🎚️config/🟦️.ts"
mkdir -p "$G/c4p3-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$CWD\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/c4p3-vitest-cache\",\"coverageDirectory\":\"$G/c4p3-vitest-cov\",\"budgetMs\":1500000}"
cd "$CWD"
perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$CFG" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/c4p3-vt-$PH-cad.json" "$@" > "$G/c4p3-vt-$PH-cad.log" 2>&1
echo "exit=$?" >> "$G/c4p3-vt-$PH-cad.log"
