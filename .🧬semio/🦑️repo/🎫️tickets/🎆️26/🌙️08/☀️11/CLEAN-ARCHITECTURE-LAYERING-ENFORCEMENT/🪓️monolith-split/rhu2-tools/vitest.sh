#!/bin/zsh
# usage: vitest.sh <renderer|kernel|framework> <logname> [vitest args...]
. "$(dirname "$0")/env.sh"
which="$1"; shift
mkdir -p "$G/rhu2-vitest-cache"
case "$which" in
  renderer) P="$RT/🎯️targets/⚛️react/📦️packages/🟦️typescript"; CFG="../../🧪️tests/🎚️config/🟦️.ts";;
  kernel) P="$KD/📦️packages/🟦️typescript"; CFG="../../🧪️tests/🎚️config/🟦️.ts";;
  framework) P="$R/🧰️framework/📦️packages/🟦️typescript"; CFG="${FW_CFG:-../../🧪️tests/🎚️config/🟦️.ts}";;
esac
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$P\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/rhu2-vitest-cache\",\"coverageDirectory\":\"$G/rhu2-vitest-cov\",\"budgetMs\":1500000}"
cd "$P"
perl -e 'alarm 1700; exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "${VITEST_CONFIG:-$CFG}" --testTimeout 120000 --reporter=default "$@"
