#!/bin/sh
# usage: pipeline-test-react.sh <filter...>   runs targeted staffage-react vitest cases (cwd = the react package, as the repo script does)
set -e
ROOT=/Users/ueli/Documents/semio
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
PKG="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/📦️packages/🟦️typescript"
cd "$PKG"
exec "$BUN" "$ROOT/node_modules/vitest/vitest.mjs" run --configLoader native --config "../../🧪️tests/🎚️config/🟦️.ts" --maxWorkers=2 "$@"
