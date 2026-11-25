#!/usr/bin/env bash
# 🧪️ One grant, many suites: runs vitest of the engine (tag titles), the React target (every case file but catalog-loading) and the site (every case but label-fit) one after the other in ONE process each, one worker, so the peak of the chain is the peak of one vitest process. Inside a grant of `repo capacity run`; logs and JSON in the output folder. Usage: repo capacity run --memory=2500 -- chain-suites.sh <name>
set -uo pipefail
source "$(dirname "$0")/env.sh"
NAME="$1"
mkdir -p "$OUT/$NAME"
cd "$ROOT"
run() {
  local step="$1" package="$2" config="$3"; shift 3
  local started; started=$(date +%s)
  SEMIO_BUILD_MODE="${SEMIO_BUILD_MODE:-prototype}" timeout "${STEP_TIMEOUT_S:-2400}" "$BUN" --cwd "$package" "$ROOT/node_modules/vitest/vitest.mjs" run --config "$config" --maxWorkers=1 --reporter=default --reporter=json --outputFile.json="$OUT/$NAME/$step.json" "$@" > "$OUT/$NAME/$step.log" 2>&1
  local code=$?
  node -e 'try{const r=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));console.log(process.argv[2],"exit",process.argv[3],"files",r.numTotalTestSuites,"passed",r.numPassedTests,"failed",r.numFailedTests,"skipped",r.numPendingTests)}catch(e){console.log(process.argv[2],"exit",process.argv[3],"no json")}' "$OUT/$NAME/$step.json" "$step" "$code" | tee -a "$OUT/$NAME/summary.txt"
  echo "$step seconds $(( $(date +%s) - started ))" >> "$OUT/$NAME/summary.txt"
}
ENGINE="$S/📦️packages/🟦️typescript"
REACT="$S/🎯️targets/⚛️react"
SITE="$S/🌐️site"
run engine-tag-titles "$ENGINE" "$S/🧪️tests/🎚️config/🟦️.ts" /🏷️tag-titles/
CASES=()
for folder in "$REACT"/🧪️tests/*/; do
  case="$(basename "$folder")"
  case "$case" in 🎚️config|🧰️support|📚️catalog-loading) continue ;; esac
  [ -f "$folder/🟦️.tsx" ] && CASES+=("/$case/")
done
SEMIO_TEST_LEVEL=quick run react "$REACT/📦️packages/🟦️typescript" "$REACT/🧪️tests/🎚️config/🟦️.ts" "${CASES[@]}"
SEMIO_TEST_LEVEL=long run site "$SITE/📦️packages/🟦️typescript" "$SITE/🧪️tests/🎚️config/🟦️.ts" --configLoader native /📄️host-document/ /📦️catalog-release/ /🛟️stale-chunk/ /🧊️model-split/ /🧵️flow-worker/
