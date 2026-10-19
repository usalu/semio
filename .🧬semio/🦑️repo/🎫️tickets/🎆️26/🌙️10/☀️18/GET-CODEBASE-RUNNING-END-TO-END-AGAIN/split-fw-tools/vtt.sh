#!/bin/zsh
# usage: vtt.sh <log-name> <config> <filters...>
cd /Users/ueli/Documents/semio
export SEMIO_TICKET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
log="$SEMIO_TICKET_DIR/🗑️generated/split-fw/$1.log"; shift
cfg="$1"; shift
G="$SEMIO_TICKET_DIR/🗑️generated/split-fw"
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PWD\",\"toolPath\":\"$PWD/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(command -v node)\",\"coverageRuntime\":\"$(command -v node)\",\"cacheRoot\":\"$G/vitest-cache\",\"coverageDirectory\":\"$G/coverage\",\"budgetMs\":280000}"
perl -e 'alarm 480; exec @ARGV' -- node --experimental-transform-types ./node_modules/vitest/vitest.mjs run --configLoader native --config "$cfg" "$@" > "$log" 2>&1
echo "exit $?" >> "$log"
