#!/bin/sh
# usage: run-actor-vitest.sh <suite path relative to the actor package root> <log> [extra vitest args]
S=/Users/ueli/Documents/semio
R="$S/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript"
cd "$R" || exit 2
SUITE="$1"; LOG="$2"; shift 2
export NODE_OPTIONS=--max-old-space-size=2048
exec "$(command -v node)" "$S/node_modules/vitest/vitest.mjs" run --config ../../🧪️tests/🎚️config/🟦️.ts --configLoader native --pool=forks --maxWorkers=1 --no-file-parallelism --reporter=verbose "$SUITE" "$@" > "$LOG" 2>&1
