#!/usr/bin/env bash
# ⛓️ Rebuild norm-energy and probe DIN V 18599 until the Play refusal is gone, then rebuild every Standards guest, sweep all 15 routes and run the official plugin and JS tests.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
n=1
until "$T/rebuild-and-probe.sh" "energy$n" norm-energy din18599 && grep -q "din18599 browser outcome 0" "$G/routes-energy$n.log"; do
  echo "[DEBUG] din18599 still refused, round $n $(date +%H:%M)"
  n=$((n + 1))
  sleep 600
done
echo "[DEBUG] din18599 green round $n $(date +%H:%M)"
"$T/rebuild-and-probe.sh" sweep6 "norm-eurocode-structural norm-eurocode norm-eurocode-materials norm" \
  din4108 din16798 iso16757 vdi3805 din18599 en1990 en1991 en1997 en1998 en1992 en1993 en1994 en1995 en1996 en1999
cd "$ROOT"
repo cache gc --lang=en > "$G/cache-gc-test-6.log" 2>&1
SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=7200000 NATIVE_EXEC_TARGET=@semio-tech/norm-plugin:test repo "$T/native-exec.ts" "🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" ./📜️script.ts test quick > "$G/normtest-rs-test-h6.log" 2>&1
echo "[DEBUG] norm-plugin:test exit $?"
repo run @semio-tech/norm-js:test > "$G/norm-js-test-h6.log" 2>&1
echo "[DEBUG] norm-js:test exit $?"
