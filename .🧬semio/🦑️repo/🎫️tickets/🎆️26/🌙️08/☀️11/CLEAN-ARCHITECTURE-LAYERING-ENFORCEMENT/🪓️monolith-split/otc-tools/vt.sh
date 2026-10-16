#!/bin/zsh
# usage: vt.sh <phase> <label> <cwd> <config> [vitest args...]  ; json -> 🗑️generated/otc-vt-<phase>-<label>.json
R=/Users/ueli/Documents/semio
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
P="$1"; L="$2"; CWD="$3"; CFG="$4"; shift 4
mkdir -p "$G/otc-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$CWD\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/otc-vitest-cache\",\"coverageDirectory\":\"$G/otc-vitest-cov\",\"budgetMs\":1500000}"
cd "$CWD"
perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$CFG" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/otc-vt-$P-$L.json" "$@" > "$G/otc-vt-$P-$L.log" 2>&1
echo "$L exit=$? $(date +%T)" >> "$G/otc-vt-$P-status.txt"
