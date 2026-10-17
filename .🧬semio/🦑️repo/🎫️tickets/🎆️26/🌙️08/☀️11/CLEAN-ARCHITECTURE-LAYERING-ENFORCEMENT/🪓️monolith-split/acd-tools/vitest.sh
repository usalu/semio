#!/bin/zsh
# usage: vitest.sh <label> <package-dir-relative-to-R> <config-relative-to-package> [vitest args...] -> G/acd-vt-<label>.json/.log
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/acd-tools/env.sh
LABEL="$1"; PKG="$2"; CFG="$3"; shift 3
P="$R/$PKG"
mkdir -p "$G/acd-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/acd-vitest-cache\",\"coverageDirectory\":\"$G/acd-vitest-cov\",\"budgetMs\":580000}"
cd "$P"
perl -e 'alarm 585; exec @ARGV' -- nice -n 10 node "$R/node_modules/vitest/vitest.mjs" run --config "$CFG" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/acd-vt-$LABEL.json" "$@" > "$G/acd-vt-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/acd-vt-$LABEL.log"
python3 "$T/vtsum.py" "$G/acd-vt-$LABEL.json" "$G/acd-vt-$LABEL.set" 2>&1 | tail -3
tail -2 "$G/acd-vt-$LABEL.log"
