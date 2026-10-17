#!/bin/zsh
# usage: vt.sh <phase> [filter]  runs the hub-tests vitest route and writes hsc-vt-<phase>.json/.log
. "$(dirname "$0")/env.sh"
phase="$1"; filter="$2"
cfg="$R/🌎️hub/🧪️tests/🎚️config/🟦️.ts"
cwd="$R/🌎️hub/📦️packages/🟦️typescript"
export SEMIO_TEST_LEVEL=long
mkdir -p "$G/hsc-vitest-cache"
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$cwd\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/hsc-vitest-cache\",\"coverageDirectory\":\"$G/hsc-vitest-cov\",\"budgetMs\":1500000}"
cd "$cwd" || exit 9
start=$(date +%s)
perl -e 'alarm ($ENV{ALARM} || 900); exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$cfg" --testTimeout 60000 --reporter=json --outputFile="$G/hsc-vt-$phase.json" $filter > "$G/hsc-vt-$phase.log" 2>&1
echo "$phase exit=$? secs=$(( $(date +%s) - start ))" >> "$G/hsc-vt-status.log"
