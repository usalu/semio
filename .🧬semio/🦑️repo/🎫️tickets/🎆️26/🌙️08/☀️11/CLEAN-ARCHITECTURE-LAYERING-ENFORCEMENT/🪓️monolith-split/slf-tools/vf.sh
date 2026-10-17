#!/bin/zsh
# usage: vf.sh <label> <filter>  runs one route restricted to files matching <filter> with the verbose reporter
. "$(dirname "$0")/env.sh"
label="$1"; filter="$2"
cfg_rel=$(awk -F'\t' -v l="$label" '$1==l{print $2}' "$T/suites.tsv")
cfg="$R/$cfg_rel"
base="$(dirname "$(dirname "$(dirname "$cfg")")")"
[ -d "$base/📦️packages/🟦️typescript" ] && cwd="$base/📦️packages/🟦️typescript" || cwd="$base"
export SEMIO_TEST_LEVEL=long
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$cwd\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/slf-vitest-cache\",\"coverageDirectory\":\"$G/slf-vitest-cov\",\"budgetMs\":1500000}"
cd "$cwd" || exit 9
perl -e 'alarm ($ENV{ALARM} || 600); exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$cfg" --testTimeout 60000 --reporter=default "$filter" 2>&1
