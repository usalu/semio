#!/bin/sh
# usage: site-loading-test.sh react|site|kernel NAME [vitest arguments...]  (native vitest under the repo's bun while the repo graph service stalls; log under 🗑️generated/site-loading/)
cd /Users/ueli/Documents/semio || exit 1
WHICH="$1"; NAME="$2"; shift 2
G="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🗑️generated/site-loading"
mkdir -p "$G/cache"
BIN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
case "$WHICH" in
  react) CONFIG="🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts" ;;
  site) CONFIG="🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🧪️tests/🎚️config/🟦️.ts" ;;
  kernel) CONFIG="🧰️framework/🔨️modules/🧊️3d/🧪️tests/🎚️config/🟦️.ts" ;;
esac
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$PWD\",\"toolPath\":\"$PWD/node_modules/vitest/vitest.mjs\",\"runtime\":\"$BIN\",\"coverageRuntime\":\"$BIN\",\"cacheRoot\":\"$G/cache\",\"coverageDirectory\":\"$G/coverage\",\"budgetMs\":1800000,\"maxWorkers\":2}"
"$BIN" node_modules/vitest/vitest.mjs run --configLoader native --config "$CONFIG" --maxWorkers=2 "$@" > "$G/$NAME.log" 2>&1
echo "exit $?" >> "$G/$NAME.log"
grep -a -E "✓|×|FAIL|Test Files|Tests |exit |Error|expected|AssertionError" "$G/$NAME.log" | cut -c1-300 | tail -70
