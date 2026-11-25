#!/usr/bin/env bash
# 🧪️ One vitest process of the React target or the site inside a grant, one worker, JSON report; prints the counts. Usage: repo capacity run --memory=<MB> -- vitest-one.sh <react|site> <name> [vitest args...]  (env: SEMIO_TEST_LEVEL, SEMIO_TEST_BUDGET_MS, TEST_TIMEOUT_S)
set -uo pipefail
source "$(dirname "$0")/env.sh"
W="$1" NAME="$2"; shift 2
mkdir -p "$OUT/$NAME"
cd "$ROOT"
case "$W" in
  react) PKG="$S/🎯️targets/⚛️react/📦️packages/🟦️typescript"; CFG=(--config "$S/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts") ;;
  site) PKG="$S/🌐️site/📦️packages/🟦️typescript"; CFG=(--config "$S/🌐️site/🧪️tests/🎚️config/🟦️.ts" --configLoader native) ;;
esac
started=$(date +%s)
SEMIO_BUILD_MODE="${SEMIO_BUILD_MODE:-prototype}" timeout "${TEST_TIMEOUT_S:-2400}" /usr/bin/time -l -o "$OUT/$NAME/time.txt" "$BUN" --cwd "$PKG" "$ROOT/node_modules/vitest/vitest.mjs" run "${CFG[@]}" --maxWorkers=1 --reporter=default --reporter=json --outputFile.json="$OUT/$NAME/result.json" "$@" > "$OUT/$NAME/run.log" 2>&1
code=$?
node -e 'const fs=require("fs");try{const r=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));console.log("exit",process.argv[2],"passed",r.numPassedTests,"failed",r.numFailedTests,"skipped",r.numPendingTests);for(const f of r.testResults)for(const t of f.assertionResults)if(t.status==="failed")console.log("FAILED",f.name.split("/").slice(-2,-1)[0],"|",t.fullName,"|",(t.failureMessages||[]).join(" ").split("\n")[0].slice(0,400))}catch(e){console.log("exit",process.argv[2],"no json")}' "$OUT/$NAME/result.json" "$code" | tee "$OUT/$NAME/summary.txt"
echo "seconds $(( $(date +%s) - started )); peak $(grep -i 'maximum resident' "$OUT/$NAME/time.txt" | awk '{print int($1/1048576)" MB"}')" | tee -a "$OUT/$NAME/summary.txt"
