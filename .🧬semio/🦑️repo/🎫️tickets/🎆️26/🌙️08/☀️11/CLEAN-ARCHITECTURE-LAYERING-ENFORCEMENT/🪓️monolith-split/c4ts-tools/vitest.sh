#!/bin/zsh
# usage: vitest.sh <label> <package-dir> <config-rel> [vitest args...]
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4ts-tools/env.sh
LABEL="$1"; P="$2"; CFG="$3"; shift; shift; shift
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
cd "$P"
nice -n 10 perl -e 'alarm 1500; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$CFG" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/c4ts-vt-$LABEL.json" "$@" > "$G/c4ts-vt-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/c4ts-vt-$LABEL.log"
tail -3 "$G/c4ts-vt-$LABEL.log"
