#!/bin/sh
ROOT=/Users/ueli/Documents/semio
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/📦️packages/🟦️typescript" 2>/dev/null || cd "$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site"
exec "$BUN" "$ROOT/node_modules/vitest/vitest.mjs" run --configLoader native --config "$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🧪️tests/🎚️config/🟦️.ts" --maxWorkers=2 "$@"
