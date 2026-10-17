#!/bin/zsh
# usage: vitest.sh <configOwnerDir (absolute)> <config path relative to repo> <test file relative to repo> [extra vitest args]; supplies an explicit Vitest policy whose cache lives in the bd8 generated folder
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
OWNER="$1"; CONFIG="$2"; TEST="$3"; shift 3
mkdir -p "$F/🗑️generated/bd8-vitest-cache" "$F/🗑️generated/bd8-vitest-coverage"
export SEMIO_VITEST_POLICY=$(python3 -c 'import json,sys; print(json.dumps({"version":1,"cwd":sys.argv[1],"toolPath":sys.argv[2],"runtime":"bun","coverageRuntime":"bun","cacheRoot":sys.argv[3],"coverageDirectory":sys.argv[4],"budgetMs":120000}))' "$OWNER" "$R/node_modules/.bin/vitest" "$F/🗑️generated/bd8-vitest-cache" "$F/🗑️generated/bd8-vitest-coverage")
cd "$R" && ulimit -n 65536 && nice -n 10 perl -e 'alarm 590; exec @ARGV' -- node_modules/.bin/vitest run --config "$CONFIG" --testTimeout 120000 "$TEST" "$@"
