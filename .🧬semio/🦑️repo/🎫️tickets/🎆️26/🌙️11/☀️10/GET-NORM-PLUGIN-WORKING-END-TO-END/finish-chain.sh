#!/usr/bin/env bash
# ⛓️ Run the remaining serialized Rust steps: guest rebuilds with the full route sweep, then the official plugin test.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
"$T/rebuild-and-probe.sh" sweep "${COMPONENTS:-norm-eurocode norm-eurocode-materials norm-eurocode-structural}" \
  din4108 din16798 iso16757 vdi3805 din18599 en1990 en1991 en1997 en1998 en1992 en1993 en1994 en1995 en1996 en1999
cd "$ROOT"
repo cache gc --lang=en > "$G/cache-gc-test.log" 2>&1
NATIVE_EXEC_TARGET=@semio-tech/norm-plugin:test repo "$T/native-exec.ts" "🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" ./📜️script.ts test > "$G/normtest-rs-test-h2.log" 2>&1
echo "[DEBUG] norm-plugin:test exit $?"
