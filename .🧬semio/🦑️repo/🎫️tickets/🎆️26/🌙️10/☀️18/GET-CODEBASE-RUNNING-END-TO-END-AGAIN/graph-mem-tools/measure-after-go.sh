#!/bin/sh
# live acceptance after GO: one cold-record-store `show projects` with phase rows, reading the peak RSS of the worker every 5 s and the governor's statistics
# usage: measure-after-go.sh [LOG]       (heavy: run once, alone, with the load below 30)
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && while [ ! -e .git ]; do cd ..; done; pwd)
LOG=${1:-/private/tmp/graph-mem/live-show.log}
sh "$HERE/guard.sh"
cd "$ROOT"
SEMIO_GRAPH_PHASES=1 nice -n 10 bun repo show projects > "$LOG.out" 2> "$LOG" &
pid=$!; peak=0
while kill -0 "$pid" 2>/dev/null; do
  for child in $(pgrep -P "$pid") "$pid"; do rss=$(ps -o rss= -p "$child" 2>/dev/null | tr -d ' '); [ -n "$rss" ] && [ "$rss" -gt "$peak" ] && peak=$rss; done
  sleep 5
done
echo "peak rss MB: $((peak / 1024))"
grep -E '\[graph-phase\]|graph: ' "$LOG" | tail -20
