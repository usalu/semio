#!/bin/zsh
# usage: vitest.sh <react|core> <label> [vitest args...]
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p51-tools/env.sh
KIND="$1"; LABEL="$2"; shift; shift
if [ "$KIND" = react ]; then
  P="$Q/🎯️targets/⚛️react/📦️packages/🟦️typescript"; CFG="../../🧪️tests/🎚️config/🟦️.ts"
else
  P="$Q/📦️packages/🟦️typescript"; CFG="../../🧪️tests/🎚️config/🟦️.ts"
fi
mkdir -p "$G/c4p51-vitest-cache"
export SEMIO_TEST_LEVEL=${SEMIO_TEST_LEVEL:-long}
cd "$P"
nice -n 10 perl -e 'alarm 900; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$CFG" --testTimeout 120000 --hookTimeout 120000 --reporter=json --outputFile="$G/c4p51-vt-$KIND-$LABEL.json" "$@" > "$G/c4p51-vt-$KIND-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/c4p51-vt-$KIND-$LABEL.log"
