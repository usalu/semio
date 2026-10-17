#!/bin/zsh
# usage: vt.sh <phase> <label>   (reads config path for label from suites.tsv)
. "$(dirname "$0")/env.sh"
phase="$1"; label="$2"
cfg_rel=$(awk -F'\t' -v l="$label" '$1==l{print $2}' "$T/suites.tsv")
[ -n "$CFG_SIBLING" ] && cfg_rel="$(dirname "$cfg_rel")/$CFG_SIBLING"
cfg="$R/$cfg_rel"
base="$(dirname "$(dirname "$(dirname "$cfg")")")"
[ -d "$base/📦️packages/🟦️typescript" ] && cwd="$base/📦️packages/🟦️typescript" || cwd="$base"
export SEMIO_TEST_LEVEL=long
mkdir -p "$G/slf-vitest-cache"
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$cwd\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/slf-vitest-cache\",\"coverageDirectory\":\"$G/slf-vitest-cov\",\"budgetMs\":1500000}"
cd "$cwd" || exit 9
start=$(date +%s)
perl -e 'alarm ($ENV{ALARM} || 900); exec @ARGV' -- node "$R/node_modules/vitest/vitest.mjs" run --config "$cfg" --testTimeout 60000 --reporter=json --outputFile="$G/slf-vt-$phase-$label.json" > "$G/slf-vt-$phase-$label.log" 2>&1
echo "$label exit=$? secs=$(( $(date +%s) - start ))" >> "$G/slf-vt-$phase-status.log"
