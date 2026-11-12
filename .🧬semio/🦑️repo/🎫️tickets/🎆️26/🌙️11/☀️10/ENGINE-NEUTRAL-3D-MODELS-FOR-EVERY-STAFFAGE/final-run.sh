#!/bin/sh
# usage: final-run.sh <name> <config> [vitest args...]
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE"
G="$T/🗑️generated/final"
name=$1; config=$2; shift 2
cd "${RUNCWD:-$R}"
exec /private/tmp/claude-501/sg/bin/bun $R/node_modules/vitest/vitest.mjs run --configLoader native --config "$config" --reporter=json --outputFile="$G/$name.json" --maxWorkers=2 "$@" > "$G/$name.txt" 2>&1
