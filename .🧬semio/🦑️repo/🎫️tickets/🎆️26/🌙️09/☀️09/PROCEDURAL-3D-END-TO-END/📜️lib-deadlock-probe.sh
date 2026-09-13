#!/usr/bin/env bash
# 🪤 Reproduces the generation3d `--lib` serial-lock deadlock and samples the blocked threads.
# Usage: 📜️lib-deadlock-probe.sh <test-filter> [seconds-before-sample] [extra libtest args...]
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../../../.." && pwd)"
OUT="$(dirname "${BASH_SOURCE[0]}")/🗑️generated/lib-suite"
mkdir -p "$OUT"
FILTER="${1:?test filter}"
WAIT="${2:-60}"
shift 2 || true
cd "$ROOT"
export RUST_MIN_STACK=33554432
BIN_JSON="$OUT/probe-build.json"
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-run --message-format=json >"$BIN_JSON" 2>"$OUT/probe-build.txt" || { echo "BUILD FAILED"; tail -40 "$OUT/probe-build.txt"; exit 1; }
BIN="$(python3 - "$BIN_JSON" <<'PY'
import json,sys
path=None
for line in open(sys.argv[1]):
    try: m=json.loads(line)
    except Exception: continue
    if m.get("reason")=="compiler-artifact" and m.get("target",{}).get("kind")==["lib"] and m.get("executable"):
        path=m["executable"]
print(path or "")
PY
)"
[ -n "$BIN" ] || { echo "no test binary"; exit 1; }
echo "binary: $BIN"
"$BIN" "$FILTER" --nocapture "$@" >"$OUT/probe-run.txt" 2>&1 &
PID=$!
echo "pid: $PID waiting ${WAIT}s"
sleep "$WAIT"
if kill -0 "$PID" 2>/dev/null; then
  echo "STILL RUNNING after ${WAIT}s — sampling"
  sample "$PID" 3 -file "$OUT/probe-sample.txt" >/dev/null 2>&1
  kill -9 "$PID" 2>/dev/null
  echo "HUNG"
else
  wait "$PID"; echo "exited $?"
fi
tail -30 "$OUT/probe-run.txt"
