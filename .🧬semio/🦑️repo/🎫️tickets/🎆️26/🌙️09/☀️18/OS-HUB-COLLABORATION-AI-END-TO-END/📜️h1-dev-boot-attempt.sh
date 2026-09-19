#!/bin/sh
# 🚀️ H1: runs the real `os-hub` dev route against a brand new OS_HUB_DATA with no
# SEMIO_BUILD_BUDGET_MS in the environment, and stops as soon as the trusted stdio+GIS
# materialization has proved which side of the build-budget fix it lands on:
#   - "trusted-stdio-gis-bootstrap capture-codecs" = the zero budget is read as unlimited (fixed),
#   - "trusted codec capture cancelled"            = the zero budget is still read as expired.
# Usage: 📜️h1-dev-boot-attempt.sh <capture-file> [wait-seconds]
set -eu
capture=$1
waits=${2:-600}
repo=$(cd "$(dirname "$0")/../../../../../../.." && pwd)
data=$(mktemp -d /private/tmp/h1-dev-data-XXXXXX)
unset SEMIO_BUILD_BUDGET_MS || true
echo "repo=$repo" > "$capture"
echo "fresh OS_HUB_DATA=$data SEMIO_BUILD_BUDGET_MS=${SEMIO_BUILD_BUDGET_MS:-<unset>}" >> "$capture"
cd "$repo/🌎️hub/📦️packages/🦀️rust"
OS_HUB_DATA="$data" OS_HUB_PORT="${OS_HUB_PORT:-8815}" bun ./📜️script.ts dev >> "$capture" 2>&1 &
pid=$!
elapsed=0
while [ "$elapsed" -lt "$waits" ]; do
  if grep -qE "capture-codecs|trusted codec capture cancelled|error|Error" "$capture"; then break; fi
  if ! kill -0 "$pid" 2>/dev/null; then break; fi
  sleep 3
  elapsed=$((elapsed + 3))
done
echo "--- probe stopping dev pid=$pid after ${elapsed}s ---" >> "$capture"
kill -TERM "$pid" 2>/dev/null || true
sleep 2
kill -9 "$pid" 2>/dev/null || true
pkill -TERM -P "$pid" 2>/dev/null || true
rm -rf "$data"
grep -nE "capture-codecs|trusted codec capture cancelled|Missing Nx-staged|error|Error" "$capture" | head -20 || true
