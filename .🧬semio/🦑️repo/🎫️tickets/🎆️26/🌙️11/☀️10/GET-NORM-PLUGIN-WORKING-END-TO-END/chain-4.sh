#!/usr/bin/env bash
# ⛓️ Wait until the store replay regression is fixed, then rebuild every affected Standards guest, sweep all 15 routes and run the official plugin test.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
R="$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/📦️packages/🦀️rust"
while pgrep -f chain-3.sh > /dev/null; do sleep 60; done
until (cd "$R" && repo cargo --store=shared --configuration=prototype -- test -p semio-os-store-initialization-replay --lib --offline > "$G/replay-gate.log" 2>&1) && grep -q "^test result: ok" "$G/replay-gate.log"; do
  echo "[DEBUG] replay gate red $(date +%H:%M)"
  sleep 900
done
echo "[DEBUG] replay gate green $(date +%H:%M)"
"$T/rebuild-and-probe.sh" sweep4 "norm-eurocode-structural norm-energy norm-eurocode norm-eurocode-materials" \
  din4108 din16798 iso16757 vdi3805 din18599 en1990 en1991 en1997 en1998 en1992 en1993 en1994 en1995 en1996 en1999
cd "$ROOT"
repo cache gc --lang=en > "$G/cache-gc-test-4.log" 2>&1
SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=7200000 NATIVE_EXEC_TARGET=@semio-tech/norm-plugin:test repo "$T/native-exec.ts" "🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" ./📜️script.ts test quick > "$G/normtest-rs-test-h4.log" 2>&1
echo "[DEBUG] norm-plugin:test exit $?"
