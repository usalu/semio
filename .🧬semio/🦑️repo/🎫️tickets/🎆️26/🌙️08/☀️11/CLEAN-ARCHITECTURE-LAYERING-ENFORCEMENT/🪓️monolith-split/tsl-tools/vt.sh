#!/bin/zsh
# usage: vt.sh <phase> <label> <cwd> <config> [vitest args...]  ; json -> 🗑️generated/tsl-vt-<phase>-<label>.json
R=/Users/ueli/Documents/semio
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
P="$1"; L="$2"; CWD="$3"; CFG="$4"; shift 4
mkdir -p "$G/tsl-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$CWD\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/tsl-vitest-cache\",\"coverageDirectory\":\"$G/tsl-vitest-cov\",\"budgetMs\":1500000}"
cd "$CWD"
perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$CFG" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/tsl-vt-$P-$L.json" "$@" > "$G/tsl-vt-$P-$L.log" 2>&1
echo "$L exit=$? $(date +%T)" >> "$G/tsl-vt-$P-status.txt"
