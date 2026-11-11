#!/bin/bash
# usage: serve.sh [start|stop|status]  — detached demonstrator dev server on 127.0.0.1:6029 (coordinator runs it after activate-prototype is green)
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$HERE"; while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
[ -f "$ROOT/AGENTS.md" ] || { echo "repo root not found" >&2; exit 2; }
G="$HERE/../🗑️generated/1105b"; mkdir -p "$G"
LOG="$G/serve-6029.log"; PID="$G/serve-6029.pid"; PORT=6029; URL="http://127.0.0.1:$PORT/"
D="$ROOT/♻️mit-bestand/🧺️demonstrator"
REPO="${REPO_BIN:-$(command -v repo || echo "$HOME/.local/bin/repo")}"
listener() { lsof -nP -iTCP:$PORT -sTCP:LISTEN 2>/dev/null | tail -n +2; }
case "${1:-start}" in
stop) [ -f "$PID" ] && kill "$(cat "$PID")" 2>/dev/null; sleep 2; listener; echo "stopped (check the listener above is empty)"; exit 0;;
status) [ -f "$PID" ] && kill -0 "$(cat "$PID")" 2>/dev/null && echo "pid $(cat "$PID") alive" || echo "not running"; listener; curl -s -o /dev/null -w 'http %{http_code}\n' --max-time 45 "$URL"; exit 0;;
start) ;;
*) echo "usage: serve.sh [start|stop|status]" >&2; exit 2;;
esac
[ -z "$(listener)" ] || { echo "port $PORT already has a listener:" >&2; listener >&2; exit 3; }
R="$ROOT/.cache/b/p/m/browser/prototype/runtime/react"
for lane in generator energy fem3d; do [ -f "$R/$lane/activation/🔣️receipt.json" ] || { echo "lane receipt missing: $R/$lane/activation/🔣️receipt.json (run activate-prototype first)" >&2; exit 4; }; done
[ -d "$D/dist/♻️activation/prototype" ] || { echo "union receipt directory missing: $D/dist/♻️activation/prototype" >&2; exit 4; }
cd "$D" || exit 2
MIT_BESTAND_DEMONSTRATOR_PORT=$PORT SEMIO_BUILD_MODE=prototype NODE_OPTIONS= SEMIO_BUILD_DYNAMIC_OUTPUT=false SEMIO_BUILD_TERMINAL=false \
  nohup "$REPO" ./🔨️modules/🧩️runtime/📜️script.ts serve > "$LOG" 2>&1 < /dev/null &
echo $! > "$PID"; disown
echo "serve pid $(cat "$PID"), log $LOG"
for i in $(seq 1 60); do
  kill -0 "$(cat "$PID")" 2>/dev/null || { echo "serve exited early; tail of log:" >&2; tail -20 "$LOG" >&2; exit 5; }
  if /usr/bin/grep -q "Demonstrator ready" "$LOG" 2>/dev/null; then
    code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 45 "$URL"); echo "ready, GET / -> $code"; listener; [ "$code" = 200 ] && exit 0; exit 6
  fi
  sleep 5
done
echo "not ready after 300 s; see $LOG (vite dep-optimizer race: stop, empty the vite cacheDir from viteCacheDirectory (cacheDir at ♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts:55), start once more)" >&2; exit 7
