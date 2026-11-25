#!/bin/sh
# 🧪️ One vitest suite of the staffage in one process, verbose, optionally with a name filter: suite.sh <tag> <suite folder> [<name filter>] writes 🗑️generated/matrix/MX/vt/<tag>.log and prints its totals (run it through `repo capacity run --memory=2500`).
R=/Users/ueli/Documents/semio
S="$R/🎓️teaching/🏛️architecture/🧍️staffage"
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🗑️generated/matrix/MX/vt"
B=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "$G"
cd "$S/📦️packages/🟦️typescript" || exit 2
if [ -n "$3" ]; then
  timeout "${VT_TIMEOUT:-3000}" "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$2/🟦️.ts" -t "$3" > "$G/$1.log" 2>&1
else
  timeout "${VT_TIMEOUT:-3000}" "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$2/🟦️.ts" > "$G/$1.log" 2>&1
fi
echo "exit $?"
grep -E '^ *(Test Files|Tests|Duration) ' "$G/$1.log"
grep -E '✗|×|FAIL|AssertionError|Error:' "$G/$1.log" | head -40
