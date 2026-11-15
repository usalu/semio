#!/bin/sh
# usage: pipeline-test.sh <filter...>   runs targeted staffage vitest cases; output under T/🗑️generated/pipeline/
set -e
ROOT=/Users/ueli/Documents/semio
T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "$T/🗑️generated/pipeline"
cd "$ROOT"
exec "$BUN" node_modules/vitest/vitest.mjs run --configLoader native --config "🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts" --maxWorkers=2 "$@"
