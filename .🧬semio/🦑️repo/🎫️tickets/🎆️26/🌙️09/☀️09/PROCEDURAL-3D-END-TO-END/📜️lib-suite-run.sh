#!/usr/bin/env bash
# 🏁 Runs the featured generation3d `--lib` suite to completion, watching for the serial-lock stall:
# if the test binary emits no new output for STALL seconds it is sampled (thread stacks) and killed.
# Usage: 📜️lib-suite-run.sh <label> <stall-seconds> [libtest args...]
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../../../.." && pwd)"
OUT="$(dirname "${BASH_SOURCE[0]}")/🗑️generated/lib-suite"
mkdir -p "$OUT"
LABEL="${1:?label}"
STALL="${2:-120}"
shift 2 || true
cd "$ROOT"
export RUST_MIN_STACK=33554432
BIN_JSON="$OUT/$LABEL-build.json"
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-run --message-format=json >"$BIN_JSON" 2>"$OUT/$LABEL-build.txt" || {
  echo "BUILD FAILED"
  python3 - "$BIN_JSON" <<'PY'
import json,sys
for line in open(sys.argv[1]):
    try: m=json.loads(line)
    except Exception: continue
    if m.get("reason")=="compiler-message" and m["message"].get("level")=="error":
        print(m["message"]["rendered"][:1200])
PY
  exit 1
}
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
LOG="$OUT/$LABEL-run.txt"
: >"$LOG"
START=$(date +%s)
"$BIN" "$@" >"$LOG" 2>&1 &
PID=$!
LAST_SIZE=-1
LAST_CHANGE=$(date +%s)
while kill -0 "$PID" 2>/dev/null; do
  sleep 10
  SIZE=$(wc -c <"$LOG")
  NOW=$(date +%s)
  if [ "$SIZE" != "$LAST_SIZE" ]; then LAST_SIZE=$SIZE; LAST_CHANGE=$NOW; fi
  if [ $((NOW - LAST_CHANGE)) -ge "$STALL" ]; then
    echo "STALLED: no output for $((NOW - LAST_CHANGE))s after $((NOW - START))s — sampling"
    sample "$PID" 3 -file "$OUT/$LABEL-sample.txt" >/dev/null 2>&1
    kill -9 "$PID" 2>/dev/null
    echo "STALLED"
    break
  fi
done
wait "$PID" 2>/dev/null
echo "wall: $(( $(date +%s) - START ))s"
grep -E "^test result:" "$LOG" || tail -5 "$LOG"
