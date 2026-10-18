#!/bin/zsh
# usage: vitest.sh <label> <package-dir> <config-rel> [vitest args...]
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tsx-tools/env.sh
LABEL="$1"; P="$2"; CFG="$3"; shift; shift; shift
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
mkdir -p "$G/tsx-vtcache"
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/tsx-vtcache\",\"coverageDirectory\":\"$G/tsx-vtcov\",\"budgetMs\":900000}"
cd "$P"
nice -n 10 perl -e 'alarm 2400; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$CFG" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/tsx-vt-$LABEL.json" "$@" > "$G/tsx-vt-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/tsx-vt-$LABEL.log"
tail -3 "$G/tsx-vt-$LABEL.log"
python3 "$T/vtsum.py" "$G/tsx-vt-$LABEL.json" "$G/tsx-vtset-$LABEL.txt"
