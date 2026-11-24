#!/usr/bin/env bash
# ⛓️ Take over from chain 6 after its current DIN V 18599 probe: accept a route that fails only on temporary [DEBUG] prints, then rebuild every Standards guest, sweep all 15 routes and run the official plugin and JS tests.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
until [ -f "$G/routes-energy3.log" ] && grep -q "routes exit" "$G/chain-6.out" && [ "$(grep -c 'routes exit' "$G/chain-6.out")" -ge 3 ]; do sleep 60; done
log="$G/norm-base-preview-route-din18599.log"
components="norm-eurocode-structural norm-eurocode norm-eurocode-materials norm"
if grep -q "din18599 browser outcome 0" "$G/routes-energy3.log"; then
  echo "[DEBUG] din18599 clean, chain 6 continues $(date +%H:%M)"
  exit 0
elif python3 "$T/route-debug-only.py" "$log"; then
  pkill -f chain-6.sh
  echo "[DEBUG] din18599 works, only [DEBUG] prints $(date +%H:%M)"
  components="$components norm-energy"
else
  echo "[DEBUG] din18599 genuinely failing $(date +%H:%M)"
  exit 1
fi
"$T/rebuild-and-probe.sh" sweep7 "$components" \
  din4108 din16798 iso16757 vdi3805 din18599 en1990 en1991 en1997 en1998 en1992 en1993 en1994 en1995 en1996 en1999
cd "$ROOT"
repo cache gc --lang=en > "$G/cache-gc-test-7.log" 2>&1
SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=7200000 NATIVE_EXEC_TARGET=@semio-tech/norm-plugin:test repo "$T/native-exec.ts" "🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" ./📜️script.ts test quick > "$G/normtest-rs-test-h7.log" 2>&1
echo "[DEBUG] norm-plugin:test exit $?"
repo run @semio-tech/norm-js:test > "$G/norm-js-test-h7.log" 2>&1
echo "[DEBUG] norm-js:test exit $?"
