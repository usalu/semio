#!/bin/zsh
# usage: vitest.sh <label> <cwd-relative-to-R> <config-relative-to-cwd> [vitest args...] -> G/tso-vt-<label>.json/.log/.set
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tso-tools/env.sh
LABEL="$1"; PKG="$2"; CFG="$3"; shift 3
P="$R/$PKG"
mkdir -p "$G/tso-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/tso-vitest-cache\",\"coverageDirectory\":\"$G/tso-vitest-cov\",\"budgetMs\":${TSO_BUDGET_MS:-1450000}}"
cd "$P"
perl -e 'alarm 1500; exec @ARGV' -- nice -n 10 ${TSO_RUNTIME:-node} "$R/node_modules/vitest/vitest.mjs" run --config "$CFG" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/tso-vt-$LABEL.json" "$@" > "$G/tso-vt-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/tso-vt-$LABEL.log"
python3 "$T/vtsum.py" "$G/tso-vt-$LABEL.json" "$G/tso-vt-$LABEL.set" 2>&1 | tail -3
tail -2 "$G/tso-vt-$LABEL.log"
