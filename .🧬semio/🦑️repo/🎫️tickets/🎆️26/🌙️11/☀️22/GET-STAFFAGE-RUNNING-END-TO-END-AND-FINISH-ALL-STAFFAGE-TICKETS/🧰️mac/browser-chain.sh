#!/usr/bin/env bash
# 🌐️ One grant, the browser steps on the release build one after the other: smoke in four contexts, the label case, the idle overview case, the idle memory probe. Usage: repo capacity run --memory=3000 -- browser-chain.sh <name>
set -uo pipefail
source "$(dirname "$0")/env.sh"
NAME="$1"
mkdir -p "$OUT/$NAME"
DIST="$S/🌐️site/📦️packages/🟦️typescript/dist/release"
cd "$ROOT"
echo "smoke" | tee -a "$OUT/$NAME/summary.txt"
timeout 2400 node "$HERE/smoke.mjs" "$DIST" "$OUT/$NAME/smoke" > "$OUT/$NAME/smoke.log" 2>&1
echo "smoke exit $? ($(grep -c 'PASS' "$OUT/$NAME/smoke.log") pass, $(grep -c 'FAIL' "$OUT/$NAME/smoke.log") fail)" | tee -a "$OUT/$NAME/summary.txt"
export TEACHING_ARCHITECTURE_STAFFAGE_BROWSER_CHANNEL=chrome SEMIO_TEST_LEVEL=long SEMIO_BUILD_MODE=release
TEACHING_ARCHITECTURE_STAFFAGE_LABEL_FIT_OUT="$OUT/$NAME/label-fit" TEST_TIMEOUT_S=2400 "$HERE/vitest-one.sh" site "$NAME-label-fit" /🏷️label-fit/ 2>&1 | tee -a "$OUT/$NAME/summary.txt"
TEACHING_ARCHITECTURE_STAFFAGE_IDLE_OUT="$OUT/$NAME/idle-overview" TEACHING_ARCHITECTURE_STAFFAGE_IDLE_SECONDS=60 TEST_TIMEOUT_S=1200 "$HERE/vitest-one.sh" site "$NAME-idle-overview" /🪟️idle-overview/ 2>&1 | tee -a "$OUT/$NAME/summary.txt"
timeout 900 node "$HERE/idle-probe.mjs" "$DIST" "$OUT/$NAME/idle-probe" "" 150 > "$OUT/$NAME/idle-probe.log" 2>&1
node "$HERE/samples.mjs" "$OUT/$NAME/idle-probe/s7-samples.jsonl" | tail -8 | tee -a "$OUT/$NAME/summary.txt"
echo done | tee -a "$OUT/$NAME/summary.txt"
