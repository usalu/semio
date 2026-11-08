#!/bin/bash
# usage: serve.sh [start|stop|status]  — coordinator only. Detached energy-only React dev serve on 127.0.0.1:6106 (declared playground port), after activate.sh is green.
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$HERE"; while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
[ -f "$ROOT/AGENTS.md" ] || { echo "repo root not found" >&2; exit 2; }
G="$HERE/../../🗑️generated/1105b"; mkdir -p "$G"
LOG="$G/canary-serve-6106.log"; PID="$G/canary-serve-6106.pid"; PORT=6106; URL="http://127.0.0.1:$PORT/?plugin=energy"
D="$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
REPO="${REPO_BIN:-$(command -v repo || echo "$HOME/.local/bin/repo")}"
listener() { lsof -nP -iTCP:$PORT -sTCP:LISTEN 2>/dev/null | tail -n +2; }
case "${1:-start}" in
stop) [ -f "$PID" ] && kill "$(cat "$PID")" 2>/dev/null; sleep 2; listener; echo "stopped (check the listener above is empty)"; exit 0;;
status) [ -f "$PID" ] && kill -0 "$(cat "$PID")" 2>/dev/null && echo "pid $(cat "$PID") alive" || echo "not running"; listener; curl -s -o /dev/null -w 'http %{http_code}\n' --max-time 45 "$URL"; exit 0;;
start) ;;
*) echo "usage: serve.sh [start|stop|status]" >&2; exit 2;;
esac
[ -z "$(listener)" ] || { echo "port $PORT already has a listener:" >&2; listener >&2; exit 3; }
R="$ROOT/.cache/b/p/m/browser/prototype/runtime/react/energy/activation/🔣️receipt.json"
[ -f "$R" ] || { echo "lane receipt missing: $R (run activate.sh first; serve would otherwise activate inline)" >&2; exit 4; }
cd "$D" || exit 2
SEMIO_PLUGIN=energy SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=$PORT S_LOCAL_ONLY=1 SEMIO_BUILD_MODE=prototype NODE_OPTIONS= SEMIO_BUILD_TERMINAL=false \
  nohup "$REPO" ./📜️script.ts serve energy react prototype > "$LOG" 2>&1 < /dev/null &
echo $! > "$PID"; disown
echo "serve pid $(cat "$PID"), log $LOG"
for i in $(seq 1 90); do
  kill -0 "$(cat "$PID")" 2>/dev/null || { echo "serve exited early; tail of log:" >&2; tail -20 "$LOG" >&2; exit 5; }
  if [ -n "$(listener)" ]; then
    code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 60 "$URL"); echo "listening, GET -> $code"; [ "$code" = 200 ] && exit 0; exit 6
  fi
  sleep 5
done
echo "no listener after 450 s; see $LOG (vite dep-optimizer race: stop, empty the vite cacheDir from viteCacheDirectory in 🏗️builder/🌐️vite/🟦️.ts, start once more)" >&2; exit 7
