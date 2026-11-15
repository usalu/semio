#!/bin/sh
# usage: kernel-test.sh bun|node NAME [vitest filter...]  (targeted kernel tests, output under 🗑️generated/kernel/)
cd /Users/ueli/Documents/semio || exit 1
RT="$1"; NAME="$2"; shift 2
G="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🗑️generated/kernel"
mkdir -p "$G/cache"
if [ "$RT" = bun ]; then BIN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1); else BIN="$HOME/.local/bin/node"; fi
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PWD\",\"toolPath\":\"$PWD/node_modules/vitest/vitest.mjs\",\"runtime\":\"$BIN\",\"coverageRuntime\":\"$BIN\",\"cacheRoot\":\"$G/cache\",\"coverageDirectory\":\"$G/coverage\",\"budgetMs\":600000,\"maxWorkers\":2}"
"$BIN" node_modules/vitest/vitest.mjs run --configLoader native --config "🧰️framework/🔨️modules/🧊️3d/🧪️tests/🎚️config/🟦️.ts" --maxWorkers=2 "$@" > "$G/$NAME.$RT.out" 2>&1
echo "exit $?" >> "$G/$NAME.$RT.out"
tail -60 "$G/$NAME.$RT.out"
