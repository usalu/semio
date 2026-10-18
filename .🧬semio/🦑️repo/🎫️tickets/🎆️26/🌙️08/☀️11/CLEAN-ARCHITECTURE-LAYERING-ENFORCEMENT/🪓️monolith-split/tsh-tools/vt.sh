#!/bin/zsh
# usage: vt.sh <phase> <suite> <cwd-rel-to-repo> <config-rel-to-repo> [filter...]  -> $G/tsh-vt-<suite>-<phase>.{json,log}
. "$(dirname "$0")/env.sh"
phase="$1"; suite="$2"; cwdrel="$3"; cfgrel="$4"; shift 4
cfg="$R/$cfgrel"; cwd="$R/$cwdrel"
export SEMIO_TEST_LEVEL=long
export SEMIO_TEST_ARTIFACT_DIR="/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/tsh-art"
mkdir -p "$G/tsh-vitest-cache"
export SEMIO_VITEST_POLICY="{\"version\":1,\"cwd\":\"$cwd\",\"toolPath\":\"$R/node_modules/vitest/vitest.mjs\",\"runtime\":\"$(which bun)\",\"coverageRuntime\":\"$(which bun)\",\"cacheRoot\":\"$G/tsh-vitest-cache\",\"coverageDirectory\":\"$G/tsh-vitest-cov\",\"budgetMs\":1500000}"
cd "$cwd" || exit 9
start=$(date +%s)
if [ -n "$VERBOSE" ]; then rep=(--reporter=default --no-color); else rep=(--reporter=json --outputFile="$G/tsh-vt-$suite-$phase.json"); fi
ALARM=${ALARM:-900} perl -e 'alarm ($ENV{ALARM}); exec @ARGV' -- nice -n 10 node "$R/node_modules/vitest/vitest.mjs" run --config "$cfg" --testTimeout 60000 $rep "$@" > "$G/tsh-vt-$suite-$phase.log" 2>&1
echo "$suite $phase exit=$? secs=$(( $(date +%s) - start ))" >> "$G/tsh-status.log"
