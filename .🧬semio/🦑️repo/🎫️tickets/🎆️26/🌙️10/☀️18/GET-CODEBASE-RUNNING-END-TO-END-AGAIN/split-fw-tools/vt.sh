#!/bin/zsh
# usage: vt.sh <log-name> <filters...>
cd /Users/ueli/Documents/semio
export SEMIO_TICKET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
log="$SEMIO_TICKET_DIR/🗑️generated/split-fw/$1.log"; shift
perl -e 'alarm 300; exec @ARGV' -- node --experimental-transform-types ./node_modules/vitest/vitest.mjs run --configLoader native --config '🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts' "$@" > "$log" 2>&1
echo "exit $?" >> "$log"
