#!/usr/bin/env bash
# 🏁️ Closing verification of the staffage goal on the frozen catalog, two lanes (at most two capacity requests at any time), every step one `repo capacity run`, resumable.
# Usage: run.sh [--force] [step …]        all steps, or only the named ones (a step with its `<step>.exit` file at 0 is skipped unless --force)
#        run.sh dry                       small validation of the plumbing, no heavy step
# Lane M: `matrix` (MM's full.sh, one request at a time, requests widened to PLAN_SCALE x jobs up to PLAN_CAP_MB), then `proofs` (MM's proofs.sh, frozen catalog, after the matrix).
# Lane O: `light` (every light engine suite one by one + check CLI over the nine categories), `build` (release build), `browser` (smoke, label-fit, idle-overview, idle probe), `sil` (catalog-silhouette in two name shards), `tv_tc` (three-views beside the type checks of engine, React target, site), `coverage` (multiview-coverage), `reactsite` (React and site suites), `split` (model-split), `loading` (catalog-loading).
# Logs, JSON, exit files: <U>/🗑️generated/closing/ ; one line per step in summary.txt ; proof sheets in <K>/🖨️sheets/ (kept).
# @see ../🧰️matrix/MM/full.sh, ../🧰️matrix/MM/proofs.sh, ../🧰️mac/*.sh, ../older/suite.sh, ../check-categories.sh
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
U="$(cd "$HERE/.." && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
S="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage"
K="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun 2>/dev/null | head -1)"
SELF="$HERE/run.sh"
G="$U/🗑️generated/closing"
MAC="$U/🧰️mac"
export STAFFAGE_LANGUAGE="${STAFFAGE_LANGUAGE:-en}"
HEAVY="🧊️catalog-silhouette 🧊️three-views 🔭️multiview-coverage"
CATEGORIES="humans animals vegetation furniture appliances objects vehicles machinery components"
SHARD_A='its volume [a-l][a-z0-9-]* fills'
SHARD_B='^(?!.*its volume [a-l][a-z0-9-]* fills)'

count() { node -e 'try{const r=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));console.log("files",r.numTotalTestSuites,"passed",r.numPassedTests,"failed",r.numFailedTests,"skipped",r.numPendingTests)}catch(e){console.log("no json")}' "$1"; }

vt() {
  local label="$1" limit="$2"; shift 2
  local json="$G/$label.json" started; started=$(date +%s)
  (cd "$ROOT" && timeout "$limit" "$BUN" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --reporter=default --reporter=json --outputFile.json="$json" --config "$S/🧪️tests/🎚️config/🟦️.ts" "$@") > "$G/$label.log" 2>&1
  local code=$?
  echo "$label exit $code $(count "$json") seconds $(( $(date +%s) - started ))" | tee -a "$G/summary.txt"
}

step_light() {
  local d name
  for d in "$S"/🧪️tests/*/; do
    name="$(basename "$d")"
    case "$name" in node_modules|🧰️support|🎚️config) continue ;; esac
    [ -f "$d/🟦️.ts" ] || continue
    case " $HEAVY 🧊️model-catalog " in *" $name "*) continue ;; esac
    vt "suite-$name" 1800 "/$name/"
  done
}

check_one() {
  local c="$1" started; started=$(date +%s)
  (cd "$S" && timeout 7200 "$BUN" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --category "$c") > "$G/check/$c.txt" 2>&1
  echo "check-$c exit $? seconds $(( $(date +%s) - started )) :: $(tail -1 "$G/check/$c.txt" | cut -c1-160)" | tee -a "$G/summary.txt"
}

step_check() {
  mkdir -p "$G/check"
  local c
  if [ -n "${CHECK_CATEGORIES:-}" ]; then for c in $CHECK_CATEGORIES; do check_one "$c"; done; return; fi
  (check_one vegetation) &
  (for c in humans furniture components; do check_one "$c"; done) &
  (for c in animals appliances objects vehicles machinery; do check_one "$c"; done) &
  wait
}

step_sil() {
  vt sil-a 14400 /🧊️catalog-silhouette/ -t "$SHARD_A" --testTimeout=900000 &
  vt sil-b 14400 /🧊️catalog-silhouette/ -t "$SHARD_B" --testTimeout=900000 &
  wait
}

step_tv_tc() {
  vt three-views 5400 /🧊️three-views/ &
  local project p
  for project in "$S/📦️packages/🟦️typescript" "$S/🎯️targets/⚛️react/📦️packages/🟦️typescript" "$S/🌐️site/📦️packages/🟦️typescript"; do
    p="$(basename "$(dirname "$(dirname "$project")")")"
    local started; started=$(date +%s)
    (cd "$ROOT" && node --max-old-space-size=2000 node_modules/typescript/lib/tsc.js --noEmit --incremental false --pretty false -p "$project/tsconfig.json") > "$G/tsc-$p.txt" 2>&1
    echo "tsc-$p exit $? errors $(grep -c 'error TS' "$G/tsc-$p.txt") seconds $(( $(date +%s) - started ))" | tee -a "$G/summary.txt"
  done
  wait
}

step_modelcat() { vt model-catalog 10800 /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000; }

step_reds() { local f; for f in $REDS; do vt "reds-$(echo "$f" | tr -d "/")" 3600 "$f" --testTimeout=900000; done; }

step_coverage() { vt multiview-coverage 5400 /🔭️multiview-coverage/ --testTimeout=900000; }

step_reactsite() { "$MAC/chain-suites.sh" closing-suites 2>&1 | tee -a "$G/summary.txt"; }

step_browser() { "$MAC/browser-chain.sh" closing-browser 2>&1 | tee -a "$G/summary.txt"; }

step_dry() {
  vt dry-asset-power 600 /⚡️asset-power/
  (cd "$S" && timeout 600 "$BUN" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset boxwood-ball) > "$G/dry-check.txt" 2>&1; echo "dry-check exit $?" | tee -a "$G/summary.txt"
  node -e '
const fs=require("fs");const names=fs.readFileSync(process.argv[1],"utf8").split("\n").filter(l=>l.includes("every asset of the catalog")).map(l=>l.replace(/^.*?🟦️\.ts > /u,"").replace(/ \d+ms$/u,""));
const a=new RegExp(process.argv[2],"u"),b=new RegExp(process.argv[3],"u");
const ia=names.filter(n=>a.test(n)).length,ib=names.filter(n=>b.test(n)).length;
console.log("shards",ia,ib,ia+ib,"of",names.length,ia+ib===names.length&&names.every(n=>a.test(n)!==b.test(n))?"disjoint-complete":"BAD")' "$U/🗑️generated/older/catalog-silhouette-2.txt" "$SHARD_A" "$SHARD_B" | tee -a "$G/summary.txt"
}

want() { [ ${#ONLY[@]} -eq 0 ] && return 0; local x; for x in "${ONLY[@]}"; do [ "$x" = "$1" ] && return 0; done; return 1; }

grant() {
  local name="$1" memory="$2" limit="$3"; shift 3
  want "$name" || return 0
  [ "$FORCE" = 0 ] && [ "$(cat "$G/$name.exit" 2>/dev/null)" = 0 ] && { echo "$name already done" | tee -a "$G/summary.txt"; return 0; }
  local started; started=$(date +%s)
  "$@" > "$G/$name.log" 2>&1
  local code=$?
  echo "$code" > "$G/$name.exit"
  echo "STEP $name exit $code seconds $(( $(date +%s) - started ))" | tee -a "$G/summary.txt"
}

cap() { local memory="$1" limit="$2"; shift 2; repo capacity run --memory="$memory" -- timeout "$limit" "$@"; }

lane_m() {
  want matrix && grant matrix 0 0 env STAFFAGE_LANGUAGE="$STAFFAGE_LANGUAGE" PLAN_SCALE="${PLAN_SCALE:-2}" PLAN_CAP_MB="${PLAN_CAP_MB:-4500}" sh "$U/🧰️matrix/MM/full.sh" "$G/matrix" --lanes 1
  want proofs && grant proofs 0 0 sh "$U/🧰️matrix/MM/proofs.sh" "$K/🖨️sheets"
}

lane_o() {
  grant light 2500 5400 cap 2500 5400 bash "$SELF" step light
  grant build 0 0 env BUILD_TIMEOUT_S=3600 "$MAC/build.sh" release
  grant browser 3000 3600 cap 3000 3600 "$MAC/browser-chain.sh" closing-browser
  grant sil 4600 14700 cap 4600 14700 bash "$SELF" step sil
  grant tv_tc 4600 5700 cap 4600 5700 bash "$SELF" step tv_tc
  grant check 3500 14700 cap 3500 14700 bash "$SELF" step check
  grant modelcat 2500 10900 cap 2500 10900 bash "$SELF" step modelcat
  grant coverage 3600 5700 cap 3600 5700 bash "$SELF" step coverage
  grant reactsite 2500 3600 cap 2500 3600 "$MAC/chain-suites.sh" closing-suites
  grant split 0 0 env SEMIO_TEST_LEVEL=long TEST_MEMORY_MB=3500 TEST_TIMEOUT_S=3600 "$MAC/test.sh" site /🧊️model-split/ --testTimeout=3000000
  grant loading 0 0 env SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=3300000 TEST_TIMEOUT_S=3400 "$MAC/test.sh" react /📚️catalog-loading/ --testTimeout=3300000
}

main() {
  if [ "${1:-}" = step ]; then mkdir -p "$G"; "step_$2"; exit $?; fi
  FORCE=0; ONLY=()
  while [ $# -gt 0 ]; do case "$1" in --force) FORCE=1 ;; dry) ONLY=(dry) ;; *) ONLY+=("$1") ;; esac; shift; done
  mkdir -p "$G" "$K/🖨️sheets"
  cd "$ROOT" || exit 1
  if [ "${ONLY[*]:-}" = dry ]; then FORCE=1; cap 2500 1500 bash "$SELF" step dry; exit $?; fi
  echo "closing start $(date '+%F %T')" | tee -a "$G/summary.txt"
  lane_m & local m=$!
  lane_o & local o=$!
  wait "$m" "$o"
  echo "closing end $(date '+%F %T')" | tee -a "$G/summary.txt"
  touch "$G/DONE"
}
main "$@"
